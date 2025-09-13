/// separa string `"chave:valor"` em `Option<("chave", "valor")>`
pub fn key_value(text: &str) -> Option<(&str, &str)> {
    return text.split_once(':')
}