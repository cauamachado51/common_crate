//! DataSystem module

mod parse_array;
pub use parse_array::parse_array;

mod parse_vec;
pub use parse_vec::parse_vec;

mod wildmatch;
pub use wildmatch::wildmatch;