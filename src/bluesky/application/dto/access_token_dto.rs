use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
}
