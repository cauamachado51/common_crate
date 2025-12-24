fn main() {
    use common_crate::fs::expand_path;
    println!("%USERPROFILE%/Desktop/teste.ps1: {}", expand_path("%USERPROFILE%/Desktop/teste.ps1"));
    println!("$env:USERPROFILE/Desktop/teste.ps1: {}", expand_path("$env:USERPROFILE/Desktop/teste.ps1"));
    println!("~/Desktop/teste.ps1: {}", expand_path("~/Desktop/teste.ps1"));
    println!("$HOME/Desktop/teste.ps1: {}", expand_path("$HOME/Desktop/teste.ps1"));
}