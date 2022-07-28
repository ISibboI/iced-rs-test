use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, Sequence, Eq, PartialEq)]
pub enum Action {
    #[default]
    Wait,
    Sleep,
}

impl Action {
    pub fn verb_progressive(&self) -> &str {
        match self {
            Action::Wait => "waiting",
            Action::Sleep => "sleeping",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionInProgress {
    pub action: Action,
    pub start: GameTime,
    pub end: GameTime,
}

impl ActionInProgress {
    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}

impl Default for ActionInProgress {
    fn default() -> Self {
        Self {
            action: Action::Wait,
            start: GameTime::from_seconds(0),
            end: GameTime::from_seconds(0),
        }
    }
}
