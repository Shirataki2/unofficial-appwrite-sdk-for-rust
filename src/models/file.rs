use std::path::Path;

use chrono::serde::ts_seconds;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::error::Error;

use super::{
    bucket::{BucketId, FileSize},
    permission::Permission,
    HasId, ListKey, TimeStamp,
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
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$read")]
    pub read_perms: Vec<Permission>,
    #[serde(rename = "$write")]
    pub write_perms: Vec<Permission>,
    pub name: String,
    pub signature: String,
    pub mime_type: String,
    pub size_original: FileSize,
    pub chunks_total: u64,
    pub chunks_uploaded: u64,
}

impl ListKey for File {
    fn list_key() -> &'static str {
        "files"
    }
}

impl HasId for File {
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
