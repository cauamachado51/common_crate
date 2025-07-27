## Sobre o que é a lib
comandos que desejo que Rust tenha, com nomenclatura de PowerShell se possível.
## Comandos suportados
### windows e Linux
- `read_host(prompt: &str)` Exibe um prompt, lê o que o usuário digitar no console e o retorna. Caso falhe em parsear, exibe o erro e repete a requisição ao usuário. exemplo:
  ```rust
  use common_crate::read_host;

  fn main() {
      let stringprompt: String = String::from("Digite sua idade: ");

      let usize: u8 = read_host(&stringprompt);
      let string: String = loop {
          let input: String = read_host("Digite seu nome: ");
          if input.is_empty() { println!("empty input.") } else { break input }
      };
      let boolean: bool = read_host("Digite se você é empregado (true/false): ");
      printar(&read_host::<String>("Digite qualquer coisa: "));

      println!("Usize: {}", usize);
      println!("String: {}", string);
      println!("Boolean: {}", boolean);
  }

  fn printar(param: &str) {
      println!("param: {}", param);
  }
  ```
- `pause()` pausa (o console) e exibe "Press enter to continue..."
- `test_path(path: &str)` Testa se o caminho existe
- `test_file(path: &str)` Testa se o caminho é um arquivo
- `test_dir(path: &str)` Testa se o caminho é um diretório
- `test_symlink(path: &str)` Testa se o caminho é um symlink
- `expand_path(path: &str)` Expande variáveis de ambiente do caminho. Exemplo:
  ```rust
  use common_crate::{expand_path, test_path};
  
  fn main() {
      println!("expand %USERPROFILE%/Desktop/teste.ps1: {}", expand_path("%USERPROFILE%/Desktop/teste.ps1")); // C:\Users\USER\Desktop\teste.ps1
      println!("expand&test $env:USERPROFILE/Desktop/teste.ps1: {}", test_path(&expand_path("$env:USERPROFILE/Desktop/teste.ps1"))); // true or false
      println!("expand ~/Desktop/teste.ps1: {}", expand_path("~/Desktop/teste.ps1")); // /home/USER/Desktop/teste.ps1
      println!("expand&test $HOME/Desktop/teste.ps1: {}", test_path(&expand_path("$HOME/Desktop/teste.ps1"))); // true or false
  }
  ```
  para listar variáveis de ambiente, use `std::env::vars().for_each(|(k, v)| println!("{}: {}", k, v));`
### windows
- `call_console()` Cria um console feio temporário que será liberado após a função acabar. Exemplo:
  ```rust
  #![windows_subsystem = "windows"] // iniciar o programa apenas GUI
  use common_crate::{call_console, read_host};
   
  fn main() {
      algo();
      println!("não é exibido");
  }
   
  fn algo() {
      let _variavel_que_sera_destruida = call_console();
      let nome: String = read_host("Digite seu nome: ");
      println!("Seu nome é: {}", nome);
      read_host::<String>("pressione enter para sair");
  }
  ```
- `get_all_languages()` Retorna todos os idiomas que o sistema pode suportar. Exemplo:
  ```rust
  let all: Vec<String> = get_all_languages();
  println!("Idiomas disponíveis: {:?}", all);
  ```
- `get_current_language()` Retorna o idioma atual do sistema. Exemplo:
  ```rust
  let language: String = get_current_language();
  println!("Idioma atual: {}", language);
  ```
- `get_installed_languages()` Retorna os idiomas instalados no sistema. Exemplo:
  ```rust
  let installed: Vec<String> = get_installed_languages();
  println!("Idiomas instalados: {:?}", installed);
  ```
- `verify_user_language(language: &[&str])` Verifica se algum dos idiomas do aplicativo foi encontrado nos idiomas instalados, retorna o mais próximo do atual. Retorna string vazia se não encontrar um idioma compatível. Exemplo:
  ```rust
  use common_crate::verify_user_language;

  const LINGUAGENS_DISPONIVEIS_NO_PROGRAMA: [&str; 4] = ["es-ES", "en-US", "pt-BR", "fr-FR"];
  let idioma_mais_proximo_do_atual: String = verify_user_language(&LINGUAGENS_DISPONIVEIS_NO_PROGRAMA);
  println!("Idioma mais proximo do atual: {}", idioma_mais_proximo_do_atual);
  ```