use common_crate::fs::find_in_path;
use which::which;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benchmark(c: &mut Criterion) {
	c.bench_function("find_in_path_single", |b| b.iter(|| black_box(find_in_path("singleinstance.exe_source.7z")))); // 531.74 µs
	c.bench_function("which_single", |b| b.iter(|| black_box(which("singleinstance.exe_source.7z")))); // 6.1007 ms
	c.bench_function("find_in_path_notepad", |b| b.iter(|| black_box(find_in_path("notepad")))); // 1.3107 ms
	c.bench_function("which_notepad", |b| b.iter(|| black_box(which("notepad")))); // 2.7505 ms
}

criterion_group!(benches, benchmark);
criterion_main!(benches);