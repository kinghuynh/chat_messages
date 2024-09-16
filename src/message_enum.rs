use std::collections::HashMap;
use std::fmt;

use crate::{AiMessage, BaseMessageFields, HumanMessage, SystemMessage};
use crate::{BaseMessage, MessageType};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")] // Automatically uses "role" field for dispatch
pub enum MessageEnum {
    Ai(AiMessage),
    Human(HumanMessage),
    System(SystemMessage),
}

impl BaseMessage for MessageEnum {
    fn content(&self) -> &str {
        match self {
            MessageEnum::Ai(message) => message.content(),
            MessageEnum::Human(message) => message.content(),
            MessageEnum::System(message) => message.content(),
        }
    }

    fn message_type(&self) -> &MessageType {
        match self {
            MessageEnum::Ai(message) => message.message_type(),
            MessageEnum::Human(message) => message.message_type(),
            MessageEnum::System(message) => message.message_type(),
        }
    }

    fn role(&self) -> &str {
        match self {
            MessageEnum::Ai(_) => "ai",
            MessageEnum::Human(_) => "human",
            MessageEnum::System(_) => "system",
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            MessageEnum::Ai(message) => message.name(),
            MessageEnum::Human(message) => message.name(),
            MessageEnum::System(message) => message.name(),
        }
    }

    fn is_example(&self) -> bool {
        match self {
            MessageEnum::Ai(message) => message.is_example(),
            MessageEnum::Human(message) => message.is_example(),
            MessageEnum::System(message) => message.is_example(),
        }
    }

    fn additional_kwargs(&self) -> &HashMap<String, String> {
        match self {
            MessageEnum::Ai(message) => message.additional_kwargs(),
            MessageEnum::Human(message) => message.additional_kwargs(),
            MessageEnum::System(message) => message.additional_kwargs(),
        }
    }

    fn response_metadata(&self) -> &HashMap<String, String> {
        match self {
            MessageEnum::Ai(message) => message.response_metadata(),
            MessageEnum::Human(message) => message.response_metadata(),
            MessageEnum::System(message) => message.response_metadata(),
        }
    }

    fn id(&self) -> Option<&str> {
        match self {
            MessageEnum::Ai(message) => message.id(),
            MessageEnum::Human(message) => message.id(),
            MessageEnum::System(message) => message.id(),
        }
    }
}

impl fmt::Debug for MessageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageEnum::Ai(message) => write!(f, "AiMessage({:?})", message),
            MessageEnum::Human(message) => write!(f, "HumanMessage({:?})", message),
            MessageEnum::System(message) => write!(f, "SystemMessage({:?})", message),
        }
    }
}

impl From<AiMessage> for MessageEnum {
    fn from(message: AiMessage) -> Self {
        MessageEnum::Ai(message)
    }
}

impl From<SystemMessage> for MessageEnum {
    fn from(message: SystemMessage) -> Self {
        MessageEnum::System(message)
    }
}

impl From<HumanMessage> for MessageEnum {
    fn from(message: HumanMessage) -> Self {
        MessageEnum::Human(message)
    }
}

