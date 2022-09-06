use crate::game_template::parser::WeightedIdentifier;
use crate::game_template::IdMaps;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorationEvent {
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledExplorationEvent {
    pub id: ExplorationEventId,
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightedExplorationEvent {
    pub id_str: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledWeightedExplorationEvent {
    pub id: ExplorationEventId,
    pub weight: f64,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct ExplorationEventId(usize);

impl ExplorationEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledExplorationEvent {
        CompiledExplorationEvent {
            id: *id_maps.exploration_events.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
        }
    }
}

impl WeightedExplorationEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledWeightedExplorationEvent {
        CompiledWeightedExplorationEvent {
            id: *id_maps.exploration_events.get(&self.id_str).unwrap(),
            weight: self.weight,
        }
    }
}

impl From<usize> for ExplorationEventId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<WeightedIdentifier> for WeightedExplorationEvent {
    fn from(weighted_identifier: WeightedIdentifier) -> Self {
        Self {
            id_str: weighted_identifier.identifier,
            weight: weighted_identifier.weight,
        }
    }
}
