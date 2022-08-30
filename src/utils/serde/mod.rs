use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{ser, Deserialize, Serialize, Serializer};
use std::cell::Cell;
use std::fmt::Formatter;
use std::marker::PhantomData;

pub struct IteratorSerializer<I> {
    iterator: Cell<Option<I>>,
}

impl<I> IteratorSerializer<I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: Some(iterator).into(),
        }
    }
}

impl<I: Iterator> Serialize for IteratorSerializer<I>
where
    <I as Iterator>::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let iterator = self
            .iterator
            .replace(None)
            .ok_or_else(|| ser::Error::custom("IteratorSerializer cannot be serialized twice"))?;
        let (min_size, max_size) = iterator.size_hint();
        let exact_size = if Some(min_size) == max_size {
            Some(min_size)
        } else {
            None
        };
        let mut sequence = serializer.serialize_seq(exact_size)?;
        for element in iterator {
            sequence.serialize_element(&element)?;
        }
        sequence.end()
    }
}

pub struct SequenceToVecDeserializer<T> {
    phantom_data: PhantomData<T>,
}

impl<T> Default for SequenceToVecDeserializer<T> {
    fn default() -> Self {
        Self {
            phantom_data: Default::default(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for SequenceToVecDeserializer<T> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a compiled quest")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut result = if let Some(size) = seq.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(element) = seq.next_element()? {
            result.push(element);
        }
        Ok(result)
    }
}
