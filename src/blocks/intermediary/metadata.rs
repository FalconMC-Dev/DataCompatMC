use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData<'raw> {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'raw str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'raw str>,
}

impl<'raw> MetaData<'raw> {
    pub fn new(id: i32, name: Option<&'raw str>, note: Option<&'raw str>) -> Self { Self { id, name, note } }
}
