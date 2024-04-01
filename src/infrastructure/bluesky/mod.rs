pub struct BlueSky;

use crate::domain::message::Message;
pub use crate::domain::message_sender::MessageSender;

impl MessageSender for BlueSky {
    fn send_message(&self, message: Message) {
        println!("BlueSky: {}", message.get_content());
    }

    fn login(&self, username: &str, password: &str) -> bool {
        true
    }
}
