use crate::mastodon::domain::message::model::commit_message::CommitMessage;
use crate::shared::domain::message_service::{MessageService, MessageServiceImpl};

pub fn set_post_message() -> CommitMessage {
    let message = MessageServiceImpl::message_from_json_file("message.json").unwrap();
    let content_with_fixed_hashtags =
        format!("{} {}", message.content, message.fixed_hashtags.mastodon);
    CommitMessage::new(&content_with_fixed_hashtags)
}
