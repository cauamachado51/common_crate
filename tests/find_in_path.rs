use std::time::Instant;
use common_crate::fs::find_in_path;
use which::which;

#[test]
fn main(){ // cargo test --test find_in_path -- --nocapture
	let time = Instant::now();
	let path = find_in_path("bootstat.dat").unwrap_or_default();
    println!("find_in_path {:?} em {}µs", path, time.elapsed().as_micros()); // 367µs
	let time = Instant::now();
	let path = which("bootstat.dat").unwrap_or_default();
    println!("which        {:?} em {}µs", path, time.elapsed().as_micros()); // 1801µs
	let time = Instant::now();
    let path = find_in_path("notepad").unwrap_or_default(); // 1156µs
	println!("find_in_path {:?} em {}µs", path, time.elapsed().as_micros());
	let time = Instant::now();
    let path = which("notepad").unwrap_or_default(); // 2860µs
	println!("which        {:?} em {}µs", path, time.elapsed().as_micros());
}