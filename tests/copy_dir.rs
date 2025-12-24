use std::{fs, io, path::Path, time::Instant};
// copie para a main.rs

/// copia pasta recursivamente.
/// - `only_different`: caso arquivo de origem e destino tem tamanho e data de modificação iguais, não copia.
pub fn copy_dir<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()> // 7ms
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_file = entry.path();
        let src_file_name = entry.file_name();
        let dst_file = dst.join(&src_file_name);

        if src_file.is_dir() {
            copy_dir(&src_file, &dst_file, only_different)?;
        } else if only_different && dst_file.exists() {
            let src_meta = fs::metadata(&src_file)?;
            let dst_meta = fs::metadata(&dst_file)?;

            if src_meta.len() == dst_meta.len() // tamanho
            && src_meta.modified()? == dst_meta.modified()? // data de modificação
            { continue; } 

            fs::copy(&src_file, &dst_file)?;
        } else {
            fs::copy(&src_file, &dst_file)?;
        }
    }
    Ok(())
}


#[inline(always)]
pub fn copy_dir2<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()> // 1ms
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if only_different {
        copy_dir_different(src, dst)
    } else {
        copy_dir_ever(src, dst)
    }
}

fn copy_dir_different<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_file = entry.path();
        let src_file_name = entry.file_name();
        let dst_file = dst.join(&src_file_name);

        if src_file.is_dir() {
            copy_dir_different(&src_file, &dst_file)?;
            continue;
        } else if dst_file.exists() {
            let src_meta = fs::metadata(&src_file)?;
            let dst_meta = fs::metadata(&dst_file)?;

            if src_meta.len() == dst_meta.len() // tamanho
            && src_meta.modified()? == dst_meta.modified()? // data de modificação
            { continue; } 
        }
        fs::copy(&src_file, &dst_file)?;
    }
    Ok(())
}

fn copy_dir_ever<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_file = entry.path();
        let src_file_name = entry.file_name();
        let dst_file = dst.join(&src_file_name);

        if src_file.is_dir() {
            copy_dir_ever(&src_file, &dst_file)?;
        } else {
            fs::copy(&src_file, &dst_file)?;
        }
    }
    Ok(())
}

fn main() {
    fs::remove_dir_all("dst").unwrap();
    let timeinstant = Instant::now();
    copy_dir2("src", "dst", true).unwrap(); // 8ms
    println!("{} ms", timeinstant.elapsed().as_millis());
    fs::remove_dir_all("dst").unwrap();
    let timeinstant = Instant::now();
    copy_dir("src", "dst", true).unwrap(); // 7ms
    println!("{} ms", timeinstant.elapsed().as_millis());
}
