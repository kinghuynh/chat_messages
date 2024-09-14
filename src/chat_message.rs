use crate::prelude::*;
use derive_base_message::BaseMessage;

#[derive(BaseMessage, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    role: String,
    base: BaseMessageFields,
}
