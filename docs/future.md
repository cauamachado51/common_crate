# erros
`let idade: u8 = read_host("Digite sua idade: ").parse().expect("erro ao parsear o retorno do read_host");` retorna erro se não digitar no console.

# melhorias
## 1
acho melhor na v2 renomear read_host2 para read_host e remover os outros ou read_host dar erro se não for digitado algo, dai o usuário precisa de:
```rust
let nome = read_host("Digite seu nome: ").unwrap_or_default(); // para retornar vazio, ou
let nome = read_host("Digite seu nome: ").unwrap_or("joão"); // para default_output
```
## 2
queria adicionar o método .if_empty():
```rust
let algo = funcao_qualquer().if_empty("entrada vazia") // default
let algo = funcao_qualquer().if_empty(panic!("não pode entrada vazia")) // encerrar o programa
let algo = funcao_qualquer().if_empty(return) // sair da função atual, retornando para main
```
testado e fica com uma mensagem chata sobre ser enrecheable quando coloco expressões divergentes.
existe uma crate com isso: [if_empty](https://crates.io/crates/if_empty)

em rust não existe Invoke-Expression para executar um código em variavel que o usuário digitou, mas funções podem dar o resultado desejado ou parar com expressões divergentes:
```rust
fn funcao(algo: &str) -> String {
    if algo.is_empty() {
        panic!("String vazia!"); // Diverge, equivalente a `!`
    }
    return algo.to_string()
}
```
meu plano é com regex e match/if verificar se em if_empty("valor") está escrito panic!("mensagem"), return, exit(code), caso não, retorne como string.
## 3
quero criar um método **opcional** para read_host: `.empty_loop()`. caso o usuário deixe vazio a entrada, exibe "empty input." e recomeça o loop. Sintaxe:
```rust
let nome: String = read_host("Digite seu nome: ").empty_loop();
let idade: u8 = read_host("Digite sua idade: ");
```
## 4
esse future.md está ficando grande, devo parar de adicionar coisas sem definir os antigos como inalcançável e exclui-los.