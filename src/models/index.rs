use super::{attribute::Attribute, DataStatus, ListKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub key: String,
    #[serde(rename = "type")]
    pub index_type: String,
    pub status: DataStatus,
    pub attributes: Vec<Attribute>,
    // TODO: add model
    pub indexes: Vec<String>,
}

impl ListKey for Index {
    fn list_key() -> &'static str {
        "indexes"
    }
}
