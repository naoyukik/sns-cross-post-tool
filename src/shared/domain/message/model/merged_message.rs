use crate::shared::domain::message::model::message_template;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MergedMessage {
    pub content: String,
    pub receivers: Vec<Receivers>,
    pub fixed_hashtags: FixedHashtags,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum Receivers {
    Bluesky,
    Mastodon,
}

impl From<message_template::Receivers> for Receivers {
    fn from(receiver: message_template::Receivers) -> Self {
        match receiver {
            message_template::Receivers::Bluesky => Self::Bluesky,
            message_template::Receivers::Mastodon => Self::Mastodon,
        }
    }
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
