use std::{path::Path, time::Instant};
use common_crate::fs::PathExt;

#[test]
fn main() { // cargo test --test extends_path -- --nocapture
    let path = Path::new("Cargo.toml");

    let time = Instant::now();
    let work = path.add_verbatim().unwrap();
    println!("add_verbatim: {} em {}", work.display(), time.elapsed().as_micros()); // ~8, mas em main.rs ~20
    let time = Instant::now();
    let work = path.canonicalize().unwrap();
    println!("canonicalize: {} em {}", work.display(), time.elapsed().as_micros()); // ~145, mas em main.rs ~98

	for _ in 0..5 { let _ = path.canonicalize().unwrap(); let _ = path.add_verbatim().unwrap(); } // aquecer
    let time = Instant::now();
    let work = path.add_verbatim().unwrap();
    println!("add_verbatim: {} em {}", work.display(), time.elapsed().as_micros()); // ~1
    let time = Instant::now();
    let work = path.canonicalize().unwrap();
    println!("canonicalize: {} em {}", work.display(), time.elapsed().as_micros()); // ~124, mas em main.rs ~58
}
