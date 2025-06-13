use crate::mastodon::domain::message::model::commit_message::CommitMessage;
use crate::shared::domain::message::model::message_template::MessageTemplate;

pub fn set_post_message(message: &MessageTemplate) -> CommitMessage {
    let content_with_fixed_hashtags =
        format!("{} {}", message.content, message.fixed_hashtags.mastodon);
    CommitMessage::new(&content_with_fixed_hashtags)
}
