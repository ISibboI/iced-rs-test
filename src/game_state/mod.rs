use crate::game_state::actions::{Action, ActionInProgress};
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use log::debug;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod time;

pub const GAME_TIME_PER_SECOND: GameTime = GameTime::from_minutes(15);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: String,
    pub name: String,
    pub level: usize,
    pub race: CharacterRace,
    pub current_action: ActionInProgress,
    pub selected_action: Action,
    pub current_time: GameTime,
}

impl GameState {
    pub fn new(savegame_file: String, name: String, race: CharacterRace) -> Self {
        Self {
            savegame_file,
            name,
            level: 1,
            race,
            current_action: Default::default(),
            selected_action: Action::Wait,
            current_time: Default::default(),
        }
    }

    pub fn update(&mut self, passed_real_seconds: f64) {
        let passed_game_seconds = passed_real_seconds * GAME_TIME_PER_SECOND.seconds() as f64;
        let passed_game_seconds = GameTime::from_seconds(passed_game_seconds as i128);
        self.current_time += passed_game_seconds;

        while self.current_action.end < self.current_time {
            self.next_action();
            debug!("New action: {:?}", self.current_action);
        }
    }

    fn next_action(&mut self) {
        let start_time = self.current_action.end;
        let action = if start_time.hour_of_day() >= 22 || start_time.hour_of_day() < 6 {
            // sleep until 6 in the morning
            let end_time = if start_time.hour_of_day() < 6 {
                start_time.floor_day()
            } else {
                start_time.ceil_day()
            } + GameTime::from_hours(6);

            ActionInProgress {
                action: Action::Sleep,
                start: start_time,
                end: end_time,
            }
        } else {
            ActionInProgress {
                action: self.selected_action.clone(),
                start: start_time,
                end: start_time + GameTime::from_hours(1),
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
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Sequence, Eq, PartialEq)]
pub enum CharacterRace {
    #[default]
    Human,
    Orc,
    Elf,
}

impl ToString for CharacterRace {
    fn to_string(&self) -> String {
        match self {
            CharacterRace::Human => "Human".to_string(),
            CharacterRace::Orc => "Orc".to_string(),
            CharacterRace::Elf => "Elf".to_string(),
        }
    }
}
