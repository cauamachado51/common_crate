use std::path::Path;
use parselnk::Lnk;

fn main() {
	let lnk = Lnk::try_from(Path::new(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Administrative Tools\Registry Editor.lnk")).unwrap();
	println!("{lnk:#?}");
	println!("{:#?}", lnk.relative_path());
	println!("{:#?}", lnk.arguments());
	println!("{:#?}", lnk.working_dir());
}