use std::fmt::{Display, Formatter};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::separated_pair;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Identifier<'a> {
    namespace: &'a str,
    location: &'a str,
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.location)
    }
}

impl<'a> Identifier<'a> {
    pub fn from_full(namespace: &'a str, location: &'a str) -> Self {
        Identifier {
            namespace,
            location,
        }
    }

    pub fn from_location(location: &'a str) -> Self {
        Identifier {
            namespace: "minecraft",
            location,
        }
    }

    pub fn namespace(&self) -> &'a str {
        self.namespace
    }

    pub fn location(&self) -> &'a str {
        self.location
    }
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = usize;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let namespace_domain = take_while::<_, _, Error<&'a str>>(|i| "0123456789abcdefghijklmnopqrstuvwxyz-_.".contains(i));
        let location_domain = take_while::<_, _, Error<&'a str>>(|i| "0123456789abcdefghijklmnopqrstuvwxyz-_./".contains(i));
        let namespace_location = separated_pair(namespace_domain, char(':'), location_domain);
        let location_only = map(
            take_while::<_, _, Error<&'a str>>(|i| "0123456789abcdefghijklmnopqrstuvwxyz-_./".contains(i)),
            |location: &'a str| ("minecraft", location)
        );
        let (input, (namespace, location)) = alt((namespace_location, location_only))(input).unwrap();
        if !input.is_empty() {
            Err(location.len())
        } else {
            Ok(Identifier {
                namespace,
                location,
            })
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Identifier<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: serde::Deserializer<'de>
    {
        let ident = <&'a str as Deserialize>::deserialize(deserializer)?;
        ident.try_into()
            .map_err(|e| serde::de::Error::custom(format!("invalid character at position {}", e)))
    }
}

impl<'a> Serialize for Identifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut result = String::with_capacity(self.namespace.len() + 1 + self.location.len());
        result.push_str(self.namespace);
        result.push(':');
        result.push_str(self.location);
        serializer.serialize_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, Token, assert_de_tokens_error};

    use super::*;

    #[test]
    fn test_identifier_de() {
        let identifier = Identifier::from_location("test1");
        assert_de_tokens(&identifier, &[
            Token::BorrowedStr("test1"),
        ]);
        
        let identifier = Identifier::from_full("test_2", "other/value");
        assert_de_tokens(&identifier, &[
            Token::BorrowedStr("test_2:other/value"),
        ]);
    }

    #[test]
    fn test_identifier_de_error() {
        assert_de_tokens_error::<Identifier>(
        &[
            Token::BorrowedStr("test/2:other"),
        ], "invalid character at position 6")
    }
}
