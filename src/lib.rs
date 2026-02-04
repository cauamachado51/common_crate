//! ## Sobre o que é a lib
//! comandos que desejo que Rust tenha, com nomenclatura de PowerShell se possível.
use std::sync::Mutex;

pub mod console;
pub mod ds;
pub mod fs;
pub mod metaprograming;
// pub mod metaprograming;

/// Usado para bloquear a thread.
/// - [`fs::copy_dir`]: não travar o disco.
/// 
/// 64 bytes que não saem da RAM ao usar as funções que o usam.
pub static THREAD_LOCK: Mutex<()> = Mutex::new(());
