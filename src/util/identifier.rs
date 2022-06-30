use std::fmt::{Display, Formatter};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::separated_pair;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::Visitor;

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

struct IdentifierVisitor;

impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier<'de>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a minecraft identifier (<namespace>:<location>)")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        match Identifier::try_from(v) {
            Ok(result) => Ok(result),
            Err(error) => Err(E::custom(format!("invalid character at position {}", error)))
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Identifier<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(IdentifierVisitor)
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