impl<'de> Deserialize<'de> for MessageEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempMessage {
            role: String,
            content: String,
            #[serde(default)]
            example: bool,
            #[serde(default)]
            additional_kwargs: HashMap<String, String>,
            #[serde(default)]
            response_metadata: HashMap<String, String>,
            #[serde(default)]
            id: Option<String>,
            #[serde(default)]
            name: Option<String>,
        }

        let temp = TempMessage::deserialize(deserializer)?;
        let message_type =
            MessageType::try_from(temp.role.as_str()).map_err(serde::de::Error::custom)?;

        let base = BaseMessageFields {
            content: temp.content,
            example: temp.example,
            additional_kwargs: temp.additional_kwargs,
            response_metadata: temp.response_metadata,
            id: temp.id,
            name: temp.name,
            message_type,
        };

        match message_type {
            MessageType::Ai => Ok(MessageEnum::Ai(AiMessage { base })),
            MessageType::Human => Ok(MessageEnum::Human(HumanMessage { base })),
            MessageType::System => Ok(MessageEnum::System(SystemMessage { base })),
            _ => Err(serde::de::Error::custom("Unsupported message type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BaseMessageFields;

    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_message_enum_serialization_ai_message() {
        let ai_message = AiMessage {
            base: BaseMessageFields {
                content: "Hello from AI.".to_string(),
                example: false,
                message_type: MessageType::Ai,
                additional_kwargs: HashMap::new(),
                response_metadata: HashMap::new(),
                id: None,
                name: None,
            },
        };

        let message_enum = MessageEnum::Ai(ai_message);

        let expected_json = json!({
            "role": "ai",
            "content": "Hello from AI.",
            "example": false,
            "message_type": "Ai"
        });

        let serialized = serde_json::to_string(&message_enum).unwrap();
        let actual_json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_message_enum_serialization_human_message() {
        let human_message = HumanMessage {
            base: BaseMessageFields {
                content: "Hello from Human.".to_string(),
                example: false,
                message_type: MessageType::Human,
                additional_kwargs: HashMap::new(),
                response_metadata: HashMap::new(),
                id: None,
                name: None,
            },
        };

        let message_enum = MessageEnum::Human(human_message);

        let expected_json = json!({
            "role": "human",
            "content": "Hello from Human.",
            "example": false,
            "message_type": "Human"
        });

        let serialized = serde_json::to_string(&message_enum).unwrap();
        let actual_json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_message_enum_serialization_system_message() {
        let system_message = SystemMessage {
            base: BaseMessageFields {
                content: "This is a system message.".to_string(),
                example: false,
                message_type: MessageType::System,
                additional_kwargs: HashMap::new(),
                response_metadata: HashMap::new(),
                id: None,
                name: None,
            },
        };

        let message_enum = MessageEnum::System(system_message);

        let expected_json = json!({
            "role": "system",
            "content": "This is a system message.",
            "example": false,
            "message_type": "System"
        });

        let serialized = serde_json::to_string(&message_enum).unwrap();
        let actual_json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_message_enum_deserialization_ai_message() {
        let json_data = json!({
            "role": "ai",
            "content": "Hello from AI.",
            "example": false,
            "message_type": "Ai"
        })
        .to_string();

        let message_enum: MessageEnum = serde_json::from_str(&json_data).unwrap();
        match message_enum {
            MessageEnum::Ai(ai_message) => {
                assert_eq!(ai_message.content(), "Hello from AI.");
                assert_eq!(ai_message.message_type(), &MessageType::Ai);
            }
            _ => panic!("Expected AiMessage"),
        }
    }

    #[test]
    fn test_message_enum_deserialization_human_message() {
        let json_data = json!({
            "role": "human",
            "content": "Hello from Human.",
            "example": false,
            "message_type": "Human"
        })
        .to_string();

        let message_enum: MessageEnum = serde_json::from_str(&json_data).unwrap();
        match message_enum {
            MessageEnum::Human(human_message) => {
                assert_eq!(human_message.content(), "Hello from Human.");
                assert_eq!(human_message.message_type(), &MessageType::Human);
            }
            _ => panic!("Expected HumanMessage"),
        }
    }

    #[test]
    fn test_message_enum_deserialization_system_message() {
        let json_data = json!({
            "role": "system",
            "content": "This is a system message.",
            "example": false,
            "message_type": "System"
        })
        .to_string();

        let message_enum: MessageEnum = serde_json::from_str(&json_data).unwrap();
        match message_enum {
            MessageEnum::System(system_message) => {
                assert_eq!(system_message.content(), "This is a system message.");
                assert_eq!(system_message.message_type(), &MessageType::System);
            }
            _ => panic!("Expected SystemMessage"),
        }
    }

    #[test]
    fn test_message_enum_equality() {
        let ai_message1 = AiMessage::new("Hello from AI.");
        let ai_message2 = AiMessage::new("Hello from AI.");
        let human_message = HumanMessage::new("Hello from Human.");

        let message_enum_ai1 = MessageEnum::Ai(ai_message1.clone());
        let message_enum_ai2 = MessageEnum::Ai(ai_message2.clone());
        let message_enum_human = MessageEnum::Human(human_message.clone());

        // Check equality between identical AiMessages
        assert_eq!(message_enum_ai1, message_enum_ai2);

        // Check inequality between AiMessage and HumanMessage
        assert_ne!(message_enum_ai1, message_enum_human);
    }

    #[test]
    fn test_message_enum_debug_format() {
        let system_message = SystemMessage::new("System message.");
        let message_enum = MessageEnum::System(system_message);

        let debug_output = format!("{:?}", message_enum);
        let expected_debug_output = r#"SystemMessage(SystemMessage { base: BaseMessageFields { content: "System message.", example: false, message_type: System, additional_kwargs: {}, response_metadata: {}, id: None, name: None } })"#;
        assert_eq!(debug_output, expected_debug_output);
    }

    #[test]
    fn test_message_enum_serialize_with_optional_fields() {
        let mut human_message = HumanMessage::new("Hello.");
        human_message.base.id = Some("1234".to_string());
        human_message.base.name = Some("Human User".to_string());

        let message_enum = MessageEnum::Human(human_message);
        let expected_json = json!({
            "role": "human",
            "content": "Hello.",
            "example": false,
            "message_type": "Human",
            "id": "1234",
            "name": "Human User"
        });

        let serialized = serde_json::to_string(&message_enum).unwrap();
        let actual_json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_message_enum_deserialize_with_optional_fields() {
        let json_data = json!({
            "role": "human",
            "content": "Hello.",
            "example": false,
            "message_type": "Human",
            "id": "1234",
            "name": "Human User"
        })
        .to_string();

        let message_enum: MessageEnum = serde_json::from_str(&json_data).unwrap();
        match message_enum {
            MessageEnum::Human(human_message) => {
                assert_eq!(human_message.id(), Some("1234"));
                assert_eq!(human_message.name(), Some("Human User"));
            }
            _ => panic!("Expected HumanMessage"),
        }
    }

    #[test]
    fn test_serialize_vec_of_messages() {
        let ai_message = AiMessage::new("Hello from AI.");
        let system_message = SystemMessage::new("System message.");
        let human_message = HumanMessage::new("Hello from Human.");

        let messages: Vec<MessageEnum> = vec![
            ai_message.into(),
            system_message.into(),
            human_message.into(),
        ];

        let serialized = serde_json::to_value(&messages).unwrap();

        let expected_json = json!([
            {
                "role": "ai",
                "content": "Hello from AI.",
                "example": false,
                "message_type": "Ai"
            },
            {
                "role": "system",
                "content": "System message.",
                "example": false,
                "message_type": "System"
            },
            {
                "role": "human",
                "content": "Hello from Human.",
                "example": false,
                "message_type": "Human"
            }
        ]);

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_deserialize_vec_of_messages() {
        let json_data = json!([
            {
                "role": "ai",
                "content": "Hello from AI.",
                "example": false,
                "message_type": "Ai"
            },
            {
                "role": "system",
                "content": "System message.",
                "example": false,
                "message_type": "System"
            },
            {
                "role": "human",
                "content": "Hello from Human.",
                "example": false,
                "message_type": "Human"
            }
        ])
        .to_string();

        let messages: Vec<MessageEnum> = serde_json::from_str(&json_data).unwrap();

        assert_eq!(messages.len(), 3);

        if let MessageEnum::Ai(ai_message) = &messages[0] {
            assert_eq!(ai_message.content(), "Hello from AI.");
            assert_eq!(ai_message.message_type(), &crate::MessageType::Ai);
        } else {
            panic!("Expected AiMessage");
        }

        if let MessageEnum::System(system_message) = &messages[1] {
            assert_eq!(system_message.content(), "System message.");
            assert_eq!(system_message.message_type(), &crate::MessageType::System);
        } else {
            panic!("Expected SystemMessage");
        }

        if let MessageEnum::Human(human_message) = &messages[2] {
            assert_eq!(human_message.content(), "Hello from Human.");
            assert_eq!(human_message.message_type(), &crate::MessageType::Human);
        } else {
            panic!("Expected HumanMessage");
        }
    }

    #[test]
    fn test_message_enum_deserialization_with_defaults() {
        let json_data = json!({
            "role": "ai",
            "content": "Hello, AI."
        })
        .to_string();

        let message_enum: MessageEnum = serde_json::from_str(&json_data).unwrap();

        match message_enum {
            MessageEnum::Ai(ai_message) => {
                assert_eq!(ai_message.base.content, "Hello, AI.");
                assert_eq!(ai_message.base.message_type, MessageType::Ai);
                assert!(!ai_message.base.example);
            }
            _ => panic!("Expected AiMessage"),
        }
    }

    #[test]
    fn test_message_enum_serialization_with_message_type() {
        let ai_message = AiMessage {
            base: BaseMessageFields {
                content: "Hello from AI.".to_string(),
                example: false,
                additional_kwargs: HashMap::new(),
                response_metadata: HashMap::new(),
                id: None,
                name: None,
                message_type: MessageType::Ai, // message_type is now part of serialization
            },
        };

        let message_enum = MessageEnum::Ai(ai_message);

        let expected_json = json!({
            "role": "ai",
            "content": "Hello from AI.",
            "example": false,
            "message_type": "Ai"
        });

        let serialized = serde_json::to_string(&message_enum).unwrap();
        let actual_json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
