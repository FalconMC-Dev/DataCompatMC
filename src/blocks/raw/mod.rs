use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::Deserialize;

use self::property::PropertyKind;

pub mod de;
// pub mod modern;
pub mod property;

#[derive(Debug, Deserialize)]
pub struct RawBlockData<'raw> {
    #[serde(borrow, default)]
    properties: LinkedHashMap<&'raw str, Vec<&'raw str>, RandomState>,
    #[serde(borrow)]
    states: Vec<RawBlockState<'raw>>,
}

impl<'raw> RawBlockData<'raw> {
    pub fn property_count(&self) -> usize { self.properties.len() }

    pub fn state_count(&self) -> usize {
        let mut number = 1;
        for (_, property) in &self.properties {
            number *= property.len()
        }
        number
    }

    pub fn properties<'b>(&'b self) -> impl Iterator<Item = (&'raw str, PropertyKind<'raw>)> + 'b {
        self.properties
            .iter()
            .filter_map(|(name, values)| PropertyKind::try_from(values.as_slice()).ok().map(|property| (*name, property)))
    }
}

#[derive(Debug, Deserialize)]
pub struct RawBlockState<'raw> {
    #[serde(borrow, default)]
    properties: LinkedHashMap<&'raw str, &'raw str>,
    id: i32,
    #[serde(default)]
    default: bool,
}
