//! Хеши функций Windows API (case-insensitive)

use crate::hash::djb2_ci;

// Kernel32
pub const LOADLIBRARYA: u32 = djb2_ci(b"LoadLibraryA");
pub const LOADLIBRARYW: u32 = djb2_ci(b"LoadLibraryW");
pub const GETPROCADDRESS: u32 = djb2_ci(b"GetProcAddress");
pub const FREELIBRARY: u32 = djb2_ci(b"FreeLibrary");
pub const VIRTUALALLOC: u32 = djb2_ci(b"VirtualAlloc");
pub const VIRTUALFREE: u32 = djb2_ci(b"VirtualFree");
pub const VIRTUALPROTECT: u32 = djb2_ci(b"VirtualProtect");
pub const GETLASTERROR: u32 = djb2_ci(b"GetLastError");
pub const SETLASTERROR: u32 = djb2_ci(b"SetLastError");
pub const CLOSEHANDLE: u32 = djb2_ci(b"CloseHandle");
pub const CREATEFILEA: u32 = djb2_ci(b"CreateFileA");
pub const CREATEFILEW: u32 = djb2_ci(b"CreateFileW");
pub const READFILE: u32 = djb2_ci(b"ReadFile");
pub const WRITEFILE: u32 = djb2_ci(b"WriteFile");
pub const GETFILESIZE: u32 = djb2_ci(b"GetFileSize");
pub const SLEEP: u32 = djb2_ci(b"Sleep");
pub const EXITPROCESS: u32 = djb2_ci(b"ExitProcess");
pub const GETCURRENTPROCESSID: u32 = djb2_ci(b"GetCurrentProcessId");
pub const GETMODULEHANDLEA: u32 = djb2_ci(b"GetModuleHandleA");
pub const GETMODULEFILENAMEA: u32 = djb2_ci(b"GetModuleFileNameA");
pub const GETTEMPPATH: u32 = djb2_ci(b"GetTempPathA");
pub const CREATEPROCESSA: u32 = djb2_ci(b"CreateProcessA");
pub const OPENPROCESS: u32 = djb2_ci(b"OpenProcess");
pub const TERMINATEPROCESS: u32 = djb2_ci(b"TerminateProcess");
pub const WAITFORSINGLEOBJECT: u32 = djb2_ci(b"WaitForSingleObject");
pub const GETCOMPUTERNAMEA: u32 = djb2_ci(b"GetComputerNameA");
pub const GETUSERNAMEA: u32 = djb2_ci(b"GetUserNameA");
pub const GETSYSTEMINFO: u32 = djb2_ci(b"GetSystemInfo");
pub const GLOBALALLOC: u32 = djb2_ci(b"GlobalAlloc");
pub const GLOBALFREE: u32 = djb2_ci(b"GlobalFree");
pub const GLOBALLOCK: u32 = djb2_ci(b"GlobalLock");
pub const GLOBALUNLOCK: u32 = djb2_ci(b"GlobalUnlock");
pub const GETLOGICALDRIVES: u32 = djb2_ci(b"GetLogicalDrives");
pub const GETDISKFREESPACEEXW: u32 = djb2_ci(b"GetDiskFreeSpaceExW");
pub const GETDRIVETYPEW: u32 = djb2_ci(b"GetDriveTypeW");

// User32
pub const GETDC: u32 = djb2_ci(b"GetDC");
pub const RELEASEDC: u32 = djb2_ci(b"ReleaseDC");
pub const GETSYSTEMMETRICS: u32 = djb2_ci(b"GetSystemMetrics");
pub const GETASYNCKEYSTATE: u32 = djb2_ci(b"GetAsyncKeyState");
pub const GETKEYSTATE: u32 = djb2_ci(b"GetKeyState");
pub const GETFOREGROUNDWINDOW: u32 = djb2_ci(b"GetForegroundWindow");
pub const GETWINDOWTEXTA: u32 = djb2_ci(b"GetWindowTextA");
pub const GETWINDOWTEXTW: u32 = djb2_ci(b"GetWindowTextW");
pub const MESSAGEBOXA: u32 = djb2_ci(b"MessageBoxA");
pub const FINDWINDOWA: u32 = djb2_ci(b"FindWindowA");
pub const SHOWWINDOW: u32 = djb2_ci(b"ShowWindow");
pub const SETWINDOWTEXTA: u32 = djb2_ci(b"SetWindowTextA");

// GDI32
pub const CREATECOMPATIBLEDC: u32 = djb2_ci(b"CreateCompatibleDC");
pub const CREATECOMPATIBLEBITMAP: u32 = djb2_ci(b"CreateCompatibleBitmap");
pub const SELECTOBJECT: u32 = djb2_ci(b"SelectObject");
pub const BITBLT: u32 = djb2_ci(b"BitBlt");
pub const STRETCHBLT: u32 = djb2_ci(b"StretchBlt");
pub const GETDIBITS: u32 = djb2_ci(b"GetDIBits");
pub const DELETEOBJECT: u32 = djb2_ci(b"DeleteObject");
pub const DELETEDC: u32 = djb2_ci(b"DeleteDC");

// Advapi32
pub const REGOPENKEYEXA: u32 = djb2_ci(b"RegOpenKeyExA");
pub const REGSETVALUEEXA: u32 = djb2_ci(b"RegSetValueExA");
pub const REGQUERYVALUEEXA: u32 = djb2_ci(b"RegQueryValueExA");
pub const REGCLOSEKEY: u32 = djb2_ci(b"RegCloseKey");
pub const REGCREATEKEYEXA: u32 = djb2_ci(b"RegCreateKeyExA");
pub const REGDELETEKEYEXA: u32 = djb2_ci(b"RegDeleteKeyExA");
pub const REGDELETEVALUEA: u32 = djb2_ci(b"RegDeleteValueA");
pub const OPENPROCESSTOKEN: u32 = djb2_ci(b"OpenProcessToken");
pub const GETTOKENINFORMATION: u32 = djb2_ci(b"GetTokenInformation");
pub const LOOKUPACCOUNTSIDA: u32 = djb2_ci(b"LookupAccountSidA");

// NTDLL
pub const NTQUERYSYSTEMINFORMATION: u32 = djb2_ci(b"NtQuerySystemInformation");
pub const NTQUERYINFORMATIONPROCESS: u32 = djb2_ci(b"NtQueryInformationProcess");
pub const NTCREATETHREADEX: u32 = djb2_ci(b"NtCreateThreadEx");
pub const NTALLOCATEVIRTUALMEMORY: u32 = djb2_ci(b"NtAllocateVirtualMemory");
pub const NTFREEVIRTUALMEMORY: u32 = djb2_ci(b"NtFreeVirtualMemory");
pub const NTPROTECTVIRTUALMEMORY: u32 = djb2_ci(b"NtProtectVirtualMemory");
pub const NTWRITEVIRTUALMEMORY: u32 = djb2_ci(b"NtWriteVirtualMemory");
pub const NTREADVIRTUALMEMORY: u32 = djb2_ci(b"NtReadVirtualMemory");
pub const RTLGETVERSION: u32 = djb2_ci(b"RtlGetVersion");

// iphlpapi
pub const GETADAPTERSADDRESSES: u32 = djb2_ci(b"GetAdaptersAddresses");
pub const GETADAPTERSINFO: u32 = djb2_ci(b"GetAdaptersInfo");
