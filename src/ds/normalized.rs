pub struct Normalized<'a>(pub &'a str);

impl Normalized<'_> {
	pub const A: [char; 10] = ['a', 'á', 'à', 'â', 'ã', 'A', 'Á', 'À', 'Â', 'Ã'];
	pub const B: [char; 2]  = ['b', 'B'];
	pub const C: [char; 4]  = ['c', 'ç', 'C', 'Ç'];
	pub const D: [char; 2]  = ['d', 'D'];
	pub const E: [char; 8]  = ['e', 'é', 'è', 'ê', 'E', 'É', 'È', 'Ê'];
	pub const F: [char; 2]  = ['f', 'F'];
	pub const G: [char; 2]  = ['g', 'G'];
	pub const H: [char; 2]  = ['h', 'H'];
	pub const I: [char; 6]  = ['i', 'í', 'ì', 'I', 'Í', 'Ì'];
	pub const J: [char; 2]  = ['j', 'J'];
	pub const K: [char; 2]  = ['k', 'K'];
	pub const L: [char; 2]  = ['l', 'L'];
	pub const M: [char; 2]  = ['m', 'M'];
	pub const N: [char; 2]  = ['n', 'N'];
	pub const O: [char; 10] = ['o', 'ó', 'ò', 'ô', 'õ', 'O', 'Ó', 'Ò', 'Ô', 'Õ'];
	pub const P: [char; 2]  = ['p', 'P'];
	pub const Q: [char; 2]  = ['q', 'Q'];
	pub const R: [char; 2]  = ['r', 'R'];
	pub const S: [char; 2]  = ['s', 'S'];
	pub const T: [char; 2]  = ['t', 'T'];
	pub const U: [char; 6]  = ['u', 'ú', 'ù', 'U', 'Ú', 'Ù'];
	pub const V: [char; 2]  = ['v', 'V'];
	pub const W: [char; 2]  = ['w', 'W'];
	pub const X: [char; 2]  = ['x', 'X'];
	pub const Y: [char; 2]  = ['y', 'Y'];
	pub const Z: [char; 2]  = ['z', 'Z'];
	
	pub fn equal(a: &char, b: &char) -> bool {
		if a == b { // números, pontuação e espaços
			return true;
		} else if Self::A.contains(a) && Self::A.contains(b) {
			return true;
		} else if Self::B.contains(a) && Self::B.contains(b) {
			return true;
		} else if Self::C.contains(a) && Self::C.contains(b) {
			return true;
		} else if Self::D.contains(a) && Self::D.contains(b) {
			return true;
		} else if Self::E.contains(a) && Self::E.contains(b) {
			return true;
		} else if Self::F.contains(a) && Self::F.contains(b) {
			return true;
		} else if Self::G.contains(a) && Self::G.contains(b) {
			return true;
		} else if Self::H.contains(a) && Self::H.contains(b) {
			return true;
		} else if Self::I.contains(a) && Self::I.contains(b) {
			return true;
		} else if Self::J.contains(a) && Self::J.contains(b) {
			return true;
		} else if Self::K.contains(a) && Self::K.contains(b) {
			return true;
		} else if Self::L.contains(a) && Self::L.contains(b) {
			return true;
		} else if Self::M.contains(a) && Self::M.contains(b) {
			return true;
		} else if Self::N.contains(a) && Self::N.contains(b) {
			return true;
		} else if Self::O.contains(a) && Self::O.contains(b) {
			return true;
		} else if Self::P.contains(a) && Self::P.contains(b) {
			return true;
		} else if Self::Q.contains(a) && Self::Q.contains(b) {
			return true;
		} else if Self::R.contains(a) && Self::R.contains(b) {
			return true;
		} else if Self::S.contains(a) && Self::S.contains(b) {
			return true;
		} else if Self::T.contains(a) && Self::T.contains(b) {
			return true;
		} else if Self::U.contains(a) && Self::U.contains(b) {
			return true;
		} else if Self::V.contains(a) && Self::V.contains(b) {
			return true;
		} else if Self::W.contains(a) && Self::W.contains(b) {
			return true;
		} else if Self::X.contains(a) && Self::X.contains(b) {
			return true;
		} else if Self::Y.contains(a) && Self::Y.contains(b) {
			return true;
		} else if Self::Z.contains(a) && Self::Z.contains(b) {
			return true;
		}

		false
	}
	/// filtra Combining Diacritical Marks (comum no MacOS) `'\u{0300}'..='\u{036F}'`, palavras que tem outros diacriticos darão false.
	/// ```
	/// use common_crate::ds::Normalized;
	/// let texto_alvo = "Um Coração partido";
	/// let entrada_do_usuario = "coracao";
	/// assert!(Normalized(texto_alvo).contains(entrada_do_usuario));
	/// ```
	/// trata mais chars como iguais que não estão nas constantes de Normalized e é mais rápido quando target é um livro (caso contrario, menos rápido):
	/// ```ignore
	/// use unicode_normalization::UnicodeNormalization;
	/// #[inline]
	/// fn normalized_contains(target: &str, contain: &str) -> bool {
	/// 	let target: String = target.nfd().filter(|&c| !matches!(c, '\u{0300}'..='\u{036f}')).flat_map(char::to_lowercase).collect();
	/// 	let contain: String = contain.nfd().filter(|&c| !matches!(c, '\u{0300}'..='\u{036f}')).flat_map(char::to_lowercase).collect();
	/// 	target.contains(&contain)
	/// }
	/// ```
	pub fn contains(&self, other: &str) -> bool {
		let contain: Vec<char> = other.chars().filter(|c| !matches!(c, '\u{0300}'..='\u{036F}')).collect();
		if contain.is_empty() { return true }
	
		let mut target = self.0.chars().filter(|c| !matches!(c, '\u{0300}'..='\u{036F}'));
	
		loop {
			let mut target_window = target.clone(); // O(1): copia só o estado do &str
			let mut matched = true;
		
			for contain_char in &contain {
				match target_window.next() {
					Some(target_char) if Self::equal(&target_char, contain_char) => {}
					Some(_) => { matched = false; break; }
					// target esgotou antes do contain: nenhuma posição futura pode dar match
					None => return false,
				}
			}
		
			if matched { return true }
		
			// avança a janela em um char
			if target.next().is_none() { return false }
		}
	}
}

