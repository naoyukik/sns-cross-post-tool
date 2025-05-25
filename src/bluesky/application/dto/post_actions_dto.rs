use crate::shared::domain::message::model::message_input::MessageInput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PostActionsDto {
    message: String,
}

impl From<&MessageInput> for PostActionsDto {
    fn from(post_message: &MessageInput) -> Self {
        Self {
            message: post_message.get_value().to_string(),
        }
    }
}
