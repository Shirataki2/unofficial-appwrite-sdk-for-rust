use crate::{client::AppWriteClient, error::Error};
pub struct AccountsService;

impl AccountsService {
    pub fn create_oauth2_session(client: &AppWriteClient, provider: &str) -> Result<String, Error> {
        let base_url = client.get_host_url();
        let project_id = client.get_project_id();
        let url = format!("{base_url}/account/sessions/oauth2/{provider}?project={project_id}&success=http://localhost:2222");
        Ok(url)
    }
}
