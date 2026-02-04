use std::str::FromStr;
use std::fmt::Debug;

/// Separa e parseia string `"[1, 2, 3]"` ou `"1, 2, 3"` em Vec `vec![1, 2, 3]`. Retorna `Result` com `Ok(Vec<T>)` em caso de sucesso ou `Err(String)` em caso de falha.
/// Esta função aceita tamanho dinâmico, determinado em tempo de execução.
/// ### Exemplo
/// ```
/// use common_crate::ds::parse_vec;
/// let vec1: Vec<u8> = parse_vec("[1, 2, 3]").unwrap();
/// let vec2: Vec<bool> = parse_vec("true, false, true").unwrap();
/// ```
pub fn parse_vec<T>(text: &str) -> Result<Vec<T>, String>
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

	// Trabalha diretamente com o iterador
	let parts = cleaned.split(',').map(|s| s.trim());
	
	let mut result = Vec::new();
	
	for part in parts {
		match part.parse::<T>() {
			Ok(value) => result.push(value),
			Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
		}
	}
	
	Ok(result)
}