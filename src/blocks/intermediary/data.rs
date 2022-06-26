use ahash::RandomState;
use hashlink::LinkedHashMap;
use crate::blocks::raw::modern::PropertyKind;
use crate::util::identifier::Identifier;

type PropertyList<'raw> = Vec<(&'raw str, PropertyKind<'raw>)>;
type BlockList<'raw> = LinkedHashMap<Identifier<'raw>, ModernBlockData<'raw>, RandomState>;

#[derive(Debug)]
pub struct ModernBlockList<'raw> {
    pub properties: PropertyList<'raw>,
    pub blocks: BlockList<'raw>,
}

impl<'raw> ModernBlockList<'raw> {
    pub fn new(properties: PropertyList<'raw>, blocks: BlockList<'raw>) -> Self {
        ModernBlockList {
            properties,
            blocks,
        }
    }
}

#[derive(Debug)]
pub struct ModernBlockData<'raw> {
    kinds: Option<LinkedHashMap<&'raw str, PropertyKind<'raw>, RandomState>>,
    base_id: i32,
    default_id: i32,
}

impl<'raw> ModernBlockData<'raw> {
    pub fn new(kinds: Option<LinkedHashMap<&'raw str, PropertyKind<'raw>, RandomState>>, base_id: i32, default_id: i32) -> Self {
        ModernBlockData {
            kinds,
            base_id,
            default_id
        }
    }
}