use messageforge::prelude::*;

#[test]
fn test_message_integration() {
    let ai_msg = AiMessage::new("This is an AI response");
    assert_eq!(ai_msg.content(), "This is an AI response");
    assert!(!ai_msg.is_example());
    assert_eq!(ai_msg.message_type(), MessageType::Ai);
    println!("AiMessage content: {}", ai_msg.content());

    let chat_msg = ChatMessage::new("Hello from Chat!", "User".to_string());
    assert_eq!(chat_msg.content(), "Hello from Chat!");
    assert_eq!(chat_msg.role(), "User");
    assert_eq!(chat_msg.message_type(), MessageType::Chat);
    println!("ChatMessage content: {}", chat_msg.content());

    let human_msg = HumanMessage::new("This is a human message");
    assert_eq!(human_msg.content(), "This is a human message");
    assert!(!human_msg.is_example());
    assert_eq!(human_msg.message_type(), MessageType::Human);
    println!("HumanMessage content: {}", human_msg.content());

    let system_msg = SystemMessage::new("System message content");
    assert_eq!(system_msg.content(), "System message content");
    assert!(!system_msg.is_example());
    assert_eq!(system_msg.message_type(), MessageType::System);
    println!("SystemMessage content: {}", system_msg.content());
}
