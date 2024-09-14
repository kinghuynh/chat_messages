#[cfg(test)]
mod tests {
    use messageforge::prelude::*;

    define_message!(Human);
    define_message!(Ai);
    define_message!(System);

    #[test]
    fn test_human_message_creation() {
        let msg = HumanMessage::new("Hello, Human!");

        assert_eq!(msg.content(), "Hello, Human!");

        assert_eq!(msg.message_type(), &MessageType::Human);

        assert!(!msg.base.example);
        assert!(msg.base.additional_kwargs.is_empty());
        assert!(msg.base.response_metadata.is_empty());
        assert!(msg.base.id.is_none());
        assert!(msg.base.name.is_none());
    }

    #[test]
    fn test_ai_message_creation() {
        let msg = AiMessage::new("Hello, AI!");

        assert_eq!(msg.content(), "Hello, AI!");

        assert_eq!(msg.message_type(), &MessageType::Ai);
    }

    #[test]
    fn test_system_message_creation() {
        let msg = SystemMessage::new("System initialized!");

        assert_eq!(msg.content(), "System initialized!");

        assert_eq!(msg.message_type(), &MessageType::System);
    }

    #[test]
    fn test_custom_fields() {
        let mut msg = HumanMessage::new("Hello, World!");

        msg.base.example = true;
        msg.base
            .additional_kwargs
            .insert("key".to_string(), "value".to_string());
        msg.base
            .response_metadata
            .insert("token_count".to_string(), "42".to_string());
        msg.base.id = Some("12345".to_string());
        msg.base.name = Some("User".to_string());

        assert!(msg.base.example);
        assert_eq!(
            msg.base.additional_kwargs.get("key"),
            Some(&"value".to_string())
        );
        assert_eq!(
            msg.base.response_metadata.get("token_count"),
            Some(&"42".to_string())
        );
        assert_eq!(msg.base.id, Some("12345".to_string()));
        assert_eq!(msg.base.name, Some("User".to_string()));
    }

    #[test]
    fn test_empty_content() {
        let msg = HumanMessage::new("");

        assert_eq!(msg.content(), "");
        assert_eq!(msg.message_type(), &MessageType::Human);
    }

    #[test]
    fn test_multiple_message_types() {
        let human_msg = HumanMessage::new("Hello, Human!");
        let ai_msg = AiMessage::new("Hello, AI!");
        let system_msg = SystemMessage::new("System initiated");

        assert_eq!(human_msg.message_type(), &MessageType::Human);
        assert_eq!(human_msg.content(), "Hello, Human!");

        assert_eq!(ai_msg.message_type(), &MessageType::Ai);
        assert_eq!(ai_msg.content(), "Hello, AI!");

        assert_eq!(system_msg.message_type(), &MessageType::System);
        assert_eq!(system_msg.content(), "System initiated");
    }

    #[test]
    fn test_default_additional_kwargs() {
        let mut msg = AiMessage::new("AI Test");

        assert!(msg.base.additional_kwargs.is_empty());

        msg.base
            .additional_kwargs
            .insert("test_key".to_string(), "test_value".to_string());

        assert_eq!(
            msg.base.additional_kwargs.get("test_key"),
            Some(&"test_value".to_string())
        );
    }

    #[test]
    fn test_message_equality() {
        let msg1 = HumanMessage::new("Test message");
        let msg2 = HumanMessage::new("Test message");

        assert_eq!(msg1.content(), msg2.content());
        assert_eq!(msg1.message_type(), msg2.message_type());
    }

    #[test]
    fn test_struct_macro_implementation() {
        let msg = HumanMessage::new("Testing from the macro");

        assert_eq!(msg.content(), "Testing from the macro");
        assert_eq!(msg.message_type(), &MessageType::Human);
    }

    #[test]
    fn test_fully_qualified_human_message_getters() {
        define_message!(MessageType::Human);
        let mut human_message = HumanMessage::new("Hello, Human!");

        assert_eq!(human_message.content(), "Hello, Human!");
        assert!(!human_message.is_example());
        assert!(human_message.additional_kwargs().is_empty());
        assert!(human_message.response_metadata().is_empty());
        assert_eq!(human_message.id(), None);
        assert_eq!(human_message.name(), None);
        assert_eq!(human_message.message_type(), &MessageType::Human);

        human_message.base.example = true;
        human_message
            .base
            .additional_kwargs
            .insert("key".to_string(), "value".to_string());
        human_message
            .base
            .response_metadata
            .insert("metadata_key".to_string(), "metadata_value".to_string());
        human_message.base.id = Some("12345".to_string());
        human_message.base.name = Some("Test User".to_string());

        assert!(human_message.is_example());
        assert_eq!(
            human_message.additional_kwargs().get("key"),
            Some(&"value".to_string())
        );
        assert_eq!(
            human_message.response_metadata().get("metadata_key"),
            Some(&"metadata_value".to_string())
        );
        assert_eq!(human_message.id(), Some("12345"));
        assert_eq!(human_message.name(), Some("Test User"));
    }
}
