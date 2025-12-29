/// Tamanho máximo aceito pelo [`parse_bytes`]. u64::MAX = 18446744073709551615.
/// 
/// sem unidade (==b), b, kb, mb, gb, tb, pb, eb. maiúsculo/minúsculo não importa.<br>
pub const PARSE_BYTES_MAX_SIZE: &str = "18.4 EB";
/// Tamanho máximo aceito pelo [`parse_bytes`]. u64::MAX = 18446744073709551615.
/// 
/// sem unidade (==b), b, kib, mib, gib, tib, pib, eib. maiúsculo/minúsculo não importa.<br>
pub const PARSE_BYTES_MAX_SIZEI: &str = "15.9 EiB";

/// Converte um tamanho de arquivo em string para bytes.
/// ### Exemplo
/// ```
/// use common_crate::fs::bytes::{parse_bytes, PARSE_BYTES_MAX_SIZE};
/// let size: u64 = parse_bytes(PARSE_BYTES_MAX_SIZE).unwrap();
/// assert_eq!(size, 18399999999999997952); // impreciso, deveria 18400000000000000000
/// let size: u64 = parse_bytes("1,5kb").unwrap();
/// assert_eq!(size, 1500);
/// let size: u64 = parse_bytes("1,5kib").unwrap();
/// assert_eq!(size, 1536);
/// ```
/// ### Outros
/// [`PARSE_BYTES_MAX_SIZE`] [`PARSE_BYTES_MAX_SIZEI`]<br>
/// o explorer do windows exibe numero kib, mas mostra a unidade erroneamente como kb.<br>
/// FIXME: concertar imprecisão/arredondamento
pub fn parse_bytes(size: &str) -> Result<u64, String> {
    let size_clean = size.trim().to_lowercase().replace(',', ".");

	// achar o indice de onde não é numérico e ponto, ou padrão ultimo indice "" (caso sem unit)
    let split_idx = size_clean.find(|c: char| !c.is_numeric() && c != '.').unwrap_or(size_clean.len());
    let (num_part, unit_part) = size_clean.split_at(split_idx);
    let value: f64 = num_part.parse().map_err(|_| format!("Número inválido: {}", num_part))?;
    
    let multiplier: f64 = match unit_part.trim() {
        "b" | "" => 1.0,
		"kb" => 1000.0,
        "mb" => 1000.0 * 1000.0,
        "gb" => 1000.0 * 1000.0 * 1000.0,
        "tb" => 1000.0 * 1000.0 * 1000.0 * 1000.0,
		"pb" => 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
		"eb" => 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
		"kib" => 1024.0,
        "mib" => 1024.0 * 1024.0,
        "gib" => 1024.0 * 1024.0 * 1024.0,
        "tib" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
		"pib" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
		"eib" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return Err(format!("Unidade desconhecida: {}", unit_part)),
    };

    Ok((value * multiplier).round() as u64)
}

/// Unidade de bytes para [`convert_bytes`].
pub enum Unit { Binary, Decimal }

/// Converte bytes para string de tamanho automatico com unidade.
/// ### Exemplo
/// ```
/// use common_crate::fs::bytes::{Unit, parse_bytes, convert_bytes};
///	let bytes = parse_bytes("1,668mb").unwrap(); // arredonda
///	let b1 = convert_bytes(bytes, Unit::Binary);
///	let b2 = convert_bytes(bytes, Unit::Decimal);
///	assert_eq!(b1, "1.591 MiB");
///	assert_eq!(b2, "1.668 MB");
/// ```
pub fn convert_bytes(bytes: u64, unit: Unit) -> String {
    let (divisor, suffixes) = match unit {
        Unit::Binary => (1024.0, ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"]),
        Unit::Decimal => (1000.0, ["B", "KB", "MB", "GB", "TB", "PB", "EB"]),
    };

    let bytes_f = bytes as f64;
    
    let magnitude = (bytes_f.log(divisor).floor() as usize).min(suffixes.len() - 1);
    let value = bytes_f / divisor.powi(magnitude as i32);

    format!("{:.3} {}", value, suffixes[magnitude])
}
