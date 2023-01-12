use std::path::Path;

use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{client::AppWriteClient, error::Error, services::storages::StoragesService};

use super::{
    bucket::{BucketId, FileSize},
    permission::Permission,
    Id, ListKey, TimeStamp,
};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FileId(pub String);

impl FileId {
    pub fn new(id: String) -> Self {
        FileId(id)
    }

    pub fn unique() -> Self {
        FileId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(rename = "$id")]
    pub id: FileId,
    pub bucket_id: BucketId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$permissions")]
    pub permissions: Vec<Permission>,
    pub name: String,
    pub signature: String,
    pub mime_type: String,
    pub size_original: FileSize,
    pub chunks_total: u64,
    pub chunks_uploaded: u64,
}

impl File {
    pub async fn create(
        &self,
        client: &AppWriteClient,
        input_file: InputFile,
        permissions: &[Permission],
    ) -> Result<File, crate::error::Error> {
        StoragesService::create_file(client, &self.bucket_id, &self.id, input_file, permissions)
            .await
    }

    pub async fn get_preview(
        &self,
        client: &AppWriteClient,
    ) -> Result<bytes::Bytes, crate::error::Error> {
        StoragesService::get_file_preview(client, &self.bucket_id, &self.id).await
    }

    pub async fn get_download(
        &self,
        client: &AppWriteClient,
    ) -> Result<reqwest::Response, crate::error::Error> {
        StoragesService::get_file_download(client, &self.bucket_id, &self.id).await
    }

    pub async fn update(
        &self,
        client: &AppWriteClient,
        permissions: &[Permission],
    ) -> Result<File, crate::error::Error> {
        StoragesService::update_file(client, &self.bucket_id, &self.id, permissions).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        StoragesService::delete_file(client, &self.bucket_id, &self.id).await
    }
}

impl ListKey for File {
    fn list_key() -> &'static str {
        "files"
    }
}

impl Id for File {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}

#[derive(Debug)]
pub enum InputFileType {
    File(tokio::fs::File),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub struct InputFile {
    pub name: String,
    pub mime_type: String,
    pub size: usize,
    pub cursor: usize,
    pub ty: InputFileType,
}

impl InputFile {
    pub async fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let file = tokio::fs::File::open(path).await?;
        let meta = file.metadata().await?;
        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => "Unknown".to_string(),
        };
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        Ok(InputFile {
            name,
            size: meta.len() as usize,
            mime_type,
            cursor: 0,
            ty: InputFileType::File(file),
        })
    }

    pub async fn from_bytes(
        bytes: Vec<u8>,
        name: Option<String>,
        mime_type: Option<String>,
    ) -> Result<Self, Error> {
        let name = name.unwrap_or_else(|| "Unknown".to_string());
        let mime_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
        Ok(InputFile {
            name,
            size: bytes.len(),
            mime_type,
            cursor: 0,
            ty: InputFileType::Bytes(bytes),
        })
    }

    pub async fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
        let n = match self.ty {
            InputFileType::File(ref mut file) => {
                if self.cursor + buf.len() > self.size {
                    buf = &mut buf[..self.size - self.cursor];
                }
                file.read_exact(buf).await?
            }
            InputFileType::Bytes(ref mut bytes) => {
                let n = bytes.len() - self.cursor;
                let n = std::cmp::min(n, buf.len());
                buf[..n].copy_from_slice(&bytes[self.cursor..self.cursor + n]);
                n
            }
        };
        self.cursor += n;
        Ok(n)
    }

    pub async fn seek(&mut self, pos: usize) -> Result<(), Error> {
        self.cursor = pos;
        if let InputFileType::File(ref mut file) = self.ty {
            file.seek(std::io::SeekFrom::Start(pos as u64)).await?;
        }
        Ok(())
    }
}

#[allow(clippy::from_over_into)]
impl Into<reqwest::Body> for InputFile {
    fn into(self) -> reqwest::Body {
        match self.ty {
            InputFileType::File(file) => reqwest::Body::from(file),
            InputFileType::Bytes(bytes) => reqwest::Body::from(bytes),
        }
    }
}