impl PartialEq for Normalized<'_> {
	/// filtra Combining Diacritical Marks (comum no MacOS) `'\u{0300}'..='\u{036F}'`, palavras que tem outros diacriticos darão false.
	/// ```
	/// use common_crate::ds::Normalized;
	/// let texto_alvo = "Coração";
	/// let entrada_do_usuario = "coracao";
	/// assert!(Normalized(texto_alvo) == Normalized(entrada_do_usuario));
	/// ```
	/// trata mais chars como iguais que não estão nas constantes de Normalized:
	/// ```ignore
	/// use unicode_normalization::UnicodeNormalization;
	/// #[inline]
	/// fn normalized_equal(text1: &str, text2: &str) -> bool {
	/// 	let text1 = text1.nfd().filter(|&c| !matches!(c, '\u{0300}'..='\u{036f}')).flat_map(char::to_lowercase);
	/// 	let text2 = text2.nfd().filter(|&c| !matches!(c, '\u{0300}'..='\u{036f}')).flat_map(char::to_lowercase);
	/// 	text1.eq(text2)
	/// }
	/// ```
	fn eq(&self, other: &Self) -> bool {
		let mut self_chars = self.0.chars().filter(|c| !matches!(c, '\u{0300}'..='\u{036F}'));
		let mut other_chars = other.0.chars().filter(|c| !matches!(c, '\u{0300}'..='\u{036F}'));

		loop {
			match (self_chars.next(), other_chars.next()) {
				// Ambos os caracteres existem
				(Some(self_char), Some(other_char)) => {
					if !Normalized::equal(&self_char, &other_char) {
						return false;
					}
				}
				// Ambos os iteradores terminaram ao mesmo tempo
				(None, None) => return true,
				// Um iterador terminou antes do outro (tamanhos diferentes)
				_ => return false,
			}
		}
	}
}

impl<T: AsRef<str>> PartialEq<T> for Normalized<'_> {
	/// [`Normalized::eq`]
	fn eq(&self, other: &T) -> bool {
		*self == Normalized(other.as_ref())
	}
}