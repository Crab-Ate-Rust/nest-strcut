extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dsl(_arg: TokenStream, input: TokenStream) -> TokenStream {
    print!("{}", input.to_string());
    // let expand = quote! {};
    // expand.parse().unwrap()
    input
}

