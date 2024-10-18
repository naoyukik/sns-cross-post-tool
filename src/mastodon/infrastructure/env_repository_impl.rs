use std::env;
use std::path::Path;
use crate::mastodon::domain::env::env_repository::EnvRepository;
use crate::mastodon::domain::env::model::access_token::AccessToken;
use crate::mastodon::domain::env::model::config::Config;

pub struct EnvRepositoryImpl {}

impl EnvRepository for EnvRepositoryImpl {
    fn get_login_credential(env_file_path: String) -> Config {
        let env_path = Path::new(&env_file_path);
        dotenvy::from_path(env_path).expect("Failed to load .env file");
        let domain =
            env::var("MASTODON_DOMAIN").expect("Please set the MASTODON_DOMAIN environment variable");
        let password = env::var("MASTODON_APP_PASSWORD")
            .expect("Please set the MASTODON_APP_PASSWORD environment variable");
        let account =
            env::var("MASTODON_ACCOUNT").expect("Please set the MASTODON_ACCOUNT environment variable");

        let access_token = AccessToken::new(password);

        Config::new(
            domain,
            access_token,
            account,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::mastodon::domain::env::env_repository::EnvRepository;
    use crate::mastodon::infrastructure::env_repository_impl::EnvRepositoryImpl;

    #[test]
    fn can_login_credentials_new() {
        let env_path = "./tests/resources/.env".to_string();
        let credentials = EnvRepositoryImpl::get_login_credential(env_path);
    }
}
