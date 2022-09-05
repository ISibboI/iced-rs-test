use crate::game_state::character::CharacterAttributeProgressFactor;
use crate::game_state::currency::Currency;
use crate::game_state::player_actions::{PlayerAction, PlayerActionType};
use crate::game_state::story::quests::Quest;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::{GameAction, GameEvent};
use crate::game_template::game_initialisation::GameInitialisation;
use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::error::{unexpected_eof, ParserError, ParserErrorKind};
use crate::game_template::parser::tokenizer::{
    KeyTokenKind, RangedElement, SectionTokenKind, Token, TokenIterator, TokenKind, ValueTokenKind,
};
use crate::game_template::parser::{expect_identifier, parse_trigger};
use crate::game_template::GameTemplate;
use async_std::io::Read;
use event_trigger_action_system::{event_count, or, Trigger};
use log::trace;
use std::mem;

#[derive(Debug)]
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

    activation: Option<RangedElement<String>>,
    deactivation: Option<RangedElement<String>>,
    completion: Option<RangedElement<String>>,
    failure: Option<RangedElement<String>>,

    starting_location: Option<RangedElement<String>>,
}

pub async fn parse_section(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    section_kind: &SectionTokenKind,
) -> Result<(GameTemplateSection, Option<Token>), ParserError> {
    trace!("Parsing section {section_kind:?}");
    let (id_str, id_range) = if section_kind == &SectionTokenKind::SectionInitialisation {
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
                KeyTokenKind::KeyName => {
                    section.name = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeyProgressive => {
                    section.progressive = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeySimplePast => {
                    section.simple_past = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeyTitle => {
                    section.title = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeyDescription => {
                    section.description = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeyStrength => {
                    let strength = tokens.expect_string_value().await?;
                    let parsed = strength.element.parse();
                    section.strength = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(strength.element.into()),
                                strength.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyStamina => {
                    let stamina = tokens.expect_string_value().await?;
                    let parsed = stamina.element.parse();
                    section.stamina = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(stamina.element.into()),
                                stamina.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyDexterity => {
                    let dexterity = tokens.expect_string_value().await?;
                    let parsed = dexterity.element.parse();
                    section.dexterity = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(dexterity.element.into()),
                                dexterity.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyIntelligence => {
                    let intelligence = tokens.expect_string_value().await?;
                    let parsed = intelligence.element.parse();
                    section.intelligence = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(intelligence.element.into()),
                                intelligence.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyWisdom => {
                    let wisdom = tokens.expect_string_value().await?;
                    let parsed = wisdom.element.parse();
                    section.wisdom = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(wisdom.element.into()),
                                wisdom.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyCharisma => {
                    let charisma = tokens.expect_string_value().await?;
                    let parsed = charisma.element.parse();
                    section.charisma = Some(RangedElement::new(
                        parsed.map_err(move |_| {
                            ParserError::with_coordinates(
                                ParserErrorKind::ExpectedFloat(charisma.element.into()),
                                charisma.range,
                            )
                        })?,
                        range,
                    ));
                }
                KeyTokenKind::KeyCurrency => {
                    if let Some(token) = tokens.next().await? {
                        let (kind, range) = token.decompose();
                        match kind {
                            TokenKind::Value(ValueTokenKind::Integer(integer)) => {
                                section.currency = Some(RangedElement::new(
                                    Currency::from_copper(integer.into()),
                                    range,
                                ));
                            }
                            kind => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::ExpectedInteger(kind.into()),
                                    range,
                                ))
                            }
                        }
                    } else {
                        return Err(unexpected_eof());
                    }
                }
                KeyTokenKind::KeyType => {
                    section.type_name = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
                KeyTokenKind::KeyDuration => {
                    if let Some(token) = tokens.next().await? {
                        let (kind, range) = token.decompose();
                        match kind {
                            TokenKind::Value(ValueTokenKind::Time(time)) => {
                                section.duration = Some(RangedElement::new(time, range));
                            }
                            kind => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::ExpectedTime(kind.into()),
                                    range,
                                ))
                            }
                        }
                    } else {
                        return Err(unexpected_eof());
                    }
                }
                KeyTokenKind::KeyActivation => {
                    let id_str = format!("{}_activation", section.id_str);
                    parse_trigger(
                        game_template,
                        tokens,
                        id_str.clone(),
                        vec![match section_kind {
                            SectionTokenKind::SectionBuiltinAction
                            | SectionTokenKind::SectionAction
                            | SectionTokenKind::SectionQuestAction => GameAction::ActivateAction {
                                id: section.id_str.clone(),
                            },
                            SectionTokenKind::SectionQuest => GameAction::ActivateQuest {
                                id: section.id_str.clone(),
                            },
                            SectionTokenKind::SectionInitialisation => {
                                return Err(ParserError::with_coordinates(
                                    ParserErrorKind::UnexpectedActivation(section.id_str.clone()),
                                    section.id_range,
                                ))
                            }
                        }],
                    )
                    .await?;
                    section.activation = Some(RangedElement::new(id_str, range));
                }
                KeyTokenKind::KeyDeactivation => {
                    let id_str = format!("{}_deactivation", section.id_str);
                    parse_trigger(
                        game_template,
                        tokens,
                        id_str.clone(),
                        vec![GameAction::DeactivateAction {
                            id: section.id_str.clone(),
                        }],
                    )
                    .await?;
                    section.deactivation = Some(RangedElement::new(id_str, range));
                }
                KeyTokenKind::KeyCompletion => {
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
                    section.completion = Some(RangedElement::new(id_str, range));
                }
                KeyTokenKind::KeyFailure => {
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
                    section.failure = Some(RangedElement::new(id_str, range));
                }
                KeyTokenKind::KeyStartingLocation => {
                    section.starting_location = Some(RangedElement::new(
                        tokens.expect_string_value().await?.element,
                        range,
                    ))
                }
            },
            kind => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::UnexpectedActionKey(kind),
                    range,
                ))
            }
        }

        if section_token.is_some() {
            break;
        }
    }

    Ok((section, section_token))
}

