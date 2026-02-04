mod command;
mod stack_string;
mod env_tokens;
use proc_macro::TokenStream;

/// Executa um comando e retorna a saida dele como código rust.
/// ### Exemplo
/// ```
/// use procedural::command;
/// command!("pwsh", "-File", "tests/command.ps1");
/// println!("{}", data()); // ex.: 02/02/2026 05:31:14
/// ```
/// arquivo.ps1:
/// ```PowerShell
/// $data = Get-Date -Format "dd/MM/yyyy HH:mm:ss"
/// Write-Output "pub fn data() -> &'static str { ""$data"" }"
/// ```
/// ### Outros
/// `command!("cargo", "run", "--bin", "meu_bin")` compila eternamente, use: `command!("target/debug/meu_bin.exe")`.<br>
/// `command!("pwsh", "-command", "write-host oi")` sem estar em arquivo, write-host retorna em stdout.
#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
	command::command(input)
}

/// Converte uma string literal em um array de bytes em tempo de compilação.
/// ### Exemplo
/// ```
/// use procedural::stack_string;
/// let stack: [u8; 7] = stack_string!("olá❤"); 
/// assert_eq!(stack, [111, 108, 195, 161, 226, 157, 164]);
/// assert_eq!(str::from_utf8(&stack).unwrap(), "olá❤");
/// ```
/// ### Contexto
/// Em rust, só existe string estatica (que fica "pra sempre" na RAM) ou heap (que precisa de estatica).
#[proc_macro]
pub fn stack_string(input: TokenStream) -> TokenStream {
    stack_string::stack_string(input)
}

/// Retorna o conteudo de uma variavel de ambiente como código rust.
/// ### Exemplo
/// main.rs
/// ```ignore
/// use procedural::env_tokens;
/// env_tokens!("INJECTED_CODE");
/// vindo_do_build()
/// ```
/// build.rs
/// ```ignore
/// let code = "fn vindo_do_build() { println!(\"Isso veio do build.rs\"); }";
/// println!("cargo:rustc-env=INJECTED_CODE={}", code);
/// ```
#[proc_macro]
pub fn env_tokens(input: TokenStream) -> TokenStream {
	env_tokens::env_tokens(input)
}