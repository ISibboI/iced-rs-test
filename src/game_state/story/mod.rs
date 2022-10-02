use crate::game_state::story::quests::{CompiledQuest, QuestId, QuestState};
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::iter;

pub mod quests;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Story {
    quests: Vec<CompiledQuest>,
    inactive_quests: HashSet<QuestId>,
    active_quests: HashSet<QuestId>,
    active_quests_by_activation_time: BTreeSet<(GameTime, QuestId)>,
    completed_quests: HashSet<QuestId>,
    completed_quests_by_completion_time: BTreeSet<(GameTime, QuestId)>,
    inactive_failed_quests: HashSet<QuestId>,
    inactive_failed_quests_by_failure_time: BTreeSet<(GameTime, QuestId)>,
    active_failed_quests: HashSet<QuestId>,
    active_failed_quests_by_failure_time: BTreeSet<(GameTime, QuestId)>,
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
            inactive_failed_quests: Default::default(),
            inactive_failed_quests_by_failure_time: Default::default(),
            active_failed_quests: Default::default(),
            active_failed_quests_by_failure_time: Default::default(),
        }
    }

    pub fn quest(&self, quest_id: QuestId) -> &CompiledQuest {
        &self.quests[quest_id.0]
    }

    pub fn quest_mut(&mut self, quest_id: QuestId) -> &mut CompiledQuest {
        &mut self.quests[quest_id.0]
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

    pub fn activate_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest = self.quest_mut(quest_id);
        assert!(quest.state.is_inactive());
        quest.state = QuestState::Active {
            activation_time: time,
        };
        assert!(self.inactive_quests.remove(&quest_id));
        assert!(self.active_quests.insert(quest_id));
        assert!(self
            .active_quests_by_activation_time
            .insert((time, quest_id)));
        iter::empty()
    }

    pub fn complete_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest = self.quest_mut(quest_id);
        assert!(quest.state.is_active());
        let activation_time = quest.state.activation_time().unwrap();
        quest.state = QuestState::Completed {
            activation_time,
            completion_time: time,
        };
        assert!(self.active_quests.remove(&quest_id));
        assert!(self
            .active_quests_by_activation_time
            .remove(&(activation_time, quest_id)));
        assert!(self.completed_quests.insert(quest_id));
        assert!(self
            .completed_quests_by_completion_time
            .insert((time, quest_id)));
        iter::empty()
    }

    pub fn fail_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest = self.quest_mut(quest_id);
        assert!(quest.state.is_inactive() || quest.state.is_active());
        match quest.state {
            QuestState::Inactive => {
                quest.state = QuestState::FailedWhileInactive { failure_time: time };
                assert!(self.inactive_quests.remove(&quest_id));
                assert!(self.inactive_failed_quests.insert(quest_id));
                assert!(self
                    .inactive_failed_quests_by_failure_time
                    .insert((time, quest_id)));
            }
            QuestState::Active { activation_time } => {
                quest.state = QuestState::FailedWhileActive {
                    activation_time,
                    failure_time: time,
                };
                assert!(self.active_quests.remove(&quest_id));
                assert!(self
                    .active_quests_by_activation_time
                    .remove(&(activation_time, quest_id)));
                assert!(self.active_failed_quests.insert(quest_id));
                assert!(self
                    .active_failed_quests_by_failure_time
                    .insert((time, quest_id)));
            }
            _ => unreachable!(),
        }
        iter::empty()
    }
}
