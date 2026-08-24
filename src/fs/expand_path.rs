use std::env;

/// Expande variáveis de ambiente do texto.
/// ### Exemplo
/// ```
/// use common_crate::fs::expand_vars;
/// unsafe { std::env::set_var("windir", r"C:\Windows") };
/// unsafe { std::env::set_var("username", "james") };
/// unsafe { std::env::set_var("home", "casa") };
/// assert_eq!(expand_vars(r"$env:windir\%username%"), r"C:\Windows\james"); // Windows Powershell e Cmd
/// assert_eq!(expand_vars(r"$windir\${username}"), r"C:\Windows\james"); // Linux Bash
/// assert_eq!(expand_vars(r"~\minha"), r"casa\minha"); // sugar syntax para home/userprofile (só se for o primeiro caractere)
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_vars(path: impl Into<String>) -> String {
	let mut result = path.into();

	// Substitui variáveis do Windows
	{
		// Substitui variáveis de Windows CMD (%USERPROFILE%)
		let mut pos = 0; // posição da visibilidade de texto
		while let Some(rel) = result[pos..].find('%') {
			let start = pos + rel; // posição do '%' de abertura
			let Some(len) = result[start + 1..].find('%') else {
				break; // sem '%' de fechamento: nada mais a expandir
			};
			let end = start + 1 + len; // posição do '%' de fechamento

			if let Ok(value) = env::var(&result[start + 1..end]) {
				result.replace_range(start..=end, &value);
				pos = start + value.len(); // não olha para onde já substituiu
			} else {
				pos = end; // mantém o '%' de fechamento e continua procurando
			}
		}

		// Substitui variáveis de Windows PowerShell ($env:USERPROFILE)
		let mut pos = 0;
		while let Some(rel) = result[pos..].find("$env:") {
			let start = pos + rel;
			let var_start = start + "$env:".len();
			let var_end = result[var_start..]
				.find(|c: char| !c.is_alphanumeric() && c != '_')
				.map(|p| var_start + p)
				.unwrap_or(result.len());

			if let Ok(value) = env::var(&result[var_start..var_end]) {
				result.replace_range(start..var_end, &value);
				pos = start + value.len();
			} else {
				pos = var_end; // o nome só tem [A-Za-z0-9_], logo não contém "$env:"
			}
		}
	}

	// Substitui variáveis do Linux
	{
		let mut pos = 0;
		while let Some(rel) = result[pos..].find('$') {
			let start = pos + rel;
			let rest = &result[start + 1..];

			let (var_name, end_offset) = if rest.starts_with('{') {
				// Formato ${VAR}
				if let Some(close) = rest.find('}') {
					(&rest[1..close], close + 2)
				} else {
					pos = start + 1;
					continue;
				}
			} else {
				// Formato $VAR
				let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(rest.len());
				if end == 0 {
					pos = start + 1;
					continue;
				}
				(&rest[..end], end + 1)
			};

			if let Ok(value) = env::var(var_name) {
				result.replace_range(start..start + end_offset, &value);
				pos = start + value.len();
			} else {
				pos = start + 1; // pula este $ e continua procurando
			}
		}
	}

	// Substitui ~ no início (Linux e ~Windows)
	if result.starts_with('~') {
		if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
			result = result.replacen('~', &home, 1);
		}
	}

	result
}