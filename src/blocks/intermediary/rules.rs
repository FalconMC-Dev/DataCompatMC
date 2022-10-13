use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};

use crate::blocks::raw::property::{EnumProperty, PropertyKind};

#[derive(Debug, Deserialize, Serialize)]
#[serde(from = "LinkedHashMap<&'raw str, EnumProperty<'raw>, RandomState>")]
pub struct ModernPropertyRules<'raw> {
    #[serde(borrow, flatten)]
    rule_data: LinkedHashMap<EnumProperty<'raw>, &'raw str, RandomState>,
}

impl<'raw> ModernPropertyRules<'raw> {
    pub fn transform(&self, name: &'raw str, property: PropertyKind<'raw>) -> (&'raw str, PropertyKind<'raw>) {
        match &property {
            PropertyKind::Enum(enum_property) => (self.rule_data.get(&enum_property).cloned().unwrap_or(name), property),
            _ => (name, property),
        }
    }
}

impl<'raw> From<LinkedHashMap<&'raw str, EnumProperty<'raw>, RandomState>> for ModernPropertyRules<'raw> {
    fn from(other: LinkedHashMap<&'raw str, EnumProperty<'raw>, RandomState>) -> Self {
        Self {
            rule_data: other.into_iter().map(|(name, values)| (values, name)).collect(),
        }
    }
}
