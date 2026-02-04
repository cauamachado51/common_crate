use std::time::Instant;
use common_crate::fs::windows::read_dir;

fn main() {
	let time = Instant::now();
	for entry in read_dir("C:/Windows/*.log").unwrap() { println!("{:?}", entry.unwrap().path()) }
	println!("Time: {:?}", time.elapsed());
	let time = Instant::now();
	for entry in std::fs::read_dir("C:/Windows").unwrap() { println!("{:?}", entry.unwrap().path()) }
	println!("Time: {:?}", time.elapsed()); // 10-14x mais lento
}