macro_rules! ensure_empty {
    ($self:ident, $id:ident, $unexpected:ident) => {
        if let Some($id) = $self.$id.take() {
            let (_, range) = $id.decompose();
            return Err(ParserError::with_coordinates(
                ParserErrorKind::$unexpected(stringify!($id).to_owned()),
                range,
            ));
        }
    };
}

macro_rules! taker {
    ($id:ident, $missing:ident, $t:ty) => {
        fn $id(&mut self) -> Result<RangedElement<$t>, ParserError> {
            self.$id.take().ok_or_else(|| {
                ParserError::with_coordinates(
                    ParserErrorKind::$missing(self.id_str.clone()),
                    self.id_range,
                )
            })
        }
    };
}

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
            activation: None,
            deactivation: None,
            completion: None,
            failure: None,
            starting_location: None,
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

        let id_str = mem::replace(&mut self.id_str, String::new());

        let result = Ok(PlayerAction {
            id_str,
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration: Default::default(),
            attribute_progress_factor: Default::default(),
            currency_gain: Default::default(),
            activation_condition: self.activation()?.element,
            deactivation_condition: self.deactivation()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_action(mut self) -> Result<PlayerAction, ParserError> {
        match self.id_str.as_str() {
            "EXPLORE" | "SLEEP" | "TAVERN" | "WAIT" => {
                unreachable!("Trying to parse a builtin action as normal action")
            }
            _ => {}
        }

        let id_str = mem::replace(&mut self.id_str, String::new());
        let action_type = self.type_name()?;
        let parsed_action_type = action_type.element.parse();
        let action_type = parsed_action_type.map_err(move |_| {
            ParserError::with_coordinates(
                ParserErrorKind::ExpectedActionType(action_type.element),
                action_type.range,
            )
        })?;

        let result = Ok(PlayerAction {
            id_str,
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration: self.duration()?.element,
            attribute_progress_factor: self.take_character_attribute_progress_factor(),
            currency_gain: self.currency()?.element,
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

        let id_str = mem::replace(&mut self.id_str, String::new());
        let action_type = self.type_name()?;
        let parsed_action_type = action_type.element.parse();
        let action_type = parsed_action_type.map_err(move |_| {
            ParserError::with_coordinates(
                ParserErrorKind::ExpectedActionType(action_type.element),
                action_type.range,
            )
        })?;

        let activation_condition = format!("{}_activation", id_str);
        let deactivation_condition = format!("{}_deactivation", id_str);
        game_template.triggers.push(Trigger::new(
            activation_condition.clone(),
            event_count(
                GameEvent::Action(GameAction::ActivateQuest { id: id_str.clone() }),
                1,
            ),
            vec![GameAction::ActivateAction { id: id_str.clone() }],
        ));
        game_template.triggers.push(Trigger::new(
            deactivation_condition.clone(),
            or(vec![
                event_count(
                    GameEvent::Action(GameAction::CompleteQuest { id: id_str.clone() }),
                    1,
                ),
                event_count(
                    GameEvent::Action(GameAction::FailQuest { id: id_str.clone() }),
                    1,
                ),
            ]),
            vec![GameAction::DeactivateAction { id: id_str.clone() }],
        ));

        let result = Ok(PlayerAction {
            id_str,
            name: self.name()?.element,
            verb_progressive: self.progressive()?.element,
            verb_simple_past: self.simple_past()?.element,
            action_type,
            duration: self.duration()?.element,
            attribute_progress_factor: self.take_character_attribute_progress_factor(),
            currency_gain: self.currency()?.element,
            activation_condition,
            deactivation_condition,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_quest(mut self) -> Result<Quest, ParserError> {
        let id_str = mem::replace(&mut self.id_str, String::new());

        let result = Ok(Quest {
            id_str,
            title: self.title()?.element,
            description: self.description()?.element,
            activation_condition: self.activation()?.element,
            completion_condition: self.completion()?.element,
            failure_condition: self.failure()?.element,
        });
        self.ensure_empty()?;
        result
    }

    pub fn into_initialisation(mut self) -> Result<GameInitialisation, ParserError> {
        let result = Ok(GameInitialisation {
            starting_location: self.starting_location()?.element,
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

    fn ensure_empty(mut self) -> Result<(), ParserError> {
        ensure_empty!(self, name, UnexpectedName);
        ensure_empty!(self, progressive, UnexpectedProgressive);
        ensure_empty!(self, simple_past, UnexpectedSimplePast);
        ensure_empty!(self, title, UnexpectedTitle);
        ensure_empty!(self, description, UnexpectedDescription);

        ensure_empty!(self, strength, UnexpectedStrength);
        ensure_empty!(self, stamina, UnexpectedStamina);
        ensure_empty!(self, dexterity, UnexpectedDexterity);
        ensure_empty!(self, intelligence, UnexpectedIntelligence);
        ensure_empty!(self, wisdom, UnexpectedWisdom);
        ensure_empty!(self, charisma, UnexpectedCharisma);
        ensure_empty!(self, currency, UnexpectedCurrency);

        ensure_empty!(self, type_name, UnexpectedType);
        ensure_empty!(self, duration, UnexpectedDuration);

        ensure_empty!(self, activation, UnexpectedActivation);
        ensure_empty!(self, deactivation, UnexpectedDeactivation);
        ensure_empty!(self, completion, UnexpectedCompletion);
        ensure_empty!(self, failure, UnexpectedFailure);

        ensure_empty!(self, starting_location, UnexpectedStartingLocation);

        Ok(())
    }

    taker!(name, MissingName, String);
    taker!(progressive, MissingProgressive, String);
    taker!(simple_past, MissingSimplePast, String);
    taker!(title, MissingTitle, String);
    taker!(description, MissingDescription, String);

    taker!(strength, MissingStrength, f64);
    taker!(stamina, MissingStamina, f64);
    taker!(dexterity, MissingDexterity, f64);
    taker!(intelligence, MissingIntelligence, f64);
    taker!(wisdom, MissingWisdom, f64);
    taker!(charisma, MissingCharisma, f64);
    taker!(currency, MissingCurrency, Currency);

    taker!(type_name, MissingType, String);
    taker!(duration, MissingDuration, GameTime);

    taker!(activation, MissingActivation, String);
    taker!(deactivation, MissingDeactivation, String);
    taker!(completion, MissingCompletion, String);
    taker!(failure, MissingFailure, String);

    taker!(starting_location, MissingStartingLocation, String);
}
