use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MergedMessage {
    pub content: String,
    pub receivers: Vec<Receivers>,
    pub fixed_hashtags: FixedHashtags,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FixedHashtags {
    pub mastodon: String,
    pub bluesky: String,
}

impl FixedHashtags {
    pub fn new(bluesky: &str, mastodon: &str) -> Self {
        Self {
            mastodon: mastodon.to_string(),
            bluesky: bluesky.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum Receivers {
    Bluesky,
    Mastodon,
}
