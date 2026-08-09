//! FileSystem module

mod expand_path;
pub use expand_path::expand_path;

mod copy_dir;
pub use copy_dir::copy_dir;

pub mod metaprogramming;

mod find_in_path;
pub use find_in_path::find_in_path;

pub mod bytes;

mod path_ext;
pub use path_ext::PathExt;
pub use path_ext::VERBATIM;

#[cfg(windows)]
pub mod windows;

mod open;
pub use open::open;
pub use open::open2;