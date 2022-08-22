use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        attribute::Attribute,
        collection::{Collection, CollectionId, CollectionPermission},
        database::{Database, DatabaseId},
        document::{Document, DocumentId},
        index::Index,
        permission::Permission,
        ListResponse,
    },
};

use super::{SearchPayload, SearchQueryPayload};

pub struct DatabasesService;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabasePayload {
    pub user_id: DatabaseId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionPayload {
    pub collection_id: CollectionId,
    pub name: String,
    pub permission: CollectionPermission,
    pub read: Vec<Permission>,
    pub write: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIndexPayload {
    pub key: String,
    #[serde(rename = "type")]
    pub index_type: String,
    pub attributes: Vec<Attribute>,
    // TODO: add model
    pub orders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentPayload {
    pub document_id: DocumentId,
    pub data: serde_json::Value,
    pub read: Option<Vec<Permission>>,
    pub write: Option<Vec<Permission>>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentPayload {
    pub data: Option<serde_json::Value>,
    pub read: Option<Vec<Permission>>,
    pub write: Option<Vec<Permission>>,
}

impl DatabasesService {
    pub async fn create_database(
        client: &AppWriteClient,
        payload: CreateDatabasePayload,
    ) -> Result<Database, crate::error::Error> {
        let url = "/databases";
        let response = client
            .call(
                Method::POST,
                url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Database: response))
    }

    pub async fn list_databases(
        client: &AppWriteClient,
        payload: SearchPayload<DatabaseId>,
    ) -> Result<ListResponse<Database>, crate::error::Error> {
        let url = "/databases";
        let response = client
            .call(
                Method::GET,
                url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Database>: response))
    }

    pub async fn get_database(
        client: &AppWriteClient,
        database_id: &DatabaseId,
    ) -> Result<Database, crate::error::Error> {
        let url = format!("/databases/{database_id}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Database: response))
    }

    pub async fn update_database_name(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        name: &str,
    ) -> Result<Database, crate::error::Error> {
        let url = format!("/databases/{database_id}");
        let body = serde_json::json!({
            "name": name,
        });
        let response = client
            .call(Method::PUT, &url, RequestData::Json(body))
            .await?;
        Ok(check_response!(Database: response))
    }

    pub async fn delete_database(
        client: &AppWriteClient,
        database_id: &DatabaseId,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/databases/{database_id}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_collection(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        payload: CreateCollectionPayload,
    ) -> Result<Collection, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections");
        let response = client
            .call(
                Method::POST,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Collection: response))
    }

    pub async fn list_collections(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        payload: SearchPayload<CollectionId>,
    ) -> Result<ListResponse<Collection>, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections");
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Collection>: response))
    }

    pub async fn create_attribute(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        attribute: Attribute,
    ) -> Result<(), crate::error::Error> {
        let url = attribute.get_url_endpoint(database_id, collection_id);
        let payload = attribute.convert_to_request_body()?;
        let response = client
            .call(Method::POST, &url, RequestData::Json(payload))
            .await?;
        Ok(check_response!(response))
    }

    pub async fn delete_attribute(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        key: &str,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/attributes/{key}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_index(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        payload: CreateIndexPayload,
    ) -> Result<Index, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/indexes");
        let response = client
            .call(
                Method::POST,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Index: response))
    }

    pub async fn list_indexes(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
    ) -> Result<ListResponse<Index>, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/indexes");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Index>: response))
    }

    pub async fn get_index(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        key: &str,
    ) -> Result<Index, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/indexes/{key}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Index: response))
    }

    pub async fn delete_index(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        key: &str,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/indexes/{key}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_document(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        payload: CreateDocumentPayload,
    ) -> Result<Document, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/documents");
        let response = client
            .call(
                Method::POST,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Document: response))
    }

    pub async fn list_documents(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        payload: SearchQueryPayload<DocumentId>,
    ) -> Result<ListResponse<Document>, crate::error::Error> {
        let url = format!("/databases/{database_id}/collections/{collection_id}/documents");
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Document>: response))
    }

    pub async fn get_document(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        document_id: &DocumentId,
    ) -> Result<Document, crate::error::Error> {
        let url =
            format!("/databases/{database_id}/collections/{collection_id}/documents/{document_id}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Document: response))
    }

    pub async fn update_document(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        document_id: &DocumentId,
        payload: UpdateDocumentPayload,
    ) -> Result<Document, crate::error::Error> {
        let url =
            format!("/databases/{database_id}/collections/{collection_id}/documents/{document_id}");
        let response = client
            .call(
                Method::PATCH,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Document: response))
    }

    pub async fn delete_document(
        client: &AppWriteClient,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
        document_id: &DocumentId,
    ) -> Result<(), crate::error::Error> {
        let url =
            format!("/databases/{database_id}/collections/{collection_id}/documents/{document_id}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }
}
