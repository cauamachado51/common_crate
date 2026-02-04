use std::time::Instant;
use common_crate::ds::wildmatch;
use wildmatch::WildMatch;

#[test]
fn main() { // cargo test --test wildmatch -- --nocapture
	let input = "abCdEfgh";
	let pattern = "ab*de*gh";

	println!("{:=^20}", "sensitive");
	for _ in 0..9 { wildmatch(input, pattern, true); }
	let time = Instant::now();
	let _a = wildmatch(input, pattern, true);
	println!("wildmatch {}", time.elapsed().as_nanos()); // ~400

	for _ in 0..9 { WildMatch::new(pattern).matches(input); }
	let time = Instant::now();
	let _b = WildMatch::new(pattern).matches(input);
	println!("WildMatch	  {}", time.elapsed().as_nanos()); // ~1300

	println!("{:=^20}", "insensitive");
	for _ in 0..9 { wildmatch(input, pattern, false); }
	let time = Instant::now();
	let _c = wildmatch(input, pattern, false);
	println!("wildmatch {}", time.elapsed().as_nanos()); // ~500

	for _ in 0..9 { wildmatch(input.to_lowercase().as_str(), pattern.to_lowercase().as_str(), true); }
	let time = Instant::now();
	let _d = wildmatch(input.to_lowercase().as_str(), pattern.to_lowercase().as_str(), true);
	println!("wildmatch lower {}", time.elapsed().as_nanos()); // ~600

	for _ in 0..9 { WildMatch::new(pattern.to_lowercase().as_str()).matches(input.to_lowercase().as_str()); }
	let time = Instant::now();
	let _e = WildMatch::new(pattern.to_lowercase().as_str()).matches(input.to_lowercase().as_str());
	println!("WildMatch	  {}", time.elapsed().as_nanos()); // ~1400
}