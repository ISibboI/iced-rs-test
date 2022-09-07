use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use crate::game_state::world::events::CompiledExplorationEvent;
use crate::game_state::world::locations::{CompiledLocation, LocationId, LocationState};
use crate::game_state::world::monsters::CompiledMonster;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter;

pub mod events;
pub mod locations;
pub mod monsters;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub selected_location: LocationId,
    locations: Vec<CompiledLocation>,
    events: Vec<CompiledExplorationEvent>,
    monsters: Vec<CompiledMonster>,
    active_locations: HashSet<LocationId>,
}

impl World {
    pub fn new(
        starting_location: LocationId,
        locations: Vec<CompiledLocation>,
        events: Vec<CompiledExplorationEvent>,
        monsters: Vec<CompiledMonster>,
    ) -> Self {
        Self {
            selected_location: starting_location,
            locations,
            events,
            monsters,
            active_locations: Default::default(),
        }
    }

    #[deprecated]
    pub fn locations(&self) -> impl '_ + Iterator<Item = &'_ CompiledLocation> {
        self.locations.iter()
    }

    pub fn location(&self, location_id: LocationId) -> &CompiledLocation {
        &self.locations[location_id.0]
    }

    pub fn location_mut(&mut self, location_id: LocationId) -> &mut CompiledLocation {
        &mut self.locations[location_id.0]
    }

    pub fn activate_location(
        &mut self,
        location_id: LocationId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let location = self.location_mut(location_id);
        assert!(location.state.is_inactive());
        location.state = LocationState::Active {
            activation_time: time,
        };
        assert!(self.active_locations.insert(location_id));
        iter::empty()
    }

    pub fn deactivate_location(
        &mut self,
        location_id: LocationId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let location = self.location_mut(location_id);
        assert!(location.state.is_active());
        match location.state {
            LocationState::Active { activation_time } => {
                location.state = LocationState::Deactivated {
                    activation_time,
                    deactivation_time: time,
                };
                assert!(self.active_locations.remove(&location_id));
            }
            _ => unreachable!(),
        }
        iter::empty()
    }
}

/*pub fn init_locations() -> Vec<Location> {
    vec![
        Location::new(LOCATION_VILLAGE, [("rat", 1.0)], [("normal", 1.0)]),
        Location::new(
            "Forest",
            [("rat", 0.1), ("hare", 1.0), ("deer", 1.0)],
            [("normal", 1.0), ("young", 0.1), ("old", 0.1)],
        ),
        Location::new(
            "Deep forest",
            [
                ("rat", 0.01),
                ("hare", 0.1),
                ("deer", 1.0),
                ("fox", 1.0),
                ("boar", 1.0),
                ("wolf", 0.1),
            ],
            [
                ("normal", 1.0),
                ("young", 0.1),
                ("old", 0.1),
                ("strong", 0.1),
                ("weak", 0.1),
            ],
        ),
    ]
}*/

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WeightedMonster {
    monster: String,
    weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WeightedMonsterModifier {
    monster_modifier: String,
    weight: f64,
}
