use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::sync::{Mutex, OnceLock};

// Constantes da Win32 API
const LOCALE_ALL: u32 = 0x00000000; // Todas as localidades

// Declaração das funções Win32
unsafe extern "system" {
    fn EnumSystemLocalesEx(
        lpLocaleEnumProcEx: extern "system" fn(*const u16, u32, i64) -> i32,
        dwFlags: u32,
        lParam: i64,
        lpReserved: *mut std::ffi::c_void,
    ) -> i32;
}

// Vetor global para armazenar as localidades (usando Mutex para segurança)
static LOCALES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_locales() -> &'static Mutex<Vec<String>> {
    LOCALES.get_or_init(|| Mutex::new(Vec::new()))
}

// Função de callback para EnumSystemLocalesEx
extern "system" fn locale_enum_proc(locale: *const u16, _flags: u32, _param: i64) -> i32 {
    unsafe {
        if locale.is_null() {
            return 1;
        }

        let locale_name = OsString::from_wide({
            let len = (0..).take_while(|&i| *locale.offset(i) != 0).count();
            std::slice::from_raw_parts(locale, len)
        });

        if let Ok(locale_str) = locale_name.into_string() {
            if !locale_str.is_empty() {
                get_locales().lock().unwrap().push(locale_str);
            }
        }
    }
    1 // Continuar enumeração
}

/// Retorna todos os idiomas que o sistema pode suportar.
/// Exemplo:
/// ```no_run
/// let all: Vec<String> = get_all_languages();
/// println!("Idiomas disponíveis: {:?}", all);
/// ```
pub fn get_all_languages() -> Vec<String> {
    unsafe {
        // Limpa o vetor global
        get_locales().lock().unwrap().clear();

        // Enumera todas as localidades do sistema
        EnumSystemLocalesEx(locale_enum_proc, LOCALE_ALL, 0, ptr::null_mut());

        // Retorna uma cópia do vetor
        get_locales().lock().unwrap().clone()
    }
}