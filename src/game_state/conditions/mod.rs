#![allow(dead_code)]

use crate::game_state::actions::{ActionId, ActionInProgress};
use crate::game_state::story::quests::{QuestId, QuestStateMarker};
use crate::game_state::time::GameTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
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

    QuestInactive {
        quest: String,
    },
    QuestActive {
        quest: String,
    },
    QuestCompleted {
        quest: String,
    },

    And {
        conditions: Vec<Condition>,
    },
    Or {
        conditions: Vec<Condition>,
    },
    AnyN {
        conditions: Vec<Condition>,
        n: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CompiledCondition {
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
        conditions: Vec<CompiledCondition>,
    },
    Or {
        conditions: Vec<CompiledCondition>,
    },
    AnyN {
        conditions: Vec<CompiledCondition>,
        n: usize,
    },
}

impl Condition {
    pub fn compile(
        self,
        quest_id_map: &HashMap<String, QuestId>,
        action_id_map: &HashMap<String, ActionId>,
    ) -> CompiledCondition {
        match self {
            Condition::None => CompiledCondition::None,
            Condition::ActionIs { action } => CompiledCondition::ActionIs {
                action: *action_id_map.get(&action).unwrap(),
                fulfilled: false,
            },
            Condition::ActionIsNot { action } => CompiledCondition::ActionIsNot {
                action: *action_id_map.get(&action).unwrap(),
                fulfilled: false,
            },
            Condition::ActionCount { action, required } => CompiledCondition::ActionCount {
                action: *action_id_map.get(&action).unwrap(),
                required,
                current: 0,
            },
            Condition::TimeGeq { time } => CompiledCondition::TimeGeq {
                time,
                fulfilled: false,
            },
            Condition::QuestInactive { quest } => CompiledCondition::Inactive {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            Condition::QuestActive { quest } => CompiledCondition::Active {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            Condition::QuestCompleted { quest } => CompiledCondition::Completed {
                quest: *quest_id_map.get(&quest).unwrap(),
                state: QuestStateMarker::Inactive,
            },
            Condition::And { conditions } => CompiledCondition::And {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
            },
            Condition::Or { conditions } => CompiledCondition::Or {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
            },
            Condition::AnyN { conditions, n } => CompiledCondition::AnyN {
                conditions: conditions
                    .into_iter()
                    .map(|condition| condition.compile(quest_id_map, action_id_map))
                    .collect(),
                n,
            },
        }
    }
}

impl CompiledCondition {
    pub fn update_action_completed(
        &mut self,
        action_in_progress: &ActionInProgress,
    ) -> QuestConditionEvaluation {
        match self {
            CompiledCondition::None => QuestConditionEvaluation::True,
            CompiledCondition::ActionIs { action, fulfilled } => {
                if action_in_progress.action == *action {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledCondition::ActionIsNot { action, fulfilled } => {
                if action_in_progress.action != *action {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledCondition::ActionCount {
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
            CompiledCondition::TimeGeq { time, fulfilled } => {
                if action_in_progress.end >= *time {
                    *fulfilled = true;
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::False
                }
            }
            CompiledCondition::Inactive { state, .. } => {
                if state.is_inactive() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledCondition::Active { state, .. } => {
                if state.is_active() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledCondition::Completed { state, .. } => {
                if state.is_completed() {
                    QuestConditionEvaluation::True
                } else {
                    QuestConditionEvaluation::FulfillableByQuestStateChanges
                }
            }
            CompiledCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.update_action_completed(action_in_progress)
                }),
            CompiledCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.update_action_completed(action_in_progress)
                }),
            CompiledCondition::AnyN { conditions, n } => {
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
            CompiledCondition::None => QuestConditionEvaluation::True,
            CompiledCondition::ActionIs { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::ActionIsNot { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::ActionCount {
                current, required, ..
            } => (current >= required).into(),
            CompiledCondition::TimeGeq { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::Inactive { quest, state } => {
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
            CompiledCondition::Active { quest, state } => {
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
            CompiledCondition::Completed { quest, state } => {
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
            CompiledCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.activate_quests(activated_quests)
                }),
            CompiledCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.activate_quests(activated_quests)
                }),
            CompiledCondition::AnyN { conditions, n } => {
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
            CompiledCondition::None => QuestConditionEvaluation::True,
            CompiledCondition::ActionIs { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::ActionIsNot { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::ActionCount {
                current, required, ..
            } => (current >= required).into(),
            CompiledCondition::TimeGeq { fulfilled, .. } => fulfilled.into(),
            CompiledCondition::Inactive { quest, state } => {
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
            CompiledCondition::Active { quest, state } => {
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
            CompiledCondition::Completed { quest, state } => {
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
            CompiledCondition::And { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::True, |result, condition| {
                    result & condition.complete_quests(completed_quests)
                }),
            CompiledCondition::Or { conditions } => conditions
                .iter_mut()
                .fold(QuestConditionEvaluation::False, |result, condition| {
                    result | condition.complete_quests(completed_quests)
                }),
            CompiledCondition::AnyN { conditions, n } => {
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
            CompiledCondition::None => (0.0, 0.0),
            CompiledCondition::ActionIs { fulfilled, .. } => (bool_to_one_zero(*fulfilled), 1.0),
            CompiledCondition::ActionIsNot { fulfilled, .. } => (bool_to_one_zero(*fulfilled), 1.0),
            CompiledCondition::ActionCount {
                current, required, ..
            } => ((*current.min(required)) as f64, (*required) as f64),
            CompiledCondition::TimeGeq { fulfilled, .. } => (bool_to_one_zero(*fulfilled), 1.0),
            CompiledCondition::Inactive { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Inactive), 1.0)
            }
            CompiledCondition::Active { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Active), 1.0)
            }
            CompiledCondition::Completed { state, .. } => {
                (bool_to_one_zero(*state == QuestStateMarker::Completed), 1.0)
            }
            CompiledCondition::And { conditions } => {
                conditions
                    .iter()
                    .fold((0.0, 0.0), |(progress, goal), condition| {
                        let (additional_progress, additional_goal) = condition.progress();
                        (progress + additional_progress, goal + additional_goal)
                    })
            }
            CompiledCondition::Or { conditions } => {
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
            CompiledCondition::AnyN { conditions, n } => {
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

pub fn none() -> Condition {
    Condition::None
}

pub fn action_is(action: impl ToString) -> Condition {
    Condition::ActionIs {
        action: action.to_string(),
    }
}

pub fn action_is_not(action: impl ToString) -> Condition {
    Condition::ActionIsNot {
        action: action.to_string(),
    }
}

pub fn action_count(action: impl ToString, count: usize) -> Condition {
    Condition::ActionCount {
        action: action.to_string(),
        required: count,
    }
}

pub fn time_geq(time: GameTime) -> Condition {
    Condition::TimeGeq { time }
}

pub fn quest_inactive(quest: impl ToString) -> Condition {
    Condition::QuestInactive {
        quest: quest.to_string(),
    }
}

pub fn quest_active(quest: impl ToString) -> Condition {
    Condition::QuestActive {
        quest: quest.to_string(),
    }
}

pub fn quest_completed(quest: impl ToString) -> Condition {
    Condition::QuestCompleted {
        quest: quest.to_string(),
    }
}

pub fn any_n(conditions: impl AsRef<[Condition]>, n: usize) -> Condition {
    Condition::AnyN {
        conditions: conditions.as_ref().into(),
        n,
    }
}

impl BitAnd for Condition {
    type Output = Condition;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Condition::And {
                    conditions: mut conditions_self,
                },
                Condition::And {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_self.append(&mut conditions_rhs);
                Self::And {
                    conditions: conditions_self,
                }
            }
            (
                Condition::And {
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
                Condition::And {
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

impl BitOr for Condition {
    type Output = Condition;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Condition::Or {
                    conditions: mut conditions_self,
                },
                Condition::Or {
                    conditions: mut conditions_rhs,
                },
            ) => {
                conditions_self.append(&mut conditions_rhs);
                Self::Or {
                    conditions: conditions_self,
                }
            }
            (
                Condition::Or {
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
                Condition::Or {
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
