use crate::shared::domain::message::model::message_input::MessageInput;
use crate::shared::domain::message::model::message_template::MessageTemplate;
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;

pub trait MessageService {
    fn message_from_json_file(file_path: &str) -> Result<MessageTemplate, Error>;
    fn merge_message(
        message_from_json: &MessageTemplate,
        message_from_args: &MessageInput,
    ) -> MessageTemplate;
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
    ) -> MessageTemplate {
        let message = if !message_from_args.get_value().trim().is_empty() {
            message_from_args.get_value()
        } else {
            message_from_json.content.as_str()
        };

        MessageTemplate {
            content: message.to_string(),
            receivers: message_from_json.receivers.clone(),
            fixed_hashtags: message_from_json.fixed_hashtags.clone(),
        }
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
        let message_from_json = MessageTemplate {
            content: "Default message.".to_string(),
            receivers: vec![Receivers::Bluesky],
            fixed_hashtags: FixedHashtags {
                bluesky: "#default_bluesky".to_string(),
                mastodon: "#default_mastodon".to_string(),
            },
        };

        let message_from_args = MessageInput::new("Override message.");

        let merged_message =
            MessageServiceImpl::merge_message(&message_from_json, &message_from_args);

        assert_eq!(merged_message.content, "Override message.");
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

        let merged_message =
            MessageServiceImpl::merge_message(&message_from_json, &message_from_args);

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

        let merged_message =
            MessageServiceImpl::merge_message(&message_from_json, &message_from_args);

        assert_eq!(merged_message.content, "Default message.");
    }
}
