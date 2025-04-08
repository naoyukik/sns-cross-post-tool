use crate::bluesky::domain::env::env_repository::EnvRepository;
use crate::bluesky::domain::login::model::login_credential::LoginCredential;
use std::env;
use std::path::Path;

pub struct EnvRepositoryImpl {}

impl EnvRepository for EnvRepositoryImpl {
    fn get_login_credential(env_file_path: String) -> LoginCredential {
        let env_path = Path::new(&env_file_path);
        dotenvy::from_path(env_path).expect("Failed to load .env file");

        let identifier = env::var("BLUESKY_LOGIN_NAME")
            .expect("Please set the BLUESKY_LOGIN_NAME environment variable");
        let password = env::var("BLUESKY_APP_PASSWORD")
            .expect("Please set the BLUESKY_APP_PASSWORD environment variable");

        LoginCredential::new(identifier, password)
    }
}

#[cfg(test)]
mod tests {
    use crate::bluesky::domain::env::env_repository::EnvRepository;
    use crate::bluesky::infrastructure::env_repository_impl::EnvRepositoryImpl;

    #[test]
    fn can_login_credentials_new() {
        let env_path = "./tests/resources/.env".to_string();
        let _credentials = EnvRepositoryImpl::get_login_credential(env_path);
    }
}
