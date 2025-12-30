//! Пример простого плагина ping/pong
//!
//! Демонстрирует базовое использование SDK:
//! - Регистрацию команды
//! - Обработку callback
//! - Возврат результата

#![no_std]
#![no_main]

use milow_plugin_sdk::*;

/// ID плагина (ASCII: "PING")
const PLUGIN_ID: PluginId = PluginId::from_str(b"PING");

/// Entry point плагина
///
/// # Safety
/// Вызывается агентом при загрузке плагина.
#[no_mangle]
pub unsafe extern "C" fn _start(table: *mut PluginCallbackTable) -> i32 {
    // Проверяем ABI совместимость
    if !check_abi_version(table) {
        return SdkError::AbiMismatch.code() as i32;
    }

    let table = &mut *table;

    // Способ 1: Регистрация через builder (рекомендуется)
    let cmd = CommandBuilder::new(hashes::cmd::PING_HASH)
        .callback(cmd_ping)
        .plugin_id(PLUGIN_ID)
        .flags(CommandFlags::NONE)
        .build();

    // Находим свободный слот и регистрируем
    for slot in table.commands.iter_mut() {
        if !slot.is_active() {
            *slot = cmd;
            table.count += 1;
            return 0;
        }
    }

    // Способ 2: Простая регистрация (для совместимости)
    // table.register(hashes::cmd::PING, cmd_ping, PLUGIN_ID.raw());

    SdkError::RegistrationFailed.code() as i32
}

/// Обработчик команды ping
///
/// # Safety
/// - `result_buf` должен быть валидным указателем на буфер размером MAX_RESULT_SIZE
/// - `result_len` должен быть валидным указателем
unsafe extern "C" fn cmd_ping(
    _task_id: u64,
    _data: *const u8,
    _data_len: usize,
    result_buf: *mut u8,
    result_len: *mut usize,
) -> u32 {
    // Записываем ответ
    write_str(result_buf, result_len, b"pong");
    SdkError::Success.code()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
