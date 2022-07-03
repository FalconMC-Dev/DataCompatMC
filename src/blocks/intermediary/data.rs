use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use crate::util::identifier::Identifier;

type PropertyList<'raw> = LinkedHashMap<&'raw str, Vec<&'raw str>>;
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

    pub fn properties(&self) -> &LinkedHashMap<&'raw str, Vec<&'raw str>> {
        &self.properties
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModernBlockData<'raw> {
    #[serde(borrow, skip_serializing_if = "Option::is_none", rename = "properties")]
    kinds: Option<LinkedHashMap<&'raw str, TextOrRange<'raw>, RandomState>>,
    #[serde(rename = "base")]
    base_id: i32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "default")]
    default_id: Option<i32>,
}

impl<'raw> ModernBlockData<'raw> {
    pub fn new(kinds: Option<LinkedHashMap<&'raw str, TextOrRange<'raw>, RandomState>>, base_id: i32, default_id: Option<i32>) -> Self {
        ModernBlockData {
            kinds,
            base_id,
            default_id
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextOrRange<'raw> {
    #[serde(borrow)]
    Text(&'raw str),
    Range([i32; 2]),
}

impl<'raw> TextOrRange<'raw> {
    pub fn text(text: &'raw str) -> Self {
        Self::Text(text)
    }
    
    pub fn range(start: i32, end: i32) -> Self {
        Self::Range([start, end])
    }
}