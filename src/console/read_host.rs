use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;


/// Exibe um prompt, lê o que o usuário digitar no console e o retorna.
/// Requer anotação de tipo, caso falhe em parsear, exibe o erro e repete a requisição ao usuário.
/// ### Exemplo
/// ```no_run
/// use common_crate::console::read_host;
/// 
/// let string: String = loop {
///     let user_input: String = read_host("Digite seu nome: ");
///     if user_input.is_empty() { eprintln!("empty input.") } else { break user_input }
/// };
/// let usize: u8 = read_host("Digite sua idade: ");
/// let boolean: bool = read_host("Digite se você é empregado (true/false): ");
/// ```
pub fn read_host<T>(prompt: &str) -> T
where 
    T: FromStr,
    T::Err: Display
{
    loop {
        // exibir o prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // exibir imediatamente

        // Ler a entrada
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Tentar fazer o parse para o tipo desejado
        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(e) => println!("{e}"),
        }
    }
}
