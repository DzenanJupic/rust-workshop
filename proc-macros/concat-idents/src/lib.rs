extern crate proc_macro;

use proc_macro::TokenStream;

use syn::parse::ParseStream;

#[derive(Debug)]
struct InputParser {}

impl syn::parse::Parse for InputParser {
    fn parse(_: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

impl InputParser {
    fn into_token_stream(self) -> TokenStream {
        TokenStream::new()
    }
}


// fn_ident = $fn, _, $struct {
//     #[test]
//     fn fn_ident() {
//         // -- snip --
//     }
// }
#[proc_macro]
pub fn concat_idents(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as InputParser);
    eprintln!("{:#?}", input);
    TokenStream::new()
}
