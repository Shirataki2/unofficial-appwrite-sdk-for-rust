use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    insert_if_some,
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

impl UsersService {
    pub async fn create_user(
        client: &AppWriteClient,
        user_id: UserId,
        email: Option<String>,
        phone: Option<String>,
        password: Option<String>,
        name: Option<String>,
    ) -> Result<User, Error> {
        let url = "/users";
        let user = serde_json::json!({
            "userId": user_id,
            "email": email,
            "phone": phone,
            "password": password,
            "name": name,
        });
        let response = client
            .call(Method::POST, url, RequestData::Json(user))
            .await?;
        Ok(check_response!(User: response))
    }

    // TODO: ほかの暗号メソッドでのユーザー作成も実装する
    pub async fn create_user_with_md5(
        client: &AppWriteClient,
        user_id: UserId,
        email: String,
        password: String,
        name: Option<String>,
    ) -> Result<User, Error> {
        let url = "/users/md5";
        let user = serde_json::json!({
            "userId": user_id,
            "email": email,
            "password": password,
            "name": name,
        });
        let response = client
            .call(Method::POST, url, RequestData::Json(user))
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
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<ListResponse<Log>, Error> {
        let url = format!("/users/{}/logs", user_id);
        let mut payload = vec![];
        insert_if_some!(payload => "offset", offset; "limit", limit);
        let response = client
            .call(Method::GET, &url, RequestData::Params(payload))
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

    pub async fn update_email_verification(
        client: &AppWriteClient,
        user_id: &UserId,
        activate: bool,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/verification", user_id);
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::json!({ "emailVerification": activate })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_phone_verification(
        client: &AppWriteClient,
        user_id: &UserId,
        activate: bool,
    ) -> Result<User, Error> {
        let url = format!("/users/{}/verification/phone", user_id);
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
