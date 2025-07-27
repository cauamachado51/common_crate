use std::io::{self, Write};

/// Exibe um prompt, lê o que o usuário digitar no console e o retorna.
/// caso ele não digite (apenas enter) e tenha default_output, o retorna. exemplo:
/// ```
/// use powershell::read_host;
/// 
/// fn main() {
///     let nome = read_host!("Digite seu nome: ");
///     println!("Seu nome é: {}", nome);
///     
///     let idade: u8 = read_host!("Digite sua idade: ", "21").parse().expect("erro ao parsear o retorno do read_host");
///     println!("Sua idade é: {}", idade);
/// }
/// ```
/// não aparece a definição fantasma de `prompt` e `default_output`.
#[macro_export]
macro_rules! read_host {
    ($prompt:expr) => {
        $crate::read_host($prompt)
    };
    ($prompt:expr, $default_output:expr) => {
        $crate::read_host2($prompt, $default_output)
    };
}

// Run Doctest falha ao parsear o vazio.
/// Exibe um prompt, lê o que o usuário digitar no console e o retorna. exemplo:
/// ```
/// use powershell::read_host;
/// 
/// fn main() {
///     let nome = read_host("Digite seu nome: ");
///     println!("Seu nome é: {}", nome);
///     
///     let idade: u8 = read_host("Digite sua idade: ").parse().expect("erro ao parsear o retorno do read_host");
///     println!("Sua idade é: {}", idade);
/// }
/// ```
pub fn read_host(prompt: &str) -> String {
    // exibir o prompt
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // exibir imediatamente

    // Ler a entrada
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string() // trim para remover o \n do enter
}

/// Exibe um prompt, lê o que o usuário digitar no console e o retorna.
/// caso ele não digite (apenas enter) e tenha default_output, o retorna. exemplo:
/// ```
/// use powershell::read_host2;
/// 
/// fn main() {
///     let nome = read_host2("Digite seu nome: ", "");
///     println!("Seu nome é: {}", nome);
///     
///     let idade: u8 = read_host2("Digite sua idade: ", "21").parse().expect("erro ao parsear o retorno do read_host");
///     println!("Sua idade é: {}", idade);
/// }
/// ```
pub fn read_host2(prompt: &str, default_output: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Erro ao limpar stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");

    let input = input.trim();
    if input.is_empty() {
        return default_output.to_string()
    } else {
        return input.to_string()
    }
}