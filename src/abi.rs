//! ABI версионирование и структуры данных
//!
//! Этот модуль определяет ABI между агентом и плагинами.

/// Текущая версия ABI SDK
pub const SDK_ABI_VERSION: u32 = 2;

/// Магическое число для валидации таблицы callback'ов
pub const CALLBACK_TABLE_MAGIC: u32 = 0x4D4C5054; // "MLPT"

/// Максимальное количество команд на плагин
pub const MAX_PLUGIN_COMMANDS: usize = 8;

/// Максимальный размер результата команды (16 KB)
pub const MAX_RESULT_SIZE: usize = 16384;

/// Тип callback-функции плагина
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

    #[inline(always)]
    pub const fn is_active(&self) -> bool {
        self.active
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
