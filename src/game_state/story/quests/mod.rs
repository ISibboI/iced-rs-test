use crate::game_state::actions::{ActionInProgress, ACTION_SLEEP};
use crate::game_state::story::quests::quest_conditions::*;
use crate::game_state::time::GameTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod quest_conditions;

lazy_static! {
    pub static ref QUESTS: HashMap<String, Quest> = [
        Quest::new(
            "init",
            "Wake up!",
            "Wait until six o'clock, and you will wake up to a new day full of adventure!",
            [],
            action_is(ACTION_SLEEP) & time_geq(GameTime::from_seconds(1)) // dodge the initial dummy sleeping action that ends at time 0
        ),
        Quest::new("train_str", "Lift weights", "Lift weights a few times to gain some strength.", ["init"], action_count("Lift weights", 5)),
        Quest::new("train_dex", "Lift weights", "Lift weights a few times to gain some strength.", ["init"], action_count("Lift weights", 5)),
    ]
    .into_iter()
    .map(|quest| (quest.id.clone(), quest))
    .collect();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub precondition: Vec<String>,
    pub condition: QuestCondition,
}

impl Quest {
    fn new<'a>(
        id: impl ToString,
        title: impl ToString,
        description: impl ToString,
        precondition: impl AsRef<[&'a str]>,
        condition: impl Into<QuestCondition>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            precondition: precondition
                .as_ref()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            condition: condition.into(),
        }
    }

    pub fn update(&mut self, action_in_progress: &ActionInProgress) -> bool {
        self.condition.update(action_in_progress)
    }
}
