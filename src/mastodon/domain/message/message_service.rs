use crate::mastodon::domain::message::model::commit_message::CommitMessage;
use crate::shared::domain::message::model::message_input::MessageInput;
use crate::shared::domain::message_service::{MessageService, MessageServiceImpl};

pub fn set_post_message(message_from_input: &MessageInput) -> CommitMessage {
    let message_from_json = MessageServiceImpl::message_from_json_file("message.json").unwrap();
    let message = MessageServiceImpl::merge_message(&message_from_json, &message_from_input);
    let content_with_fixed_hashtags =
        format!("{} {}", message.content, message.fixed_hashtags.mastodon);
    CommitMessage::new(&content_with_fixed_hashtags)
}
