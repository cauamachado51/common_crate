use std::{io::{self, Write}, sync::RwLock};

pub static PAUSE_TEXT: RwLock<&'static str> = RwLock::new("Pressione enter para continuar...");

/// Pausa e exibe "Pressione enter para continuar...".
/// ### exemplo
/// ```no_run
/// use common_crate::console::{pause, PAUSE_TEXT};
/// *PAUSE_TEXT.write().unwrap() = "Press enter to continue...";
/// pause()
/// ```
pub fn pause() {
    print!("{}", PAUSE_TEXT.read().unwrap());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
