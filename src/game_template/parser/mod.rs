use crate::game_state::currency::Currency;
use crate::game_state::player_actions::{PlayerAction, PlayerActionType};
use crate::game_state::triggers::{GameAction, GameEvent};
use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::error::{unexpected_eof, ParserError, ParserErrorKind};
use crate::game_template::parser::tokenizer::{
    RangedElement, SectionTokenKind, Token, TokenIterator, TokenKind, ValueTokenKind,
};
use crate::game_template::GameTemplate;
use async_recursion::async_recursion;
use async_std::io::Read;
use event_trigger_action_system::{
    and, any_n, event_count, geq, never, none, or, sequence, Trigger, TriggerCondition,
};

mod character_iterator;
pub mod error;
mod tokenizer;

pub async fn parse_game_template_file(
    game_template: &mut GameTemplate,
    input: impl Read + Unpin + Send,
) -> Result<(), ParserError> {
    parse(game_template, &mut TokenIterator::new(input)).await
}

async fn parse(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<(), ParserError> {
    let mut next_token = tokens.next().await?;
    while let Some(token) = next_token {
        next_token = match token.kind() {
            TokenKind::Section(section) => match section {
                SectionTokenKind::SectionBuiltinAction => {
                    parse_builtin_action(game_template, tokens).await?
                }
                SectionTokenKind::SectionAction => parse_action(game_template, tokens).await?,
                SectionTokenKind::SectionQuestAction => {
                    parse_quest_action(game_template, tokens).await?
                }
                SectionTokenKind::SectionQuest => parse_quest(game_template, tokens).await?,
            },
            _ => return Err(token.error(|kind| ParserErrorKind::ExpectedSection(kind))),
        };
    }

    Ok(())
}

async fn parse_builtin_action(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<Option<Token>, ParserError> {
    let (identifier, range) = expect_identifier(tokens).await?.decompose();
    let action_type = match identifier.as_str() {
        "EXPLORE" => PlayerActionType::Explore,
        "SLEEP" => PlayerActionType::Sleep,
        "TAVERN" => PlayerActionType::Tavern,
        "WAIT" => PlayerActionType::Wait,
        _ => {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::UnknownBuiltinAction(identifier),
                range,
            ))
        }
    };

    let action = PlayerAction {
        id_str: identifier,
        name: "".to_string(),
        verb_progressive: "".to_string(),
        verb_simple_past: "".to_string(),
        action_type,
        attribute_progress_factor: Default::default(),
        currency_gain: Default::default(),
    };

    parse_action_body(game_template, tokens, action, range, true).await
}

