use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{io, mem};
use windows::Win32::Foundation::{
	ERROR_FILE_NOT_FOUND, ERROR_NO_MORE_FILES, HANDLE, INVALID_HANDLE_VALUE,
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
		// delega para std usando o path completo
		std::fs::metadata(self.path()).map(Metadata)
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

// ── Metadata (wrapper fino) ───────────────────────────────────────────────────

pub struct Metadata(std::fs::Metadata);

impl std::ops::Deref for Metadata {
	type Target = std::fs::Metadata;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
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