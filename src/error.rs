use nom::error::{ParseError, FromExternalError};
use crate::{Container, Marker};

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
    #[error("{0:?}")]
    ParseError(nom::error::VerboseError<&'a [u8]>),
    #[error(transparent)]
    NumericConversionError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    MarkerEnumConversionError(#[from] num_enum::TryFromPrimitiveError<Marker>),
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

pub type UbjsonResult<'a, T> = Result<T, UbjsonError<'a>>;
