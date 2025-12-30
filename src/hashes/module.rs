//! Хеши модулей Windows (case-insensitive)

use crate::hash::djb2_ci;

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
