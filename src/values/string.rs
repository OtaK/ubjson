use std::{borrow::Cow, ops::Deref};

use crate::{UbjsonError, parse_length};
use nom::combinator::map_res;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue<'a>(Cow<'a, str>);

impl<'a> Deref for StringValue<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<'a> StringValue<'a> {
    pub fn unwrap(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> crate::Parsable<'a> for StringValue<'a> {
    fn parse(i: &'a [u8]) -> nom::IResult<&'a [u8], Self, UbjsonError> {
        let (i, length) = parse_length(i)?;
        let (i, ret) = map_res(nom::bytes::streaming::take(length), |bytes: &[u8]| std::str::from_utf8(bytes).map(Cow::from))(i)?;
        Ok((i, Self(ret)))
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use crate::{Marker, Parsable};

    const STRING_SMALL: &[u8] = include_bytes!("../../test/samples/string/string_small.ubj");
    const STRING_BIG: &[u8] = include_bytes!("../../test/samples/string/string_big.ubj");

    #[test]
    fn parse_small() {
        let (i, arr_marker) = Marker::parse(STRING_SMALL).unwrap();
        assert_eq!(arr_marker, Marker::ArrayStart);
        let (i, marker) = Marker::parse(i).unwrap();
        assert_eq!(marker, Marker::String);
        let (i, container) = marker.parse_to_container(i).unwrap();
        let hello: String = container.try_into().unwrap();
        assert_eq!(hello, String::from("привет"));
        let (_, arr_marker) = Marker::parse(i).unwrap();
        assert_eq!(arr_marker, Marker::ArrayEnd);
    }

    #[test]
    fn parse_big() {
        let (i, arr_marker) = Marker::parse(STRING_BIG).unwrap();
        assert_eq!(arr_marker, Marker::ArrayStart);
        let (i, marker) = Marker::parse(i).unwrap();
        assert_eq!(marker, Marker::String);
        let (i, container) = marker.parse_to_container(i).unwrap();
        let hello: String = container.try_into().unwrap();
        assert_eq!(hello, String::from("There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc."));
        let (_, arr_marker) = Marker::parse(i).unwrap();
        assert_eq!(arr_marker, Marker::ArrayEnd);
    }
}
