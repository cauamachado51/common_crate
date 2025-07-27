use std::io::{self, Write};

pub static TEXT_PAUSE: std::sync::RwLock<&'static str> = std::sync::RwLock::new("Pressione enter para continuar...");

/// pausa (o console) e exibe "Pressione enter para continuar...". exemplo:
/// ```no_run
/// *common_crate::LANGUAGE.write().unwrap() = "Press enter to continue...";
/// pause()
/// ```
pub fn pause() {
    print!("{}", TEXT_PAUSE.read().unwrap());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
