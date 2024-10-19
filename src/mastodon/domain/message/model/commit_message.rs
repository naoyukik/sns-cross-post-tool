use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessage {
    status: String,
}

impl CommitMessage {
    pub fn new(status: &str) -> CommitMessage {
        CommitMessage {
            status: status.to_string(),
        }
    }

    pub fn get_status(&self) -> String {
        self.status.to_string()
    }
}
