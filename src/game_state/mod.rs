use crate::game_state::actions::{Action, ActionInProgress};
use crate::game_state::character::{Character, CharacterRace};
use crate::game_state::combat::{CombatStyle, SpawnedMonster};
use crate::game_state::time::GameTime;
use log::debug;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod character;
pub mod combat;
pub mod time;

pub const GAME_TIME_PER_SECOND: GameTime = GameTime::from_minutes(15);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: String,
    pub character: Character,
    pub current_action: ActionInProgress,
    pub selected_action: Action,
    pub selected_combat_style: CombatStyle,
    pub current_time: GameTime,
}

impl GameState {
    pub fn new(savegame_file: String, name: String, race: CharacterRace) -> Self {
        let selected_combat_style = race.starting_combat_style();
        Self {
            savegame_file,
            character: Character::new(name, race),
            current_action: ActionInProgress {
                action: Action::Sleep,
                start: Default::default(),
                end: Default::default(),
                attribute_progress: (0.0, 0.0, 0.0, 0.0),
                monster: None,
            },
            selected_action: Action::Wait,
            selected_combat_style,
            current_time: Default::default(),
        }
    }

    pub fn update(&mut self, passed_real_seconds: f64) {
        let passed_game_seconds = passed_real_seconds * GAME_TIME_PER_SECOND.seconds() as f64;
        let passed_game_seconds = GameTime::from_seconds(passed_game_seconds as i128);
        self.current_time += passed_game_seconds;

        while self.current_action.end < self.current_time {
            self.character
                .add_attribute_progress(self.current_action.attribute_progress);

            self.next_action();
            debug!("New action: {:?}", self.current_action);
        }
    }

    fn next_action(&mut self) {
        let earliest_tavern_time = GameTime::from_hours(19);
        let latest_tavern_time = GameTime::from_hours(21);

        let start_time = self.current_action.end;
        let hour_of_day = start_time.hour_of_day();
        let time_of_day = start_time.time_of_day();

        let action = if !(6..22).contains(&hour_of_day) {
            // sleep until 6 in the morning
            let end_time = if hour_of_day < 6 {
                start_time.floor_day()
            } else {
                start_time.ceil_day()
            } + GameTime::from_hours(6);

            ActionInProgress {
                action: Action::Sleep,
                start: start_time,
                end: end_time,
                attribute_progress: Action::Sleep.attribute_progress_str_dex_int_chr(),
                monster: None,
            }
        } else if rand::thread_rng()
            .gen_range(earliest_tavern_time.seconds()..=latest_tavern_time.seconds())
            <= time_of_day.seconds()
        {
            ActionInProgress {
                action: Action::Tavern,
                start: start_time,
                end: start_time + GameTime::from_hours(1),
                attribute_progress: Action::Tavern.attribute_progress_str_dex_int_chr(),
                monster: None,
            }
        } else {
            let action = self.selected_action.clone();

            if action == Action::FightMonsters {
                let monster = SpawnedMonster::spawn(self.character.level);
                let damage = self.damage_output();
                let attribute_progress = self.evaluate_combat_attribute_progress();
                let duration = GameTime::from_seconds((monster.hitpoints / damage * 60.0) as i128);

                ActionInProgress {
                    action,
                    start: start_time,
                    end: start_time + duration,
                    attribute_progress,
                    monster: Some(monster),
                }
            } else {
                ActionInProgress {
                    start: start_time,
                    end: start_time + GameTime::from_hours(1),
                    attribute_progress: action.attribute_progress_str_dex_int_chr(),
                    monster: None,
                    action,
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

    pub fn list_feasible_actions(&self) -> Vec<Action> {
        let result = vec![
            Action::Wait,
            Action::WeightLift,
            Action::Jog,
            Action::Read,
            Action::FightMonsters,
        ];
        result
    }

    pub fn damage_output(&self) -> f64 {
        match self.selected_combat_style {
            CombatStyle::CloseContact => {
                0.8 * self.character.strength as f64
                    + 0.1 * self.character.dexterity as f64
                    + 0.1 * self.character.intelligence as f64
            }
            CombatStyle::Ranged => {
                0.1 * self.character.strength as f64
                    + 0.8 * self.character.dexterity as f64
                    + 0.1 * self.character.intelligence as f64
            }
            CombatStyle::Magic => {
                0.1 * self.character.strength as f64
                    + 0.1 * self.character.dexterity as f64
                    + 0.8 * self.character.intelligence as f64
            }
        }
    }

    fn evaluate_combat_attribute_progress(&self) -> (f64, f64, f64, f64) {
        match self.selected_combat_style {
            CombatStyle::CloseContact => {
                let damage = self.damage_output();
                (damage * 0.8, damage * 0.1, damage * 0.1, 0.0)
            }
            CombatStyle::Ranged => {
                let damage = self.damage_output();
                (damage * 0.1, damage * 0.8, damage * 0.1, 0.0)
            }
            CombatStyle::Magic => {
                let damage = self.damage_output();
                (damage * 0.1, damage * 0.1, damage * 0.8, 0.0)
            }
        }
    }
}
