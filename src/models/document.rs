use chrono::serde::ts_seconds;

use super::{collection::CollectionId, permission::Permission, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new(id: String) -> Self {
        DocumentId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "$id")]
    pub id: DocumentId,
    #[serde(rename = "$collection")]
    pub collection_id: CollectionId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$read")]
    pub read_perms: Vec<Permission>,
    #[serde(rename = "$write")]
    pub write_perms: Vec<Permission>,
}

impl ListKey for Document {
    fn list_key() -> &'static str {
        "documents"
    }
}
