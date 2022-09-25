use crate::game_state::time::GameTime;
use crate::game_template::parser::character_iterator::{
    CharacterCoordinateRange, CharacterIterator, CharacterIteratorWithCoordinates,
    PeekableCharacterIteratorWithCoordinates,
};
use crate::game_template::parser::error::{ParserError, ParserErrorKind};
use async_std::io::Read;
use log::trace;

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    range: CharacterCoordinateRange,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Section(SectionTokenKind),
    Key(KeyTokenKind),
    Value(ValueTokenKind),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SectionTokenKind {
    Initialisation,

    BuiltinAction,
    Action,
    QuestAction,
    Quest,
    Location,
    ExplorationEvent,
    Monster,
}

#[derive(Debug, Clone)]
pub enum KeyTokenKind {
    Name,
    Url,
    Progressive,
    SimplePast,
    Title,
    Description,

    Strength,
    Stamina,
    Dexterity,
    Intelligence,
    Wisdom,
    Charisma,
    Currency,

    Type,
    Duration,
    Events,
    Monsters,
    Hitpoints,

    Activation,
    Deactivation,
    Completion,
    Failure,

    StartingLocation,
    StartingTime,
}

#[derive(Debug, Clone)]
pub enum ValueTokenKind {
    OpenParenthesis,
    CloseParenthesis,
    Comma,

    Integer(u64),
    Float(f64),
    Time(GameTime),
    Identifier(String),
    String(String),
}

pub struct TokenIterator<Input> {
    input: PeekableCharacterIteratorWithCoordinates<Input>,
    is_first_of_line: bool,
}

impl<Input> TokenIterator<Input> {
    pub fn new(input: Input) -> Self {
        Self {
            input: PeekableCharacterIteratorWithCoordinates::new(
                CharacterIteratorWithCoordinates::new(CharacterIterator::with_capacity(
                    1024 * 1024,
                    input,
                )),
            ),
            is_first_of_line: true,
        }
    }
}

