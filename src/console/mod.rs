mod read_host;
pub use read_host::read_host;

mod call_console;
#[cfg(windows)]
pub use call_console::call_console;

mod pause;
pub use pause::pause;
pub use pause::PAUSE_TEXT;

mod iprintln;
#[doc(inline)]
pub use crate::iprintln;
#[doc(inline)]
pub use crate::iprint;