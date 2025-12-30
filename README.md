# Milow Plugin SDK

SDK для разработки плагинов Milow Agent на Rust.

## Возможности

- **no_std** - работает без стандартной библиотеки
- **ABI версионирование** - совместимость между версиями агента
- **API резолвер** - динамическое разрешение WinAPI через PEB
- **Предвычисленные хеши** - compile-time DJB2 хеши для команд и API
- **Утилиты** - хелперы для работы с буферами и памятью

## Структура

```
src/
├── lib.rs          # Главный модуль, реэкспорты
├── abi.rs          # ABI структуры (PluginCallbackTable, PluginCommand)
├── error.rs        # Коды ошибок
├── hash.rs         # DJB2 хеширование
├── ffi.rs          # Windows типы и константы
├── util.rs         # Утилиты (write_*, mem*, strlen)
├── hashes/
│   ├── mod.rs      # Модуль хешей
│   ├── cmd.rs      # Хеши команд (ping, shell, etc.)
│   ├── module.rs   # Хеши модулей (kernel32.dll, ntdll.dll)
│   └── api.rs      # Хеши WinAPI (VirtualAlloc, NtQuerySystemInformation)
└── win/
    ├── mod.rs      # Модуль Windows
    ├── peb.rs      # Доступ к PEB/TEB
    ├── resolver.rs # get_module_by_hash, get_proc_by_hash
    └── modules.rs  # get_kernel32, get_ntdll, get_iphlpapi, etc.
```

## Использование

```rust
#![no_std]
#![no_main]

use milow_plugin_sdk::*;

const PLUGIN_ID: u64 = 0x50494E47; // "PING"

#[no_mangle]
pub unsafe extern "C" fn _start(table: *mut PluginCallbackTable) -> i32 {
    if !check_abi_version(table) {
        return error::ABI_MISMATCH as i32;
    }
    
    let table = &mut *table;
    
    if !table.register(hashes::cmd::PING, cmd_ping, PLUGIN_ID) {
        return error::REGISTRATION_FAILED as i32;
    }
    
    0
}

unsafe extern "C" fn cmd_ping(
    _task_id: u64,
    _data: *const u8,
    _data_len: usize,
    result_buf: *mut u8,
    result_len: *mut usize,
) -> u32 {
    write_str(result_buf, result_len, b"pong");
    error::SUCCESS
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

## API Reference

### ABI

| Константа | Значение | Описание |
|-----------|----------|----------|
| `SDK_ABI_VERSION` | 2 | Версия ABI |
| `MAX_PLUGIN_COMMANDS` | 8 | Макс. команд на плагин |
| `MAX_RESULT_SIZE` | 16384 | Макс. размер результата |

### Коды ошибок

| Код | Значение | Описание |
|-----|----------|----------|
| `SUCCESS` | 0 | Успех |
| `INVALID_ARGS` | 1 | Неверные аргументы |
| `NOT_FOUND` | 2 | Не найдено |
| `OUT_OF_MEMORY` | 7 | Недостаточно памяти |
| `ABI_MISMATCH` | 12 | Несовместимая версия |

### Функции win модуля

| Функция | Описание |
|---------|----------|
| `win::get_peb()` | Получить PEB |
| `win::get_kernel32()` | kernel32.dll |
| `win::get_ntdll()` | ntdll.dll |
| `win::get_iphlpapi()` | iphlpapi.dll |
| `win::get_module_by_hash(hash)` | Найти модуль по хешу |
| `win::get_proc_by_hash(module, hash)` | Найти функцию по хешу |

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
milow-plugin-sdk = { git = "https://github.com/LuxVTZ/milow-plugin-sdk" }

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
rustflags = ["-C", "link-arg=-nostdlib", "-C", "link-arg=-Wl,-e,_start"]
```

## License

MIT
