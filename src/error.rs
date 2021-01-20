use nom::error::{ParseError, FromExternalError};
use crate::{Container, Marker, UbjsonSerdeError};

#[derive(Debug, thiserror::Error)]
pub enum UbjsonError<'a> {
    #[error("Conversion failed, expected {expected:?}, got {actual:?}")]
    ConversionFailed {
        expected: Container<'a>,
        actual: Container<'a>,
    },
    #[error("Encountered an unexpected Marker: expected {expected:?}, got {actual:?}")]
    UnexpectedMarker {
        expected: Marker,
        actual: Marker,
    },
    #[error("Extraneous marker detected without an associated start: {0:?}")]
    ExtraMarker(Marker),
    #[error("Couldn't parse data length, missing or different type")]
    LengthParseError,
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Not enough data to parse")]
    Incomplete,
    #[error("{0:?}")]
    ParseError(nom::error::VerboseError<&'a [u8]>),
    #[error(transparent)]
    NumericConversionError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    MarkerEnumConversionError(#[from] num_enum::TryFromPrimitiveError<Marker>),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    SerdeError(#[from] UbjsonSerdeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl<'a> ParseError<&'a [u8]> for UbjsonError<'a> {
    fn from_error_kind(input: &'a [u8], kind: nom::error::ErrorKind) -> Self {
        Self::ParseError(nom::error::VerboseError::from_error_kind(input, kind))
    }

    fn append(input: &'a [u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            Self::ParseError(error) => Self::ParseError(nom::error::VerboseError::append(input, kind, error)),
            other => other
        }
    }
}

impl<I: std::fmt::Debug, E: std::error::Error + Send + Sync + 'static> FromExternalError<I, E> for UbjsonError<'_> where Self: From<E> {
    fn from_external_error(_: I, _: nom::error::ErrorKind, e: E) -> Self {
        e.into()
    }
}


impl serde::ser::Error for UbjsonError<'_> {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        Self::SerdeError(UbjsonSerdeError::SerdeMessage(msg.to_string()))
    }
}

impl serde::de::Error for UbjsonError<'_> {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        Self::SerdeError(UbjsonSerdeError::SerdeMessage(msg.to_string()))
    }
}

impl<'a> From<nom::Err<UbjsonError<'a>>> for UbjsonError<'a> {
    fn from(e: nom::Err<UbjsonError<'a>>) -> Self {
        match e {
            nom::Err::Incomplete(_) => UbjsonError::Incomplete,
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}


pub type UbjsonResult<'a, T> = Result<T, UbjsonError<'a>>;
