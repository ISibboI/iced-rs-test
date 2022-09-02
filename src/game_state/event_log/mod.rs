use crate::game_state::player_actions::ActionInProgress;
use crate::game_state::time::GameTime;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub static EVENT_LOG_SIZE: usize = 100;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventLog {
    events: VecDeque<GameEvent>,
}

impl EventLog {
    pub fn log(&mut self, event: impl Into<GameEvent>) {
        while self.events.len() >= EVENT_LOG_SIZE - 1 {
            self.events.pop_front();
        }
        self.events.push_back(event.into());
    }

    pub fn iter_rev(&self) -> impl Iterator<Item = &GameEvent> {
        self.events.iter().rev()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameEvent {
    pub time: GameTime,
    pub kind: GameEventKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameEventKind {
    Action(ActionInProgress),
}

impl From<ActionInProgress> for GameEvent {
    fn from(action: ActionInProgress) -> Self {
        Self {
            time: action.end,
            kind: GameEventKind::Action(action),
        }
    }
}
