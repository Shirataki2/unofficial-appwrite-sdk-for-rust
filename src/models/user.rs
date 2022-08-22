use chrono::serde::ts_seconds;

use crate::{
    client::AppWriteClient,
    error::Error,
    services::{users::*, SearchPayload},
};

use super::{ListKey, ListResponse, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UserId(pub String);

impl UserId {
    pub fn new(id: String) -> Self {
        UserId(id)
    }

    pub fn unique() -> Self {
        UserId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "$id")]
    pub id: UserId,
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    pub name: String,
    #[serde(with = "ts_seconds")]
    pub registration: TimeStamp,
    pub status: bool,
    #[serde(with = "ts_seconds")]
    pub password_update: TimeStamp,
    pub email: String,
    pub phone: String,
    pub email_verification: bool,
    pub phone_verification: bool,
    // TODO: add preferences model
    pub prefs: serde_json::Value,
}

impl ListKey for User {
    fn list_key() -> &'static str {
        "users"
    }
}

impl User {
    pub async fn get(client: &AppWriteClient, user_id: &UserId) -> Result<Self, Error> {
        UsersService::get_user(client, user_id).await
    }

    pub async fn create(
        client: &AppWriteClient,
        payload: CreateUserPayload,
    ) -> Result<Self, Error> {
        UsersService::create_user(client, payload).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<UserId>,
    ) -> Result<ListResponse<Self>, Error> {
        UsersService::list_users(client, payload).await
    }
}
