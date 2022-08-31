use crate::game_state::actions::{ActionInProgress, Actions};
use crate::game_state::story::quests::quest_conditions::QuestConditionEvaluation;
use crate::game_state::story::quests::{init_quests, CompiledQuest, QuestId};
use crate::game_state::time::GameTime;
use log::debug;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

pub mod io;
pub mod quests;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Story {
    pub inactive_quests: HashMap<QuestId, CompiledQuest>,
    pub active_quests: HashMap<QuestId, CompiledQuest>,
    pub active_quests_by_activation_time: BTreeSet<(GameTime, QuestId)>,
    pub completed_quests: HashMap<QuestId, CompiledQuest>,
    pub completed_quests_by_completion_time: BTreeSet<(GameTime, QuestId)>,
}

impl Story {
    pub fn new(actions: &Actions) -> Self {
        let mut result = Self {
            inactive_quests: init_quests(actions),
            active_quests: Default::default(),
            active_quests_by_activation_time: Default::default(),
            completed_quests: Default::default(),
            completed_quests_by_completion_time: Default::default(),
        };
        let all_quests: Vec<_> = result.inactive_quests.keys().copied().collect();
        result.update_activation_and_completion(
            GameTime::default(),
            Default::default(),
            Default::default(),
            all_quests.clone(),
            all_quests,
        );
        result
    }

    /// Check the given quests for activation.
    fn activate_quests(
        &mut self,
        quests: impl IntoIterator<Item = QuestId>,
        activation_time: GameTime,
    ) {
        quests.into_iter().for_each(|id| {
            let quest = self.inactive_quests.remove(&id).unwrap();
            assert!(quest.state.is_active() || quest.state.is_completed());
            debug!("Activated quest {}", quest.id_str);
            self.active_quests.insert(id, quest);
            self.active_quests_by_activation_time
                .insert((activation_time, id));
        });
    }

    /// Check the given quests for completion.
    fn complete_quests(
        &mut self,
        quests: impl IntoIterator<Item = QuestId>,
        completion_time: GameTime,
    ) {
        quests.into_iter().for_each(|id| {
            let quest = self.active_quests.remove(&id).unwrap();
            self.active_quests_by_activation_time
                .remove(&(quest.state.activation_time().unwrap(), id));
            assert!(quest.state.is_completed());
            debug!("Completed quest {}", quest.id_str);
            self.completed_quests.insert(id, quest);
            self.completed_quests_by_completion_time
                .insert((completion_time, id));
        });
    }

    pub fn update(&mut self, action_in_progress: &ActionInProgress) {
        assert!(action_in_progress.success);
        let time = action_in_progress.end;
        let mut activable_quests_by_state_change = Vec::new();
        let mut completable_quests_by_state_change = Vec::new();

        let activated_quests: HashSet<_> = self
            .inactive_quests
            .values_mut()
            .filter_map(|quest| {
                let evaluation = quest.update_action_completed(action_in_progress);
                if evaluation == QuestConditionEvaluation::True {
                    Some(quest.id)
                } else if evaluation == QuestConditionEvaluation::FulfillableByQuestStateChanges {
                    activable_quests_by_state_change.push(quest.id);
                    None
                } else {
                    None
                }
            })
            .collect();

        let completed_quests: HashSet<_> = self
            .active_quests
            .values_mut()
            .filter_map(|quest| {
                let evaluation = quest.update_action_completed(action_in_progress);
                if evaluation == QuestConditionEvaluation::True {
                    Some(quest.id)
                } else if evaluation == QuestConditionEvaluation::FulfillableByQuestStateChanges {
                    completable_quests_by_state_change.push(quest.id);
                    None
                } else {
                    None
                }
            })
            .collect();

        self.update_activation_and_completion(
            time,
            activated_quests,
            completed_quests,
            activable_quests_by_state_change,
            completable_quests_by_state_change,
        );
    }

    fn update_activation_and_completion(
        &mut self,
        time: GameTime,
        mut activated_quests: HashSet<QuestId>,
        mut completed_quests: HashSet<QuestId>,
        mut activable_quests_by_state_change: Vec<QuestId>,
        mut completable_quests_by_state_change: Vec<QuestId>,
    ) {
        while !activated_quests.is_empty() || !completed_quests.is_empty() {
            self.activate_quests(activated_quests.iter().copied(), time);
            self.complete_quests(completed_quests.iter().copied(), time);

            let old_activated_quests = activated_quests;
            let old_completed_quests = completed_quests;
            let old_activable_quests_by_state_change = activable_quests_by_state_change;
            let old_completable_quests_by_state_change = completable_quests_by_state_change;
            activable_quests_by_state_change = Vec::new();
            completable_quests_by_state_change = Vec::new();

            activated_quests = old_activable_quests_by_state_change
                .into_iter()
                .filter(|id| {
                    let inactive_quest = self.inactive_quests.get_mut(id).unwrap();
                    inactive_quest.activate_quests(&old_activated_quests, time);
                    let evaluation = inactive_quest.complete_quests(&old_completed_quests, time);
                    if evaluation == QuestConditionEvaluation::True {
                        true
                    } else if evaluation == QuestConditionEvaluation::FulfillableByQuestStateChanges
                    {
                        activable_quests_by_state_change.push(*id);
                        false
                    } else {
                        false
                    }
                })
                .collect();

            completed_quests = old_completable_quests_by_state_change
                .into_iter()
                .filter(|id| {
                    let active_quest = self.active_quests.get_mut(id).unwrap();
                    active_quest.activate_quests(&old_activated_quests, time);
                    let evaluation = active_quest.complete_quests(&old_completed_quests, time);
                    if evaluation == QuestConditionEvaluation::True {
                        true
                    } else if evaluation == QuestConditionEvaluation::FulfillableByQuestStateChanges
                    {
                        completable_quests_by_state_change.push(*id);
                        false
                    } else {
                        false
                    }
                })
                .collect();
        }
    }
}
