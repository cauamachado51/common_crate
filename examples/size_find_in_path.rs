use common_crate::fs::find_in_path;

fn main(){ // cargo bloat --example size_find_in_path --release: 135.5KiB
	match find_in_path("singleinstance.exe_source.7z") { _ => () };
}