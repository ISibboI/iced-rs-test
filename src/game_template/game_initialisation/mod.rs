use crate::game_state::time::GameTime;
use crate::game_state::world::locations::LocationId;
use crate::game_template::IdMaps;

#[derive(Debug)]
pub struct GameInitialisation {
    pub starting_location: String,
    pub starting_time: GameTime,
}

#[derive(Debug, Clone)]
pub struct CompiledGameInitialisation {
    pub starting_location: LocationId,
    pub starting_time: GameTime,
}

impl GameInitialisation {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameInitialisation {
        CompiledGameInitialisation {
            starting_location: *id_maps.locations.get(&self.starting_location).unwrap(),
            starting_time: self.starting_time,
        }
    }
}
