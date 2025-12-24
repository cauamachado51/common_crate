use std::env;

/// Expande variáveis de ambiente do caminho. não testa se existe.
/// ### Exemplo
/// ```
/// use common_crate::fs::expand_path;
/// // Windows cmd
/// println!("%USERPROFILE%/Desktop/teste.ps1: {}", expand_path("%USERPROFILE%/Desktop/teste.ps1"));
/// // Windows PowerShell
/// println!("$env:USERPROFILE/Desktop/teste.ps1: {}", expand_path("$env:USERPROFILE/Desktop/teste.ps1"));
/// // Linux e Windows sugar syntaxe
/// println!("~/Desktop/teste.ps1: {}", expand_path("~/Desktop/teste.ps1"));
/// // Linux
/// println!("$HOME/Desktop/teste.ps1: {}", expand_path("$HOME/Desktop/teste.ps1"));
/// ```
/// para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
pub fn expand_path(path: &str) -> String {
    let mut result = path.to_string();

    #[cfg(windows)]
    {
        // Substitui variáveis de CMD no Windows (%USERPROFILE%)
        while let Some(start) = result.find('%') {
            if let Some(end) = result[start + 1..].find('%') {
                let var_name = &result[start + 1..start + 1 + end];
                if let Ok(value) = env::var(var_name) {
                    result.replace_range(start..start + end + 2, &value);
                } else {
                    break; // Evita loop infinito se variável não existir
                }
            } else {
                break;
            }
        }

        // Substitui variáveis de PowerShell no Windows ($env:USERPROFILE)
        while let Some(start) = result.find("$env:") {
            let var_start = start + 5; // pula "$env:"
            let var_end = result[var_start..]
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .map(|pos| var_start + pos)
                .unwrap_or(result.len());

            let var_name = &result[var_start..var_end];
            if let Ok(value) = env::var(var_name) {
                result.replace_range(start..var_end, &value);
            } else {
                break; // Evita loop infinito se variável não existir
            }
        }
    }

    // Substitui variáveis do Linux ($VAR ou ${VAR})
    #[cfg(unix)]
    {
        let mut pos = 0;
        while let Some(start) = result[pos..].find('$') {
            let start = pos + start;
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
                let end = rest
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(rest.len());
                if end == 0 {
                    pos = start + 1;
                    continue;
                }
                (&rest[..end], end + 1)
            };

            if let Ok(value) = env::var(var_name) {
                result.replace_range(start..start + end_offset, &value);
                pos = start + value.len(); // continua após o valor inserido
            } else {
                pos = start + 1; // pula este $ e continua procurando
            }
        }
    }

    // Substitui ~ no início (funciona para Linux e Windows)
    if result.starts_with("~/") || result == "~" {
        if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            result = result.replacen("~", &home, 1);
        }
    }

    // Normaliza as barras para o Windows
    #[cfg(windows)]
    {
        result = result.replace('/', "\\");
    }

    result
}