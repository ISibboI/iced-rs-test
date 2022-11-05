use crate::game_state::currency::Currency;
use crate::game_state::triggers::{GameAction, GameEvent};
use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::error::{unexpected_eof, ParserError, ParserErrorKind};
use crate::game_template::parser::section::parse_section;
use crate::game_template::parser::tokenizer::{
    RangedElement, SectionTokenKind, Token, TokenIterator, TokenKind, ValueTokenKind,
};
use crate::game_template::GameTemplate;
use async_recursion::async_recursion;
use async_std::io::Read;
use event_trigger_action_system::{
    and, any_n, event_count, geq, never, none, or, sequence, Trigger, TriggerCondition,
};
use log::{debug, trace};

mod character_iterator;
pub mod error;
mod section;
mod tokenizer;

#[derive(Debug)]
pub struct WeightedIdentifier {
    pub weight: f64,
    pub identifier: String,
}

#[derive(Debug)]
pub struct ExpectedIdentifierCount {
    pub mean: f64,
    pub variance: f64,
    pub identifier: String,
}

pub async fn parse_game_template_file(
    game_template: &mut GameTemplate,
    input: impl Read + Unpin + Send,
) -> Result<(), ParserError> {
    debug!("Parsing game template file");
    parse(game_template, &mut TokenIterator::new(input)).await
}

async fn parse(
    game_template: &mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<(), ParserError> {
    let mut next_token = tokens.next().await?;
    trace!("First token: {next_token:?}");
    while let Some(token) = next_token {
        next_token = match token.kind() {
            TokenKind::Section(section) => {
                let (section_template, next_token) =
                    parse_section(game_template, tokens, section, None).await?;
                match section {
                    SectionTokenKind::Initialisation => {
                        if game_template
                            .initialisation
                            .replace(section_template.into_initialisation()?)
                            .is_some()
                        {
                            return Err(ParserError::with_coordinates(
                                ParserErrorKind::DuplicateInitialisation,
                                token.range(),
                            ));
                        };
                    }
                    SectionTokenKind::BuiltinAction => {
                        let builtin_action = section_template.into_builtin_action(game_template)?;
                        game_template.actions.push(builtin_action);
                    }
                    SectionTokenKind::Action => {
                        let action = section_template.into_action(game_template)?;
                        game_template.actions.push(action);
                    }
                    SectionTokenKind::QuestStageAction => {
                        let quest_action =
                            section_template.into_quest_stage_action(game_template)?;
                        game_template.actions.push(quest_action);
                    }
                    SectionTokenKind::Quest => {
                        let quest = section_template.into_quest(game_template)?;
                        game_template.quests.push(quest);
                    }
                    SectionTokenKind::QuestStage => {
                        return Err(token.error(|_| ParserErrorKind::UnexpectedQuestStage));
                    }
                    SectionTokenKind::Location => {
                        let location = section_template.into_location(game_template)?;
                        game_template.locations.push(location);
                    }
                    SectionTokenKind::ExplorationEvent => {
                        let exploration_event =
                            section_template.into_exploration_event(game_template)?;
                        game_template.exploration_events.push(exploration_event);
                    }
                    SectionTokenKind::Monster => {
                        let monster = section_template.into_monster(game_template)?;
                        game_template.monsters.push(monster);
                    }
                    SectionTokenKind::Item => {
                        let item = section_template.into_item(game_template)?;
                        game_template.items.push(item);
                    }
                }
                next_token
            }
            _ => return Err(token.error(ParserErrorKind::ExpectedSection)),
        };
    }

    Ok(())
}

async fn parse_trigger<'trigger>(
    game_template: &'trigger mut GameTemplate,
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
    id_str: String,
    game_actions: Vec<GameAction>,
) -> Result<&'trigger mut Trigger<GameEvent, GameAction>, ParserError> {
    let condition = parse_trigger_condition(tokens).await?;
    game_template
        .triggers
        .push(Trigger::new(id_str, condition, game_actions));
    Ok(game_template.triggers.last_mut().unwrap())
}

