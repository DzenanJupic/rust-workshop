extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use syn::{Block, Token};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;

#[derive(Debug)]
struct InputParser {
    replace_ident: Ident,
    concatenated_ident: Ident,
    block: Block,
}

impl syn::parse::Parse for InputParser {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let replace_ident: Ident = stream.parse()?;
        let _: Token![=] = stream.parse()?;
        let idents: _ = todo!();
        let block: Block = stream.parse()?;

        let concatenated_ident = idents
            .into_iter()
            .map(|ident| ident.to_string())
            .collect::<String>();
        let concatenated_ident = Ident::new(&concatenated_ident, Span::call_site());

        Ok(Self { replace_ident, concatenated_ident, block })
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
