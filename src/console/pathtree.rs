use std::{fmt, fs, io, path::Path};
use crate::ds::wildmatch;

#[derive(Debug)]
pub struct PathTree<'a> {
	pub root: &'a Path,
	pub include: &'a [&'a str],
	pub exclude: &'a [&'a str],
	pub respect_gitignore: bool,
}

impl<'a> PathTree<'a> {
	pub fn new(root: &'a (impl AsRef<Path> + ?Sized)) -> PathTree<'a> {
		PathTree {
			root: root.as_ref(),
			include: &[],
			exclude: &[],
			respect_gitignore: true,
		}
	}
	/// itens a incluir, relativos ao root. aceita [wildcards](wildmatch).
	///
	/// Lazy: não entra em pastas que não são ancestrais ou parte da base dos patterns.
	pub fn include(mut self, paths: &'a [&'a str]) -> Self {
		self.include = paths;
		self
	}
	/// itens a excluir, relativos ao root. aceita [wildcards](wildmatch).
	///
	/// Lazy: se o item for uma pasta, nem entrará dentro dela.
	pub fn exclude(mut self, paths: &'a [&'a str]) -> Self {
		self.exclude = paths;
		self
	}
	/// adiciona a pasta .git e caminhos do arquivo [.gitignore](https://git-scm.com/docs/gitignore/pt_BR) no root a exclude
	pub fn respect_gitignore(mut self, value: bool) -> Self {
		self.respect_gitignore = value;
		self
	}
	/// ```txt
	/// pasta/
	/// ├─ Nova pasta/
	/// │  ├─ Nova pasta again/
	/// │  ├─ teste again.py
	/// │  └─ teste again.ps1
	/// ├─ teste.py
	/// └─ teste.ps1
	/// ```
	/// ### Erros Comuns
	/// - não é uma pasta
	/// - permissão de leitura negada
	pub fn write(&self, w: &mut impl fmt::Write) -> io::Result<()> {
		if !self.root.is_dir() { return Err(io::Error::new(io::ErrorKind::Other, "root não é uma pasta")) }

		let mut buf;
		let mut exclude_vec;
		let exclude: &[&str] = if self.respect_gitignore {
			buf = String::new();
			exclude_vec = excludes_of_gitignore(self.root, &mut buf);
			exclude_vec.extend(self.exclude);
			&exclude_vec
		} else { self.exclude };

		// normaliza patterns (Windows usa `\`, gitignore usa `/`).
		let exclude: Vec<String> = exclude.iter().map(|p| p.trim_end_matches('/').trim_end_matches('\\').replace('\\', "/")).collect();
		let include: Vec<String> = self.include.iter().map(|p| p.trim_end_matches('/').trim_end_matches('\\').replace('\\', "/")).collect();

		// caso não de nome, é "."
		write!(w, "{}/", self.root.file_name().unwrap_or(self.root.as_os_str()).display()).map_err(|_| io::Error::new(io::ErrorKind::Other, "fmt error"))?;
		write_tree(w, self.root, self.root, "", &include, &exclude)
	}
	/// Recria a estrutura de arquivos/pastas a partir da saída do `PathTree`.
	///
	/// Suporta saída com `├─`, `└─`, `│` e variações de quantidade de `─`.
	///
	/// ### Exemplo
	/// ```no_run
	/// use common_crate::console::PathTree;
	/// let tree = r"pasta/
	/// ├─ Nova pasta/
	/// │  ├─ Nova pasta again/
	/// │  ├─ teste again.py
	/// │  └─ teste again.ps1
	/// ├─ teste.py
	/// └─ teste.ps1";
	/// PathTree::recreate(tree, ".").unwrap();
	/// ```
	pub fn recreate(tree: &str, target: &str) -> io::Result<()> {
		let lines: Vec<&str> = tree.lines().filter(|l| !l.trim().is_empty()).collect();
		if lines.is_empty() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input vazio.")) }

		let root_name = lines[0].trim();
		let root_path = Path::new(target).join(root_name);
		if !root_path.exists() { fs::create_dir_all(&root_path)?; }

		// mapeia profundidade -> caminho pai
		let mut depth_map = vec![root_path];

		for line in &lines[1..] {
			// Procura o conector (├─* ou └─*) e extrai prefixo + nome
			let Some(connector_start) = line.find('├').or_else(|| line.find('└')) else { continue };
			// conta profundidade pelo prefixo (cada nível são 3 chars: "│  " ou "   ")
			let depth = (connector_start / 3) + 1;

			// pula o conector: '├' ou '└', depois '─'+ e espaço
			let after_connector = &line[connector_start + '├'.len_utf8()..];
			let raw_name = after_connector.trim_start_matches('─').trim_start();

			let parent = depth_map.get(depth - 1)
				.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("Profundidade inválida na linha: {line}")))?;

			let full_path = parent.join(raw_name);

			depth_map.truncate(depth); // limpa pastas irmãs e seus filhos

			if raw_name.ends_with('/') {
				if !full_path.exists() { fs::create_dir_all(&full_path)?; }
				depth_map.push(full_path);
			} else if !full_path.exists() {
				if let Some(parent_dir) = full_path.parent() {
					fs::create_dir_all(parent_dir)?;
				}
				fs::File::create(&full_path)?;
			}
		}

		Ok(())
	}
}

