use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct BaseMessageFields {
    pub content: String,
    pub example: bool,
    pub additional_kwargs: HashMap<String, String>,
    pub response_metadata: HashMap<String, String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MessageType {
    Human,
    AI,
    System,
}

#[derive(Debug)]
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
            "Human" => Ok(MessageType::Human),
            "AI" => Ok(MessageType::AI),
            "System" => Ok(MessageType::System),
            _ => Err(InvalidMessageTypeError),
        }
    }
}

pub trait BaseMessage {
    fn content(&self) -> &str;
    fn message_type(&self) -> MessageType;
}
