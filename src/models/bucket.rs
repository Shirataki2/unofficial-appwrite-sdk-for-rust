use chrono::serde::ts_seconds;

use super::{ListKey, TimeStamp};

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
    #[serde(rename = "$createdAt", with = "ts_seconds")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt", with = "ts_seconds")]
    pub updated_at: TimeStamp,
    pub name: String,
    #[serde(with = "ts_seconds")]
    pub registration: TimeStamp,
    pub status: bool,
    #[serde(with = "ts_seconds")]
    pub password_update: TimeStamp,
    pub email: String,
    pub phone: String,
    pub email_verification: bool,
    pub phone_verification: bool,
    // TODO: add preferences model
    pub prefs: serde_json::Value,
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
