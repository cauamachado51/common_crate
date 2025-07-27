use std::{array, str::FromStr};

/// separa string `"chave:valor"` em `Option<("chave", "valor")>`
pub fn key_value(text: &str) -> Option<(&str, &str)> {
    return text.split_once(':')
}

/// Separa e parsea string `"[1, 2, 3]"` ou `"1, 2, 3"` em array `[1, 2, 3]`. exemplo:
/// ```no_run
/// let array1: [u8; 3] = parse_array("[1, 2, 3]");
/// let array2 = parse_array::<bool, 3>("true, false, true");
/// ```
pub fn parse_array<T, const N: usize>(text: &str) -> [T; N]
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
    let mut parts = cleaned.split(',').map(|s| s.trim());
    
    // Cria array convertendo cada elemento para o tipo T
    return array::from_fn(|_| { parts.next().unwrap().parse::<T>().unwrap() });
}

