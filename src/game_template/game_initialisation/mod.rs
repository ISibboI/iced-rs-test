use crate::game_state::world::locations::LocationId;
use crate::game_template::IdMaps;

#[derive(Debug)]
pub struct GameInitialisation {
    pub starting_location: String,
}

#[derive(Debug, Clone)]
pub struct CompiledGameInitialisation {
    pub starting_location: LocationId,
}

impl GameInitialisation {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameInitialisation {
        CompiledGameInitialisation {
            starting_location: *id_maps.locations.get(&self.starting_location).unwrap(),
        }
    }
}
