use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginCredentialDto {
    pub identifier: String,
    pub password: String,
}
