#[macro_export]
macro_rules! define_message {
    (MessageType::$message_type_enum:ident) => {
        define_message!($message_type_enum);
    };

    ($message_type_enum:expr) => {
        paste::item! {
            pub struct [<$message_type_enum Message>] {
                pub base: BaseMessageFields,
            }

            impl [<$message_type_enum Message>] {
                pub fn new(content: &str) -> Self {
                    Self {
                        base: BaseMessageFields {
                            content: content.to_string(),
                            example: false,
                            additional_kwargs: std::collections::HashMap::new(),
                            response_metadata: std::collections::HashMap::new(),
                            id: None,
                            name: None,
                        }
                    }
                }

                pub fn content(&self) -> &str {
                    &self.base.content
                }

                pub fn message_type(&self) -> MessageType {
                    $message_type_enum
                }
            }

            impl BaseMessage for [<$message_type_enum Message>] {
                fn content(&self) -> &str {
                    &self.base.content
                }

                fn message_type(&self) -> MessageType {
                    $message_type_enum
                }
            }
        }
    };
}
