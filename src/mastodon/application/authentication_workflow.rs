use crate::mastodon::application::dto::config_dto::ConfigDto;
use crate::mastodon::domain::env::env_repository::EnvRepository;
use crate::mastodon::infrastructure::env_repository_impl::EnvRepositoryImpl;

pub fn login() -> ConfigDto {
    let result = EnvRepositoryImpl::get_login_credential("./.env".to_string());
    ConfigDto::from(&result)
}

#[cfg(test)]
mod tests {}
