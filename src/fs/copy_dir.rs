use std::{fs, io, path::Path};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "rayon")]
use crate::THREAD_LOCK;
#[cfg(feature = "rayon")]
const LIMIT_PARALLEL_SIZE: u64 = 100 * 1024 * 1024; // 100MiB

/// Copia pasta recursivamente.
/// - `only_different`: caso arquivo de origem e destino tem tamanho e data de modificação iguais, não copia.
/// ### Testes
/// - 2x1gb: 15390ms
/// - 198x1mb: 127ms
/// - 198x1mb+2x1gb: 17966ms
/// ### Outros
/// multithread apenas se o tamanho do arquivo for menor que 100MiB, maiores espera.<br>
/// paralelismo de grandes arquivos é ruim por causa que o disco não aguenta, por isso usa o [`THREAD_LOCK`].
#[cfg(feature = "rayon")]
pub fn copy_dir<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()>
where
	P: AsRef<Path>,
	Q: AsRef<Path>,
{
	let dst = dst.as_ref();
	fs::create_dir_all(dst)?;

	fs::read_dir(src)?.par_bridge().try_for_each(|entry| -> io::Result<()> {
		let entry = entry?;
		let src_entry = entry.path();
		let dst_entry = dst.join(entry.file_name());

		if src_entry.is_dir() {
			return copy_dir(&src_entry, &dst_entry, only_different);
		}

		let src_meta = fs::metadata(&src_entry)?;
		let src_len = src_meta.len();
		if only_different && dst_entry.exists() {
			let dst_meta = fs::metadata(&dst_entry)?;
			if src_len == dst_meta.len() // tamanho
			&& src_meta.modified()? == dst_meta.modified()? // data de modificação
			{ return Ok(()) }
		}

		if src_len > LIMIT_PARALLEL_SIZE {
			let _guard = THREAD_LOCK.lock().unwrap(); // espera e bloqueia
			fs::copy(&src_entry, &dst_entry)?;
		} else {
			fs::copy(&src_entry, &dst_entry)?;
		}

		Ok(())
	})
}

/// copia pasta recursivamente.
/// - `only_different`: caso arquivo de origem e destino tem tamanho e data de modificação iguais, não copia.
/// ### Testes
/// - 2x1gb: 15683ms
/// - 198x1mb: 211ms
/// - 198x1mb+2x1gb: 18650ms
#[cfg(not(feature = "rayon"))]
pub fn copy_dir<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_entry = entry.path();
        let dst_entry = dst.join(entry.file_name());

        if src_entry.is_dir() {
            copy_dir(&src_entry, &dst_entry, only_different)?;
        } else if only_different && dst_entry.exists() {
            let src_meta = fs::metadata(&src_entry)?;
            let dst_meta = fs::metadata(&dst_entry)?;

            if src_meta.len() == dst_meta.len() // tamanho
            && src_meta.modified()? == dst_meta.modified()? // data de modificação
            { continue; } 

            fs::copy(&src_entry, &dst_entry)?;
        } else {
            fs::copy(&src_entry, &dst_entry)?;
        }
    }
    Ok(())
}