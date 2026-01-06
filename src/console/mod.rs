mod read_host;
pub use read_host::read_host;

#[cfg(windows)]
pub mod windows;

mod pause;
pub use pause::pause;
pub use pause::PAUSE_TEXT;

mod iprintln;
#[doc(inline)]
pub use crate::iprintln;
#[doc(inline)]
pub use crate::iprint;