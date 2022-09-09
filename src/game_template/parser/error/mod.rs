use crate::game_template::parser::character_iterator::CharacterCoordinateRange;
use crate::game_template::parser::tokenizer::TokenKind;
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
    UnexpectedEof,
    ExpectedIdentifier(TokenKind),
    UnknownBuiltinAction(String),
    ExpectedNonemptyString,
    ExpectedActionType(String),
    ExpectedInteger(TokenKindOrString),
    ExpectedFloat(TokenKindOrString),
    ExpectedTime(TokenKindOrString),
    UnexpectedActionKey(TokenKind),
    UnexpectedTriggerCondition(String),
    ExpectedOpenParenthesis(TokenKind),
    ExpectedCloseParenthesis(TokenKind),
    ExpectedComma(TokenKind),
    UnexpectedGameEvent(String),
    ExpectedCommaOrCloseParenthesis(TokenKind),
    DuplicateInitialisation,
    DuplicateActionIdentifier(String),
    DuplicateQuestIdentifier(String),
    DuplicateLocationIdentifier(String),
    DuplicateExplorationEventIdentifier(String),
    DuplicateMonsterIdentifier(String),
    DuplicateTriggerIdentifier(String),
    ReservedActionId(String),
    IllegalWeight(f64),
    AllWeightsZero,

    MissingSectionInitialisation,
    MissingActionWait,
    MissingActionSleep,
    MissingActionTavern,
    MissingActionExplore,

    MissingName(String),
    MissingProgressive(String),
    MissingSimplePast(String),
    MissingTitle(String),
    MissingDescription(String),

    MissingStrength(String),
    MissingStamina(String),
    MissingDexterity(String),
    MissingIntelligence(String),
    MissingWisdom(String),
    MissingCharisma(String),
    MissingCurrency(String),

    MissingType(String),
    MissingDuration(String),
    MissingEvents(String),
    MissingMonster(String),
    MissingHitpoints(String),

    MissingActivation(String),
    MissingDeactivation(String),
    MissingCompletion(String),
    MissingFailure(String),

    MissingStartingLocation(String),

    DuplicateName(String),
    DuplicateProgressive(String),
    DuplicateSimplePast(String),
    DuplicateTitle(String),
    DuplicateDescription(String),

    DuplicateStrength(String),
    DuplicateStamina(String),
    DuplicateDexterity(String),
    DuplicateIntelligence(String),
    DuplicateWisdom(String),
    DuplicateCharisma(String),
    DuplicateCurrency(String),

    DuplicateType(String),
    DuplicateDuration(String),
    DuplicateEvents(String),
    DuplicateMonster(String),
    DuplicateHitpoints(String),

    DuplicateActivation(String),
    DuplicateDeactivation(String),
    DuplicateCompletion(String),
    DuplicateFailure(String),

    DuplicateStartingLocation(String),

    UnexpectedName(String),
    UnexpectedProgressive(String),
    UnexpectedSimplePast(String),
    UnexpectedTitle(String),
    UnexpectedDescription(String),

    UnexpectedStrength(String),
    UnexpectedStamina(String),
    UnexpectedDexterity(String),
    UnexpectedIntelligence(String),
    UnexpectedWisdom(String),
    UnexpectedCharisma(String),
    UnexpectedCurrency(String),

    UnexpectedType(String),
    UnexpectedDuration(String),
    UnexpectedEvents(String),
    UnexpectedMonster(String),
    UnexpectedHitpoints(String),

    UnexpectedActivation(String),
    UnexpectedDeactivation(String),
    UnexpectedCompletion(String),
    UnexpectedFailure(String),

    UnexpectedStartingLocation(String),
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
