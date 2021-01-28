use std::{borrow::Cow, collections::HashMap, convert::TryInto};

use crate::UbjsonError;

/// Container for UBJSON values
#[derive(Debug, Clone, PartialEq)]
pub enum Container<'a> {
    Null,
    Noop,
    Boolean(bool),
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    HighPrecisionNumber(Cow<'a, str>),
    Char(char),
    String(Cow<'a, str>),
    Array(Vec<Container<'a>>),
    Object(HashMap<Cow<'a, str>, Container<'a>>),
}

impl<'a> TryInto<String> for Container<'a> {
    type Error = UbjsonError<'a>;

    fn try_into(self) -> Result<String, Self::Error> {
        if let Self::String(value) = self {
            Ok(value.into())
        } else {
            Err(UbjsonError::ConversionFailed {
                expected: Self::String("any".into()),
                actual: self
            })
        }
    }
}

impl<'a> TryInto<bool> for Container<'a> {
    type Error = UbjsonError<'a>;

    fn try_into(self) -> Result<bool, Self::Error> {
        if let Self::Boolean(value) = self {
            Ok(value)
        } else {
            Err(UbjsonError::ConversionFailed {
                expected: Self::Boolean(true),
                actual: self
            })
        }
    }
}

// impl<T> std::convert::TryInto<Option<T>> for Container where T: std::convert::TryFrom<Container> {
//     type Error = crate::UbjsonError;

//     fn try_into(self) -> Result<Option<T>, Self::Error> {
//         match self {
//             Container::Null => Ok(None),
//             Container::Noop => Ok(None),
//             _ => Ok(Some(T::try_from(self)?)),
//         }
//     }
// }

impl<'a> TryInto<i8> for Container<'a> {
    type Error = crate::UbjsonError<'a>;
    fn try_into(self) -> Result<i8, Self::Error> {
        match self {
            Self::Int8(value) => Ok(value),
            _ => Err(crate::UbjsonError::ConversionFailed {
                expected: Container::Int8(0),
                actual: self
            })
        }
    }
}

impl<'a> TryInto<usize> for Container<'a> {
    type Error = crate::UbjsonError<'a>;
    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Self::Uint8(value) => Ok(value as usize),
            Self::Int8(value) => Ok(value.try_into()?),
            Self::Int16(value) => Ok(value.try_into()?),
            Self::Int32(value) => Ok(value.try_into()?),
            Self::Int64(value) => Ok(value.try_into()?),
            _ => Err(crate::UbjsonError::LengthParseError)
        }
    }
}