async fn parse_action(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<Option<Token>, ParserError> {
    let (identifier, range) = expect_identifier(tokens).await?.decompose();

    let action = PlayerAction {
        id_str: identifier,
        name: "".to_string(),
        verb_progressive: "".to_string(),
        verb_simple_past: "".to_string(),
        action_type: PlayerActionType::Explore,
        attribute_progress_factor: Default::default(),
        currency_gain: Default::default(),
    };

    parse_action_body(game_template, tokens, action, range, false).await
}

async fn parse_action_body(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    mut action: PlayerAction,
    id_range: CharacterCoordinateRange,
    is_builtin: bool,
) -> Result<Option<Token>, ParserError> {
    let mut result = None;
    while let Some(token) = tokens.next().await? {
        let (kind, range) = token.decompose();
        match kind {
            TokenKind::Section(section) => {
                result = Some(Token::new(TokenKind::Section(section), range));
            }
            TokenKind::KeyName => action.name = tokens.expect_string_value().await?.element,
            TokenKind::KeyProgressive => {
                action.verb_progressive = tokens.expect_string_value().await?.element
            }
            TokenKind::KeySimplePast => {
                action.verb_simple_past = tokens.expect_string_value().await?.element
            }
            TokenKind::KeyType => {
                let action_type = tokens.expect_string_value().await?;
                let parsed = action_type.element.parse();
                action.action_type = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedActionType(action_type.element),
                        action_type.range,
                    )
                })?;
            }
            TokenKind::KeyStrength => {
                let strength = tokens.expect_string_value().await?;
                let parsed = strength.element.parse();
                action.attribute_progress_factor.strength = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(strength.element.into()),
                        strength.range,
                    )
                })?;
            }
            TokenKind::KeyStamina => {
                let stamina = tokens.expect_string_value().await?;
                let parsed = stamina.element.parse();
                action.attribute_progress_factor.stamina = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(stamina.element.into()),
                        stamina.range,
                    )
                })?;
            }
            TokenKind::KeyDexterity => {
                let dexterity = tokens.expect_string_value().await?;
                let parsed = dexterity.element.parse();
                action.attribute_progress_factor.dexterity = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(dexterity.element.into()),
                        dexterity.range,
                    )
                })?;
            }
            TokenKind::KeyIntelligence => {
                let intelligence = tokens.expect_string_value().await?;
                let parsed = intelligence.element.parse();
                action.attribute_progress_factor.intelligence = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(intelligence.element.into()),
                        intelligence.range,
                    )
                })?;
            }
            TokenKind::KeyWisdom => {
                let wisdom = tokens.expect_string_value().await?;
                let parsed = wisdom.element.parse();
                action.attribute_progress_factor.wisdom = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(wisdom.element.into()),
                        wisdom.range,
                    )
                })?;
            }
            TokenKind::KeyCharisma => {
                let charisma = tokens.expect_string_value().await?;
                let parsed = charisma.element.parse();
                action.attribute_progress_factor.charisma = parsed.map_err(move |_| {
                    ParserError::with_coordinates(
                        ParserErrorKind::ExpectedFloat(charisma.element.into()),
                        charisma.range,
                    )
                })?;
            }
            TokenKind::KeyCurrency => {
                if let Some(token) = tokens.next().await? {
                    let (kind, range) = token.decompose();
                    match kind {
                        TokenKind::Value(ValueTokenKind::Integer(integer)) => {
                            action.currency_gain = Currency::from_copper(integer.into())
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
            TokenKind::KeyActivation => {
                parse_trigger(
                    game_template,
                    tokens,
                    format!("{}_activation", action.id_str),
                    vec![GameAction::ActivateAction {
                        id: action.id_str.clone(),
                    }],
                )
                .await?;
            }
            kind => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::UnexpectedActionKey(kind),
                    range,
                ))
            }
        }

        if result.is_some() {
            break;
        }
    }

    if action.name.is_empty() {
        return Err(ParserError::with_coordinates(
            ParserErrorKind::ActionMissesName(action.id_str),
            id_range,
        ));
    }
    if action.verb_progressive.is_empty() {
        return Err(ParserError::with_coordinates(
            ParserErrorKind::ActionMissesVerbProgressive(action.id_str),
            id_range,
        ));
    }
    if action.verb_simple_past.is_empty() {
        return Err(ParserError::with_coordinates(
            ParserErrorKind::ActionMissesVerbSimplePast(action.id_str),
            id_range,
        ));
    }
    if action.action_type == PlayerActionType::Explore && !is_builtin {
        return Err(ParserError::with_coordinates(
            ParserErrorKind::ActionMissesType(action.id_str),
            id_range,
        ));
    }

    game_template.actions.push(action);

    Ok(result)
}

async fn parse_quest_action(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<Option<Token>, ParserError> {
    todo!()
}

async fn parse_quest(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<Option<Token>, ParserError> {
    todo!()
}

async fn parse_trigger(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    id_str: String,
    game_actions: Vec<GameAction>,
) -> Result<(), ParserError> {
    let condition = parse_trigger_condition(tokens).await?;
    game_template
        .triggers
        .push(Trigger::new(id_str, condition, game_actions));
    Ok(())
}

#[async_recursion]
async fn parse_trigger_condition(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<TriggerCondition<GameEvent>, ParserError> {
    let (identifier, range) = expect_identifier(tokens).await?.decompose();
    Ok(match identifier.as_str() {
        "none" => none(),
        "never" => never(),
        "event_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let event = parse_game_event(tokens).await?;
            expect_close_parenthesis(tokens).await?;
            event_count(event, count as usize)
        }
        "geq" => {
            expect_open_parenthesis(tokens).await?;
            let event = parse_game_event(tokens).await?;
            expect_close_parenthesis(tokens).await?;
            geq(event)
        }
        "and" => and(parse_trigger_condition_sequence(tokens, true).await?),
        "or" => or(parse_trigger_condition_sequence(tokens, true).await?),
        "sequence" => sequence(parse_trigger_condition_sequence(tokens, true).await?),
        "any_n" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let events = parse_trigger_condition_sequence(tokens, false).await?;
            any_n(events, count as usize)
        }
        "action_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let action = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(GameEvent::ActionCompleted { id: action }, count as usize)
        }
        "monster_killed_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let monster = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(GameEvent::MonsterKilled { id: monster }, count as usize)
        }
        _ => {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::UnexpectedTriggerCondition(identifier),
                range,
            ))
        }
    })
}

#[async_recursion]
async fn parse_trigger_condition_sequence(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    contains_open_parenthesis: bool,
) -> Result<Vec<TriggerCondition<GameEvent>>, ParserError> {
    if contains_open_parenthesis {
        expect_open_parenthesis(tokens).await?;
    }

    let mut result = Vec::new();
    loop {
        result.push(parse_trigger_condition(tokens).await?);

        let (kind, range) = expect_any(tokens).await?.decompose();
        match kind {
            TokenKind::Value(ValueTokenKind::Comma) => {}
            TokenKind::Value(ValueTokenKind::CloseParenthesis) => return Ok(result),
            kind => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::ExpectedCommaOrCloseParenthesis(kind),
                    range,
                ))
            }
        }
    }
}

