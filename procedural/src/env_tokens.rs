use proc_macro::TokenStream;
use std::env;
use std::str::FromStr;
use syn::{parse_macro_input, LitStr};

#[inline(always)]
pub fn env_tokens(input: TokenStream) -> TokenStream {
    let key = parse_macro_input!(input as LitStr).value();
    let codigo = env::var(&key).unwrap();
    TokenStream::from_str(&codigo).unwrap()
}