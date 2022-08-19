use crate::game_state::actions::ActionInProgress;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Story {}

pub trait QuestCondition {
    fn update(&mut self, action_in_progress: &ActionInProgress);
}
