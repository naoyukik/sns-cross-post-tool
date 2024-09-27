use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginCredential {
    identifier: String,
    password: String,
}

impl LoginCredential {
    pub fn new(identifier: String, password: String) -> LoginCredential {
        LoginCredential {
            identifier,
            password,
        }
    }
}
