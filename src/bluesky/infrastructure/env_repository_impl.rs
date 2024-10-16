use crate::bluesky::application::dto::login_credential_dto::LoginCredentialDto;
use crate::bluesky::domain::env::env_repository::EnvRepository;
use std::env;
use std::path::Path;

pub struct EnvRepositoryImpl {}

impl EnvRepository for EnvRepositoryImpl {
    fn get_login_credential(env_file_path: String) -> LoginCredentialDto {
        let env_path = Path::new(&env_file_path);
        let _ = dotenvy::from_path(env_path).expect("Failed to load .env file");

        let identifier = env::var("BLUESKY_LOGIN_NAME")
            .expect("Please set the BLUESKY_LOGIN_NAME environment variable");
        let password = env::var("BLUESKY_APP_PASSWORD")
            .expect("Please set the BLUESKY_APP_PASSWORD environment variable");

        LoginCredentialDto {
            identifier,
            password,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bluesky::domain::env::env_repository::EnvRepository;
    use crate::bluesky::infrastructure::env_repository_impl::EnvRepositoryImpl;

    #[test]
    fn can_login_credentials_new() {
        let env_path = "./tests/resources/.env".to_string();
        let credentials = EnvRepositoryImpl::get_login_credential(env_path);
    }
}
