use crate::game_state::story::quests::{
    CompiledQuest, QuestId, QuestStageId, QuestStageState, QuestState,
};
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;

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
        if quest.state.is_failed() {
            return None.into_iter();
        }

        assert!(quest.state.is_inactive());
        quest.state = QuestState::Active {
            activation_time: time,
            active_stage: 0,
        };
        let stage = quest.stages.first_mut().unwrap();
        assert!(stage.state.is_inactive());
        stage.state = QuestStageState::Active {
            activation_time: time,
        };
        assert!(self.inactive_quests.remove(&quest_id));
        assert!(self.active_quests.insert(quest_id));
        assert!(self
            .active_quests_by_activation_time
            .insert((time, quest_id)));

        Some(CompiledGameEvent::QuestStageActivated {
            id: QuestStageId {
                quest_id,
                stage_id: 0,
            },
        })
        .into_iter()
    }

    pub fn complete_quest_stage(
        &mut self,
        quest_stage_id: QuestStageId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest_id = quest_stage_id.quest_id;
        let quest = self.quest_mut(quest_id);
        if quest.state.is_failed() {
            return None.into_iter();
        }

        assert!(quest.state.is_active());
        let active_stage = quest.active_stage_mut().unwrap();
        assert!(active_stage.state.is_active());
        let activation_time = active_stage.state.activation_time().unwrap();
        active_stage.state = QuestStageState::Completed {
            activation_time,
            completion_time: time,
        };

        quest.state.increment_active_stage();
        if let Some(active_stage) = quest.active_stage_mut() {
            assert!(active_stage.state.is_inactive());
            active_stage.state = QuestStageState::Active {
                activation_time: time,
            };
            Some(CompiledGameEvent::QuestStageActivated {
                id: QuestStageId {
                    quest_id,
                    stage_id: quest.state.active_stage().unwrap(),
                },
            })
            .into_iter()
        } else {
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
            Some(CompiledGameEvent::QuestCompleted { id: quest_id }).into_iter()
        }
    }

    pub fn fail_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let failed_event_creator = move |stage_id| CompiledGameEvent::QuestStageFailed {
            id: QuestStageId { quest_id, stage_id },
        };
        let quest = self.quest_mut(quest_id);
        assert!(!quest.state.is_failed());

        match quest.state {
            QuestState::Inactive => {
                quest.state = QuestState::FailedWhileInactive { failure_time: time };
                assert!(self.inactive_quests.remove(&quest_id));
                assert!(self.inactive_failed_quests.insert(quest_id));
                assert!(self
                    .inactive_failed_quests_by_failure_time
                    .insert((time, quest_id)));

                let quest = self.quest_mut(quest_id);
                for stage in &mut quest.stages {
                    assert!(stage.state.is_inactive());
                    stage.state = QuestStageState::FailedWhileInactive { failure_time: time };
                }

                (0..quest.stages.len()).map(failed_event_creator)
            }
            QuestState::Active {
                activation_time,
                active_stage,
            } => {
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

                let quest = self.quest_mut(quest_id);
                assert!(quest.stages[active_stage].state.is_active());
                let active_stage_activation_time =
                    quest.stages[active_stage].state.activation_time().unwrap();
                quest.stages[active_stage].state = QuestStageState::FailedWhileActive {
                    activation_time: active_stage_activation_time,
                    failure_time: time,
                };
                for stage in quest.stages.iter_mut().skip(active_stage + 1) {
                    assert!(stage.state.is_inactive());
                    stage.state = QuestStageState::FailedWhileInactive { failure_time: time };
                }

                (active_stage..quest.stages.len()).map(failed_event_creator)
            }
            _ => unreachable!(),
        }
    }
}
