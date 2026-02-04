/// Exibe texto diretamente no terminal, sem usar canais como stdout e stderr.
/// ### Exemplo
/// ```
/// use common_crate::console::iprintln;
/// iprintln!("texto exibido"); // direto no terminal
/// println!("texto retornado"); // stdout
/// ```
/// veja:
/// ```PowerShell
/// PS > $var = cargo run
/// texto exibido
/// PS > $var
/// texto retornado
/// PS > cargo run *> $null # redirecionar canais para o vazio
/// texto exibido
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! iprintln {
	($($arg:tt)*) => {{
		use std::fs::OpenOptions;
		use std::io::Write;

		#[cfg(windows)]
		let device = "CONOUT$";
		#[cfg(not(windows))]
		let device = "/dev/tty";

		if let Ok(mut console) = OpenOptions::new().write(true).open(device) {
			let _ = writeln!(console, $($arg)*);
		}
	}};
}

/// Exibe texto diretamente no terminal, sem usar canais como stdout e stderr.
/// ### Exemplo
/// ```
/// use common_crate::console::iprint;
/// iprint!("texto exibido"); // direto no terminal
/// println!("texto retornado"); // stdout
/// ```
/// veja:
/// ```PowerShell
/// PS > $var = cargo run
/// texto exibido
/// PS > $var
/// texto retornado
/// PS > cargo run *> $null # redirecionar canais para o vazio
/// texto exibido
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! iprint {
	($($arg:tt)*) => {{
		use std::fs::OpenOptions;
		use std::io::Write;

		#[cfg(windows)]
		let device = "CONOUT$";
		#[cfg(not(windows))]
		let device = "/dev/tty";

		if let Ok(mut console) = OpenOptions::new().write(true).open(device) {
			let _ = write!(console, $($arg)*);
		}
	}};
}