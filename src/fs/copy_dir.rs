use std::{fs, io, path::Path};

// 2xgb: 23000-25000ms. 200x1mb: 357ms. 2x1gb+198x1mb: 26000-34000ms.
/// copia pasta recursivamente.
/// - `only_different`: caso arquivo de origem e destino tem tamanho e data de modificação iguais, não copia.
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

/*
use rayon::prelude::*;
use std::{fs, io, path::Path};

// 2x1gb: 22000-33000ms. 200x1mb: 105ms. 2x1gb+198x1mb: 33000ms
pub fn copy_dir<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()>
where
    P: AsRef<Path> + Sync, 
    Q: AsRef<Path> + Sync,
{
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;

    fs::read_dir(src)?.par_bridge().try_for_each(|entry| -> io::Result<()> {
            let entry = entry?;
            let src_entry = entry.path();
            let dst_entry = dst.join(entry.file_name());

            if src_entry.is_dir() {
                copy_dir(&src_entry, &dst_entry, only_different)?;
            } else {
                if only_different && dst_entry.exists() {
                    let src_meta = fs::metadata(&src_entry)?;
                    let dst_meta = fs::metadata(&dst_entry)?;

                    if src_meta.len() == dst_meta.len() // tamanho
                    && src_meta.modified()? == dst_meta.modified()? // data de modificação
                    { return Ok(()); }
                }
                fs::copy(&src_entry, &dst_entry)?;
            }
            Ok(())
        })
}
*/