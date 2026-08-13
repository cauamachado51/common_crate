use std::{ffi::OsStr, io, os::windows::ffi::OsStrExt};
use windows::core::{PCWSTR, w};
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

	let result = unsafe {
		ShellExecuteW(
			None,
			w!("open"),
			PCWSTR(the.as_ptr()),
			PCWSTR::null(),
			PCWSTR::null(),
			SW_SHOW,
		)
	};

	let code = result.0 as isize;
	if code > 32 {
		Ok(())
	} else {
		Err(io::Error::from_raw_os_error(code as i32))
	}
}