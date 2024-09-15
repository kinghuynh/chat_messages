use crate::prelude::*;

define_message!(System);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    define_message!(MessageType::System);

    #[test]
    fn test_systemmessage_creation() {
        let mut system_message = SystemMessage::new("This is a system message.");
        assert_eq!(system_message.content(), "This is a system message.");
        assert_eq!(system_message.message_type(), &MessageType::System);
        assert!(!system_message.is_example());
        assert!(system_message.additional_kwargs().is_empty());
        assert!(system_message.response_metadata().is_empty());
        assert_eq!(system_message.id(), None);
        assert_eq!(system_message.name(), None);
        system_message.set_content("new_content");
        assert_eq!(system_message.content(), "new_content");
    }

    #[test]
    fn test_systemmessage_example() {
        let mut system_message = SystemMessage::new("Example system message.");
        system_message.set_example(true);
        assert!(system_message.is_example());
    }

    #[test]
    fn test_systemmessage_with_id_and_name() {
        let mut system_message = SystemMessage::new("This is a system message.");
        system_message.set_id(Some("54321".to_string()));
        system_message.set_name(Some("System Admin".to_string()));

        assert_eq!(system_message.id(), Some("54321"));
        assert_eq!(system_message.name(), Some("System Admin"));
    }

    #[test]
    fn test_systemmessage_with_additional_kwargs() {
        let mut system_message = SystemMessage::new("This is a system message.");
        system_message
            .base
            .additional_kwargs
            .insert("key".to_string(), "value".to_string());

        assert_eq!(
            system_message.additional_kwargs().get("key"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_systemmessage_with_response_metadata() {
        let mut system_message = SystemMessage::new("This is a system message.");
        system_message
            .base
            .response_metadata
            .insert("source".to_string(), "System Process".to_string());

        assert_eq!(
            system_message.response_metadata().get("source"),
            Some(&"System Process".to_string())
        );
    }

    #[test]
    fn test_systemmessage_serialization() {
        let system_message = SystemMessage::new("This is a system message.");
        let expected_json = json!({
            "content": "This is a system message.",
            "example": false,
            "message_type": "System"
        });

        let serialized: Value = serde_json::to_value(&system_message).unwrap();
        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_systemmessage_deserialization() {
        let json_data = json!({
            "content": "This is a system message.",
            "example": false,
            "message_type": "System"
        });

        let system_message: SystemMessage = serde_json::from_value(json_data).unwrap();
        assert_eq!(system_message.content(), "This is a system message.");
        assert_eq!(system_message.message_type(), &MessageType::System);
    }

    #[test]
    fn test_systemmessage_debug_format() {
        let system_message = SystemMessage::new("Debug system message.");
        let debug_output = format!("{:?}", system_message);
        let expected_debug_output = r#"SystemMessage { base: BaseMessageFields { content: "Debug system message.", example: false, message_type: System, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
        assert_eq!(debug_output, expected_debug_output);
    }

    #[test]
    fn test_systemmessage_with_all_fields() {
        let mut system_message = SystemMessage::new("This is a system message.");
        system_message.base.id = Some("SYS123".to_string());
        system_message.base.name = Some("System Bot".to_string());
        system_message
            .base
            .additional_kwargs
            .insert("task".to_string(), "system monitoring".to_string());
        system_message
            .base
            .response_metadata
            .insert("process".to_string(), "systemd".to_string());

        assert_eq!(system_message.id(), Some("SYS123"));
        assert_eq!(system_message.name(), Some("System Bot"));
        assert_eq!(
            system_message.additional_kwargs().get("task"),
            Some(&"system monitoring".to_string())
        );
        assert_eq!(
            system_message.response_metadata().get("process"),
            Some(&"systemd".to_string())
        );

        let expected_json = json!({
            "content": "This is a system message.",
            "example": false,
            "message_type": "System",
            "id": "SYS123",
            "name": "System Bot",
            "additional_kwargs": {
                "task": "system monitoring"
            },
            "response_metadata": {
                "process": "systemd"
            }
        });

        let serialized: Value = serde_json::to_value(&system_message).unwrap();
        assert_eq!(serialized, expected_json);
    }
}
