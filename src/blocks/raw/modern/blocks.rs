use std::fmt::Formatter;
use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::{de, Deserialize, Deserializer};
use serde::de::{MapAccess, Visitor};
use super::{RawBlockData, RawBlockList};
use crate::blocks::raw::modern::PropertyKind;
use crate::util::identifier::Identifier;

#[derive(Deserialize)]
struct RawData<'raw> {
    #[serde(borrow)]
    properties: Option<LinkedHashMap<&'raw str, Vec<&'raw str>, RandomState>>,
    #[serde(borrow)]
    states: Vec<RawBlockState<'raw>>,
}

#[derive(Deserialize)]
struct RawBlockState<'raw> {
    #[serde(borrow)]
    properties: Option<LinkedHashMap<&'raw str, &'raw str>>,
    id: i32,
    default: Option<bool>,
}

impl<'raw, 'de: 'raw> Deserialize<'de> for RawBlockList<'raw> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
    {
        struct BlockListVisitor;
        impl<'de> Visitor<'de> for BlockListVisitor {
            type Value = RawBlockList<'de>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a 1.13+ generated block list from minecraft")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut properties = Vec::new();
                let mut blocks = LinkedHashMap::with_capacity(map.size_hint().unwrap_or(0));

                while let Some((identifier, raw_block)) = map.next_entry::<Identifier<'de>, RawData<'de>>()? {
                    let mut local_properties = None;
                    let base_id = raw_block.states.get(0).ok_or(de::Error::invalid_length(0, &"a non-empty blockstate list"))?.id;
                    let default = if let Some(props) = &raw_block.properties {
                        let mut local = Vec::with_capacity(props.len());
                        for (&name, values) in props {
                            let name = if name == "type" { "kind" } else { name };
                            let kind = PropertyKind::from(values.as_slice());
                            if !properties.iter().any(|(n, k)| *n == name && *k == kind) {
                                properties.push((name, kind.clone()));
                            }
                            local.push(((name, None), kind));
                        }
                        local_properties = Some(local);

                        let mut default = None;
                        for state in &raw_block.states {
                            let mut factor = 1;
                            let mut prev_factor = 1;
                            let mut offset = 0;
                            for (&name, &value) in state.properties.as_ref().ok_or(de::Error::missing_field("corresponding properties"))?.iter().rev() {
                                let values = props.iter().find_map(|(&a, b)| if a == name { Some(b) } else { None })
                                    .ok_or(de::Error::custom(format!("found a non-matching property \"{}\" for \"{}\"", name, identifier)))?;
                                match values.iter().position(|&x| x == value) {
                                    Some(index) => {
                                        factor *= prev_factor;
                                        prev_factor = values.len();
                                        offset += factor * index;
                                    }
                                    None => {
                                        return Err(de::Error::custom(format!("invalid property value \"{}\" found while deserializing \"{}\"", value, identifier)))
                                    }
                                }
                            }
                            let id = base_id + offset as i32;
                            if id != state.id {
                                return Err(de::Error::custom(format!("incorrect id match for \"{}\", found {} while expecting {}", identifier, state.id, id)));
                            }
                            if let Some(true) = state.default {
                                default = Some(id);
                            }
                        }
                        default.ok_or(de::Error::custom(format!("could not find default id for \"{}\"", identifier)))?
                    } else {
                        base_id
                    };
                    blocks.insert(identifier, RawBlockData::new(local_properties, base_id, default));
                }

                Ok(RawBlockList::new(properties, blocks))
            }
        }
        deserializer.deserialize_map(BlockListVisitor)
    }
}