#[async_recursion]
async fn parse_trigger_condition(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<TriggerCondition<GameEvent>, ParserError> {
    let (identifier, range) = expect_identifier(tokens).await?.decompose();
    Ok(match identifier.as_str() {
        "none" => none(),
        "never" => never(),
        "exploration_event_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let event = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(
                GameEvent::ExplorationEventCompleted { id: event },
                count as usize,
            )
        }
        "game_event_count" => {
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
        "sequence" | "seq" => sequence(parse_trigger_condition_sequence(tokens, true).await?),
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
        "level_geq" => {
            expect_open_parenthesis(tokens).await?;
            let level = expect_integer(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            geq(GameEvent::PlayerLevelChanged { value: level })
        }
        "explore_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let location = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(
                GameEvent::ExplorationCompleted { id: location },
                count as usize,
            )
        }
        "quest_activated" => {
            expect_open_parenthesis(tokens).await?;
            let quest = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(
                GameEvent::Action(GameAction::ActivateQuest { id: quest }),
                1,
            )
        }
        "quest_completed" => {
            expect_open_parenthesis(tokens).await?;
            let quest = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(GameEvent::QuestCompleted { id: quest }, 1)
        }
        "quest_failed" => {
            expect_open_parenthesis(tokens).await?;
            let quest = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(GameEvent::Action(GameAction::FailQuest { id: quest }), 1)
        }
        "item_count" => {
            expect_open_parenthesis(tokens).await?;
            let count = expect_integer(tokens).await?.element;
            expect_comma(tokens).await?;
            let item = expect_identifier(tokens).await?.element;
            expect_close_parenthesis(tokens).await?;
            event_count(
                GameEvent::ItemCountChanged {
                    id: item,
                    count: count as usize,
                },
                1,
            )
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
        "exploration_started" => {
            parse_f_identifier(tokens, |identifier| GameEvent::ExplorationStarted {
                id: identifier,
            })
            .await?
        }
        "exploration_completed" => {
            parse_f_identifier(tokens, |identifier| GameEvent::ExplorationCompleted {
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

async fn parse_weighted_identifiers(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<RangedElement<Vec<WeightedIdentifier>>, ParserError> {
    let mut result = Vec::new();
    let mut is_first_event = true;
    let mut range: Option<CharacterCoordinateRange> = None;
    let mut has_nonzero_weight = false;

    while !tokens.is_first_of_line().await? {
        if is_first_event {
            is_first_event = false;
        } else {
            expect_comma(tokens).await?;
        }

        let mut local_range = expect_open_parenthesis(tokens).await?;
        let (weight, weight_range) = expect_float(tokens).await?.decompose();
        if !weight.is_finite() || weight < 0.0 {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::IllegalWeight(weight),
                weight_range,
            ));
        } else if weight > 0.0 {
            has_nonzero_weight = true;
        }
        expect_comma(tokens).await?;
        let identifier = expect_identifier(tokens).await?.element;
        local_range.merge(expect_close_parenthesis(tokens).await?);
        result.push(WeightedIdentifier::new(weight, identifier));
        if let Some(range) = &mut range {
            range.merge(local_range);
        } else {
            range = Some(local_range);
        }
    }

    if !has_nonzero_weight {
        return Err(ParserError::with_coordinates(
            ParserErrorKind::AllWeightsZero,
            range.unwrap_or_else(CharacterCoordinateRange::zero),
        ));
    }

    Ok(RangedElement::new(
        result,
        range.unwrap_or_else(CharacterCoordinateRange::zero),
    ))
}

async fn parse_expected_identifier_counts(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<RangedElement<Vec<ExpectedIdentifierCount>>, ParserError> {
    let mut result = Vec::new();
    let mut is_first_event = true;
    let mut range: Option<CharacterCoordinateRange> = None;

    while !tokens.is_first_of_line().await? {
        if is_first_event {
            is_first_event = false;
        } else {
            expect_comma(tokens).await?;
        }

        let mut local_range = expect_open_parenthesis(tokens).await?;
        let (mean, mean_range) = expect_float(tokens).await?.decompose();
        if !mean.is_finite() {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::IllegalMean(mean),
                mean_range,
            ));
        }
        expect_comma(tokens).await?;
        let (variance, variance_range) = expect_float(tokens).await?.decompose();
        if !variance.is_finite() {
            return Err(ParserError::with_coordinates(
                ParserErrorKind::IllegalVariance(variance),
                variance_range,
            ));
        }
        expect_comma(tokens).await?;
        let identifier = expect_identifier(tokens).await?.element;
        local_range.merge(expect_close_parenthesis(tokens).await?);
        result.push(ExpectedIdentifierCount::new(mean, variance, identifier));
        if let Some(range) = &mut range {
            range.merge(local_range);
        } else {
            range = Some(local_range);
        }
    }

    Ok(RangedElement::new(
        result,
        range.unwrap_or_else(CharacterCoordinateRange::zero),
    ))
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

async fn expect_float(
    tokens: &mut TokenIterator<impl Read + Unpin + Send>,
) -> Result<RangedElement<f64>, ParserError> {
    let (kind, range) = expect_any(tokens).await?.decompose();
    match kind {
        TokenKind::Value(ValueTokenKind::Float(float)) => Ok(RangedElement::new(float, range)),
        TokenKind::Value(ValueTokenKind::Integer(integer)) => {
            Ok(RangedElement::new(integer as f64, range))
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

impl WeightedIdentifier {
    fn new(weight: f64, identifier: String) -> Self {
        Self { weight, identifier }
    }
}

impl ExpectedIdentifierCount {
    fn new(mean: f64, variance: f64, identifier: String) -> Self {
        Self {
            mean,
            variance,
            identifier,
        }
    }
}
