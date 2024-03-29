use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PropertyKind<'raw> {
    Bool,
    Int([u8; 2]),
    Enum(EnumProperty<'raw>),
}

#[derive(Debug, Error)]
pub enum PropertyKindParseError {
    #[error("Boolean property has more than 2 variants")]
    BooleanTooLong,
    #[error("First value was a boolean but second value was not")]
    BooleanInvalidType,
    #[error("Invalid integer value")]
    IntError(#[from] ParseIntError),
}

impl<'b, 'raw: 'b> TryFrom<&'b [&'raw str]> for PropertyKind<'raw> {
    type Error = PropertyKindParseError;

    fn try_from(values: &'b [&'raw str]) -> Result<Self, Self::Error> {
        if let Some(&"true") = values.first() {
            if let Some(&"false") = values.get(1) {
                if values.len() == 2 {
                    Ok(PropertyKind::Bool)
                } else {
                    Err(PropertyKindParseError::BooleanTooLong)
                }
            } else {
                Err(PropertyKindParseError::BooleanInvalidType)
            }
        } else if let Some(Ok(first)) = values.first().map(|d| d.parse::<u8>()) {
            Ok(match values.last().map(|d| d.parse::<u8>()).transpose()? {
                Some(second) => PropertyKind::Int([first, second]),
                None => PropertyKind::Int([first, first]),
            })
        } else {
            Ok(PropertyKind::Enum(EnumProperty::new(values)))
        }
    }
}

impl<'raw, 'de: 'raw> Deserialize<'de> for PropertyKind<'raw> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values = <Vec<&'de str> as Deserialize>::deserialize(deserializer)?;
        values.as_slice().try_into().map_err(serde::de::Error::custom)
    }
}

impl<'raw> Display for PropertyKind<'raw> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyKind::Bool => f.write_str("bool"),
            PropertyKind::Int(range) => f.write_fmt(format_args!("integer ({}..={})", range[0], range[1])),
            PropertyKind::Enum(property) => f.write_fmt(format_args!("enum {:?}", property.values)),
        }
    }
}

impl<'raw> PropertyKind<'raw> {
    pub fn len(&self) -> usize {
        match self {
            PropertyKind::Bool => 2,
            PropertyKind::Int(range) => (range[1] - range[0] + 1) as usize,
            PropertyKind::Enum(values) => values.fields().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == match self {
            PropertyKind::Bool => 2,
            PropertyKind::Int(range) => (range[1] - range[0] + 1) as usize,
            PropertyKind::Enum(values) => values.fields().len(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(from = "Vec<&'raw str>")]
#[serde(into = "Vec<&'raw str>")]
pub struct EnumProperty<'raw> {
    #[serde(borrow)]
    values: Vec<&'raw str>,
}

impl<'raw> EnumProperty<'raw> {
    pub fn new(values: &[&'raw str]) -> Self {
        EnumProperty {
            values: values.to_vec(),
        }
    }

    pub fn fields<'b>(&'b self) -> &'b [&'raw str] { &self.values }
}

impl<'raw> From<Vec<&'raw str>> for EnumProperty<'raw> {
    fn from(values: Vec<&'raw str>) -> Self { Self { values } }
}
impl<'raw> From<EnumProperty<'raw>> for Vec<&'raw str> {
    fn from(ep: EnumProperty<'raw>) -> Self { ep.values }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::EnumProperty;

    #[test]
    pub fn test_enum_property_de() {
        let values = vec!["value1", "value2"];
        let enum_property = EnumProperty::new(&values);

        assert_de_tokens(&enum_property, &[Token::Seq { len: Some(2) }, Token::BorrowedStr("value1"), Token::BorrowedStr("value2"), Token::SeqEnd]);
    }
}
