use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Facet {
    index: FacetIndex,
    features: Vec<FacetFeatures>,
}

impl Facet {
    pub fn create(index: FacetIndex, features: Vec<FacetFeatures>) -> Facet {
        Facet { index, features }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacetIndex {
    #[serde(rename = "byteStart")]
    byte_start: u16,
    #[serde(rename = "byteEnd")]
    byte_end: u16,
}

impl FacetIndex {
    pub fn create(byte_start: &u16, byte_end: &u16) -> FacetIndex {
        FacetIndex {
            byte_start: *byte_start,
            byte_end: *byte_end,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FacetFeatures {
    #[serde(rename = "$type")]
    facet_type: String,
    #[serde(flatten)]
    feature_mode: FeatureMode,
}

impl FacetFeatures {
    pub fn create(facet_type: String, feature_mode: FeatureMode) -> FacetFeatures {
        FacetFeatures {
            facet_type,
            feature_mode,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FeatureMode {
    Tag(String),
    Uri(String),
}
