#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

/* cdb_unpack.c: unpack 32bit integer
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_unpack(mut buf: *const libc::c_uchar)
 -> libc::c_uint {
    let mut n: libc::c_uint =
        *buf.offset(3 as libc::c_int as isize) as libc::c_uint;
    n <<= 8 as libc::c_int;
    n |= *buf.offset(2 as libc::c_int as isize) as libc::c_uint;
    n <<= 8 as libc::c_int;
    n |= *buf.offset(1 as libc::c_int as isize) as libc::c_uint;
    n <<= 8 as libc::c_int;
    n |= *buf.offset(0 as libc::c_int as isize) as libc::c_uint;
    return n;
}
