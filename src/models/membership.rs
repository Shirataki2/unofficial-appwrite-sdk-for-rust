use chrono::serde::ts_seconds;

use super::{user::UserId, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MembershipId(pub String);

impl MembershipId {
    pub fn new(id: String) -> Self {
        MembershipId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    #[serde(rename = "$id")]
    pub id: MembershipId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    pub user_id: UserId,
    pub user_name: String,
    pub user_email: String,
    // TODO: implement team model
    pub team_id: String,
    pub team_name: String,
    #[serde(with = "ts_seconds")]
    pub invited: TimeStamp,
    #[serde(with = "ts_seconds")]
    pub joined: TimeStamp,
    pub confirm: bool,
    pub roles: Vec<String>,
}

impl ListKey for Membership {
    fn list_key() -> &'static str {
        "memberships"
    }
}
