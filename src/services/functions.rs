use std::collections::HashMap;

use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        deployment::{Deployment, DeploymentId},
        execution::{Execution, ExecutionId},
        file::InputFile,
        function::{Function, FunctionId},
        permission::Permission,
        runtime::Runtime,
        ListResponse,
    },
};

use super::SearchPayload;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunctionPayload {
    pub function_id: FunctionId,
    pub name: String,
    pub execute: Vec<Permission>,
    pub runtime: String,
    pub vars: Option<HashMap<String, String>>,
    // TODO: Add event and schedule models
    pub events: Option<Vec<String>>,
    pub schedule: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFunctionPayload {
    pub name: String,
    pub execute: Vec<Permission>,
    pub runtime: String,
    pub vars: Option<HashMap<String, String>>,
    // TODO: Add event and schedule models
    pub events: Option<Vec<String>>,
    pub schedule: Option<String>,
    pub timeout: Option<u64>,
}

pub struct FunctionsService;

impl FunctionsService {
    pub async fn create_function(
        client: &AppWriteClient,
        payload: CreateFunctionPayload,
    ) -> Result<Function, crate::error::Error> {
        let url = "/functions";
        let response = client
            .call(
                Method::POST,
                url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Function: response))
    }

    pub async fn list_functions(
        client: &AppWriteClient,
        payload: SearchPayload<FunctionId>,
    ) -> Result<ListResponse<Function>, crate::error::Error> {
        let url = "/functions";
        let response = client
            .call(
                Method::GET,
                url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Function>: response))
    }

    pub async fn list_runtimes(
        client: &AppWriteClient,
    ) -> Result<ListResponse<Runtime>, crate::error::Error> {
        let url = "/functions/runtimes";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Runtime>: response))
    }

    pub async fn get_function(
        client: &AppWriteClient,
        function_id: &FunctionId,
    ) -> Result<Function, crate::error::Error> {
        let url = format!("/functions/{function_id}", function_id = function_id);
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Function: response))
    }

    pub async fn update_function(
        client: &AppWriteClient,
        function_id: &FunctionId,
        payload: UpdateFunctionPayload,
    ) -> Result<Function, crate::error::Error> {
        let url = format!("/functions/{function_id}", function_id = function_id);
        let response = client
            .call(
                Method::PUT,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Function: response))
    }

    pub async fn update_function_deployment(
        client: &AppWriteClient,
        function_id: &FunctionId,
        deployment_id: &DeploymentId,
    ) -> Result<Function, crate::error::Error> {
        let url = format!("/functions/{function_id}/deployment/{deployment_id}");
        let response = client.call(Method::PATCH, &url, RequestData::None).await?;
        Ok(check_response!(Function: response))
    }

    pub async fn delete_function(
        client: &AppWriteClient,
        function_id: &FunctionId,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/functions/{function_id}", function_id = function_id);
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_deployment(
        client: &AppWriteClient,
        function_id: &FunctionId,
        input_file: InputFile,
        entrypoint: String,
        activate: bool,
    ) -> Result<Deployment, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/deployments",
            function_id = function_id
        );
        let mut form: Vec<(String, String)> = Vec::new();
        form.push(("entrypoint".into(), entrypoint));
        form.push((
            "activate".into(),
            if activate {
                "true".into()
            } else {
                "false".into()
            },
        ));
        let response = client
            .chunk_upload::<Deployment>(
                Method::POST,
                &url,
                "code",
                "",
                input_file,
                form,
                None,
                None,
            )
            .await?;
        let response = match response {
            None => return Err(Error::SendFailed),
            Some(response) => response,
        };
        Ok(response)
    }

    pub async fn list_deployments(
        client: &AppWriteClient,
        function_id: &FunctionId,
        payload: SearchPayload<DeploymentId>,
    ) -> Result<ListResponse<Deployment>, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/deployments",
            function_id = function_id
        );
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Deployment>: response))
    }

    pub async fn get_deployment(
        client: &AppWriteClient,
        function_id: &FunctionId,
        deployment_id: &DeploymentId,
    ) -> Result<Deployment, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/deployments/{deployment_id}",
            function_id = function_id,
            deployment_id = deployment_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Deployment: response))
    }

    pub async fn delete_deployment(
        client: &AppWriteClient,
        function_id: &FunctionId,
        deployment_id: &DeploymentId,
    ) -> Result<(), crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/deployments/{deployment_id}",
            function_id = function_id,
            deployment_id = deployment_id
        );
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_execution(
        client: &AppWriteClient,
        function_id: &FunctionId,
        data: Option<String>,
        asynchronize: Option<bool>,
    ) -> Result<Execution, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/executions",
            function_id = function_id
        );
        let payload = serde_json::json!( {
            "data": data,
            "asynchronize": asynchronize,
        });
        let response = client
            .call(Method::POST, &url, RequestData::Json(payload))
            .await?;
        Ok(check_response!(Execution: response))
    }

    pub async fn list_executions(
        client: &AppWriteClient,
        function_id: &FunctionId,
        payload: SearchPayload<ExecutionId>,
    ) -> Result<ListResponse<Execution>, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/executions",
            function_id = function_id
        );
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Execution>: response))
    }

    pub async fn get_execution(
        client: &AppWriteClient,
        function_id: &FunctionId,
        execution_id: &ExecutionId,
    ) -> Result<Execution, crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/executions/{execution_id}",
            function_id = function_id,
            execution_id = execution_id
        );
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Execution: response))
    }

    pub async fn retry_build(
        client: &AppWriteClient,
        function_id: &FunctionId,
        deployment_id: &DeploymentId,
        build_id: &str, // TODO: use BuildId
    ) -> Result<(), crate::error::Error> {
        let url = format!(
            "/functions/{function_id}/deployments/{deployment_id}/builds/{build_id}",
            function_id = function_id,
            deployment_id = deployment_id,
            build_id = build_id
        );
        let response = client.call(Method::POST, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }
}
