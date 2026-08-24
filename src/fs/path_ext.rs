#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;
use std::{ borrow::Cow, fs::OpenOptions, io::{self, ErrorKind}, path::{self, Path, PathBuf} };

/// Prefixo de caminho em windows.
///
/// vantagens:
/// - desabilta limite de 260 chars
///
/// desvantagens:
/// - programas antigos como cmd esperam receber caminho sem o prefixo.
/// - interpreta o caminho de forma literal, não funciona com caminhos relativos, apenas absolutos.
pub const VERBATIM: &str = r"\\?\";

/// Trait que extende tipos de caminho da std, adicionando métodos que faltam.
pub trait PathExt {
	/// Remove o prefixo [`VERBATIM`] em windows.
	///
	/// a impl de Pathbuf é boa somente se você necessitar que retorne Pathbuf (evita alocar outro Pathbuf se não ter o que remover),
	/// caso contrario, faça `(&pathbuf).clean_verbatim()`.
	///
	/// Util para usar depois de [`Path::canonicalize`]
	/// (caso o queira em vez de [`PathExt::absolute`] por seguir link simbolico e corrigir caixa alta/baixa no nome).
	/// ### Exemplo
	/// ```
	/// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
	/// assert_eq!(
	/// 	Path::new(r"\\?\C:\Users\Rust").clean_verbatim(),
	/// 	Path::new(r"C:\Users\Rust")
	/// );
	/// ```
	fn clean_verbatim(self) -> Self;

	/// Adiciona o prefixo [`VERBATIM`] em windows (torna [`path::absolute`] se não for).
	///
	/// ~5x mais rápido que canonicalize.
	/// ### Exemplo
	/// ```
	/// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
	/// assert_eq!(
	/// 	Path::new(r"C:\Users\Rust").add_verbatim().unwrap(),
	/// 	Path::new(r"\\?\C:\Users\Rust")
	/// );
	/// ```
	fn add_verbatim(self) -> io::Result<Self::Out>;
	type Out;

	/// Wrapper para [`path::absolute`]
	///
	/// TODO: remover quando o método oficial vier a estavel.
	fn absolute(self) -> io::Result<PathBuf>;

	/// Verifica se o usuario tem permissão de leitura.
	/// ```
	/// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
	/// assert!(!Path::new(r"C:\Program Files\WindowsApps").is_readable());
	/// assert!(!Path::new(r"inexistente.txt").is_readable());
	/// assert!(Path::new(r"cargo.toml").is_readable());
	/// assert!(Path::new(r"src").is_readable());
	/// ```
	/// Veja o benchmark vs [faccess](https://docs.rs/faccess/0.2.4/faccess/trait.PathExt.html) em `tests/path_ext.rs`.
	fn is_readable(&self) -> bool;

	/// Verifica se o usuário tem permissão de escrita.
	/// ```
	/// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
	/// assert!(!Path::new(r"C:\Program Files\WindowsApps").is_writable());
	/// assert!(!Path::new(r"inexistente.txt").is_writable());
	/// assert!(Path::new(r"cargo.toml").is_writable());
	/// assert!(Path::new(r"src").is_writable());
	/// ```
	/// Veja o benchmark vs [faccess](https://docs.rs/faccess/0.2.4/faccess/trait.PathExt.html) em `tests/path_ext.rs`.
	fn is_writable(&self) -> bool;

	/// Verifica se o usuário tem permissão de execução.
	/// ```
	/// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
	/// assert!(!Path::new(r"C:\Program Files\WindowsApps").is_executable());
	/// assert!(!Path::new(r"inexistente.txt").is_executable());
	/// assert!(Path::new(r"cargo.toml").is_executable());
	/// assert!(Path::new(r"src").is_executable());
	/// ```
	/// Veja o benchmark vs [faccess](https://docs.rs/faccess/0.2.4/faccess/trait.PathExt.html) em `tests/path_ext.rs`.
	fn is_executable(&self) -> bool;

	/// ```
	/// use common_crate::fs::{PathExt, Exist}; use std::path::Path;
	/// assert_eq!(Path::new(r"src/main.rs").exist_part(), Exist::All);
	/// assert_eq!(Path::new(r"src/inexistente/x.txt").exist_part(), Exist::Part(Path::new("src")));
	/// assert_eq!(Path::new(r"inexistente/x.txt").exist_part(), Exist::None);
	/// ```
	fn exist_part(&self) -> Exist<'_>;
}

