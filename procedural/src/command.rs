use std::{process::Command, str::FromStr};
use proc_macro::TokenStream;
use syn::{
	LitStr, Token, parse_macro_input, punctuated::Punctuated,
	parse::{Parse, ParseStream}
};

struct CommandArgs {
	program: String,
	args: Vec<String>,
}

impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut iter = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?.into_iter();
	
		let program = match iter.next() {
			Some(lit) => lit.value(),
			None => return Err(input.error("Requer um programa")),
		};
	
		let args = iter.map(|lit| lit.value()).collect();
	
		Ok(CommandArgs { program, args })
	}
}

#[inline(always)]
pub fn command(input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(input as CommandArgs);

	let output = match Command::new(&args.program).args(&args.args).output() {
		Ok(o) => o,
		Err(e) => panic!("{e}"),
	};

	let stdout = String::from_utf8(output.stdout).expect("O comando retornou não UTF-8");

	TokenStream::from_str(&stdout).unwrap()
}