use chat_messages::{BaseMessage, BaseMessageFields, MessageType};
use derive_base_message::BaseMessage;

#[derive(BaseMessage)]
pub struct HumanMessage {
    pub role: String,
    pub base: BaseMessageFields,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_message_new_method() {
        let msg = HumanMessage::new("Hello, world!", "Admin".to_string());

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
        let msg = HumanMessage::new("Hello, world!", "User".to_string());
        assert_eq!(msg.content(), "Hello, world!");
        assert_eq!(msg.message_type(), MessageType::Human);
    }
}
