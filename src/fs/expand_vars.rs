use std::env;

/// expande variáveis de ambiente cmd.
/// ```
/// use common_crate::fs::expand_cmd;
/// unsafe { std::env::set_var("windir", r"C:\Windows") };
/// unsafe { std::env::set_var("username", "james") };
/// assert_eq!(expand_cmd(r"%windir%\%inexistente%\%username%"), r"C:\Windows\%inexistente%\james");
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_cmd(path: impl Into<String>) -> String {
	let mut result = path.into();

	// substitui variáveis de Windows CMD (%USERPROFILE%)
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
	result
}

/// expande variáveis de ambiente powershell.
/// ```
/// use common_crate::fs::expand_ps;
/// unsafe { std::env::set_var("windir", r"C:\Windows") };
/// unsafe { std::env::set_var("username", "james") };
/// assert_eq!(expand_ps(r"$env:windir\$env:inexistente\$env:username"), r"C:\Windows\$env:inexistente\james");
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_ps(path: impl Into<String>) -> String {
	let mut result = path.into();

	// substitui variáveis de Windows PowerShell ($env:USERPROFILE)
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
	result
}

/// expande variáveis de ambiente bash.
/// ```
/// use common_crate::fs::expand_bash;
/// unsafe { std::env::set_var("windir", r"C:\Windows") };
/// unsafe { std::env::set_var("username", "james") };
/// assert_eq!(expand_bash(r"$windir\$inexistente\${username}"), r"C:\Windows\$inexistente\james");
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_bash(path: impl Into<String>) -> String {
	let mut result = path.into();

	let mut pos = 0;
	while let Some(rel) = result[pos..].find('$') {
		let start = pos + rel;
		let rest = &result[start + 1..];

		let (var_name, end_offset) = if rest.starts_with('{') {
			// formato ${VAR}
			if let Some(close) = rest.find('}') {
				(&rest[1..close], close + 2)
			} else {
				pos = start + 1;
				continue;
			}
		} else {
			// formato $VAR
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
	result
}

/// expande alguma coisa.
/// ```
/// use common_crate::fs::expand_sugar;
/// #[cfg(target_os = "linux")]
/// unsafe { std::env::set_var("home", "casa") };
/// #[cfg(target_os = "windows")]
/// unsafe { std::env::set_var("USERPROFILE", "casa") };
/// assert_eq!(expand_sugar(r"~\minha"), r"casa\minha"); // só se for o primeiro caractere
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_sugar(path: impl Into<String>) -> String {
	let mut result = path.into();

	// substitui ~ no início
	if result.starts_with('~') {
		#[cfg(target_os = "linux")]
		let home = env::var("HOME");
		#[cfg(target_os = "windows")]
		let home = env::var("USERPROFILE");

		if let Ok(home) = home {
			result = result.replacen('~', &home, 1);
		}
	}
	result
}