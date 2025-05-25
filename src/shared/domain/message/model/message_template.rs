use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MessageTemplate {
    pub content: String,
    pub receivers: Vec<Receivers>,
    pub fixed_hashtags: FixedHashtags,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FixedHashtags {
    pub mastodon: String,
    pub bluesky: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Receivers {
    Bluesky,
    Mastodon,
}
