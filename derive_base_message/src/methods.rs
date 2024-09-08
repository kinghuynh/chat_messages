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
}
