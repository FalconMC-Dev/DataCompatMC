use crate::blocks::intermediary::data::ModernBlockData;

#[derive(Debug)]
pub enum PropertyComparison {
    /// The base is equal to the target property -> compatible
    Equal,
    /// The base has less elements than the target property -> compatible
    MissingEnd,
    /// The base has more elements than the target property -> incompatible
    LongerEnd,
    /// The order of the base elements is different from the target property -> compatible
    DifferentOrdering,
    /// The values (and maybe ordering) of the base elements is different from the target property -> incompatible
    DifferentValues,
}

impl PropertyComparison {
    pub fn compare<'raw>(base: &Vec<&'raw str>, target: &Vec<&'raw str>) -> Self {
        if base == target {
            Self::Equal
        } else {
            let mut last_index = 0;
            for (i, value) in base.iter().enumerate() {
                if matches!(target.get(i), Some(x) if x == value) {
                    if last_index != i {
                        break;
                    }
                    last_index += 1;
                }
            }
            if last_index == base.len() || last_index == target.len() {
                return if target.len() > base.len() {
                    Self::MissingEnd
                } else {
                    Self::LongerEnd
                }
            }
            for value in base {
                if !target.contains(value) {
                    return Self::DifferentValues
                }
            }
            Self::DifferentOrdering
        }
    }
}

#[derive(Debug)]
pub enum BlockComparison {
    /// The base properties equal to the target -> compatible
    Equal,
    /// The base has one or more different types from the target -> incompatible
    DifferentType,
    /// The base has a different enum type from the target -> maybe compatible
    DifferentEnum,
    /// The base has a different range from the target -> maybe compatible
    DifferentRange,
    /// The base has more properties than the target -> compatible
    MoreProperties,
    /// The base has less properties than the target -> incompatible
    LessProperties,
    /// If all else fails, this usually means the properties would be in a different order,
    /// sometimes compatible, sometimes incompatible
    DifferentOrder,
}

impl BlockComparison {
    pub fn compare<'raw>(base: &ModernBlockData<'raw>, target: &ModernBlockData<'raw>) -> Self {
        if base.properties() == target.properties() {
            Self::Equal
        } else {
            if let Some(properties) = base.properties() {
                let target = if let Some(target) = target.properties() {
                    target
                } else {
                    return Self::MoreProperties;
                };
                for (name, kind) in properties {
                    if let Some(target) = target.get(name) {
                        if kind != target {
                            return if kind.is_enum() && target.is_enum() {
                                Self::DifferentEnum
                            } else if kind.is_range() && target.is_range() {
                                Self::DifferentRange
                            } else {
                                Self::DifferentType
                            }
                        }
                    }
                }
                if properties.len() < target.len() {
                    Self::LessProperties
                } else if properties.len() > target.len() {
                    Self::MoreProperties
                } else {
                    Self::DifferentOrder
                }
            } else {
                Self::LessProperties
            }
        }
    }
}
