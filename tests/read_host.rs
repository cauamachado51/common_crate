// por ter interatividade compila eternamente, copie e cole em main.rs.
use common_crate::{read_host, read_host2};

fn main() {
    // macro
    let nome = read_host!("Digite seu nome: ");
    println!("Seu nome é: {}", nome);
    
    let idade: u8 = read_host!("Digite sua idade: ", "21").parse().expect("erro ao parsear o retorno do read_host");
    println!("Sua idade é: {}", idade);

    // função 1
    let nome = read_host("Digite seu nome: ");
    println!("Seu nome é: {}", nome);
    
    let idade: u8 = read_host("Digite sua idade: ").parse().expect("erro ao parsear o retorno do read_host");
    println!("Sua idade é: {}", idade);
    
    // função 2
    let nome = read_host2("Digite seu nome: ", "");
    println!("Seu nome é: {}", nome);
    
    let idade: u8 = read_host2("Digite sua idade: ", "21").parse().expect("erro ao parsear o retorno do read_host");
    println!("Sua idade é: {}", idade);
}
