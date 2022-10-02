use crate::game_state::player_actions::PlayerActionType;
use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::section::{
    GameTemplateSectionError, GameTemplateSectionErrorKind,
};
use crate::game_template::parser::tokenizer::{TokenKind, ValueTokenKind};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub coordinates: Option<CharacterCoordinateRange>,
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    Io(Arc<std::io::Error>),
    MalformedTimeString(String),
    IllegalKeyword(String),
    ExpectedSection(TokenKind),
    ExpectedSectionOrEnd(TokenKind),
    UnexpectedQuestStage,
    UnexpectedEof,
    ExpectedIdentifier(TokenKind),
    UnknownBuiltinAction(String),
    ExpectedNonemptyString,
    ExpectedActionType(String),
    ExpectedInteger(TokenKindOrString),
    ExpectedFloat(TokenKindOrString),
    ExpectedTime(TokenKindOrString),
    UnexpectedValue(ValueTokenKind),
    UnexpectedTriggerCondition(String),
    ExpectedOpenParenthesis(TokenKind),
    ExpectedCloseParenthesis(TokenKind),
    ExpectedComma(TokenKind),
    UnexpectedGameEvent(String),
    ExpectedCommaOrCloseParenthesis(TokenKind),
    DuplicateInitialisation,
    DuplicateActionIdentifier(String),
    DuplicateQuestIdentifier(String),
    DuplicateQuestStageIdentifier(String),
    DuplicateLocationIdentifier(String),
    DuplicateExplorationEventIdentifier(String),
    DuplicateMonsterIdentifier(String),
    DuplicateTriggerIdentifier(String),
    ReservedActionId(String),
    IllegalWeight(f64),
    AllWeightsZero,
    IllegalActionType(PlayerActionType),
    BeginWithoutEnd,
    EndWithoutBegin,

    MissingSectionInitialisation,
    MissingActionWait,
    MissingActionSleep,
    MissingActionTavern,
    MissingActionExplore,

    MissingField { id_str: String, field: String },
    DuplicateField { id_str: String, field: String },
    UnexpectedField { id_str: String, field: String },
}

#[derive(Debug, Clone)]
pub enum TokenKindOrString {
    TokenKind(TokenKind),
    String(String),
}

pub fn unexpected_eof() -> ParserError {
    ParserError::without_coordinates(ParserErrorKind::UnexpectedEof)
}

impl ParserError {
    pub fn with_coordinates(kind: ParserErrorKind, coordinates: CharacterCoordinateRange) -> Self {
        Self {
            kind,
            coordinates: Some(coordinates),
        }
    }

    pub fn without_coordinates(kind: ParserErrorKind) -> Self {
        Self {
            kind,
            coordinates: None,
        }
    }
}

impl From<TokenKind> for TokenKindOrString {
    fn from(token_kind: TokenKind) -> Self {
        Self::TokenKind(token_kind)
    }
}

impl From<String> for TokenKindOrString {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<std::io::Error> for ParserErrorKind {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Arc::new(error))
    }
}

impl From<GameTemplateSectionError> for ParserError {
    fn from(error: GameTemplateSectionError) -> Self {
        match error.kind {
            GameTemplateSectionErrorKind::MissingField => ParserError::with_coordinates(
                ParserErrorKind::MissingField {
                    id_str: error.id_str,
                    field: error.field,
                },
                error.range,
            ),
            GameTemplateSectionErrorKind::UnexpectedField => ParserError::with_coordinates(
                ParserErrorKind::UnexpectedField {
                    id_str: error.id_str,
                    field: error.field,
                },
                error.range,
            ),
            GameTemplateSectionErrorKind::DuplicateField => ParserError::with_coordinates(
                ParserErrorKind::DuplicateField {
                    id_str: error.id_str,
                    field: error.field,
                },
                error.range,
            ),
        }
    }
}
