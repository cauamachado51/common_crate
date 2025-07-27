use common_crate::{get_all_languages, get_current_language, get_installed_languages, verify_user_language};

#[test]
fn languages() {
    let all = get_all_languages();
    let current = get_current_language();
    let installed = get_installed_languages();
    println!("Idiomas disponíveis: {:?}", all);
    println!("Idioma atual: {}", current);
    println!("Idiomas instalados: {:?}", installed);
    
    let verify = ["es-ES", "en-US", "pt-BR", "fr-FR"];
    let verify_language = verify_user_language(&verify);
    if !verify_language.is_empty() {
        println!("Idioma mais próximo entre os instalados: {}", verify_language);
    } else {
        println!("Nenhum dos idiomas do aplicativo foi encontrado nos idiomas instalados");
    }
}
