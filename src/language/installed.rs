use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::sync::{Mutex, OnceLock};

// Declaração da função Win32
unsafe extern "system" {
    fn EnumUILanguagesW(
        lpUILanguageEnumProc: extern "system" fn(*const u16, i64) -> i32,
        dwFlags: u32,
        lParam: i64,
    ) -> i32;
}

const MUI_LANGUAGE_NAME: u32 = 0x8; // Formato BCP47
static LOCALES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_locales() -> &'static Mutex<Vec<String>> {
    LOCALES.get_or_init(|| Mutex::new(Vec::new()))
}

extern "system" fn ui_language_enum_proc(language: *const u16, _param: i64) -> i32 {
    unsafe {
        if language.is_null() {
            return 1;
        }
        let len = (0..).take_while(|&i| *language.offset(i) != 0).count();
        let lang = OsString::from_wide(std::slice::from_raw_parts(language, len));
        if let Ok(lang_str) = lang.into_string() {
            if !lang_str.is_empty() {
                get_locales().lock().unwrap().push(lang_str);
            }
        }
        1
    }
}

/// Retorna os idiomas instalados no sistema.
/// Exemplo:
/// ```no_run
/// let installed: Vec<String> = get_installed_languages();
/// println!("Idiomas instalados: {:?}", installed);
/// ```
pub fn get_installed_languages() -> Vec<String> {
    unsafe {
        let locales = get_locales();
        locales.lock().unwrap().clear();
        EnumUILanguagesW(ui_language_enum_proc, MUI_LANGUAGE_NAME, 0);
        return locales.lock().unwrap().clone()
    }
}