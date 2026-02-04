fn main() {
	println!("cargo:rerun-if-changed=build.rs");
    let code = "fn vindo_do_build() { println!(\"Isso veio do build.rs\"); }";
    println!("cargo:rustc-env=INJECTED_CODE={}", code);
}
