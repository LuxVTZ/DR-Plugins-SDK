# Milow Plugin SDK

SDK для разработки плагинов Milow Agent на Rust.

## Возможности

- **no_std** - работает без стандартной библиотеки
- **Type Safety** - newtype wrappers для хешей (`CmdHash`, `ModuleHash`, `ApiHash`)
- **Builder Pattern** - удобное создание команд через `CommandBuilder`
- **ABI версионирование** - совместимость между версиями агента
- **API резолвер** - динамическое разрешение WinAPI через PEB
- **Предвычисленные хеши** - compile-time DJB2 хеши
- **Утилиты** - хелперы для работы с буферами

## Структура

```
src/
├── lib.rs          # Главный модуль, реэкспорты, макросы
├── abi.rs          # ABI (PluginCallbackTable, CommandBuilder)
├── types.rs        # Type-safe wrappers (CmdHash, PluginId, CommandFlags)
├── error.rs        # SdkError enum + коды ошибок
├── hash.rs         # DJB2 хеширование
├── ffi.rs          # Windows типы и константы
├── util.rs         # Утилиты (write_*, mem*, strlen)
├── hashes/
│   ├── cmd.rs      # Хеши команд (PING_HASH, SHELL_HASH, etc.)
│   ├── module.rs   # Хеши модулей (KERNEL32, NTDLL, etc.)
│   └── api.rs      # Хеши WinAPI функций
└── win/
    ├── peb.rs      # PEB/TEB доступ
    ├── resolver.rs # Динамический резолвинг API
    └── modules.rs  # Хелперы для модулей
```

## Быстрый старт

```rust
#![no_std]
#![no_main]

use milow_plugin_sdk::*;

const PLUGIN_ID: PluginId = PluginId::from_str(b"PING");

#[no_mangle]
pub unsafe extern "C" fn _start(table: *mut PluginCallbackTable) -> i32 {
    if !check_abi_version(table) {
        return SdkError::AbiMismatch.code() as i32;
    }

    let table = &mut *table;

    // Builder pattern (рекомендуется)
    let cmd = CommandBuilder::new(hashes::cmd::PING_HASH)
        .callback(cmd_ping)
        .plugin_id(PLUGIN_ID)
        .flags(CommandFlags::NONE)
        .build();

    for slot in table.commands.iter_mut() {
        if !slot.is_active() {
            *slot = cmd;
            table.count += 1;
            return 0;
        }
    }

    SdkError::RegistrationFailed.code() as i32
}

unsafe extern "C" fn cmd_ping(
    _task_id: u64, _data: *const u8, _data_len: usize,
    result_buf: *mut u8, result_len: *mut usize,
) -> u32 {
    write_str(result_buf, result_len, b"pong");
    SdkError::Success.code()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
```

## API Reference

### Type-Safe Types

| Тип | Описание |
|-----|----------|
| `CmdHash` | Хеш команды (case-sensitive) |
| `ModuleHash` | Хеш модуля Windows (case-insensitive) |
| `ApiHash` | Хеш API функции (case-insensitive) |
| `PluginId` | ID плагина (8 bytes) |
| `CommandFlags` | Флаги команды (bitflags) |
| `SdkError` | Enum кодов ошибок |

### CommandFlags

```rust
CommandFlags::NONE        // Нет флагов
CommandFlags::ELEVATED    // Требует привилегий
CommandFlags::LONG_RUNNING // Длительная операция
CommandFlags::DESTRUCTIVE  // Изменяет систему
CommandFlags::NETWORK      // Требует сеть
CommandFlags::FILESYSTEM   // Работа с FS
```

### SdkError

```rust
SdkError::Success         // 0 - Успех
SdkError::InvalidArgs     // 1 - Неверные аргументы
SdkError::NotFound        // 2 - Не найдено
SdkError::AccessDenied    // 3 - Доступ запрещён
SdkError::AbiMismatch     // 12 - Несовместимый ABI
// ... и другие
```

### Функции win модуля

| Функция | Описание |
|---------|----------|
| `win::peb()` | Получить PEB |
| `win::kernel32()` | kernel32.dll |
| `win::ntdll()` | ntdll.dll |
| `win::iphlpapi()` | iphlpapi.dll |
| `win::get_module_by_hash(hash)` | Найти модуль |
| `win::get_proc_by_hash(mod, hash)` | Найти функцию |

### Утилиты

| Функция | Описание |
|---------|----------|
| `write_str(buf, len, data)` | Записать строку |
| `write_decimal(val, dst, offset)` | Записать число |
| `copy_str(src, dst, offset, max)` | Копировать строку |
| `utf16_to_ascii(src, len, dst, max)` | UTF-16 → ASCII |
| `memzero(ptr, len)` | Обнулить память |

## Сборка плагина

```toml
# Cargo.toml
[package]
name = "my-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
milow-plugin-sdk = { path = "../milow-plugin-sdk" }

[profile.release]
opt-level = "z"
lto = true
panic = "abort"
strip = true
```

```toml
# .cargo/config.toml
[build]
target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
rustflags = ["-C", "link-arg=-nostdlib", "-C", "link-arg=-Wl,-e,_start"]
```

## License

MIT
