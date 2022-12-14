use crate::game_state::character::{Character, CharacterRace};
use crate::game_state::currency::Currency;
use crate::game_state::event_log::EventLog;
use crate::game_state::inventory::Inventory;
use crate::game_state::player_actions::{
    PlayerActionInProgressKind, PlayerActionInProgressSource, PlayerActions, ACTION_EXPLORE,
    ACTION_SLEEP, ACTION_TAVERN, ACTION_WAIT,
};
use crate::game_state::story::Story;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::{CompiledGameAction, CompiledGameEvent};
use crate::game_state::world::World;
use crate::game_template::CompiledGameTemplate;
use crate::io::pathbuf_serde::PathBufSerde;
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
pub mod currency;
pub mod event_log;
pub mod inventory;
pub mod player_actions;
pub mod story;
pub mod time;
pub mod triggers;
pub mod world;

pub const GAME_TIME_PER_MILLISECOND: GameTime = GameTime::from_milliseconds(900);
pub const MIN_COMBAT_DURATION: GameTime = GameTime::from_minutes(10);
pub const MAX_COMBAT_DURATION: GameTime = GameTime::from_hours(4);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: PathBufSerde,
    pub rng: Xoshiro512PlusPlus,
    pub character: Character,
    pub current_time: GameTime,
    pub last_update: DateTime<Utc>,
    pub log: EventLog,
    pub actions: PlayerActions,
    pub story: Story,
    pub world: World,
    pub inventory: Inventory,
    pub triggers: CompiledTriggers<CompiledGameEvent>,
}

#[derive(Clone, Debug)]
pub struct GameStateInitialisation {
    pub savegame_file: PathBuf,
    pub name: String,
    pub pronoun: String,
    pub race: CharacterRace,
}

impl GameState {
    pub fn new(
        game_template: CompiledGameTemplate,
        initialisation: GameStateInitialisation,
    ) -> Self {
        let mut result = Self {
            savegame_file: initialisation.savegame_file.into(),
            rng: SeedableRng::from_entropy(),
            character: Character::new(
                initialisation.name,
                initialisation.pronoun,
                initialisation.race,
            ),
            current_time: game_template.initialisation.starting_time,
            last_update: Utc::now(),
            log: EventLog::default(),
            actions: game_template.actions,
            story: game_template.story,
            world: game_template.world,
            inventory: game_template.inventory,
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
                self.inventory.currency += self.actions.in_progress().currency_reward;
                game_events.extend(
                    self.inventory
                        .add_multiple(self.actions.in_progress().items.iter().copied()),
                );

                if self.actions.in_progress().currency_reward != Currency::zero() {
                    game_events.push(CompiledGameEvent::CurrencyChanged {
                        value: self.inventory.currency,
                    })
                }
                game_events.push(CompiledGameEvent::ActionCompleted {
                    id: self.actions.in_progress().source.action_id(),
                });
                if self.actions.in_progress().source.action_id() == ACTION_EXPLORE {
                    game_events.push(CompiledGameEvent::ExplorationCompleted {
                        id: self.actions.in_progress().location,
                    });
                }
                match self.actions.in_progress().kind {
                    PlayerActionInProgressKind::Combat(monster) => {
                        game_events.push(CompiledGameEvent::MonsterKilled { id: monster });
                    }
                    PlayerActionInProgressKind::None => {}
                }
                match self.actions.in_progress().source {
                    PlayerActionInProgressSource::Action(_) => {}
                    PlayerActionInProgressSource::Exploration(exploration_event) => {
                        game_events.push(CompiledGameEvent::ExplorationEventCompleted {
                            id: exploration_event,
                        });
                    }
                }
            } else {
                match self.actions.in_progress().kind {
                    PlayerActionInProgressKind::Combat(monster) => {
                        game_events.push(CompiledGameEvent::MonsterFailed { id: monster });
                    }
                    PlayerActionInProgressKind::None => {}
                }
            }

            self.log.log(self.actions.in_progress().deref().clone());

            self.triggers.execute_events(game_events.iter());
            self.execute_all_triggered_actions();

            game_events.extend(self.next_player_action(self.actions.in_progress().end));
            debug!("New action: {:?}", self.actions.in_progress());
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

        let tavern_currency_gain = self.actions.action(ACTION_TAVERN).currency_reward;

        let action = if !(6..22).contains(&hour_of_day) {
            // sleep until 6 in the morning
            let end_time = if hour_of_day < 6 {
                start_time.floor_day()
            } else {
                start_time.ceil_day()
            } + GameTime::from_hours(6);

            let action = self.actions.action(ACTION_SLEEP);
            let mut action_in_progress =
                action.spawn(&mut self.rng, start_time, self.world.selected_location);
            action_in_progress.end = end_time;
            action_in_progress
        } else if self.inventory.currency >= -tavern_currency_gain
            && rand::thread_rng()
                .gen_range(earliest_tavern_time.seconds()..=latest_tavern_time.seconds())
                <= time_of_day.seconds()
        {
            let action = self.actions.action(ACTION_TAVERN);
            action.spawn(&mut self.rng, start_time, self.world.selected_location)
        } else {
            let action = self.actions.action(self.actions.selected_action);

            if action.id == ACTION_EXPLORE {
                self.world
                    .explore(&mut self.rng, start_time, action.duration, &self.character)
                    .unwrap_or_else(|| {
                        self.actions.action(ACTION_WAIT).spawn(
                            &mut self.rng,
                            start_time,
                            self.world.selected_location,
                        )
                    })
            } else {
                action.spawn(&mut self.rng, start_time, self.world.selected_location)
            }
        };

        assert!(
            self.actions
                .action(action.source.action_id())
                .state
                .is_active(),
            "{:?}",
            self.actions.action(action.source.action_id())
        );
        self.actions.set_in_progress(action);
        iter::once(CompiledGameEvent::ActionStarted {
            id: self.actions.in_progress().source.action_id(),
        })
        .chain(
            if self.actions.in_progress().source.action_id() == ACTION_EXPLORE {
                Some(CompiledGameEvent::ExplorationStarted {
                    id: self.actions.in_progress().location,
                })
            } else {
                None
            },
        )
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
            CompiledGameAction::CompleteQuestStage { id } => {
                Box::new(self.story.complete_quest_stage(
                    &mut self.rng,
                    &mut self.inventory,
                    id,
                    self.current_time,
                ))
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
            CompiledGameAction::ActivateExplorationEvent { id } => {
                Box::new(self.world.activate_exploration_event(id, self.current_time))
            }
            CompiledGameAction::DeactivateExplorationEvent { id } => Box::new(
                self.world
                    .deactivate_exploration_event(id, self.current_time),
            ),
            CompiledGameAction::ActivateMonster { id } => {
                Box::new(self.world.activate_monster(id, self.current_time))
            }
            CompiledGameAction::DeactivateMonster { id } => {
                Box::new(self.world.deactivate_monster(id, self.current_time))
            }
            CompiledGameAction::ActivateItem { id } => {
                Box::new(self.inventory.activate_item(id, self.current_time))
            }
            CompiledGameAction::DeactivateItem { id } => {
                Box::new(self.inventory.deactivate_item(id, self.current_time))
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
}
