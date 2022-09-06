use crate::game_template::parser::error::{ParserError, ParserErrorKind};
use async_std::io::{Read, ReadExt};
use log::trace;
use std::cmp::Ordering;

pub struct CharacterIterator<Input> {
    input: Input,
    buffer: Vec<u8>,
    offset: usize,
    limit: usize,
}

pub struct CharacterIteratorWithCoordinates<Input> {
    characters: CharacterIterator<Input>,
    current_line_number: usize,
    current_column_number: usize,
}

pub struct PeekableCharacterIteratorWithCoordinates<Input> {
    iterator: CharacterIteratorWithCoordinates<Input>,
    peek: Option<Result<Option<CharacterWithCoordinates>, ParserError>>,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterWithCoordinates {
    character: char,
    coordinates: CharacterCoordinates,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterCoordinateRange {
    from: CharacterCoordinates,
    to: CharacterCoordinates,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CharacterCoordinates {
    line_number: usize,
    column_number: usize,
}

impl<Input> CharacterIterator<Input> {
    pub fn with_capacity(capacity: usize, input: Input) -> Self {
        Self {
            input,
            buffer: vec![0; capacity],
            offset: 0,
            limit: 0,
        }
    }
}

impl<Input: Read + Unpin> CharacterIterator<Input> {
    pub async fn next(&mut self) -> Result<Option<char>, ParserErrorKind> {
        loop {
            debug_assert!(
                self.offset <= self.limit && self.limit <= self.buffer.len(),
                "offset: {}; limit: {}; len: {}",
                self.offset,
                self.limit,
                self.buffer.len()
            );
            if self.offset < self.limit {
                let slice_len = (self.limit - self.offset).min(4);
                match std::str::from_utf8(&self.buffer[self.offset..self.offset + slice_len]) {
                    Ok(string) => {
                        let mut char_indices = string.char_indices();
                        let (_, character) = unsafe { char_indices.next().unwrap_unchecked() };
                        self.offset += if let Some((index, _)) = char_indices.next() {
                            index
                        } else {
                            slice_len
                        };
                        return Ok(Some(character));
                    }
                    Err(error) => {
                        let valid_up_to = error.valid_up_to();
                        if valid_up_to > 0 {
                            let valid_string = &self.buffer[self.offset..self.offset + valid_up_to];
                            self.offset += valid_up_to;
                            let character = unsafe {
                                std::str::from_utf8_unchecked(valid_string)
                                    .chars()
                                    .next()
                                    .unwrap_unchecked()
                            };
                            return Ok(Some(character));
                        }
                    }
                }
            }

            let remaining_len = self.limit - self.offset;
            if self.offset > 0 {
                for i in 0..remaining_len {
                    self.buffer[i] = self.buffer[i + self.offset];
                }
            }

            debug_assert!(self.buffer.len() > remaining_len);
            let bytes_read = self.input.read(&mut self.buffer[remaining_len..]).await?;
            trace!("Read {bytes_read} bytes");
            if bytes_read == 0 {
                return Ok(None);
            }

            self.offset = 0;
            self.limit = remaining_len + bytes_read;
        }
    }
}

impl<Input> CharacterIteratorWithCoordinates<Input> {
    pub fn new(characters: CharacterIterator<Input>) -> Self {
        Self {
            characters,
            current_line_number: 1,
            current_column_number: 1,
        }
    }
}

impl<Input: Read + Unpin> CharacterIteratorWithCoordinates<Input> {
    pub async fn next(&mut self) -> Result<Option<CharacterWithCoordinates>, ParserError> {
        match self.characters.next().await {
            Ok(Some(character)) => {
                let result = CharacterWithCoordinates::new(
                    character,
                    self.current_line_number,
                    self.current_column_number,
                );
                self.current_column_number += 1;
                if character == '\n' {
                    self.current_line_number += 1;
                    self.current_column_number = 1;
                }
                Ok(Some(result))
            }
            Ok(None) => Ok(None),
            Err(kind) => Err(ParserError::with_coordinates(
                kind,
                CharacterCoordinates::new(self.current_line_number, self.current_column_number)
                    .into(),
            )),
        }
    }
}

impl<Input> PeekableCharacterIteratorWithCoordinates<Input> {
    pub fn new(iterator: CharacterIteratorWithCoordinates<Input>) -> Self {
        Self {
            iterator,
            peek: None,
        }
    }
}

impl<Input: Read + Unpin> PeekableCharacterIteratorWithCoordinates<Input> {
    pub async fn next(&mut self) -> Result<Option<CharacterWithCoordinates>, ParserError> {
        if let Some(peek) = self.peek.take() {
            peek
        } else {
            self.iterator.next().await
        }
    }

    pub async fn peek(&mut self) -> Result<Option<CharacterWithCoordinates>, ParserError> {
        if self.peek.is_none() {
            self.peek = Some(self.iterator.next().await);
        }
        self.peek.clone().unwrap()
    }
}

impl CharacterWithCoordinates {
    fn new(character: char, line_number: usize, column_number: usize) -> Self {
        Self {
            character,
            coordinates: CharacterCoordinates {
                line_number,
                column_number,
            },
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn range(&self) -> CharacterCoordinateRange {
        self.coordinates.into()
    }

    #[allow(dead_code)]
    pub fn coordinates(&self) -> CharacterCoordinates {
        self.coordinates
    }

    #[allow(dead_code)]
    pub fn line_number(&self) -> usize {
        self.coordinates.line_number
    }

    #[allow(dead_code)]
    pub fn column_number(&self) -> usize {
        self.coordinates.column_number
    }
}

impl CharacterCoordinates {
    pub fn zero() -> Self {
        Self {
            line_number: 0,
            column_number: 0,
        }
    }

    fn new(line_number: usize, column_number: usize) -> Self {
        Self {
            line_number,
            column_number,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn column_number(&self) -> usize {
        self.column_number
    }
}

impl CharacterCoordinateRange {
    pub fn zero() -> Self {
        Self {
            from: CharacterCoordinates::zero(),
            to: CharacterCoordinates::zero(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.from = self.from.min(other.from);
        self.to = self.to.max(other.to);
    }
}

impl From<CharacterCoordinates> for CharacterCoordinateRange {
    fn from(coordinates: CharacterCoordinates) -> Self {
        let mut to = coordinates;
        to.column_number += 1;
        Self {
            from: coordinates,
            to,
        }
    }
}

impl Ord for CharacterCoordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line_number.cmp(&other.line_number) {
            Ordering::Equal => self.column_number.cmp(&other.column_number),
            ordering => ordering,
        }
    }
}

impl PartialOrd for CharacterCoordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
