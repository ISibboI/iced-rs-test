use crate::game_state::character::{CharacterAttributeProgress, CharacterAttributeProgressFactor};
use crate::game_state::currency::Currency;
use crate::game_state::player_actions::{PlayerAction, PlayerActionType};
use crate::game_state::story::quests::Quest;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::{GameAction, GameEvent};
use crate::game_state::world::events::{ExplorationEvent, ExplorationEventKind};
use crate::game_state::world::locations::Location;
use crate::game_state::world::monsters::Monster;
use crate::game_template::game_initialisation::GameInitialisation;
use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::error::{unexpected_eof, ParserError, ParserErrorKind};
use crate::game_template::parser::tokenizer::{
    KeyTokenKind, RangedElement, SectionTokenKind, Token, TokenIterator, TokenKind, ValueTokenKind,
};
use crate::game_template::parser::{
    expect_identifier, parse_trigger, parse_weighted_events, WeightedIdentifier,
};
use crate::game_template::GameTemplate;
use async_std::io::Read;
use event_trigger_action_system::{event_count, or, Trigger, TriggerCondition};
use log::trace;
use section_parser_derive::SectionParser;

#[derive(Debug, SectionParser)]
pub struct GameTemplateSection {
    id_str: String,
    id_range: CharacterCoordinateRange,
    name: Option<RangedElement<String>>,
    progressive: Option<RangedElement<String>>,
    simple_past: Option<RangedElement<String>>,
    title: Option<RangedElement<String>>,
    description: Option<RangedElement<String>>,

    strength: Option<RangedElement<f64>>,
    stamina: Option<RangedElement<f64>>,
    dexterity: Option<RangedElement<f64>>,
    intelligence: Option<RangedElement<f64>>,
    wisdom: Option<RangedElement<f64>>,
    charisma: Option<RangedElement<f64>>,
    currency: Option<RangedElement<Currency>>,

    type_name: Option<RangedElement<String>>,
    duration: Option<RangedElement<GameTime>>,
    events: Option<RangedElement<Vec<WeightedIdentifier>>>,
    monster: Option<RangedElement<String>>,
    hitpoints: Option<RangedElement<f64>>,

    activation: Option<RangedElement<String>>,
    deactivation: Option<RangedElement<String>>,
    completion: Option<RangedElement<String>>,
    failure: Option<RangedElement<String>>,

    starting_location: Option<RangedElement<String>>,
    starting_time: Option<RangedElement<GameTime>>,
}

pub struct GameTemplateSectionError {
    pub id_str: String,
    pub field: String,
    pub range: CharacterCoordinateRange,
    pub kind: GameTemplateSectionErrorKind,
}

#[allow(clippy::enum_variant_names)]
pub enum GameTemplateSectionErrorKind {
    MissingField,
    UnexpectedField,
    DuplicateField,
}

