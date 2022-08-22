use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        log::Log,
        membership::Membership,
        session::{Session, SessionId},
        user::{User, UserId},
        ListResponse,
    },
};

use super::SearchPayload;
pub struct UsersService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPayload {
    pub user_id: UserId,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, SerializeParams, Default)]
#[params(rename_all = "camelCase")]
pub struct ListLogsPayload {
    limit: Option<u64>,
    offset: Option<u64>,
}

impl UsersService {
    pub async fn create_user(
        client: &AppWriteClient,
        user: CreateUserPayload,
    ) -> Result<User, Error> {
        let url = "/users";
        let response = client
            .call(
                Method::POST,
                url,
                RequestData::Json(serde_json::to_value(user)?),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn get_user(client: &AppWriteClient, user_id: &UserId) -> Result<User, Error> {
        let url = format!("/users/{}", user_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;

        Ok(check_response!(User: response))
    }

    pub async fn list_users(
        client: &AppWriteClient,
        payload: SearchPayload<UserId>,
    ) -> Result<ListResponse<User>, Error> {
        let url = "/users";
        let response = client
            .call(
                Method::GET,
                url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<User>: response))
    }

    pub async fn get_prefs<T>(client: &AppWriteClient, user_id: &UserId) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("/users/{}/prefs", user_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(T: response))
    }

    pub async fn get_sessions(
        client: &AppWriteClient,
        user_id: &UserId,
    ) -> Result<ListResponse<Session>, Error> {
        let url = format!("/users/{}/sessions", user_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Session>: response))
    }

    pub async fn get_memberships(
        client: &AppWriteClient,
        user_id: &UserId,
    ) -> Result<ListResponse<Membership>, Error> {
        let url = format!("/users/{}/memberships", user_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Membership>: response))
    }

    pub async fn get_logs(
        client: &AppWriteClient,
        user_id: &UserId,
        payload: ListLogsPayload,
    ) -> Result<ListResponse<Log>, Error> {
        let url = format!("/users/{}/logs", user_id);
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Log>: response))
    }

    pub async fn update_user_status(
        client: &AppWriteClient,
        user_id: &UserId,
        activate: bool,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/status", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "status": activate })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_email_varification(
        client: &AppWriteClient,
        user_id: &UserId,
        activate: bool,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/varification", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "emailVerification": activate })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_phone_varification(
        client: &AppWriteClient,
        user_id: &UserId,
        activate: bool,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/varification/phone", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "phoneVerification": activate })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_name(
        client: &AppWriteClient,
        user_id: &UserId,
        name: &str,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/name", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "name": name })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_password(
        client: &AppWriteClient,
        user_id: &UserId,
        password: &str,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/password", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "password": password })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_email(
        client: &AppWriteClient,
        user_id: &UserId,
        email: &str,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/email", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "email": email })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_phone(
        client: &AppWriteClient,
        user_id: &UserId,
        phone: &str,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/phone", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "phone": phone })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_prefs<T>(
        client: &AppWriteClient,
        user_id: &UserId,
        payload: T,
    ) -> Result<User, Error>
    where
        T: serde::Serialize,
    {
        let url = format!("/users/{}/prefs", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!(payload)),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn delete_user_session(
        client: &AppWriteClient,
        user_id: &UserId,
        session_id: &SessionId,
    ) -> Result<(), Error> {
        let url = format!("/users/{}/sessions/{}", user_id, session_id);
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        check_response!(response);
        Ok(())
    }

    pub async fn delete_user_sessions(
        client: &AppWriteClient,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let url = format!("/users/{}/sessions", user_id);
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        check_response!(response);
        Ok(())
    }

    pub async fn delete_user(client: &AppWriteClient, user_id: &UserId) -> Result<(), Error> {
        let url = format!("/users/{}", user_id);
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        check_response!(response);
        Ok(())
    }
}
