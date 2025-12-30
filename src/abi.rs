//! ABI версионирование и структуры данных
//!
//! Этот модуль определяет ABI между агентом и плагинами.
//!
//! # Пример
//!
//! ```ignore
//! use milow_plugin_sdk::*;
//!
//! let cmd = CommandBuilder::new(hashes::cmd::PING_HASH)
//!     .callback(cmd_ping)
//!     .plugin_id(PluginId::from_str(b"PING"))
//!     .flags(CommandFlags::NONE)
//!     .build();
//! ```

use crate::types::{CmdHash, PluginId, CommandFlags};

/// Текущая версия ABI SDK
pub const SDK_ABI_VERSION: u32 = 2;

/// Магическое число для валидации таблицы callback'ов
pub const CALLBACK_TABLE_MAGIC: u32 = 0x4D4C5054; // "MLPT"

/// Максимальное количество команд на плагин
pub const MAX_PLUGIN_COMMANDS: usize = 8;

/// Максимальный размер результата команды (16 KB)
pub const MAX_RESULT_SIZE: usize = 16384;

/// Тип callback-функции плагина
///
/// # Arguments
/// - `task_id` - ID задачи
/// - `data` - Входные данные команды
/// - `data_len` - Длина входных данных
/// - `result_buf` - Буфер для результата
/// - `result_len` - Указатель на длину результата
///
/// # Returns
/// Код ошибки (0 = успех)
///
/// # Safety
/// Callback вызывается из unsafe контекста. Реализация должна:
/// - Не писать за пределы `result_buf` (макс. MAX_RESULT_SIZE)
/// - Корректно устанавливать `*result_len`
pub type PluginCallback = unsafe extern "C" fn(
    task_id: u64,
    data: *const u8,
    data_len: usize,
    result_buf: *mut u8,
    result_len: *mut usize,
) -> u32;

/// Команда плагина (32 байта, выровнено)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct PluginCommand {
    pub name_hash: u32,
    pub flags: u32,
    pub callback: Option<PluginCallback>,
    pub plugin_id: u64,
    pub active: bool,
    pub _reserved: [u8; 7],
}

impl PluginCommand {
    /// Создать пустую команду
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            name_hash: 0,
            flags: 0,
            callback: None,
            plugin_id: 0,
            active: false,
            _reserved: [0; 7],
        }
    }

    /// Проверить активность команды
    #[inline(always)]
    pub const fn is_active(&self) -> bool {
        self.active
    }

    /// Получить хеш команды
    #[inline(always)]
    pub const fn hash(&self) -> CmdHash {
        CmdHash::from_raw(self.name_hash)
    }

    /// Получить флаги команды
    #[inline(always)]
    pub const fn command_flags(&self) -> CommandFlags {
        CommandFlags::from_raw(self.flags)
    }
}

/// Builder для создания команд плагина
///
/// # Example
/// ```ignore
/// let cmd = CommandBuilder::new(CmdHash::new(b"mycommand"))
///     .callback(my_handler)
///     .plugin_id(PluginId::from_str(b"MYPLUGIN"))
///     .flags(CommandFlags::ELEVATED)
///     .build();
/// ```
pub struct CommandBuilder {
    name_hash: CmdHash,
    flags: CommandFlags,
    callback: Option<PluginCallback>,
    plugin_id: PluginId,
}

impl CommandBuilder {
    /// Создать builder с хешем команды
    #[inline]
    pub const fn new(hash: CmdHash) -> Self {
        Self {
            name_hash: hash,
            flags: CommandFlags::NONE,
            callback: None,
            plugin_id: PluginId::from_raw(0),
        }
    }

    /// Установить callback функцию
    #[inline]
    pub const fn callback(mut self, cb: PluginCallback) -> Self {
        self.callback = Some(cb);
        self
    }

    /// Установить ID плагина
    #[inline]
    pub const fn plugin_id(mut self, id: PluginId) -> Self {
        self.plugin_id = id;
        self
    }

    /// Установить флаги команды
    #[inline]
    pub const fn flags(mut self, flags: CommandFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Построить команду
    #[inline]
    pub const fn build(self) -> PluginCommand {
        PluginCommand {
            name_hash: self.name_hash.raw(),
            flags: self.flags.raw(),
            callback: self.callback,
            plugin_id: self.plugin_id.raw(),
            active: true,
            _reserved: [0; 7],
        }
    }
}

/// Callback-таблица плагина (272 байта)
#[repr(C)]
pub struct PluginCallbackTable {
    pub magic: u32,
    pub abi_version: u32,
    pub commands: [PluginCommand; MAX_PLUGIN_COMMANDS],
    pub count: usize,
}

impl PluginCallbackTable {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            magic: CALLBACK_TABLE_MAGIC,
            abi_version: SDK_ABI_VERSION,
            commands: [PluginCommand::empty(); MAX_PLUGIN_COMMANDS],
            count: 0,
        }
    }

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.magic == CALLBACK_TABLE_MAGIC
    }

    #[inline(always)]
    pub const fn is_abi_compatible(&self) -> bool {
        self.abi_version == SDK_ABI_VERSION
    }

    #[inline]
    pub fn register(&mut self, name_hash: u32, callback: PluginCallback, plugin_id: u64) -> bool {
        if self.count >= MAX_PLUGIN_COMMANDS {
            return false;
        }

        for cmd in self.commands.iter_mut() {
            if !cmd.active {
                cmd.name_hash = name_hash;
                cmd.flags = 0;
                cmd.callback = Some(callback);
                cmd.plugin_id = plugin_id;
                cmd.active = true;
                cmd._reserved = [0; 7];
                self.count += 1;
                return true;
            }
        }
        false
    }

    #[inline]
    pub fn register_with_flags(
        &mut self,
        name_hash: u32,
        callback: PluginCallback,
        plugin_id: u64,
        flags: u32,
    ) -> bool {
        if self.count >= MAX_PLUGIN_COMMANDS {
            return false;
        }

        for cmd in self.commands.iter_mut() {
            if !cmd.active {
                cmd.name_hash = name_hash;
                cmd.flags = flags;
                cmd.callback = Some(callback);
                cmd.plugin_id = plugin_id;
                cmd.active = true;
                cmd._reserved = [0; 7];
                self.count += 1;
                return true;
            }
        }
        false
    }
}

/// Проверить ABI версию таблицы
#[inline(always)]
pub unsafe fn check_abi_version(table: *const PluginCallbackTable) -> bool {
    if table.is_null() {
        return false;
    }
    let t = &*table;
    t.is_valid() && t.is_abi_compatible()
}
