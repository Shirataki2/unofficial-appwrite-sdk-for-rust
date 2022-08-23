use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    insert_if_some,
    models::avatar::CreditcardCode,
};

pub struct AvatarService;

impl AvatarService {
    pub async fn get_creditcard_icon(
        client: &AppWriteClient,
        icon: CreditcardCode,
        width: Option<u32>,
        height: Option<u32>,
        quality: Option<u32>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let icon = serde_json::to_string(&icon)?;
        let url = format!("/avatars/credit-cards/{icon}");
        let mut payload = vec![];
        insert_if_some!(
            payload =>
            "width", width;
            "height", height;
            "quality", quality
        );
        let response = client
            .call(Method::GET, &url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }

    pub async fn get_browser_icon(
        client: &AppWriteClient,
        icon: String,
        width: Option<u32>,
        height: Option<u32>,
        quality: Option<u32>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = format!("/avatars/browsers/{icon}");
        let mut payload = vec![];
        insert_if_some!(
            payload =>
            "width", width;
            "height", height;
            "quality", quality
        );
        let response = client
            .call(Method::GET, &url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }

    pub async fn get_country_icon(
        client: &AppWriteClient,
        icon: String,
        width: Option<u32>,
        height: Option<u32>,
        quality: Option<u32>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = format!("/avatars/countries/{icon}");
        let mut payload = vec![];
        insert_if_some!(
            payload =>
            "width", width;
            "height", height;
            "quality", quality
        );
        let response = client
            .call(Method::GET, &url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }

    pub async fn get_image(
        client: &AppWriteClient,
        url: String,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let api_url = "/avatars/image";
        let mut payload = vec![("url".to_string(), url.to_string())];
        insert_if_some!(
            payload =>
            "width", width;
            "height", height
        );
        let response = client
            .call(Method::GET, api_url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }
    pub async fn get_favicon(
        client: &AppWriteClient,
        url: String,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let api_url = "/avatars/favicon";
        let payload = vec![("url".to_string(), url.to_string())];
        let response = client
            .call(Method::GET, api_url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }
    pub async fn get_qr(
        client: &AppWriteClient,
        data: String,
        size: Option<usize>,
        margin: Option<usize>,
        download: Option<bool>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = "/avatars/qr";
        let mut payload = vec![("data".to_string(), data.to_string())];
        insert_if_some!(
            payload =>
            "size", size;
            "margin", margin;
            "download", download
        );
        let response = client
            .call(Method::GET, url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }
    pub async fn get_user_initial_avatar(
        client: &AppWriteClient,
        username: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        color: Option<String>,
        background: Option<String>,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = "/avatars/initials";
        let mut payload = vec![];
        insert_if_some!(
            payload =>
            "username", username;
            "width", width;
            "height", height;
            "color", color;
            "background", background
        );
        let response = client
            .call(Method::GET, url, RequestData::Params(payload))
            .await?;
        check_response!(response);
        Ok(response)
    }
}
