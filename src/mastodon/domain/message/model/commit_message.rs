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
}
