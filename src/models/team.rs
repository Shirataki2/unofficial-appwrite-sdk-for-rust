use chrono::serde::ts_seconds;

use super::{ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TeamId(pub String);

impl TeamId {
    pub fn new(id: String) -> Self {
        TeamId(id)
    }

    pub fn unique() -> Self {
        TeamId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "$id")]
    pub id: TeamId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    pub name: String,
    pub total: u64,
}

impl ListKey for Team {
    fn list_key() -> &'static str {
        "teams"
    }
}
