fn main() {
	let input = PARSE_BYTES_MAX_SIZE; 
    println!("Original: {}", input);

    let v_float = parse_bytes(input).unwrap(); // versão antiga
    println!("Float:    {}", v_float); // 18399999999999997952
    let v_int = parse_bytes2(input).unwrap();  // versão nova
    println!("Int:      {}", v_int);   // 18400000000000000000
}

/// Tamanho máximo aceitado pelo parse_bytes. u64::MAX = 18446744073709551615.
pub const PARSE_BYTES_MAX_SIZE: &str = "18.4 EB";
pub const PARSE_BYTES_MAX_SIZEI: &str = "15.99 EiB";

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

pub fn parse_bytes2(size: &str) -> Result<u64, String> {
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
    let int_num: u64 = int_str.parse().map_err(|_| format!("Número inválido: {}", int_str))?;
    let mut total_bytes = int_num.checked_mul(multiplier).ok_or("Overflow no cálculo")?;

    // 2. Calcula parte fracionária (regra de três com arredondamento)
    if !frac_str.is_empty() {
        let frac_num: u64 = frac_str.parse().map_err(|_| format!("Fração inválida: {}", frac_str))?;
        let divisor = 10u64.pow(frac_str.len() as u32);
        
        // Fórmula: (frac_num * multiplier) / 10^casas_decimais
        // Adiciona (divisor / 2) antes de dividir para arredondar o inteiro corretamente
        let numerator = frac_num.checked_mul(multiplier).ok_or_else(||format!("Overflow na fração: {}", frac_num))?;
        let frac_bytes = (numerator + (divisor / 2)) / divisor;
        
        total_bytes = total_bytes.checked_add(frac_bytes).ok_or("Overflow total")?;
    }

    Ok(total_bytes)
}
