extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

mod data;

#[proc_macro]
pub fn make_styles(input: TokenStream) -> TokenStream {
    let _style = syn::parse_macro_input!(input as data::Style);

    // Generate the code using quote!()
    // TODO
    let expanded = quote!();

    // Convert the generated code to a TokenStream
    TokenStream::from(expanded)
}
