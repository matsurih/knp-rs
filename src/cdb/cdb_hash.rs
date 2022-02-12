#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

/* cdb_hash.c: cdb hashing routine
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_hash(mut buf: *const libc::c_void,
                                  mut len: libc::c_uint) -> libc::c_uint {
    let mut p: *const libc::c_uchar =
        buf as *const libc::c_uchar; /* start value */
    let mut end: *const libc::c_uchar = p.offset(len as isize);
    let mut hash: libc::c_uint = 5381 as libc::c_int as libc::c_uint;
    while p < end {
        let fresh0 = p;
        p = p.offset(1);
        hash =
            hash.wrapping_add(hash << 5 as libc::c_int) ^
                *fresh0 as libc::c_uint
    }
    return hash;
}
