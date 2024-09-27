use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Facet {
    index: FacetIndex,
    features: Vec<FacetFeatures>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FacetIndex {
    #[serde(rename = "byteStart")]
    byte_start: u16,
    #[serde(rename = "byteEnd")]
    byte_end: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct FacetFeatures {
    #[serde(rename = "$type")]
    facet_type: String,
    #[serde(flatten)]
    feature_mode: crate::bluesky::domain::dto::commit_message_dto::FeatureMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum FeatureMode {
    Tag(String),
    Uri(String),
}
