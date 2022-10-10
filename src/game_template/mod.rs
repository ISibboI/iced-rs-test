use crate::game_state::player_actions::{
    PlayerAction, PlayerActionId, PlayerActionType, PlayerActions,
};
use crate::game_state::story::quests::quest_stages::QuestStageId;
use crate::game_state::story::quests::{Quest, QuestId};
use crate::game_state::story::Story;
use crate::game_state::triggers::{CompiledGameEvent, GameAction, GameEvent};
use crate::game_state::world::events::{ExplorationEvent, ExplorationEventId};
use crate::game_state::world::locations::{Location, LocationId};
use crate::game_state::world::monsters::{Monster, MonsterId};
use crate::game_state::world::World;
use crate::game_template::game_initialisation::{CompiledGameInitialisation, GameInitialisation};
use crate::game_template::parser::error::{ParserError, ParserErrorKind};
use event_trigger_action_system::{CompiledTriggers, Trigger, TriggerHandle};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
pub mod compiler;
pub mod game_initialisation;
pub mod parser;

#[derive(Debug, Default)]
pub struct GameTemplate {
    initialisation: Option<GameInitialisation>,
    actions: Vec<PlayerAction>,
    quests: Vec<Quest>,
    locations: Vec<Location>,
    exploration_events: Vec<ExplorationEvent>,
    monsters: Vec<Monster>,
    triggers: Vec<Trigger<GameEvent, GameAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub quest_stages: HashMap<(QuestId, String), QuestStageId>,
    pub locations: HashMap<String, LocationId>,
    pub exploration_events: HashMap<String, ExplorationEventId>,
    pub monsters: HashMap<String, MonsterId>,
    pub triggers: HashMap<String, TriggerHandle>,
}

impl IdMaps {
    pub fn from_game_template(game_template: &GameTemplate) -> Result<Self, ParserError> {
        let quests = build_id_map(
            &game_template.quests,
            |quest| quest.id_str.clone(),
            |identifier| {
                ParserError::without_coordinates(ParserErrorKind::DuplicateQuestIdentifier(
                    identifier,
                ))
            },
        )?;

        let mut quest_stages = HashMap::new();
        for quest in &game_template.quests {
            let quest_id = *quests.get(&quest.id_str).unwrap();
            for (index, stage) in quest.stages.iter().enumerate() {
                let previous = quest_stages.insert(
                    (quest_id, stage.id_str.clone()),
                    QuestStageId {
                        quest_id,
                        stage_id: index,
                    },
                );
                if previous.is_some() {
                    return Err(ParserError::without_coordinates(
                        ParserErrorKind::DuplicateQuestStageIdentifier(stage.id_str.clone()),
                    ));
                }
            }
        }

        Ok(Self {
            actions: build_id_map(
                &game_template.actions,
                |action| action.id_str.clone(),
                |identifier| {
                    ParserError::without_coordinates(ParserErrorKind::DuplicateActionIdentifier(
                        identifier,
                    ))
                },
            )?,
            quests,
            quest_stages,
            locations: build_id_map(
                &game_template.locations,
                |location| location.id_str.clone(),
                |identifier| {
                    ParserError::without_coordinates(ParserErrorKind::DuplicateLocationIdentifier(
                        identifier,
                    ))
                },
            )?,
            exploration_events: build_id_map(
                &game_template.exploration_events,
                |exploration_event| exploration_event.id_str.clone(),
                |identifier| {
                    ParserError::without_coordinates(
                        ParserErrorKind::DuplicateExplorationEventIdentifier(identifier),
                    )
                },
            )?,
            monsters: build_id_map(
                &game_template.monsters,
                |monster| monster.id_str.clone(),
                |identifier| {
                    ParserError::without_coordinates(ParserErrorKind::DuplicateMonsterIdentifier(
                        identifier,
                    ))
                },
            )?,
            triggers: build_id_map(
                &game_template.triggers,
                |trigger| trigger.id_str.clone(),
                |identifier| {
                    ParserError::without_coordinates(ParserErrorKind::DuplicateTriggerIdentifier(
                        identifier,
                    ))
                },
            )?,
        })
    }
}

impl GameTemplate {
    pub fn compile(mut self) -> Result<CompiledGameTemplate, ParserError> {
        debug!("Compiling game template");

        // put the builtin actions to their required position
        self.actions.sort_by_key(|action| match action.action_type {
            PlayerActionType::Wait => 0,
            PlayerActionType::Sleep => 1,
            PlayerActionType::Tavern => 2,
            PlayerActionType::Explore => 3,
            PlayerActionType::Train | PlayerActionType::Work => i32::MAX,
        });

        let id_maps = IdMaps::from_game_template(&self)?;

        let initialisation = self
            .initialisation
            .ok_or_else(|| {
                ParserError::without_coordinates(ParserErrorKind::MissingSectionInitialisation)
            })?
            .compile(&id_maps);

        Ok(CompiledGameTemplate {
            actions: PlayerActions::new(
                self.actions
                    .into_iter()
                    .map(|action| action.compile(&id_maps))
                    .collect(),
            )?,
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
        })
    }
}

fn build_id_map<'elements, Element: 'elements, Handle: From<usize>>(
    elements: impl IntoIterator<Item = &'elements Element>,
    id_getter: impl Fn(&Element) -> String,
    error: impl FnOnce(String) -> ParserError,
) -> Result<HashMap<String, Handle>, ParserError> {
    let mut result = HashMap::new();
    for (index, element) in elements.into_iter().enumerate() {
        if result.insert(id_getter(element), index.into()).is_some() {
            return Err(error(id_getter(element)));
        }
    }
    Ok(result)
}
