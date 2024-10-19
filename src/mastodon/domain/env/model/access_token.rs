use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    value: String,
}

impl AccessToken {
    pub fn new(value: String) -> Self {
        AccessToken { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
