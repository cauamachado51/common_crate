use procedural::command;

fn main() {
	command!("pwsh", "-File", "tests/command.ps1");
	println!("{}", data()); // ex.: 02/02/2026 05:31:14

	let a = command!("target/debug/meu_bin.exe");
	println!("{a}");
}
