use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

pub fn key_value(text: &str) -> [&str; 2] { // 18.049 ns
	let mut parts = text.split(':');
	let chave = parts.next().unwrap();
	let valor = parts.next().unwrap();
	return [chave, valor];
}

pub fn key_value2(text: &str) -> [&str; 2] { // 101.75 ns
	let parts: Vec<&str> = text.split(":").collect();
	[parts[0], parts[1]]
}

pub fn key_value3(text: &str) -> (&str, &str) { // 8.9947 ns
	return text.split_once(':').unwrap();
}

fn benchmark(c: &mut Criterion) {
	c.bench_function("key_value", |b| b.iter(|| black_box(key_value("chave:valor"))));
	c.bench_function("key_value2", |b| b.iter(|| black_box(key_value2("chave:valor"))));
	c.bench_function("key_value3", |b| b.iter(|| black_box(key_value3("chave:valor"))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);