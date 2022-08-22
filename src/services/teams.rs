use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        membership::{Membership, MembershipId},
        team::{Team, TeamId},
        user::UserId,
        ListResponse,
    },
};

use super::SearchPayload;

pub struct TeamsService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamPayload {
    pub team_id: TeamId,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMembershipPayload {
    pub email: String,
    pub roles: Vec<String>,
    #[serde(rename = "url")]
    pub redirect_url: String,
    pub name: Option<String>,
}

impl TeamsService {
    pub async fn create_team(
        client: &AppWriteClient,
        payload: CreateTeamPayload,
    ) -> Result<ListResponse<Team>, crate::error::Error> {
        let url = "/teams";
        let response = client
            .call(
                Method::POST,
                url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(ListResponse<Team>: response))
    }

    pub async fn list_teams(
        client: &AppWriteClient,
        payload: SearchPayload<TeamId>,
    ) -> Result<ListResponse<Team>, crate::error::Error> {
        let url = "/teams";
        let response = client
            .call(
                Method::GET,
                url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Team>: response))
    }

    pub async fn get_team(
        client: &AppWriteClient,
        team_id: &TeamId,
    ) -> Result<Team, crate::error::Error> {
        let url = format!("/teams/{team_id}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Team: response))
    }

    pub async fn update_name(
        client: &AppWriteClient,
        team_id: &TeamId,
        name: &str,
    ) -> Result<Team, crate::error::Error> {
        let url = format!("/teams/{team_id}");
        let payload = serde_json::json!({ "name": name });
        let response = client
            .call(Method::PUT, &url, RequestData::Json(payload))
            .await?;
        Ok(check_response!(Team: response))
    }

    pub async fn delete_team(
        client: &AppWriteClient,
        team_id: &TeamId,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/teams/{team_id}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }

    pub async fn create_membership(
        client: &AppWriteClient,
        team_id: &TeamId,
        payload: CreateMembershipPayload,
    ) -> Result<Membership, crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships");
        let response = client
            .call(
                Method::POST,
                &url,
                RequestData::Json(serde_json::to_value(payload)?),
            )
            .await?;
        Ok(check_response!(Membership: response))
    }

    pub async fn get_memberships(
        client: &AppWriteClient,
        team_id: &TeamId,
        payload: SearchPayload<MembershipId>,
    ) -> Result<ListResponse<Membership>, crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships", team_id = team_id);
        let response = client
            .call(
                Method::GET,
                &url,
                RequestData::Params(payload.serialize_params()),
            )
            .await?;
        Ok(check_response!(ListResponse<Membership>: response))
    }

    pub async fn get_membership(
        client: &AppWriteClient,
        team_id: &TeamId,
        membership_id: &MembershipId,
    ) -> Result<Membership, crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships/{membership_id}");
        let response = client.call(Method::GET, &url, RequestData::None).await?;
        Ok(check_response!(Membership: response))
    }

    pub async fn update_membership_roles(
        client: &AppWriteClient,
        team_id: &TeamId,
        membership_id: &MembershipId,
        roles: Vec<String>,
    ) -> Result<Membership, crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships/{membership_id}");
        let payload = serde_json::json!({ "roles": roles });
        let response = client
            .call(Method::PATCH, &url, RequestData::Json(payload))
            .await?;
        Ok(check_response!(Membership: response))
    }

    pub async fn update_membership_status(
        client: &AppWriteClient,
        team_id: &TeamId,
        membership_id: &MembershipId,
        user_id: &UserId,
        status: bool,
    ) -> Result<Membership, crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships/{membership_id}/status");
        let payload = serde_json::json!({
            "user_id": user_id,
            "status": status
        });
        let response = client
            .call(Method::PATCH, &url, RequestData::Json(payload))
            .await?;
        Ok(check_response!(Membership: response))
    }

    pub async fn delete_membership(
        client: &AppWriteClient,
        team_id: &TeamId,
        membership_id: &MembershipId,
    ) -> Result<(), crate::error::Error> {
        let url = format!("/teams/{team_id}/memberships/{membership_id}");
        let response = client.call(Method::DELETE, &url, RequestData::None).await?;
        Ok(check_response!(response))
    }
}
