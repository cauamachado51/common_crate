use std::path::{Path, PathBuf};

/// Prefixo de caminho em windows.
/// 
/// vantagens:
/// - desabilta limite de 260 chars 
/// - interpretar o caminho de forma literal
/// 
/// desvantagens:
/// - programas antigos como cmd esperam receber caminho sem o prefixo.
pub const VERBATIM: &str = r"\\?\";

// Is not dyn-compatible due to having a method clean_verbatim that is not dispatchable due to the return type referencing Self.
// A compatibilidade com dyn só é importante se você planeja criar coleções mistas (ex.: Vec<Box<dyn PathExt>> contendo Path e PathBuf misturados)
// ou passar esses objetos para funções que aceitam "qualquer coisa que tenha esse trait".
/// Trait que extende tipos de caminho da std, adicionando métodos que faltam.
pub trait PathExt {
	/// remove o prefixo [`VERBATIM`] em windows, retorna si proprio se não tem ou não for windows.
	/// ### Exemplo
	/// ```
	/// use common_crate::fs::PathExt;
	/// let sujo = std::path::PathBuf::from(r"\\?\C:\Users\Rust");
	/// let limpo = sujo.clean_verbatim();
	/// let sujo2 = std::path::Path::new(r"\\?\C:\Users\Rust");
	/// let limpo2 = sujo2.clean_verbatim();
	/// assert_eq!(limpo, std::path::PathBuf::from(r"C:\Users\Rust"));
	/// assert_eq!(limpo2, std::path::Path::new(r"C:\Users\Rust"));
	/// ```
    fn clean_verbatim(self) -> Self;
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
}