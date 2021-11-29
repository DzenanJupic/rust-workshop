extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

use log::Level;
use proc_macro2::Span;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{AttributeArgs, Block, Ident, Lit, Meta, NestedMeta, Token};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;

// #[log_attributes::log(input, "{fn} was called with {a} and returned {return}")]
// fn do_calc(a: u32) -> Result<String, String> {
//     // -- do calc --
//
//     todo!()
// }
//
// fn do_calc(a: u32) -> Result<String, String> {
//      let __RET_VALUE__ = {
//          -- do calc --
//
//          todo!()
//      };
//      log::info!(...)
//      __RET_VALUE__
// }


struct Config {
    log_level: log::Level,
    fmt_string: String,
}

impl Config {
    fn new(args: AttributeArgs) -> Self {
        let level = match args[0] {
            NestedMeta::Meta(Meta::Path(ref path)) => path.get_ident().unwrap().to_string(),
            _ => panic!("expected meta")
        };
        let log_level = log::Level::from_str(&level).unwrap();

        let fmt_string = match args[1] {
            NestedMeta::Lit(Lit::Str(ref s)) => s.value(),
            _ => panic!("expected lit str"),
        };


        Self { log_level, fmt_string }
    }
}

struct FmtArgs {
    // a = a,
    // b = b (,)?
    fmt_args: Punctuated<FmtArg, Token![,]>,
    takes_fn_name: bool,
    takes_return_value: bool,
}

struct FmtArg(Ident);

impl FmtArg {
    fn from_str(s: &str) -> Self {
        Self(Ident::new(s, Span::call_site()))
    }
}

impl quote::ToTokens for FmtArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // <ident> = <ident>
        tokens.append(self.0.clone());
        tokens.append(proc_macro2::Punct::new('=', proc_macro2::Spacing::Alone));
        tokens.append(self.0.clone());
    }
}

impl FmtArgs {
    pub fn parse_fmt_args(fmt_string: &str) -> Self {
        let mut fmt_args = Self {
            fmt_args: Punctuated::new(),
            takes_fn_name: false,
            takes_return_value: false,
        };

        let mut in_arg = false;
        let mut skip = false;
        let mut arg = String::new();

        let mut chars = fmt_string.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                '{' if chars.peek() == Some(&'{') => skip = true,
                '{' if skip => {},
                '{' if in_arg => { /* we let the compiler throw the error */ },
                '{' => in_arg = true,

                '}' if chars.peek() == Some(&'}') => {},
                '}' if !in_arg => { /* we let the compiler throw the error */ },
                '}' => {
                    in_arg = false;
                    skip = false;
                    match &*std::mem::take(&mut arg) {
                        "fn" => fmt_args.takes_fn_name = true,
                        "return" => fmt_args.takes_return_value = true,
                        arg => fmt_args.fmt_args.push(FmtArg::from_str(arg)),
                    }
                }

                ':' if in_arg => skip = true,

                char if in_arg && !skip => arg.push(char),
                _ => {},
            }
        }

        fmt_args
    }
}


#[proc_macro_attribute]
pub fn log(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let mut item_fn = syn::parse_macro_input!(item as syn::ItemFn);

    let config = Config::new(attr_args);

    let block = item_fn.block;
    let level = match config.log_level {
        Level::Error => format_ident!("Error"),
        Level::Warn => format_ident!("Warn"),
        Level::Info => format_ident!("Info"),
        Level::Debug => format_ident!("Debug"),
        Level::Trace => format_ident!("Trace"),
    };
    let fmt_string = config.fmt_string;

    let FmtArgs {
        fmt_args,
        takes_fn_name,
        takes_return_value,
    } = FmtArgs::parse_fmt_args(&fmt_string);

    let fn_arg = takes_fn_name.then(|| {
        let ident = &item_fn.sig.ident;
        quote! { fn = ::core::stringify!(#ident), }
    });
    let return_arg = takes_return_value.then(|| {
        let ident = format_ident!("__RET_VALUE__");
        quote! { return = #ident, }
    });

    let block = quote! {{
        let __RET_VALUE__ = #block;
        ::log::log!(
            ::log::Level::#level,
            #fmt_string,
            #fn_arg
            #return_arg
            #fmt_args
        );
    }};

    item_fn.block = Box::new(Block::parse.parse(block.into()).unwrap());
    (quote! { #item_fn }).into()
}
