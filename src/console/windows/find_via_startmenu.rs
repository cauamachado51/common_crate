use std::{env, fs, io, path::PathBuf};
use parselnk::Lnk;

pub enum Level {
	/// `%AppData%\Microsoft\Windows\Start Menu`
	User,
	/// `C:\ProgramData\Microsoft\Windows\Start Menu`
	Global,
	/// `%AppData%\Microsoft\Windows\Start Menu` + `C:\ProgramData\Microsoft\Windows\Start Menu`
	Both,
}

impl Level {
	pub fn iter(self) -> impl Iterator<Item = PathBuf> {
		let (p1, p2);
		match self {
			Level::User => {
				let mut user = env::var("AppData").unwrap();
				user.push_str(r"\Microsoft\Windows\Start Menu");
				p1 = Some(PathBuf::from(user));
				p2 = None;
			}
			Level::Global => {
				p1 = Some(PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu"));
				p2 = None
			}
			Level::Both => {
				let mut user = env::var("AppData").unwrap();
				user.push_str(r"\Microsoft\Windows\Start Menu");
				p1 = Some(PathBuf::from(user));
				p2 = Some(PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu"));
			}
		}
		p1.into_iter().chain(p2.into_iter())
	}
}

/// Encontra o caminho de arquivos através do menu iniciar.
/// ```
/// use common_crate::console::windows::{find_via_startmenu, Level};
/// let path = find_via_startmenu("steam.exe", Level::User).unwrap().0; // local_base_path
/// assert_eq!(path, std::path::PathBuf::from(r"C:\\Program Files\\Steam\\steam.exe"));
/// let path = find_via_startmenu("reGedit.exe", Level::Global).unwrap().0; // EnvironmentVariableDataBlock
/// assert_eq!(path, std::path::PathBuf::from("C:\\Windows\\regedit.exe"));
/// let path = find_via_startmenu("AMDInstallManager.exe", Level::Global).unwrap().0; // relative_path + fs::canonicalize
/// assert_eq!(path, std::path::PathBuf::from(r"\\?\C:\Program Files\AMD\AMDInstallManager\AMDInstallManager.exe"));
/// ```
pub fn find_via_startmenu(file_name: impl AsRef<str>, on: Level) -> Result<(PathBuf, Vec<FVSMError>), Vec<FVSMError>> {
	let file_name = file_name.as_ref();
	let mut errors = Vec::new();

	for folder in on.iter() {
		match find_recurse(folder, file_name, &mut errors) {
			Some(path) => return Ok((path, errors)),
			None => {},
		}
	}
	Err(errors)
}

fn find_recurse(folder: PathBuf, file_name: &str, errors: &mut Vec<FVSMError>) -> Option<PathBuf> {
	let entries = match fs::read_dir(&folder) {
		Ok(v) => v,
		Err(e) => { errors.push(FVSMError::IoReadDir(folder, e)); return None }
	};

	for entry in entries {
		let path = match entry {
			Ok(v) => v.path(),
			Err(e) => { errors.push(FVSMError::IoEntry(e)); continue }
		};

		if path.extension().and_then(|e| e.to_str()) == Some("lnk") {
			let lnk = match Lnk::try_from(path.as_path()) {
				Ok(v) => v,
				Err(e) => { errors.push(FVSMError::Lnk(e)); continue },
			};

			let target = if let Some(local) = lnk.link_info.local_base_path {
				// caso normal (criado pelo usuário): atalho com LinkInfo
				PathBuf::from(local)
			} else if let Some(rel) = lnk.string_data.relative_path {
				// caso FORCE_NO_LINK_INFO (apps nativos do Windows, que por acaso são hardlinks):
				// resolver o relative_path contra a pasta onde o .lnk está
				let base = match path.parent() {
					Some(p) => p,
					None => continue,
				};
				let to_canonicalize = base.join(&rel);
				match fs::canonicalize(&to_canonicalize) {
					Ok(canon) => canon,
					Err(e) => { errors.push(FVSMError::IoCanonicalize(to_canonicalize, e)); continue },
				}
			/*
			{ else if let Some(env_path) = env_target(&path) { // para crates.io que não tem o campo environment_props publico
				env_path // 3-4ms
			}
			*/
			} else if let Some(path_with_env) = lnk.extra_data.environment_props { // 2ms
				// EnvironmentVariableDataBlock (ex.: `%windir%\regedit.exe`)
				let raw = match path_with_env.target_unicode().ok().filter(|s| !s.is_empty())
					.or_else(|| path_with_env.target_ansi().ok().filter(|s| !s.is_empty()))
				{
					Some(v) => v,
					None => continue,
				};
				match expand_env(&raw) {
					Some(expanded) => PathBuf::from(expanded),
					None => continue,
				}
			} else {
				continue
			};

			let name = match target.file_name() {
				Some(v) => v,
				None => continue,
			};

			if name.eq_ignore_ascii_case(file_name) {
				return Some(target)
			}
		} else if path.is_dir() {
			if let Some(found) = find_recurse(path, file_name, errors) {
				return Some(found);
			}
		}
	}
	None
}

#[derive(Debug)]
pub enum FVSMError {
	/// 100% das vezes irá dar erro ao tentar ler a pasta `Programs` adjacente a `Start Menu` traduzida para o idioma do usuario.
	///
	/// apesar de 99.muitos9% das vezes só ter atalhos após `Programs`, o windows exibe no menu iniciar desde `Start Menu`.
	IoReadDir(PathBuf, io::Error),
	IoEntry(io::Error),
	Lnk(parselnk::error::Error),
	IoCanonicalize(PathBuf, io::Error),
}

/// Lê o EnvironmentVariableDataBlock direto do .lnk (a versão de parselnk do crates.io não expõe esse campo).
#[allow(dead_code)]
fn env_target(lnk_path: &std::path::Path) -> Option<PathBuf> {
	// BlockSize 0x0000_0314 + BlockSignature 0xA000_0001, ambos little-endian
	const MAGIC: [u8; 8] = [0x14, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0xA0];
	let data = fs::read(lnk_path).ok()?;
	let start = data.windows(MAGIC.len()).position(|w| w == MAGIC)?;
	let uni = start + MAGIC.len() + 260; // pula o TargetAnsi
	let bytes = data.get(uni..uni + 520)?;
	let wide: Vec<u16> = bytes
		.chunks_exact(2)
		.map(|c| u16::from_le_bytes([c[0], c[1]]))
		.take_while(|&u| u != 0)
		.collect();
	expand_env(&String::from_utf16(&wide).ok()?).map(PathBuf::from)
}

/// Expande `%VAR%`. Retorna None se a variável não existir ou houver `%` sem par.
fn expand_env(s: &str) -> Option<String> {
	let mut parts = s.split('%');
	let mut out = String::from(parts.next()?);
	while let Some(var) = parts.next() {
		let rest = parts.next()?;
		out.push_str(&env::var(var).ok()?);
		out.push_str(rest);
	}
	Some(out)
}