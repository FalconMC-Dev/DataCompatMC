use ahash::{RandomState, AHashMap};
use hashlink::LinkedHashMap;
use serde::Deserialize;
use serde::de::{Visitor, DeserializeSeed};

use crate::blocks::intermediary::rules::ModernPropertyRules;

use super::RawBlockList;
use super::property::{PropertyKind, EnumProperty};

// pub struct RuleProvider<'a, 'raw>(&'a ModernPropertyRules<'raw>);
//
// impl<'a, 'raw, 'de: 'raw> Visitor<'de> for RuleProvider<'a, 'raw> {
//     type Value = RawBlockList<'raw>;
//
//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("a 1.13+ minecraft-generated block list")
//     }
//
//     fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
//         where
//             A: serde::de::MapAccess<'de>,
//     {
//         let properties = AHashMap::new();
//         let blocks = LinkedHashMap::with_capacity(map.size_hint().unwrap_or(0));
//     }
// }
//
// impl<'a, 'raw, 'de: 'raw> DeserializeSeed<'de> for RuleProvider<'a, 'raw> {
//     type Value = RawBlockList<'raw>;
//
//     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//     }
// }
//
