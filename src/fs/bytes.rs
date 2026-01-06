/// Tamanho máximo aceito pelo [`parse_bytes`]. u64::MAX = 18446744073709551615.
/// 
/// sem unidade (==b), b, kb, mb, gb, tb, pb, eb. maiúsculo/minúsculo não importa.<br>
/// u32 seria 4.2 GB, o que seria suficiente para qualquer arquivo basicamente, mas a std usa u64.
pub const PARSE_BYTES_MAX_SIZE: &str = "18.4 EB";
/// Tamanho máximo aceito pelo [`parse_bytes`]. u64::MAX = 18446744073709551615.
/// 
/// sem unidade (==b), b, kib, mib, gib, tib, pib, eib. maiúsculo/minúsculo não importa.
pub const PARSE_BYTES_MAX_SIZEI: &str = "15.9 EiB";

/// por tratar a fração como u64, unidades altas dão panic na fração.
pub const PARSE_BYTES_ERROR: &str = r#"
let baixo = parse_bytes("0.9999999999 gb").unwrap();
println!("0.9999999999 gb: {}b", baixo);
let certo = parse_bytes("0.9 eb").unwrap();
println!("0.9 eb: {}b", certo);
let erro = parse_bytes("0.90 eb");
match erro {
	Ok(v) => println!("0.90 eb Ok: {}b", v),
	Err(e) => println!("0.90 eb Err: {}", e) // Overflow na fração: 90
}
"#; 

pub const PARSE_BYTES_CORRECTION: &str = r#"
    if !frac_str.is_empty() {
		if frac_str.len() > 38 { return Err(String::from("fração u128: max 38 dígitos")); }
        let frac_num: u128 = frac_str.parse().map_err(|_| format!("Fração inválida: {}", frac_str))?;
        let divisor = 10u128.pow(frac_str.len() as u32);

        // Fórmula: (frac_num * multiplier) / 10^casas_decimais
        // Adiciona (divisor / 2) antes de dividir para arredondar o inteiro corretamente
        let numerator = frac_num.checked_mul(multiplier as u128).ok_or_else(||format!("Overflow na fração: {}", frac_num))?;
        let frac_bytes = (numerator + (divisor / 2)) / divisor;

        total_bytes = total_bytes.checked_add(frac_bytes as u64).ok_or("Overflow total de bytes")?;
    }
"#;

pub const PARSE_BYTES_CORRECTION_COST: &str = r#"
let time = Instant::now();
let a = parse_bytes_with_correction("0.9 eb").unwrap();
println!("{} em {}ns", a, time.elapsed().as_nanos()); // 53300ns
let time = Instant::now();
let b = parse_bytes("0.9 eb").unwrap();
println!("{} em {}ns", b, time.elapsed().as_nanos()); // 7800ns
"#;

/// Converte string de tamanho de arquivo para bytes.
/// ### Exemplo
/// ```
/// use common_crate::fs::bytes::{parse_bytes, PARSE_BYTES_MAX_SIZE};
/// let size: u64 = parse_bytes(PARSE_BYTES_MAX_SIZE).unwrap();
/// assert_eq!(size, 18400000000000000000);
/// let size: u64 = parse_bytes("1,5kb").unwrap();
/// assert_eq!(size, 1500);
/// let size: u64 = parse_bytes("1,5kib").unwrap();
/// assert_eq!(size, 1536);
/// ```
/// ### Outros
/// [`PARSE_BYTES_MAX_SIZE`] [`PARSE_BYTES_MAX_SIZEI`]<br>
/// o explorer do windows exibe numero kib, mas mostra a unidade erroneamente como kb.<br>
/// veja [`PARSE_BYTES_ERROR`], [`PARSE_BYTES_CORRECTION`] e [`PARSE_BYTES_CORRECTION_COST`]. por demorar 6.8x mais e eu não usar tanta fração, não vou corrigir. 
pub fn parse_bytes(size: &str) -> Result<u64, String> {
    let size_clean = size.trim().to_lowercase().replace(',', ".");

    // achar o indice de onde não é numérico e ponto, ou padrão ultimo indice "" (caso sem unit)
    let split_idx = size_clean.find(|c: char| !c.is_numeric() && c != '.').unwrap_or(size_clean.len());
    let (num_part, unit_part) = size_clean.split_at(split_idx);

    let multiplier: u64 = match unit_part.trim() {
        "b" | "" => 1,
        "kb" => 1_000,
        "mb" => 1_000_000,
        "gb" => 1_000_000_000,
        "tb" => 1_000_000_000_000,
		"pb" => 1_000_000_000_000_000,
		"eb" => 1_000_000_000_000_000_000,
        "kib" => 1_024,
        "mib" => 1_024 * 1_024,
        "gib" => 1_024 * 1_024 * 1_024,
        "tib" => 1_024 * 1_024 * 1_024 * 1_024,
		"pib" => 1_024 * 1_024 * 1_024 * 1_024 * 1_024,
		"eib" => 1_024 * 1_024 * 1_024 * 1_024 * 1_024 * 1_024,
        _ => return Err(format!("Unidade desconhecida: {}", unit_part)),
    };

    // Separa inteiro e fração (ex: "7.30" -> "7" e "30")
    let (int_str, frac_str) = match num_part.split_once('.') {
        Some((int_str, frac_str)) => (int_str, frac_str),
        None => (num_part, ""),
    };

    // 1. Calcula parte inteira
    let int_num: u64 = int_str.parse().map_err(|_| format!("Inteiro inválido: {}", int_str))?;
    let mut total_bytes = int_num.checked_mul(multiplier).ok_or_else(||format!("Overflow no inteiro: {}", int_num))?;

    // 2. Calcula parte fracionária (regra de três com arredondamento)
    if !frac_str.is_empty() {
        let frac_num: u64 = frac_str.parse().map_err(|_| format!("Fração inválida: {}", frac_str))?;
        let divisor = 10u64.pow(frac_str.len() as u32);

        // Fórmula: (frac_num * multiplier) / 10^casas_decimais
        // Adiciona (divisor / 2) antes de dividir para arredondar o inteiro corretamente
        let numerator = frac_num.checked_mul(multiplier).ok_or_else(||format!("Overflow na fração: {}", frac_num))?;
        let frac_bytes = (numerator + (divisor / 2)) / divisor;

        total_bytes = total_bytes.checked_add(frac_bytes).ok_or("Overflow total de bytes")?;
    }

    Ok(total_bytes)
}

