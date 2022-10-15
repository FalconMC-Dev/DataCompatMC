use ahash::{AHashMap, AHashSet};
use serde::de::{DeserializeSeed, IgnoredAny, Visitor};
use serde::Deserialize;

use super::rules::ModernPropertyRules;
use crate::blocks::raw::property::{EnumProperty, PropertyKind};

/// A collection of possible property collisions in raw data.
///
/// There are two types of collisions:
/// - name collisions
/// - value collisions
///
/// Name collisions make it impossible to uniquely
/// identify a property name with its values. This is a requirement
/// for `DataCompatMC` and thus will abort the program.
///
/// Value collisions aren't critical for the compression
/// of raw data but it can make the compressed data ever
/// so slightly larger in size due to resulting duplicates
/// in the property list. This will not abort the program.
#[derive(Debug)]
pub struct CollisionList<'raw> {
    by_name: AHashMap<&'raw str, AHashSet<EnumProperty<'raw>>>,
    by_values: AHashMap<EnumProperty<'raw>, AHashSet<&'raw str>>,
}

impl<'raw> CollisionList<'raw> {
    pub fn should_exit(&self) -> bool {
        !self.by_name.is_empty()
    }

    /// Displays a summary of the different collisions found in the raw data.
    pub fn display(&self) {
        if !self.by_name.is_empty() {
            eprintln!("Name collisions:");
            for (name, values) in &self.by_name {
                eprintln!("========");
                for value in values {
                    eprintln!("{} -> {:?}", name, value.fields());
                }
            }
            eprintln!("========");
            eprintln!("Found {} name collisions! \u{274C}\n", self.by_name.len());
        }
        if !self.by_values.is_empty() {
            eprintln!("Value collisions:");
            for (values, names) in &self.by_values {
                eprintln!("========");
                for name in names {
                    eprintln!("{:?} -> {}", values.fields(), name);
                }
            }
            eprintln!("========");
            eprintln!("Found {} value collisions! \u{26A0}\u{FE0F}\n", self.by_values.len());
        }
    }
}

pub struct CollisionRuleProvider<'a, 'raw>(Option<&'a ModernPropertyRules<'raw>>);

impl<'a, 'raw> CollisionRuleProvider<'a, 'raw> {
    /// Constructs a new `CollisionRuleProvider` given a set of rules
    pub fn new(rules: Option<&'a ModernPropertyRules<'raw>>) -> Self {
        Self(rules)
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
            if let Some(rules) = self.0 {
                rules.transform(name, property)
            } else {
                (name, property)
            }
        })
    }
}

impl<'a, 'raw, 'de: 'raw> Visitor<'de> for CollisionRuleProvider<'a, 'raw> {
    type Value = CollisionList<'raw>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 1.13+ minecraft-generated block list")
    }

    /// Simply collect all the properties and keep the ones
    /// that share either name or values
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut by_name = AHashMap::<&'raw str, AHashSet<EnumProperty>>::new();
        let mut by_values = AHashMap::<EnumProperty<'raw>, AHashSet<&'raw str>>::new();

        while let Some((_, data)) = map.next_entry::<IgnoredAny, RawBlockData<'de>>()? {
            for (name, property) in self.transform(data.properties) {
                if let PropertyKind::Enum(property) = property {
                    if let Some(names) = by_values.get_mut(&property) {
                        names.insert(name);
                    } else {
                        by_values.insert(property.clone(), AHashSet::from([name]));
                    }
                    if let Some(values) = by_name.get_mut(&name) {
                        values.insert(property);
                    } else {
                        by_name.insert(name, AHashSet::from([property]));
                    }
                }
            }
        }

        by_name.retain(|_, set| set.len() > 1);
        by_values.retain(|_, set| set.len() > 1);

        Ok(CollisionList { by_name, by_values })
    }
}

impl<'a, 'raw, 'de: 'raw> DeserializeSeed<'de> for CollisionRuleProvider<'a, 'raw> {
    type Value = CollisionList<'raw>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

#[derive(Debug, Deserialize)]
struct RawBlockData<'raw> {
    #[serde(borrow, default)]
    properties: AHashMap<&'raw str, PropertyKind<'raw>>,
    #[serde(rename = "states")]
    _states: IgnoredAny,
}
