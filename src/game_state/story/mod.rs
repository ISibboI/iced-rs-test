use crate::game_state::currency::Currency;
use crate::game_state::inventory::Inventory;
use crate::game_state::story::quests::{CompiledQuest, QuestId};
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use log::debug;
use quests::quest_stages::QuestStageId;
use rand::Rng;
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
    failed_quests: HashSet<QuestId>,
    failed_quests_by_failure_time: BTreeSet<(GameTime, QuestId)>,
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
            failed_quests: Default::default(),
            failed_quests_by_failure_time: Default::default(),
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

    pub fn iter_failed_quests_by_failure_time(
        &self,
    ) -> impl Iterator<Item = &'_ CompiledQuest> + DoubleEndedIterator {
        self.failed_quests_by_failure_time
            .iter()
            .map(|(_, quest_id)| self.quest(*quest_id))
    }

    pub fn iter_all_quests(&self) -> impl Iterator<Item = &'_ CompiledQuest> + DoubleEndedIterator {
        self.quests.iter()
    }

    pub fn activate_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest = self.quest_mut(quest_id);
        debug!("Activating quest {}", quest.id_str);
        if !quest.state().is_failed() {
            let result = quest.activate(time);
            assert!(self.inactive_quests.remove(&quest_id));
            assert!(self.active_quests.insert(quest_id));
            assert!(self
                .active_quests_by_activation_time
                .insert((time, quest_id)));
            Some(result)
        } else {
            None
        }
        .into_iter()
        .flatten()
    }

    pub fn complete_quest_stage(
        &mut self,
        rng: &mut impl Rng,
        inventory: &mut Inventory,
        quest_stage_id: QuestStageId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest_id = quest_stage_id.quest_id;
        let quest = self.quests.get_mut(quest_stage_id.quest_id.0).unwrap();
        let result = if !quest.state().is_failed() {
            let result = quest.complete_quest_stage(
                rng,
                inventory,
                quest_stage_id,
                time,
                |quest, rng, inventory, activation_time| {
                    assert!(self.active_quests.remove(&quest_id));
                    assert!(self
                        .active_quests_by_activation_time
                        .remove(&(activation_time, quest_id)));
                    assert!(self.completed_quests.insert(quest_id));
                    assert!(self
                        .completed_quests_by_completion_time
                        .insert((time, quest_id)));

                    let currency_change_event = if quest.currency_reward > Currency::zero() {
                        inventory.currency += quest.currency_reward;
                        Some(CompiledGameEvent::CurrencyChanged {
                            value: inventory.currency,
                        })
                    } else {
                        Default::default()
                    }
                    .into_iter();
                    let mut stage_item_change_events = Vec::new();
                    for item in quest.items.iter() {
                        let count = item.spawn(rng);
                        if count.count > 0 {
                            stage_item_change_events.extend(inventory.add(count.id, count.count));
                        }
                    }
                    let item_change_events = stage_item_change_events.into_iter();

                    Box::new(currency_change_event.chain(item_change_events))
                },
            );
            Some(result)
        } else {
            None
        };
        result.into_iter().flatten()
    }

    pub fn fail_quest(
        &mut self,
        quest_id: QuestId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let quest = self.quests.get_mut(quest_id.0).unwrap();
        debug!("Failing quest {}", quest.id_str);
        quest.fail(time, |activation_time| {
            if let Some(activation_time) = activation_time {
                assert!(self.active_quests.remove(&quest_id));
                assert!(self
                    .active_quests_by_activation_time
                    .remove(&(activation_time, quest_id)));
                assert!(self.active_failed_quests.insert(quest_id));
                assert!(self
                    .active_failed_quests_by_failure_time
                    .insert((time, quest_id)));
                assert!(self.failed_quests.insert(quest_id));
                assert!(self.failed_quests_by_failure_time.insert((time, quest_id)));
            } else {
                assert!(self.inactive_quests.remove(&quest_id));
                assert!(self.inactive_failed_quests.insert(quest_id));
                assert!(self
                    .inactive_failed_quests_by_failure_time
                    .insert((time, quest_id)));
                assert!(self.failed_quests.insert(quest_id));
                assert!(self.failed_quests_by_failure_time.insert((time, quest_id)));
            }
        })
    }
}
