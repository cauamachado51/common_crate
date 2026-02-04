use criterion::{criterion_group, criterion_main, Criterion};
use std::{array, hint::black_box, str::FromStr, mem::MaybeUninit};

pub fn parse_array<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	// Remove colchetes se presentes, caso contrário usa o texto como está
	let cleaned = {
		if text.starts_with('[') && text.ends_with(']') {
			&text[1..text.len() - 1]
		} else {
			text
		}
	};

	// Divide por vírgula e coleta as partes
	let parts: Vec<&str> = cleaned.split(',').map(|s| s.trim()).collect();
	
	// Verifica se o número de elementos é correto
	if parts.len() != N {
		return Err(format!("Esperado {} elementos, encontrado {}", N, parts.len()));
	}

	// Cria array convertendo cada elemento para o tipo T
	let mut result = array::from_fn(|_| None);
	for (i, &part) in parts.iter().enumerate() {
		match part.parse::<T>() {
			Ok(value) => result[i] = Some(value),
			Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
		}
	}
	
	// Converte Option<T> em T, sabendo que todos são Some
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array2<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	// Remove colchetes se presentes, caso contrário usa o texto como está
	let cleaned = {
		if text.starts_with('[') && text.ends_with(']') {
			&text[1..text.len() - 1]
		} else {
			text
		}
	};

	// Divide por vírgula e cria iterador
	let mut parts = cleaned.split(',').map(|s| s.trim());
	
	// Verifica número de elementos
	let count = parts.clone().count();
	if count != N {
		return Err(format!("Esperado {} elementos, encontrado {}", N, count));
	}

	// Cria array convertendo cada elemento para o tipo T
	let mut result = array::from_fn(|_| None);
	for i in 0..N {
		match parts.next() {
			Some(part) => match part.parse::<T>() {
				Ok(value) => result[i] = Some(value),
				Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
			},
			None => return Err("Iterador terminou inesperadamente".to_string()),
		}
	}
	
	// Converte Option<T> em T, sabendo que todos são Some
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array3<T, const N: usize>(text: &str) -> [T; N]
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	// Remove colchetes se presentes, caso contrário usa o texto como está
	let cleaned = {
		if text.starts_with('[') && text.ends_with(']') {
			&text[1..text.len() - 1]
		} else {
			text
		}
	};

	// Divide por vírgula e coleta as partes
	let mut parts = cleaned.split(',').map(|s| s.trim());
	
	// Cria array convertendo cada elemento para o tipo T
	return array::from_fn(|_| { parts.next().unwrap().parse::<T>().unwrap() });
}

pub fn parse_array4<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	// Remove colchetes se presentes, caso contrário usa o texto como está
	let cleaned = {
		if text.starts_with('[') && text.ends_with(']') {
			&text[1..text.len() - 1]
		} else {
			text
		}
	};

	// Divide por vírgula e cria iterador
	let mut parts = cleaned.split(',').map(|s| s.trim());
	
	// Cria array convertendo cada elemento para o tipo T
	let mut result = array::from_fn(|_| None);
	for i in 0..N {
		match parts.next() {
			Some(part) => match part.parse::<T>() {
				Ok(value) => result[i] = Some(value),
				Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
			},
			None => return Err(format!("Esperado {} elementos, encontrado menos", N)),
		}
	}

	// Verifica se há elementos extras
	if parts.next().is_some() {
		return Err(format!("Esperado {} elementos, encontrado mais", N));
	}
	
	// Converte Option<T> em T, sabendo que todos são Some
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array5<T, const N: usize>(text: &str) -> Result<[T; N], &'static str>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	let cleaned = if text.starts_with('[') && text.ends_with(']') {
		&text[1..text.len() - 1]
	} else {
		text
	};

	let mut parts = cleaned.split(',').map(|s| s.trim());
	let mut result = array::from_fn(|_| None);

	for i in 0..N {
		match parts.next() {
			Some(part) => match part.parse::<T>() {
				Ok(value) => result[i] = Some(value),
				Err(_) => return Err("Falha ao parsear elemento"),
			},
			None => return Err("Número insuficiente de elementos"),
		}
	}

	if parts.next().is_some() {
		return Err("Excesso de elementos");
	}
	
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array6<T, const N: usize>(text: &str) -> Result<[T; N], &'static str>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let cleaned = text.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(text);
	let mut parts = cleaned.split(',').map(|s| s.trim());
	let mut result = array::from_fn(|_| None);

	for i in 0..N {
		match parts.next() {
			Some(part) => result[i] = Some(part.parse::<T>().map_err(|_| "Falha ao parsear elemento")?),
			None => return Err("Número insuficiente de elementos"),
		}
	}

	if parts.next().is_some() {
		return Err("Excesso de elementos");
	}
	
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array7<T, const N: usize>(text: &str) -> Result<[T; N], &'static str>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let cleaned = text.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(text);
	let mut parts = cleaned.split(',').map(|s| s.trim());
	let result = array::from_fn(|_| {
		parts.next()?.parse::<T>().ok()
	});

	if result.iter().any(|opt| opt.is_none()) {
		return Err("Falha ao parsear elemento");
	}
	if parts.next().is_some() {
		return Err("Excesso de elementos");
	}
	
	Ok(result.map(|opt| opt.unwrap()))
}

