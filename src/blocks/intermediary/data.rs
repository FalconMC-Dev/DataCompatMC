use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};

use super::MetaData;
use crate::blocks::raw::property::EnumProperty;
use crate::util::identifier::Identifier;

/// A shorter form of the property list of the compact format.
///
/// This is a mapping of a property name with corresponding property values.
pub type PropertyList<'raw> = LinkedHashMap<&'raw str, EnumProperty<'raw>, RandomState>;
/// A shorter form of the block list of the compact format.
pub type BlockList<'raw> = LinkedHashMap<Identifier<'raw>, ModernBlockData<'raw>, RandomState>;

/// The compact blockstates format.
///
/// In this format there are two lists:
/// - The first list is a list of all possible properties. i.e. a property name
///   mapped to more than one property value.
/// - The second list is a list of all the blocks. i.e. a collection of
///   blockstates, if the block has one or more properties it will have these
///   listed referring to the first list when it's an enum property. If the
///   block has more than one blockstate, there will also be a `default_id`
///   field.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModernBlockList<'raw> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetaData<'raw>>,
    #[serde(borrow)]
    pub properties: PropertyList<'raw>,
    #[serde(borrow)]
    pub blocks: BlockList<'raw>,
}

impl<'raw> ModernBlockList<'raw> {
    pub(crate) fn new(metadata: Option<MetaData<'raw>>, properties: PropertyList<'raw>, blocks: BlockList<'raw>) -> Self {
        ModernBlockList {
            metadata,
            properties,
            blocks,
        }
    }
}

/// Compact way of identifying block data.
///
/// Only if the block has one ore more properties, the
/// [`ModernBlockData::kinds`] will be serialized. If the block has more than
/// one blockstate, a default_id field will be serialized as well.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModernBlockData<'raw> {
    #[serde(borrow, skip_serializing_if = "LinkedHashMap::is_empty", rename = "properties")]
    #[serde(default)]
    pub kinds: LinkedHashMap<&'raw str, PropertyValue<'raw>, RandomState>,
    #[serde(rename = "base")]
    pub base_id: i32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "default")]
    pub default_id: Option<i32>,
}

impl<'raw> ModernBlockData<'raw> {
    pub fn new(kinds: LinkedHashMap<&'raw str, PropertyValue<'raw>, RandomState>, base_id: i32, default_id: Option<i32>) -> Self {
        ModernBlockData {
            kinds,
            base_id,
            default_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue<'raw> {
    Range([u8; 2]),
    #[serde(borrow)]
    Text(&'raw str),
}

impl<'raw> PropertyValue<'raw> {
    pub fn bool() -> Self { Self::Text("bool") }

    pub fn enum_name(value: &'raw str) -> Self { Self::Text(value) }

    pub fn range(start: u8, end: u8) -> Self { Self::Range([start, end]) }
}
