use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum MessageType {
    Ai,
    Chat,
    Human,
    System,
    Tool,
}

impl MessageType {
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::Ai => "ai",
            MessageType::Chat => "chat",
            MessageType::Human => "human",
            MessageType::System => "system",
            MessageType::Tool => "tool",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidMessageTypeError;

impl fmt::Display for InvalidMessageTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid message type")
    }
}

impl TryFrom<&str> for MessageType {
    type Error = InvalidMessageTypeError;

    fn try_from(s: &str) -> Result<MessageType, InvalidMessageTypeError> {
        match s {
            "human" | "Human" | "HumanMessage" => Ok(MessageType::Human),
            "ai" | "Ai" | "AiMessage" => Ok(MessageType::Ai),
            "system" | "System" | "SystemMessage" => Ok(MessageType::System),
            "chat" | "Chat" | "ChatMessage" => Ok(MessageType::Chat),
            "tool" | "Tool" | "ToolMessage" => Ok(MessageType::Tool),
            _ => Err(InvalidMessageTypeError),
        }
    }
}
