use std::{ io, path::{self, Path, PathBuf} };

/// Prefixo de caminho em windows.
///
/// vantagens:
/// - desabilta limite de 260 chars
///
/// desvantagens:
/// - programas antigos como cmd esperam receber caminho sem o prefixo.
/// - interpreta o caminho de forma literal, não funciona com caminhos relativos, apenas absolutos.
pub const VERBATIM: &str = r"\\?\";

// Is not dyn-compatible due to having a method clean_verbatim that is not dispatchable due to the return type referencing Self.
// A compatibilidade com dyn só é importante se você planeja criar coleções mistas (ex.: Vec<Box<dyn PathExt>> contendo Path e PathBuf misturados)
// ou passar esses objetos para funções que aceitam "qualquer coisa que tenha esse trait".
/// Trait que extende tipos de caminho da std, adicionando métodos que faltam.
pub trait PathExt {
    /// Remove o prefixo [`VERBATIM`] em windows, retorna si proprio se não tem ou não for windows.
	/// 
	/// Util para usar depois de [`Path::canonicalize`] (caso o queira em vez de [`PathExt::absolute`] por seguir link simbolico e corrigir nome).
    /// ### Exemplo
    /// ```
    /// use common_crate::fs::PathExt; use std::path::{Path, PathBuf};
    /// assert_eq!(
	/// 	PathBuf::from(r"\\?\C:\Users\Rust").clean_verbatim(),
	/// 	PathBuf::from(r"C:\Users\Rust")
	/// );
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
	/// 	PathBuf::from(r"C:\Users\Rust").add_verbatim().unwrap(),
	/// 	PathBuf::from(r"\\?\C:\Users\Rust"));
    /// assert_eq!(
	/// 	Path::new(r"C:\Users\Rust").add_verbatim().unwrap(),
	/// 	Path::new(r"\\?\C:\Users\Rust")
	/// );
    /// ```
    fn add_verbatim(self) -> io::Result<PathBuf>;

	/// Wrapper para [`path::absolute`]
    fn absolute(self) -> io::Result<PathBuf>;
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
				if let Some(path_str) = self.absolute()?.to_str() {
					return Ok(PathBuf::from(format!("{}{}", VERBATIM, path_str)));
				}
			}
        }
        Ok(PathBuf::from(self))
    }

    fn absolute(self) -> io::Result<PathBuf> {
        path::absolute(self)
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
				if let Some(path_str) = self.as_path().absolute()?.to_str() {
					return Ok(PathBuf::from(format!("{}{}", VERBATIM, path_str)));
				}

			}
        }
        Ok(self)
    }

    fn absolute(self) -> io::Result<PathBuf> {
        path::absolute(self)
    }
}
