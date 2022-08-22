use bytes::Bytes;
use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        bucket::{Bucket, BucketId, FileSize},
        file::{File, FileId, InputFile},
        permission::Permission,
        ListResponse,
    },
};

use super::SearchPayload;

pub struct StoragesService;

#[derive(Debug, Clone, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct CreateBucketPayload {
    pub bucket_id: BucketId,
    pub name: String,
    pub permission: Permission,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub read: Vec<Permission>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub write: Vec<Permission>,
    #[default = true]
    pub enabled: bool,
    pub maximum_file_size: Option<FileSize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_file_extensions: Vec<String>,
    pub encryption: Option<bool>,
    pub antivirus: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBucketPayload {
    pub name: String,
    pub permission: Permission,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub read: Vec<Permission>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub write: Vec<Permission>,
    #[default = true]
    pub enabled: bool,
    pub maximum_file_size: Option<FileSize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_file_extensions: Vec<String>,
    pub encryption: Option<bool>,
    pub antivirus: Option<bool>,
}

impl StoragesService {
    pub async fn create_bucket(
        client: &AppWriteClient,
        payload: CreateBucketPayload,
    ) -> Result<Bucket, crate::error::Error> {
        let url = "/storage/buckets";
        let response = client
            .call(
                Method::POST,
                url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Bucket: response))
    }

    pub async fn list_buckets(
        client: &AppWriteClient,
        payload: SearchPayload<BucketId>,
    ) -> Result<ListResponse<Bucket>, crate::error::Error> {
        let url = "/storage/buckets";
        let response = client
            .call(
                Method::GET,
                url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Bucket>: response))
    }

    pub async fn get_bucket(
        client: &AppWriteClient,
        bucket_id: &BucketId,
    ) -> Result<Bucket, crate::error::Error> {
        let url = format!("/storage/buckets/{bucket_id}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Bucket: response))
    }

    pub async fn update_bucket(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        payload: UpdateBucketPayload,
    ) -> Result<Bucket, crate::error::Error> {
        let url = format!("/storage/buckets/{bucket_id}");
        let response = client
            .call(
                Method::PUT,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Bucket: response))
    }

    pub async fn delete_bucket(
        client: &AppWriteClient,
        bucket_id: &BucketId,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/storage/buckets/{bucket_id}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_file(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
        input_file: InputFile,
        read: &[Permission],
        write: &[Permission],
    ) -> Result<File, crate::error::Error> {
        let url = format!("/storage/buckets/{bucket_id}/files", bucket_id = bucket_id);
        let mut form = Vec::new();
        for r in read.iter() {
            form.push(("read[]".to_string(), r.to_string()));
        }
        for w in write.iter() {
            form.push(("write[]".to_string(), w.to_string()));
        }
        form.push(("fileId".to_string(), file_id.to_string()));
        let response = client
            .chunk_upload::<File>(
                Method::POST,
                &url,
                "file",
                "fileId",
                input_file,
                form,
                None,
                Some(file_id.0.clone()),
            )
            .await?;
        let response = match response {
            None => return Err(Error::SendFailed),
            Some(response) => response,
        };
        Ok(response)
    }

    pub async fn list_files(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        payload: SearchPayload<FileId>,
    ) -> Result<ListResponse<File>, crate::error::Error> {
        let url = format!("/storage/buckets/{bucket_id}/files", bucket_id = bucket_id);
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<File>: response))
    }

    pub async fn get_file(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
    ) -> Result<File, crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(File: response))
    }

    pub async fn get_file_preview(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
        // TODO: add preview query params
    ) -> Result<Bytes, crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}/preview",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        check_response!(response);
        let binary = response.bytes().await?;
        Ok(binary)
    }

    pub async fn get_file_download(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}/download",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        check_response!(response);
        Ok(response)
    }

    pub async fn get_file_view(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
    ) -> Result<reqwest::Response, crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}/view",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        check_response!(response);
        Ok(response)
    }

    pub async fn update_file(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
        read: &[Permission],
        write: &[Permission],
    ) -> Result<File, crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client
            .call(
                Method::PUT,
                &url,
                RequestData::Json(serde_json::json!({
                    "read": read,
                    "write": write,
                })),
            )
            .await?;
        Ok(check_response!(File: response))
    }

    pub async fn delete_file(
        client: &AppWriteClient,
        bucket_id: &BucketId,
        file_id: &FileId,
    ) -> Result<(), crate::error::Error> {
        let url = format!(
            "/storage/buckets/{bucket_id}/files/{file_id}",
            bucket_id = bucket_id,
            file_id = file_id
        );
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }
}
