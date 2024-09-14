use crate::prelude::*;

define_message!(Ai);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    define_message!(MessageType::Ai);

    #[test]
    fn test_aimessage_creation() {
        let mut ai_message = AiMessage::new("This is an AI message.");
        assert_eq!(ai_message.content(), "This is an AI message.");
        assert_eq!(ai_message.message_type(), &MessageType::Ai);
        assert!(!ai_message.is_example());
        assert!(ai_message.additional_kwargs().is_empty());
        assert!(ai_message.response_metadata().is_empty());
        assert_eq!(ai_message.id(), None);
        assert_eq!(ai_message.name(), None);
        ai_message.set_content("new_content");
        assert_eq!(ai_message.content(), "new_content");
    }

    #[test]
    fn test_aimessage_example() {
        let mut ai_message = AiMessage::new("Example message.");
        ai_message.set_example(true);
        assert!(ai_message.is_example());
    }

    #[test]
    fn test_aimessage_with_id_and_name() {
        let mut ai_message = AiMessage::new("This is an AI message.");
        ai_message.set_id(Some("12345".to_string()));
        ai_message.set_name(Some("AI Bot".to_string()));

        assert_eq!(ai_message.id(), Some("12345"));
        assert_eq!(ai_message.name(), Some("AI Bot"));
    }

    #[test]
    fn test_aimessage_with_additional_kwargs() {
        let mut ai_message = AiMessage::new("This is an AI message.");
        ai_message
            .base
            .additional_kwargs
            .insert("key".to_string(), "value".to_string());

        assert_eq!(
            ai_message.additional_kwargs().get("key"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_aimessage_with_response_metadata() {
        let mut ai_message = AiMessage::new("This is an AI message.");
        ai_message
            .base
            .response_metadata
            .insert("source".to_string(), "AI Model".to_string());

        assert_eq!(
            ai_message.response_metadata().get("source"),
            Some(&"AI Model".to_string())
        );
    }

    #[test]
    fn test_aimessage_serialization() {
        let ai_message = AiMessage::new("This is an AI message.");
        let expected_json = json!({
            "content": "This is an AI message.",
            "example": false,
            "message_type": "Ai",
            "role": "ai",
        });
        let serialized = serde_json::to_string(&ai_message).unwrap();
        let expected = expected_json.to_string();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_aimessage_deserialization() {
        let json_data = json!({
            "role": "ai",
            "content": "This is an AI message.",
            "example": false,
            "message_type": "Ai",
        })
        .to_string();

        let ai_message: AiMessage = serde_json::from_str(&json_data).unwrap();
        assert_eq!(ai_message.content(), "This is an AI message.");
        assert_eq!(ai_message.message_type(), &MessageType::Ai);
    }

    #[test]
    fn test_aimessage_debug_format() {
        let ai_message = AiMessage::new("Debug AI message.");
        let debug_output = format!("{:?}", ai_message);
        let expected_debug_output = r#"AiMessage { base: BaseMessageFields { content: "Debug AI message.", example: false, message_type: Ai, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
        assert_eq!(debug_output, expected_debug_output);
    }

    #[test]
    fn test_aimessage_with_all_fields() {
        let mut ai_message = AiMessage::new("This is an AI message.");
        ai_message.base.id = Some("AI123".to_string());
        ai_message.base.name = Some("AI Assistant".to_string());
        ai_message
            .base
            .additional_kwargs
            .insert("task".to_string(), "information retrieval".to_string());
        ai_message
            .base
            .response_metadata
            .insert("model".to_string(), "gpt-3".to_string());

        assert_eq!(ai_message.id(), Some("AI123"));
        assert_eq!(ai_message.name(), Some("AI Assistant"));
        assert_eq!(
            ai_message.additional_kwargs().get("task"),
            Some(&"information retrieval".to_string())
        );
        assert_eq!(
            ai_message.response_metadata().get("model"),
            Some(&"gpt-3".to_string())
        );

        let expected_json = json!({
            "role": "ai",
            "content": "This is an AI message.",
            "example": false,
            "message_type": "Ai",
            "id": "AI123",
            "name": "AI Assistant",
            "additional_kwargs": {
                "task": "information retrieval"
            },
            "response_metadata": {
                "model": "gpt-3"
            }
        });
        let serialized = serde_json::to_string(&ai_message).unwrap();
        let expected = expected_json.to_string();
        assert_eq!(serialized, expected);
    }
}
