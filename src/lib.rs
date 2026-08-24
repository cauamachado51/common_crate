//! ## Sobre o que é a lib
//! comandos que desejo que Rust tenha, com nomenclatura de PowerShell se possível.
//! ## Features
//! - `rayon`: [`fs::copy_dir`] singlethread -> multithread.
use std::sync::Mutex;

pub mod console;
pub mod ds;
pub mod fs;
pub mod metaprogramming;
pub mod fmt;

/// Usado para bloquear a thread.
/// - [`fs::copy_dir`]: não travar o disco.
///
/// 64 bytes que não saem da RAM ao usar as funções que o usam.
pub static THREAD_LOCK: Mutex<()> = Mutex::new(());

/// ```
/// use common_crate::dbg;
/// let a = 2;
/// let b = dbg!(a * 2) + 1; // em perfil debug/dev: [src/main.rs:2:9] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
#[macro_export]
macro_rules! dbg {
	() => { // não é apenas $tokens:tt para conseguir retornar tupla
		if cfg!(debug_assertions) { std::dbg!() } else { () }
	};
	($val:expr $(,)?) => {
		if cfg!(debug_assertions) { std::dbg!($val) } else { $val }
	};
	($($val:expr),+ $(,)?) => {
		($($crate::dbg!($val)),+,)
	};
}