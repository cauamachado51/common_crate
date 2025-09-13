use std::{array, str::FromStr};

/// separa string `"chave:valor"` em `Option<("chave", "valor")>`
pub fn key_value(text: &str) -> Option<(&str, &str)> {
    return text.split_once(':')
}

/// Separa e parseia string `"[1, 2, 3]"` ou `"1, 2, 3"` em array `[1, 2, 3]`. Retorna `Result` com `Ok([T; N])` em caso de sucesso ou `Err(String)` em caso de falha.
/// Exemplo:
/// ```no_run
/// let array1: Result<[u8; 3], String> = parse_array("[1, 2, 3]");
/// let array2: Result<[bool; 3], String> = parse_array("true, false, true");
/// ```
pub fn parse_array<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
    T: FromStr,
    T::Err: std::fmt::Debug,
{    
    let text = text.trim();
    // Remove colchetes se presentes, caso contrário usa o texto como está
    let cleaned = {
        if text.starts_with('[') && text.ends_with(']') {
            &text[1..text.len() - 1]
        } else {
            text
        }
    };

    // Divide por vírgula e coleta as partes
    let parts: Vec<&str> = cleaned.split(',').map(|s| s.trim()).collect();
    
    // Verifica se o número de elementos é correto
    if parts.len() != N {
        return Err(format!("Esperado {} elementos, encontrado {}", N, parts.len()));
    }

    // Cria array convertendo cada elemento para o tipo T
    let mut result = array::from_fn(|_| None);
    for (i, &part) in parts.iter().enumerate() {
        match part.parse::<T>() {
            Ok(value) => result[i] = Some(value),
            Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
        }
    }
    
    // Converte Option<T> em T, sabendo que todos são Some
    Ok(result.map(|opt| opt.unwrap()))
}

