use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacetDto {
    index: FacetIndex,
    features: Vec<FacetFeatures>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacetIndex {
    #[serde(rename = "byteStart")]
    byte_start: u16,
    #[serde(rename = "byteEnd")]
    byte_end: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FacetFeatures {
    #[serde(rename = "$type")]
    facet_type: String,
    #[serde(flatten)]
    feature_mode: FeatureMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FeatureMode {
    Tag(String),
    Uri(String),
}
