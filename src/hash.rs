//! DJB2 хеширование

/// DJB2 хеш (case-sensitive)
#[inline(always)]
pub const fn djb2(s: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    let mut i = 0;
    while i < s.len() {
        hash = hash.wrapping_mul(33).wrapping_add(s[i] as u32);
        i += 1;
    }
    hash
}

/// DJB2 хеш (case-insensitive)
#[inline(always)]
pub const fn djb2_ci(s: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    let mut i = 0;
    while i < s.len() {
        let mut c = s[i] as u32;
        if c >= 0x41 && c <= 0x5A {
            c += 0x20;
        }
        hash = hash.wrapping_mul(33).wrapping_add(c);
        i += 1;
    }
    hash
}

// Алиасы для совместимости
pub use djb2 as djb2_hash;
pub use djb2_ci as djb2_hash_ci;
