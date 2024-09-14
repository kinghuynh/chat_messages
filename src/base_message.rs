use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use crate::MessageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseMessageFields {
    pub content: String,
    pub example: bool,
    pub message_type: MessageType,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub additional_kwargs: HashMap<String, String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub response_metadata: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

pub trait BaseMessage {
    fn content(&self) -> &str;
    fn message_type(&self) -> &MessageType;
    fn role(&self) -> &str;
    fn name(&self) -> Option<&str>;
    fn is_example(&self) -> bool;
    fn additional_kwargs(&self) -> &std::collections::HashMap<String, String>;
    fn response_metadata(&self) -> &std::collections::HashMap<String, String>;
    fn id(&self) -> Option<&str>;
}

impl Debug for dyn BaseMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BaseMessage {{ content: \"{}\", message_type: {:?} }}",
            self.content(),
            self.message_type()
        )
    }
}
