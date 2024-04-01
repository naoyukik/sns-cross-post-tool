use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,
    sender: String,
    receivers: Vec<Receivers>,
}

enum Receivers {
    BlueSky,
    Mastodon,
}

impl Message {
    pub fn new(content: String, sender: String, receivers: Vec<Receivers>) -> Self {
        Message {
            content,
            sender,
            receivers,
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}
