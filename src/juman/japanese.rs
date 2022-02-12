#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::ctools::{exit, fprintf, free, malloc, stderr, strdup, strlen};

static mut stable: [[libc::c_int; 2]; 63] =
    [[129 as libc::c_int, 66 as libc::c_int],
     [129 as libc::c_int, 117 as libc::c_int],
     [129 as libc::c_int, 118 as libc::c_int],
     [129 as libc::c_int, 65 as libc::c_int],
     [129 as libc::c_int, 69 as libc::c_int],
     [131 as libc::c_int, 146 as libc::c_int],
     [131 as libc::c_int, 64 as libc::c_int],
     [131 as libc::c_int, 66 as libc::c_int],
     [131 as libc::c_int, 68 as libc::c_int],
     [131 as libc::c_int, 70 as libc::c_int],
     [131 as libc::c_int, 72 as libc::c_int],
     [131 as libc::c_int, 131 as libc::c_int],
     [131 as libc::c_int, 133 as libc::c_int],
     [131 as libc::c_int, 135 as libc::c_int],
     [131 as libc::c_int, 98 as libc::c_int],
     [129 as libc::c_int, 91 as libc::c_int],
     [131 as libc::c_int, 65 as libc::c_int],
     [131 as libc::c_int, 67 as libc::c_int],
     [131 as libc::c_int, 69 as libc::c_int],
     [131 as libc::c_int, 71 as libc::c_int],
     [131 as libc::c_int, 73 as libc::c_int],
     [131 as libc::c_int, 74 as libc::c_int],
     [131 as libc::c_int, 76 as libc::c_int],
     [131 as libc::c_int, 78 as libc::c_int],
     [131 as libc::c_int, 80 as libc::c_int],
     [131 as libc::c_int, 82 as libc::c_int],
     [131 as libc::c_int, 84 as libc::c_int],
     [131 as libc::c_int, 86 as libc::c_int],
     [131 as libc::c_int, 88 as libc::c_int],
     [131 as libc::c_int, 90 as libc::c_int],
     [131 as libc::c_int, 92 as libc::c_int],
     [131 as libc::c_int, 94 as libc::c_int],
     [131 as libc::c_int, 96 as libc::c_int],
     [131 as libc::c_int, 99 as libc::c_int],
     [131 as libc::c_int, 101 as libc::c_int],
     [131 as libc::c_int, 103 as libc::c_int],
     [131 as libc::c_int, 105 as libc::c_int],
     [131 as libc::c_int, 106 as libc::c_int],
     [131 as libc::c_int, 107 as libc::c_int],
     [131 as libc::c_int, 108 as libc::c_int],
     [131 as libc::c_int, 109 as libc::c_int],
     [131 as libc::c_int, 110 as libc::c_int],
     [131 as libc::c_int, 113 as libc::c_int],
     [131 as libc::c_int, 116 as libc::c_int],
     [131 as libc::c_int, 119 as libc::c_int],
     [131 as libc::c_int, 122 as libc::c_int],
     [131 as libc::c_int, 125 as libc::c_int],
     [131 as libc::c_int, 126 as libc::c_int],
     [131 as libc::c_int, 128 as libc::c_int],
     [131 as libc::c_int, 129 as libc::c_int],
     [131 as libc::c_int, 130 as libc::c_int],
     [131 as libc::c_int, 132 as libc::c_int],
     [131 as libc::c_int, 134 as libc::c_int],
     [131 as libc::c_int, 136 as libc::c_int],
     [131 as libc::c_int, 137 as libc::c_int],
     [131 as libc::c_int, 138 as libc::c_int],
     [131 as libc::c_int, 139 as libc::c_int],
     [131 as libc::c_int, 140 as libc::c_int],
     [131 as libc::c_int, 141 as libc::c_int],
     [131 as libc::c_int, 143 as libc::c_int],
     [131 as libc::c_int, 147 as libc::c_int],
     [129 as libc::c_int, 74 as libc::c_int],
     [129 as libc::c_int, 75 as libc::c_int]];
