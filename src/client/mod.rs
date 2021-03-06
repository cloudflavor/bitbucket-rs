//! Client represents a bitbucket server client.
use crate::builder::Builder;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Client {
    pub token: String,
    pub disable_ssl: bool,
    pub api_url: String,
    builder: Builder,
}

impl Client {
    pub fn new(token: String, api_url: String, disable_ssl: bool) -> Self {
        Self {
            token,
            api_url,
            disable_ssl,
        }
    }
    pub fn projects(self) -> Project {
        Project::new(self)
    }
    pub fn repositories(self) -> Repository {
        Repository::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_client() {}
}
