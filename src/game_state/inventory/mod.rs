use crate::game_state::currency::Currency;
use crate::game_state::inventory::item::{CompiledItem, ItemId, ItemState};
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use hashbag::HashBag;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter;

pub mod item;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<CompiledItem>,
    pub active_items: HashSet<ItemId>,
    owned: HashBag<ItemId>,

    pub currency: Currency,
}

impl Inventory {
    pub fn new(items: Vec<CompiledItem>) -> Self {
        Self {
            items,
            active_items: Default::default(),
            owned: Default::default(),

            currency: Currency::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn item(&self, item_id: ItemId) -> &CompiledItem {
        &self.items[item_id.0]
    }

    pub fn item_mut(&mut self, item_id: ItemId) -> &mut CompiledItem {
        &mut self.items[item_id.0]
    }

    pub fn add(
        &mut self,
        item_id: ItemId,
        count: usize,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let new_count = self.owned.insert_many(item_id, count);
        assert!(new_count >= count);
        if count > 0 {
            Some(CompiledGameEvent::ItemCountChanged {
                id: item_id,
                count: new_count,
            })
        } else {
            None
        }
        .into_iter()
    }

    /*/// Remove some items from the inventory.
    /// If more are supposed to be removed than there are, then that is ignored.
    pub fn remove(
        &mut self,
        item_id: ItemId,
        count: usize,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let old_count = self.owned.contains(&item_id);
        for _ in 0..count {
            self.owned.remove(&item_id);
        }
        let new_count = self.owned.contains(&item_id);

        if old_count != new_count {
            Some(CompiledGameEvent::ItemCountChanged {
                id: item_id,
                count: self.owned.contains(&item_id),
            })
        } else {
            None
        }
        .into_iter()
    }*/

    pub fn activate_item(
        &mut self,
        item_id: ItemId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let item = self.item_mut(item_id);
        assert!(item.state.is_inactive());
        item.state = ItemState::Active {
            activation_time: time,
        };
        assert!(self.active_items.insert(item_id));
        iter::empty()
    }

    pub fn deactivate_item(
        &mut self,
        item_id: ItemId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let item = self.item_mut(item_id);
        assert!(item.state.is_active());
        match item.state {
            ItemState::Active { activation_time } => {
                item.state = ItemState::Deactivated {
                    activation_time,
                    deactivation_time: time,
                };
                assert!(self.active_items.remove(&item_id));
            }
            _ => unreachable!(),
        }
        iter::empty()
    }
}
