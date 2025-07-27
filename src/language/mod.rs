mod all;
pub use all::get_all_languages;
mod current;
pub use current::get_current_language;
mod installed;
pub use installed::get_installed_languages;

/// Verifica se algum dos idiomas do aplicativo foi encontrado nos idiomas instalados, retorna o mais próximo do atual.
/// Retorna string vazia se não encontrar um idioma compatível.
/// Exemplo:
/// ```
/// use common_crate::verify_user_language;
/// 
/// const LINGUAGENS_DISPONIVEIS_NO_PROGRAMA: [&str; 4] = ["es-ES", "en-US", "pt-BR", "fr-FR"];
/// let idioma_mais_proximo_do_atual: String = verify_user_language(&LINGUAGENS_DISPONIVEIS_NO_PROGRAMA);
/// println!("Idioma mais proximo do atual: {}", idioma_mais_proximo_do_atual);
/// ```
pub fn verify_user_language<const N: usize>(languages: &[&str; N]) -> String { // para ser dinamico, usar &[&str]
    let installed = get_installed_languages();
    
    for installed_lang in installed {
        if languages.contains(&installed_lang.as_str()) { return installed_lang }
    }
    
    String::new()
}
