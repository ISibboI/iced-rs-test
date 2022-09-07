use crate::game_state::time::GameTime;
use crate::game_state::world::events::{
    CompiledExplorationEvent, CompiledWeightedExplorationEvent, ExplorationEventId,
    WeightedExplorationEvent,
};
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use rand::distributions::WeightedError;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub id_str: String,
    pub name: String,
    pub events: Vec<WeightedExplorationEvent>,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledLocation {
    pub id: LocationId,
    pub id_str: String,
    pub state: LocationState,
    pub name: String,
    pub events: Vec<CompiledWeightedExplorationEvent>,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LocationState {
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
pub struct LocationId(pub usize);

impl Location {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledLocation {
        CompiledLocation {
            id: *id_maps.locations.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            state: LocationState::Inactive,
            name: self.name,
            events: self
                .events
                .into_iter()
                .map(|event| event.compile(id_maps))
                .collect(),
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation_condition: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
        }
    }
}

impl CompiledLocation {
    pub fn explore(
        &self,
        rng: &mut impl Rng,
        exploration_events: &Vec<CompiledExplorationEvent>,
    ) -> Option<ExplorationEventId> {
        assert!(self.state.is_active());
        let active_events: Vec<_> = self
            .events
            .iter()
            .filter(|weighted_event| exploration_events[weighted_event.id.0].state.is_active())
            .collect();
        match active_events.choose_weighted(rng, |active_event| active_event.weight) {
            Ok(event) => Some(event.id),
            Err(error) => match error {
                WeightedError::NoItem => None,
                WeightedError::InvalidWeight
                | WeightedError::TooMany
                | WeightedError::AllWeightsZero => panic!("Error: {:?}", error),
            },
        }
    }
}

impl LocationState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, LocationState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, LocationState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, LocationState::Deactivated { .. })
    }
}

impl From<usize> for LocationId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
