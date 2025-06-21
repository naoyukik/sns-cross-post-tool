use crate::shared::domain::message::model::fixed_hashtag_input::FixedHashtagInput;
use crate::shared::domain::message::model::merged_message::{FixedHashtags, MergedMessage};
use crate::shared::domain::message::model::message_input::MessageInput;
use crate::shared::domain::message::model::message_template::{MessageTemplate, Receivers};
use regex::{Captures, Regex};
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;

pub trait MessageService {
    fn message_from_json_file(file_path: &str) -> Result<MessageTemplate, Error>;
    fn merge_message(
        message_from_json: &MessageTemplate,
        message_from_args: &MessageInput,
        fixed_hashtag_from_args: &FixedHashtagInput,
    ) -> MergedMessage;
    fn merge_receivers(
        message_from_json: &MessageTemplate,
        receivers_from_input: Option<&[Receivers]>,
    ) -> Vec<Receivers>;
    fn find_hash_tags(haystack: &str) -> Vec<Captures>;
    fn find_link_string(haystack: &str) -> Vec<Captures>;
    fn unescape_newlines(input: &str) -> String; // 新しく追加
}

pub struct MessageServiceImpl;

impl MessageService for MessageServiceImpl {
    fn message_from_json_file(file_path: &str) -> Result<MessageTemplate, Error> {
        let file = File::open(file_path).expect("File not found");
        let reader = BufReader::new(file);

        let json_object: MessageTemplate = serde_json::from_reader(reader)?;

        Ok(json_object)
    }

    fn merge_message(
        message_from_json: &MessageTemplate,
        message_from_args: &MessageInput,
        fixed_hashtag_from_args: &FixedHashtagInput,
    ) -> MergedMessage {
        MergedMessage {
            content: if !message_from_args.get_value().trim().is_empty() {
                Self::unescape_newlines(message_from_args.get_value())
            } else {
                message_from_json.content.clone()
            },
            receivers: message_from_json
                .receivers
                .clone()
                .into_iter()
                .map(|r| r.into())
                .collect(),
            fixed_hashtags: FixedHashtags {
                bluesky: if fixed_hashtag_from_args.bluesky.clone().is_some() {
                    fixed_hashtag_from_args.bluesky.clone().unwrap().to_string()
                } else {
                    message_from_json.fixed_hashtags.bluesky.clone()
                },
                mastodon: if fixed_hashtag_from_args.mastodon.clone().is_some() {
                    fixed_hashtag_from_args
                        .mastodon
                        .clone()
                        .unwrap()
                        .to_string()
                } else {
                    message_from_json.fixed_hashtags.mastodon.clone()
                },
            },
        }
    }

    fn merge_receivers(
        message_from_json: &MessageTemplate,
        receivers_from_input: Option<&[Receivers]>,
    ) -> Vec<Receivers> {
        match receivers_from_input {
            Some(receivers) => receivers.to_vec(),
            None => message_from_json.receivers.clone(),
        }
    }

    fn find_hash_tags(haystack: &str) -> Vec<Captures> {
        let pattern = r"(^|\s)(#\w*)";
        let regex_pattern = Regex::new(pattern).unwrap();
        regex_pattern.captures_iter(haystack).collect()
    }

    fn find_link_string(haystack: &str) -> Vec<Captures> {
        let pattern = r"(^|\s)(https?://(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))";
        let regex = Regex::new(pattern).unwrap();
        regex.captures_iter(haystack).collect()
    }

