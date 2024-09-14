#[cfg(test)]
mod tests {
    use derive_base_message::BaseMessage;
    use messageforge::prelude::*;

    #[derive(BaseMessage, Deserialize)]
    pub struct ChatMessage {
        pub role: String,
        pub base: BaseMessageFields,
    }

    #[test]
    fn test_human_message_new_method() {
        let msg = ChatMessage::new("Hello, world!", "Admin".to_string());

        assert_eq!(msg.base.content, "Hello, world!");
        assert_eq!(msg.role, "Admin");
        assert!(!msg.base.example);
        assert!(msg.base.additional_kwargs.is_empty());
        assert!(msg.base.response_metadata.is_empty());
        assert_eq!(msg.base.id, None);
        assert_eq!(msg.base.name, None);
    }

    #[test]
    fn test_human_message_base_message_trait_methods() {
        let msg = ChatMessage::new("Hello, world!", "User".to_string());
        assert_eq!(msg.content(), "Hello, world!");
        assert_eq!(msg.message_type(), &MessageType::Chat);
    }
}
