// por ter interatividade compila eternamente em tests.
use common_crate::read_host;

fn main() {
    let stringprompt: String = String::from("Digite um usize (0-255): ");

    let usize: u8 = read_host(&stringprompt);
    let integer: i8 = read_host("Digite um integer (±127): ");
    let string: String = loop {
        let input: String = read_host("Digite uma string: ");
        if input.is_empty() { println!("empty input.") } else { break input }
    };
    let float: f64 = read_host("Digite um float (ex.: 5.5): ");
    let boolean: bool = read_host("Digite um booleano (true/false): ");
    let character: char = read_host("Digite um character: ");
    printar(&read_host::<String>("Digite uma string: "));

    println!("Usize: {}", usize);
    println!("Integer: {}", integer);
    println!("String: {}", string);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character); 
}
fn printar(param: &str) {
    println!("param: {}", param);
}