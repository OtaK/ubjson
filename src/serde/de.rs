use std::borrow::Cow;
use serde::{Deserialize, de};

use crate::{Container, UbjsonError, UbjsonResult, UbjsonSerdeError};

#[derive(Debug, Clone, Copy)]
pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl Deserializer<'_> {
    fn len(&self) -> usize {
        self.input.len()
    }
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Self { input }
    }
}

pub fn from_slice<'a, T: Deserialize<'a>>(i: &'a [u8]) -> UbjsonResult<T> {
    let mut deserializer = Deserializer::from_slice(i);
    let t = T::deserialize(&mut deserializer)?;
    let len = deserializer.len();
    if len == 0 {
        Ok(t)
    } else {
        Err(UbjsonSerdeError::TrailingData.into())
    }
}

impl<'de, 'a: 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = UbjsonError<'a>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>
    {
        let (_, container) = crate::parse_one(self.input)?;
        match container {
            crate::Container::Null => visitor.visit_none(),
            crate::Container::Noop => visitor.visit_unit(),
            crate::Container::Boolean(value) => visitor.visit_bool(value),
            crate::Container::Int8(v) => visitor.visit_i8(v),
            crate::Container::Uint8(v) => visitor.visit_u8(v),
            crate::Container::Int16(v) => visitor.visit_i16(v),
            crate::Container::Int32(v) => visitor.visit_i32(v),
            crate::Container::Int64(v) => visitor.visit_i64(v),
            crate::Container::Float32(v) => visitor.visit_f32(v),
            crate::Container::Float64(v) => visitor.visit_f64(v),
            crate::Container::HighPrecisionNumber(v) => visitor.visit_string(v.to_string()),
            crate::Container::Char(v) => visitor.visit_char(v),
            crate::Container::String(v) => visitor.visit_string(v.to_string()),
            crate::Container::Array(v) => visitor.visit_seq(ContainerSequence {
                de: self,
                items: v.into_iter()
            }),
            crate::Container::Object(v) => visitor.visit_map(ContainerMap {
                de: self,
                items: v.into_iter(),
                current: None,
            }),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[derive(Debug)]
struct ContainerSequence<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    items: std::vec::IntoIter<Container<'a>>
}

impl<'de, 'a: 'de> de::SeqAccess<'de> for ContainerSequence<'a, 'de> {
    type Error = UbjsonError<'a>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>
    {
        if self.items.next().is_none() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[derive(Debug)]
struct ContainerMap<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    items: std::collections::hash_map::IntoIter<Cow<'a, str>, Container<'a>>,
    current: Option<(Cow<'a, str>, Container<'a>)>,
}

impl<'de, 'a> de::MapAccess<'de> for ContainerMap<'a, 'de> {
    type Error = UbjsonError<'a>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>
    {
        if let Some(tuple) = self.items.next() {
            self.current = Some(tuple);
        } else {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>
    {
        if let Some(_) = self.current.take() {
            seed.deserialize(&mut *self.de)
        } else {
            Err(UbjsonError::SerdeError(UbjsonSerdeError::MissingValueInMap))
        }
    }
}