async fn parse_game_event(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<GameEvent, ParserError> {
    let token = tokens.next().await?;
    if let Some(token) = token {
        let (identifier, range) = expect_identifier(tokens).await?.decompose();
        Ok(match identifier.as_str() {
            "currency_changed" => {
                parse_f_currency(tokens, |currency| GameEvent::CurrencyChanged {
                    value: currency,
                })
                .await?
            }
            "level_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerLevelChanged { value }).await?
            }
            "strength_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerStrengthChanged { value }).await?
            }
            "stamina_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerStaminaChanged { value }).await?
            }
            "dexterity_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerDexterityChanged { value }).await?
            }
            "intelligence_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerIntelligenceChanged {
                    value,
                })
                .await?
            }
            "wisdom_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerWisdomChanged { value }).await?
            }
            "charisma_changed" => {
                parse_f_integer(tokens, |value| GameEvent::PlayerCharismaChanged { value }).await?
            }
            "action_started" => {
                parse_f_identifier(tokens, |identifier| GameEvent::ActionStarted {
                    id: identifier,
                })
                .await?
            }
            "action_completed" => {
                parse_f_identifier(tokens, |identifier| GameEvent::ActionCompleted {
                    id: identifier,
                })
                .await?
            }
            "monster_killed" => {
                parse_f_identifier(tokens, |identifier| GameEvent::MonsterKilled {
                    id: identifier,
                })
                .await?
            }
            "monster_failed" => {
                parse_f_identifier(tokens, |identifier| GameEvent::MonsterFailed {
                    id: identifier,
                })
                .await?
            }
            _ => {
                return Err(ParserError::with_coordinates(
                    ParserErrorKind::UnexpectedGameEvent(identifier),
                    range,
                ))
            }
        })
    } else {
        Err(unexpected_eof())
    }
}

async fn parse_f_currency(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    constructor: impl FnOnce(Currency) -> GameEvent,
) -> Result<GameEvent, ParserError> {
    expect_open_parenthesis(tokens).await?;
    let integer = expect_integer(tokens).await?.element;
    expect_close_parenthesis(tokens).await?;
    Ok(constructor(Currency::from_copper(integer.into())))
}

async fn parse_f_integer(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    constructor: impl FnOnce(u64) -> GameEvent,
) -> Result<GameEvent, ParserError> {
    expect_open_parenthesis(tokens).await?;
    let integer = expect_integer(tokens).await?.element;
    expect_close_parenthesis(tokens).await?;
    Ok(constructor(integer))
}

async fn parse_f_identifier(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    constructor: impl FnOnce(String) -> GameEvent,
) -> Result<GameEvent, ParserError> {
    expect_open_parenthesis(tokens).await?;
    let identifier = expect_identifier(tokens).await?.element;
    expect_close_parenthesis(tokens).await?;
    Ok(constructor(identifier))
}

async fn expect_identifier(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<RangedElement<String>, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::Identifier(identifier)) => {
            Ok(RangedElement::new(identifier, range))
        }
        other => Err(ParserError::with_coordinates(
            ParserErrorKind::ExpectedIdentifier(other),
            range,
        )),
    }
}

async fn expect_integer(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<RangedElement<u64>, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::Integer(integer)) => {
            Ok(RangedElement::new(integer, range))
        }
        other => Err(ParserError::with_coordinates(
            ParserErrorKind::ExpectedInteger(other.into()),
            range,
        )),
    }
}

async fn expect_open_parenthesis(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<CharacterCoordinateRange, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::OpenParenthesis) => Ok(range),
        other => Err(ParserError::with_coordinates(
            ParserErrorKind::ExpectedOpenParenthesis(other),
            range,
        )),
    }
}

async fn expect_close_parenthesis(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<CharacterCoordinateRange, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::CloseParenthesis) => Ok(range),
        other => Err(ParserError::with_coordinates(
            ParserErrorKind::ExpectedCloseParenthesis(other),
            range,
        )),
    }
}

async fn expect_comma(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<CharacterCoordinateRange, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::Comma) => Ok(range),
        other => Err(ParserError::with_coordinates(
            ParserErrorKind::ExpectedComma(other),
            range,
        )),
    }
}

async fn expect_any(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<Token, ParserError> {
    match tokens.next().await? {
        Some(token) => Ok(token),
        None => Err(unexpected_eof()),
    }
}
