use crate::prelude::*;

define_message!(Human);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    define_message!(MessageType::Human);

    #[test]
    fn test_humanmessage_creation() {
        let mut human_message = HumanMessage::new("This is a human message.");
        assert_eq!(human_message.content(), "This is a human message.");
        assert_eq!(human_message.message_type(), &MessageType::Human);
        assert!(!human_message.is_example());
        assert!(human_message.additional_kwargs().is_empty());
        assert!(human_message.response_metadata().is_empty());
        assert_eq!(human_message.id(), None);
        assert_eq!(human_message.name(), None);
        human_message.set_content("new_content");
        assert_eq!(human_message.content(), "new_content");
    }

    #[test]
    fn test_humanmessage_example() {
        let mut human_message = HumanMessage::new("Example human message.");
        human_message.set_example(true);
        assert!(human_message.is_example());
    }

    #[test]
    fn test_humanmessage_with_id_and_name() {
        let mut human_message = HumanMessage::new("This is a human message.");
        human_message.set_id(Some("98765".to_string()));
        human_message.set_name(Some("User123".to_string()));

        assert_eq!(human_message.id(), Some("98765"));
        assert_eq!(human_message.name(), Some("User123"));
    }

    #[test]
    fn test_humanmessage_with_additional_kwargs() {
        let mut human_message = HumanMessage::new("This is a human message.");
        human_message
            .base
            .additional_kwargs
            .insert("mood".to_string(), "curious".to_string());

        assert_eq!(
            human_message.additional_kwargs().get("mood"),
            Some(&"curious".to_string())
        );
    }

    #[test]
    fn test_humanmessage_with_response_metadata() {
        let mut human_message = HumanMessage::new("This is a human message.");
        human_message
            .base
            .response_metadata
            .insert("source".to_string(), "User".to_string());

        assert_eq!(
            human_message.response_metadata().get("source"),
            Some(&"User".to_string())
        );
    }

    #[test]
    fn test_humanmessage_serialization() {
        let human_message = HumanMessage::new("This is a human message.");
        let expected_json = json!({
            "content": "This is a human message.",
            "example": false,
            "message_type": "Human"
        });

        let serialized: Value = serde_json::to_value(&human_message).unwrap();
        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_humanmessage_deserialization() {
        let json_data = json!({
            "content": "This is a human message.",
            "example": false,
            "message_type": "Human"
        });

        let human_message: HumanMessage = serde_json::from_value(json_data).unwrap();
        assert_eq!(human_message.content(), "This is a human message.");
        assert_eq!(human_message.message_type(), &MessageType::Human);
    }

    #[test]
    fn test_humanmessage_debug_format() {
        let human_message = HumanMessage::new("Debug human message.");
        let debug_output = format!("{:?}", human_message);
        let expected_debug_output = r#"HumanMessage { base: BaseMessageFields { content: "Debug human message.", example: false, message_type: Human, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
        assert_eq!(debug_output, expected_debug_output);
    }

    #[test]
    fn test_humanmessage_with_all_fields() {
        let mut human_message = HumanMessage::new("This is a human message.");
        human_message.base.id = Some("HUM123".to_string());
        human_message.base.name = Some("User123".to_string());
        human_message
            .base
            .additional_kwargs
            .insert("intent".to_string(), "query".to_string());
        human_message
            .base
            .response_metadata
            .insert("platform".to_string(), "mobile".to_string());

        assert_eq!(human_message.id(), Some("HUM123"));
        assert_eq!(human_message.name(), Some("User123"));
        assert_eq!(
            human_message.additional_kwargs().get("intent"),
            Some(&"query".to_string())
        );
        assert_eq!(
            human_message.response_metadata().get("platform"),
            Some(&"mobile".to_string())
        );

        let expected_json = json!({
            "content": "This is a human message.",
            "example": false,
            "message_type": "Human",
            "id": "HUM123",
            "name": "User123",
            "additional_kwargs": {
                "intent": "query"
            },
            "response_metadata": {
                "platform": "mobile"
            }
        });

        let serialized: Value = serde_json::to_value(&human_message).unwrap();
        assert_eq!(serialized, expected_json);
    }
}
