extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn concat_idents(input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", input);
    TokenStream::new()
}
