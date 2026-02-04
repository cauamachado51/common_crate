use std::fs;
use std::io::Write;

// em desenvolvimento/testes, vou criar uma macro que calcula o tamanho de uma funcao via cargo bloat
macro_rules! func_size {
    ($body:block) => {
        let code_str = stringify!($body);
        let file_content = format!("fn main() {}", code_str);

        let mut file = fs::File::create("teste.rs").expect("Falha ao criar arquivo");
        file.write_all(file_content.as_bytes()).expect("Falha ao escrever");
		todo!();
    };
}

fn main() {
    func_size!({ minha_funcao(); });
}