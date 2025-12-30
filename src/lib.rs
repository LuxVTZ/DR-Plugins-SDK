//! Milow Plugin SDK v2.1
//!
//! SDK для разработки плагинов Milow на Rust.
//!
//! # Возможности
//!
//! - **ABI версионирование** - совместимость между версиями агента
//! - **API резолвер** - динамическое разрешение WinAPI
//! - **Предвычисленные хеши** - compile-time DJB2 хеши
//! - **Утилиты** - хелперы для работы с буферами
//!
//! # Пример
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use milow_plugin_sdk::*;
//!
//! const PLUGIN_ID: u64 = 0x0001;
//!
//! #[no_mangle]
//! pub unsafe extern "C" fn _start(table: *mut PluginCallbackTable) -> i32 {
//!     if !check_abi_version(table) {
//!         return error::ABI_MISMATCH as i32;
//!     }
//!     
//!     let table = &mut *table;
//!     table.register(hashes::cmd::PING, cmd_ping, PLUGIN_ID);
//!     0
//! }
//!
//! unsafe extern "C" fn cmd_ping(
//!     _task_id: u64, _data: *const u8, _data_len: usize,
//!     result_buf: *mut u8, result_len: *mut usize,
//! ) -> u32 {
//!     util::write_str(result_buf, result_len, b"pong");
//!     error::SUCCESS
//! }
//!
//! #[panic_handler]
//! fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
//! ```

#![no_std]

// Модули
mod abi;
mod hash;
mod ffi;
mod util;

pub mod error;
pub mod hashes;
pub mod win;

// Реэкспорт ABI
pub use abi::{
    SDK_ABI_VERSION,
    CALLBACK_TABLE_MAGIC,
    MAX_PLUGIN_COMMANDS,
    MAX_RESULT_SIZE,
    PluginCallback,
    PluginCommand,
    PluginCallbackTable,
    check_abi_version,
};

// Реэкспорт хешей
pub use hash::{djb2_hash, djb2_hash_ci, djb2, djb2_ci};

// Реэкспорт FFI
pub use ffi::*;

// Реэкспорт утилит
pub use util::*;

// Макросы

/// Макрос для вычисления хеша команды
#[macro_export]
macro_rules! cmd_hash {
    ($s:expr) => {
        $crate::djb2_hash($s.as_bytes())
    };
}

/// Макрос для вычисления case-insensitive хеша
#[macro_export]
macro_rules! api_hash {
    ($s:expr) => {
        $crate::djb2_hash_ci($s.as_bytes())
    };
}

/// Макрос для резолвинга функции из модуля
#[macro_export]
macro_rules! resolve_api {
    ($module:expr, $fn_type:ty, $hash:expr) => {{
        let ptr = $crate::win::get_proc_by_hash($module, $hash);
        if ptr.is_null() {
            None
        } else {
            Some(core::mem::transmute::<_, $fn_type>(ptr))
        }
    }};
}

/// Макрос для определения информации о плагине с panic handler
#[macro_export]
macro_rules! plugin_info {
    ($id:expr, $version:expr) => {
        pub const PLUGIN_ID: u64 = $id;
        pub const PLUGIN_VERSION: u64 = $version;

        #[panic_handler]
        fn panic(_: &core::panic::PanicInfo) -> ! {
            loop { core::hint::spin_loop(); }
        }
    };
}
