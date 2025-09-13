use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;


/// Exibe um prompt, lê o que o usuário digitar no console e o retorna.
/// Caso falhe em parsear, exibe o erro e repete a requisição ao usuário.
/// exemplo:
/// ```no_run
/// use common_crate::read_host;
/// 
/// fn main() {
///     let stringprompt: String = String::from("Digite sua idade: ");
/// 
///     let usize: u8 = read_host(&stringprompt);
///     let string: String = loop {
///         let input: String = read_host("Digite seu nome: ");
///         if input.is_empty() { println!("empty input.") } else { break input }
///     };
///     let boolean: bool = read_host("Digite se você é empregado (true/false): ");
///     printar(&read_host::<String>("Digite qualquer coisa: "));
/// 
///     println!("Usize: {}", usize);
///     println!("String: {}", string);
///     println!("Boolean: {}", boolean);
/// }
/// fn printar(param: &str) {
///     println!("param: {}", param);
/// }
/// ```
// TODO: implementar empty_loop
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
            Err(e) => println!("{e}."),
        }
    }
}
