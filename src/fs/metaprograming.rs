//! auxiliar de desenvolvimento
use std::{fs::{self, File}, io, path::{Path, PathBuf}};
use crate::fs::bytes::parse_bytes;

/// Cria uma estrutura de pastas e arquivos para testes de desempenho.
/// - `path`: caminho onde irão ser criadas as pastas e arquivos.
/// - `folder_recursion`: quantas subpastas irão ser criadas.
/// - `file_quantity`: quantos arquivos irão ser criados em cada pasta.
/// - `file_size`: tamanho dos arquivos. ex.: "1,5KB"
pub fn new_fs_test_structure<P>(path: P, folder_recursion: u16, file_quantity: u16, file_size: &str) -> io::Result<()>
where 
	P: AsRef<Path>
{
	let mut path = PathBuf::from(path.as_ref());
	let file_size = parse_bytes(file_size).unwrap();

    for folder_n in 0..=folder_recursion {
        fs::create_dir_all(&path)?;
        
        for file_n in 1..=file_quantity {
            let file_name = format!("file_{:04}.bin", file_n);
            let file_path = path.join(file_name);

            let file = File::create(&file_path)?;
            file.set_len(file_size)?; 
        }

        // Atualiza o Path para a próxima iteração, se ainda houver recursão
        if folder_n < folder_recursion {
            path.push(format!("SubFolder_{}", folder_n + 1));
        }
    }

    Ok(())
}