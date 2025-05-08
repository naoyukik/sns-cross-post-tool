use crate::bluesky::domain::message::model::post_message::PostMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PostActionsDto {
    message: String,
}

impl From<&PostMessage> for PostActionsDto {
    fn from(post_message: &PostMessage) -> Self {
        Self {
            message: post_message.get_value().to_string(),
        }
    }
}
