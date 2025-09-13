#[link(name = "kernel32")]
unsafe extern "system" {
    fn AllocConsole() -> i32;
    fn FreeConsole() -> i32;
}

pub struct Console;

impl Console {
    fn new() -> Self {
        unsafe { AllocConsole(); }
        Console
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { FreeConsole(); }
    }
}

/// Cria um console feio temporário que será liberado após a função acabar. Exemplo:
/// ```
/// #![windows_subsystem = "windows"] // iniciar o programa apenas GUI
/// use common_crate::{call_console, read_host, pause};
/// 
/// fn main() {
///     algo();
///     println!("não é exibido");
/// }
/// 
/// fn algo() {
///     let _variavel_que_sera_destruida = call_console();
///     let nome: String = read_host("Digite seu nome: ");
///     println!("Seu nome é: {}", nome);
///     pause();
/// }
/// ```
pub fn call_console() -> Console {
    Console::new()
}