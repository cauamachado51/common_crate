/// 2-3x mais rápido que WildMatch da crate wildmatch. suporte apenas a `*`.
/// ### Exemplo
/// ```
/// use common_crate::ds::wildmatch;
/// assert!(wildmatch("12oi45tchau78", "12*45*78", false));
/// assert!(wildmatch("mEudia", "me*ia", false)); // insensitive
/// assert!(!wildmatch("mEudia", "me*ia", true)); // sensitive
/// ```
pub fn wildmatch(input: &str, pattern: &str, sensitive: bool) -> bool {
	if !pattern.contains('*') { 
		return if sensitive { input == pattern } else { input.eq_ignore_ascii_case(pattern) } 
	}
	let mut pattern_parts = pattern.split('*');

	// Valida o começo obrigatório
	let first = pattern_parts.next().unwrap();
	if sensitive {
		if !input.starts_with(first) { return false }
	} else {
		if input.len() < first.len() // true: "inici", "inicio*" // não panicar: input[..first.len()]
		|| !input.is_char_boundary(first.len()) // true: "área", "a*" // não panicar ↑
		|| !input[..first.len()].eq_ignore_ascii_case(first) { return false }
	}
	
	// Remove o first da area de busca
	let mut search_area = &input[first.len()..];

	// Valida o final obrigatório
	let last = pattern_parts.next_back().unwrap();
	if sensitive {
		if !search_area.ends_with(last) { return false }
	} else {
		if search_area.len() < last.len() // true: "iniciofi", "inicio*fim" // não panicar: search_area[search_area.len() - last.len()..]
		|| !search_area.is_char_boundary(search_area.len() - last.len()) // true: "olá", "*a" // não panicar ↑
		|| !search_area[search_area.len() - last.len()..].eq_ignore_ascii_case(last) { return false }
	}

	// Remove o last da area de busca
	search_area = &search_area[..search_area.len() - last.len()];

	// Procura o miolo que sobrou
	if sensitive {
		for part in pattern_parts {
			match search_area.find(part) {
				Some(index) => search_area = &search_area[index + part.len()..],
				None => return false
			}
		}
	}
	else {
		for part in pattern_parts {
			let found = search_area.char_indices().find(|&(i, _)| {
				let end = i + part.len();
				search_area.len() >= end
				&& search_area.is_char_boundary(end) // false: "mês", "*s*"
				&& search_area[i..end].eq_ignore_ascii_case(part)
			});

			match found {
				Some((index, _)) => search_area = &search_area[index + part.len()..],
				None => return false
			}
		}
	}

	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn slice_panic_start() { assert!(!wildmatch("área", "a*", false)) }
	#[test]
	fn slice_panic_end() { assert!(!wildmatch("olá", "*a", false)) }
	#[test]
	fn slice_panic_middle() { assert!(wildmatch("mês", "*s*", false)) }
}