pub fn parse_array8<T, const N: usize>(text: &str) -> Result<[T; N], String>
where 
	T: FromStr,
	T::Err: std::fmt::Debug,
{	
	let text = text.trim();
	// Remove colchetes se presentes, caso contrário usa o texto como está
	let cleaned = {
		if text.starts_with('[') && text.ends_with(']') {
			&text[1..text.len() - 1]
		} else {
			text
		}
	};

	// Trabalha diretamente com o iterador sem alocar Vec
	let mut parts = cleaned.split(',').map(|s| s.trim());
	
	// Cria array usando MaybeUninit para evitar alocações desnecessárias
	let mut result: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
	
	for i in 0..N {
		match parts.next() {
			Some(part) => {
				match part.parse::<T>() {
					Ok(value) => result[i] = MaybeUninit::new(value),
					Err(_) => return Err(format!("Falha ao parsear elemento '{}'", part)),
				}
			},
			None => return Err(format!("Esperado {} elementos, encontrado {}", N, i)),
		}
	}
	
	// Verifica se há elementos extras
	if parts.next().is_some() {
		// Conta quantos elementos restam para dar um erro preciso
		let remaining = 1 + parts.count();
		return Err(format!("Esperado {} elementos, encontrado {}", N, N + remaining));
	}
	
	// Converte MaybeUninit<T> para [T; N] com segurança
	Ok(result.map(|item| unsafe { item.assume_init() }))
}

fn benchmark(c: &mut Criterion) {
	c.bench_function("parse_array", |b| b.iter(|| black_box(parse_array::<i32, 3>("[1, 2, 3]")))); // 105.53 ns
	c.bench_function("parse_array2", |b| b.iter(|| black_box(parse_array2::<i32, 3>("[1, 2, 3]")))); // 58.126 ns
	c.bench_function("parse_array3", |b| b.iter(|| black_box(parse_array3::<i32, 3>("[1, 2, 3]")))); // 30.349 ns
	c.bench_function("parse_array4", |b| b.iter(|| black_box(parse_array4::<i32, 3>("[1, 2, 3]")))); // 35.433 ns
	c.bench_function("parse_array5", |b| b.iter(|| black_box(parse_array5::<i32, 3>("[1, 2, 3]")))); // 33.700 ns
	c.bench_function("parse_array6", |b| b.iter(|| black_box(parse_array6::<i32, 3>("[1, 2, 3]")))); // 33.548 ns
	c.bench_function("parse_array7", |b| b.iter(|| black_box(parse_array7::<i32, 3>("[1, 2, 3]")))); // 34.113 ns
	c.bench_function("parse_array8", |b| b.iter(|| black_box(parse_array8::<i32, 3>("[1, 2, 3]")))); // 34.796 ns
}

criterion_group!(benches, benchmark);
criterion_main!(benches);