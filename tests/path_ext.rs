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
fn readable_my() { // cargo test --test path_ext readable_my --release -- --nocapture // ~300.6µs
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