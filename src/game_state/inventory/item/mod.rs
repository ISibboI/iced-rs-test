use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use crate::game_template::parser::ExpectedIdentifierCount;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id_str: String,
    pub name: String,
    pub description: String,
    pub value: Currency,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledItem {
    pub id: ItemId,
    pub id_str: String,
    pub state: ItemState,
    pub name: String,
    pub description: String,
    pub value: Currency,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpectedItemCount {
    pub id_str: String,
    pub mean: f64,
    pub variance: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledExpectedItemCount {
    pub id: ItemId,
    pub mean: f64,
    pub variance: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemCount {
    pub id: ItemId,
    pub count: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemState {
    Inactive,
    Active {
        activation_time: GameTime,
    },
    Deactivated {
        activation_time: GameTime,
        deactivation_time: GameTime,
    },
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct ItemId(pub usize);

impl Item {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledItem {
        CompiledItem {
            id: *id_maps.items.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            state: ItemState::Inactive,
            name: self.name,
            description: self.description,
            value: self.value,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation_condition: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
        }
    }
}

impl ExpectedItemCount {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledExpectedItemCount {
        CompiledExpectedItemCount {
            id: *id_maps.items.get(&self.id_str).unwrap(),
            mean: self.mean,
            variance: self.variance,
        }
    }
}

impl CompiledExpectedItemCount {
    pub fn spawn(&self, rng: &mut impl Rng) -> ItemCount {
        ItemCount {
            id: self.id,
            count: Normal::new(self.mean, self.variance)
                .unwrap()
                .sample(rng)
                .max(0.0)
                .round() as usize,
        }
    }
}

impl From<ExpectedIdentifierCount> for ExpectedItemCount {
    fn from(value: ExpectedIdentifierCount) -> Self {
        Self {
            id_str: value.identifier,
            mean: value.mean,
            variance: value.variance,
        }
    }
}

#[allow(dead_code)]
impl ItemState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, ItemState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ItemState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, ItemState::Deactivated { .. })
    }
}

impl From<usize> for ItemId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
