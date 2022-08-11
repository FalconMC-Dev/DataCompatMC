pub(crate) mod de;
pub(crate) mod properties;

pub use properties::{PropertyKind, EnumProperty};

use ahash::RandomState;
use hashlink::LinkedHashMap;
use crate::blocks::intermediary::data::{ModernBlockData, ModernBlockList, TextOrRange};
use crate::util::identifier::Identifier;

type PropertyList<'raw> = Vec<(&'raw str, PropertyKind<'raw>)>;
type BlockList<'raw> = LinkedHashMap<Identifier<'raw>, RawBlockData<'raw>, RandomState>;

#[derive(Debug)]
pub struct RawBlockList<'raw> {
    properties: PropertyList<'raw>,
    blocks: BlockList<'raw>,
}

impl<'raw> From<RawBlockList<'raw>> for ModernBlockList<'raw> {
    fn from(mut list: RawBlockList<'raw>) -> Self {
        ModernBlockList::new(
            list.properties.into_iter().filter_map(|(name, kind)| {
                if let PropertyKind::Enum(kind) = kind {
                    Some((name, kind.fields_owned()))
                } else {
                    None
                }
            }).collect(),
            list.blocks.into_iter().map(|(id, block)| (id, block.into())).collect()
        )
    }
}

impl<'raw> RawBlockList<'raw> {
    pub fn new(properties: PropertyList<'raw>, blocks: BlockList<'raw>) -> Self {
        RawBlockList {
            properties,
            blocks,
        }
    }

    pub fn properties(&self) -> &[(&'raw str, PropertyKind<'raw>)] {
        self.properties.as_slice()
    }

    pub fn properties_mut(&mut self) -> &mut Vec<(&'raw str, PropertyKind<'raw>)> {
        &mut self.properties
    }
    
    pub fn blocks(&self) -> &BlockList<'raw> {
        &self.blocks
    }

    pub fn blocks_mut(&mut self) -> &mut BlockList<'raw> {
        &mut self.blocks
    }
}

#[derive(Debug)]
pub struct RawBlockData<'raw> {
    kinds: Option<Vec<((&'raw str, Option<&'raw str>), PropertyKind<'raw>)>>,
    base_id: i32,
    default_id: Option<i32>,
}

impl<'raw> RawBlockData<'raw> {
    pub fn new(kinds: Option<Vec<((&'raw str, Option<&'raw str>), PropertyKind<'raw>)>>, base_id: i32, default_id: i32) -> Self {
        RawBlockData {
            kinds,
            base_id,
            default_id: if base_id == default_id { None } else { Some(default_id) },
        }
    }

    pub fn kinds_mut(&mut self) -> Option<&mut Vec<((&'raw str, Option<&'raw str>), PropertyKind<'raw>)>> {
        self.kinds.as_mut()
    }
}

impl<'raw> From<RawBlockData<'raw>> for ModernBlockData<'raw> {
    fn from(data: RawBlockData<'raw>) -> Self {
        ModernBlockData::new(
            data.kinds.map(|mut data| data.drain(..).map(|((orig, name), kind)| {
                match kind {
                    PropertyKind::Bool => (orig, TextOrRange::text("bool")),
                    PropertyKind::Int(range) => (orig, TextOrRange::range(range.start as i32, (range.end - 1) as i32)),
                    PropertyKind::Enum(_) => (orig, TextOrRange::text(name.unwrap_or(orig)))
                }
            }).collect()),
            data.base_id,
            data.default_id,
        )
    }
}
