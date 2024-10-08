use crate::bluesky::application::dto::access_token_dto::AccessToken;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::domain::message::message_service::set_post_message;
use crate::bluesky::infrastructure::message_repository_impl::MessageRepositoryImpl;

pub fn send_message(access_token: &AccessToken) -> Result<bool, curl::Error> {
    let post_data = set_post_message(access_token);
    MessageRepositoryImpl::send(access_token, &post_data)
}
