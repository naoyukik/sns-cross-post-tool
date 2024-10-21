use crate::bluesky::domain::login::model::login_credential::LoginCredential;

pub trait EnvRepository {
    fn get_login_credential(env_path: String) -> LoginCredential;
}
