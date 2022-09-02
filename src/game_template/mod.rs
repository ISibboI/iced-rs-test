use crate::game_state::player_actions::{PlayerAction, PlayerActionId, PlayerActions};
use crate::game_state::story::quests::{Quest, QuestId};
use crate::game_state::story::Story;
use crate::game_state::triggers::{CompiledGameEvent, GameAction, GameEvent};
use crate::game_state::world::events::{ExplorationEvent, ExplorationEventId};
use crate::game_state::world::locations::{Location, LocationId};
use crate::game_state::world::monsters::{Monster, MonsterId};
use crate::game_state::world::World;
use crate::game_template::game_initialisation::{CompiledGameInitialisation, GameInitialisation};
use event_trigger_action_system::{CompiledTriggers, Trigger, TriggerHandle};
use std::collections::HashMap;

pub mod game_initialisation;

#[derive(Debug, Default)]
pub struct GameTemplate {
    initialisation: GameInitialisation,
    actions: Vec<PlayerAction>,
    quests: Vec<Quest>,
    locations: Vec<Location>,
    exploration_events: Vec<ExplorationEvent>,
    monsters: Vec<Monster>,
    triggers: Vec<Trigger<GameEvent, GameAction>>,
}

#[derive(Debug)]
pub struct CompiledGameTemplate {
    pub initialisation: CompiledGameInitialisation,
    pub actions: PlayerActions,
    pub story: Story,
    pub world: World,
    pub triggers: CompiledTriggers<CompiledGameEvent>,
}

#[derive(Debug)]
pub struct IdMaps {
    pub actions: HashMap<String, PlayerActionId>,
    pub quests: HashMap<String, QuestId>,
    pub locations: HashMap<String, LocationId>,
    pub exploration_events: HashMap<String, ExplorationEventId>,
    pub monsters: HashMap<String, MonsterId>,
    pub triggers: HashMap<String, TriggerHandle>,
}

impl IdMaps {
    pub fn from_game_template(game_template: &GameTemplate) -> Self {
        Self {
            actions: build_id_map(&game_template.actions, |action| action.id_str.clone()),
            quests: build_id_map(&game_template.quests, |quest| quest.id_str.clone()),
            locations: build_id_map(&game_template.locations, |location| location.id_str.clone()),
            exploration_events: build_id_map(
                &game_template.exploration_events,
                |exploration_event| exploration_event.id_str.clone(),
            ),
            monsters: build_id_map(&game_template.monsters, |monster| monster.id_str.clone()),
            triggers: build_id_map(&game_template.triggers, |trigger| trigger.id_str.clone()),
        }
    }
}

impl GameTemplate {
    pub fn compile(self) -> CompiledGameTemplate {
        let id_maps = IdMaps::from_game_template(&self);

        let initialisation = self.initialisation.compile(&id_maps);

        CompiledGameTemplate {
            actions: PlayerActions::new(
                self.actions
                    .into_iter()
                    .map(|action| action.compile(&id_maps))
                    .collect(),
            ),
            story: Story::new(
                self.quests
                    .into_iter()
                    .map(|quest| quest.compile(&id_maps))
                    .collect(),
            ),
            world: World::new(
                initialisation.starting_location,
                self.locations
                    .into_iter()
                    .map(|location| location.compile(&id_maps))
                    .collect(),
                self.exploration_events
                    .into_iter()
                    .map(|exploration_event| exploration_event.compile(&id_maps))
                    .collect(),
                self.monsters
                    .into_iter()
                    .map(|monster| monster.compile(&id_maps))
                    .collect(),
            ),
            triggers: CompiledTriggers::new(
                self.triggers
                    .into_iter()
                    .map(|trigger| {
                        trigger.compile(&|event| event.compile(&id_maps), &|action| {
                            action.compile(&id_maps)
                        })
                    })
                    .collect(),
            ),
            initialisation,
        }
    }
}

fn build_id_map<'elements, Element: 'elements, Handle: From<usize>>(
    elements: impl IntoIterator<Item = &'elements Element>,
    name_getter: impl Fn(&Element) -> String,
) -> HashMap<String, Handle> {
    elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| (name_getter(element), index.into()))
        .collect()
}
