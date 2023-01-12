
use crate::{
    client::AppWriteClient,
    error::Error,
    prelude::SessionId,
    services::{users::*, SearchPayload},
};

use super::{log::Log, membership::Membership, session::Session, ListKey, ListResponse, TimeStamp};

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

#[rustfmt::skip]
impl UserId {
    pub async fn get(&self, client: &AppWriteClient) -> Result<User, Error> {
        UsersService::get_user(client, self).await
    }

    pub async fn get_prefs<T>(&self, client: &AppWriteClient) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        UsersService::get_prefs(client, self).await
    }

    pub async fn get_sessions(&self, client: &AppWriteClient) -> Result<ListResponse<Session>, Error> {
        UsersService::get_sessions(client, self).await
    }

    pub async fn get_memberships(&self, client: &AppWriteClient) -> Result<ListResponse<Membership>, Error> {
        UsersService::get_memberships(client, self).await
    }

    pub async fn get_logs(&self, client: &AppWriteClient, offset: Option<u64>, limit: Option<u64>) -> Result<ListResponse<Log>, Error> {
        UsersService::get_logs(client, self, offset, limit).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "$id")]
    pub id: UserId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub name: String,
    #[serde()]
    pub registration: TimeStamp,
    pub status: bool,
    #[serde()]
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
        user_id: UserId,
        email: Option<String>,
        phone: Option<String>,
        password: Option<String>,
        name: Option<String>,
    ) -> Result<Self, Error> {
        UsersService::create_user(client, user_id, email, phone, password, name).await
    }

    pub async fn create_with_md5(
        client: &AppWriteClient,
        user_id: UserId,
        email: String,
        password: String,
        name: Option<String>,
    ) -> Result<Self, Error> {
        UsersService::create_user_with_md5(client, user_id, email, password, name).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<UserId>,
    ) -> Result<ListResponse<Self>, Error> {
        UsersService::list_users(client, payload).await
    }

    pub async fn get_prefs(&self, client: &AppWriteClient) -> Result<serde_json::Value, Error> {
        UsersService::get_prefs(client, &self.id).await
    }

    pub async fn list_sessions(
        &self,
        client: &AppWriteClient,
    ) -> Result<ListResponse<Session>, Error> {
        UsersService::get_sessions(client, &self.id).await
    }

    pub async fn list_memberships(
        &self,
        client: &AppWriteClient,
    ) -> Result<ListResponse<Membership>, Error> {
        UsersService::get_memberships(client, &self.id).await
    }

    pub async fn list_logs(
        &self,
        client: &AppWriteClient,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<ListResponse<Log>, Error> {
        UsersService::get_logs(client, &self.id, offset, limit).await
    }

    pub async fn update_status(
        &self,
        client: &AppWriteClient,
        status: bool,
    ) -> Result<User, Error> {
        UsersService::update_user_status(client, &self.id, status).await
    }

    pub async fn update_email_verification(
        &self,
        client: &AppWriteClient,
        status: bool,
    ) -> Result<User, Error> {
        UsersService::update_email_verification(client, &self.id, status).await
    }

    pub async fn update_phone_verification(
        &self,
        client: &AppWriteClient,
        status: bool,
    ) -> Result<User, Error> {
        UsersService::update_phone_verification(client, &self.id, status).await
    }

    pub async fn update_name(&self, client: &AppWriteClient, name: String) -> Result<User, Error> {
        UsersService::update_name(client, &self.id, &name).await
    }

    pub async fn update_email(
        &self,
        client: &AppWriteClient,
        email: String,
    ) -> Result<User, Error> {
        UsersService::update_email(client, &self.id, &email).await
    }

    pub async fn update_phone(
        &self,
        client: &AppWriteClient,
        phone: String,
    ) -> Result<User, Error> {
        UsersService::update_phone(client, &self.id, &phone).await
    }

    pub async fn update_password(
        &self,
        client: &AppWriteClient,
        password: String,
    ) -> Result<User, Error> {
        UsersService::update_password(client, &self.id, &password).await
    }

    pub async fn update_prefs(
        &self,
        client: &AppWriteClient,
        prefs: serde_json::Value,
    ) -> Result<User, Error> {
        UsersService::update_prefs(client, &self.id, &prefs).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), Error> {
        UsersService::delete_user(client, &self.id).await
    }

    pub async fn delete_session(
        &self,
        client: &AppWriteClient,
        session_id: &SessionId,
    ) -> Result<(), Error> {
        UsersService::delete_user_session(client, &self.id, session_id).await
    }

    pub async fn delete_sessions(&self, client: &AppWriteClient) -> Result<(), Error> {
        UsersService::delete_user_sessions(client, &self.id).await
    }
}