impl<Input: Read + Unpin> TokenIterator<Input> {
    pub async fn next(&mut self) -> Result<Option<Token>, ParserError> {
        self.skip_whitespace().await?;
        let is_first_of_line = self.is_first_of_line;
        self.is_first_of_line = false;

        if let Some(first_character) = self.input.next().await? {
            let mut word = String::new();
            word.push(first_character.character());
            let mut range = first_character.range();

            if is_first_of_line {
                range.merge(
                    self.read_until(&mut word, char::is_whitespace)
                        .await?
                        .unwrap_or(range),
                );
                trace!("Word at first of line: {word:?}");

                match word.as_str() {
                    "INITIALISATION" => Ok(Some(Token::new(
                        SectionTokenKind::Initialisation.into(),
                        range,
                    ))),
                    "BUILTIN_ACTION" => Ok(Some(Token::new(
                        SectionTokenKind::BuiltinAction.into(),
                        range,
                    ))),
                    "ACTION" => Ok(Some(Token::new(SectionTokenKind::Action.into(), range))),
                    "QUEST_ACTION" => Ok(Some(Token::new(
                        SectionTokenKind::QuestAction.into(),
                        range,
                    ))),
                    "QUEST" => Ok(Some(Token::new(SectionTokenKind::Quest.into(), range))),
                    "LOCATION" => Ok(Some(Token::new(SectionTokenKind::Location.into(), range))),
                    "EXPLORATION_EVENT" => Ok(Some(Token::new(
                        SectionTokenKind::ExplorationEvent.into(),
                        range,
                    ))),
                    "MONSTER" => Ok(Some(Token::new(SectionTokenKind::Monster.into(), range))),

                    "name" => Ok(Some(Token::new(TokenKind::Key(KeyTokenKind::Name), range))),
                    "url" => Ok(Some(Token::new(TokenKind::Key(KeyTokenKind::Url), range))),
                    "progressive" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Progressive),
                        range,
                    ))),
                    "simple_past" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::SimplePast),
                        range,
                    ))),
                    "title" => Ok(Some(Token::new(TokenKind::Key(KeyTokenKind::Title), range))),
                    "description" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Description),
                        range,
                    ))),

                    "str" | "strength" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Strength),
                        range,
                    ))),
                    "sta" | "stamina" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Stamina),
                        range,
                    ))),
                    "dex" | "dexterity" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Dexterity),
                        range,
                    ))),
                    "int" | "intelligence" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Intelligence),
                        range,
                    ))),
                    "wis" | "wisdom" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Wisdom),
                        range,
                    ))),
                    "chr" | "charisma" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Charisma),
                        range,
                    ))),
                    "currency" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Currency),
                        range,
                    ))),

                    "type" => Ok(Some(Token::new(TokenKind::Key(KeyTokenKind::Type), range))),
                    "duration" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Duration),
                        range,
                    ))),
                    "events" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Events),
                        range,
                    ))),
                    "monster" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Monsters),
                        range,
                    ))),
                    "hitpoints" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Hitpoints),
                        range,
                    ))),

                    "activation" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Activation),
                        range,
                    ))),
                    "deactivation" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Deactivation),
                        range,
                    ))),
                    "completion" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Completion),
                        range,
                    ))),
                    "failure" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::Failure),
                        range,
                    ))),

                    "starting_location" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::StartingLocation),
                        range,
                    ))),
                    "starting_time" => Ok(Some(Token::new(
                        TokenKind::Key(KeyTokenKind::StartingTime),
                        range,
                    ))),

                    _ => Err(ParserError::with_coordinates(
                        ParserErrorKind::IllegalKeyword(word),
                        range,
                    )),
                }
            } else {
                match first_character.character() {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        range.merge(
                            self.read_until(&mut word, |character| {
                                character.is_whitespace()
                                    || character == '('
                                    || character == ')'
                                    || character == ','
                            })
                            .await?
                            .unwrap_or(range),
                        );
                        trace!("Numeric value later in line: {word:?}");

                        if let Ok(integer) = word.parse() {
                            Ok(Some(Token::new(
                                ValueTokenKind::Integer(integer).into(),
                                range,
                            )))
                        } else if let Ok(float) = word.parse() {
                            Ok(Some(Token::new(ValueTokenKind::Float(float).into(), range)))
                        } else {
                            let mut time = GameTime::zero();
                            for summand in word.split('+') {
                                let summand = summand.trim();
                                if summand.is_empty() {
                                    return Err(ParserError::with_coordinates(
                                        ParserErrorKind::MalformedTimeString(word),
                                        range,
                                    ));
                                }

                                let last_character_index =
                                    summand.char_indices().rev().next().unwrap().0;
                                let (number, unit) = summand.split_at(last_character_index);
                                let number = number.trim();
                                let number_float = number.parse().map_err(|_| {
                                    ParserError::with_coordinates(
                                        ParserErrorKind::MalformedTimeString(word.clone()),
                                        range,
                                    )
                                })?;

                                time += match unit {
                                    "s" => GameTime::from_seconds_f64(number_float),
                                    "m" => GameTime::from_minutes_f64(number_float),
                                    "h" => GameTime::from_hours_f64(number_float),
                                    "d" => GameTime::from_days_f64(number_float),
                                    "w" => GameTime::from_weeks_f64(number_float),
                                    "y" => GameTime::from_years_f64(number_float),
                                    "e" => {
                                        let number_int = number.parse().map_err(|_| {
                                            ParserError::with_coordinates(
                                                ParserErrorKind::MalformedTimeString(word.clone()),
                                                range,
                                            )
                                        })?;
                                        GameTime::from_eras(number_int).ok_or_else(|| {
                                            ParserError::with_coordinates(
                                                ParserErrorKind::MalformedTimeString(word.clone()),
                                                range,
                                            )
                                        })?
                                    }
                                    _ => {
                                        return Err(ParserError::with_coordinates(
                                            ParserErrorKind::MalformedTimeString(word.clone()),
                                            range,
                                        ))
                                    }
                                };
                            }

                            Ok(Some(Token::new(ValueTokenKind::Time(time).into(), range)))
                        }
                    }
                    '(' => Ok(Some(Token::new(
                        ValueTokenKind::OpenParenthesis.into(),
                        range,
                    ))),
                    ')' => Ok(Some(Token::new(
                        ValueTokenKind::CloseParenthesis.into(),
                        range,
                    ))),
                    ',' => Ok(Some(Token::new(ValueTokenKind::Comma.into(), range))),
                    _ => {
                        range.merge(
                            self.read_until(&mut word, |character| {
                                character.is_whitespace()
                                    || character == '('
                                    || character == ')'
                                    || character == ','
                            })
                            .await?
                            .unwrap_or(range),
                        );
                        trace!("Word later in line: {word:?}");
                        if word
                            .chars()
                            .all(|character| character.is_ascii_alphanumeric() || character == '_')
                        {
                            Ok(Some(Token::new(
                                ValueTokenKind::Identifier(word).into(),
                                range,
                            )))
                        } else {
                            Ok(Some(Token::new(ValueTokenKind::String(word).into(), range)))
                        }
                    }
                }
            }
        } else {
            Ok(None)
        }
    }

    pub async fn expect_string_value(&mut self) -> Result<RangedElement<String>, ParserError> {
        let skipped = self.skip_whitespace().await?;
        if self.is_first_of_line {
            Err(ParserError::with_coordinates(
                ParserErrorKind::ExpectedNonemptyString,
                skipped.unwrap(),
            ))
        } else {
            let mut result = String::new();
            let range = self
                .read_until(&mut result, |character| {
                    character == '\n' || character == '\r'
                })
                .await?;
            Ok(RangedElement::new(result, range.unwrap()))
        }
    }

    pub async fn is_first_of_line(&mut self) -> Result<bool, ParserError> {
        self.skip_whitespace().await?;
        Ok(self.is_first_of_line)
    }

    async fn skip_whitespace(&mut self) -> Result<Option<CharacterCoordinateRange>, ParserError> {
        let mut range: Option<CharacterCoordinateRange> = None;
        while let Some(character) = self.input.peek().await? {
            if character.character().is_whitespace() {
                if character.character() == '\n' {
                    self.is_first_of_line = true;
                }

                if let Some(range) = range.as_mut() {
                    range.merge(character.range());
                } else {
                    range = Some(character.range());
                }
                self.input.next().await?;
            } else {
                break;
            }
        }
        trace!("Skipped whitespace {range:?}");
        Ok(range)
    }

    async fn read_until(
        &mut self,
        string: &mut String,
        condition: impl Fn(char) -> bool,
    ) -> Result<Option<CharacterCoordinateRange>, ParserError> {
        debug_assert!(condition('\n'));
        let mut range: Option<CharacterCoordinateRange> = None;
        while let Some(character) = self.input.peek().await? {
            if condition(character.character()) {
                break;
            }

            self.input.next().await?;
            string.push(character.character());
            if let Some(range) = range.as_mut() {
                range.merge(character.range());
            } else {
                range = Some(character.range());
            }
        }
        Ok(range)
    }
}

impl Token {
    pub fn new(kind: TokenKind, range: CharacterCoordinateRange) -> Self {
        Self { kind, range }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn range(&self) -> CharacterCoordinateRange {
        self.range
    }

    pub fn decompose(self) -> (TokenKind, CharacterCoordinateRange) {
        (self.kind, self.range)
    }

    pub fn error(self, error_kind: impl FnOnce(TokenKind) -> ParserErrorKind) -> ParserError {
        ParserError::with_coordinates(error_kind(self.kind), self.range)
    }
}

impl From<SectionTokenKind> for TokenKind {
    fn from(kind: SectionTokenKind) -> Self {
        TokenKind::Section(kind)
    }
}

impl From<ValueTokenKind> for TokenKind {
    fn from(kind: ValueTokenKind) -> Self {
        TokenKind::Value(kind)
    }
}

#[derive(Debug, Clone)]
pub struct RangedElement<T> {
    pub element: T,
    pub range: CharacterCoordinateRange,
}

impl<T> RangedElement<T> {
    pub fn new(element: T, range: CharacterCoordinateRange) -> Self {
        Self { element, range }
    }

    pub fn decompose(self) -> (T, CharacterCoordinateRange) {
        (self.element, self.range)
    }
}
