use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData<'raw> {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'raw str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<&'raw str>,
}

impl<'raw> MetaData<'raw> {
    pub fn new(id: i32, name: Option<&'raw str>, note: Option<&'raw str>) -> Self {
        Self {
            id,
            name,
            note,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> Cow<'raw, str> {
        self.name
            .map(|name| name.into())
            .unwrap_or(Cow::Owned(self.id.to_string()))
    }

    pub fn note(&self) -> Option<&'raw str> {
        self.note
    }
}
