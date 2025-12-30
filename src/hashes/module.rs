//! Хеши модулей Windows (case-insensitive)
//!
//! Предвычисленные DJB2 хеши для системных DLL.

use crate::hash::djb2_ci;
use crate::types::ModuleHash;

// Type-safe хеши
pub const KERNEL32: ModuleHash = ModuleHash::from_raw(djb2_ci(b"kernel32.dll"));
pub const NTDLL: ModuleHash = ModuleHash::from_raw(djb2_ci(b"ntdll.dll"));
pub const USER32: ModuleHash = ModuleHash::from_raw(djb2_ci(b"user32.dll"));
pub const GDI32: ModuleHash = ModuleHash::from_raw(djb2_ci(b"gdi32.dll"));
pub const ADVAPI32: ModuleHash = ModuleHash::from_raw(djb2_ci(b"advapi32.dll"));
pub const WS2_32: ModuleHash = ModuleHash::from_raw(djb2_ci(b"ws2_32.dll"));
pub const IPHLPAPI: ModuleHash = ModuleHash::from_raw(djb2_ci(b"iphlpapi.dll"));

// Raw u32 хеши для обратной совместимости
pub const KERNEL32_DLL: u32 = djb2_ci(b"kernel32.dll");
pub const NTDLL_DLL: u32 = djb2_ci(b"ntdll.dll");
pub const USER32_DLL: u32 = djb2_ci(b"user32.dll");
pub const GDI32_DLL: u32 = djb2_ci(b"gdi32.dll");
pub const ADVAPI32_DLL: u32 = djb2_ci(b"advapi32.dll");
pub const WS2_32_DLL: u32 = djb2_ci(b"ws2_32.dll");
pub const WINHTTP_DLL: u32 = djb2_ci(b"winhttp.dll");
pub const WININET_DLL: u32 = djb2_ci(b"wininet.dll");
pub const SHELL32_DLL: u32 = djb2_ci(b"shell32.dll");
pub const OLE32_DLL: u32 = djb2_ci(b"ole32.dll");
pub const MSVCRT_DLL: u32 = djb2_ci(b"msvcrt.dll");
pub const IPHLPAPI_DLL: u32 = djb2_ci(b"iphlpapi.dll");
pub const PSAPI_DLL: u32 = djb2_ci(b"psapi.dll");
