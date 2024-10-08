use std::collections::HashMap;
use std::fmt;

use crate::tool_message::ToolStatus;
use crate::{
    AiMessage, BaseMessageFields, HumanMessage, InvalidMessageTypeError, SystemMessage, ToolMessage,
};
use crate::{BaseMessage, MessageType};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum MessageEnum {
    Ai(AiMessage),
    Human(HumanMessage),
    System(SystemMessage),
    Tool(ToolMessage),
}

impl MessageEnum {
    pub fn as_human(&self) -> Option<&HumanMessage> {
        if let MessageEnum::Human(ref message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn as_ai(&self) -> Option<&AiMessage> {
        if let MessageEnum::Ai(ref message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn as_system(&self) -> Option<&SystemMessage> {
        if let MessageEnum::System(ref message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn as_tool(&self) -> Option<&ToolMessage> {
        if let MessageEnum::Tool(ref message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn human_from(input: &str) -> Result<HumanMessage, InvalidMessageTypeError> {
        match MessageEnum::try_from(input)? {
            MessageEnum::Human(human_message) => Ok(human_message),
            _ => Err(InvalidMessageTypeError::new(format!(
                "Expected a HumanMessage, got a different type: {}",
                input
            ))),
        }
    }

    pub fn ai_from(input: &str) -> Result<AiMessage, InvalidMessageTypeError> {
        match MessageEnum::try_from(input)? {
            MessageEnum::Ai(ai_message) => Ok(ai_message),
            _ => Err(InvalidMessageTypeError::new(format!(
                "Expected an AiMessage, got a different type: {}",
                input
            ))),
        }
    }

    pub fn system_from(input: &str) -> Result<SystemMessage, InvalidMessageTypeError> {
        match MessageEnum::try_from(input)? {
            MessageEnum::System(system_message) => Ok(system_message),
            _ => Err(InvalidMessageTypeError::new(format!(
                "Expected a SystemMessage, got a different type: {}",
                input
            ))),
        }
    }

    pub fn tool_from(input: &str) -> Result<ToolMessage, InvalidMessageTypeError> {
        match MessageEnum::try_from(input)? {
            MessageEnum::Tool(tool_message) => Ok(tool_message),
            _ => Err(InvalidMessageTypeError::new(format!(
                "Expected a ToolMessage, got a different type: {}",
                input
            ))),
        }
    }

    fn parse_tool_message(content: &str) -> Result<Self, InvalidMessageTypeError> {
        let tool_parts: Vec<&str> = content.splitn(2, ": ").collect();
        if tool_parts.len() == 2 {
            let tool_id = tool_parts[0].to_string();
            let tool_content = tool_parts[1];
            Ok(MessageEnum::Tool(ToolMessage::new(
                tool_content,
                tool_id,
                None,
                ToolStatus::Success,
            )))
        } else {
            Err(InvalidMessageTypeError::new(format!(
                "Invalid tool message format: {}",
                content
            )))
        }
    }

    pub fn parse_messages(input: &str) -> Result<Vec<MessageEnum>, InvalidMessageTypeError> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Filter out empty lines
            .map(MessageEnum::try_from)
            .collect()
    }
}

impl BaseMessage for MessageEnum {
    fn content(&self) -> &str {
        match self {
            MessageEnum::Ai(message) => message.content(),
            MessageEnum::Human(message) => message.content(),
            MessageEnum::System(message) => message.content(),
            MessageEnum::Tool(message) => message.content(),
        }
    }

    fn message_type(&self) -> &MessageType {
        match self {
            MessageEnum::Ai(message) => message.message_type(),
            MessageEnum::Human(message) => message.message_type(),
            MessageEnum::System(message) => message.message_type(),
            MessageEnum::Tool(message) => message.message_type(),
        }
    }

    fn role(&self) -> &str {
        match self {
            MessageEnum::Ai(_) => "ai",
            MessageEnum::Human(_) => "human",
            MessageEnum::System(_) => "system",
            MessageEnum::Tool(_) => "tool",
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            MessageEnum::Ai(message) => message.name(),
            MessageEnum::Human(message) => message.name(),
            MessageEnum::System(message) => message.name(),
            MessageEnum::Tool(message) => message.name(),
        }
    }

    fn is_example(&self) -> bool {
        match self {
            MessageEnum::Ai(message) => message.is_example(),
            MessageEnum::Human(message) => message.is_example(),
            MessageEnum::System(message) => message.is_example(),
            MessageEnum::Tool(message) => message.is_example(),
        }
    }

    fn additional_kwargs(&self) -> &HashMap<String, String> {
        match self {
            MessageEnum::Ai(message) => message.additional_kwargs(),
            MessageEnum::Human(message) => message.additional_kwargs(),
            MessageEnum::System(message) => message.additional_kwargs(),
            MessageEnum::Tool(message) => message.additional_kwargs(),
        }
    }

    fn response_metadata(&self) -> &HashMap<String, String> {
        match self {
            MessageEnum::Ai(message) => message.response_metadata(),
            MessageEnum::Human(message) => message.response_metadata(),
            MessageEnum::System(message) => message.response_metadata(),
            MessageEnum::Tool(message) => message.response_metadata(),
        }
    }

    fn id(&self) -> Option<&str> {
        match self {
            MessageEnum::Ai(message) => message.id(),
            MessageEnum::Human(message) => message.id(),
            MessageEnum::System(message) => message.id(),
            MessageEnum::Tool(message) => message.id(),
        }
    }
}

impl fmt::Debug for MessageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageEnum::Ai(message) => write!(f, "AiMessage({:?})", message),
            MessageEnum::Human(message) => write!(f, "HumanMessage({:?})", message),
            MessageEnum::System(message) => write!(f, "SystemMessage({:?})", message),
            MessageEnum::Tool(message) => write!(f, "ToolMessage({:?})", message),
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

impl From<ToolMessage> for MessageEnum {
    fn from(message: ToolMessage) -> Self {
        MessageEnum::Tool(message)
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

            // ToolMessage specific fields
            #[serde(default)]
            tool_call_id: Option<String>,
            #[serde(default)]
            artifact: Option<String>,
            #[serde(default)]
            status: Option<ToolStatus>,
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
            MessageType::Tool => {
                let tool_call_id = temp.tool_call_id.ok_or_else(|| {
                    serde::de::Error::custom("Missing tool_call_id for ToolMessage")
                })?;
                let status = temp
                    .status
                    .ok_or_else(|| serde::de::Error::custom("Missing status for ToolMessage"))?;
                Ok(MessageEnum::Tool(ToolMessage::new_with_base(
                    tool_call_id,
                    temp.artifact,
                    status,
                    base,
                )))
            }
            _ => Err(serde::de::Error::custom("Unsupported message type")),
        }
    }
}

impl TryFrom<&str> for MessageEnum {
    type Error = InvalidMessageTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.splitn(2, ": ").collect();

        if parts.len() != 2 {
            return Err(InvalidMessageTypeError::new(format!(
                "Invalid message format: {}",
                value
            )));
        }

        let (role_part, content) = (parts[0], parts[1]);

        match role_part.to_lowercase().as_str() {
            "human" => Ok(MessageEnum::Human(HumanMessage::new(content))),
            "ai" => Ok(MessageEnum::Ai(AiMessage::new(content))),
            "system" => Ok(MessageEnum::System(SystemMessage::new(content))),
            "tool" => Self::parse_tool_message(content),
            _ => Err(InvalidMessageTypeError::new(format!(
                "Invalid message type: {}",
                value
            ))),
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
    fn test_message_enum_serialization_tool_message() {
        let base = BaseMessageFields {
            content: "Tool message content".to_string(),
            example: false,
            message_type: MessageType::Tool,
            additional_kwargs: HashMap::new(),
            response_metadata: HashMap::new(),
            id: None,
            name: None,
        };

        let tool_message = ToolMessage::new_with_base(
            "tool_call_001".to_string(),
            Some("artifact_001".to_string()),
            ToolStatus::Success,
            base,
        );

        let message_enum = MessageEnum::Tool(tool_message);

        let expected_json = json!({
            "tool_call_id": "tool_call_001",
            "artifact": "artifact_001",
            "status": "Success",
            "content": "Tool message content",
            "example": false,
            "role": "tool",
            "message_type": "Tool"
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
    fn test_message_enum_tool_deserialization() {
        let json_data = r#"
    {
        "role": "tool",
        "content": "Tool message content",
        "tool_call_id": "tool_call_001",
        "status": "Success",
        "artifact": "some_artifact",
        "example": true,
        "id": "tool_123",
        "name": "Tool Example",
        "additional_kwargs": {
            "key": "value"
        },
        "response_metadata": {
            "meta_key": "meta_value"
        }
    }
    "#;

        let message_enum: MessageEnum =
            serde_json::from_str(json_data).expect("Deserialization failed");

        if let MessageEnum::Tool(tool_message) = message_enum {
            assert_eq!(tool_message.tool_call_id(), "tool_call_001");
            assert_eq!(tool_message.artifact().as_deref(), Some("some_artifact"));
            assert_eq!(tool_message.status(), &ToolStatus::Success);
            assert_eq!(tool_message.content(), "Tool message content");
            assert!(tool_message.is_example());
            assert_eq!(tool_message.id(), Some("tool_123"));
            assert_eq!(tool_message.name(), Some("Tool Example"));
            assert_eq!(
                tool_message.additional_kwargs().get("key").unwrap(),
                "value"
            );
            assert_eq!(
                tool_message.response_metadata().get("meta_key").unwrap(),
                "meta_value"
            );
        } else {
            panic!("Expected ToolMessage, got something else");
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
        let tool_message = ToolMessage::new(
            "Tool message content",
            "tool_call_001".to_string(),
            Some("artifact_001".to_string()),
            ToolStatus::Success,
        );

        let messages: Vec<MessageEnum> = vec![
            ai_message.into(),
            system_message.into(),
            human_message.into(),
            tool_message.into(),
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
            },
            {
                "tool_call_id": "tool_call_001",
                "artifact": "artifact_001",
                "status": "Success",
                "content": "Tool message content",
                "example": false,
                "message_type": "Tool",
                "role": "tool"
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
            },
            {
                "tool_call_id": "tool_call_001",
                "artifact": "artifact_001",
                "status": "Success",
                "content": "Tool message content",
                "example": false,
                "message_type": "Tool",
                "role": "tool"
            }
        ])
        .to_string();

        let messages: Vec<MessageEnum> = serde_json::from_str(&json_data).unwrap();

        assert_eq!(messages.len(), 4);

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

        if let MessageEnum::Tool(tool_message) = &messages[3] {
            assert_eq!(tool_message.tool_call_id(), "tool_call_001");
            assert_eq!(tool_message.artifact().as_deref(), Some("artifact_001"));
            assert_eq!(tool_message.status(), &ToolStatus::Success);
            assert_eq!(tool_message.content(), "Tool message content");
        } else {
            panic!("Expected ToolMessage");
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
                message_type: MessageType::Ai,
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
    fn test_as_human() {
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

        let message_enum = MessageEnum::Human(human_message.clone());

        assert!(message_enum.as_human().is_some());
        let extracted_message = message_enum.as_human().unwrap();
        assert_eq!(extracted_message.content(), "Hello from Human.");

        assert!(message_enum.as_ai().is_none());
        assert!(message_enum.as_system().is_none());
    }

    #[test]
    fn test_as_ai() {
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

        let message_enum = MessageEnum::Ai(ai_message.clone());

        assert!(message_enum.as_ai().is_some());
        let extracted_message = message_enum.as_ai().unwrap();
        assert_eq!(extracted_message.content(), "Hello from AI.");

        assert!(message_enum.as_human().is_none());
        assert!(message_enum.as_system().is_none());
    }

    #[test]
    fn test_as_system() {
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

        let message_enum = MessageEnum::System(system_message.clone());

        // Test valid SystemMessage
        assert!(message_enum.as_system().is_some());
        let extracted_message = message_enum.as_system().unwrap();
        assert_eq!(extracted_message.content(), "This is a system message.");

        // Ensure invalid cast returns None
        assert!(message_enum.as_human().is_none());
        assert!(message_enum.as_ai().is_none());
    }

    #[test]
    fn test_as_tool() {
        let tool_message = ToolMessage::new(
            "Tool message content",
            "tool_call_001".to_string(),
            Some("artifact_001".to_string()),
            ToolStatus::Success,
        );

        let message_enum = MessageEnum::Tool(tool_message.clone());

        // Test valid ToolMessage
        assert!(message_enum.as_tool().is_some());
        let extracted_message = message_enum.as_tool().unwrap();
        assert_eq!(extracted_message.content(), "Tool message content");
        assert_eq!(extracted_message.tool_call_id(), "tool_call_001");
        assert_eq!(
            extracted_message.artifact().as_deref(),
            Some("artifact_001")
        );
        assert_eq!(extracted_message.status(), &ToolStatus::Success);

        // Ensure invalid cast returns None
        assert!(message_enum.as_human().is_none());
        assert!(message_enum.as_ai().is_none());
        assert!(message_enum.as_system().is_none());
    }

    #[test]
    fn test_mixed_message_enum() {
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

        let system_message = SystemMessage {
            base: BaseMessageFields {
                content: "System message.".to_string(),
                example: false,
                message_type: MessageType::System,
                additional_kwargs: HashMap::new(),
                response_metadata: HashMap::new(),
                id: None,
                name: None,
            },
        };

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

        let tool_message = ToolMessage::new(
            "Tool message content",
            "tool_call_001".to_string(),
            Some("artifact_001".to_string()),
            ToolStatus::Success,
        );

        let message_enum_human = MessageEnum::Human(human_message.clone());
        let message_enum_system = MessageEnum::System(system_message.clone());
        let message_enum_ai = MessageEnum::Ai(ai_message.clone());
        let message_enum_tool = MessageEnum::Tool(tool_message.clone());

        // Ensure each function returns the correct type and None for others
        assert_eq!(message_enum_human.as_human().unwrap().role(), "human");
        assert!(message_enum_human.as_ai().is_none());
        assert!(message_enum_human.as_system().is_none());

        assert_eq!(message_enum_system.as_system().unwrap().role(), "system");
        assert!(message_enum_system.as_human().is_none());
        assert!(message_enum_system.as_ai().is_none());

        assert_eq!(message_enum_ai.as_ai().unwrap().role(), "ai");
        assert!(message_enum_ai.as_human().is_none());
        assert!(message_enum_ai.as_system().is_none());

        assert_eq!(message_enum_tool.as_tool().unwrap().role(), "tool");
    }

    #[test]
    fn test_try_from_human_message_success() {
        let input = "Human: Hello from Human.";
        let message_enum = MessageEnum::try_from(input).unwrap();

        match message_enum {
            MessageEnum::Human(human_message) => {
                assert_eq!(human_message.content(), "Hello from Human.");
            }
            _ => panic!("Expected HumanMessage"),
        }

        let input = "human: Hello from Human.";
        let message_enum = MessageEnum::try_from(input).unwrap();

        match message_enum {
            MessageEnum::Human(human_message) => {
                assert_eq!(human_message.content(), "Hello from Human.");
            }
            _ => panic!("Expected HumanMessage"),
        }
    }

    #[test]
    fn test_try_from_ai_message_success() {
        let input = "Ai: Hello from AI.";
        let message_enum = MessageEnum::try_from(input).unwrap();

        match message_enum {
            MessageEnum::Ai(ai_message) => {
                assert_eq!(ai_message.content(), "Hello from AI.");
            }
            _ => panic!("Expected AiMessage"),
        }
    }

    #[test]
    fn test_try_from_system_message_success() {
        let input = "System: System message content.";
        let message_enum = MessageEnum::try_from(input).unwrap();

        match message_enum {
            MessageEnum::System(system_message) => {
                assert_eq!(system_message.content(), "System message content.");
            }
            _ => panic!("Expected SystemMessage"),
        }
    }

    #[test]
    fn test_try_from_tool_message_success() {
        let input = "Tool: tool_123: Tool message content.";
        let message_enum = MessageEnum::try_from(input).unwrap();

        match message_enum {
            MessageEnum::Tool(tool_message) => {
                assert_eq!(tool_message.tool_call_id(), "tool_123");
                assert_eq!(tool_message.content(), "Tool message content.");
            }
            _ => panic!("Expected ToolMessage"),
        }
    }

    #[test]
    fn test_try_from_tool_message_invalid_format() {
        let input = "Tool: Invalid tool format";
        let err = MessageEnum::try_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Invalid tool message format: Invalid tool format"
        );
    }

    #[test]
    fn test_try_from_invalid_message_format() {
        let input = "Invalid format";
        let err = MessageEnum::try_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Invalid message format: Invalid format"
        );
    }

    #[test]
    fn test_human_from_success() {
        let input = "Human: Hello from Human.";
        let human_message = MessageEnum::human_from(input).unwrap();

        assert_eq!(human_message.content(), "Hello from Human.");
    }

    #[test]
    fn test_human_from_invalid_type() {
        let input = "Ai: Hello from AI.";
        let err = MessageEnum::human_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Expected a HumanMessage, got a different type: Ai: Hello from AI."
        );
    }

    #[test]
    fn test_ai_from_success() {
        let input = "Ai: Hello from AI.";
        let ai_message = MessageEnum::ai_from(input).unwrap();

        assert_eq!(ai_message.content(), "Hello from AI.");
    }

    #[test]
    fn test_ai_from_invalid_type() {
        let input = "Human: Hello from Human.";
        let err = MessageEnum::ai_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Expected an AiMessage, got a different type: Human: Hello from Human."
        );
    }

    #[test]
    fn test_system_from_success() {
        let input = "System: System message content.";
        let system_message = MessageEnum::system_from(input).unwrap();

        assert_eq!(system_message.content(), "System message content.");
    }

    #[test]
    fn test_system_from_invalid_type() {
        let input = "Ai: System message content.";
        let err = MessageEnum::system_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Expected a SystemMessage, got a different type: Ai: System message content."
        );
    }

    #[test]
    fn test_tool_from_success() {
        let input = "Tool: tool_123: Tool message content.";
        let tool_message = MessageEnum::tool_from(input).unwrap();

        assert_eq!(tool_message.tool_call_id(), "tool_123");
        assert_eq!(tool_message.content(), "Tool message content.");
    }

    #[test]
    fn test_tool_from_invalid_type() {
        let input = "Human: Tool message content.";
        let err = MessageEnum::tool_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Expected a ToolMessage, got a different type: Human: Tool message content."
        );
    }

    #[test]
    fn test_tool_from_invalid_format() {
        let input = "Tool: Invalid format";
        let err = MessageEnum::tool_from(input).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid message type: Invalid tool message format: Invalid format"
        );
    }

    #[test]
    fn test_parse_valid_multi_line() {
        let input =
            "human: What is 2+2?\nai: 4\n\n\nhuman: What is 2+3?\nai: 5\n\nhuman: What is 4+4?";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_ok());
        let messages = result.unwrap();

        assert_eq!(messages.len(), 5);

        if let MessageEnum::Human(human_message) = &messages[0] {
            assert_eq!(human_message.content(), "What is 2+2?");
        } else {
            panic!("Expected HumanMessage for the first message");
        }

        if let MessageEnum::Ai(ai_message) = &messages[1] {
            assert_eq!(ai_message.content(), "4");
        } else {
            panic!("Expected AiMessage for the second message");
        }

        if let MessageEnum::Human(human_message) = &messages[2] {
            assert_eq!(human_message.content(), "What is 2+3?");
        } else {
            panic!("Expected HumanMessage for the third message");
        }

        if let MessageEnum::Ai(ai_message) = &messages[3] {
            assert_eq!(ai_message.content(), "5");
        } else {
            panic!("Expected AiMessage for the fourth message");
        }

        if let MessageEnum::Human(human_message) = &messages[4] {
            assert_eq!(human_message.content(), "What is 4+4?");
        } else {
            panic!("Expected HumanMessage for the fifth message");
        }
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let input = "\nhuman: What is 2+2?\n\n\nai: 4\n\nhuman: What is 4+4?\n\n";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_ok());
        let messages = result.unwrap();

        assert_eq!(messages.len(), 3);

        if let MessageEnum::Human(human_message) = &messages[0] {
            assert_eq!(human_message.content(), "What is 2+2?");
        } else {
            panic!("Expected HumanMessage for the first message");
        }

        if let MessageEnum::Ai(ai_message) = &messages[1] {
            assert_eq!(ai_message.content(), "4");
        } else {
            panic!("Expected AiMessage for the second message");
        }

        if let MessageEnum::Human(human_message) = &messages[2] {
            assert_eq!(human_message.content(), "What is 4+4?");
        } else {
            panic!("Expected HumanMessage for the third message");
        }
    }

    #[test]
    fn test_parse_invalid_message_format() {
        let input = "unknown: What is 2+2?\nai: 4";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mixed_valid_and_invalid_messages() {
        let input = "human: What is 2+2?\ninvalid: format\nai: 4";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_only_valid_messages() {
        let input = "human: What is 2+2?\nai: 4";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_ok());
        let messages = result.unwrap();

        assert_eq!(messages.len(), 2);

        if let MessageEnum::Human(human_message) = &messages[0] {
            assert_eq!(human_message.content(), "What is 2+2?");
        } else {
            panic!("Expected HumanMessage for the first message");
        }

        if let MessageEnum::Ai(ai_message) = &messages[1] {
            assert_eq!(ai_message.content(), "4");
        } else {
            panic!("Expected AiMessage for the second message");
        }
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";

        let result = MessageEnum::parse_messages(input);
        assert!(result.is_ok());
        let messages = result.unwrap();

        assert_eq!(messages.len(), 0);
    }
}
