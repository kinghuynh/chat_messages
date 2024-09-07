use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum MessageType {
    Human,
    AI,
    System,
}

pub trait BaseMessage {
    fn content(&self) -> &str;
    fn message_type(&self) -> MessageType;
    fn additional_kwargs(&self) -> &HashMap<String, String>;
    fn response_metadata(&self) -> &HashMap<String, String>;
    fn id(&self) -> Option<&str>;
    fn name(&self) -> Option<&str>;
}
