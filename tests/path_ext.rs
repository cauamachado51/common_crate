use std::{path::Path, time::Instant};
use common_crate::fs::PathExt;


#[test]
fn add_verbatim() { // cargo test --test path_ext add_verbatim --release -- --nocapture
	let path = Path::new("Cargo.toml");

	let time = Instant::now();
	let work = path.add_verbatim().unwrap();
	println!("add_verbatim: {} em {}", work.display(), time.elapsed().as_micros()); // ~8
	let time = Instant::now();
	let work = path.canonicalize().unwrap();
	println!("canonicalize: {} em {}", work.display(), time.elapsed().as_micros()); // ~101

	for _ in 0..5 { let _ = path.canonicalize().unwrap(); let _ = path.add_verbatim().unwrap(); } // aquecer
	let time = Instant::now();
	let work = path.add_verbatim().unwrap();
	println!("add_verbatim: {} em {}", work.display(), time.elapsed().as_micros()); // ~0
	let time = Instant::now();
	let work = path.canonicalize().unwrap();
	println!("canonicalize: {} em {}", work.display(), time.elapsed().as_micros()); // ~61
}

#[test]
fn readable_my() { // cargo test --test path_ext readable_my --release -- --nocapture // ~170.3µs
	let time: Instant = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").is_readable());
	assert!(!Path::new(r"inexistente.txt").is_readable());
	assert!(Path::new(r"cargo.toml").is_readable());
	assert!(Path::new(r"src").is_readable());
	println!("{:?}", time.elapsed())
}

#[test]
fn readable_faccess() { // cargo test --test path_ext readable_faccess --release -- --nocapture // ~700.7µs
	use faccess::PathExt;
	let time = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").readable());
	assert!(!Path::new(r"inexistente.txt").readable());
	assert!(Path::new(r"cargo.toml").readable());
	assert!(Path::new(r"src").readable());
	println!("{:?}", time.elapsed())
}

#[test]
fn writable_my() { // cargo test --test path_ext writable_my --release -- --nocapture // ~181.2µs
	let time: Instant = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").is_writable());
	assert!(!Path::new(r"inexistente.txt").is_writable());
	assert!(Path::new(r"cargo.toml").is_writable());
	assert!(Path::new(r"src").is_writable());
	println!("{:?}", time.elapsed())
}

#[test]
fn writable_faccess() { // cargo test --test path_ext writable_faccess --release -- --nocapture // ~681µs
	use faccess::PathExt;
	let time = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").writable());
	assert!(!Path::new(r"inexistente.txt").writable());
	assert!(Path::new(r"cargo.toml").writable());
	assert!(Path::new(r"src").writable());
	println!("{:?}", time.elapsed())
}

#[test]
fn executable_my() { // cargo test --test path_ext executable_my --release -- --nocapture // ~183µs
	let time: Instant = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").is_executable());
	assert!(!Path::new(r"inexistente.txt").is_executable());
	assert!(Path::new(r"cargo.toml").is_executable());
	assert!(Path::new(r"src").is_executable());
	println!("{:?}", time.elapsed())
}

#[test]
fn executable_faccess() { // cargo test --test path_ext executable_faccess --release -- --nocapture // ~651.5µs
	use faccess::PathExt;
	let time = Instant::now();
	assert!(!Path::new(r"C:\Program Files\WindowsApps").executable());
	assert!(!Path::new(r"inexistente.txt").executable());
	assert!(!Path::new(r"cargo.toml").executable()); // ele desqualifica os que não tem extensão: exe, com, bat ou cmd
	assert!(Path::new(r"src").executable());
	println!("{:?}", time.elapsed())
}