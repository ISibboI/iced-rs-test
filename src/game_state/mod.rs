use crate::game_state::character::{Character, CharacterAttributeProgress, CharacterRace};
use crate::game_state::combat::CombatStyle;
use crate::game_state::currency::Currency;
use crate::game_state::event_log::EventLog;
use crate::game_state::player_actions::{
    ActionInProgress, PlayerActions, ACTION_EXPLORE, ACTION_SLEEP, ACTION_TAVERN,
};
use crate::game_state::story::Story;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::{CompiledGameAction, CompiledGameEvent};
use crate::game_state::world::World;
use crate::game_template::CompiledGameTemplate;
use crate::savegames::pathbuf_serde::PathBufSerde;
use async_std::path::PathBuf;
use chrono::{DateTime, Duration, Utc};
use event_trigger_action_system::CompiledTriggers;
use log::{debug, warn};
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro512PlusPlus;
use serde::{Deserialize, Serialize};
use std::iter;
use std::ops::Deref;

pub mod character;
pub mod combat;
pub mod currency;
pub mod event_log;
pub mod player_actions;
pub mod story;
pub mod time;
pub mod triggers;
pub mod world;

pub const GAME_TIME_PER_MILLISECOND: GameTime = GameTime::from_milliseconds(900);
pub const MAX_COMBAT_DURATION: GameTime = GameTime::from_hours(4);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: PathBufSerde,
    pub rng: Xoshiro512PlusPlus,
    pub character: Character,
    pub selected_combat_style: CombatStyle,
    pub current_time: GameTime,
    pub last_update: DateTime<Utc>,
    pub log: EventLog,
    pub actions: PlayerActions,
    pub story: Story,
    pub world: World,
    pub triggers: CompiledTriggers<CompiledGameEvent>,
}

impl GameState {
    pub fn new(
        game_template: CompiledGameTemplate,
        savegame_file: PathBuf,
        name: String,
        race: CharacterRace,
    ) -> Self {
        let selected_combat_style = race.starting_combat_style();

        let mut result = Self {
            savegame_file: savegame_file.into(),
            rng: SeedableRng::from_entropy(),
            character: Character::new(name, race),
            selected_combat_style,
            current_time: Default::default(),
            last_update: Utc::now(),
            log: EventLog::default(),
            actions: game_template.actions,
            story: game_template.story,
            world: game_template.world,
            triggers: game_template.triggers,
        };
        result.execute_all_triggered_actions();
        result.update(0);
        result
    }

    pub fn update(&mut self, passed_real_milliseconds: i64) {
        if passed_real_milliseconds < 0 {
            warn!("Attempting to update with negative duration: {passed_real_milliseconds}; last_update: {}", self.last_update.naive_local());
            return;
        }

        let passed_game_time = passed_real_milliseconds * GAME_TIME_PER_MILLISECOND;
        self.current_time += passed_game_time;

        if !self.actions.has_action_in_progress() {
            let game_events = self.next_player_action(self.current_time);
            self.triggers.execute_owned_events(game_events);
            self.execute_all_triggered_actions();
            debug!("New action: {:?}", self.actions.in_progress());
        }

        while self.actions.in_progress().end < self.current_time {
            let mut game_events = Vec::new();
            if self.actions.in_progress().success {
                game_events.extend(
                    self.character
                        .add_attribute_progress(self.actions.in_progress().attribute_progress),
                );
                self.character.currency += self.actions.in_progress().currency_reward;

                if self.actions.in_progress().currency_reward != Currency::zero() {
                    game_events.push(CompiledGameEvent::CurrencyChanged {
                        value: self.character.currency,
                    })
                }
                game_events.push(CompiledGameEvent::ActionCompleted {
                    id: self.actions.in_progress().action.id,
                });
            }
            self.log.log(self.actions.in_progress().deref().clone());

            game_events.extend(self.next_player_action(self.actions.in_progress().end));
            debug!("New action: {:?}", self.actions.in_progress());

            self.triggers.execute_events(game_events.iter());
            self.execute_all_triggered_actions();
        }

        self.last_update += Duration::milliseconds(passed_real_milliseconds);
    }

