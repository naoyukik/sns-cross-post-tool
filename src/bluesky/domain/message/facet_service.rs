use crate::bluesky::domain::message::model::facet::{
    Facet, FacetFeatures, FacetIndex, FeatureMode,
};
use crate::util;
use regex::{Match, Regex};

pub fn create_facets(message: &str) -> Vec<Facet> {
    let tags_facets = create_tags_facets(message);
    let links_facets = create_links_facets(message);
    let mut facets = Vec::new();
    facets.extend(tags_facets);
    facets.extend(links_facets);
    facets
}

fn create_tags_facets(message_content: &str) -> Vec<Facet> {
    util::find_hash_tags(message_content)
        .iter()
        .filter_map(|capture| {
            capture
                .get(2)
                .map(|cap| to_facet_dto(cap, FeatureMode::Tag("Tag".to_string())))
        })
        .collect()
}

fn create_links_facets(message_content: &str) -> Vec<Facet> {
    util::find_link_string(message_content)
        .iter()
        .filter_map(|capture| {
            capture
                .get(2)
                .map(|cap| to_facet_dto(cap, FeatureMode::Uri("Uri".to_string())))
        })
        .collect()
}

fn to_facet_dto(capture_group: Match, feature_mode: FeatureMode) -> Facet {
    let start = capture_group.start() as u16;
    let end = capture_group.end() as u16;
    let match_str = capture_group.as_str();
    Facet::create(
        to_facet_index(&start, &end),
        to_facet_feature(feature_mode, match_str),
    )
}

fn to_facet_feature(feature_mode: FeatureMode, match_str: &str) -> Vec<FacetFeatures> {
    match feature_mode {
        FeatureMode::Tag(_) => tags_to_facet_features(match_str),
        FeatureMode::Uri(_) => links_to_facet_features(match_str),
    }
}

fn to_facet_index(start: &u16, end: &u16) -> FacetIndex {
    FacetIndex::create(start, end)
}

fn tags_to_facet_features(tag: &str) -> Vec<FacetFeatures> {
    let re = Regex::new(r"^#").unwrap();
    vec![FacetFeatures::create(
        "app.bsky.richtext.facet#tag".to_string(),
        FeatureMode::Tag(re.replace(tag.trim(), "").to_string()),
    )]
}

fn links_to_facet_features(tag: &str) -> Vec<FacetFeatures> {
    vec![FacetFeatures::create(
        "app.bsky.richtext.facet#link".to_string(),
        FeatureMode::Uri(tag.trim().to_string()),
    )]
}
