#[allow(unused_imports)]
use std::str::FromStr;

/// implementa [`TryFrom`] via [`FromStr`].
/// ### Exemplo
/// ```
/// use common_crate::metaprogramming::impl_try_from_via_from_str;
/// struct Email(String);
/// impl std::str::FromStr for Email {
/// 	type Err = &'static str;
/// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
/// 		if s.contains('@') { Ok(Email(s.to_string())) }
/// 		else { Err("Falta @") }
/// 	}
/// }
/// impl_try_from_via_from_str!(Email);
/// println!("{}", Email::try_from("joao@teste.com").unwrap().0);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! impl_try_from_via_from_str {
	($tipo:ty) => {
		impl TryFrom<&str> for $tipo {
			type Error = <$tipo as std::str::FromStr>::Err;
			fn try_from(s: &str) -> Result<Self, Self::Error> {
				s.parse()
			}
		}
	};
}

/// implementa [`FromStr`] via [`TryFrom`].
/// ### Exemplo
/// ```
/// use common_crate::metaprogramming::impl_from_str_via_try_from;
/// struct Email(String);
/// impl TryFrom<&str> for Email {
/// 	type Error = &'static str;
/// 	fn try_from(s: &str) -> Result<Self, Self::Error> {
/// 		if s.contains('@') { Ok(Email(s.to_string())) }
/// 		else { Err("Falta @") }
/// 	}
/// }
/// impl_from_str_via_try_from!(Email, &'static str);
/// println!("{}", "joao@teste.com".parse::<Email>().unwrap().0);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! impl_from_str_via_try_from {
	($tipo:ty, $erro:ty) => {
		impl std::str::FromStr for $tipo {
			type Err = $erro;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Self::try_from(s)
			}
		}
	};
}
