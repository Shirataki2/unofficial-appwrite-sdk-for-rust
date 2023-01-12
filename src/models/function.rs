
use crate::{empty_deploy_as_none, prelude::*};

use super::{Id, ListKey};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FunctionId(pub String);

impl FunctionId {
    pub fn new(id: String) -> Self {
        FunctionId(id)
    }

    pub fn unique() -> Self {
        FunctionId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "$id")]
    pub id: FunctionId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub execute: Vec<String>,
    pub name: String,
    pub status: FunctionStatus,
    pub runtime: ExecutionRuntime,
    #[serde(deserialize_with = "empty_deploy_as_none")]
    pub deployment: Option<DeploymentId>,
    // TODO: v1.2では環境変数が未設定の場合には空の配列が返ってくるが、
    // 設定済みの場合にはJSONオブジェクトが返ってくる。
    // 仕様変更に応じて修正する。
    pub vars: serde_json::Value,
    pub events: Vec<String>,
    pub schedule: String,
    #[serde()]
    pub schedule_next: TimeStamp,
    #[serde()]
    pub schedule_previous: TimeStamp,
    pub timeout: u64,
}

impl ListKey for Function {
    fn list_key() -> &'static str {
        "functions"
    }
}

impl Id for Function {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}

impl Function {
    pub async fn create(
        client: &AppWriteClient,
        payload: CreateFunctionPayload,
    ) -> Result<Function, crate::error::Error> {
        FunctionsService::create_function(client, payload).await
    }

    pub async fn get(
        client: &AppWriteClient,
        function_id: &FunctionId,
    ) -> Result<Function, crate::error::Error> {
        FunctionsService::get_function(client, function_id).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<FunctionId>,
    ) -> Result<ListResponse<Function>, crate::error::Error> {
        FunctionsService::list_functions(client, payload).await
    }

    pub async fn update(
        &self,
        client: &AppWriteClient,
        payload: UpdateFunctionPayload,
    ) -> Result<Function, crate::error::Error> {
        FunctionsService::update_function(client, &self.id, payload).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        FunctionsService::delete_function(client, &self.id).await
    }

    pub async fn list_runtimes(
        client: &AppWriteClient,
    ) -> Result<ListResponse<Runtime>, crate::error::Error> {
        FunctionsService::list_runtimes(client).await
    }

    pub async fn create_deployment(
        &self,
        client: &AppWriteClient,
        input_file: InputFile,
        entrypoint: String,
        activate: bool,
    ) -> Result<Deployment, crate::error::Error> {
        FunctionsService::create_deployment(client, &self.id, input_file, entrypoint, activate)
            .await
    }

    pub async fn list_deployments(
        &self,
        client: &AppWriteClient,
        payload: SearchPayload<DeploymentId>,
    ) -> Result<ListResponse<Deployment>, crate::error::Error> {
        FunctionsService::list_deployments(client, &self.id, payload).await
    }

    pub async fn get_deployment(
        &self,
        client: &AppWriteClient,
        deployment_id: &DeploymentId,
    ) -> Result<Deployment, crate::error::Error> {
        FunctionsService::get_deployment(client, &self.id, deployment_id).await
    }

    pub async fn change_deployment(
        &self,
        client: &AppWriteClient,
        deployment_id: &DeploymentId,
    ) -> Result<Function, crate::error::Error> {
        FunctionsService::update_function_deployment(client, &self.id, deployment_id).await
    }

    pub async fn delete_deployment(
        &self,
        client: &AppWriteClient,
        deployment_id: &DeploymentId,
    ) -> Result<(), crate::error::Error> {
        FunctionsService::delete_deployment(client, &self.id, deployment_id).await
    }

    pub async fn create_execution(
        &self,
        client: &AppWriteClient,
        data: Option<String>,
        asynchronize: Option<bool>,
    ) -> Result<Execution, crate::error::Error> {
        FunctionsService::create_execution(client, &self.id, data, asynchronize).await
    }

    pub async fn list_executions(
        &self,
        client: &AppWriteClient,
        payload: SearchPayload<ExecutionId>,
    ) -> Result<ListResponse<Execution>, crate::error::Error> {
        FunctionsService::list_executions(client, &self.id, payload).await
    }

    pub async fn get_execution(
        &self,
        client: &AppWriteClient,
        execution_id: &ExecutionId,
    ) -> Result<Execution, crate::error::Error> {
        FunctionsService::get_execution(client, &self.id, execution_id).await
    }

    pub async fn create_build(
        &self,
        client: &AppWriteClient,
        deployment_id: &DeploymentId,
        build_id: &BuildId,
    ) -> Result<(), crate::error::Error> {
        FunctionsService::create_build(client, &self.id, deployment_id, build_id).await
    }

    pub async fn create_varible<V>(
        &self,
        client: &AppWriteClient,
        key: &str,
        value: V,
    ) -> Result<Variable, crate::error::Error>
    where
        V: serde::Serialize,
    {
        FunctionsService::create_variable(client, &self.id, key, value).await
    }

    pub async fn list_variables(
        &self,
        client: &AppWriteClient,
    ) -> Result<ListResponse<Variable>, crate::error::Error> {
        FunctionsService::list_variables(client, &self.id).await
    }

    pub async fn get_variable(
        &self,
        client: &AppWriteClient,
        variable_id: &VariableId,
    ) -> Result<Variable, crate::error::Error> {
        FunctionsService::get_variable(client, &self.id, variable_id).await
    }
}
