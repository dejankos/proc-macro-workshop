extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);

    TokenStream::default()
    //unimplemented!()
}
