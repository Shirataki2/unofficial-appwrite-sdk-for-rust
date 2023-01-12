use std::net::IpAddr;


use super::{user::UserId, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new(id: String) -> Self {
        SessionId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(rename = "$id")]
    pub id: SessionId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    pub user_id: UserId,
    #[serde()]
    pub expire: TimeStamp,
    pub provider: String,
    pub provider_uid: String,
    pub provider_access_token: String,
    pub provider_access_token_expiry: TimeStamp,
    pub provider_refresh_token: String,
    pub ip: IpAddr,
    pub os_code: String,
    pub os_name: String,
    pub os_version: String,
    pub client_type: String,
    pub client_code: String,
    pub client_name: String,
    pub client_version: String,
    pub client_engine: String,
    pub client_engine_version: String,
    pub device_name: String,
    pub device_brand: String,
    pub device_model: String,
    pub cuntry_code: String,
    pub cuntry_name: String,
    pub current: bool,
}

impl ListKey for Session {
    fn list_key() -> &'static str {
        "sessions"
    }
}
