use crate::Args;
use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::domain::message::message_service::set_post_message;
use crate::bluesky::infrastructure::message_repository_impl::MessageRepositoryImpl;
use crate::shared::domain::message::model::message_input::MessageInput;
use crate::shared::domain::message_service::{MessageService, MessageServiceImpl};

pub fn send_message(raw_access_token: &String, args: &Args) -> Result<bool, curl::Error> {
    let access_token = AccessToken::new(raw_access_token.to_string());
    let message_from_input = MessageInput::new(args.message());
    let message_from_json =
        MessageServiceImpl::message_from_json_file(args.message_file_path()).unwrap();
    let merged_message = MessageServiceImpl::merge_message(&message_from_json, &message_from_input);
    let post_data = set_post_message(&access_token, &merged_message);
    MessageRepositoryImpl::send(&access_token, &post_data)
}
