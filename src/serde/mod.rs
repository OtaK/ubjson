pub mod de;
pub mod ser;

#[derive(Debug, thiserror::Error)]
pub enum UbjsonSerdeError {
    #[error("The buffer is not a complete deserialization payload; Trailing data detected")]
    TrailingData,
    #[error("Missing value in map")]
    MissingValueInMap,
    #[error("{0}")]
    SerdeMessage(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl serde::ser::Error for UbjsonSerdeError {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        Self::SerdeMessage(msg.to_string())
    }
}

impl serde::de::Error for UbjsonSerdeError {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        Self::SerdeMessage(msg.to_string())
    }
}
