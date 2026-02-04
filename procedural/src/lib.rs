mod command;
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