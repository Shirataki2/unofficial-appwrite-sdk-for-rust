use chrono::serde::ts_seconds;

use super::{function::FunctionId, permission::Permission, HasId, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExecutionId(pub String);

impl ExecutionId {
    pub fn new(id: String) -> Self {
        ExecutionId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExecutionStatus {
    Waiting,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExecutionTrigger {
    Http,
    Schedule,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    #[serde(rename = "$id")]
    pub id: ExecutionId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$read")]
    pub read_perms: Vec<Permission>,
    pub function_id: FunctionId,
    pub trigger: ExecutionTrigger,
    pub status: ExecutionStatus,
    pub status_code: u16,
    pub response: String,
    pub stderr: String,
    pub time: f64,
}

impl ListKey for Execution {
    fn list_key() -> &'static str {
        "deployments"
    }
}

impl HasId for Execution {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}
