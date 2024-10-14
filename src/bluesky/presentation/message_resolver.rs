use crate::bluesky::application::authentication_workflow::login;
use crate::bluesky::application::send_message_workflow::send_message;

pub fn post() -> Result<bool, curl::Error> {
    let access_token = login();
    let result = send_message(&access_token.value)?;
    Ok(result)
}
