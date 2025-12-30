//! Type-safe wrappers для хешей и идентификаторов
//!
//! Использование newtype pattern предотвращает путаницу между разными типами хешей.

use core::fmt;

/// Хеш команды плагина (DJB2, case-sensitive)
///
/// # Example
/// ```ignore
/// let hash = CmdHash::new(b"ping");
/// // или через константу
/// const PING: CmdHash = CmdHash::from_raw(0x7c9e6865);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CmdHash(pub u32);

impl CmdHash {
    /// Вычислить хеш команды
    #[inline]
    pub const fn new(name: &[u8]) -> Self {
        Self(crate::hash::djb2(name))
    }

    /// Создать из сырого значения
    #[inline]
    pub const fn from_raw(hash: u32) -> Self {
        Self(hash)
    }

    /// Получить сырое значение
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for CmdHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CmdHash(0x{:08x})", self.0)
    }
}

/// Хеш модуля Windows (DJB2, case-insensitive)
///
/// # Example
/// ```ignore
/// let hash = ModuleHash::new(b"kernel32.dll");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleHash(pub u32);

impl ModuleHash {
    /// Вычислить хеш модуля (case-insensitive)
    #[inline]
    pub const fn new(name: &[u8]) -> Self {
        Self(crate::hash::djb2_ci(name))
    }

    /// Создать из сырого значения
    #[inline]
    pub const fn from_raw(hash: u32) -> Self {
        Self(hash)
    }

    /// Получить сырое значение
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for ModuleHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModuleHash(0x{:08x})", self.0)
    }
}

/// Хеш функции Windows API (DJB2, case-insensitive)
///
/// # Example
/// ```ignore
/// let hash = ApiHash::new(b"VirtualAlloc");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ApiHash(pub u32);

impl ApiHash {
    /// Вычислить хеш API функции (case-insensitive)
    #[inline]
    pub const fn new(name: &[u8]) -> Self {
        Self(crate::hash::djb2_ci(name))
    }

    /// Создать из сырого значения
    #[inline]
    pub const fn from_raw(hash: u32) -> Self {
        Self(hash)
    }

    /// Получить сырое значение
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for ApiHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiHash(0x{:08x})", self.0)
    }
}

/// ID плагина
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PluginId(pub u64);

impl PluginId {
    /// Создать ID из строки (до 8 символов ASCII)
    #[inline]
    pub const fn from_str(s: &[u8]) -> Self {
        let mut id: u64 = 0;
        let len = if s.len() > 8 { 8 } else { s.len() };
        let mut i = 0;
        while i < len {
            id |= (s[i] as u64) << (i * 8);
            i += 1;
        }
        Self(id)
    }

    /// Создать из числа
    #[inline]
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// Получить сырое значение
    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for PluginId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PluginId(0x{:016x})", self.0)
    }
}

/// Флаги команды плагина
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandFlags(pub u32);

impl CommandFlags {
    /// Нет флагов
    pub const NONE: Self = Self(0);
    /// Команда требует повышенных привилегий
    pub const ELEVATED: Self = Self(1 << 0);
    /// Команда может быть длительной
    pub const LONG_RUNNING: Self = Self(1 << 1);
    /// Команда изменяет состояние системы
    pub const DESTRUCTIVE: Self = Self(1 << 2);
    /// Команда требует сетевого доступа
    pub const NETWORK: Self = Self(1 << 3);
    /// Команда работает с файловой системой
    pub const FILESYSTEM: Self = Self(1 << 4);

    /// Создать из сырого значения
    #[inline]
    pub const fn from_raw(flags: u32) -> Self {
        Self(flags)
    }

    /// Получить сырое значение
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Проверить наличие флага
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Объединить флаги
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl core::ops::BitOr for CommandFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for CommandFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl fmt::Debug for CommandFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandFlags(0x{:08x})", self.0)
    }
}
