pub trait NumExt {
	/// Remove zeros do final do número.
	/// ### Exemplo
	/// ```
	/// use common_crate::ds::NumExt;
	/// let mut n: u64 = 123000;
	/// n.trim_end_zeros();
	/// assert_eq!(n, 123);
	/// ```
	fn trim_end_zeros(&mut self);
}

macro_rules! impl_num_ext {
	($type:ty) => {
		impl NumExt for $type {
			fn trim_end_zeros(&mut self) {
				if *self != 0 {
					while *self % 10 == 0 {
						*self /= 10;
					}
				}
			}
		}	
	};
}

impl_num_ext!(usize);
impl_num_ext!(u8);
impl_num_ext!(u16);
impl_num_ext!(u32);
impl_num_ext!(u64);
impl_num_ext!(u128);
impl_num_ext!(isize);
impl_num_ext!(i8);
impl_num_ext!(i16);
impl_num_ext!(i32);
impl_num_ext!(i64);
impl_num_ext!(i128);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_trim_end_zeros() {
		let mut n: usize = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: u8 = 120;
		n.trim_end_zeros();
		assert_eq!(n, 12);
		let mut n: u16 = 12000;
		n.trim_end_zeros();
		assert_eq!(n, 12);
		let mut n: u32 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: u64 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: u128 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: i8 = 120;
		n.trim_end_zeros();
		assert_eq!(n, 12);
		let mut n: i16 = 12000;
		n.trim_end_zeros();
		assert_eq!(n, 12);
		let mut n: i32 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: i64 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
		let mut n: i128 = 123000;
		n.trim_end_zeros();
		assert_eq!(n, 123);
	}
}
