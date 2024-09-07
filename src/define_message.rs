#[macro_export]
macro_rules! define_message {
    ($message_type:ident, $message_enum_str:expr) => {
        pub struct $message_type {
            pub base: BaseMessageFields,
        }

        impl $message_type {
            pub fn new(content: &str) -> Self {
                Self {
                    base: BaseMessageFields {
                        content: content.to_string(),
                        example: false,
                        additional_kwargs: HashMap::new(),
                        response_metadata: HashMap::new(),
                        id: None,
                        name: None,
                    },
                }
            }

            pub fn content(&self) -> &str {
                &self.base.content
            }

            pub fn message_type(&self) -> Result<MessageType, InvalidMessageTypeError> {
                MessageType::try_from($message_enum_str)
            }
        }
    };
}
