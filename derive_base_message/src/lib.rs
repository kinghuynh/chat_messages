mod derive_macro;
mod fields;
mod methods;

use derive_macro::derive_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(BaseMessage)]
pub fn derive_base_message(input: TokenStream) -> TokenStream {
    derive_macro(input.into()).into()
}
