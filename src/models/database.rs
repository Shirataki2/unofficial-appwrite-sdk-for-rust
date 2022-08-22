use chrono::serde::ts_seconds;

use super::{ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DatabaseId(pub String);

impl DatabaseId {
    pub fn new(id: String) -> Self {
        DatabaseId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    #[serde(rename = "$id")]
    pub id: DatabaseId,
    pub name: String,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
}

impl ListKey for Database {
    fn list_key() -> &'static str {
        "databases"
    }
}
