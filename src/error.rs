//! Коды ошибок для плагинов
//!
//! Предоставляет type-safe enum для обработки ошибок.

use core::fmt;

/// Результат выполнения команды плагина
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SdkError {
    /// Успешное выполнение
    Success = 0,
    /// Неверные аргументы
    InvalidArgs = 1,
    /// Объект не найден
    NotFound = 2,
    /// Доступ запрещён
    AccessDenied = 3,
    /// Превышено время ожидания
    Timeout = 4,
    /// Ошибка ввода-вывода
    IoError = 5,
    /// Процесс не найден
    ProcessNotFound = 6,
    /// Недостаточно памяти
    OutOfMemory = 7,
    /// Плагин не найден
    PluginNotFound = 8,
    /// Ошибка шифрования
    CryptoError = 9,
    /// Ошибка выполнения
    ExecutionError = 10,
    /// Внутренняя ошибка
    InternalError = 11,
    /// Несовместимая версия ABI
    AbiMismatch = 12,
    /// Ошибка регистрации команды
    RegistrationFailed = 13,
    /// Буфер слишком мал
    BufferTooSmall = 14,
    /// Функция API не найдена
    FunctionNotFound = 15,
    /// Модуль не найден
    ModuleNotFound = 16,
}

impl SdkError {
    /// Создать из кода ошибки
    #[inline]
    pub const fn from_code(code: u32) -> Self {
        match code {
            0 => Self::Success,
            1 => Self::InvalidArgs,
            2 => Self::NotFound,
            3 => Self::AccessDenied,
            4 => Self::Timeout,
            5 => Self::IoError,
            6 => Self::ProcessNotFound,
            7 => Self::OutOfMemory,
            8 => Self::PluginNotFound,
            9 => Self::CryptoError,
            10 => Self::ExecutionError,
            11 => Self::InternalError,
            12 => Self::AbiMismatch,
            13 => Self::RegistrationFailed,
            14 => Self::BufferTooSmall,
            15 => Self::FunctionNotFound,
            16 => Self::ModuleNotFound,
            _ => Self::InternalError,
        }
    }

    /// Получить код ошибки
    #[inline]
    pub const fn code(self) -> u32 {
        self as u32
    }

    /// Проверить успешность
    #[inline]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    /// Проверить наличие ошибки
    #[inline]
    pub const fn is_error(self) -> bool {
        !self.is_success()
    }

    /// Получить текстовое описание
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::InvalidArgs => "Invalid arguments",
            Self::NotFound => "Not found",
            Self::AccessDenied => "Access denied",
            Self::Timeout => "Timeout",
            Self::IoError => "I/O error",
            Self::ProcessNotFound => "Process not found",
            Self::OutOfMemory => "Out of memory",
            Self::PluginNotFound => "Plugin not found",
            Self::CryptoError => "Crypto error",
            Self::ExecutionError => "Execution error",
            Self::InternalError => "Internal error",
            Self::AbiMismatch => "ABI version mismatch",
            Self::RegistrationFailed => "Registration failed",
            Self::BufferTooSmall => "Buffer too small",
            Self::FunctionNotFound => "Function not found",
            Self::ModuleNotFound => "Module not found",
        }
    }
}

impl fmt::Debug for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SdkError::{:?}({})", self.as_str(), self.code())
    }
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<SdkError> for u32 {
    #[inline]
    fn from(e: SdkError) -> u32 {
        e.code()
    }
}

impl From<u32> for SdkError {
    #[inline]
    fn from(code: u32) -> Self {
        Self::from_code(code)
    }
}

// Константы для обратной совместимости
pub const SUCCESS: u32 = SdkError::Success as u32;
pub const INVALID_ARGS: u32 = SdkError::InvalidArgs as u32;
pub const NOT_FOUND: u32 = SdkError::NotFound as u32;
pub const ACCESS_DENIED: u32 = SdkError::AccessDenied as u32;
pub const TIMEOUT: u32 = SdkError::Timeout as u32;
pub const IO_ERROR: u32 = SdkError::IoError as u32;
pub const PROCESS_NOT_FOUND: u32 = SdkError::ProcessNotFound as u32;
pub const OUT_OF_MEMORY: u32 = SdkError::OutOfMemory as u32;
pub const PLUGIN_NOT_FOUND: u32 = SdkError::PluginNotFound as u32;
pub const CRYPTO_ERROR: u32 = SdkError::CryptoError as u32;
pub const EXECUTION_ERROR: u32 = SdkError::ExecutionError as u32;
pub const INTERNAL_ERROR: u32 = SdkError::InternalError as u32;
pub const ABI_MISMATCH: u32 = SdkError::AbiMismatch as u32;
pub const REGISTRATION_FAILED: u32 = SdkError::RegistrationFailed as u32;
pub const BUFFER_TOO_SMALL: u32 = SdkError::BufferTooSmall as u32;
pub const FUNCTION_NOT_FOUND: u32 = SdkError::FunctionNotFound as u32;
pub const MODULE_NOT_FOUND: u32 = SdkError::ModuleNotFound as u32;

/// Получить текстовое описание ошибки по коду
#[inline]
pub const fn as_str(code: u32) -> &'static str {
    SdkError::from_code(code).as_str()
}
