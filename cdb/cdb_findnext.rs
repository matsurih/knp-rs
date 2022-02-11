#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn cdb_hash(buf: *const libc::c_void, len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb {
    pub cdb_fd: libc::c_int,
    pub cdb_fsize: libc::c_uint,
    pub cdb_dend: libc::c_uint,
    pub cdb_mem: *const libc::c_uchar,
    pub cdb_vpos: libc::c_uint,
    pub cdb_vlen: libc::c_uint,
    pub cdb_kpos: libc::c_uint,
    pub cdb_klen: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb_find {
    pub cdb_cdbp: *mut cdb,
    pub cdb_hval: libc::c_uint,
    pub cdb_htp: *const libc::c_uchar,
    pub cdb_htab: *const libc::c_uchar,
    pub cdb_htend: *const libc::c_uchar,
    pub cdb_httodo: libc::c_uint,
    pub cdb_key: *const libc::c_void,
    pub cdb_klen: libc::c_uint,
}
/* cdb_findnext.c: sequential cdb_find routines
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
/* see cdb_find.c for comments */
#[no_mangle]
pub unsafe extern "C" fn cdb_findinit(mut cdbfp: *mut cdb_find,
                                      mut cdbp: *mut cdb,
                                      mut key: *const libc::c_void,
                                      mut klen: libc::c_uint) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut pos: libc::c_uint = 0;
    (*cdbfp).cdb_cdbp = cdbp;
    (*cdbfp).cdb_key = key;
    (*cdbfp).cdb_klen = klen;
    (*cdbfp).cdb_hval = cdb_hash(key, klen);
    (*cdbfp).cdb_htp =
        (*cdbp).cdb_mem.offset(((*cdbfp).cdb_hval << 3 as libc::c_int &
                                    2047 as libc::c_int as libc::c_uint) as
                                   isize);
    n = cdb_unpack((*cdbfp).cdb_htp.offset(4 as libc::c_int as isize));
    (*cdbfp).cdb_httodo = n << 3 as libc::c_int;
    if n == 0 { return 0 as libc::c_int }
    pos = cdb_unpack((*cdbfp).cdb_htp);
    if n > (*cdbp).cdb_fsize >> 3 as libc::c_int || pos < (*cdbp).cdb_dend ||
           pos > (*cdbp).cdb_fsize ||
           (*cdbfp).cdb_httodo > (*cdbp).cdb_fsize.wrapping_sub(pos) {
        *__errno_location() = 71 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*cdbfp).cdb_htab = (*cdbp).cdb_mem.offset(pos as isize);
    (*cdbfp).cdb_htend =
        (*cdbfp).cdb_htab.offset((*cdbfp).cdb_httodo as isize);
    (*cdbfp).cdb_htp =
        (*cdbfp).cdb_htab.offset((((*cdbfp).cdb_hval >>
                                       8 as libc::c_int).wrapping_rem(n) <<
                                      3 as libc::c_int) as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_findnext(mut cdbfp: *mut cdb_find)
 -> libc::c_int {
    let mut cdbp: *mut cdb = (*cdbfp).cdb_cdbp;
    let mut pos: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut klen: libc::c_uint = (*cdbfp).cdb_klen;
    while (*cdbfp).cdb_httodo != 0 {
        pos = cdb_unpack((*cdbfp).cdb_htp.offset(4 as libc::c_int as isize));
        if pos == 0 { return 0 as libc::c_int }
        n =
            (cdb_unpack((*cdbfp).cdb_htp) == (*cdbfp).cdb_hval) as libc::c_int
                as libc::c_uint;
        (*cdbfp).cdb_htp = (*cdbfp).cdb_htp.offset(8 as libc::c_int as isize);
        if (*cdbfp).cdb_htp >= (*cdbfp).cdb_htend {
            (*cdbfp).cdb_htp = (*cdbfp).cdb_htab
        }
        (*cdbfp).cdb_httodo =
            (*cdbfp).cdb_httodo.wrapping_sub(8 as libc::c_int as
                                                 libc::c_uint);
        if n != 0 {
            if pos >
                   (*cdbp).cdb_fsize.wrapping_sub(8 as libc::c_int as
                                                      libc::c_uint) {
                *__errno_location() = 71 as libc::c_int;
                return -(1 as libc::c_int)
            }
            if cdb_unpack((*cdbp).cdb_mem.offset(pos as isize)) == klen {
                if (*cdbp).cdb_fsize.wrapping_sub(klen) <
                       pos.wrapping_add(8 as libc::c_int as libc::c_uint) {
                    *__errno_location() = 71 as libc::c_int;
                    return -(1 as libc::c_int)
                }
                if memcmp((*cdbfp).cdb_key,
                          (*cdbp).cdb_mem.offset(pos as
                                                     isize).offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                              as *const libc::c_void, klen as libc::c_ulong)
                       == 0 as libc::c_int {
                    n =
                        cdb_unpack((*cdbp).cdb_mem.offset(pos as
                                                              isize).offset(4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                    pos = pos.wrapping_add(8 as libc::c_int as libc::c_uint);
                    if (*cdbp).cdb_fsize < n ||
                           (*cdbp).cdb_fsize.wrapping_sub(n) <
                               pos.wrapping_add(klen) {
                        *__errno_location() = 71 as libc::c_int;
                        return -(1 as libc::c_int)
                    }
                    (*cdbp).cdb_kpos = pos;
                    (*cdbp).cdb_klen = klen;
                    (*cdbp).cdb_vpos = pos.wrapping_add(klen);
                    (*cdbp).cdb_vlen = n;
                    return 1 as libc::c_int
                }
            }
        }
    }
    return 0 as libc::c_int;
}
