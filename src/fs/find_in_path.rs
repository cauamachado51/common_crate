use std::{env, path::{PathBuf, Path}};
use crate::fs::PathExt;
#[allow(unused_imports)]
use std::fs;

/// Busca o caminho de um arquivo no path do sistema.
/// ### Exemplo
/// ```
/// use common_crate::fs::find_in_path;
/// let notepad = find_in_path("notepad").unwrap();
/// assert_eq!(notepad, std::path::PathBuf::from("C:/Windows/system32/notepad.exe"));
/// ```
/// ### Testes
/// cargo bloat --release --example String: size_find_in_path 135.5KiB, size_which 162.5KiB<br>
/// cargo test --test find_in_path -- --nocapture: find_in_path 1156µs, which 2860µs
#[cfg(not(windows))]
pub fn find_in_path(file: &str) -> Option<PathBuf> {
    let paths_str = env::var_os("PATH")?;
	let paths_iter = env::split_paths(&paths_str);
	let mut file_paths = paths_iter.map(|dir|dir.join(&file));
	return file_paths.find(|file_path| file_path.is_file())
}

/// Busca o caminho de um arquivo no path do sistema.
/// ### Exemplo
/// ```
/// use common_crate::fs::find_in_path;
/// let notepad = find_in_path("notepad").unwrap();
/// assert_eq!(notepad, std::path::PathBuf::from(r"C:\Windows\System32\notepad.exe"));
/// ```
/// ### Testes
/// cargo bloat --release --example String: size_find_in_path 135.5KiB, size_which 162.5KiB<br>
/// cargo test --test find_in_path -- --nocapture: find_in_path 1156µs, which 2860µs
/// ### Outros
/// 1. caso passe arquivo sem extensão, tenta com as extensões do PATHEXT, mas caso passe nome do arquivo com .algo, não adiciona.
/// ex.: passar notepad++.exe não acha notepad++.exe.bat (igual which).
/// 2. usa [`fs::canonicalize`] (mais rápido que [`fs::read_dir`]) para evitar erros de maiúsculas/minúsculas em extensões.
#[cfg(windows)]
pub fn find_in_path(file: &str) -> Option<PathBuf> {
    let paths_str = env::var_os("PATH")?;
    let paths_iter = env::split_paths(&paths_str);

    let exts: Vec<String> =
	if Path::new(file).extension().is_none()
	{ env::var("PATHEXT").unwrap_or_default()
        .split(';').filter(|s| !s.is_empty())
        .map(|s| s.to_string()).collect()
    }
	else { vec![String::new()] };

    for dir in paths_iter {
        for ext in &exts {
            let mut file_path = dir.join(file);

            if !ext.is_empty() { file_path.as_mut_os_string().push(ext); }

			if file_path.exists() {
            	if let Ok(file_canon) = file_path.canonicalize() { // canon para não ser EXE/exe erroneamente
            	    return Some(file_canon.clean_verbatim());
            	}
			}
        }
    }
    None
}