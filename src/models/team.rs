
use crate::prelude::*;

use super::{ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TeamId(pub String);

impl TeamId {
    pub fn new(id: String) -> Self {
        TeamId(id)
    }

    pub fn unique() -> Self {
        TeamId("unique()".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "$id")]
    pub id: TeamId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub name: String,
    pub total: u64,
}

impl ListKey for Team {
    fn list_key() -> &'static str {
        "teams"
    }
}

impl Team {
    pub async fn create(
        client: &AppWriteClient,
        payload: CreateTeamPayload,
    ) -> Result<Team, crate::error::Error> {
        TeamsService::create_team(client, payload).await
    }

    pub async fn get(client: &AppWriteClient, id: &TeamId) -> Result<Team, crate::error::Error> {
        TeamsService::get_team(client, id).await
    }

    pub async fn list(
        client: &AppWriteClient,
        payload: SearchPayload<TeamId>,
    ) -> Result<ListResponse<Team>, crate::error::Error> {
        TeamsService::list_teams(client, payload).await
    }

    pub async fn update(
        &self,
        client: &AppWriteClient,
        name: &str,
    ) -> Result<Team, crate::error::Error> {
        TeamsService::update_name(client, &self.id, name).await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        TeamsService::delete_team(client, &self.id).await
    }

    pub async fn create_membership(
        &self,
        client: &AppWriteClient,
        payload: CreateMembershipPayload,
    ) -> Result<Membership, crate::error::Error> {
        TeamsService::create_membership(client, &self.id, payload).await
    }

    pub async fn get_membership(
        &self,
        client: &AppWriteClient,
        id: &MembershipId,
    ) -> Result<Membership, crate::error::Error> {
        TeamsService::get_membership(client, &self.id, id).await
    }
}
