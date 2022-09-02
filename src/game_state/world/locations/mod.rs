use crate::game_state::combat::SpawnedMonster;
use crate::game_state::world::events::{
    CompiledWeightedExplorationEvent, WeightedExplorationEvent,
};
use crate::game_template::IdMaps;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub id_str: String,
    pub name: String,
    pub events: Vec<WeightedExplorationEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledLocation {
    pub id: LocationId,
    pub id_str: String,
    pub name: String,
    pub events: Vec<CompiledWeightedExplorationEvent>,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct LocationId(pub usize);

impl Location {
    pub fn new(id_str: String, name: String, events: Vec<WeightedExplorationEvent>) -> Self {
        Self {
            id_str,
            name,
            events,
        }
    }

    pub fn compile(self, id_maps: &IdMaps) -> CompiledLocation {
        CompiledLocation {
            id: *id_maps.locations.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            name: self.name,
            events: self
                .events
                .into_iter()
                .map(|event| event.compile(id_maps))
                .collect(),
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

impl From<usize> for LocationId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
