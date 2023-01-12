use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    insert_if_some,
    models::{
        account::Token,
        log::Log,
        session::{Session, SessionId},
        user::{User, UserId},
        ListResponse,
    },
};
pub struct AccountsService;

impl AccountsService {
    pub async fn get_account(client: &AppWriteClient) -> Result<User, Error> {
        let url = "/account";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(User: response))
    }

    pub async fn get_preference<Pref>(client: &AppWriteClient) -> Result<Pref, Error>
    where
        Pref: serde::de::DeserializeOwned,
    {
        let url = "/account/prefs";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Pref: response))
    }

    pub async fn get_sessions(client: &AppWriteClient) -> Result<ListResponse<Session>, Error> {
        let url = "/account/sessions";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Session>: response))
    }

    pub async fn get_logs(
        client: &AppWriteClient,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ListResponse<Log>, Error> {
        let url = "/account/logs";
        let mut params = vec![];
        insert_if_some!(
            params =>
            "limit", limit;
            "offset", offset
        );
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Log>: response))
    }

    pub async fn get_session(
        client: &AppWriteClient,
        session_id: &SessionId,
    ) -> Result<Session, Error> {
        let url = format!("/account/sessions/{session_id}", session_id = session_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Session: response))
    }

    pub async fn update_account_name(client: &AppWriteClient, name: &str) -> Result<User, Error> {
        let url = "/account/name";
        let response = client
            .call(
                Method::PATCH,
                url,
                RequestData::Json(serde_json::json!({ "name": name })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_password(
        client: &AppWriteClient,
        password: &str,
        old_password: Option<&str>,
    ) -> Result<User, Error> {
        let url = "/account/password";
        let response = client
            .call(
                Method::PATCH,
                url,
                RequestData::Json(serde_json::json!({
                    "password": password,
                    "oldPassword": old_password
                })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_user_email(
        client: &AppWriteClient,
        email: &str,
        password: &str,
    ) -> Result<User, Error> {
        let url = "/account/email";
        let response = client
            .call(
                Method::PATCH,
                url,
                RequestData::Json(serde_json::json!({
                    "email": email,
                    "password": password
                })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_phone(
        client: &AppWriteClient,
        phone_number: &str,
        password: &str,
    ) -> Result<User, Error> {
        let url = "/account/phone";
        let response = client
            .call(
                Method::PATCH,
                url,
                RequestData::Json(serde_json::json!({
                    "number": phone_number,
                    "password": password
                })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_preference<Pref>(
        client: &AppWriteClient,
        preference: &Pref,
    ) -> Result<User, Error>
    where
        Pref: serde::Serialize,
    {
        let url = "/account/prefs";
        let response = client
            .call(
                Method::PATCH,
                url,
                RequestData::Json(serde_json::json!({ "prefs": preference })),
            )
            .await?;
        Ok(check_response!(User: response))
    }

    pub async fn update_status(client: &AppWriteClient) -> Result<User, Error> {
        let url = "/account/status";
        let response = client.call(Method::PATCH, url, RequestData::None).await?;
        Ok(check_response!(User: response))
    }

    pub async fn delete_session(
        client: &AppWriteClient,
        session_id: &SessionId,
    ) -> Result<(), Error> {
        let url = format!("/account/sessions/{session_id}", session_id = session_id);
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn update_session(
        client: &AppWriteClient,
        session_id: &SessionId,
    ) -> Result<Session, Error> {
        let url = format!("/account/sessions/{session_id}", session_id = session_id);
        let response = client.call(Method::PATCH, &url, RequestData::None).await?;
        Ok(check_response!(Session: response))
    }

    pub async fn delete_sessions(client: &AppWriteClient) -> Result<(), Error> {
        let url = "/account/sessions";
        let response = client.call(Method::DELETE, url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn password_recovery(
        client: &AppWriteClient,
        email: &str,
        url: &str,
    ) -> Result<Token, Error> {
        let api_url = "/account/recovery";
        let response = client
            .call(
                Method::POST,
                api_url,
                RequestData::Json(serde_json::json!({
                    "email": email,
                    "url": url
                })),
            )
            .await?;
        Ok(check_response!(Token: response))
    }

    pub async fn confirm_password_recovery(
        client: &AppWriteClient,
        user_id: &UserId,
        secret: &str,
        password: &str,
        password_again: &str,
    ) -> Result<Token, Error> {
        let api_url = "/account/recovery";
        let response = client
            .call(
                Method::PUT,
                api_url,
                RequestData::Json(serde_json::json!({
                    "userId": user_id,
                    "secret": secret,
                    "password": password,
                    "passwordAgain": password_again
                })),
            )
            .await?;
        Ok(check_response!(Token: response))
    }

    pub async fn create_email_verification(
        client: &AppWriteClient,
        url: &str,
    ) -> Result<Token, Error> {
        let api_url = "/account/verification";
        let response = client
            .call(
                Method::POST,
                api_url,
                RequestData::Json(serde_json::json!({ "url": url })),
            )
            .await?;
        Ok(check_response!(Token: response))
    }

    pub async fn confirm_email_verification(
        client: &AppWriteClient,
        user_id: &UserId,
        secret: &str,
        password: &str,
        password_again: &str,
    ) -> Result<Token, Error> {
        let api_url = "/account/verification";
        let response = client
            .call(
                Method::PUT,
                api_url,
                RequestData::Json(serde_json::json!({
                    "userId": user_id,
                    "secret": secret,
                    "password": password,
                    "passwordAgain": password_again
                })),
            )
            .await?;
        Ok(check_response!(Token: response))
    }

    pub async fn create_phone_verification(client: &AppWriteClient) -> Result<Token, Error> {
        let api_url = "/account/verification/phone";
        let response = client
            .call(Method::POST, api_url, RequestData::None)
            .await?;
        Ok(check_response!(Token: response))
    }

    pub async fn confirm_phone_verification(
        client: &AppWriteClient,
        user_id: &UserId,
        secret: &str,
    ) -> Result<Token, Error> {
        let api_url = "/account/verification/phone";
        let response = client
            .call(
                Method::PUT,
                api_url,
                RequestData::Json(serde_json::json!({
                    "userId": user_id,
                    "secret": secret,
                })),
            )
            .await?;
        Ok(check_response!(Token: response))
    }
}
