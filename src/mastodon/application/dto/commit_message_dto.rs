use crate::bluesky::application::dto::embed_dto::EmbedDto;
use crate::bluesky::application::dto::facet_dto::FacetDto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessageDto {
    pub repo: String,
    pub collection: String,
    pub record: CommitMessageRecordDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessageRecordDto {
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub facets: Vec<FacetDto>,
    #[serde(rename = "$type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<EmbedDto>,
}