/// Wrapper do [`parse_bytes`] que se ajusta ao sistema operacional.
/// - **Linux/Mac**: Comportamento padrão (1 KB = 1000 bytes, 1 KiB = 1024 bytes).
/// - **Windows**: Comportamento do Explorer (1 KB = 1024 bytes (1 KiB continua 1024 bytes)).
///
/// ### Exemplo
/// ```
/// use common_crate::fs::bytes::parse_bytes_sys;
/// #[cfg(windows)] // Windows
/// {
///     let size = parse_bytes_sys("1 KB").unwrap();
///     assert_eq!(size, 1024); // KB lido como KiB
/// }
///
/// #[cfg(not(windows))] // Linux/Mac
/// {
///     let size = parse_bytes_sys("1 KB").unwrap();
///     assert_eq!(size, 1000); // Padrão decimal
/// }
/// ```
pub fn parse_bytes_sys(size: &str) -> Result<u64, String> {
	#[cfg(windows)]
	{
    	let size_clean = size.trim().to_lowercase();
		
    	// Encontra onde começa a unidade (primeiro caractere alfabético ou vazio)
    	// Isso evita problemas com ',' ou '.' que fazem parte do número
    	let split_idx = size_clean.find(|c: char| c.is_alphabetic()).unwrap_or(size_clean.len());
    	let (num_part, unit_part) = size_clean.split_at(split_idx);

    	let new_unit_part = match unit_part {
    	    "kb" => "kib",
    	    "mb" => "mib",
    	    "gb" => "gib",
    	    "tb" => "tib",
    	    "pb" => "pib",
    	    "eb" => "eib",
    	    _ => unit_part, // Mantém b, kib ou vazio
    	};

    	// Reconstrói a string e delega a matemática para o parse_bytes oficial
    	parse_bytes(&format!("{}{}", num_part, new_unit_part))
	}
	#[cfg(not(windows))]
	{
		parse_bytes(size)
	}
}

/// Unidade de bytes para [`convert_bytes`].
/// - Binary: 1024 bytes = 1 KiB.
/// - Decimal: 1000 bytes = 1 KB.
/// - Sysd: Comportamento do [`parse_bytes_sys`]:
///   - Linux/Mac: 1000 bytes = 1 KB.
///   - Windows: 1024 bytes = 1 KB.
/// - Sysb: Comportamento do [`parse_bytes_sys`]:
///   - Linux/Mac: 1024 bytes = 1 KiB.
///   - Windows: 1024 bytes = 1 KB.
pub enum Unit { Binary, Decimal, Sysd, Sysb }

/// Converte bytes para string de tamanho de arquivo com unidade automatica.
/// ### Exemplo
/// ```
/// use common_crate::fs::bytes::{Unit, parse_bytes, convert_bytes};
///	let bytes = parse_bytes("1,668mb").unwrap();
///	let b1 = convert_bytes(bytes, Unit::Binary, 3);
///	assert_eq!(b1, "1.591 MiB");
///	let b2 = convert_bytes(bytes, Unit::Decimal, 0);
///	assert_eq!(b2, "2 MB"); // precision: 1 = 1.7 MB, 2 = 1.67 MB...
/// ```
pub fn convert_bytes(bytes: u64, unit: Unit, precision: usize) -> String {
    let (divisor, suffixes) = match unit {
        Unit::Binary => (1024.0, ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"]),
        Unit::Decimal => (1000.0, ["B", "KB", "MB", "GB", "TB", "PB", "EB"]),
        Unit::Sysd => {
            #[cfg(windows)]
            { (1024.0, ["B", "KB", "MB", "GB", "TB", "PB", "EB"]) }
            #[cfg(not(windows))]
            { (1000.0, ["B", "KB", "MB", "GB", "TB", "PB", "EB"]) }
        }
		Unit::Sysb => {
			#[cfg(windows)]
			{ (1024.0, ["B", "KB", "MB", "GB", "TB", "PB", "EB"]) }
			#[cfg(not(windows))]
			{ (1024.0, ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"]) }		
		}
    };

    let mut bytes = bytes as f64;
    let mut magnitude = 0;

    while bytes >= divisor && magnitude < suffixes.len() - 1 {
        bytes /= divisor;
        magnitude += 1;
    }

    format!("{:.2$} {}", bytes, suffixes[magnitude], precision)
}
