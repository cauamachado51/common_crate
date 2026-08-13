use std::ffi::OsStr;

/// Abre arquivo, pasta e url no app padrão.
/// 
/// suporta Windows, Linux e MacOS. não retorna erro de arquivo inexistente, bloqueado, etc.
/// ```no_run
/// use common_crate::fs;
/// fs::open(r"src");
/// fs::open(r"Cargo.toml");
/// fs::open("https:youtube.com");
/// ```
#[inline(always)]
pub fn open(the: &str) {
	use std::process::Command;
	#[cfg(target_os = "windows")]
	{
		let _ = Command::new("rundll32.exe").args(["url.dll,FileProtocolHandler", the]).spawn();
	}
	#[cfg(target_os = "linux")]
	{// ver depois: https://portland.freedesktop.org/doc/xdg-open.html
		let _ = Command::new("xdg-open").arg(the).spawn();
	}
	#[cfg(target_os = "macos")]
	{
		let _ = Command::new("open").arg(the).spawn();
	}
}

/// Abre arquivo, pasta e url no app padrão.
/// 
/// suporta Windows, Linux e MacOS. 39x mais lento que [`open`](self::open).
/// ```no_run
/// use common_crate::fs;
/// fs::open2(r"src").unwrap();
/// fs::open2(r"Cargo.toml").unwrap();
/// fs::open2("https:youtube.com").unwrap();
/// ```
#[inline(always)]
pub fn open2(the: impl AsRef<OsStr>) -> Result<(), opener::OpenError> {
	opener::open(the)
}
