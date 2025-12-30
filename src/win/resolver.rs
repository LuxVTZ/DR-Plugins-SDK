//! Динамическое разрешение API

use super::peb::get_peb;
use crate::hash::djb2_ci;
use crate::hashes;

/// Получить базовый адрес модуля по хешу имени
#[inline(never)]
pub unsafe fn get_module_by_hash(name_hash: u32) -> *mut u8 {
    let peb = get_peb();
    if peb.is_null() {
        return core::ptr::null_mut();
    }

    // PEB->Ldr (offset 0x18 на x64)
    let ldr = *(peb.add(0x18) as *const *mut u8);
    if ldr.is_null() {
        return core::ptr::null_mut();
    }

    #[repr(C)]
    struct ListEntry {
        flink: *mut ListEntry,
        blink: *mut ListEntry,
    }

    let list_head = ldr.add(0x10) as *const ListEntry;
    let mut current = (*list_head).flink;

    while !current.is_null() && current != list_head as *mut ListEntry {
        let entry = current as *mut u8;
        let name_len = *(entry.add(0x58) as *const u16) as usize / 2;
        let name_buf = *(entry.add(0x60) as *const *const u16);

        if !name_buf.is_null() && name_len > 0 && name_len <= 64 {
            let mut ascii_buf = [0u8; 64];
            for i in 0..name_len {
                ascii_buf[i] = (*name_buf.add(i) & 0xFF) as u8;
            }

            if djb2_ci(&ascii_buf[..name_len]) == name_hash {
                return *(entry.add(0x30) as *const *mut u8);
            }
        }
        current = (*current).flink;
    }
    core::ptr::null_mut()
}

/// Получить адрес функции в модуле по хешу имени
#[inline(never)]
pub unsafe fn get_proc_by_hash(module_base: *mut u8, name_hash: u32) -> *mut u8 {
    if module_base.is_null() {
        return core::ptr::null_mut();
    }

    // DOS signature
    if *(module_base as *const u16) != 0x5A4D {
        return core::ptr::null_mut();
    }

    // PE header
    let pe_offset = *(module_base.add(0x3C) as *const u32) as usize;
    let pe_header = module_base.add(pe_offset);

    // PE signature
    if *(pe_header as *const u32) != 0x00004550 {
        return core::ptr::null_mut();
    }

    // Export Directory RVA (offset 0x88 для x64)
    let export_rva = *(pe_header.add(0x88) as *const u32) as usize;
    if export_rva == 0 {
        return core::ptr::null_mut();
    }

    let export_dir = module_base.add(export_rva);
    let number_of_names = *(export_dir.add(0x18) as *const u32) as usize;
    let address_of_functions = *(export_dir.add(0x1C) as *const u32) as usize;
    let address_of_names = *(export_dir.add(0x20) as *const u32) as usize;
    let address_of_name_ordinals = *(export_dir.add(0x24) as *const u32) as usize;

    let names = module_base.add(address_of_names) as *const u32;
    let ordinals = module_base.add(address_of_name_ordinals) as *const u16;
    let functions = module_base.add(address_of_functions) as *const u32;

    for i in 0..number_of_names {
        let name_rva = *names.add(i) as usize;
        let name_ptr = module_base.add(name_rva);

        let mut len = 0usize;
        while *name_ptr.add(len) != 0 && len < 256 {
            len += 1;
        }

        if djb2_ci(core::slice::from_raw_parts(name_ptr, len)) == name_hash {
            let ordinal = *ordinals.add(i) as usize;
            let func_rva = *functions.add(ordinal) as usize;
            return module_base.add(func_rva);
        }
    }
    core::ptr::null_mut()
}

/// Загрузить модуль по имени
#[inline]
pub unsafe fn load_library(kernel32: *mut u8, name: &[u8]) -> *mut u8 {
    type LoadLibraryAFn = unsafe extern "system" fn(*const u8) -> *mut u8;

    let ptr = get_proc_by_hash(kernel32, hashes::api::LOADLIBRARYA);
    if ptr.is_null() {
        return core::ptr::null_mut();
    }

    let func: LoadLibraryAFn = core::mem::transmute(ptr);
    func(name.as_ptr())
}
