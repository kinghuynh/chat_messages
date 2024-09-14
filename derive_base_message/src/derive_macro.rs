use crate::fields::{extract_fields, field_args, field_initializers};
use crate::methods::{implement_base_getters, implement_base_setters};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Ident};

fn implement_struct_new(input: &DeriveInput) -> Result<TokenStream2, Error> {
    let named_fields = extract_fields(input)?;
    let field_args = field_args(named_fields, &["base"]);
    let field_initializers = field_initializers(named_fields, &["base"]);

    Ok(quote! {
        pub fn new(content: &str, #field_args) -> Self {
            Self::new_with_example(content, false, #field_initializers)
        }

        pub fn new_with_example(content: &str, example: bool, #field_args) -> Self {
            Self {
                base: BaseMessageFields {
                    content: content.to_string(),
                    example,
                    additional_kwargs: std::collections::HashMap::new(),
                    response_metadata: std::collections::HashMap::new(),
                    id: None,
                    name: None,
                },
                #field_initializers
            }
        }
    })
}

fn extract_message_type_name(input: &DeriveInput) -> Ident {
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    let message_type_str = struct_name_str
        .strip_suffix("Message")
        .unwrap_or(&struct_name_str);
    format_ident!("{}", message_type_str)
}

fn implement_base_message(input: &DeriveInput) -> TokenStream2 {
    let struct_name = &input.ident;
    let message_type_name = extract_message_type_name(input);

    quote! {
        impl BaseMessage for #struct_name {
            fn content(&self) -> &str {
                &self.base.content
            }

            fn message_type(&self) -> MessageType {
                MessageType::#message_type_name
            }

            fn role(&self) -> &str {
                MessageType::#message_type_name.as_str()
            }
        }
    }
}

pub fn derive_macro(input: TokenStream2) -> TokenStream2 {
    let ast: DeriveInput = match syn::parse2(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    let struct_name = &ast.ident;

    let struct_new_impl = match implement_struct_new(&ast) {
        Ok(impl_code) => impl_code,
        Err(err) => return err.to_compile_error(),
    };

    let base_getters = implement_base_getters();
    let base_setters = implement_base_setters();
    let base_message_impl = implement_base_message(&ast);

    quote! {
        impl #struct_name {
            #struct_new_impl
            #base_getters
            #base_setters
        }
        #base_message_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_derive_macro_generates_correct_code() {
        let input: DeriveInput = parse_quote! {
            struct HumanMessage {
                role: String,
                base: BaseMessageFields,
            }
        };

        let generated = derive_macro(quote! { #input });

        let expected = quote! {
            impl HumanMessage {
                pub fn new(content: &str, role: String) -> Self {
                    Self::new_with_example(content, false, role)
                }

                pub fn new_with_example(content: &str, example: bool, role: String) -> Self {
                    Self {
                        base: BaseMessageFields {
                            content: content.to_string(),
                            example,
                            additional_kwargs: std::collections::HashMap::new(),
                            response_metadata: std::collections::HashMap::new(),
                            id: None,
                            name: None,
                        },
                        role
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

            impl BaseMessage for HumanMessage {
                fn content(&self) -> &str {
                    &self.base.content
                }

                fn message_type(&self) -> MessageType {
                    MessageType::Human
                }

                fn role(&self) -> &str {
                    MessageType::Human.as_str()
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }
}
