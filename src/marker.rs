use crate::{Container, Parsable, UbjsonError, values::string::StringValue, parse_length, parse_one};

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum Marker {
    Null = b'Z',
    Noop = b'N',
    True = b'T',
    False = b'F',
    Int8 = b'i',
    Uint8 = b'U',
    Int16 = b'I',
    Int32 = b'l',
    Int64 = b'L',
    Float32 = b'd',
    Float64 = b'D',
    HighPrecisionNumber = b'H',
    Char = b'C',
    String = b'S',
    ArrayStart = b'[',
    ArrayEnd = b']',
    ObjectStart = b'{',
    ObjectEnd = b'}',
}

impl<'a> Parsable<'a> for Marker {
    fn parse(i: &'a [u8]) -> nom::IResult<&'a [u8], Self, UbjsonError> {
        use std::convert::TryFrom as _;
        let (i, marker) = nom::combinator::map_res(nom::number::streaming::be_u8, Marker::try_from)(i)?;
        Ok((i, marker))
    }
}

impl Marker {
    pub fn parse_to_container(self, i: &[u8]) -> nom::IResult<&[u8], Container, UbjsonError> {
        match self {
            Marker::Null => Ok((i, Container::Null)),
            Marker::Noop => Ok((i, Container::Noop)),
            Marker::True => Ok((i, Container::Boolean(true))),
            Marker::False => Ok((i, Container::Boolean(false))),
            Marker::Int8 => nom::combinator::map(nom::number::streaming::be_i8, |n| Container::Int8(n))(i),
            Marker::Uint8 => nom::combinator::map(nom::number::streaming::be_u8, |n| Container::Uint8(n))(i),
            Marker::Int16 => nom::combinator::map(nom::number::streaming::be_i16, |n| Container::Int16(n))(i),
            Marker::Int32 => nom::combinator::map(nom::number::streaming::be_i32, |n| Container::Int32(n))(i),
            Marker::Int64 => nom::combinator::map(nom::number::streaming::be_i64, |n| Container::Int64(n))(i),
            Marker::Float32 => nom::combinator::map(nom::number::streaming::be_f32, |n| Container::Float32(n))(i),
            Marker::Float64 => nom::combinator::map(nom::number::streaming::be_f64, |n| Container::Float64(n))(i),
            Marker::Char => nom::combinator::map(nom::number::streaming::be_u8, |n| Container::Char(n as char))(i),
            Marker::HighPrecisionNumber => {
                let (i, result) = StringValue::parse(i)?;
                Ok((i, Container::HighPrecisionNumber(result.unwrap().into())))
            },
            Marker::String => {
                let (i, result) = StringValue::parse(i)?;
                Ok((i, Container::String(result.unwrap().into())))
            },
            Marker::ArrayStart => {
                let (i, mut specialized_parser, mut maybe_count) = {
                    let (i, maybe_type_marker): (&[u8], Option<&[u8]>) = nom::combinator::opt(nom::bytes::streaming::tag("$"))(i)?;
                    let (i, mut maybe_type): (&[u8], Option<Marker>) = nom::combinator::cond(maybe_type_marker.is_some(), Marker::parse)(i)?;
                    let (i, maybe_count_marker): (&[u8], Option<&[u8]>) = nom::combinator::opt(nom::bytes::streaming::tag("#"))(i)?;
                    let (i, maybe_count): (&[u8], Option<usize>) = nom::combinator::cond(maybe_count_marker.is_some(), parse_length)(i)?;

                    let specialized_parser = if let Some(mtype) = maybe_type.take() {
                        Some(move |i| -> nom::IResult<&[u8], Container, UbjsonError> {
                            let (i, marker) = Marker::parse(i)?;
                            if marker != mtype {
                                return Err(nom::Err::Error(UbjsonError::UnexpectedMarker {
                                    expected: mtype,
                                    actual: marker
                                }));
                            }
                            marker.parse_to_container(i)
                        })
                    } else {
                        None
                    };

                    (i, specialized_parser, maybe_count)
                };

                let (i, values) = if let Some(count) = maybe_count.take() {
                    if let Some(specialized_parser) = specialized_parser.take() {
                        nom::multi::many_m_n(
                            count,
                            count,
                            specialized_parser,
                        )(i)
                    } else {
                        nom::multi::many_m_n(
                            count,
                            count,
                            parse_one,
                        )(i)
                    }
                } else {
                    let (i, (values, _)) = nom::multi::many_till(parse_one, nom::bytes::streaming::tag(&[Marker::ArrayEnd as u8]))(i)?;
                    Ok((i, values))
                }?;

                Ok((i, Container::Array(values)))
            },
            Marker::ObjectStart => {
                let (i, mut specialized_value_parser, mut maybe_count) = {
                    let (i, maybe_type_marker): (&[u8], Option<&[u8]>) = nom::combinator::opt(nom::bytes::streaming::tag("$"))(i)?;
                    let (i, mut maybe_type): (&[u8], Option<Marker>) = nom::combinator::cond(maybe_type_marker.is_some(), Marker::parse)(i)?;
                    let (i, maybe_count_marker): (&[u8], Option<&[u8]>) = nom::combinator::opt(nom::bytes::streaming::tag("#"))(i)?;
                    let (i, maybe_count): (&[u8], Option<usize>) = nom::combinator::cond(maybe_count_marker.is_some(), parse_length)(i)?;

                    let specialized_parser = if let Some(mtype) = maybe_type.take() {
                        Some(move |i| -> nom::IResult<&[u8], Container, UbjsonError> {
                            let (i, marker) = Marker::parse(i)?;
                            if marker != mtype {
                                return Err(nom::Err::Error(UbjsonError::UnexpectedMarker {
                                    expected: mtype,
                                    actual: marker
                                }));
                            }
                            marker.parse_to_container(i)
                        })
                    } else {
                        None
                    };

                    (i, specialized_parser, maybe_count)
                };

                let (i, values) = if let Some(count) = maybe_count.take() {
                    if let Some(specialized_value_parser) = specialized_value_parser.take() {
                        nom::multi::many_m_n(
                            count,
                            count,
                            nom::sequence::pair(StringValue::parse, specialized_value_parser),
                        )(i)
                    } else {
                        nom::multi::many_m_n(
                            count,
                            count,
                            nom::sequence::pair(StringValue::parse, parse_one),
                        )(i)
                    }
                } else {
                    let (i, (values, _)) = nom::multi::many_till(nom::sequence::pair(StringValue::parse, parse_one), nom::bytes::streaming::tag(&[Marker::ObjectEnd as u8]))(i)?;
                    Ok((i, values))
                }?;

                let hash: std::collections::HashMap<std::borrow::Cow<str>, Container, _> = values.into_iter().map(|(sval, container)| (sval.unwrap(), container)).collect();
                Ok((i, Container::Object(hash)))
            },
            marker => Err(nom::Err::Error(UbjsonError::ExtraMarker(marker))),
        }
    }
}
