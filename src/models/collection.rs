
use crate::{client::AppWriteClient, services::databases::*};

use super::{
    attribute::Attribute,
    database::DatabaseId,
    document::{Document, DocumentId},
    index::Index,
    permission::Permission,
    query::Query,
    ListKey, ListResponse, TimeStamp,
};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CollectionId(pub String);

impl CollectionId {
    pub fn new(id: String) -> Self {
        CollectionId(id)
    }

    pub fn unique() -> Self {
        CollectionId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "$id")]
    pub id: CollectionId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    #[serde(rename = "$permissions")]
    pub permissions: Vec<Permission>,
    pub database_id: DatabaseId,
    pub name: String,
    pub enabled: bool,
    pub document_security: bool,
    pub attributes: Vec<Attribute>,
    pub indexes: Vec<Index>,
}

impl ListKey for Collection {
    fn list_key() -> &'static str {
        "collections"
    }
}

impl Collection {
    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        DatabasesService::delete_collection(client, &self.database_id, &self.id).await
    }

    pub async fn create_attribute(
        &self,
        client: &AppWriteClient,
        attr: Attribute,
    ) -> Result<(), crate::error::Error> {
        DatabasesService::create_attribute(client, &self.database_id, &self.id, attr).await
    }

    pub async fn delete_attribute(
        &self,
        client: &AppWriteClient,
        key: &str,
    ) -> Result<(), crate::error::Error> {
        DatabasesService::delete_attribute(client, &self.database_id, &self.id, key).await
    }

    pub async fn create_index(
        &self,
        client: &AppWriteClient,
        payload: CreateIndexPayload,
    ) -> Result<Index, crate::error::Error> {
        DatabasesService::create_index(client, &self.database_id, &self.id, payload).await
    }

    pub async fn list_indexes(
        &self,
        client: &AppWriteClient,
    ) -> Result<ListResponse<Index>, crate::error::Error> {
        DatabasesService::list_indexes(client, &self.database_id, &self.id).await
    }

    pub async fn get_index(
        &self,
        client: &AppWriteClient,
        key: &str,
    ) -> Result<Index, crate::error::Error> {
        DatabasesService::get_index(client, &self.database_id, &self.id, key).await
    }

    pub async fn delete_index(
        &self,
        client: &AppWriteClient,
        key: &str,
    ) -> Result<(), crate::error::Error> {
        DatabasesService::delete_index(client, &self.database_id, &self.id, key).await
    }

    pub async fn create_document<T>(
        &self,
        client: &AppWriteClient,
        payload: CreateDocumentPayload,
    ) -> Result<Document<T>, crate::error::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        DatabasesService::create_document(client, &self.database_id, &self.id, payload).await
    }

    pub async fn list_documents<T>(
        &self,
        client: &AppWriteClient,
        queries: Vec<Query>,
    ) -> Result<ListResponse<Document<T>>, crate::error::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        DatabasesService::list_documents(client, &self.database_id, &self.id, Some(queries)).await
    }

    pub async fn get_document<T>(
        &self,
        client: &AppWriteClient,
        document_id: &DocumentId,
    ) -> Result<Document<T>, crate::error::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        DatabasesService::get_document(client, &self.database_id, &self.id, document_id).await
    }

    pub async fn update_document<T>(
        &self,
        client: &AppWriteClient,
        document_id: &DocumentId,
        payload: UpdateDocumentPayload,
    ) -> Result<Document<T>, crate::error::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        DatabasesService::update_document(client, &self.database_id, &self.id, document_id, payload)
            .await
    }

    pub async fn delete_document(
        &self,
        client: &AppWriteClient,
        document_id: &DocumentId,
    ) -> Result<(), crate::error::Error> {
        DatabasesService::delete_document(client, &self.database_id, &self.id, document_id).await
    }
}
