use chrono::serde::ts_seconds;

use super::{user::UserId, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TokenId(pub String);

impl TokenId {
    pub fn new(id: String) -> Self {
        TokenId(id)
    }

    pub fn unique() -> Self {
        TokenId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename = "$id")]
    pub id: TokenId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    pub user_id: UserId,
    pub secret: String,
    pub expire: TimeStamp,
}
