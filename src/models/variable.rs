use crate::prelude::*;

use super::{Id, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct VariableId(pub String);

impl VariableId {
    pub fn new(id: String) -> Self {
        VariableId(id)
    }

    pub fn unique() -> Self {
        VariableId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    #[serde(rename = "$id")]
    pub id: VariableId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub key: String,
    pub value: String,
    pub function_id: FunctionId,
}

impl ListKey for Variable {
    fn list_key() -> &'static str {
        "variables"
    }
}

impl Id for Variable {
    fn id(&self) -> String {
        self.id.0.clone()
    }
}

impl Variable {
    pub async fn update<V>(
        &mut self,
        client: &AppWriteClient,
        key: String,
        value: Option<V>,
    ) -> Result<Variable, crate::error::Error>
    where
        V: serde::Serialize,
    {
        let new =
            FunctionsService::update_variable(client, &self.function_id, &self.id, key, value)
                .await?;
        *self = new.clone();
        Ok(new)
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        FunctionsService::delete_variable(client, &self.function_id, &self.id).await
    }
}
