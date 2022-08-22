use std::collections::HashMap;

use chrono::serde::ts_seconds;

use super::{permission::Permission, HasId, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FunctionId(pub String);

impl FunctionId {
    pub fn new(id: String) -> Self {
        FunctionId(id)
    }

    pub fn unique() -> Self {
        FunctionId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "$id")]
    pub id: FunctionId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    pub execute: Vec<Permission>,
    pub name: String,
    pub status: FunctionStatus,
    pub runtime: String,
    // TODO: Change string to DeploymentId
    pub deployment: String,
    pub vars: HashMap<String, String>,
    pub events: Vec<String>,
    pub schedule: String,
    pub schedule_next: TimeStamp,
    pub schedule_previous: TimeStamp,
    pub timeout: u64,
}

impl ListKey for Function {
    fn list_key() -> &'static str {
        "teams"
    }
}

impl HasId for Function {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}
