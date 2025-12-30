//! Утилиты для плагинов

/// Записать данные в result buffer
#[inline(always)]
pub unsafe fn write_result(buf: *mut u8, len: *mut usize, data: &[u8]) {
    for (i, &byte) in data.iter().enumerate() {
        *buf.add(i) = byte;
    }
    *len = data.len();
}

/// Записать строку в result buffer (алиас)
#[inline(always)]
pub unsafe fn write_str(buf: *mut u8, len: *mut usize, data: &[u8]) {
    write_result(buf, len, data);
}

/// Записать число как строку
#[inline]
pub unsafe fn write_num(buf: *mut u8, len: *mut usize, mut num: usize) {
    if num == 0 {
        *buf = b'0';
        *len = 1;
        return;
    }

    let mut temp = [0u8; 20];
    let mut i = 0;

    while num > 0 {
        temp[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    for j in 0..i {
        *buf.add(j) = temp[i - 1 - j];
    }
    *len = i;
}

/// Записать u64 как десятичную строку в буфер
#[inline]
pub unsafe fn write_decimal(mut value: u64, dst: *mut u8, offset: &mut usize) {
    if value == 0 {
        *dst.add(*offset) = b'0';
        *offset += 1;
        return;
    }
    
    let mut buf = [0u8; 20];
    let mut pos = 20;
    
    while value > 0 {
        pos -= 1;
        buf[pos] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    
    let len = 20 - pos;
    core::ptr::copy_nonoverlapping(buf.as_ptr().add(pos), dst.add(*offset), len);
    *offset += len;
}

/// Записать hex byte
#[inline]
pub unsafe fn write_hex_byte(value: u8, dst: *mut u8, offset: &mut usize) {
    const HEX: &[u8] = b"0123456789ABCDEF";
    *dst.add(*offset) = HEX[(value >> 4) as usize];
    *offset += 1;
    *dst.add(*offset) = HEX[(value & 0xF) as usize];
    *offset += 1;
}

/// Записать IPv4 адрес
#[inline]
pub unsafe fn write_ipv4(addr: [u8; 4], dst: *mut u8, offset: &mut usize) {
    for i in 0..4 {
        if i > 0 { *dst.add(*offset) = b'.'; *offset += 1; }
        write_decimal(addr[i] as u64, dst, offset);
    }
}

/// Записать размер в GB
#[inline]
pub unsafe fn write_size_gb(bytes: u64, dst: *mut u8, offset: &mut usize) {
    let gb = bytes / (1024 * 1024 * 1024);
    let mb_remainder = (bytes % (1024 * 1024 * 1024)) / (1024 * 1024 * 100);
    
    write_decimal(gb, dst, offset);
    *dst.add(*offset) = b'.';
    *offset += 1;
    write_decimal(mb_remainder, dst, offset);
    copy_str(b" GB", dst, offset, 16384);
}

/// Копировать строку в буфер
#[inline]
pub unsafe fn copy_str(src: &[u8], dst: *mut u8, offset: &mut usize, max_len: usize) {
    let len = src.len().min(max_len - *offset);
    if len > 0 {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst.add(*offset), len);
        *offset += len;
    }
}

/// Конвертировать UTF-16 в ASCII
#[inline]
pub unsafe fn utf16_to_ascii(src: *const u16, src_len: usize, dst: *mut u8, max_len: usize) -> usize {
    let len = src_len.min(max_len);
    for i in 0..len {
        let c = *src.add(i);
        *dst.add(i) = if c < 128 { c as u8 } else { b'?' };
    }
    len
}

/// Прочитать входные данные как slice
#[inline(always)]
pub unsafe fn read_input<'a>(data: *const u8, len: usize) -> &'a [u8] {
    if data.is_null() || len == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(data, len)
    }
}

/// Копировать память
#[inline(always)]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, len: usize) {
    for i in 0..len {
        *dest.add(i) = *src.add(i);
    }
}

/// Обнулить память (volatile)
#[inline(always)]
pub unsafe fn memzero(dest: *mut u8, len: usize) {
    for i in 0..len {
        core::ptr::write_volatile(dest.add(i), 0);
    }
}

/// Сравнить память
#[inline(always)]
pub unsafe fn memcmp(a: *const u8, b: *const u8, len: usize) -> bool {
    for i in 0..len {
        if *a.add(i) != *b.add(i) {
            return false;
        }
    }
    true
}

/// Длина null-terminated строки
#[inline(always)]
pub unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}
