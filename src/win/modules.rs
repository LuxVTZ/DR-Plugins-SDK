//! Функции для получения стандартных модулей Windows

use super::resolver::{get_module_by_hash, load_library};
use crate::hashes::module;

/// Получить kernel32.dll
#[inline(always)]
pub unsafe fn get_kernel32() -> *mut u8 {
    get_module_by_hash(module::KERNEL32_DLL)
}

/// Получить ntdll.dll
#[inline(always)]
pub unsafe fn get_ntdll() -> *mut u8 {
    get_module_by_hash(module::NTDLL_DLL)
}

/// Получить user32.dll (загружает если не загружен)
#[inline]
pub unsafe fn get_user32() -> *mut u8 {
    let user32 = get_module_by_hash(module::USER32_DLL);
    if !user32.is_null() {
        return user32;
    }
    let kernel32 = get_kernel32();
    if kernel32.is_null() {
        return core::ptr::null_mut();
    }
    load_library(kernel32, b"user32.dll\0")
}

/// Получить gdi32.dll (загружает если не загружен)
#[inline]
pub unsafe fn get_gdi32() -> *mut u8 {
    let gdi32 = get_module_by_hash(module::GDI32_DLL);
    if !gdi32.is_null() {
        return gdi32;
    }
    let kernel32 = get_kernel32();
    if kernel32.is_null() {
        return core::ptr::null_mut();
    }
    load_library(kernel32, b"gdi32.dll\0")
}

/// Получить advapi32.dll (загружает если не загружен)
#[inline]
pub unsafe fn get_advapi32() -> *mut u8 {
    let advapi32 = get_module_by_hash(module::ADVAPI32_DLL);
    if !advapi32.is_null() {
        return advapi32;
    }
    let kernel32 = get_kernel32();
    if kernel32.is_null() {
        return core::ptr::null_mut();
    }
    load_library(kernel32, b"advapi32.dll\0")
}

/// Получить iphlpapi.dll (загружает если не загружен)
#[inline]
pub unsafe fn get_iphlpapi() -> *mut u8 {
    let iphlpapi = get_module_by_hash(module::IPHLPAPI_DLL);
    if !iphlpapi.is_null() {
        return iphlpapi;
    }
    let kernel32 = get_kernel32();
    if kernel32.is_null() {
        return core::ptr::null_mut();
    }
    load_library(kernel32, b"iphlpapi.dll\0")
}

/// Получить ws2_32.dll (загружает если не загружен)
#[inline]
pub unsafe fn get_ws2_32() -> *mut u8 {
    let ws2_32 = get_module_by_hash(module::WS2_32_DLL);
    if !ws2_32.is_null() {
        return ws2_32;
    }
    let kernel32 = get_kernel32();
    if kernel32.is_null() {
        return core::ptr::null_mut();
    }
    load_library(kernel32, b"ws2_32.dll\0")
}
