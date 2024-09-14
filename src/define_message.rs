#[macro_export]
macro_rules! define_message {
    (MessageType::$message_type_enum:ident) => {
        define_message!($message_type_enum);
    };

    ($message_type_enum:expr) => {
        paste::item! {
            #[derive(Debug, Deserialize)]
            pub struct [<$message_type_enum Message>] {
                #[serde(flatten)]
                pub base: BaseMessageFields,
            }

            impl [<$message_type_enum Message>] {
                pub fn new(content: &str) -> Self {
                    Self::new_with_example(content, false)
                }

                pub fn new_with_example(content: &str, example: bool) -> Self {
                    Self {
                        base: BaseMessageFields {
                            content: content.to_string(),
                            example,
                            additional_kwargs: std::collections::HashMap::new(),
                            response_metadata: std::collections::HashMap::new(),
                            id: None,
                            name: None,
                        }
                    }
                }

                pub fn is_example(&self) -> bool {
                    self.base.example
                }

                pub fn additional_kwargs(&self) -> &std::collections::HashMap<String, String> {
                    &self.base.additional_kwargs
                }

                pub fn response_metadata(&self) -> &std::collections::HashMap<String, String> {
                    &self.base.response_metadata
                }

                pub fn id(&self) -> Option<&str> {
                    self.base.id.as_deref()
                }

                pub fn name(&self) -> Option<&str> {
                    self.base.name.as_deref()
                }

                pub fn set_content(&mut self, new_content: &str) {
                    self.base.content = new_content.to_string();
                }

                pub fn set_example(&mut self, example: bool) {
                    self.base.example = example;
                }

                pub fn set_id(&mut self, id: Option<String>) {
                    self.base.id = id;
                }

                pub fn set_name(&mut self, name: Option<String>) {
                    self.base.name = name;
                }
            }

            impl BaseMessage for [<$message_type_enum Message>] {
                fn content(&self) -> &str {
                    &self.base.content
                }

                fn message_type(&self) -> MessageType {
                    $message_type_enum
                }

                fn role(&self) -> &str {
                    $message_type_enum.as_str()
                }
            }

            impl Serialize for [<$message_type_enum Message>] {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut state = serde_json::Map::new();
                    state.insert("role".to_string(), serde_json::Value::String(self.role().to_string()));
                    let base_json = serde_json::to_value(&self.base).map_err(serde::ser::Error::custom)?;
                    if let serde_json::Value::Object(map) = base_json {
                        state.extend(map);
                    }
                    serde_json::Value::Object(state).serialize(serializer)
                }
            }
        }
    };
}
