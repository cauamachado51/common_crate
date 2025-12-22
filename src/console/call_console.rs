#[link(name = "kernel32")]
unsafe extern "system" {
    fn AllocConsole() -> i32;
    fn FreeConsole() -> i32;
    fn GetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, lpMode: *mut u32) -> i32;
    fn SetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, dwMode: u32) -> i32;
    fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 4; // habilita ansi

pub struct Console;

impl Console {
    fn new() -> Self {
        unsafe {
            AllocConsole();
            Self::enable_ansi();
        }
        return Console
    }

    /// Ativa sequências de escape ANSI (cores, negrito, cursor) no conhost
    unsafe fn enable_ansi() {
        let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        let mut mode: u32 = 0;
        if unsafe { GetConsoleMode(handle, &mut mode) } != 0 {
            unsafe { SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) };
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { FreeConsole(); }
    }
}

/// Cria um console feio temporário que será liberado ao dropar a variavel. Exemplo:
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
///     let _variavel_que_sera_dropada = call_console();
///     let nome: String = read_host("Digite seu nome: ");
///     println!("Seu nome é: \x1b[31m{}\x1b[0m", nome);
///     pause();
/// }
/// ```
pub fn call_console() -> Console {
    Console::new()
}