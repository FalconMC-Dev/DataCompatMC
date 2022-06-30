use std::fmt::{Display, Formatter};
use std::ops::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PropertyKind<'raw> {
    Bool,
    Int(Range<u8>),
    Enum(EnumProperty<'raw>),
}

impl<'b, 'raw: 'b> From<&'b [&'raw str]> for PropertyKind<'raw> {
    fn from(raw_values: &'b [&'raw str]) -> Self {
        if raw_values.contains(&"true") && raw_values.contains(&"false") {
            return PropertyKind::Bool;
        }
        let first = raw_values.first().map(|e| e.parse::<u8>());
        if let Some(Ok(first)) = first {
            match raw_values.last().map(|e| e.parse::<u8>()).unwrap() {
                Ok(second) => return PropertyKind::Int(first..second + 1),
                Err(_) => {
                    eprintln!("Data file was corrupted, integer ranged contained non-integer elements!");
                    ::std::process::exit(exitcode::DATAERR);
                }
            }
        }
        PropertyKind::Enum(EnumProperty::new(raw_values))
    }
}

impl<'raw> Display for PropertyKind<'raw> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyKind::Bool => f.write_str("bool"),
            PropertyKind::Int(range) => f.write_fmt(format_args!("integer ({}..{})", range.start, range.end)),
            PropertyKind::Enum(property) => f.write_fmt(format_args!("enum {:?}", property.values)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
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

    pub fn fields<'b>(&'b self) -> &'b [&'raw str] {
        &self.values
    }

    pub fn fields_owned(self) -> Vec<&'raw str> {
        self.values
    }
}