    fn unescape_newlines(input: &str) -> String {
        input
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::domain::message::model::message_input::MessageInput;
    use crate::shared::domain::message::model::message_template::FixedHashtags;
    use crate::shared::domain::message::model::message_template::{MessageTemplate, Receivers};

    #[test]
    fn test_message_from_json_file() {
        let result = MessageServiceImpl::message_from_json_file(
            "tests/resources/shared/domain/test_message_from_json_file.json",
        );
        assert!(result.is_ok());

        let message_template = result.unwrap();
        assert_eq!(message_template.content, "Test message content.");
        assert_eq!(
            message_template.receivers,
            vec![Receivers::Bluesky, Receivers::Mastodon]
        );
        assert_eq!(message_template.fixed_hashtags.bluesky, "#test_bluesky");
        assert_eq!(message_template.fixed_hashtags.mastodon, "#test_mastodon");
    }

    #[test]
    fn test_merge_message_with_input() {
        let message_from_json = MessageServiceImpl::message_from_json_file(
            "tests/resources/shared/domain/test_message_from_json_file.json",
        )
        .unwrap();

        let message_from_args = MessageInput::new("Override message.\n\n#override_hashtag");

        let fixed_hashtag_from_args =
            FixedHashtagInput::new(Some("#test_bluesky"), Some("#test_mastodon"));
        let merged_message = MessageServiceImpl::merge_message(
            &message_from_json,
            &message_from_args,
            &fixed_hashtag_from_args,
        );

        assert_eq!(
            merged_message.content,
            "Override message.\n\n#override_hashtag"
        );
    }

    #[test]
    fn test_merge_message_without_input() {
        let message_from_json = MessageTemplate {
            content: "Default message.".to_string(),
            receivers: vec![Receivers::Mastodon],
            fixed_hashtags: FixedHashtags {
                bluesky: "#default_bluesky".to_string(),
                mastodon: "#default_mastodon".to_string(),
            },
        };

        let message_from_args = MessageInput::new("");

        let fixed_hashtag_from_args =
            FixedHashtagInput::new(Some("#default_bluesky"), Some("#default_mastodon"));
        let merged_message = MessageServiceImpl::merge_message(
            &message_from_json,
            &message_from_args,
            &fixed_hashtag_from_args,
        );

        assert_eq!(merged_message.content, "Default message.");
    }

    #[test]
    fn test_merge_message_with_whitespace_input() {
        let message_from_json = MessageTemplate {
            content: "Default message.".to_string(),
            receivers: vec![Receivers::Mastodon],
            fixed_hashtags: FixedHashtags {
                bluesky: "#default_bluesky".to_string(),
                mastodon: "#default_mastodon".to_string(),
            },
        };

        let message_from_args = MessageInput::new("   ");

        let fixed_hashtag_from_args =
            FixedHashtagInput::new(Some("#default_bluesky"), Some("#default_mastodon"));
        let merged_message = MessageServiceImpl::merge_message(
            &message_from_json,
            &message_from_args,
            &fixed_hashtag_from_args,
        );

        assert_eq!(merged_message.content, "Default message.");
    }

    #[test]
    fn can_find_hash_tags() {
        let hashes = [
            ["#test", "29", "34"],
            ["#日本語のテスト", "35", "57"],
            ["#1111", "58", "63"],
        ];

        let matches =
            MessageServiceImpl::find_hash_tags("ハッシュ投稿テスト\n\n#test #日本語のテスト #1111");
        for (hash_index, caps) in matches.iter().enumerate() {
            if let Some(captures) = caps.get(2) {
                println!("{}", hash_index);
                match hashes.get(hash_index) {
                    Some(hash) => {
                        assert_eq!(&captures.as_str(), hash.first().unwrap());
                        assert_eq!(
                            captures.start().to_string(),
                            hash.get(1).unwrap().to_string()
                        );
                        assert_eq!(captures.end().to_string(), hash.get(2).unwrap().to_string());
                    }
                    None => panic!("No hash found at index 0"),
                }
            }
        }
    }

    #[test]
    fn test_find_link_string_single_link() {
        let text =
            "This is the text of the test.Link: https://example.com/path?query=value#fragment";
        let matches = MessageServiceImpl::find_link_string(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0].get(2).unwrap().as_str(),
            "https://example.com/path?query=value#fragment"
        );
    }

    #[test]
    fn test_find_link_string_multiple_links() {
        let text =
            "The first link is https://example.com and the second is http://www.example.net/";
        let matches = MessageServiceImpl::find_link_string(text);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].get(2).unwrap().as_str(), "https://example.com");
        assert_eq!(
            matches[1].get(2).unwrap().as_str(),
            "http://www.example.net/"
        );
    }

    #[test]
    fn test_find_link_string_no_link() {
        let text = "This text contains no links.";
        let matches = MessageServiceImpl::find_link_string(text);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_link_string_link_at_start() {
        let text = "https://example.com There is a link.";
        let matches = MessageServiceImpl::find_link_string(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].get(2).unwrap().as_str(), "https://example.com");
    }

    #[test]
    fn test_find_link_string_japanese_text_with_link() {
        let text = "日本語のテキストにリンク https://jp.example.com/ です。";
        let matches = MessageServiceImpl::find_link_string(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0].get(2).unwrap().as_str(),
            "https://jp.example.com/"
        );
    }
}