impl<'a> PathExt for &'a Path {
	fn clean_verbatim(self) -> &'a Path {
		#[cfg(windows)]
		{
			if let Some(path_str) = self.to_str() {
				if let Some(path_str_stripped) = path_str.strip_prefix(VERBATIM) {
					return Path::new(path_str_stripped);
				}
			}
		}
		self
	}

	type Out = Cow<'a, Path>;
	fn add_verbatim(self) -> io::Result<Cow<'a, Path>> {
		#[cfg(windows)]
		{
			if self.is_absolute() {
				if let Some(path_str) = self.to_str() {
					if !path_str.starts_with(VERBATIM) {
						return Ok(Cow::Owned(PathBuf::from(format!("{}{}", VERBATIM, path_str))));
					}
				}
			}
			else {
				#[allow(unstable_name_collisions)]
				if let Some(path_str) = self.absolute()?.to_str() {
					return Ok(Cow::Owned(PathBuf::from(format!("{}{}", VERBATIM, path_str))));
				}
			}
		}
		Ok(Cow::Borrowed(self))
	}

	#[inline(always)]
	fn absolute(self) -> io::Result<PathBuf> {
		path::absolute(self)
	}

	fn is_readable(&self) -> bool {
		const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;
		let mut options = OpenOptions::new();
		options.read(true);

		#[cfg(windows)]
		options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);

		match options.open(self) {
			Ok(_) => true,
			Err(e) => match e.kind() {
				ErrorKind::PermissionDenied => false,
				_ => false
			},
		}
	}

	fn is_writable(&self) -> bool {
		const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;
		let mut options = OpenOptions::new();
		options.write(true);

		#[cfg(windows)]
		options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);

		match options.open(self) {
			Ok(_) => true,
			Err(e) => match e.kind() {
				ErrorKind::PermissionDenied => false,
				_ => false
			},
		}
	}

	fn is_executable(&self) -> bool {
		#[cfg(windows)]
		{
			const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;
			const GENERIC_EXECUTE: u32 = 0x20000000;

			let mut options = OpenOptions::new();
			options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);
			options.access_mode(GENERIC_EXECUTE);

			match options.open(self) {
				Ok(_) => true,
				Err(e) => match e.kind() {
					ErrorKind::PermissionDenied => false,
					_ => false
				},
			}
		}

		#[cfg(not(windows))]
		{
			use std::os::unix::fs::PermissionsExt;
			match std::fs::metadata(self) {
				Ok(meta) => meta.permissions().mode() & 0o111 != 0,
				Err(_) => false,
			}
		}
	}

	#[inline(always)]
	fn exist_part(&self) -> Exist<'_> {
		let mut paths = self.ancestors();
		if let Some(all) = paths.next() {
			if all.exists() {
				return Exist::All
			}
		}
		for ancestor in paths {
			if ancestor.exists() { return Exist::Part(ancestor) };
		}
		Exist::None
	}
}

impl PathExt for PathBuf {
	fn clean_verbatim(self) -> PathBuf {
		#[cfg(windows)]
		{
			if let Some(path_str) = self.to_str() {
				if let Some(path_str_stripped) = path_str.strip_prefix(VERBATIM) {
					return PathBuf::from(path_str_stripped);
				}
			}
		}
		self
	}

	type Out = PathBuf;
	fn add_verbatim(self) -> io::Result<PathBuf> {
		#[cfg(windows)]
		{
			if self.is_absolute() {
				if let Some(path_str) = self.to_str() {
					if !path_str.starts_with(VERBATIM) {
						return Ok(PathBuf::from(format!("{}{}", VERBATIM, path_str)));
					}
				}
			}
			else {
				#[allow(unstable_name_collisions)]
				if let Some(path_str) = self.as_path().absolute()?.to_str() {
					return Ok(PathBuf::from(format!("{}{}", VERBATIM, path_str)));
				}

			}
		}
		Ok(self)
	}

	#[inline(always)]
	fn absolute(self) -> io::Result<PathBuf> {
		path::absolute(self)
	}

	#[inline(always)]
	fn is_readable(&self) -> bool {
		self.as_path().is_readable()
	}

	#[inline(always)]
	fn is_writable(&self) -> bool {
		self.as_path().is_writable()
	}

	#[inline(always)]
	fn is_executable(&self) -> bool {
		self.as_path().is_executable()
	}
	#[inline(always)]
	fn exist_part(&self) -> Exist<'_> {
		exist_part(self)
	}
}

/// [`PathExt::exist_part`]
fn exist_part(path: &Path) -> Exist<'_> {
	let mut paths = path.ancestors();
	if let Some(all) = paths.next() {
		if all.exists() {
			return Exist::All
		}
	}
	for ancestor in paths {
		if ancestor.exists() { return Exist::Part(ancestor) };
	}
	Exist::None
}

/// [`PathExt::exist_part`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Exist<'a> {
	/// caminho completo
	All,
	/// parte do caminho
	Part(&'a Path),
	/// nenhuma parte do caminho
	None
}