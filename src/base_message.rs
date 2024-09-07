use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseMessageFields {
    pub content: String,
    pub example: bool,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub additional_kwargs: HashMap<String, String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub response_metadata: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Human,
    AI,
    System,
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