    fn next_player_action(
        &mut self,
        start_time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let earliest_tavern_time = GameTime::from_hours(19);
        let latest_tavern_time = GameTime::from_hours(21);

        if self.actions.has_action_in_progress() {
            assert_eq!(self.actions.in_progress().end, start_time);
        }
        let hour_of_day = start_time.hour_of_day();
        let time_of_day = start_time.time_of_day();

        let tavern_currency_gain = self.actions.action(ACTION_TAVERN).currency_gain;

        let action = if !(6..22).contains(&hour_of_day) {
            // sleep until 6 in the morning
            let end_time = if hour_of_day < 6 {
                start_time.floor_day()
            } else {
                start_time.ceil_day()
            } + GameTime::from_hours(6);
            let duration = end_time - start_time;

            let action = self.actions.action(ACTION_SLEEP);
            ActionInProgress {
                action: action.id,
                start: start_time,
                end: end_time,
                attribute_progress: action.attribute_progress_factor.into_progress(duration),
                monster: None,
                currency_reward: Currency::zero(),
                success: true,
            }
        } else if self.character.currency >= -tavern_currency_gain
            && rand::thread_rng()
                .gen_range(earliest_tavern_time.seconds()..=latest_tavern_time.seconds())
                <= time_of_day.seconds()
        {
            let action = self.actions.action(ACTION_TAVERN);
            let duration = GameTime::from_hours(1);
            ActionInProgress {
                action: action.id,
                start: start_time,
                end: start_time + duration,
                attribute_progress: action.attribute_progress_factor.into_progress(duration),
                monster: None,
                currency_reward: tavern_currency_gain,
                success: true,
            }
        } else {
            let action = self.actions.action(self.actions.selected_action);

            if action.id == ACTION_EXPLORE {
                let location = self.world.location(self.world.selected_location);
                let monster = location.spawn();
                let damage = self.damage_output();
                let duration = GameTime::from_milliseconds(
                    (monster.hitpoints as f64 / damage * 60_000.0).round() as i128,
                )
                .min(MAX_COMBAT_DURATION);
                let attribute_progress = self.evaluate_combat_attribute_progress(duration);
                let success = duration < MAX_COMBAT_DURATION;

                ActionInProgress {
                    action: action.id,
                    start: start_time,
                    end: start_time + duration,
                    attribute_progress,
                    currency_reward: monster.currency_reward,
                    monster: Some(monster),
                    success,
                }
            } else {
                let duration = GameTime::from_hours(1);
                ActionInProgress {
                    action: action.id,
                    start: start_time,
                    end: start_time + duration,
                    attribute_progress: action.attribute_progress_factor.into_progress(duration),
                    monster: None,
                    currency_reward: action.currency_gain,
                    success: true,
                }
            }
        };
        self.actions.set_in_progress(action);
        iter::once(CompiledGameEvent::ActionStarted {
            id: self.actions.in_progress().action.id,
        })
    }

    fn execute_all_triggered_actions(&mut self) {
        while let Some(game_action) = self.triggers.consume_action() {
            let game_events = self.execute_game_action(game_action);
            self.triggers.execute_owned_events(game_events);
        }
    }

    fn execute_game_action(
        &mut self,
        game_action: CompiledGameAction,
    ) -> Box<dyn Iterator<Item = CompiledGameEvent>> {
        match game_action {
            CompiledGameAction::ActivateQuest { id } => {
                Box::new(self.story.activate_quest(id, self.current_time))
            }
            CompiledGameAction::CompleteQuest { id } => {
                Box::new(self.story.complete_quest(id, self.current_time))
            }
            CompiledGameAction::FailQuest { id } => {
                Box::new(self.story.fail_quest(id, self.current_time))
            }
            CompiledGameAction::ActivateAction { id } => {
                Box::new(self.actions.activate_action(id, self.current_time))
            }
            CompiledGameAction::DeactivateAction { id } => {
                Box::new(self.actions.deactivate_action(id, self.current_time))
            }
            CompiledGameAction::ActivateLocation { id } => {
                Box::new(self.world.activate_location(id, self.current_time))
            }
            CompiledGameAction::DeactivateLocation { id } => {
                Box::new(self.world.deactivate_location(id, self.current_time))
            }
        }
    }

    /// The progress of the current action as value between 0.0 and 1.0.
    pub fn current_action_progress(&self) -> f32 {
        let current_action = self.actions.in_progress();
        if current_action.length().seconds() <= 0 || current_action.end <= self.current_time {
            1.0
        } else if current_action.start >= self.current_time {
            0.0
        } else {
            let duration = current_action.length().seconds() as f32;
            let progress = (self.current_time - current_action.start).seconds() as f32;
            progress / duration
        }
    }

    pub fn damage_output(&self) -> f64 {
        let attributes = self.character.attributes();
        match self.selected_combat_style {
            CombatStyle::CloseContact => {
                0.45 * attributes.strength as f64
                    + 0.45 * attributes.stamina as f64
                    + 0.1 * attributes.dexterity as f64
            }
            CombatStyle::Ranged => {
                0.1 * attributes.strength as f64
                    + 0.1 * attributes.stamina as f64
                    + 0.8 * attributes.dexterity as f64
            }
            CombatStyle::Magic => {
                0.4 * attributes.intelligence as f64 + 0.6 * attributes.wisdom as f64
            }
        }
    }

    fn evaluate_combat_attribute_progress(&self, duration: GameTime) -> CharacterAttributeProgress {
        let damage = self.damage_output();
        let damage = if damage > 1.0 { damage.sqrt() } else { damage };
        let damage = damage * duration.milliseconds() as f64;

        match self.selected_combat_style {
            CombatStyle::CloseContact => CharacterAttributeProgress::new(
                (0.45 * damage).round() as u64,
                (0.45 * damage).round() as u64,
                (0.1 * damage).round() as u64,
                0,
                0,
                0,
            ),
            CombatStyle::Ranged => CharacterAttributeProgress::new(
                (0.1 * damage).round() as u64,
                (0.1 * damage).round() as u64,
                (0.8 * damage).round() as u64,
                0,
                0,
                0,
            ),
            CombatStyle::Magic => CharacterAttributeProgress::new(
                0,
                0,
                0,
                (0.4 * damage).round() as u64,
                (0.6 * damage).round() as u64,
                0,
            ),
        }
    }
}
