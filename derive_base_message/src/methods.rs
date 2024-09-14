use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn implement_base_getters() -> TokenStream2 {
    quote! {
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
}

pub fn implement_base_setters() -> TokenStream2 {
    quote! {
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
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::methods::implement_base_getters;

    #[test]
    fn test_implement_base_methods() {
        let generated = implement_base_getters();

        let expected = quote! {
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
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_implement_base_setters() {
        let generated = super::implement_base_setters();

        let expected = quote! {
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
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }
}
