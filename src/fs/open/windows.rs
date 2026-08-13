use std::{ffi::OsStr, io, os::windows::ffi::OsStrExt};
use windows::core::PCWSTR;
use windows::Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOW};

/// Abre arquivo, pasta e url no app padrão.
/// ```
/// use common_crate::fs;
/// fs::open(r"src");
/// fs::open(r"Cargo.toml");
/// fs::open("https:youtube.com");
/// ```
pub fn open(the: impl AsRef<OsStr>) -> io::Result<()> {
	let the: Vec<u16> = the.as_ref().encode_wide().chain(std::iter::once(0)).collect();
	const OPERATION: [u16; 5] = [
		b'o' as u16,
		b'p' as u16,
		b'e' as u16,
		b'n' as u16,
		0
	];

	let result = unsafe {
		ShellExecuteW(
			None,
			PCWSTR(OPERATION.as_ptr()),
			PCWSTR(the.as_ptr()),
			PCWSTR::null(),
			PCWSTR::null(),
			SW_SHOW,
		)
	};

	if (result.0 as isize) > 32 {
		Ok(())
	} else {
		Err(io::Error::last_os_error())
	}
}