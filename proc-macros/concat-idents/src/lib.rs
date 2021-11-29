extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{Block, Ident, LitBool, LitChar, LitInt, LitStr, Token};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Underscore;

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
        let IdentParser(concatenated_ident) = stream.parse()?;
        let block: Block = stream.parse()?;

        Ok(Self { replace_ident, concatenated_ident, block })
    }
}

impl InputParser {
    fn into_token_stream(self) -> TokenStream {
        TokenStream::new()
    }
}

struct IdentParser(Ident);

impl syn::parse::Parse for IdentParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident_parts = Vec::new();

        while !input.peek(syn::token::Brace) {
            let part: IdentPart = input.parse()?;
            ident_parts.push(part);


            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        let ident = ident_parts
            .into_iter()
            .map(|part| {
                match part {
                    IdentPart::Underscore(_) => "_".to_owned(),
                    IdentPart::Ident(ident) => ident.to_string(),
                    _ => todo!()
                }
            })
            .collect::<String>();
        Ok(Self(Ident::new(&ident, Span::call_site())))
    }
}

#[derive(Debug)]
enum IdentPart {
    Underscore(Underscore),
    Ident(Ident),
    Int(LitInt),
    Bool(LitBool),
    Str(LitStr),
    Char(LitChar),
}

impl syn::parse::Parse for IdentPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if input.peek(Token![_]) {
            Ok(Self::Underscore(input.parse()?))
        } else {
            todo!();
        }
    }
}


// fn_ident = $fn, _, $struct, {
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
