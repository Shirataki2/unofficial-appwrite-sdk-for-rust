
use super::{
    file::{File, FileId, InputFile},
    permission::Permission,
    ListKey, ListResponse, TimeStamp,
};

use crate::{
    client::AppWriteClient,
    services::{storages::*, SearchPayload},
};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BucketId(pub String);

impl BucketId {
    pub fn new(id: String) -> Self {
        BucketId(id)
    }

    pub fn unique() -> Self {
        BucketId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    #[serde(rename = "$id")]
    pub id: BucketId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$permissions")]
    pub permissions: Vec<Permission>,
    pub file_security: bool,
    pub name: String,
    pub enabled: bool,
    pub maximum_file_size: FileSize,
    pub allowed_file_extensions: Vec<String>,
    #[serde(skip_serializing_if = "Compression::is_none")]
    pub compression: Compression,
    pub encryption: bool,
    pub antivirus: bool,
}

impl ListKey for Bucket {
    fn list_key() -> &'static str {
        "buckets"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSize(u64);

pub trait FileSizeExt {
    fn bytes(self) -> u64;
    fn kb(self) -> FileSize;
    fn mb(self) -> FileSize;
    fn gb(self) -> FileSize;
}

macro_rules! impl_filesize {
    ($ty:ty) => {
        impl FileSizeExt for $ty {
            fn bytes(self) -> u64 {
                self as u64
            }
            fn kb(self) -> FileSize {
                FileSize(self as u64 * 1024)
            }
            fn mb(self) -> FileSize {
                FileSize(self as u64 * 1024 * 1024)
            }
            fn gb(self) -> FileSize {
                FileSize(self as u64 * 1024 * 1024 * 1024)
            }
        }
    };
    ($ty:ty, $($rest:ty),+) => {
        impl_filesize!($ty);
        impl_filesize!($($rest),+);
    };
}

impl_filesize!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl Bucket {
    pub async fn create(
        client: &AppWriteClient,
        payload: CreateBucketPayload,
    ) -> Result<Bucket, crate::error::Error> {
        StoragesService::create_bucket(client, payload).await
    }

    pub async fn get(client: &AppWriteClient, id: BucketId) -> Result<Bucket, crate::error::Error> {
        StoragesService::get_bucket(client, &id).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<BucketId>,
    ) -> Result<ListResponse<Bucket>, crate::error::Error> {
        StoragesService::list_buckets(client, payload).await
    }

    pub async fn update(
        &self,
        client: &AppWriteClient,
        payload: UpdateBucketPayload,
    ) -> Result<Bucket, crate::error::Error> {
        StoragesService::update_bucket(client, &self.id, payload).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        StoragesService::delete_bucket(client, &self.id).await
    }

    pub async fn create_file(
        &self,
        client: &AppWriteClient,
        file_id: FileId,
        input_file: InputFile,
        permissions: &[Permission],
    ) -> Result<File, crate::error::Error> {
        StoragesService::create_file(client, &self.id, &file_id, input_file, permissions).await
    }

    pub async fn list_files(
        &self,
        client: &AppWriteClient,
        payload: SearchPayload<FileId>,
    ) -> Result<ListResponse<File>, crate::error::Error> {
        StoragesService::list_files(client, &self.id, payload).await
    }

    pub async fn get_file(
        &self,
        client: &AppWriteClient,
        file_id: &FileId,
    ) -> Result<File, crate::error::Error> {
        StoragesService::get_file(client, &self.id, file_id).await
    }
}
