#![allow(dead_code)]

mod marker;
pub use marker::*;
mod container;
pub use container::*;

pub const MIME_TYPE: &str = "application/ubjson";
pub const FILE_EXT: &str = "ubj";

fn parse_marker(i: &[u8]) -> nom::IResult<&[u8], Marker> {
    use std::convert::TryFrom;
    let (i, _) = nom::bytes::streaming::tag(&[Marker::ArrayStart as u8])(i)?;
    let (i, marker) = nom::combinator::map_res(nom::number::streaming::be_u8, Marker::try_from)(i)?;
    let (i, _) = nom::bytes::streaming::tag(&[Marker::ArrayEnd as u8])(i)?;
    Ok((i, marker))
}

fn parse_length(i: &[u8]) -> nom::IResult<&[u8], usize> {
    let (i, marker) = parse_marker(i)?;
    let (i, container) = Container::parse_from_marker(marker, i)?;
    let ret = match container {
        Container::Int8(n) => n as usize,
        Container::Uint8(n) => n as usize,
        Container::Int16(n) => n as usize,
        Container::Int32(n) => n as usize,
        Container::Int64(n) => n as usize,
        _ => return Err(nom::Err::Failure(nom::error::make_error(i, nom::error::ErrorKind::Digit))),
    };

    Ok((i, ret))
}
