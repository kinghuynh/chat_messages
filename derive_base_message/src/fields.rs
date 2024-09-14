use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Field, FieldsNamed, Type};

pub fn extract_fields(input: &DeriveInput) -> Result<&FieldsNamed, Error> {
    match input.data {
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Named(ref named),
            ..
        }) => Ok(named),
        _ => Err(Error::new_spanned(
            input,
            "Unsupported data type: expected struct with named fields",
        )),
    }
}

pub fn field_name_and_type(field: &Field) -> (&Option<Ident>, &Type) {
    let name = &field.ident;
    let ty = &field.ty;
    (name, ty)
}

fn is_excluded(name: &Option<Ident>, excludes: &[&str]) -> bool {
    name.as_ref()
        .map_or(false, |n| excludes.contains(&n.to_string().as_str()))
}

pub fn field_args(fields: &FieldsNamed, excludes: &[&str]) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(field_name_and_type)
        .filter(|(name, _)| !is_excluded(name, excludes))
        .map(|(name, ty)| {
            let name = name.as_ref().unwrap();
            quote! { #name: #ty }
        })
        .collect()
}

pub fn field_initializers(
    fields: &FieldsNamed,
    excludes: &[&str],
) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(field_name_and_type)
        .filter(|(name, _)| !is_excluded(name, excludes))
        .map(|(name, _)| {
            let name = name.as_ref().unwrap();
            quote! { #name }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    // Test: Valid struct with named fields
    #[test]
    fn test_extract_fields_valid_struct() {
        let input: DeriveInput = parse_quote! {
            struct ChatMessage {
                role: String,
                base: BaseMessageFields,
            }
        };

        let result = extract_fields(&input);
        assert!(result.is_ok());

        let fields = result.unwrap();
        let field_names: Vec<String> = fields
            .named
            .iter()
            .map(|f| f.ident.as_ref().unwrap().to_string())
            .collect();
        assert_eq!(field_names, vec!["role", "base"]);
    }

    // Test: Input is not a struct (e.g., enum)
    #[test]
    fn test_extract_fields_non_struct() {
        let input: DeriveInput = parse_quote! {
            enum ChatMessage {
                A,
                B,
            }
        };

        let result = extract_fields(&input);
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Unsupported data type: expected struct with named fields"
            );
        } else {
            panic!("Expected an error but got a successful result.");
        }
    }

    #[test]
    fn test_extract_fields_tuple_struct() {
        let input: DeriveInput = parse_quote! {
            struct ChatMessage(String, BaseMessageFields);
        };

        let result = extract_fields(&input);
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Unsupported data type: expected struct with named fields"
            );
        } else {
            panic!("Expected an error but got a successful result.");
        }
    }

    #[test]
    fn test_field_args_no_excludes() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_args(fields, &[]); // No excludes
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field1: String,
            field2: u32
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_args_single_exclude() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_args(fields, &["field1"]); // Exclude "field1"
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field2: u32
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_args_multiple_excludes() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
                field3: bool,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_args(fields, &["field1", "field3"]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field2: u32
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_args_empty_struct() {
        let input: DeriveInput = parse_quote! {
            struct EmptyStruct {}
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_args(fields, &[]); // No excludes
        let result = quote! { #(#result_vec),* };
        let expected = quote! {};

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_initializers_no_excludes() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_initializers(fields, &[]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field1, field2
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_args_with_optional_fields() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
                optional_field: Option<String>,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_args(fields, &[]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field1: String,
            field2: u32,
            optional_field: Option<String>
        };

        assert_eq!(result.to_string(), expected.to_string());
        let result_vec = field_args(fields, &["field2"]);
        let result = quote! { #(#result_vec),* };
        let expected_with_exclusion = quote! {
            field1: String,
            optional_field: Option<String>
        };

        assert_eq!(result.to_string(), expected_with_exclusion.to_string());
    }

    #[test]
    fn test_field_initializers_single_exclude() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_initializers(fields, &["field1"]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field2
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_initializers_multiple_excludes() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
                field3: bool,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_initializers(fields, &["field1", "field3"]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field2
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_initializers_empty_struct() {
        let input: DeriveInput = parse_quote! {
            struct EmptyStruct {}
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_initializers(fields, &[]);
        let result = quote! { #(#result_vec),* };
        let expected = quote! {};

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_field_initializers_with_optional_fields() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                field1: String,
                field2: u32,
                optional_field: Option<String>,
            }
        };

        let fields = extract_fields(&input).unwrap();
        let result_vec = field_initializers(fields, &[]);
        let result = quote! { #(#result_vec),* };

        let expected = quote! {
            field1,
            field2,
            optional_field
        };

        assert_eq!(result.to_string(), expected.to_string());
        let result_vec = field_initializers(fields, &["field2"]);
        let result = quote! { #(#result_vec),* };

        let expected_with_exclusion = quote! {
            field1,
            optional_field
        };

        assert_eq!(result.to_string(), expected_with_exclusion.to_string());
    }
}
