use crate::prelude::*;
use derive_base_message::BaseMessage;

#[derive(BaseMessage)]
pub struct ChatMessage {
    role: String,
    base: BaseMessageFields,
}

impl ChatMessage {
    pub fn role(&self) -> &str {
        &self.role
    }
}
