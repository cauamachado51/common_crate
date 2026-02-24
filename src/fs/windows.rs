use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{io, mem};
use windows::Win32::Foundation::{
	ERROR_FILE_NOT_FOUND, ERROR_NO_MORE_FILES, FILETIME, HANDLE, INVALID_HANDLE_VALUE,
};
use windows::Win32::Storage::FileSystem::{
	FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT, FIND_FIRST_EX_FLAGS, FindClose,
	FindExInfoBasic, FindExSearchNameMatch, FindFirstFileExW, FindNextFileW, WIN32_FIND_DATAW,
};

// ── DirEntry ──────────────────────────────────────────────────────────────────

pub struct DirEntry {
	root: Arc<PathBuf>,
	data: WIN32_FIND_DATAW,
}

impl DirEntry {
	pub fn path(&self) -> PathBuf {
		self.root.join(self.file_name())
	}

	pub fn file_name(&self) -> OsString {
		let len = self
			.data
			.cFileName
			.iter()
			.position(|&c| c == 0)
			.unwrap_or(260);
		OsString::from_wide(&self.data.cFileName[..len])
	}

	pub fn metadata(&self) -> io::Result<Metadata> {
		Ok(Metadata::from(&self.data))
	}

	pub fn file_type(&self) -> io::Result<FileType> {
		let attr = self.data.dwFileAttributes;
		Ok(FileType {
			is_dir: (attr & FILE_ATTRIBUTE_DIRECTORY.0) != 0,
			is_symlink: (attr & FILE_ATTRIBUTE_REPARSE_POINT.0) != 0,
		})
	}
}

// ── FileType ──────────────────────────────────────────────────────────────────

pub struct FileType {
	is_dir: bool,
	is_symlink: bool,
}

impl FileType {
	pub fn is_dir(&self) -> bool {
		self.is_dir && !self.is_symlink
	}
	pub fn is_file(&self) -> bool {
		!self.is_dir && !self.is_symlink
	}
	pub fn is_symlink(&self) -> bool {
		self.is_symlink
	}
}

// ── Permissions ───────────────────────────────────────────────────────────────

use windows::Win32::Storage::FileSystem::FILE_ATTRIBUTE_READONLY;

#[derive(Clone, Debug)]
pub struct Permissions {
	attrs: u32,
}

impl Permissions {
	pub fn readonly(&self) -> bool {
		(self.attrs & FILE_ATTRIBUTE_READONLY.0) != 0
	}

	pub fn set_readonly(&mut self, readonly: bool) {
		if readonly {
			self.attrs |= FILE_ATTRIBUTE_READONLY.0;
		} else {
			self.attrs &= !FILE_ATTRIBUTE_READONLY.0;
		}
	}
}

// ── Metadata ──────────────────────────────────────────────────────────────────

pub struct Metadata {
	attributes: u32,
	file_size: u64,
	created: FILETIME,
	accessed: FILETIME,
	modified: FILETIME,
}

impl Metadata {
	fn from(data: &WIN32_FIND_DATAW) -> Self {
		Self {
			attributes: data.dwFileAttributes,
			file_size: (data.nFileSizeHigh as u64) << 32 | data.nFileSizeLow as u64,
			created: data.ftCreationTime,
			accessed: data.ftLastAccessTime,
			modified: data.ftLastWriteTime,
		}
	}

	pub fn file_type(&self) -> FileType {
		FileType {
			is_dir: (self.attributes & FILE_ATTRIBUTE_DIRECTORY.0) != 0,
			is_symlink: (self.attributes & FILE_ATTRIBUTE_REPARSE_POINT.0) != 0,
		}
	}
	pub fn is_dir(&self) -> bool { (self.attributes & FILE_ATTRIBUTE_DIRECTORY.0) != 0 }
	pub fn is_file(&self) -> bool { !self.is_dir() && !self.is_symlink() }
	pub fn is_symlink(&self) -> bool { (self.attributes & FILE_ATTRIBUTE_REPARSE_POINT.0) != 0 }
	pub fn len(&self) -> u64 { self.file_size }
	pub fn permissions(&self) -> Permissions { Permissions { attrs: self.attributes } }
	pub fn modified(&self) -> io::Result<SystemTime> { systemtime_from(self.modified) }
	pub fn accessed(&self) -> io::Result<SystemTime> { systemtime_from(self.accessed) }
	pub fn created(&self) -> io::Result<SystemTime> { systemtime_from(self.created) }
}

