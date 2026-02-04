// mod func_size;
// pub use func_size::func_size;
pub use procedural::*;

mod inpl;
#[doc(inline)]
pub use crate::impl_try_from_via_from_str;
#[doc(inline)]
pub use crate::impl_from_str_via_try_from;