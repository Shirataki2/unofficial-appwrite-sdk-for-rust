
use crate::{
    client::AppWriteClient,
    models::collection::*,
    services::{databases::*, SearchPayload},
};

use super::{ListKey, ListResponse, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DatabaseId(pub String);

impl DatabaseId {
    pub fn new(id: String) -> Self {
        DatabaseId(id)
    }

    pub fn unique() -> Self {
        DatabaseId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    #[serde(rename = "$id")]
    pub id: DatabaseId,
    pub name: String,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
}

impl ListKey for Database {
    fn list_key() -> &'static str {
        "databases"
    }
}

impl Database {
    pub async fn create(
        client: &AppWriteClient,
        payload: CreateDatabasePayload,
    ) -> Result<Database, crate::error::Error> {
        DatabasesService::create_database(client, payload).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<DatabaseId>,
    ) -> Result<ListResponse<Database>, crate::error::Error> {
        DatabasesService::list_databases(client, payload).await
    }

    pub async fn get(
        client: &AppWriteClient,
        id: &DatabaseId,
    ) -> Result<Database, crate::error::Error> {
        DatabasesService::get_database(client, id).await
    }

    pub async fn update_name(
        &self,
        client: &AppWriteClient,
        name: &str,
    ) -> Result<Database, crate::error::Error> {
        DatabasesService::update_database_name(client, &self.id, name).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        DatabasesService::delete_database(client, &self.id).await
    }

    pub async fn create_collection(
        &self,
        client: &AppWriteClient,
        payload: CreateCollectionPayload,
    ) -> Result<Collection, crate::error::Error> {
        DatabasesService::create_collection(client, &self.id, payload).await
    }

    pub async fn get_collection(
        &self,
        client: &AppWriteClient,
        id: &CollectionId,
    ) -> Result<Collection, crate::error::Error> {
        DatabasesService::get_collection(client, &self.id, id).await
    }

    pub async fn list_collections(
        &self,
        client: &AppWriteClient,
        payload: SearchPayload<CollectionId>,
    ) -> Result<ListResponse<Collection>, crate::error::Error> {
        DatabasesService::list_collections(client, &self.id, payload).await
    }
}