fn systemtime_from(ft: FILETIME) -> io::Result<SystemTime> {
	let intervals = (ft.dwHighDateTime as u64) << 32 | ft.dwLowDateTime as u64;
	// FILETIME é em 100-nanosecond intervals desde 1601-01-01
	// UNIX_EPOCH é 1970-01-01, diferença = 11644473600 segundos
	const EPOCH_DIFF: u64 = 11_644_473_600;
	let secs = intervals / 10_000_000;
	let nanos = ((intervals % 10_000_000) * 100) as u32;
	if secs < EPOCH_DIFF {
		return Err(io::Error::new(io::ErrorKind::InvalidData, "time before unix epoch"));
	}
	Ok(UNIX_EPOCH + Duration::new(secs - EPOCH_DIFF, nanos))
}


// ── ReadDir ───────────────────────────────────────────────────────────────────

pub struct ReadDir {
	handle: HANDLE,
	root: Arc<PathBuf>,
	first: Option<WIN32_FIND_DATAW>,
	done: bool,
}

impl Drop for ReadDir {
	fn drop(&mut self) {
		if self.handle != INVALID_HANDLE_VALUE {
			unsafe {
				let _ = FindClose(self.handle);
			}
		}
	}
}

impl Iterator for ReadDir {
	type Item = io::Result<DirEntry>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		loop {
			let data = if let Some(first) = self.first.take() {
				first
			} else {
				let mut wfd: WIN32_FIND_DATAW = unsafe { mem::zeroed() };
				match unsafe { FindNextFileW(self.handle, &mut wfd) } {
					Ok(()) => wfd,
					Err(e) => {
						self.done = true;
						return if e.code() == ERROR_NO_MORE_FILES.to_hresult() {
							None
						} else {
							Some(Err(e.into()))
						};
					}
				}
			};

			// pula . e ..
			let name_len = data.cFileName.iter().position(|&c| c == 0).unwrap_or(260);
			let name = &data.cFileName[..name_len];
			if name == [b'.' as u16] || name == [b'.' as u16, b'.' as u16] {
				continue;
			}

			return Some(Ok(DirEntry {
				root: Arc::clone(&self.root),
				data,
			}));
		}
	}
}

// ── read_dir ──────────────────────────────────────────────────────────────────

/// ```
/// let time = Instant::now();
/// for entry in read_dir("C:/Windows/*.log").unwrap() { println!("{:?}", entry.unwrap().path()) }
/// println!("Time: {:?}", time.elapsed());
/// let time = Instant::now();
/// for entry in std::fs::read_dir("C:/Windows").unwrap() { println!("{:?}", entry.unwrap().path()) }
/// println!("Time: {:?}", time.elapsed()); // 10-14x mais lento
/// ```
pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
	let path = path.as_ref();

	// se não tem wildcard, comporta igual à stdlib: lista o diretório
	let search_path = if has_glob(path) {
		path.to_path_buf()
	} else {
		path.join("*")
	};

	// root é sempre o diretório pai para montar DirEntry::path() corretamente
	let root = search_path.parent().unwrap_or(Path::new(".")).to_path_buf();

	let wide: Vec<u16> = search_path
		.as_os_str()
		.encode_wide()
		.chain(Some(0))
		.collect();

	unsafe {
		let mut wfd: WIN32_FIND_DATAW = mem::zeroed();
		match FindFirstFileExW(
			windows::core::PCWSTR::from_raw(wide.as_ptr()),
			FindExInfoBasic,
			&mut wfd as *mut _ as _,
			FindExSearchNameMatch,
			None,
			FIND_FIRST_EX_FLAGS(0),
		) {
			Ok(handle) => Ok(ReadDir {
				handle,
				root: Arc::new(root),
				first: Some(wfd),
				done: false,
			}),
			Err(e) => {
				if e.code() == ERROR_FILE_NOT_FOUND.to_hresult() {
					// diretório existe mas vazio (ou wildcard sem match) — iterador vazio
					Ok(ReadDir {
						handle: INVALID_HANDLE_VALUE,
						root: Arc::new(root),
						first: None,
						done: true,
					})
				} else {
					Err(e.into())
				}
			}
		}
	}
}

fn has_glob(path: &Path) -> bool {
	path.as_os_str().to_string_lossy().contains(['*', '?'])
}
