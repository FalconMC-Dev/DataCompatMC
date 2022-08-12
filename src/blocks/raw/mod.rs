use ahash::{AHashMap, RandomState};
use hashlink::LinkedHashMap;
use serde::Deserialize;

use crate::util::identifier::Identifier;

use property::EnumProperty;

use self::property::PropertyKind;

use super::intermediary::data::TextOrRange;

pub mod de;
// pub mod modern;
pub mod property;

#[derive(Debug, Deserialize)]
pub struct RawBlockData<'raw> {
    #[serde(borrow, default)]
    properties: LinkedHashMap<&'raw str, PropertyKind<'raw>, RandomState>,
    #[serde(borrow)]
    states: Vec<RawBlockState<'raw>>,
}

#[derive(Debug, Deserialize)]
pub struct RawBlockState<'raw> {
    #[serde(borrow, default)]
    properties: LinkedHashMap<&'raw str, &'raw str>,
    id: i32,
    #[serde(default)]
    default: bool,
}

#[derive(Debug)]
pub struct RawBlockList<'raw> {
    properties: AHashMap<&'raw str, EnumProperty<'raw>>,
    blocks: LinkedHashMap<Identifier<'raw>, RawBlock<'raw>, RandomState>,
}

#[derive(Debug)]
pub struct RawBlock<'raw> {
    properties: LinkedHashMap<&'raw str, TextOrRange<'raw>, RandomState>,
    base_id: i32,
    default_id: Option<i32>,
}
