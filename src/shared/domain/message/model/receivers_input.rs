use crate::shared::domain::message::model::message_template::Receivers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiversInput {
    values: Vec<Receivers>,
}

impl ReceiversInput {
    pub fn new(values: &[Receivers]) -> Self {
        Self {
            values: values.to_vec(),
        }
    }

    pub fn values(&self) -> &[Receivers] {
        &self.values
    }
}
