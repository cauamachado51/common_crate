mod test_path;
pub use test_path::test_path;
pub use test_path::test_file;
pub use test_path::test_dir;
pub use test_path::test_symlink;
pub use test_path::expand_path;

mod console;
#[cfg(windows)]
pub use console::call_console;
pub use console::read_host;
pub use console::read_host2;
pub use console::pause;
pub use console::TEXT_PAUSE;

#[cfg(windows)]
mod language;
#[cfg(windows)]
pub use language::get_all_languages;
#[cfg(windows)]
pub use language::get_current_language;
#[cfg(windows)]
pub use language::get_installed_languages;
#[cfg(windows)]
pub use language::verify_user_language;

mod data_system;
pub use data_system::key_value;
pub use data_system::parse_array;