use std::ops::Deref;


use crate::services::accounts::AccountsService;

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
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    pub user_id: UserId,
    pub secret: String,
    pub expire: TimeStamp,
}

pub struct Account {
    service: AccountsService,
}

impl Account {
    pub fn new() -> Self {
        Account {
            service: AccountsService {},
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Account {
    type Target = AccountsService;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}
