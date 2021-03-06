use crate::client::Client;
use crate::prelude::{Repository, Result};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Comment {
    endpoint: String,
    client: Client,
}

impl Comment {
    pub fn new(client: Client) -> Comment {
        let endpoint = format!("{}/{}", client.api_url, "comments");
        Comment { endpoint, client }
    }
    pub fn with_repo(self, _repo: Repository) -> Result<Self> {
        Ok(self)
    }
    pub async fn get(self, _comment: &str) -> Result<Self> {
        Ok(self)
    }
    pub async fn list(self) -> Result<Vec<Self>> {
        Ok(vec![self])
    }
    pub async fn delete(self, _comment: &str) -> Result<()> {
        Ok(())
    }
    pub async fn create(self, _comment: &str) -> Result<()> {
        Ok(())
    }
    pub async fn update(self, _pullrequest: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
