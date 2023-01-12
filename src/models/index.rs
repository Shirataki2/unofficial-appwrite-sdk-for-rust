use crate::prelude::Order;

use super::{DataStatus, ListKey};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IndexType {
    Key,
    Fulltext,
    Unique,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Index {
    pub key: String,
    #[serde(rename = "type")]
    pub index_type: IndexType,
    pub status: DataStatus,
    pub attributes: Vec<String>,
    pub orders: Vec<Order>,
}

impl ListKey for Index {
    fn list_key() -> &'static str {
        "indexes"
    }
}
