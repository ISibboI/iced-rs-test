use crate::game_state::actions::ActionInProgress;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventLog {
    events: Vec<GameEvent>,
}

impl EventLog {
    pub fn log(&mut self, event: impl Into<GameEvent>) {
        self.events.push(event.into());
    }

    pub fn iter_rev(&self) -> impl Iterator<Item = &GameEvent> {
        self.events.iter().rev()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameEvent {
    Action(ActionInProgress),
}

impl From<ActionInProgress> for GameEvent {
    fn from(action: ActionInProgress) -> Self {
        Self::Action(action)
    }
}

impl AsRef<[GameEvent]> for EventLog {
    fn as_ref(&self) -> &[GameEvent] {
        self.events.as_ref()
    }
}
