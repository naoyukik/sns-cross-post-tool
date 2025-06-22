use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::domain::message::message_service::set_post_message;
use crate::bluesky::infrastructure::message_repository_impl::MessageRepositoryImpl;
use crate::shared::domain::message::model::merged_message::MergedMessage;

pub fn send_message(
    raw_access_token: &String,
    merged_message: &MergedMessage,
) -> Result<bool, curl::Error> {
    let access_token = AccessToken::new(raw_access_token.to_string());
    let post_data = set_post_message(&access_token, merged_message);
    MessageRepositoryImpl::send(&access_token, &post_data)
}
