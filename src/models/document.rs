
use crate::prelude::*;

use super::{collection::CollectionId, permission::Permission, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new(id: String) -> Self {
        DocumentId(id)
    }

    pub fn unique() -> Self {
        DocumentId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Document<T> {
    #[serde(rename = "$id")]
    pub id: DocumentId,
    #[serde(rename = "$collectionId")]
    pub collection_id: CollectionId,
    #[serde(rename = "$databaseId")]
    pub database_id: DatabaseId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$permissions")]
    pub permissions: Vec<Permission>,
    #[serde(flatten)]
    pub data: T,
}

impl<T> ListKey for Document<T> {
    fn list_key() -> &'static str {
        "documents"
    }
}

// TODO: v1.2では何故か前仕様の返り値のため、database_id実装後にコメントアウトを外す
impl<T> Document<T>
where
    T: for<'de> serde::de::Deserialize<'de> + serde::Serialize + Clone,
{
    pub async fn update(
        &self,
        client: &AppWriteClient,
    ) -> Result<Document<T>, crate::error::Error> {
        let payload = UpdateDocumentPayload {
            data: Some(serde_json::to_value(self.data.clone())?),
            permissions: self.permissions.clone(),
        };
        DatabasesService::update_document(
            client,
            &self.database_id,
            &self.collection_id,
            &self.id,
            payload,
        )
        .await
    }
}

impl<T> Document<T>
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    pub async fn delete(
        &self,
        client: &AppWriteClient,
    ) -> Result<(), crate::error::Error> {
        DatabasesService::delete_document(client, &self.database_id, &self.collection_id, &self.id).await
    }
}
