use crate::game_state::time::GameTime;
use crate::game_template::parser::WeightedIdentifier;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorationEvent {
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledExplorationEvent {
    pub id: ExplorationEventId,
    pub id_str: String,
    pub state: ExplorationEventState,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExplorationEventState {
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
pub struct ExplorationEventId(pub usize);

impl ExplorationEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledExplorationEvent {
        CompiledExplorationEvent {
            id: *id_maps.exploration_events.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            state: ExplorationEventState::Inactive,
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation_condition: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
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

impl ExplorationEventState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, ExplorationEventState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ExplorationEventState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, ExplorationEventState::Deactivated { .. })
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