/// [`PathTree::write`]
impl fmt::Display for PathTree<'_> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.write(f).map_err(|_| fmt::Error)
	}
}

fn write_tree(w: &mut impl fmt::Write, root: &Path, current_path: &Path, prefix: &str, include: &[String], exclude: &[String]) -> io::Result<()> {
	// itens em ordem: pastas primeiro, depois arquivos, ordenados alfabeticamente via
	// case-insensitive (por causa que Z vem antes de a, mas exploradores ordenam case-insensitive).
	let items = {
		let entries: Vec<fs::DirEntry> = fs::read_dir(current_path)?
			.collect::<Result<Vec<fs::DirEntry>, _>>()?;

		let mut keyed: Vec<(bool, String, fs::DirEntry)> = entries.into_iter().map(|e| {
			let is_file = e.file_type().map(|t| t.is_file())?;
			let name = e.file_name().to_string_lossy().to_lowercase();
			Ok((is_file, name, e))
		}).collect::<Result<Vec<(bool, String, fs::DirEntry)>, io::Error>>()?;

		keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

		keyed.into_iter().map(|(_, _, e)| e).collect::<Vec<fs::DirEntry>>()
	};

	let root_len = root.as_os_str().len() + 1; // +1 para o separador

	let items: Vec<fs::DirEntry> = items.into_iter().filter(|e| {
		let full = e.path();
		let relative = &full.to_string_lossy()[root_len..].replace('\\', "/");

		// Filtra excludes
		if exclude.iter().any(|pat| wildmatch(relative, pat, false)) {
			return false;
		}
		// Filtra includes
		if include.is_empty() { return true; }
		include.iter().any(|pat| {
			if full.is_dir() {
				// pasta é ancestral do padrão (ex: "tests" para "tests/pasta/*.ps1")
				if wildmatch(pat, &format!("{}/*", relative), false) {
					return true;
				}
				// pasta está dentro da base do padrão (ex: "tests/pasta/Nova pasta" para "tests/pasta/*.ps1")
				let pattern_base = pat.split(&['*', '?'][..]).next().unwrap_or("");
				if wildmatch(relative, &format!("{}*", pattern_base), false) {
					return true;
				}
			}
			wildmatch(relative, pat, false)
		})
	}).collect();

	let count = items.len();
	for (i, entry) in items.into_iter().enumerate() {
		let is_last = i == (count - 1);
		let path = entry.path();
		let name = entry.file_name();
		let name = name.to_string_lossy();
		let is_dir = path.is_dir();

		let connector = if is_last { "└─ " } else { "├─ " };
		let suffix = if is_dir { "/" } else { "" };
		write!(w, "\n{}{}{}{}", prefix, connector, name, suffix).map_err(|_| io::Error::new(io::ErrorKind::Other, "fmt error"))?;

		if is_dir {
			let new_prefix = if is_last {
				format!("{prefix}   ")
			} else {
				format!("{prefix}│  ")
			};
			write_tree(w, root, &path, &new_prefix, include, exclude)?;
		}
	}

	Ok(())
}

fn excludes_of_gitignore<'a>(root: &Path, buf: &'a mut String) -> Vec<&'a str> {
    let gitignore = root.join(".gitignore");
    if !gitignore.exists() { return vec![".git"] }

    *buf = fs::read_to_string(gitignore).unwrap_or_default();
    let mut vec: Vec<&str> = buf.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty() && !l.starts_with('#'))
		.map(|l| l.strip_prefix('/').unwrap_or(l))
		.collect();
	vec.push(".git");
	vec
}