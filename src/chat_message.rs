use crate::prelude::*;
use derive_base_message::BaseMessage;

#[derive(BaseMessage, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    role: String,
    #[serde(flatten)]
    base: BaseMessageFields,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_chat_message_serialization_with_empty_fields() {
        let chat_message = ChatMessage::new(
            "Test message",
            "User".to_string(), // role
        );

        let serialized = serde_json::to_string(&chat_message).expect("Serialization failed");

        let expected =
            r#"{"role":"User","content":"Test message","example":false,"message_type":"Chat"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_chat_message_serialization_with_values() {
        let mut additional_kwargs = HashMap::new();
        additional_kwargs.insert("key1".to_string(), "value1".to_string());

        let mut response_metadata = HashMap::new();
        response_metadata.insert("meta_key".to_string(), "meta_value".to_string());

        let mut chat_message = ChatMessage::new("Test message", "User".to_string());

        chat_message.base.example = true;
        chat_message.base.additional_kwargs = additional_kwargs.clone();
        chat_message.base.response_metadata = response_metadata.clone();
        chat_message.base.id = Some("1234".to_string());
        chat_message.base.name = Some("Test Name".to_string());

        let serialized = serde_json::to_string(&chat_message).expect("Serialization failed");

        // The expected output should not have an extra closing brace
        let expected = r#"{"role":"User","content":"Test message","example":true,"message_type":"Chat","additional_kwargs":{"key1":"value1"},"response_metadata":{"meta_key":"meta_value"},"id":"1234","name":"Test Name"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_chat_message_deserialization() {
        let json_data = r#"
        {
            "role": "User",
            "content": "Test message",
            "example": true,
            "message_type": "Chat",
            "additional_kwargs": {
                "key1": "value1"
            },
            "response_metadata": {
                "meta_key": "meta_value"
            },
            "id": "1234",
            "name": "Test Name"
        }
        "#;

        let chat_message: ChatMessage =
            serde_json::from_str(json_data).expect("Deserialization failed");

        assert_eq!(chat_message.role, "User");
        assert_eq!(chat_message.base.content, "Test message");
        assert!(chat_message.base.example);
        assert_eq!(chat_message.base.message_type, MessageType::Chat);
        assert_eq!(
            chat_message.base.additional_kwargs.get("key1").unwrap(),
            "value1"
        );
        assert_eq!(
            chat_message.base.response_metadata.get("meta_key").unwrap(),
            "meta_value"
        );
        assert_eq!(chat_message.base.id.as_deref(), Some("1234"));
        assert_eq!(chat_message.base.name.as_deref(), Some("Test Name"));
    }

    #[test]
    fn test_chat_message_serialization_with_partial_values() {
        let mut additional_kwargs = HashMap::new();
        additional_kwargs.insert("key2".to_string(), "value2".to_string());

        let mut chat_message = ChatMessage::new("Partial message", "User".to_string());

        chat_message.base.additional_kwargs = additional_kwargs.clone();
        chat_message.base.id = Some("5678".to_string());

        let serialized = serde_json::to_string(&chat_message).expect("Serialization failed");

        let expected = r#"{"role":"User","content":"Partial message","example":false,"message_type":"Chat","additional_kwargs":{"key2":"value2"},"id":"5678"}"#;
        assert_eq!(serialized, expected);
    }
}
