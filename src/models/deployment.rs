use serde_enum_str::*;

use super::{Id, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DeploymentId(pub String);

impl DeploymentId {
    pub fn new(id: String) -> Self {
        DeploymentId(id)
    }

    pub fn unique() -> Self {
        DeploymentId("unique()".to_string())
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BuildId(pub String);

impl BuildId {
    pub fn new(id: String) -> Self {
        BuildId(id)
    }

    pub fn unique() -> Self {
        BuildId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize_enum_str, Deserialize_enum_str)]
#[serde(rename_all = "camelCase")]
pub enum DeploymentStatus {
    Enabled,
    Disabled,
    #[serde(other)]
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    #[serde(rename = "$id")]
    pub id: DeploymentId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub resource_id: String, // TODO: Change string to ResourceId
    pub resource_type: String,
    pub entrypoint: String,
    pub size: u64,
    pub build_id: String, // TODO: Change string to BuildId
    pub activate: bool,
    pub status: DeploymentStatus,
    pub build_stdout: String,
    pub build_stderr: String,
}

impl ListKey for Deployment {
    fn list_key() -> &'static str {
        "deployments"
    }
}

impl Id for Deployment {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}
