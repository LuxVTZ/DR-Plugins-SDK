//! Доступ к PEB

use core::arch::asm;

/// Получить указатель на PEB через GS регистр (x64)
#[inline(always)]
pub unsafe fn get_peb() -> *mut u8 {
    let peb: *mut u8;
    asm!(
        "mov {}, gs:[0x60]",
        out(reg) peb,
        options(nostack, nomem, preserves_flags)
    );
    peb
}

/// Получить указатель на TEB через GS регистр (x64)
#[inline(always)]
pub unsafe fn get_teb() -> *mut u8 {
    let teb: *mut u8;
    asm!(
        "mov {}, gs:[0x30]",
        out(reg) teb,
        options(nostack, nomem, preserves_flags)
    );
    teb
}

/// Получить текущий PID из TEB
#[inline(always)]
pub unsafe fn get_current_pid() -> u32 {
    let teb = get_teb();
    if teb.is_null() {
        return 0;
    }
    // TEB->ClientId.UniqueProcess at offset 0x40
    *(teb.add(0x40) as *const usize) as u32
}

/// Получить текущий TID из TEB
#[inline(always)]
pub unsafe fn get_current_tid() -> u32 {
    let teb = get_teb();
    if teb.is_null() {
        return 0;
    }
    // TEB->ClientId.UniqueThread at offset 0x48
    *(teb.add(0x48) as *const usize) as u32
}
