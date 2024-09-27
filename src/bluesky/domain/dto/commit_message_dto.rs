use serde::{Deserialize, Serialize};
use crate::bluesky::domain::dto::facet::Facet;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessageDto {
    repo: String,
    collection: String,
    record: Record,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    facets: Vec<Facet>,
    #[serde(rename = "$type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<website_card_embeds::Embed>,
}
