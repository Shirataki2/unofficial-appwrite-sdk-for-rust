
use super::TimeStamp;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub ping: u32,
    pub status: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthTime {
    #[serde()]
    pub remote_time: TimeStamp,
    #[serde()]
    pub local_time: TimeStamp,
    pub diff: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthQueue {
    pub size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthAntivirusStatus {
    Disabled,
    Online,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthAntivirus {
    pub version: String,
    pub status: HealthAntivirusStatus,
}

impl Health {
    pub async fn http(client: &AppWriteClient) -> Result<Self, crate::error::Error> {
        HealthService::get_http(client).await
    }

    pub async fn db(client: &AppWriteClient) -> Result<Self, crate::error::Error> {
        HealthService::get_db(client).await
    }

    pub async fn cache(client: &AppWriteClient) -> Result<Self, crate::error::Error> {
        HealthService::get_cache(client).await
    }

    pub async fn time(client: &AppWriteClient) -> Result<HealthTime, crate::error::Error> {
        HealthService::get_time(client).await
    }

    pub async fn webhooks(client: &AppWriteClient) -> Result<Health, crate::error::Error> {
        HealthService::get_webhooks(client).await
    }

    pub async fn logs(client: &AppWriteClient) -> Result<Health, crate::error::Error> {
        HealthService::get_logs(client).await
    }

    pub async fn certificates(client: &AppWriteClient) -> Result<Health, crate::error::Error> {
        HealthService::get_certificates(client).await
    }

    pub async fn functions(client: &AppWriteClient) -> Result<Health, crate::error::Error> {
        HealthService::get_functions(client).await
    }

    pub async fn local_storage(client: &AppWriteClient) -> Result<Health, crate::error::Error> {
        HealthService::get_local_storage(client).await
    }

    pub async fn antivirus(
        client: &AppWriteClient,
    ) -> Result<HealthAntivirus, crate::error::Error> {
        HealthService::get_antivirus(client).await
    }
}