#[no_mangle]
pub unsafe extern "C" fn _jis_shift(mut p1: *mut libc::c_int,
                                    mut p2: *mut libc::c_int) {
    let mut c1: libc::c_uchar = *p1 as libc::c_uchar;
    let mut c2: libc::c_uchar = *p2 as libc::c_uchar;
    let mut rowOffset: libc::c_int =
        if (c1 as libc::c_int) < 95 as libc::c_int {
            112 as libc::c_int
        } else { 176 as libc::c_int };
    let mut cellOffset: libc::c_int =
        if c1 as libc::c_int % 2 as libc::c_int != 0 {
            if c2 as libc::c_int > 95 as libc::c_int {
                32 as libc::c_int
            } else { 31 as libc::c_int }
        } else { 126 as libc::c_int };
    *p1 =
        (c1 as libc::c_int + 1 as libc::c_int >> 1 as libc::c_int) +
            rowOffset;
    *p2 += cellOffset;
}
#[no_mangle]
pub unsafe extern "C" fn _sjis_shift(mut p1: *mut libc::c_int,
                                     mut p2: *mut libc::c_int) {
    let mut c1: libc::c_uchar = *p1 as libc::c_uchar;
    let mut c2: libc::c_uchar = *p2 as libc::c_uchar;
    let mut adjust: libc::c_int =
        ((c2 as libc::c_int) < 159 as libc::c_int) as libc::c_int;
    let mut rowOffset: libc::c_int =
        if (c1 as libc::c_int) < 160 as libc::c_int {
            112 as libc::c_int
        } else { 176 as libc::c_int };
    let mut cellOffset: libc::c_int =
        if adjust != 0 {
            if c2 as libc::c_int > 127 as libc::c_int {
                32 as libc::c_int
            } else { 31 as libc::c_int }
        } else { 126 as libc::c_int };
    *p1 = (c1 as libc::c_int - rowOffset << 1 as libc::c_int) - adjust;
    *p2 -= cellOffset;
}
#[no_mangle]
pub unsafe extern "C" fn _sjis_han2zen(mut str: *mut libc::c_uchar,
                                       mut p1: *mut libc::c_int,
                                       mut p2: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c1 = *str as libc::c_int;
    str = str.offset(1);
    *p1 =
        stable[(c1 - 161 as libc::c_int) as usize][0 as libc::c_int as usize];
    *p2 =
        stable[(c1 - 161 as libc::c_int) as usize][1 as libc::c_int as usize];
    c2 = *str as libc::c_int;
    if c2 == 222 as libc::c_int &&
           (c1 >= 182 as libc::c_int && c1 <= 196 as libc::c_int ||
                c1 >= 202 as libc::c_int && c1 <= 206 as libc::c_int ||
                c1 == 179 as libc::c_int) {
        if *p2 >= 74 as libc::c_int && *p2 <= 103 as libc::c_int ||
               *p2 >= 110 as libc::c_int && *p2 <= 122 as libc::c_int {
            *p2 += 1
        } else if *p1 == 131 as libc::c_int && *p2 == 69 as libc::c_int {
            *p2 = 148 as libc::c_int
        }
        str = str.offset(1)
    }
    if c2 == 223 as libc::c_int &&
           (c1 >= 202 as libc::c_int && c1 <= 206 as libc::c_int) &&
           (*p2 >= 110 as libc::c_int && *p2 <= 122 as libc::c_int) {
        *p2 += 2 as libc::c_int;
        str = str.offset(1)
    }
    let fresh0 = str;
    str = str.offset(1);
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn _shift2euc(mut str: *mut libc::c_uchar,
                                    mut str2: *mut libc::c_uchar) {
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    loop  {
        p1 = *str as libc::c_int;
        if !(p1 != '\u{0}' as i32) { break ; }
        if p1 >= 129 as libc::c_int && p1 <= 159 as libc::c_int ||
               p1 >= 224 as libc::c_int && p1 <= 239 as libc::c_int {
            str = str.offset(1);
            p2 = *str as libc::c_int;
            if p2 == '\u{0}' as i32 { break ; }
            if p2 >= 64 as libc::c_int && p2 <= 252 as libc::c_int {
                _sjis_shift(&mut p1, &mut p2);
                p1 += 128 as libc::c_int;
                p2 += 128 as libc::c_int
            }
            *str2 = p1 as libc::c_uchar;
            str2 = str2.offset(1);
            *str2 = p2 as libc::c_uchar;
            str2 = str2.offset(1);
            str = str.offset(1)
        } else if p1 >= 161 as libc::c_int && p1 <= 223 as libc::c_int {
            str = _sjis_han2zen(str, &mut p1, &mut p2);
            _sjis_shift(&mut p1, &mut p2);
            p1 += 128 as libc::c_int;
            p2 += 128 as libc::c_int;
            *str2 = p1 as libc::c_uchar;
            str2 = str2.offset(1);
            *str2 = p2 as libc::c_uchar;
            str2 = str2.offset(1)
        } else {
            *str2 = p1 as libc::c_uchar;
            str2 = str2.offset(1);
            str = str.offset(1)
        }
    }
    *str2 = '\u{0}' as i32 as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn _euc2shift(mut str: *mut libc::c_uchar,
                                    mut str2: *mut libc::c_uchar) {
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    loop  {
        p1 = *str as libc::c_int;
        if !(p1 != '\u{0}' as i32) { break ; }
        if p1 >= 161 as libc::c_int && p1 <= 254 as libc::c_int {
            str = str.offset(1);
            p2 = *str as libc::c_int;
            if p2 == '\u{0}' as i32 { break ; }
            if p2 >= 161 as libc::c_int && p2 <= 254 as libc::c_int {
                p1 -= 128 as libc::c_int;
                p2 -= 128 as libc::c_int;
                _jis_shift(&mut p1, &mut p2);
            }
            *str2 = p1 as libc::c_uchar;
            str2 = str2.offset(1);
            *str2 = p2 as libc::c_uchar;
            str2 = str2.offset(1);
            str = str.offset(1)
        } else {
            *str2 = p1 as libc::c_uchar;
            str2 = str2.offset(1);
            str = str.offset(1)
        }
    }
    *str2 = '\u{0}' as i32 as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn _set_buffer(mut str: *mut libc::c_char)
 -> *mut libc::c_uchar {
    static mut buf: *mut libc::c_uchar =
        0 as *const libc::c_uchar as *mut libc::c_uchar;
    buf =
        malloc(strlen(str).wrapping_add(1 as libc::c_int as
                                            libc::c_ulong).wrapping_mul(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong))
            as *mut libc::c_uchar;
    if buf.is_null() {
        fprintf(stderr,
                b"Can\'t malloc buffer\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(2 as libc::c_int);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn _replace_buffer(mut buf: *mut libc::c_uchar)
 -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = strdup(buf as *const libc::c_char);
    if str.is_null() {
        fprintf(stderr,
                b"Can\'t malloc string buffer\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(2 as libc::c_int);
    }
    free(buf as *mut libc::c_void);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn toStringEUC(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    buf = _set_buffer(str);
    _shift2euc(str as *mut libc::c_uchar, buf);
    return _replace_buffer(buf);
}
#[no_mangle]
pub unsafe extern "C" fn toStringSJIS(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    buf = _set_buffer(str);
    _euc2shift(str as *mut libc::c_uchar, buf);
    return _replace_buffer(buf);
}
