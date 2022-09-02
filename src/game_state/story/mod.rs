use crate::game_state::player_actions::{ActionInProgress, PlayerActions};
use crate::game_state::story::quests::{init_quests, CompiledQuest, QuestId};
use crate::game_state::time::GameTime;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

pub mod quests;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Story {
    quests: Vec<CompiledQuest>,
    inactive_quests: HashSet<QuestId>,
    active_quests: HashSet<QuestId>,
    active_quests_by_activation_time: BTreeSet<(GameTime, QuestId)>,
    completed_quests: HashSet<QuestId>,
    completed_quests_by_completion_time: BTreeSet<(GameTime, QuestId)>,
}

impl Story {
    pub fn new(quests: Vec<CompiledQuest>) -> Self {
        let inactive_quests = quests.iter().map(|quest| quest.id).collect();
        Self {
            quests,
            inactive_quests,
            active_quests: Default::default(),
            active_quests_by_activation_time: Default::default(),
            completed_quests: Default::default(),
            completed_quests_by_completion_time: Default::default(),
        }
    }

    pub fn quest(&self, quest_id: QuestId) -> &CompiledQuest {
        &self.quests[quest_id.0]
    }

    pub fn iter_active_quests_by_activation_time(
        &self,
    ) -> impl Iterator<Item = &'_ CompiledQuest> + DoubleEndedIterator {
        self.active_quests_by_activation_time
            .iter()
            .map(|(_, quest_id)| self.quest(*quest_id))
    }

    pub fn iter_completed_quests_by_completion_time(
        &self,
    ) -> impl Iterator<Item = &'_ CompiledQuest> + DoubleEndedIterator {
        self.completed_quests_by_completion_time
            .iter()
            .map(|(_, quest_id)| self.quest(*quest_id))
    }
}
