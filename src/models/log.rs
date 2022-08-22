use std::net::IpAddr;

use chrono::serde::ts_seconds;

use super::{user::UserId, ListKey, TimeStamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub event: String,
    pub user_id: UserId,
    pub user_name: String,
    pub user_email: String,
    pub mode: String,
    pub ip: IpAddr,
    #[serde(with = "ts_seconds")]
    pub time: TimeStamp,
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

impl ListKey for Log {
    fn list_key() -> &'static str {
        "logs"
    }
}
