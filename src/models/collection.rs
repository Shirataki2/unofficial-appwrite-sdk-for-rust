use chrono::serde::ts_seconds;

use super::{database::DatabaseId, permission::Permission, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CollectionId(pub String);

impl CollectionId {
    pub fn new(id: String) -> Self {
        CollectionId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPermission {
    Document,
    Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "$id")]
    pub id: CollectionId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$read")]
    pub read_perms: Vec<Permission>,
    #[serde(rename = "$write")]
    pub write_perms: Vec<Permission>,
    pub database_id: DatabaseId,
    pub name: String,
    pub enabled: bool,
    pub permission: CollectionPermission,
}

impl ListKey for Collection {
    fn list_key() -> &'static str {
        "collections"
    }
}
