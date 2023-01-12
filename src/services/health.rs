use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::health::{Health, HealthAntivirus, HealthTime},
};

pub struct HealthService;

impl HealthService {
    pub async fn get_http(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_db(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/db";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_cache(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/cache";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_time(client: &AppWriteClient) -> Result<HealthTime, Error> {
        let url = "/health/time";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(HealthTime: response))
    }

    pub async fn get_webhooks(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/webhooks";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_logs(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/logs";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_certificates(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/certificates";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_functions(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/functions";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_local_storage(client: &AppWriteClient) -> Result<Health, Error> {
        let url = "/health/storage/local";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Health: response))
    }

    pub async fn get_antivirus(client: &AppWriteClient) -> Result<HealthAntivirus, Error> {
        let url = "/health/anti-virus";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(HealthAntivirus: response))
    }
}
