use std::time::Instant;

#[test]
fn a() { // cargo test --test open a --release -- --nocapture // 8.6319ms
	let time = Instant::now();
	let _ = common_crate::fs::open(r"C:\Users\cauam\AppData\Local\Microsoft\WindowsApps\notepad++.exe");
	println!("{:?}", time.elapsed())
}

#[test]
fn b() { // cargo test --test open b --release -- --nocapture // 337.5832ms
	let time = Instant::now();
	let _ = opener::open(r"C:\Users\cauam\AppData\Local\Microsoft\WindowsApps\notepad++.exe");
	println!("{:?}", time.elapsed())
}

#[test]
fn c() { // cargo test --test open c --release -- --nocapture // 378.6639ms
	let time = Instant::now();
	let _ = open::that(r"C:\Users\cauam\AppData\Local\Microsoft\WindowsApps\notepad++.exe");
	println!("{:?}", time.elapsed())
}