//! Модуль для работы с Windows API
//!
//! Предоставляет функции для динамического разрешения API без импортов.

mod peb;
mod resolver;
mod modules;

pub use peb::{get_peb, get_teb, get_current_pid, get_current_tid};
pub use resolver::{get_module_by_hash, get_proc_by_hash, load_library};
pub use modules::*;
