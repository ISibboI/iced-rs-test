use crate::game_template::parser::ExpectedIdentifierCount;
use crate::game_template::IdMaps;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id_str: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledItem {
    pub id: ItemId,
    pub id_str: String,
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

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct ItemId(pub usize);

impl Item {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledItem {
        CompiledItem {
            id: *id_maps.items.get(&self.id_str).unwrap(),
            id_str: self.id_str,
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

impl From<usize> for ItemId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
