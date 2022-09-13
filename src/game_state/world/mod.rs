use crate::game_state::character::Character;
use crate::game_state::player_actions::PlayerActionInProgress;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use crate::game_state::world::events::{
    CompiledExplorationEvent, ExplorationEventId, ExplorationEventState,
};
use crate::game_state::world::locations::{CompiledLocation, LocationId, LocationState};
use crate::game_state::world::monsters::{CompiledMonster, MonsterId, MonsterState};
use rand::Rng;
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

    fn location_mut(&mut self, location_id: LocationId) -> &mut CompiledLocation {
        &mut self.locations[location_id.0]
    }

    pub fn selected_location(&self) -> &CompiledLocation {
        self.location(self.selected_location)
    }

    pub fn active_locations(&self) -> impl '_ + Iterator<Item = &'_ CompiledLocation> {
        self.active_locations
            .iter()
            .map(|location_id| self.location(*location_id))
    }

    pub fn event(&self, event_id: ExplorationEventId) -> &CompiledExplorationEvent {
        &self.events[event_id.0]
    }

    fn event_mut(&mut self, event_id: ExplorationEventId) -> &mut CompiledExplorationEvent {
        &mut self.events[event_id.0]
    }

    pub fn monster(&self, monster_id: MonsterId) -> &CompiledMonster {
        &self.monsters[monster_id.0]
    }

    fn monster_mut(&mut self, monster_id: MonsterId) -> &mut CompiledMonster {
        &mut self.monsters[monster_id.0]
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

    pub fn activate_exploration_event(
        &mut self,
        event_id: ExplorationEventId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let event = self.event_mut(event_id);
        assert!(event.state.is_inactive());
        event.state = ExplorationEventState::Active {
            activation_time: time,
        };
        iter::empty()
    }

    pub fn deactivate_exploration_event(
        &mut self,
        event_id: ExplorationEventId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let event = self.event_mut(event_id);
        assert!(event.state.is_active());
        match event.state {
            ExplorationEventState::Active { activation_time } => {
                event.state = ExplorationEventState::Deactivated {
                    activation_time,
                    deactivation_time: time,
                };
            }
            _ => unreachable!(),
        }
        iter::empty()
    }

    pub fn activate_monster(
        &mut self,
        monster_id: MonsterId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let monster = self.monster_mut(monster_id);
        assert!(monster.state.is_inactive());
        monster.state = MonsterState::Active {
            activation_time: time,
        };
        iter::empty()
    }

    pub fn deactivate_monster(
        &mut self,
        monster_id: MonsterId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let monster = self.monster_mut(monster_id);
        assert!(monster.state.is_active());
        match monster.state {
            MonsterState::Active { activation_time } => {
                monster.state = MonsterState::Deactivated {
                    activation_time,
                    deactivation_time: time,
                };
            }
            _ => unreachable!(),
        }
        iter::empty()
    }

    pub fn explore(
        &self,
        rng: &mut impl Rng,
        start_time: GameTime,
        default_duration: GameTime,
        character: &Character,
    ) -> Option<PlayerActionInProgress> {
        let location = self.selected_location();
        let event_id = location.explore(rng, &self.events)?;
        let event = self.event(event_id);
        Some(event.spawn(
            rng,
            start_time,
            default_duration,
            character,
            self.monsters.as_slice(),
        ))
    }
}

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
