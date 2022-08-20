#![allow(dead_code)]

use crate::game_state::actions::ActionInProgress;
use crate::game_state::time::GameTime;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestCondition {
    ActionIs(String),
    ActionIsNot(String),
    ActionCount {
        action: String,
        current: usize,
        required: usize,
    },

    TimeGeq(GameTime),

    And(Vec<QuestCondition>),
    Or(Vec<QuestCondition>),
}

impl QuestCondition {
    pub fn update(&mut self, action_in_progress: &ActionInProgress) -> bool {
        match self {
            QuestCondition::ActionIs(action) => action_in_progress.action.name == *action,
            QuestCondition::ActionIsNot(action) => action_in_progress.action.name != *action,
            QuestCondition::ActionCount {
                action,
                current,
                required,
            } => {
                if action_in_progress.action.name == *action {
                    *current += 1;
                }
                current >= required
            }
            QuestCondition::TimeGeq(time) => action_in_progress.end >= *time,
            QuestCondition::And(conditions) => conditions
                .iter_mut()
                .all(|condition| condition.update(action_in_progress)),
            QuestCondition::Or(conditions) => conditions
                .iter_mut()
                .any(|condition| condition.update(action_in_progress)),
        }
    }
}

pub fn action_is(action: impl ToString) -> QuestCondition {
    QuestCondition::ActionIs(action.to_string())
}

pub fn action_is_not(action: impl ToString) -> QuestCondition {
    QuestCondition::ActionIsNot(action.to_string())
}

pub fn action_count(action: impl ToString, count: usize) -> QuestCondition {
    QuestCondition::ActionCount {
        action: action.to_string(),
        current: 0,
        required: count,
    }
}

pub fn time_geq(time: GameTime) -> QuestCondition {
    QuestCondition::TimeGeq(time)
}

impl BitAnd for QuestCondition {
    type Output = QuestCondition;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (QuestCondition::And(mut conditions_self), QuestCondition::And(mut conditions_rhs)) => {
                conditions_self.append(&mut conditions_rhs);
                Self::And(conditions_self)
            }
            (QuestCondition::And(mut conditions_self), rhs) => {
                conditions_self.push(rhs);
                Self::And(conditions_self)
            }
            (lhs, QuestCondition::And(mut conditions_rhs)) => {
                conditions_rhs.push(lhs);
                Self::And(conditions_rhs)
            }
            (lhs, rhs) => Self::And(vec![lhs, rhs]),
        }
    }
}

impl BitOr for QuestCondition {
    type Output = QuestCondition;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (QuestCondition::Or(mut conditions_self), QuestCondition::Or(mut conditions_rhs)) => {
                conditions_self.append(&mut conditions_rhs);
                Self::Or(conditions_self)
            }
            (QuestCondition::Or(mut conditions_self), rhs) => {
                conditions_self.push(rhs);
                Self::Or(conditions_self)
            }
            (lhs, QuestCondition::Or(mut conditions_rhs)) => {
                conditions_rhs.push(lhs);
                Self::Or(conditions_rhs)
            }
            (lhs, rhs) => Self::Or(vec![lhs, rhs]),
        }
    }
}
