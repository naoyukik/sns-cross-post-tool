use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FixedHashtagInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluesky: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<String>,
}

impl FixedHashtagInput {
    pub fn new(bluesky: Option<&str>, mastodon: Option<&str>) -> Self {
        Self {
            bluesky: bluesky.map(|s| s.to_string()),
            mastodon: mastodon.map(|s| s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_none() {
        let input = FixedHashtagInput::new(None, None);
        assert_eq!(input.bluesky, None);
        assert_eq!(input.mastodon, None);
    }

    #[test]
    fn test_new_with_both() {
        let input = FixedHashtagInput::new(Some("blue"), Some("mast"));
        assert_eq!(input.bluesky, Some("blue".to_string()));
        assert_eq!(input.mastodon, Some("mast".to_string()));
    }

    #[test]
    fn test_new_with_bluesky_only() {
        let input = FixedHashtagInput::new(Some("blue"), None);
        assert_eq!(input.bluesky, Some("blue".to_string()));
        assert_eq!(input.mastodon, None);
    }

    #[test]
    fn test_new_with_mastodon_only() {
        let input = FixedHashtagInput::new(None, Some("mast"));
        assert_eq!(input.bluesky, None);
        assert_eq!(input.mastodon, Some("mast".to_string()));
    }
}
