use crate::mastodon::domain::env::model::config::Config;

pub trait EnvRepository {
    fn get_login_credential(env_path: String) -> Config;
}
