use messageforge::prelude::*;
// use messageforge::tool_message::ToolStatus;

#[test]
fn test_message_integration() {
    let ai_msg = AiMessage::new("This is an AI response");
    assert_eq!(ai_msg.content(), "This is an AI response");
    assert!(!ai_msg.is_example());
    assert_eq!(ai_msg.message_type(), &MessageType::Ai);

    let ai_msg_debug_output = format!("{:?}", ai_msg);
    let expected_ai_msg_debug = r#"AiMessage { base: BaseMessageFields { content: "This is an AI response", example: false, message_type: Ai, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
    assert_eq!(ai_msg_debug_output, expected_ai_msg_debug);

    let chat_msg = ChatMessage::new("Hello from Chat!", "User".to_string());
    assert_eq!(chat_msg.content(), "Hello from Chat!");
    assert_eq!(chat_msg.role(), "User");
    assert_eq!(chat_msg.message_type(), &MessageType::Chat);

    let chat_msg_debug_output = format!("{:?}", chat_msg);
    let expected_chat_msg_debug = r#"ChatMessage { role: "User", base: BaseMessageFields { content: "Hello from Chat!", example: false, message_type: Chat, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
    assert_eq!(chat_msg_debug_output, expected_chat_msg_debug);

    let human_msg = HumanMessage::new("This is a human message");
    assert_eq!(human_msg.content(), "This is a human message");
    assert!(!human_msg.is_example());
    assert_eq!(human_msg.message_type(), &MessageType::Human);

    let human_msg_debug_output = format!("{:?}", human_msg);
    let expected_human_msg_debug = r#"HumanMessage { base: BaseMessageFields { content: "This is a human message", example: false, message_type: Human, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
    assert_eq!(human_msg_debug_output, expected_human_msg_debug);

    let system_msg = SystemMessage::new("System message content");
    assert_eq!(system_msg.content(), "System message content");
    assert!(!system_msg.is_example());
    assert_eq!(system_msg.message_type(), &MessageType::System);

    let system_msg_debug_output = format!("{:?}", system_msg);
    let expected_system_msg_debug = r#"SystemMessage { base: BaseMessageFields { content: "System message content", example: false, message_type: System, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
    assert_eq!(system_msg_debug_output, expected_system_msg_debug);

    // let tool_msg = ToolMessage::new(
    //     "This is a tool message",
    //     "call_123".to_string(),
    //     Some("artifact_abc".to_string()),
    //     ToolStatus::Success,
    // );
    // assert_eq!(tool_msg.content(), "This is a tool message");
    // assert_eq!(tool_msg.tool_call_id(), "call_123");
    // assert_eq!(tool_msg.artifact(), &Some("artifact_abc".to_string()));
    // assert_eq!(tool_msg.status(), &ToolStatus::Success);
    // assert!(!tool_msg.is_example());
    // assert_eq!(tool_msg.message_type(), &MessageType::Tool);

    // let tool_msg_debug_output = format!("{:?}", tool_msg);
    // let expected_tool_msg_debug = r#"ToolMessage { tool_call_id: "call_123", artifact: Some("artifact_abc"), status: Success, base: BaseMessageFields { content: "This is a tool message", example: false, message_type: Tool, additional_kwargs: {}, response_metadata: {}, id: None, name: None } }"#;
    // assert_eq!(tool_msg_debug_output, expected_tool_msg_debug);
}
