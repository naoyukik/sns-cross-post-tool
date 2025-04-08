use crate::mastodon::domain::env::model::access_token::AccessToken;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    domain: String,
    access_token: AccessToken,
    account: String,
}

impl Config {
    pub fn new(domain: String, access_token: AccessToken, account: String) -> Config {
        Config {
            domain,
            access_token,
            account,
        }
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }

    pub fn get_access_token(&self) -> &AccessToken {
        &self.access_token
    }

    pub fn get_account(&self) -> &str {
        &self.account
    }

    pub fn base_url(&self) -> Url {
        Url::parse(&format!("https://{}", self.get_domain())).expect("Failed Parse URL")
    }

    pub fn create_endpoint_url(&self, path: &str) -> Url {
        assert!(path.starts_with('/'), "Path must start with `/`");
        self.base_url()
            .join(path)
            .expect("Failed create endpoint url")
    }
}
