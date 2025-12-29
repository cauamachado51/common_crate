use std::time::Instant;
use common_crate::fs::find_in_path;
use which::which;

#[test]
fn main(){ // cargo test --test find_in_path -- --nocapture
	let time = Instant::now();
	let result = find_in_path("singleinstance.exe_source.7z").unwrap();
    println!("find_in_path {:?} em {}ms", result, time.elapsed().as_millis()); // 0ms
	let time = Instant::now();
	let result = which("singleinstance.exe_source.7z").unwrap();
    println!("which {:?} em {}ms", result, time.elapsed().as_millis()); // 7ms
	let time = Instant::now();
    let path = find_in_path("notepad").unwrap(); // 1ms
	println!("find_in_path {:?} em {}ms", path, time.elapsed().as_millis());
	let time = Instant::now();
    let path = which("notepad").unwrap(); // 3ms
	println!("which {:?} em {}ms", path, time.elapsed().as_millis());
}