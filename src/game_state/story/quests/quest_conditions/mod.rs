#![allow(dead_code)]

use crate::game_state::actions::{ActionId, ActionInProgress};
use crate::game_state::story::quests::{QuestId, QuestStateMarker};
use crate::game_state::time::GameTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestCondition {
    None,
    ActionIs {
        action: String,
    },
    ActionIsNot {
        action: String,
    },
    ActionCount {
        action: String,
        required: usize,
    },

    TimeGeq {
        time: GameTime,
    },

    Inactive {
        quest: String,
    },
    Active {
        quest: String,
    },
    Completed {
        quest: String,
    },

    And {
        conditions: Vec<QuestCondition>,
    },
    Or {
        conditions: Vec<QuestCondition>,
    },
    AnyN {
        conditions: Vec<QuestCondition>,
        n: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CompiledQuestCondition {
    None,
    ActionIs {
        action: ActionId,
        fulfilled: bool,
    },
    ActionIsNot {
        action: ActionId,
        fulfilled: bool,
    },
    ActionCount {
        action: ActionId,
        current: usize,
        required: usize,
    },

    TimeGeq {
        time: GameTime,
        fulfilled: bool,
    },

    Inactive {
        quest: QuestId,
        state: QuestStateMarker,
    },
    Active {
        quest: QuestId,
        state: QuestStateMarker,
    },
    Completed {
        quest: QuestId,
        state: QuestStateMarker,
    },

    And {
        conditions: Vec<CompiledQuestCondition>,
    },
    Or {
        conditions: Vec<CompiledQuestCondition>,
    },
    AnyN {
        conditions: Vec<CompiledQuestCondition>,
        n: usize,
    },
}

impl QuestCondition {
    pub fn compile(
        self,
        quest_id_map: &HashMap<String, QuestId>,
        action_id_map: &HashMap<String, ActionId>,
    ) -> CompiledQuestCondition {
        match self {
            QuestCondition::None => CompiledQuestCondition::None,
            QuestCondition::ActionIs { action } => CompiledQuestCondition::ActionIs {
                action: *action_id_map.get(&action).unwrap(),
                fulfilled: false,
            },
            QuestCondition::ActionIsNot { action } => CompiledQuestCondition::ActionIsNot {
                action: *action_id_map.get(&action).unwrap(),
                fulfilled: false,
            },
            QuestCondition::ActionCount { action, required } => {
                CompiledQuestCondition::ActionCount {
                    action: *action_id_map.get(&action).unwrap(),
                    required,
                    current: 0,
                }
            }
            QuestCondition::TimeGeq { time } => CompiledQuestCondition::TimeGeq {
                time,
                fulfilled: false,
            },
            QuestCondition::Inactive { quest } => CompiledQuestCondition::Inactive {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            QuestCondition::Active { quest } => CompiledQuestCondition::Active {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            QuestCondition::Completed { quest } => CompiledQuestCondition::Completed {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            QuestCondition::And { conditions } => CompiledQuestCondition::And {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
            },
            QuestCondition::Or { conditions } => CompiledQuestCondition::Or {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
            },
            QuestCondition::AnyN { conditions, n } => CompiledQuestCondition::AnyN {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
                n,
            },
        }
    }
}

impl CompiledQuestCondition {
    pub fn update_action_completed(
        &mut self,
        action_in_progress: &ActionInProgress,
    ) -> QuestConditionEvaluation {
        match self {
            CompiledQuestCondition::None => QuestConditionEvaluation::True,
            CompiledQuestCondition::ActionIs { action, fulfilled } => {
                if action_in_progress.action == *action {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledQuestCondition::ActionIsNot { action, fulfilled } => {
                if action_in_progress.action != *action {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledQuestCondition::ActionCount {
                action,
                current,
                required,
            } => {
                if action_in_progress.action == *action {
                    *current += 1;
                }
                if current >= required {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledQuestCondition::TimeGeq { time, fulfilled } => {
                if action_in_progress.end >= *time {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledQuestCondition::Inactive { state, .. } => {
                if state.is_inactive() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledQuestCondition::Active { state, .. } => {
                if state.is_active() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledQuestCondition::Completed { state, .. } => {
                if state.is_completed() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledQuestCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.update_action_completed(action_in_progress)
                }),
            CompiledQuestCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.update_action_completed(action_in_progress)
                }),
            CompiledQuestCondition::AnyN { conditions, n } => {
                let counts = conditions.iter_mut().fold(
                    QuestConditionEvaluationCounts::default(),
                    |mut counts, condition| {
                        counts.increment(condition.update_action_completed(action_in_progress));
                        counts
                    },
                );
                counts.evaluate_any_n(*n)
            }
        }
    }

    pub fn activate_quests(
        &mut self,
        activated_quests: &HashSet<QuestId>,
    ) -> QuestConditionEvaluation {
        match self {
            CompiledQuestCondition::None => QuestConditionEvaluation::True,
            CompiledQuestCondition::ActionIs { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::ActionIsNot { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::ActionCount {
                current, required, ..
            } => (current >= required).into(),
            CompiledQuestCondition::TimeGeq { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::Inactive { quest, state } => {
                if activated_quests.contains(quest) {
                    assert_eq!(*state, QuestStateMarker::Inactive);
                    *state = QuestStateMarker::Active;
                }
                match *state {
                    QuestStateMarker::Inactive => QuestConditionEvaluation::True,
                    QuestStateMarker::Active => QuestConditionEvaluation::False,
                    QuestStateMarker::Completed => QuestConditionEvaluation::False,
                }
            }
            CompiledQuestCondition::Active { quest, state } => {
                if activated_quests.contains(quest) {
                    assert_eq!(*state, QuestStateMarker::Inactive);
                    *state = QuestStateMarker::Active;
                }
                match *state {
                    QuestStateMarker::Inactive => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Active => QuestConditionEvaluation::True,
                    QuestStateMarker::Completed => QuestConditionEvaluation::False,
                }
            }
            CompiledQuestCondition::Completed { quest, state } => {
                if activated_quests.contains(quest) {
                    assert_eq!(*state, QuestStateMarker::Inactive);
                    *state = QuestStateMarker::Active;
                }
                match *state {
                    QuestStateMarker::Inactive => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Active => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Completed => QuestConditionEvaluation::True,
                }
            }
            CompiledQuestCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.activate_quests(activated_quests)
                }),
            CompiledQuestCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.activate_quests(activated_quests)
                }),
            CompiledQuestCondition::AnyN { conditions, n } => {
                let counts = conditions.iter_mut().fold(
                    QuestConditionEvaluationCounts::default(),
                    |mut counts, condition| {
                        counts.increment(condition.activate_quests(activated_quests));
                        counts
                    },
                );
                counts.evaluate_any_n(*n)
            }
        }
    }

    pub fn complete_quests(
        &mut self,
        completed_quests: &HashSet<QuestId>,
    ) -> QuestConditionEvaluation {
        match self {
            CompiledQuestCondition::None => QuestConditionEvaluation::True,
            CompiledQuestCondition::ActionIs { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::ActionIsNot { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::ActionCount {
                current, required, ..
            } => (current >= required).into(),
            CompiledQuestCondition::TimeGeq { fulfilled, .. } => fulfilled.into(),
            CompiledQuestCondition::Inactive { quest, state } => {
                if completed_quests.contains(quest) {
                    assert!(
                        *state == QuestStateMarker::Inactive || *state == QuestStateMarker::Active
                    );
                    *state = QuestStateMarker::Completed;
                }
                match *state {
                    QuestStateMarker::Inactive => QuestConditionEvaluation::True,
                    QuestStateMarker::Active => QuestConditionEvaluation::False,
                    QuestStateMarker::Completed => QuestConditionEvaluation::False,
                }
            }
            CompiledQuestCondition::Active { quest, state } => {
                if completed_quests.contains(quest) {
                    assert!(
                        *state == QuestStateMarker::Inactive || *state == QuestStateMarker::Active
                    );
                    *state = QuestStateMarker::Completed;
                }
                match *state {
                    QuestStateMarker::Inactive => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Active => QuestConditionEvaluation::True,
                    QuestStateMarker::Completed => QuestConditionEvaluation::False,
                }
            }
            CompiledQuestCondition::Completed { quest, state } => {
                if completed_quests.contains(quest) {
                    assert!(
                        *state == QuestStateMarker::Inactive || *state == QuestStateMarker::Active
                    );
                    *state = QuestStateMarker::Completed;
                }
                match *state {
                    QuestStateMarker::Inactive => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Active => {
                        QuestConditionEvaluation::FulfillableByQuestStateChanges
                    }
                    QuestStateMarker::Completed => QuestConditionEvaluation::True,
                }
            }
            CompiledQuestCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.complete_quests(completed_quests)
                }),
            CompiledQuestCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.complete_quests(completed_quests)
                }),
            CompiledQuestCondition::AnyN { conditions, n } => {
                let counts = conditions.iter_mut().fold(
                    QuestConditionEvaluationCounts::default(),
                    |mut counts, condition| {
                        counts.increment(condition.complete_quests(completed_quests));
                        counts
                    },
                );
                counts.evaluate_any_n(*n)
            }
        }
    }

    pub fn progress(&self) -> (f64, f64) {
        match self {
            CompiledQuestCondition::None => (0.0, 0.0),
            CompiledQuestCondition::ActionIs { fulfilled, .. } => {
                (bool_to_one_zero(*fulfilled), 1.0)
            }
            CompiledQuestCondition::ActionIsNot { fulfilled, .. } => {
                (bool_to_one_zero(*fulfilled), 1.0)
            }
            CompiledQuestCondition::ActionCount {
                current, required, ..
            } => ((*current.min(required)) as f64, (*required) as f64),
            CompiledQuestCondition::TimeGeq { fulfilled, .. } => {
                (bool_to_one_zero(*fulfilled), 1.0)
            }
            CompiledQuestCondition::Inactive { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Inactive), 1.0)
            }
            CompiledQuestCondition::Active { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Active), 1.0)
            }
            CompiledQuestCondition::Completed { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Completed), 1.0)
            }
            CompiledQuestCondition::And { conditions } => {
                conditions
                    .iter()
                    .fold((0.0, 0.0), |(progress, goal), condition| {
                        let (additional_progress, additional_goal) = condition.progress();
                        (progress + additional_progress, goal + additional_goal)
                    })
            }
            CompiledQuestCondition::Or { conditions } => {
                if conditions.is_empty() {
                    (0.0, 0.0)
                } else {
                    let (relative_progress, goal) = conditions.iter().fold(
                        (f64::MIN, f64::MAX),
                        |(relative_progress, goal), condition| {
                            let (additional_progress, additional_goal) = condition.progress();
                            let additional_relative_progress =
                                additional_progress / additional_goal;
                            (
                                relative_progress.max(additional_relative_progress),
                                goal.min(additional_goal),
                            )
                        },
                    );
                    (relative_progress * goal, goal)
                }
            }
            CompiledQuestCondition::AnyN { conditions, n } => {
                if conditions.is_empty() || *n > conditions.len() {
                    (0.0, 0.0)
                } else {
                    let mut relative_progresses = Vec::with_capacity(conditions.len());
                    let mut goals = Vec::with_capacity(conditions.len());
                    for condition in conditions {
                        let (progress, goal) = condition.progress();
                        relative_progresses.push(progress / goal);
                        goals.push(goal);
                    }
                    relative_progresses.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    goals.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    let goal = goals.iter().take(*n).sum();
                    let progress =
                        relative_progresses.iter().rev().take(*n).sum::<f64>() / (*n as f64) * goal;
                    (progress, goal)
                }
            }
        }
    }

    pub fn fulfilled(&self) -> bool {
        let (a, b) = self.progress();
        a == b
    }
}

pub fn none() -> QuestCondition {
    QuestCondition::None
}

pub fn action_is(action: impl ToString) -> QuestCondition {
    QuestCondition::ActionIs {
        action: action.to_string(),
    }
}

pub fn action_is_not(action: impl ToString) -> QuestCondition {
    QuestCondition::ActionIsNot {
        action: action.to_string(),
    }
}

pub fn action_count(action: impl ToString, count: usize) -> QuestCondition {
    QuestCondition::ActionCount {
        action: action.to_string(),
        required: count,
    }
}

pub fn time_geq(time: GameTime) -> QuestCondition {
    QuestCondition::TimeGeq { time }
}

pub fn inactive(quest: impl ToString) -> QuestCondition {
    QuestCondition::Inactive {
        quest: quest.to_string(),
    }
}

pub fn active(quest: impl ToString) -> QuestCondition {
    QuestCondition::Active {
        quest: quest.to_string(),
    }
}

pub fn completed(quest: impl ToString) -> QuestCondition {
    QuestCondition::Completed {
        quest: quest.to_string(),
    }
}

pub fn any_n(conditions: impl AsRef<[QuestCondition]>, n: usize) -> QuestCondition {
    QuestCondition::AnyN {
        conditions: conditions.as_ref().into(),
        n,
    }
}

impl BitAnd for QuestCondition {
    type Output = QuestCondition;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                QuestCondition::And {
                    conditions: mut conditions_self,
                },
                QuestCondition::And {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_self.append(&mut conditions_rhs);
                Self::And {
                    conditions: conditions_self,
                }
            }
            (
                QuestCondition::And {
                    conditions: mut conditions_self,
                },
                rhs,
            ) => {
                conditions_self.push(rhs);
                Self::And {
                    conditions: conditions_self,
                }
            }
            (
                lhs,
                QuestCondition::And {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_rhs.push(lhs);
                Self::And {
                    conditions: conditions_rhs,
                }
            }
            (lhs, rhs) => Self::And {
                conditions: vec![lhs, rhs],
            },
        }
    }
}

impl BitOr for QuestCondition {
    type Output = QuestCondition;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                QuestCondition::Or {
                    conditions: mut conditions_self,
                },
                QuestCondition::Or {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_self.append(&mut conditions_rhs);
                Self::Or {
                    conditions: conditions_self,
                }
            }
            (
                QuestCondition::Or {
                    conditions: mut conditions_self,
                },
                rhs,
            ) => {
                conditions_self.push(rhs);
                Self::Or {
                    conditions: conditions_self,
                }
            }
            (
                lhs,
                QuestCondition::Or {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_rhs.push(lhs);
                Self::Or {
                    conditions: conditions_rhs,
                }
            }
            (lhs, rhs) => Self::Or {
                conditions: vec![lhs, rhs],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestConditionEvaluation {
    /// The quest condition evaluates to false.
    False,
    /// The quest condition evaluates to true.
    True,
    /// The quest condition evaluates to false, but could become true if other quests change their [QuestState](super::QuestState).
    FulfillableByQuestStateChanges,
}

impl BitAnd for QuestConditionEvaluation {
    type Output = QuestConditionEvaluation;

    fn bitand(self, rhs: Self) -> Self::Output {
        use QuestConditionEvaluation::*;
        match (self, rhs) {
            (True, False) | (False, True) => False,
            (True, FulfillableByQuestStateChanges) | (FulfillableByQuestStateChanges, True) => {
                FulfillableByQuestStateChanges
            }
            (False, FulfillableByQuestStateChanges) | (FulfillableByQuestStateChanges, False) => {
                False
            }
            (True, True) => True,
            (False, False) => False,
            (FulfillableByQuestStateChanges, FulfillableByQuestStateChanges) => {
                FulfillableByQuestStateChanges
            }
        }
    }
}

impl BitOr for QuestConditionEvaluation {
    type Output = QuestConditionEvaluation;

    fn bitor(self, rhs: Self) -> Self::Output {
        use QuestConditionEvaluation::*;
        match (self, rhs) {
            (True, False) | (False, True) => True,
            (True, FulfillableByQuestStateChanges) | (FulfillableByQuestStateChanges, True) => True,
            (False, FulfillableByQuestStateChanges) | (FulfillableByQuestStateChanges, False) => {
                FulfillableByQuestStateChanges
            }
            (True, True) => True,
            (False, False) => False,
            (FulfillableByQuestStateChanges, FulfillableByQuestStateChanges) => {
                FulfillableByQuestStateChanges
            }
        }
    }
}

#[derive(Default, Debug)]
struct QuestConditionEvaluationCounts {
    fulfilled: usize,
    unfulfilled: usize,
    fulfillable: usize,
}

impl QuestConditionEvaluationCounts {
    fn increment(&mut self, evaluation: QuestConditionEvaluation) {
        match evaluation {
            QuestConditionEvaluation::False => self.unfulfilled += 1,
            QuestConditionEvaluation::True => self.fulfilled += 1,
            QuestConditionEvaluation::FulfillableByQuestStateChanges => self.fulfillable += 1,
        }
    }

    fn get(&self, evaluation: QuestConditionEvaluation) -> usize {
        match evaluation {
            QuestConditionEvaluation::False => self.unfulfilled,
            QuestConditionEvaluation::True => self.fulfilled,
            QuestConditionEvaluation::FulfillableByQuestStateChanges => self.fulfillable,
        }
    }

    fn evaluate_any_n(&self, n: usize) -> QuestConditionEvaluation {
        if self.fulfilled >= n {
            QuestConditionEvaluation::True
        } else if self.fulfilled + self.fulfillable >= n {
            QuestConditionEvaluation::FulfillableByQuestStateChanges
        } else {
            QuestConditionEvaluation::False
        }
    }
}

impl From<bool> for QuestConditionEvaluation {
    fn from(value: bool) -> Self {
        if value {
            QuestConditionEvaluation::True
        } else {
            QuestConditionEvaluation::False
        }
    }
}

impl From<&bool> for QuestConditionEvaluation {
    fn from(value: &bool) -> Self {
        if *value {
            QuestConditionEvaluation::True
        } else {
            QuestConditionEvaluation::False
        }
    }
}

impl From<&mut bool> for QuestConditionEvaluation {
    fn from(value: &mut bool) -> Self {
        if *value {
            QuestConditionEvaluation::True
        } else {
            QuestConditionEvaluation::False
        }
    }
}

fn bool_to_one_zero(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}
