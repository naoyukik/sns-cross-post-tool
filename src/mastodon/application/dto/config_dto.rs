use serde::{Deserialize, Serialize};
use crate::mastodon::application::dto::access_token_dto::AccessTokenDto;
use crate::mastodon::domain::env::model::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDto {
    pub domain: String,
    pub access_token: AccessTokenDto,
    pub account: String,
}

impl From<&Config> for ConfigDto {
    fn from(config: &Config) -> Self {
        ConfigDto {
            domain: config.get_domain().to_string(),
            access_token: AccessTokenDto::from(config.get_access_token()),
            account: config.get_account().to_string(),
        }
    }
}