pub async fn parse_section(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    section_kind: &SectionTokenKind,
) -> Result<(GameTemplateSection, Option<Token>), ParserError> {
    trace!("Parsing section {section_kind:?}");
    let (id_str, id_range) = if section_kind == &SectionTokenKind::Initialisation {
        ("".to_string(), CharacterCoordinateRange::zero())
    } else {
        expect_identifier(tokens).await?.decompose()
    };
    let mut section = GameTemplateSection::new(id_str, id_range);
    let mut section_token = None;

    while let Some(token) = tokens.next().await? {
        let (kind, range) = token.decompose();
        match kind {
            TokenKind::Section(section) => {
                section_token = Some(Token::new(TokenKind::Section(section), range));
            }
            TokenKind::Key(key) => match key {
                KeyTokenKind::Name => {
                    section.set_name(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Progressive => {
                    section.set_progressive(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::SimplePast => {
                    section.set_simple_past(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Title => {
                    section.set_title(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Description => {
                    section.set_description(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Strength => {
                    let strength = tokens.expect_string_value().await?;
                    let parsed = strength.element.parse();
                    section.set_strength(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(strength.element.into()),
                                strength.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Stamina => {
                    let stamina = tokens.expect_string_value().await?;
                    let parsed = stamina.element.parse();
                    section.set_stamina(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(stamina.element.into()),
                                stamina.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Dexterity => {
                    let dexterity = tokens.expect_string_value().await?;
                    let parsed = dexterity.element.parse();
                    section.set_dexterity(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(dexterity.element.into()),
                                dexterity.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Intelligence => {
                    let intelligence = tokens.expect_string_value().await?;
                    let parsed = intelligence.element.parse();
                    section.set_intelligence(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(intelligence.element.into()),
                                intelligence.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Wisdom => {
                    let wisdom = tokens.expect_string_value().await?;
                    let parsed = wisdom.element.parse();
                    section.set_wisdom(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(wisdom.element.into()),
                                wisdom.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Charisma => {
                    let charisma = tokens.expect_string_value().await?;
                    let parsed = charisma.element.parse();
                    section.set_charisma(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(charisma.element.into()),
                                charisma.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Currency => {
                    if let Some(token) = tokens.next().await? {
                        let (kind, range) = token.decompose();
                        match kind {
                            TokenKind::Value(ValueTokenKind::Integer(integer)) => {
                                section.set_currency(RangedElement::new(
                                    Currency::from_copper(integer.into()),
                                    range,
                                ))?;
                            }
                            kind => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::ExpectedInteger(kind.into()),
                                    range,
                                ));
                            }
                        }
                    } else {
                        return Err(unexpected_eof());
                    }
                }
                KeyTokenKind::Type => {
                    section.set_type_name(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Duration => {
                    if let Some(token) = tokens.next().await? {
                        let (kind, range) = token.decompose();
                        match kind {
                            TokenKind::Value(ValueTokenKind::Time(time)) => {
                                section.set_duration(RangedElement::new(time, range))?;
                            }
                            kind => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::ExpectedTime(kind.into()),
                                    range,
                                ));
                            }
                        }
                    } else {
                        return Err(unexpected_eof());
                    }
                }
                KeyTokenKind::Events => {
                    section.set_events(parse_weighted_events(tokens).await?)?;
                }
                KeyTokenKind::Monsters => {
                    section.set_monster(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::Hitpoints => {
                    let hitpoints = tokens.expect_string_value().await?;
                    let parsed = hitpoints.element.parse();
                    section.set_hitpoints(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(hitpoints.element.into()),
                                hitpoints.range,
                            )
                        })?,
                        range,
                    ))?;
                }
                KeyTokenKind::Activation => {
                    let (section_name_lowercase, game_action) = match section_kind {
                        SectionTokenKind::BuiltinAction
                        | SectionTokenKind::Action
                        | SectionTokenKind::QuestAction => (
                            "action",
                            GameAction::ActivateAction {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Quest => (
                            "quest",
                            GameAction::ActivateQuest {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Location => (
                            "location",
                            GameAction::ActivateLocation {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::ExplorationEvent => (
                            "exploration_event",
                            GameAction::ActivateExplorationEvent {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Monster => (
                            "monster",
                            GameAction::ActivateMonster {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Initialisation => {
                            return Err(ParserError::with_coordinates(
                                ParserErrorKind::UnexpectedField {
                                    id_str: section.id_str.clone(),
                                    field: "activation".to_string(),
                                },
                                section.id_range,
                            ));
                        }
                    };

                    let id_str = format!("{section_name_lowercase}_{}_activation", section.id_str);
                    parse_trigger(game_template, tokens, id_str.clone(), vec![game_action]).await?;
                    section.set_activation(RangedElement::new(id_str, range))?;
                }
                KeyTokenKind::Deactivation => {
                    let (section_name_lowercase, game_action) = match section_kind {
                        SectionTokenKind::BuiltinAction
                        | SectionTokenKind::Action
                        | SectionTokenKind::QuestAction => (
                            "action",
                            GameAction::DeactivateAction {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Location => (
                            "location",
                            GameAction::DeactivateLocation {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::ExplorationEvent => (
                            "exploration_event",
                            GameAction::DeactivateExplorationEvent {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Monster => (
                            "monster",
                            GameAction::DeactivateMonster {
                                id: section.id_str.clone(),
                            },
                        ),
                        SectionTokenKind::Quest | SectionTokenKind::Initialisation => {
                            return Err(ParserError::with_coordinates(
                                ParserErrorKind::UnexpectedField {
                                    id_str: section.id_str.clone(),
                                    field: "deactivation".to_string(),
                                },
                                section.id_range,
                            ));
                        }
                    };

                    let id_str =
                        format!("{section_name_lowercase}_{}_deactivation", section.id_str);
                    parse_trigger(game_template, tokens, id_str.clone(), vec![game_action]).await?;
                    section.set_deactivation(RangedElement::new(id_str, range))?;
                }
                KeyTokenKind::Completion => {
                    let id_str = format!("{}_completion", section.id_str);
                    parse_trigger(
                        game_template,
                        tokens,
                        id_str.clone(),
                        vec![GameAction::CompleteQuest {
                            id: section.id_str.clone(),
                        }],
                    )
                    .await?;
                    section.set_completion(RangedElement::new(id_str, range))?;
                }
                KeyTokenKind::Failure => {
                    let id_str = format!("{}_fail", section.id_str);
                    parse_trigger(
                        game_template,
                        tokens,
                        id_str.clone(),
                        vec![GameAction::FailQuest {
                            id: section.id_str.clone(),
                        }],
                    )
                    .await?;
                    section.set_failure(RangedElement::new(id_str, range))?;
                }
                KeyTokenKind::StartingLocation => {
                    section.set_starting_location(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))?;
                }
                KeyTokenKind::StartingTime => {
                    if let Some(token) = tokens.next().await? {
                        let (kind, range) = token.decompose();
                        match kind {
                            TokenKind::Value(ValueTokenKind::Time(time)) => {
                                section.set_starting_time(RangedElement::new(time, range))?;
                            }
                            kind => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::ExpectedTime(kind.into()),
                                    range,
                                ));
                            }
                        }
                    } else {
                        return Err(unexpected_eof());
                    }
                }
            },
            TokenKind::Value(value) => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::UnexpectedValue(value),
                    range,
                ));
            }
        }

        if section_token.is_some() {
            break;
        }
    }

    Ok((section, section_token))
}

// TODO only allow deactivation of actions etc after they are activated, i.e. make a sequence condition for deactivation with first element being activation
// TODO also think this through for failure of quests
impl GameTemplateSection {
    fn new(id_str: String, id_range: CharacterCoordinateRange) -> Self {
        Self {
            id_str,
            id_range,
            name: None,
            progressive: None,
            simple_past: None,
            title: None,
            description: None,
            strength: None,
            stamina: None,
            dexterity: None,
            intelligence: None,
            wisdom: None,
            charisma: None,
            currency: None,
            type_name: None,
            duration: None,
            events: None,
            monster: None,
            hitpoints: None,
            activation: None,
            deactivation: None,
            completion: None,
            failure: None,
            starting_location: None,
            starting_time: None,
        }
    }

    pub fn into_builtin_action(mut self) -> Result<PlayerAction, ParserError> {
        let action_type = match self.id_str.as_str() {
            "EXPLORE" => PlayerActionType::Explore,
            "SLEEP" => PlayerActionType::Sleep,
            "TAVERN" => PlayerActionType::Tavern,
            "WAIT" => PlayerActionType::Wait,
            _ => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::UnknownBuiltinAction(self.id_str),
                    self.id_range,
                ))
            }
        };

        let duration = match action_type {
            PlayerActionType::Sleep => GameTime::default(),
            _ => self.duration()?.element,
        };

        let result = Ok(PlayerAction {
            id_str: self.id_str.clone(),
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration,
            attribute_progress_factor: Default::default(),
            currency_reward: Default::default(),
            activation_condition: self.activation()?.element,
            deactivation_condition: self.deactivation()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_action(mut self) -> Result<PlayerAction, ParserError> {
        match self.id_str.as_str() {
            "EXPLORE" | "SLEEP" | "TAVERN" | "WAIT" => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::ReservedActionId(self.id_str.clone()),
                    self.id_range,
                ));
            }
            _ => {}
        }

        let action_type = self.type_name()?;
        let parsed_action_type = action_type.element.parse();
        let action_type = parsed_action_type.map_err(move |_| {
            ParserError::with_coordinates(
                ParserErrorKind::ExpectedActionType(action_type.element),
                action_type.range,
            )
        })?;

        let result = Ok(PlayerAction {
            id_str: self.id_str.clone(),
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration: self.duration()?.element,
            attribute_progress_factor: self.take_character_attribute_progress_factor(),
            currency_reward: self.currency()?.element,
            activation_condition: self.activation()?.element,
            deactivation_condition: self.deactivation()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_quest_action(
        mut self,
        game_template: &mut GameTemplate,
    ) -> Result<PlayerAction, ParserError> {
        match self.id_str.as_str() {
            "EXPLORE" | "SLEEP" | "TAVERN" | "WAIT" => {
                unreachable!("Trying to parse a builtin action as normal action")
            }
            _ => {}
        }

        let action_type = self.type_name()?;
        let action_type_range = action_type.range;
        let parsed_action_type = action_type.element.parse();
        let action_type = parsed_action_type.map_err(move |_| {
            ParserError::with_coordinates(
                ParserErrorKind::ExpectedActionType(action_type.element),
                action_type.range,
            )
        })?;
        if matches!(
            action_type,
            PlayerActionType::Explore
                | PlayerActionType::Sleep
                | PlayerActionType::Tavern
                | PlayerActionType::Wait
        ) {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::IllegalActionType(action_type),
                action_type_range,
            ));
        }

        let activation_condition = format!("action_{}_activation", self.id_str);
        let deactivation_condition = format!("action_{}_deactivation", self.id_str);
        game_template.triggers.push(Trigger::new(
            activation_condition.clone(),
            event_count(
                GameEvent::Action(GameAction::ActivateQuest {
                    id: self.id_str.clone(),
                }),
                1,
            ),
            vec![GameAction::ActivateAction {
                id: self.id_str.clone(),
            }],
        ));
        game_template.triggers.push(Trigger::new(
            deactivation_condition.clone(),
            or(vec![
                event_count(
                    GameEvent::Action(GameAction::CompleteQuest {
                        id: self.id_str.clone(),
                    }),
                    1,
                ),
                event_count(
                    GameEvent::Action(GameAction::FailQuest {
                        id: self.id_str.clone(),
                    }),
                    1,
                ),
            ]),
            vec![GameAction::DeactivateAction {
                id: self.id_str.clone(),
            }],
        ));

        let result = Ok(PlayerAction {
            id_str: self.id_str.clone(),
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration: self.duration()?.element,
            attribute_progress_factor: self.take_character_attribute_progress_factor(),
            currency_reward: self.currency()?.element,
            activation_condition,
            deactivation_condition,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_quest(mut self) -> Result<Quest, ParserError> {
        let result = Ok(Quest {
            id_str: self.id_str.clone(),
            title: self.title()?.element,
            description: self.description()?.element,
            activation_condition: self.activation()?.element,
            completion_condition: self.completion()?.element,
            failure_condition: self.failure()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_location(mut self) -> Result<Location, ParserError> {
        let result = Ok(Location {
            id_str: self.id_str.clone(),
            name: self.name()?.element,
            events: self.events()?.element.into_iter().map(Into::into).collect(),
            activation_condition: self.activation()?.element,
            deactivation_condition: self.deactivation()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_exploration_event(
        mut self,
        game_template: &mut GameTemplate,
    ) -> Result<ExplorationEvent, ParserError> {
        let activation_condition = self.activation()?.element;
        let deactivation_condition = self.deactivation()?.element;
        if let Some(monster) = &self.monster {
            let monster = &monster.element;
            let activation_trigger = game_template
                .triggers
                .iter_mut()
                .rev()
                .find(|trigger| trigger.id_str == activation_condition)
                .unwrap();
            activation_trigger.condition &= TriggerCondition::EventCount {
                required: 1,
                event: GameEvent::Action(GameAction::ActivateMonster {
                    id: monster.clone(),
                }),
            };
            let deactivation_trigger = game_template
                .triggers
                .iter_mut()
                .rev()
                .find(|trigger| trigger.id_str == deactivation_condition)
                .unwrap();
            deactivation_trigger.condition |= TriggerCondition::EventCount {
                required: 1,
                event: GameEvent::Action(GameAction::DeactivateMonster {
                    id: monster.clone(),
                }),
            };
        }

        let kind = if self.monster.is_some() {
            ExplorationEventKind::Monster {
                monster: self.monster()?.element,
            }
        } else {
            ExplorationEventKind::Normal {
                name: self.name()?.element,
                verb_progressive: self.progressive()?.element,
                verb_simple_past: self.simple_past()?.element,
            }
        };

        let result = Ok(ExplorationEvent {
            id_str: self.id_str.clone(),
            kind,
            attribute_progress: self.take_character_attribute_progress(),
            currency_reward: self
                .currency
                .take()
                .map(|currency| currency.element)
                .unwrap_or(Currency::zero()),
            activation_condition,
            deactivation_condition,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_monster(mut self) -> Result<Monster, ParserError> {
        let result = Ok(Monster {
            id_str: self.id_str.clone(),
            name: self.name()?.element,
            hitpoints: self.hitpoints()?.element,
            activation_condition: self.activation()?.element,
            deactivation_condition: self.deactivation()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_initialisation(mut self) -> Result<GameInitialisation, ParserError> {
        let result = Ok(GameInitialisation {
            starting_location: self.starting_location()?.element,
            starting_time: self.starting_time()?.element,
        });
        self.ensure_empty()?;
        result
    }

    fn take_character_attribute_progress_factor(&mut self) -> CharacterAttributeProgressFactor {
        CharacterAttributeProgressFactor::new(
            self.strength().map(|e| e.element).unwrap_or(0.0),
            self.stamina().map(|e| e.element).unwrap_or(0.0),
            self.dexterity().map(|e| e.element).unwrap_or(0.0),
            self.intelligence().map(|e| e.element).unwrap_or(0.0),
            self.wisdom().map(|e| e.element).unwrap_or(0.0),
            self.charisma().map(|e| e.element).unwrap_or(0.0),
        )
    }

    fn take_character_attribute_progress(&mut self) -> CharacterAttributeProgress {
        CharacterAttributeProgress::new(
            self.strength()
                .map(|e| e.element.round() as u64)
                .unwrap_or(0),
            self.stamina()
                .map(|e| e.element.round() as u64)
                .unwrap_or(0),
            self.dexterity()
                .map(|e| e.element.round() as u64)
                .unwrap_or(0),
            self.intelligence()
                .map(|e| e.element.round() as u64)
                .unwrap_or(0),
            self.wisdom().map(|e| e.element.round() as u64).unwrap_or(0),
            self.charisma()
                .map(|e| e.element.round() as u64)
                .unwrap_or(0),
        )
    }

    fn missing_field_error(&self, field: &str) -> GameTemplateSectionError {
        GameTemplateSectionError {
            id_str: self.id_str.clone(),
            field: field.to_string(),
            range: self.id_range,
            kind: GameTemplateSectionErrorKind::MissingField,
        }
    }

    fn duplicate_field_error<T>(
        &self,
        field: &str,
        value: RangedElement<T>,
    ) -> GameTemplateSectionError {
        GameTemplateSectionError {
            id_str: self.id_str.clone(),
            field: field.to_string(),
            range: value.range,
            kind: GameTemplateSectionErrorKind::DuplicateField,
        }
    }

    fn unexpected_field_error<T>(
        &self,
        field: &str,
        value: RangedElement<T>,
    ) -> GameTemplateSectionError {
        GameTemplateSectionError {
            id_str: self.id_str.clone(),
            field: field.to_string(),
            range: value.range,
            kind: GameTemplateSectionErrorKind::UnexpectedField,
        }
    }
}
