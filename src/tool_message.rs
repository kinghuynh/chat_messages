// use crate::prelude::*;
// use derive_base_message::BaseMessage;

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub enum ToolStatus {
//     Success,
//     Error,
// }

// #[derive(BaseMessage, Debug, Serialize, Deserialize)]
// pub struct ToolMessage {
//     tool_call_id: String,
//     artifact: Option<String>,
//     status: ToolStatus,
//     #[serde(flatten)]
//     base: BaseMessageFields,
// }

// impl ToolMessage {
//     pub fn tool_call_id(&self) -> &str {
//         &self.tool_call_id
//     }

//     pub fn artifact(&self) -> &Option<String> {
//         &self.artifact
//     }

//     pub fn status(&self) -> &ToolStatus {
//         &self.status
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json;
//     use std::collections::HashMap;

//     #[test]
//     fn test_tool_message_serialization_with_empty_fields() {
//         let tool_message = ToolMessage::new(
//             "Test message",
//             "call_123".to_string(),
//             None,
//             ToolStatus::Success,
//         );

//         let serialized = serde_json::to_string(&tool_message).expect("Serialization failed");

//         let expected = r#"{"tool_call_id":"call_123","artifact":null,"status":"Success","content":"Test message","example":false,"message_type":"Tool"}"#;
//         assert_eq!(serialized, expected);
//     }

//     #[test]
//     fn test_tool_message_serialization_with_values() {
//         let mut additional_kwargs = HashMap::new();
//         additional_kwargs.insert("key1".to_string(), "value1".to_string());

//         let mut response_metadata = HashMap::new();
//         response_metadata.insert("meta_key".to_string(), "meta_value".to_string());

//         let mut tool_message = ToolMessage::new(
//             "Test message",
//             "call_123".to_string(),
//             Some("artifact_abc".to_string()),
//             ToolStatus::Error,
//         );

//         tool_message.base.example = true;
//         tool_message.base.additional_kwargs = additional_kwargs.clone();
//         tool_message.base.response_metadata = response_metadata.clone();
//         tool_message.base.id = Some("1234".to_string());
//         tool_message.base.name = Some("Tool Name".to_string());

//         let serialized = serde_json::to_string(&tool_message).expect("Serialization failed");

//         let expected = r#"{"tool_call_id":"call_123","artifact":"artifact_abc","status":"Error","content":"Test message","example":true,"message_type":"Tool","additional_kwargs":{"key1":"value1"},"response_metadata":{"meta_key":"meta_value"},"id":"1234","name":"Tool Name"}"#;
//         assert_eq!(serialized, expected);
//     }

//     #[test]
//     fn test_tool_message_deserialization() {
//         let json_data = r#"
//         {
//             "tool_call_id": "call_123",
//             "artifact": "artifact_abc",
//             "status": "Success",
//             "content": "Test message",
//             "example": true,
//             "message_type": "Tool",
//             "additional_kwargs": {
//                 "key1": "value1"
//             },
//             "response_metadata": {
//                 "meta_key": "meta_value"
//             },
//             "id": "1234",
//             "name": "Tool Name"
//         }
//         "#;

//         let tool_message: ToolMessage =
//             serde_json::from_str(json_data).expect("Deserialization failed");

//         assert_eq!(tool_message.tool_call_id, "call_123");
//         assert_eq!(tool_message.artifact.as_deref(), Some("artifact_abc"));
//         assert_eq!(tool_message.status, ToolStatus::Success);
//         assert_eq!(tool_message.base.content, "Test message");
//         assert!(tool_message.base.example);
//         assert_eq!(tool_message.base.message_type, MessageType::Tool);
//         assert_eq!(
//             tool_message.base.additional_kwargs.get("key1").unwrap(),
//             "value1"
//         );
//         assert_eq!(
//             tool_message.base.response_metadata.get("meta_key").unwrap(),
//             "meta_value"
//         );
//         assert_eq!(tool_message.base.id.as_deref(), Some("1234"));
//         assert_eq!(tool_message.base.name.as_deref(), Some("Tool Name"));
//     }

//     #[test]
//     fn test_tool_message_serialization_with_partial_values() {
//         let mut additional_kwargs = HashMap::new();
//         additional_kwargs.insert("key2".to_string(), "value2".to_string());

//         let mut tool_message = ToolMessage::new(
//             "Partial message",
//             "call_456".to_string(),
//             None,
//             ToolStatus::Error,
//         );

//         tool_message.base.additional_kwargs = additional_kwargs.clone();
//         tool_message.base.id = Some("5678".to_string());

//         let serialized = serde_json::to_string(&tool_message).expect("Serialization failed");

//         let expected = r#"{"tool_call_id":"call_456","artifact":null,"status":"Error","content":"Partial message","example":false,"message_type":"Tool","additional_kwargs":{"key2":"value2"},"id":"5678"}"#;
//         assert_eq!(serialized, expected);
//     }
// }
