// para ver os resultados, copie para a main.rs
use std::{fs::remove_dir_all, path::Path, time::Instant};
use common_crate::fs::{copy_dir, metaprograming::new_fs_test_structure};

fn main() {
	let fstest_big = Path::new("tests_cache/Big");  // 2x1gb
	let fstest_tiny = Path::new("tests_cache/Tiny"); // 198x1mb
	let fstest_both = Path::new("tests_cache/Both"); // 198x1mb+2x1gb
	let fstest_big_dst = Path::new("tests_cache/BigDst");  // 2x1gb
	let fstest_tiny_dst = Path::new("tests_cache/TinyDst"); // 198x1mb
	let fstest_both_dst = Path::new("tests_cache/BothDst"); // 198x1mb+2x1gb
	
	if !fstest_big.exists() || !fstest_tiny.exists() || !fstest_both.exists() {
		new_fs_test_structure(fstest_big, 0, 2, "1gb").unwrap();
		new_fs_test_structure(fstest_tiny, 0, 198, "1mb").unwrap();
		new_fs_test_structure(fstest_both, 0, 200, "1mb").unwrap();
		new_fs_test_structure(fstest_both, 0, 2, "1gb").unwrap();
	}

	// ================================================================================

	let timeinstant = Instant::now();
    copy_dir(fstest_big, fstest_big_dst, true).unwrap(); 
    println!("2x1gb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_big_dst).unwrap();

    let timeinstant = Instant::now();
    copy_dir(fstest_tiny, fstest_tiny_dst, true).unwrap(); 
    println!("198x1mb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_tiny_dst).unwrap();
	
    let timeinstant = Instant::now();
    copy_dir(fstest_both, fstest_both_dst, true).unwrap(); 
    println!("198x1mb+2x1gb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_both_dst).unwrap();
	
	println!("==================================================================");

    let timeinstant = Instant::now();
    copy_dir2(fstest_big, fstest_big_dst, true).unwrap(); 
    println!("2x1gb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_big_dst).unwrap();

    let timeinstant = Instant::now();
    copy_dir2(fstest_tiny, fstest_tiny_dst, true).unwrap(); 
    println!("198x1mb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_tiny_dst).unwrap();
	
    let timeinstant = Instant::now();
    copy_dir2(fstest_both, fstest_both_dst, true).unwrap(); 
    println!("198x1mb+2x1gb: {}ms", timeinstant.elapsed().as_millis());
	remove_dir_all(fstest_both_dst).unwrap();
}

use std::{fs, io};

///  oficial: [common_crate::fs::copy_dir]

/// copia pasta recursivamente. singlethread. 100b a menos no binário que o oficial.
/// - `only_different`: caso arquivo de origem e destino tem tamanho e data de modificação iguais, não copia.
/// ### Testes
/// - 2x1gb: 15683ms
/// - 198x1mb: 211ms
/// - 198x1mb+2x1gb: 18650ms
fn copy_dir2<P, Q>(src: P, dst: Q, only_different: bool) -> io::Result<()>
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
            copy_dir2(&src_entry, &dst_entry, only_different)?;
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