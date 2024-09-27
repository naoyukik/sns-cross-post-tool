use crate::bluesky::domain::dto::login_credential_dto::LoginCredentialDto;

pub trait EnvRepository {
    fn get_login_credential(env_path: String) -> LoginCredentialDto;
}
