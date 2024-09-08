mod field;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(BaseMessage)]
pub fn derive_base_message(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let _name = &input.ident;

    let expanded = quote! { "" };
    expanded.into()
}
