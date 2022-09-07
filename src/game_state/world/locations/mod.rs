use crate::game_state::combat::SpawnedMonster;
use crate::game_state::time::GameTime;
use crate::game_state::world::events::{
    CompiledWeightedExplorationEvent, WeightedExplorationEvent,
};
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
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
    #[deprecated]
    pub fn spawn(&self) -> SpawnedMonster {
        todo!()
        /*let mut rng = thread_rng();
        let monster = MONSTERS
            .get(
                &self
                    .monsters
                    .choose_weighted(&mut rng, |weighted_monster| weighted_monster.weight)
                    .unwrap()
                    .monster,
            )
            .unwrap()
            .clone();
        let modifier = MONSTER_MODIFIERS
            .get(
                &self
                    .monster_modifiers
                    .choose_weighted(&mut rng, |weighted_monster_modifier| {
                        weighted_monster_modifier.weight
                    })
                    .unwrap()
                    .monster_modifier,
            )
            .unwrap()
            .clone();
        monster.spawn(modifier)*/
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
