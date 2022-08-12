use ahash::RandomState;
use hashlink::LinkedHashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModernPropertyRules<'raw> {
    #[serde(borrow, flatten)]
    rule_data: LinkedHashMap<&'raw str, Vec<&'raw str>, RandomState>,
}

// #[allow(unused_must_use)]
// impl<'raw> ModernPropertyRules<'raw> {
//     pub fn apply_enum_rules<'b>(&'b self, data: &mut RawBlockList<'raw>) {
//         let cache: LinkedHashMap<EnumProperty<'raw>, &'raw str, RandomState> = self.rule_data
//             .iter()
//             .map(|(n, values)| (EnumProperty::new(values), *n))
//             .collect();
//
//         let properties = data.properties_mut();
//         for (name, kind) in properties.iter_mut() {
//             if let PropertyKind::Enum(property) = kind {
//                 if let Some(key) = self.rule_data
//                     .iter()
//                     .find_map(|(n,v)| {
//                         if v == property.fields() {
//                             Some(n)
//                         } else {
//                             None
//                         }
//                     }) {
//                     std::mem::replace(name, key);
//                 }
//             }
//         }
//         properties.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));
//         properties.dedup_by(|(n1, _), (n2, _)| n1.eq(&n2));
//
//         for (_, block) in data.blocks_mut() {
//             if let Some(properties) = block.kinds_mut() {
//                 for ((_, name), kind) in properties.iter_mut() {
//                     if let PropertyKind::Enum(kind) = kind {
//                         if let Some(key) = cache.get(kind) {
//                             std::mem::replace(name, Some(key));
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

