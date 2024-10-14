use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedDto {
    #[serde(rename = "$type")]
    _type: String,
    external: EmbedExternalDto,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedExternalDto {
    uri: String,
    thumb: EmbedExternalThumbDto,
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedExternalThumbDto {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(rename = "ref")]
    r#ref: EmbedExternalThumbRefDto,
    #[serde(rename = "mimeType")]
    mime_type: String,
    size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedExternalThumbRefDto {
    #[serde(rename = "$link")]
    _link: String,
}
