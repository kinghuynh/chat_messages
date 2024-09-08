extern crate proc_macro;
use crate::fields::{extract_fields, field_args, field_initializers};
use crate::methods::implement_base_getters;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Error};

fn implement_struct_new(input: &DeriveInput) -> Result<TokenStream2, Error> {
    let named_fields = extract_fields(input)?;
    let field_args = field_args(named_fields, &["base"]);
    let field_initializers = field_initializers(named_fields, &["base"]);

    Ok(quote! {
        pub fn new(content: &str, #field_args) -> Self {
            Self {
                base: BaseMessageFields {
                    content: content.to_string(),
                    example: false,
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

fn implement_base_message(input: &DeriveInput) -> TokenStream2 {
    let struct_name = &input.ident;

    quote! {
        impl BaseMessage for #struct_name {
            fn content(&self) -> &str {
                &self.base.content
            }

            fn message_type(&self) -> MessageType {
                MessageType::#struct_name
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
    let base_message_impl = implement_base_message(&ast);

    quote! {
        impl #struct_name {
            #struct_new_impl
            #base_getters
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
        // Step 1: Create a mock input struct using syn's parse_quote
        let input: DeriveInput = parse_quote! {
            struct HumanMessage {
                role: String,
                base: BaseMessageFields,
            }
        };

        // Step 2: Call the `derive_macro` function to generate the code
        let generated = derive_macro(quote! { #input });

        // Step 3: Define the expected output TokenStream
        let expected = quote! {
            impl HumanMessage {
                pub fn new(content: &str, role: String) -> Self {
                    Self {
                        base: BaseMessageFields {
                            content: content.to_string(),
                            example: false,
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
            }

            impl BaseMessage for HumanMessage {
                fn content(&self) -> &str {
                    &self.base.content
                }

                fn message_type(&self) -> MessageType {
                    MessageType::HumanMessage
                }
            }
        };

        // Step 4: Compare the generated output with the expected output
        assert_eq!(generated.to_string(), expected.to_string());
    }
}
