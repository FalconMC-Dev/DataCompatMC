#[derive(Debug)]
pub enum PropertyComparison {
    /// The base is equal to the target property -> 100% compatible
    Equal,
    /// The base has less elements than the target property -> 100% compatible
    MissingEnd,
    /// The base has more elements than the target property -> not compatible
    LongerEnd,
    /// The order of the base elements is different from the target property -> 100% compatible
    DifferentOrdering,
    /// The values (and maybe ordering) of the base elements is different from the target property -> not compatible
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
            if last_index == base.len() {
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
