use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[inline(always)]
pub fn stack_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let bytes = input.value().into_bytes();
    let expanded = quote! { [#(#bytes),*] };
    expanded.into()
}