use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::de::{DeserializeSeed, Visitor};

use super::property::PropertyKind;
use super::RawBlockData;
use crate::blocks::intermediary::data::{ModernBlockData, ModernBlockList, PropertyValue};
use crate::blocks::intermediary::rules::ModernPropertyRules;
use crate::blocks::intermediary::MetaData;
use crate::util::identifier::Identifier;

pub struct CompactRuleProvider<'a, 'raw> {
    pub rules: Option<&'a ModernPropertyRules<'raw>>,
    pub metadata: Option<MetaData<'raw>>,
}

impl<'a, 'raw> CompactRuleProvider<'a, 'raw> {
    pub fn new(rules: Option<&'a ModernPropertyRules<'raw>>, metadata: Option<MetaData<'raw>>) -> Self {
        Self { rules, metadata }
    }

    /// This transformation does two checks:
    /// - First it makes sure the property name is not `"type"`, this will get
    ///   transformed into `"kind"`
    /// - Secondly it uses the rules, if present, to replace the property name
    ///   if there's a match based on the property values
    pub fn transform<'b, I>(&'b self, properties: I) -> impl Iterator<Item = (&'raw str, PropertyKind<'raw>)> + 'b
    where
        I: IntoIterator<Item = (&'raw str, PropertyKind<'raw>)> + 'b,
    {
        properties.into_iter().map(|(name, property)| {
            let name = if name == "type" {
                "kind"
            } else {
                name
            };
            if let Some(rules) = self.rules {
                rules.transform(name, property)
            } else {
                (name, property)
            }
        })
    }
}

impl<'a, 'raw, 'de: 'raw> Visitor<'de> for CompactRuleProvider<'a, 'raw> {
    type Value = ModernBlockList<'raw>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 1.13+ minecraft-generated block list")
    }

    /// This implementation verifies the data
    /// and makes sure it follows the expected pattern.
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        // Overall lists that will be passed to ModernBlockList
        let mut properties = LinkedHashMap::with_hasher(RandomState::default());
        let mut blocks = LinkedHashMap::with_capacity_and_hasher(map.size_hint().unwrap_or(0), RandomState::default());

        while let Some((identifier, block)) = map.next_entry::<Identifier<'raw>, RawBlockData<'raw>>()? {
            let property_count = block.property_count();
            let state_count = block.state_count();

            // make sure there is the correct amount of blockstates
            if block.states.len() != state_count {
                return Err(serde::de::Error::invalid_length(block.states.len(), &format!("{} entries", state_count).as_str()));
            }
            let base_id = block.states[0].id;

            let mut default_id = None;
            for state in &block.states {
                // Make sure the blockstate has the correct amount of property values
                if state.properties.len() != property_count {
                    return Err(serde::de::Error::invalid_length(state.properties.len(), &format!("{} entries", property_count).as_str()));
                }

                // Check the network values for consistency,
                // this is the most important trick for compacting.
                //
                // Step 1: calculate the offset from the base id based on property values
                let mut factor = 1;
                let mut prev_factor = 1;
                let mut offset = 0;
                for (&name, &value) in state.properties.iter().rev() {
                    let values = block
                        .properties
                        .iter()
                        .find_map(|(&a, b)| {
                            if a == name {
                                Some(b)
                            } else {
                                None
                            }
                        })
                        .ok_or_else(|| serde::de::Error::custom(format!("found a non-matching a property \"{}\" for \"{}\"", name, identifier)))?;
                    match values.iter().position(|&x| x == value) {
                        Some(index) => {
                            factor *= prev_factor;
                            prev_factor = values.len();
                            offset += factor * index;
                        },
                        None => {
                            return Err(serde::de::Error::custom(format!("invalid property value \"{}\" found while deserializing \"{}\"", value, identifier)));
                        },
                    }
                }

                // Step 2: make sure that the base id plus offset equals the given blockstate id
                let id = base_id + offset as i32;
                if id != state.id {
                    return Err(serde::de::Error::custom(format!("incorrect id match for \"{}\", found {} while expecting {}", identifier, state.id, id)));
                }
                // try to find a default blockstate other than base id
                if state.default {
                    default_id = Some(id);
                }
            }
            // only if base id is different from default_id should default_id be remembered
            // (see below when adding new block)
            let default_id = default_id.and_then(|id| {
                if id == base_id {
                    None
                } else {
                    Some(id)
                }
            });

            // Extend the list of properties with the properties of this block
            properties.extend(self.transform(block.properties()).filter_map(|(name, property)| match property {
                PropertyKind::Enum(property) => Some((name, property)),
                _ => None,
            }));

            // Collect the property types to add to the compacted form of this block (see
            // below)
            let properties = block
                .properties()
                .map(|(name, kind)| {
                    if name == "type" {
                        ("kind", kind)
                    } else {
                        (name, kind)
                    }
                })
                .map(|(name, kind)| {
                    (
                        name,
                        match kind {
                            PropertyKind::Bool => PropertyValue::bool(),
                            PropertyKind::Int([start, end]) => PropertyValue::range(start, end),
                            PropertyKind::Enum(_) => {
                                if let Some(rules) = self.rules {
                                    PropertyValue::enum_name(rules.transform(name, kind).0)
                                } else {
                                    PropertyValue::enum_name(name)
                                }
                            },
                        },
                    )
                })
                .collect();

            // Add a new block using the properties, base id and default id calculated above
            blocks.insert(identifier, ModernBlockData::new(properties, base_id, default_id));
        }

        Ok(ModernBlockList::new(self.metadata, properties, blocks))
    }
}

impl<'a, 'raw, 'de: 'raw> DeserializeSeed<'de> for CompactRuleProvider<'a, 'raw> {
    type Value = ModernBlockList<'raw>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}
