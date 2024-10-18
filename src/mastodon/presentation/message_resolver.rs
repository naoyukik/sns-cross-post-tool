use crate::mastodon::application::authentication_workflow::login;
use crate::mastodon::application::send_message_workflow::send_message;

pub fn post() -> Result<bool, curl::Error> {
    let config = login();
    let result = send_message(&config)?;
    Ok(result)
}
