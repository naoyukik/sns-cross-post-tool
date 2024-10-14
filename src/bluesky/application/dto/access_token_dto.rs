use crate::bluesky::domain::login::model::access_token::AccessToken;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenDto {
    pub value: String,
}

impl From<&AccessToken> for AccessTokenDto {
    fn from(access_token: &AccessToken) -> Self {
        AccessTokenDto {
            value: access_token.value().to_string(),
        }
    }
}
