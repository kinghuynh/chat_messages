#[cfg(test)]
mod tests {
    use chat_messages::prelude::*;

    define_message!(MessageType::Human);

    #[test]
    fn test_human_message_serialization() {
        let mut human_message = HumanMessage::new("Hello, world!");

        // Add some metadata to test serialization of complex fields
        human_message
            .base
            .additional_kwargs
            .insert("key1".to_string(), "value1".to_string());
        human_message
            .base
            .response_metadata
            .insert("meta1".to_string(), "metadata1".to_string());
        human_message.base.id = Some("12345".to_string());
        human_message.base.name = Some("John Doe".to_string());

        // Serialize the message to JSON
        let serialized = serde_json::to_string(&human_message).unwrap();

        // Check if the serialized string contains the expected values
        assert!(serialized.contains("\"content\":\"Hello, world!\""));
        assert!(serialized.contains("\"key1\":\"value1\""));
        assert!(serialized.contains("\"meta1\":\"metadata1\""));
        assert!(serialized.contains("\"id\":\"12345\""));
        assert!(serialized.contains("\"name\":\"John Doe\""));

        // Output the serialized string (optional)
        println!("Serialized HumanMessage: {}", serialized);
    }

    #[test]
    fn test_human_message_deserialization() {
        // Define a JSON string for deserialization testing
        let json_str = r#"
        {
            "base": {
                "content": "Hello, world!",
                "example": false,
                "additional_kwargs": {"key1": "value1"},
                "response_metadata": {"meta1": "metadata1"},
                "id": "12345",
                "name": "John Doe"
            }
        }
        "#;

        // Deserialize the JSON string into a HumanMessage
        let deserialized: HumanMessage = serde_json::from_str(json_str).unwrap();

        // Check the content of the deserialized message
        assert_eq!(deserialized.content(), "Hello, world!");
        assert!(!deserialized.is_example());
        assert_eq!(
            deserialized.additional_kwargs().get("key1"),
            Some(&"value1".to_string())
        );
        assert_eq!(
            deserialized.response_metadata().get("meta1"),
            Some(&"metadata1".to_string())
        );
        assert_eq!(deserialized.id(), Some("12345"));
        assert_eq!(deserialized.name(), Some("John Doe"));

        // Output the deserialized message (optional)
        println!("Deserialized HumanMessage: {:?}", deserialized);
    }

    #[test]
    fn test_empty_fields_serialization() {
        let human_message = HumanMessage::new("Hello, empty fields!");

        // Serialize the message with no additional_kwargs or response_metadata
        let serialized = serde_json::to_string(&human_message).unwrap();

        // Check that additional_kwargs and response_metadata are skipped if empty
        assert!(serialized.contains("\"content\":\"Hello, empty fields!\""));
        assert!(!serialized.contains("additional_kwargs"));
        assert!(!serialized.contains("response_metadata"));

        // Output the serialized string (optional)
        println!("Serialized HumanMessage with empty fields: {}", serialized);
    }

    #[test]
    fn test_empty_fields_deserialization() {
        // Define a JSON string without additional_kwargs or response_metadata
        let json_str = r#"
        {
            "base": {
                "content": "Hello, empty fields!",
                "example": false,
                "id": null,
                "name": null
            }
        }
        "#;

        // Deserialize the JSON string into a HumanMessage
        let deserialized: HumanMessage = serde_json::from_str(json_str).unwrap();

        // Check that fields are correctly deserialized
        assert_eq!(deserialized.content(), "Hello, empty fields!");
        assert!(!deserialized.is_example());
        assert!(deserialized.additional_kwargs().is_empty());
        assert!(deserialized.response_metadata().is_empty());
        assert_eq!(deserialized.id(), None);
        assert_eq!(deserialized.name(), None);

        // Output the deserialized message (optional)
        println!(
            "Deserialized HumanMessage with empty fields: {:?}",
            deserialized
        );
    }
}
