use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

// Declaração da função Win32
unsafe extern "system" {
    fn GetUserDefaultLocaleName(lpLocaleName: *mut u16, cchLocaleName: i32) -> i32;
}

/// Retorna o idioma atual do sistema.
/// Exemplo:
/// ```no_run
/// let current: String = get_current_language();
/// println!("Idioma atual: {}", current);
/// ```
pub fn get_current_language() -> String {
    unsafe {
        let mut buffer = [0u16; 85]; // LOCALE_NAME_MAX_LENGTH
        let result = GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32);

        if result > 0 {
            let default_locale = OsString::from_wide(&buffer[..result as usize - 1]);
            default_locale.into_string().unwrap_or_default()
        } else {
            String::new()
        }
    }
}