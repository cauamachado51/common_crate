use std::{io::{self, Write}, sync::RwLock};

pub static TEXT_PAUSE: RwLock<&'static str> = RwLock::new("Pressione enter para continuar...");

/// pausa (o console) e exibe "Pressione enter para continuar...". exemplo:
/// ```no_run
/// *common_crate::TEXT_PAUSE.write().unwrap() = "Press enter to continue...";
/// pause()
/// ```
pub fn pause() {
    print!("{}", TEXT_PAUSE.read().unwrap());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
