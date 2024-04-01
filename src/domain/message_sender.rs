use super::message::Message;

pub trait MessageSender {
    fn send_message(&self, message: Message);
    fn login(&self, username: &str, password: &str) -> bool;
}
