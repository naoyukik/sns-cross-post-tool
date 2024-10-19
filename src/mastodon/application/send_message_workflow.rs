use crate::mastodon::application::dto::config_dto::ConfigDto;
use crate::mastodon::domain::env::model::access_token::AccessToken;
use crate::mastodon::domain::env::model::config::Config;
use crate::mastodon::domain::env::model::endpoints::Endpoints;
use crate::mastodon::domain::message::message_repository::MessageRepository;
use crate::mastodon::domain::message::message_service::set_post_message;
use crate::mastodon::infrastructure::message_repository_impl::MessageRepositoryImpl;

pub fn send_message(config_dto: &ConfigDto) -> Result<bool, curl::Error> {
    let endpoints = Endpoints::new();
    let config = Config::new(
        config_dto.domain.to_string(),
        AccessToken::new(config_dto.access_token.value.to_string()),
        config_dto.account.to_string(),
    );
    let endpoint = config.create_endpoint_url(endpoints.get_statuses());
    let access_token = AccessToken::new(config_dto.access_token.value.to_string());
    let post_data = set_post_message();

    MessageRepositoryImpl::send(&access_token, &endpoint, &post_data)
}
