use which::which;

fn main(){ // cargo bloat --release --example size_which: 162.5KiB
	match which("singleinstance.exe_source.7z") { _ => () };
}