//! Коды ошибок для плагинов

pub const SUCCESS: u32 = 0;
pub const INVALID_ARGS: u32 = 1;
pub const NOT_FOUND: u32 = 2;
pub const ACCESS_DENIED: u32 = 3;
pub const TIMEOUT: u32 = 4;
pub const IO_ERROR: u32 = 5;
pub const PROCESS_NOT_FOUND: u32 = 6;
pub const OUT_OF_MEMORY: u32 = 7;
pub const PLUGIN_NOT_FOUND: u32 = 8;
pub const CRYPTO_ERROR: u32 = 9;
pub const EXECUTION_ERROR: u32 = 10;
pub const INTERNAL_ERROR: u32 = 11;
pub const ABI_MISMATCH: u32 = 12;
pub const REGISTRATION_FAILED: u32 = 13;
pub const BUFFER_TOO_SMALL: u32 = 14;
pub const FUNCTION_NOT_FOUND: u32 = 15;
pub const MODULE_NOT_FOUND: u32 = 16;

/// Получить текстовое описание ошибки
#[inline]
pub const fn as_str(code: u32) -> &'static str {
    match code {
        SUCCESS => "Success",
        INVALID_ARGS => "Invalid arguments",
        NOT_FOUND => "Not found",
        ACCESS_DENIED => "Access denied",
        TIMEOUT => "Timeout",
        IO_ERROR => "I/O error",
        PROCESS_NOT_FOUND => "Process not found",
        OUT_OF_MEMORY => "Out of memory",
        PLUGIN_NOT_FOUND => "Plugin not found",
        CRYPTO_ERROR => "Crypto error",
        EXECUTION_ERROR => "Execution error",
        INTERNAL_ERROR => "Internal error",
        ABI_MISMATCH => "ABI version mismatch",
        REGISTRATION_FAILED => "Registration failed",
        BUFFER_TOO_SMALL => "Buffer too small",
        FUNCTION_NOT_FOUND => "Function not found",
        MODULE_NOT_FOUND => "Module not found",
        _ => "Unknown error",
    }
}
