use common_crate::console::windows::{Level, find_via_startmenu};

fn main() {
	println!("{:?}", find_via_startmenu("regedit.exe", Level::Global));
}