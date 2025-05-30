use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageInput {
    value: String,
}

impl MessageInput {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}
