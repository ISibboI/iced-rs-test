use crate::game_state::actions::ActionInProgress;
use crate::game_state::story::quests::{Quest, QUESTS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod quests;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Story {
    pub active_quests: HashMap<String, Quest>,
    pub completed_quests: HashMap<String, Quest>,
}

impl Story {
    pub fn update(&mut self, action_in_progress: &ActionInProgress) {
        assert!(action_in_progress.success);

        let mut completed = Vec::new();
        for active_quest in self.active_quests.values_mut() {
            if active_quest.update(action_in_progress) {
                completed.push(active_quest.clone());
            }
        }

        for completed_quest in completed {
            self.active_quests.remove(&completed_quest.id).unwrap();
            self.completed_quests
                .insert(completed_quest.id.clone(), completed_quest);
        }

        self.update_active_quests();
    }

    fn update_active_quests(&mut self) {
        for quest in QUESTS.values() {
            if self.active_quests.contains_key(&quest.id)
                || self.completed_quests.contains_key(&quest.id)
            {
                continue;
            }

            if quest
                .precondition
                .iter()
                .all(|precondition| self.completed_quests.contains_key(precondition))
            {
                self.active_quests.insert(quest.id.clone(), quest.clone());
            }
        }
    }
}

impl Default for Story {
    fn default() -> Self {
        let mut result = Self {
            active_quests: Default::default(),
            completed_quests: Default::default(),
        };
        result.update_active_quests();
        result
    }
}
