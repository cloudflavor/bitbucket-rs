use crate::client::Client;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Repository {
    endpoint: String,
    client: Client,
}

impl Repository {
    pub fn new(client: Client) -> Repository {
        let endpoint = format!("{}/{}", &client.api_url, "repos");
        Repository { endpoint, client }
    }
    pub fn with_project(self, _project: Project) -> Self {
        self
    }
    pub async fn get(self, _repository: &str) -> Result<Self> {
        Ok(self)
    }
    pub fn comment(self) -> Comment {
        Comment::new(self.client)
    }
    pub fn webhook(self) -> WebHook {
        WebHook::new(self.client)
    }
    pub fn pull_request(self) -> PullRequest {
        PullRequest::new(self.client)
    }
    pub async fn create(self, _repository: &str) -> Result<()> {
        Ok(())
    }
    pub async fn delete(self, _repository: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
