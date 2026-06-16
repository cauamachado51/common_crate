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
		#[allow(unused)] // rust-analyzer acha que não está usando p1
		let (mut p1, mut p2) = (None, None);
		match self {
			Level::User => {
				let mut user = env::var("AppData").unwrap();
				user.push_str(r"\Microsoft\Windows\Start Menu");
				p1 = Some(PathBuf::from(user));
			}
			Level::Global => {
				p1 = Some(PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu"));
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
/// let path = find_via_startmenu("steam.exe", Level::User).unwrap().0;
/// assert_eq!(path, std::path::PathBuf::from(r"C:\Program Files\Steam\steam.exe"));
/// let result = find_via_startmenu("regedit.exe", Level::Global);
/// assert!(result.is_err()); // não funciona em .lnk que é hardlink -- todos os apps que vem com o windows.
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
			let lnk_target = match Lnk::try_from(path.as_path()) {
				Ok(v) => v.link_info.local_base_path,
				Err(e) => { errors.push(FVSMError::Lnk(e)); continue },
			};
			match lnk_target {
				Some(v) => {
					let target = PathBuf::from(v);
					let name = target.file_name().unwrap();
					if name == file_name {
						return Some(target)
					}
				},
				None => continue
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
	IoReadDir(PathBuf, io::Error),
	IoEntry(io::Error),
	Lnk(parselnk::error::Error),
}