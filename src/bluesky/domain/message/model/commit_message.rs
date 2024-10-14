use crate::bluesky::domain::message::model::facet::Facet;
use crate::bluesky::domain::website_card_embeds::model::embed::Embed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessage {
    pub repo: String,
    pub collection: String,
    pub record: CommitMessageRecord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessageRecord {
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub facets: Vec<Facet>,
    #[serde(rename = "$type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
}
