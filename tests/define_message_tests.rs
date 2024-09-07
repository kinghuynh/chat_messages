#[cfg(test)]
mod tests {
    use chat_messages::{define_message, BaseMessage, BaseMessageFields, MessageType};

    use MessageType::*;

    define_message!(Human);
    define_message!(AI);
    define_message!(System);

    #[test]
    fn test_human_message_creation() {
        let msg = HumanMessage::new("Hello, Human!");

        // Check content
        assert_eq!(msg.content(), "Hello, Human!");

        // Check message type
        assert_eq!(msg.message_type(), MessageType::Human);

        // Check default fields
        assert!(!msg.base.example);
        assert!(msg.base.additional_kwargs.is_empty());
        assert!(msg.base.response_metadata.is_empty());
        assert!(msg.base.id.is_none());
        assert!(msg.base.name.is_none());
    }

    #[test]
    fn test_ai_message_creation() {
        let msg = AIMessage::new("Hello, AI!");

        // Check content
        assert_eq!(msg.content(), "Hello, AI!");

        // Check message type
        assert_eq!(msg.message_type(), MessageType::AI);
    }

    #[test]
    fn test_system_message_creation() {
        let msg = SystemMessage::new("System initialized!");

        // Check content
        assert_eq!(msg.content(), "System initialized!");

        // Check message type
        assert_eq!(msg.message_type(), MessageType::System);
    }

    #[test]
    fn test_custom_fields() {
        let mut msg = HumanMessage::new("Hello, World!");

        // Set some custom values for fields
        msg.base.example = true;
        msg.base
            .additional_kwargs
            .insert("key".to_string(), "value".to_string());
        msg.base
            .response_metadata
            .insert("token_count".to_string(), "42".to_string());
        msg.base.id = Some("12345".to_string());
        msg.base.name = Some("User".to_string());

        // Check custom values
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

        // Ensure content is empty
        assert_eq!(msg.content(), "");
        assert_eq!(msg.message_type(), MessageType::Human);
    }

    #[test]
    fn test_multiple_message_types() {
        let human_msg = HumanMessage::new("Hello, Human!");
        let ai_msg = AIMessage::new("Hello, AI!");
        let system_msg = SystemMessage::new("System initiated");

        // Check all message types and contents
        assert_eq!(human_msg.message_type(), MessageType::Human);
        assert_eq!(human_msg.content(), "Hello, Human!");

        assert_eq!(ai_msg.message_type(), MessageType::AI);
        assert_eq!(ai_msg.content(), "Hello, AI!");

        assert_eq!(system_msg.message_type(), MessageType::System);
        assert_eq!(system_msg.content(), "System initiated");
    }

    #[test]
    fn test_default_additional_kwargs() {
        let mut msg = AIMessage::new("AI Test");

        // Ensure that additional_kwargs is empty by default
        assert!(msg.base.additional_kwargs.is_empty());

        // Add a value to additional_kwargs
        msg.base
            .additional_kwargs
            .insert("test_key".to_string(), "test_value".to_string());

        // Ensure that the value is correctly inserted
        assert_eq!(
            msg.base.additional_kwargs.get("test_key"),
            Some(&"test_value".to_string())
        );
    }

    #[test]
    fn test_message_equality() {
        let msg1 = HumanMessage::new("Test message");
        let msg2 = HumanMessage::new("Test message");

        // Ensure that two messages with the same content are equal in content
        assert_eq!(msg1.content(), msg2.content());
        assert_eq!(msg1.message_type(), msg2.message_type());
    }

    #[test]
    fn test_struct_macro_implementation() {
        // Test that the macro correctly defines a struct and its methods
        let msg = HumanMessage::new("Testing from the macro");

        // Ensure the struct has the content and message_type methods
        assert_eq!(msg.content(), "Testing from the macro");
        assert_eq!(msg.message_type(), MessageType::Human);
    }
}
