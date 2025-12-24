use std::time::Instant;
use common_crate::fs::copy_dir;

fn main() {
    let timeinstant = Instant::now();
    copy_dir("src", "dst", true).unwrap(); // frio: 29ms. quente: 8ms, +iguais: 1-2ms
    println!("{} ms", timeinstant.elapsed().as_millis());
}
