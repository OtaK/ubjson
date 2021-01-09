use std::collections::HashMap;

use crate::Marker;

#[derive(Debug, Clone, PartialEq)]
pub enum Container {
    Null,
    Noop,
    True,
    False,
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    HighPrecisionNumber(String),
    Char(char),
    String(String),
    Array(Vec<Container>),
    Object(HashMap<String, Container>),
}

impl Container {
    pub fn parse_from_marker(marker: Marker, i: &[u8]) -> nom::IResult<&[u8], Self> {
        match marker {
            Marker::Null => Ok((i, Self::Null)),
            Marker::Noop => Ok((i, Self::Noop)),
            Marker::True => Ok((i, Self::True)),
            Marker::False => Ok((i, Self::False)),
            Marker::Int8 => nom::combinator::map(nom::number::streaming::be_i8, |n| Self::Int8(n))(i),
            Marker::Uint8 => nom::combinator::map(nom::number::streaming::be_u8, |n| Self::Uint8(n))(i),
            Marker::Int16 => nom::combinator::map(nom::number::streaming::be_i16, |n| Self::Int16(n))(i),
            Marker::Int32 => nom::combinator::map(nom::number::streaming::be_i32, |n| Self::Int32(n))(i),
            Marker::Int64 => nom::combinator::map(nom::number::streaming::be_i64, |n| Self::Int64(n))(i),
            Marker::Float32 => nom::combinator::map(nom::number::streaming::be_f32, |n| Self::Float32(n))(i),
            Marker::Float64 => nom::combinator::map(nom::number::streaming::be_f64, |n| Self::Float64(n))(i),
            Marker::HighPrecisionNumber => todo!(),
            Marker::Char => nom::combinator::map(nom::number::streaming::be_u8, |n| Self::Char(n as char))(i),
            Marker::String => todo!(),
            Marker::ArrayStart => todo!(),
            Marker::ArrayEnd => todo!(),
            Marker::ObjectStart => todo!(),
            Marker::ObjectEnd => todo!(),
        }
    }
}
