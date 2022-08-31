use crate::game_state::actions::{ActionId, ActionInProgress, Actions};
use crate::game_state::story::quests::quest_conditions::*;
use crate::game_state::time::GameTime;
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod quest_conditions;
#[cfg(test)]
mod tests;

pub fn init_quests(actions: &Actions) -> HashMap<QuestId, CompiledQuest> {
    let quests = vec![
        Quest::new(
            "init",
            "Wake up!",
            "Wait until six o'clock, and you will wake up to a new day full of adventure!",
            none(),
            action_is("Sleep"),
        ),
        Quest::new("train_str", "Lift weights", "Lift weights a few times to gain some strength.", completed("init"), action_count("Lift weights", 5)),
        Quest::new("train_sta", "Go for a run", "Jog around a bit to increase your stamina.", completed("init"), action_count("Jog", 5)),
        Quest::new("train_dex", "Try out juggling", "Practice some juggling to improve your dexterity.", completed("init"), action_count("Practice juggling", 5)),
        Quest::new("train_int", "Train your brain", "Read a book about logic to improve your intelligence.", completed("init"), action_count("Study logic", 5)),
        Quest::new("train_wis", "Read a book", "Read a book about the world to increase your wisdom.", completed("init"), action_count("Read", 5)),
        Quest::new("train_chr", "Talk to some strangers", "Visit the tavern and talk to some people to gain some charisma.", completed("init"), action_count("Tavern", 5)),
        Quest::hidden("fight_monsters_pre", none(), any_n([completed("train_str"), completed("train_sta"), completed("train_dex"), completed("train_int"), completed("train_wis"), completed("train_chr")], 2)),
        Quest::new("fight_monsters", "Fight some monsters", "You have done some basic training. Put it to work by being a hero and killing some beasts and bad guys!", completed("fight_monsters_pre"), action_count("Fight monsters", 10)),
    ];
    let id_map: HashMap<_, QuestId> = quests
        .iter()
        .enumerate()
        .map(|(id, quest)| (quest.id_str.clone(), id.into()))
        .collect();

    quests
        .into_iter()
        .map(|quest| {
            let compiled_quest = quest.compile(&id_map, actions.actions_by_name());
            (compiled_quest.id, compiled_quest)
        })
        .collect()
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct QuestId(usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quest {
    pub id_str: String,
    pub title: String,
    pub description: String,
    pub precondition: QuestCondition,
    pub condition: QuestCondition,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CompiledQuest {
    pub id: QuestId,
    pub id_str: String,
    pub title: String,
    pub description: String,
    pub precondition: CompiledQuestCondition,
    pub condition: CompiledQuestCondition,
    pub hidden: bool,
    pub state: QuestState,
}

impl Quest {
    fn new(
        id: impl ToString,
        title: impl ToString,
        description: impl ToString,
        precondition: impl Into<QuestCondition>,
        condition: impl Into<QuestCondition>,
    ) -> Self {
        Self {
            id_str: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            precondition: precondition.into(),
            condition: condition.into(),
            hidden: false,
        }
    }

    fn hidden(
        id: impl ToString,
        precondition: impl Into<QuestCondition>,
        condition: impl Into<QuestCondition>,
    ) -> Self {
        Self {
            id_str: id.to_string(),
            title: Default::default(),
            description: Default::default(),
            precondition: precondition.into(),
            condition: condition.into(),
            hidden: true,
        }
    }

    fn compile(
        self,
        quest_id_map: &HashMap<String, QuestId>,
        action_id_map: &HashMap<String, ActionId>,
    ) -> CompiledQuest {
        CompiledQuest {
            id: *quest_id_map.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            title: self.title,
            description: self.description,
            precondition: self.precondition.compile(quest_id_map, action_id_map),
            condition: self.condition.compile(quest_id_map, action_id_map),
            hidden: self.hidden,
            state: QuestState::Inactive,
        }
    }
}

impl CompiledQuest {
    pub fn update_action_completed(
        &mut self,
        action_in_progress: &ActionInProgress,
    ) -> QuestConditionEvaluation {
        match self.state {
            QuestState::Inactive => {
                let result = self
                    .precondition
                    .update_action_completed(action_in_progress);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Active {
                        activation_time: action_in_progress.end,
                    };
                    trace!(
                        "Quest {} was activated by action {:?}",
                        self.id_str,
                        action_in_progress.action
                    );
                }
                result
            }
            QuestState::Active { activation_time } => {
                let result = self.condition.update_action_completed(action_in_progress);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Completed {
                        activation_time,
                        completion_time: action_in_progress.end,
                    };
                    trace!(
                        "Quest {} was completed by action {:?}",
                        self.id_str,
                        action_in_progress.action
                    );
                }
                result
            }
            QuestState::Completed { .. } => unreachable!("Completed actions are never updated"),
        }
    }

    pub fn activate_quests(
        &mut self,
        activated_quests: &HashSet<QuestId>,
        time: GameTime,
    ) -> QuestConditionEvaluation {
        match self.state {
            QuestState::Inactive => {
                let result = self.precondition.activate_quests(activated_quests);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Active {
                        activation_time: time,
                    };
                    trace!("Quest {} was activated by activated quests", self.id_str);
                }
                result
            }
            QuestState::Active { activation_time } => {
                let result = self.condition.activate_quests(activated_quests);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Completed {
                        activation_time,
                        completion_time: time,
                    };
                    trace!("Quest {} was completed by completed quests", self.id_str);
                }
                result
            }
            QuestState::Completed { .. } => unreachable!("Completed actions are never updated"),
        }
    }

    pub fn complete_quests(
        &mut self,
        completed_quests: &HashSet<QuestId>,
        time: GameTime,
    ) -> QuestConditionEvaluation {
        match self.state {
            QuestState::Inactive => {
                let result = self.precondition.complete_quests(completed_quests);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Active {
                        activation_time: time,
                    };
                    trace!("Quest {} was activated by completed quests", self.id_str);
                }
                result
            }
            QuestState::Active { activation_time } => {
                let result = self.condition.complete_quests(completed_quests);
                if result == QuestConditionEvaluation::True {
                    self.state = QuestState::Completed {
                        activation_time,
                        completion_time: time,
                    };
                    trace!("Quest {} was completed by completed quests", self.id_str);
                }
                result
            }
            QuestState::Completed { .. } => unreachable!("Completed actions are never updated"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestState {
    Inactive,
    Active {
        activation_time: GameTime,
    },
    Completed {
        activation_time: GameTime,
        completion_time: GameTime,
    },
}

impl QuestState {
    #[allow(dead_code)]
    pub fn is_inactive(&self) -> bool {
        matches!(self, QuestState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, QuestState::Active { .. })
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, QuestState::Completed { .. })
    }

    pub fn activation_time(&self) -> Option<GameTime> {
        match self {
            QuestState::Inactive => None,
            QuestState::Active { activation_time } => Some(*activation_time),
            QuestState::Completed {
                activation_time, ..
            } => Some(*activation_time),
        }
    }

    pub fn completion_time(&self) -> Option<GameTime> {
        match self {
            QuestState::Inactive => None,
            QuestState::Active { .. } => None,
            QuestState::Completed {
                completion_time, ..
            } => Some(*completion_time),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestStateMarker {
    Inactive,
    Active,
    Completed,
}

impl QuestStateMarker {
    pub fn is_inactive(&self) -> bool {
        matches!(self, QuestStateMarker::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, QuestStateMarker::Active { .. })
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, QuestStateMarker::Completed { .. })
    }
}

impl From<QuestState> for QuestStateMarker {
    fn from(state: QuestState) -> Self {
        match state {
            QuestState::Inactive => QuestStateMarker::Inactive,
            QuestState::Active { .. } => QuestStateMarker::Active,
            QuestState::Completed { .. } => QuestStateMarker::Completed,
        }
    }
}

impl From<usize> for QuestId {
    fn from(n: usize) -> Self {
        QuestId(n)
    }
}
