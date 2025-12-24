use std::mem::MaybeUninit;
use std::str::FromStr;
use std::fmt::Debug;

/// Separa e parseia string `"[1, 2, 3]"` ou `"1, 2, 3"` em array `[1, 2, 3]`. Retorna `Result` com `Ok([T; N])` em caso de sucesso ou `Err(String)` em caso de falha.
/// ### Exemplo
/// ```
/// use common_crate::ds::parse_array;
/// let array1: [u8; 3] = parse_array("[1, 2, 3]").unwrap();
/// let array2: [bool; 3] = parse_array("true, false, true").unwrap();
/// ```
pub fn parse_array<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
    T: FromStr,
    T::Err: Debug,
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

    // Trabalha diretamente com o iterador sem alocar Vec
    let mut parts = cleaned.split(',').map(|s| s.trim());
    
    // Cria array usando MaybeUninit para evitar alocações desnecessárias
    let mut result: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
    
    for i in 0..N {
        match parts.next() {
            Some(part) => {
                match part.parse::<T>() {
                    Ok(value) => result[i] = MaybeUninit::new(value),
                    Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
                }
            },
            None => return Err(format!("Esperado {} elementos, encontrado {}", N, i)),
        }
    }
    
    // Verifica se há elementos extras
    if parts.next().is_some() {
        // Conta quantos elementos restam para dar um erro preciso
        let remaining = 1 + parts.count();
        return Err(format!("Esperado {} elementos, encontrado {}", N, N + remaining));
    }
    
    // Converte MaybeUninit<T> para [T; N] com segurança
    Ok(result.map(|item| unsafe { item.assume_init() }))
}