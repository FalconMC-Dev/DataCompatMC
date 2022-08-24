use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use crate::blocks::raw::property::EnumProperty;
use crate::util::identifier::Identifier;

type PropertyList<'raw> = LinkedHashMap<&'raw str, EnumProperty<'raw>, RandomState>;
type BlockList<'raw> = LinkedHashMap<Identifier<'raw>, ModernBlockData<'raw>, RandomState>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModernBlockList<'raw> {
    #[serde(borrow)]
    properties: PropertyList<'raw>,
    #[serde(borrow)]
    blocks: BlockList<'raw>,
}

impl<'raw> ModernBlockList<'raw> {
    pub fn new(properties: PropertyList<'raw>, blocks: BlockList<'raw>) -> Self {
        ModernBlockList {
            properties,
            blocks,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModernBlockData<'raw> {
    #[serde(borrow, skip_serializing_if = "LinkedHashMap::is_empty", rename = "properties")]
    kinds: LinkedHashMap<&'raw str, PropertyValue<'raw>, RandomState>,
    #[serde(rename = "base")]
    base_id: i32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "default")]
    default_id: Option<i32>,
}

impl<'raw> ModernBlockData<'raw> {
    pub fn new(kinds: LinkedHashMap<&'raw str, PropertyValue<'raw>, RandomState>, base_id: i32, default_id: Option<i32>) -> Self {
        ModernBlockData {
            kinds,
            base_id,
            default_id
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue<'raw> {
    Bool(&'raw str),
    Range([u8; 2]),
    #[serde(borrow)]
    Enum(&'raw str),
}

impl<'raw> PropertyValue<'raw> {
    pub fn bool() -> Self {
        Self::Bool("bool")
    }

    pub fn enum_name(value: &'raw str) -> Self {
        Self::Enum(value)
    }
    
    pub fn range(start: u8, end: u8) -> Self {
        Self::Range([start, end])
    }
}
