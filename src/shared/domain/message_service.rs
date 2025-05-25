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
