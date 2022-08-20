use crate::game_state::actions::{
    Action, ActionInProgress, ACTIONS, ACTION_FIGHT_MONSTERS, ACTION_SLEEP, ACTION_TAVERN,
    ACTION_WAIT,
};
use crate::game_state::character::{Character, CharacterAttributeProgress, CharacterRace};
use crate::game_state::combat::{CombatStyle, SpawnedMonster};
use crate::game_state::currency::Currency;
use crate::game_state::story::Story;
use crate::game_state::time::{GameTime, MILLISECONDS_PER_HOUR};
use chrono::{DateTime, Duration, Utc};
use log::{debug, warn};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod actions;
pub mod character;
pub mod combat;
pub mod currency;
pub mod story;
pub mod time;

pub const GAME_TIME_PER_MILLISECOND: GameTime = GameTime::from_milliseconds(900);
pub const MAX_COMBAT_DURATION: GameTime = GameTime::from_hours(4);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: String,
    pub character: Character,
    pub current_action: ActionInProgress,
    pub selected_action: String,
    pub selected_combat_style: CombatStyle,
    pub current_time: GameTime,
    pub last_update: DateTime<Utc>,
    pub story: Story,
}

impl GameState {
    pub fn new(savegame_file: String, name: String, race: CharacterRace) -> Self {
        let selected_combat_style = race.starting_combat_style();
        Self {
            savegame_file,
            character: Character::new(name, race),
            current_action: ActionInProgress {
                action: ACTIONS.get(ACTION_SLEEP).unwrap().clone(),
                start: Default::default(),
                end: Default::default(),
                attribute_progress: CharacterAttributeProgress::zero(),
                monster: None,
                currency_reward: Currency::zero(),
                success: true,
            },
            selected_action: ACTION_WAIT.to_string(),
            selected_combat_style,
            current_time: Default::default(),
            last_update: DateTime::from(SystemTime::now()),
            story: Default::default(),
        }
    }

    pub fn update(&mut self, passed_real_milliseconds: i64) {
        if passed_real_milliseconds < 0 {
            warn!("Attempting to update with negative duration: {passed_real_milliseconds}; last_update: {}", self.last_update.naive_local());
            return;
        }

        let passed_game_time = passed_real_milliseconds * GAME_TIME_PER_MILLISECOND;
        self.current_time += passed_game_time;

        while self.current_action.end < self.current_time {
            if self.current_action.success {
                self.character
                    .add_attribute_progress(self.current_action.attribute_progress);
                self.character.currency += self.current_action.currency_reward;

                self.story.update(&self.current_action);
            }

            self.next_action();
            debug!("New action: {:?}", self.current_action);
        }

        self.last_update = self.last_update + Duration::milliseconds(passed_real_milliseconds);
    }

    fn next_action(&mut self) {
        let earliest_tavern_time = GameTime::from_hours(19);
        let latest_tavern_time = GameTime::from_hours(21);

        let start_time = self.current_action.end;
        let hour_of_day = start_time.hour_of_day();
        let time_of_day = start_time.time_of_day();

        let tavern_currency_gain = ACTIONS.get(ACTION_TAVERN).unwrap().currency_gain;

        let action = if !(6..22).contains(&hour_of_day) {
            // sleep until 6 in the morning
            let end_time = if hour_of_day < 6 {
                start_time.floor_day()
            } else {
                start_time.ceil_day()
            } + GameTime::from_hours(6);
            let duration = end_time - start_time;

            let action = ACTIONS.get(ACTION_SLEEP).unwrap().clone();
            ActionInProgress {
                start: start_time,
                end: end_time,
                attribute_progress: action.attribute_progress_factor.into_progress(duration),
                monster: None,
                currency_reward: Currency::zero(),
                action,
                success: true,
            }
        } else if self.character.currency >= -tavern_currency_gain
            && rand::thread_rng()
                .gen_range(earliest_tavern_time.seconds()..=latest_tavern_time.seconds())
                <= time_of_day.seconds()
        {
            let action = ACTIONS.get(ACTION_TAVERN).unwrap().clone();
            let duration = GameTime::from_hours(1);
            ActionInProgress {
                start: start_time,
                end: start_time + duration,
                attribute_progress: action.attribute_progress_factor.into_progress(duration),
                monster: None,
                currency_reward: tavern_currency_gain,
                action,
                success: true,
            }
        } else {
            let action = ACTIONS.get(&self.selected_action).unwrap().clone();

            if action.name == ACTION_FIGHT_MONSTERS {
                let monster = SpawnedMonster::spawn(self.character.level);
                let damage = self.damage_output();
                let duration =
                    GameTime::from_seconds((monster.hitpoints as f64 / damage * 60.0) as i128)
                        .min(MAX_COMBAT_DURATION);
                let attribute_progress = self.evaluate_combat_attribute_progress(duration);
                let success = duration < MAX_COMBAT_DURATION;

                ActionInProgress {
                    action,
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
                    start: start_time,
                    end: start_time + duration,
                    attribute_progress: action.attribute_progress_factor.into_progress(duration),
                    monster: None,
                    currency_reward: action.currency_gain,
                    action,
                    success: true,
                }
            }
        };
        self.current_action = action;
    }

    /// The progress of the current action as value between 0.0 and 1.0.
    pub fn current_action_progress(&self) -> f32 {
        if self.current_action.length().seconds() <= 0
            || self.current_action.end <= self.current_time
        {
            1.0
        } else if self.current_action.start >= self.current_time {
            0.0
        } else {
            let duration = self.current_action.length().seconds() as f32;
            let progress = (self.current_time - self.current_action.start).seconds() as f32;
            progress / duration
        }
    }

    pub fn list_feasible_actions<'output>(
        &self,
    ) -> impl 'output + Iterator<Item = &'output Action> {
        let level = self.character.level;
        ACTIONS
            .values()
            .filter(move |action| level >= action.required_level)
    }

    pub fn damage_output(&self) -> f64 {
        let attributes = &self.character.attributes;
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
        let damage = damage * (duration.milliseconds() as f64 / MILLISECONDS_PER_HOUR as f64);

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
