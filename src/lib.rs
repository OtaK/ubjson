mod error;

pub use error::*;

mod marker;
pub use marker::*;
mod container;
pub use container::*;

mod values;
pub use crate::values::*;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::*;

pub const MIME_TYPE: &str = "application/ubjson";
pub const FILE_EXT: &str = "ubj";

pub trait Parsable<'a>: Sized {
    fn parse(i: &'a [u8]) -> nom::IResult<&'a [u8], Self, UbjsonError>;
}


#[inline(always)]
pub fn parse_one<'a>(i: &'a [u8]) -> nom::IResult<&'a [u8], Container, UbjsonError> {
    Marker::parse(i).and_then(|(i, marker)| marker.parse_to_container(i))
}

#[inline(always)]
fn parse_length(i: &[u8]) -> nom::IResult<&[u8], usize, UbjsonError> {
    use std::convert::TryInto as _;
    let (i, marker) = Marker::parse(i)?;
    let (i, container) = marker.parse_to_container(i)?;
    let length = container.try_into().map_err(nom::Err::Failure)?;
    Ok((i, length))
}

#[cfg(test)]
mod test {
    use crate::parse_one;

    const COMPLEX_COUCHDB: &[u8] = include_bytes!("../test/samples/complex/CouchDB4k.ubj");
    const COMPLEX_MEDIA: &[u8] = include_bytes!("../test/samples/complex/MediaContent.ubj");
    const COMPLEX_TWITTER: &[u8] = include_bytes!("../test/samples/complex/TwitterTimeline.ubj");

    #[test]
    fn test_complex_couchdb() {
        let (_, _container) = parse_one(COMPLEX_COUCHDB).unwrap();
    }

    #[test]
    fn test_complex_media() {
        let (_, _container) = parse_one(COMPLEX_MEDIA).unwrap();
    }

    #[test]
    fn test_complex_twitter() {
        let (_, _container) = parse_one(COMPLEX_TWITTER).unwrap();
    }
}
