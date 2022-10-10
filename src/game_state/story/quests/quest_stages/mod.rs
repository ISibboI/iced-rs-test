use crate::game_state::story::quests::QuestId;
use crate::game_state::time::GameTime;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestStage {
    pub id_str: String,
    pub description: Option<String>,
    pub task: String,
    pub completion_condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledQuestStage {
    pub id: QuestStageId,
    pub id_str: String,
    pub description: Option<String>,
    pub task: String,
    pub completion_condition: TriggerHandle,
    pub state: QuestStageState,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct QuestStageId {
    pub quest_id: QuestId,
    pub stage_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestStageState {
    Inactive,
    Active {
        activation_time: GameTime,
    },
    Completed {
        activation_time: GameTime,
        completion_time: GameTime,
    },
    FailedWhileInactive {
        failure_time: GameTime,
    },
    FailedWhileActive {
        activation_time: GameTime,
        failure_time: GameTime,
    },
}

impl QuestStage {
    pub fn compile(self, id_maps: &IdMaps, quest_id: QuestId) -> CompiledQuestStage {
        CompiledQuestStage {
            id: *id_maps
                .quest_stages
                .get(&(quest_id, self.id_str.clone()))
                .unwrap(),
            id_str: self.id_str.clone(),
            description: self.description,
            task: self.task,
            completion_condition: *id_maps.triggers.get(&self.completion_condition).unwrap(),
            state: QuestStageState::Inactive,
        }
    }
}

#[allow(dead_code)]
impl QuestStageState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, QuestStageState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, QuestStageState::Active { .. })
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, QuestStageState::Completed { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(
            self,
            QuestStageState::FailedWhileInactive { .. } | QuestStageState::FailedWhileActive { .. }
        )
    }

    pub fn activation_time(&self) -> Option<GameTime> {
        match self {
            QuestStageState::Inactive => None,
            QuestStageState::Active { activation_time } => Some(*activation_time),
            QuestStageState::Completed {
                activation_time, ..
            } => Some(*activation_time),
            QuestStageState::FailedWhileInactive { .. } => None,
            QuestStageState::FailedWhileActive {
                activation_time, ..
            } => Some(*activation_time),
        }
    }

    pub fn completion_time(&self) -> Option<GameTime> {
        match self {
            QuestStageState::Inactive => None,
            QuestStageState::Active { .. } => None,
            QuestStageState::Completed {
                completion_time, ..
            } => Some(*completion_time),
            QuestStageState::FailedWhileInactive { .. } => None,
            QuestStageState::FailedWhileActive { .. } => None,
        }
    }
}
