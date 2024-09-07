use std::collections::HashMap;

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

pub trait BaseMessage {
    fn content(&self) -> &str;
    fn message_type(&self) -> MessageType;
}
