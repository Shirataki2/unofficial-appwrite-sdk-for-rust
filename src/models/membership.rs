
use crate::prelude::*;

use super::{user::UserId, ListKey, TimeStamp};

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MembershipId(pub String);

impl MembershipId {
    pub fn new(id: String) -> Self {
        MembershipId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    #[serde(rename = "$id")]
    pub id: MembershipId,
    #[serde(rename = "$createdAt")]
    pub created_at: TimeStamp,
    #[serde(rename = "$updatedAt")]
    pub updated_at: TimeStamp,
    pub user_id: UserId,
    pub user_name: String,
    pub user_email: String,
    pub team_id: TeamId,
    pub team_name: String,
    #[serde()]
    pub invited: TimeStamp,
    #[serde()]
    pub joined: TimeStamp,
    pub confirm: bool,
    pub roles: Vec<String>,
}

impl ListKey for Membership {
    fn list_key() -> &'static str {
        "memberships"
    }
}

impl Membership {
    pub async fn update_role(
        &mut self,
        client: &AppWriteClient,
        roles: Vec<String>,
    ) -> Result<Membership, crate::error::Error> {
        let new =
            TeamsService::update_membership_roles(client, &self.team_id, &self.id, roles).await?;
        *self = new.clone();
        Ok(new)
    }

    pub async fn update_status(
        &self,
        client: &AppWriteClient,
        secret: String,
    ) -> Result<Membership, crate::error::Error> {
        TeamsService::update_membership_status(
            client,
            &self.team_id,
            &self.id,
            &self.user_id,
            secret,
        )
        .await
    }

    pub async fn delete(&self, client: &AppWriteClient) -> Result<(), crate::error::Error> {
        TeamsService::delete_membership(client, &self.team_id, &self.id).await
    }
}
