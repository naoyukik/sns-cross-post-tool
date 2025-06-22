use crate::bluesky::application::authentication_workflow::login;
use crate::bluesky::application::send_message_workflow::send_message;
use crate::shared::domain::message::model::merged_message::MergedMessage;

pub fn post(merged_message: &MergedMessage) -> Result<bool, curl::Error> {
    let access_token = login();
    let result = send_message(&access_token.value, merged_message)?;
    Ok(result)
}
