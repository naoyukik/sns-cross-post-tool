use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::domain::message::message_service::set_post_message;
use crate::bluesky::infrastructure::message_repository_impl::MessageRepositoryImpl;

pub fn send_message(raw_access_token: &String) -> Result<bool, curl::Error> {
    let access_token = AccessToken::new(raw_access_token.to_string());
    let post_data = set_post_message(&access_token);
    MessageRepositoryImpl::send(&access_token, &post_data)
}
