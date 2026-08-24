/// Abre arquivo, pasta e uri no app padrão.
///
/// suporta:
/// - Windows: se o executavel precisa de admin para executar, abrirá o UAC minimizado. ficar parado no UAC, não para o programa.
/// - Linux e MacOS.
///
/// [shell:algo], [shell:::{CLSID}] e [::{CLSID}].
/// ```no_run
/// use common_crate::fs;
/// fs::open("src");
/// fs::open("Cargo.toml");
/// fs::open("regedit.exe");
/// fs::open("https:youtube.com");
/// fs::open("steam:");
/// fs::open("mailto:alguem@gmail.com");
/// fs::open(r"shell:Personal\WindowsPowerShell");
/// ```
/// ### comparação
/// [`opener::open`](https://docs.rs/opener/0.9.0/opener/fn.open.html) no windows, é 39x mais lento e seu erro:
///
/// pega:
/// - arquivo não existe
///
/// não pega:
/// - arquivo sem permissão de leitura
/// - extensão do arquivo sem app padrão
/// - arquivo bloqueado por processo
///
/// [`ShellExecuteW`](https://docs.rs/windows-sys/latest/windows_sys/Win32/UI/Shell/fn.ShellExecuteW.html) é 5x mais lento.
///
/// [shell:algo]: https://www.tenforums.com/threads/shell-commands-list-for-windows-10.3109
/// [shell:::{CLSID}]: https://winaero.com/clsid-guid-shell-list-windows-10
/// [::{CLSID}]: https://doggy8088.github.io/AutoHotkeyDocs/docs/misc/CLSID-List.htm
#[inline(always)]
pub fn open(the: &str) {
	use std::process::Command;
	#[cfg(target_os = "windows")]
	{
		let _ = Command::new("rundll32.exe").args(["url.dll,FileProtocolHandler", the]).spawn();
	}
	#[cfg(target_os = "linux")]
	{
		let _ = Command::new("xdg-open").arg(the).spawn();
	}
	#[cfg(target_os = "macos")]
	{
		let _ = Command::new("open").arg(the).spawn();
	}
}

/// Abre arquivo, pasta e uri no app que selecionar no dialogo "Abrir com...".
///
/// suporta Windows e Linux.
/// ```no_run
/// use common_crate::fs;
/// #[cfg(target_os = "windows")]
/// {
/// 	let pwd = env::current_dir().unwrap();
/// 	fs::open_with(&pwd.join("src").to_string_lossy()); // pasta relativa abre ..\..\
/// 	fs::open_with(&pwd.join("Cargo.toml").to_string_lossy()); // arquivo relativo tenta abrir na pasta anterior
/// }
/// #[cfg(target_os = "linux")]
/// {
/// 	fs::open_with("src");
/// 	fs::open_with("Cargo.toml");
/// }
/// fs::open_with("https:youtube.com");
/// fs::open_with("steam:");
/// fs::open_with("mailto:alguem@gmail.com");
/// ```
#[inline(always)]
pub fn open_with(the: &str) {
	use std::process::Command;
	#[cfg(target_os = "windows")]
	{
		let _ = Command::new("rundll32.exe").args(["shell32.dll,OpenAs_RunDLL", the]).spawn();
	}
	#[cfg(target_os = "linux")]
	{
		const PORTAL: [&str; 6] = [
			"call", "--session",
			"--dest", "org.freedesktop.portal.Desktop",
			"--object-path", "/org/freedesktop/portal/desktop",
		];
		if std::path::Path::new(the).exists() {
			let _ = Command::new("sh")
				.args([
					"-c",
					r#"exec gdbus "$@" --method org.freedesktop.portal.OpenURI.OpenFile "" "@h 3" "{'ask': <true>}" 3<"$0""#,
					the,
				])
				.args(PORTAL)
				.spawn();
		} else {
			let _ = Command::new("gdbus")
				.args(PORTAL)
				.args(["--method", "org.freedesktop.portal.OpenURI.OpenURI", "", the, "{'ask': <true>}"])
				.spawn();
		}
	}
}
