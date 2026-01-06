use std::{fs::read_dir, path::{Path, PathBuf}, time::Instant, io};
use walkdir::WalkDir;

#[test]
fn main() { // cargo test --test read_dirs -- --nocapture
	let target = "src";
	for _ in 0..10 { let _ = read_dir(target).unwrap(); }
    let time = Instant::now();
    let _ = read_dir(target).unwrap().count();
    println!("read_dir: {:?} µs", time.elapsed().as_micros()); // 33 µs
	for _ in 0..10 { let _ = read_dirs(target).unwrap(); }
    let time = Instant::now();
    let _ = read_dirs(target).unwrap().len();
    println!("read_dirs: {:?} µs", time.elapsed().as_micros()); // 153 µs
	for _ in 0..10 { let _ = read_dirs2(target).unwrap(); }
    let time = Instant::now();
    let _ = read_dirs2(target).unwrap().len();
    println!("read_dirs2: {:?} µs", time.elapsed().as_micros()); // 150 µs
	for _ in 0..10 { let _ = WalkDir::new(target); }
    let time = Instant::now();
    let _ = WalkDir::new(target).into_iter().count();
    println!("WalkDir: {:?} µs", time.elapsed().as_micros()); // 241 µs
	for _ in 0..10 { let _ = read_dirs3(target).unwrap(); }
	let time = Instant::now();
	let _ = read_dirs3(target).unwrap().len();
	println!("read_dirs3: {:?} µs", time.elapsed().as_micros()); // 158 µs
	for _ in 0..10 { let _ = read_dirs4(target).unwrap(); }
	let time = Instant::now();
	let _ = read_dirs4(target).unwrap().len();
	println!("read_dirs4: {:?} µs", time.elapsed().as_micros()); // 632 µs
	println!("{:-^50}", "read_dir");
	for path in read_dir(target).unwrap() { println!("{}", path.unwrap().path().display()); } 
	println!("{:-^50}", "read_dirs");
	for path in read_dirs(target).unwrap() { println!("{}", path.display()); } 
	println!("{:-^50}", "read_dirs2");
	for path in read_dirs2(target).unwrap() { println!("{}", path.display()); }
	println!("{:-^50}", "WalkDir");
	for entry in WalkDir::new(target) { println!("{}", entry.unwrap().path().display()); }
	println!("{:-^50}", "read_dirs3");
	for path in read_dirs3(target).unwrap() { println!("{}", path.display()); }
	println!("{:-^50}", "read_dirs4");
	for path in read_dirs4(target).unwrap() { println!("{}", path.display()); }
}

// aguenta no máximo 155 subpastas (call stack overflow), menos se tiver outras funções recursivas.
fn read_dirs(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            paths.extend(read_dirs(&path)?);
        }
        
        paths.push(path);
    }

    Ok(paths)
}

// aguenta no máximo 155 subpastas (call stack overflow), menos se tiver outras funções recursivas.
fn read_dirs2(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut buffer = Vec::new();
    read_dirs_internal(path, &mut buffer)?;
    Ok(buffer)
}

fn read_dirs_internal(dir: impl AsRef<Path>, buffer: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if entry.file_type()?.is_dir() {
            read_dirs_internal(&path, buffer)?;
        }
        
        buffer.push(path);
    }
    Ok(())
}

// aguenta o quanto a ram aguentar e corrige a pasta ir depois dos arquivos (embora não fiquem em sequencia).
fn read_dirs3(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut buffer = Vec::new();
    let mut dirs = vec![path.as_ref().to_path_buf()];

    while let Some(dir) = dirs.pop() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if entry.file_type()?.is_dir() {
                dirs.push(path.clone());
            }
            buffer.push(path);
        }
    }
    Ok(buffer)
}

// corrige ordenação em troca de muita performance, a jornada em busca de read_dirs acabou, use WalkDir.
fn read_dirs4(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut buffer = Vec::new();
    let mut stack = vec![path.as_ref().to_path_buf()];

    while let Some(current) = stack.pop() {
        if current != path.as_ref() {
            buffer.push(current.clone());
        }

        if current.is_dir() {
            let children: Vec<_> = read_dir(&current)?
                .map(|e| e.unwrap().path())
                .collect();

            for child in children.into_iter().rev() {
                stack.push(child);
            }
        }
    }
    Ok(buffer)
}