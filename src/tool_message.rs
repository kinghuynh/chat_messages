use crate::prelude::*;
use derive_base_message::BaseMessage;

#[derive(Debug, PartialEq)]
pub enum ToolStatus {
    Success,
    Error,
}

#[derive(BaseMessage)]
pub struct ToolMessage {
    pub tool_call_id: String,
    pub artifact: Option<String>,
    pub status: ToolStatus,
    pub base: BaseMessageFields,
}
