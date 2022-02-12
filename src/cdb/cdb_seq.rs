#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
/* cdb_seq.c: sequential record retrieval routines
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_seqnext(mut cptr: *mut libc::c_uint,
                                     mut cdbp: *mut cdb) -> libc::c_int {
    let mut klen: libc::c_uint = 0;
    let mut vlen: libc::c_uint = 0;
    let mut pos: libc::c_uint = *cptr;
    let mut dend: libc::c_uint = (*cdbp).cdb_dend;
    let mut mem: *const libc::c_uchar = (*cdbp).cdb_mem;
    if pos > dend.wrapping_sub(8 as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    klen = cdb_unpack(mem.offset(pos as isize));
    vlen =
        cdb_unpack(mem.offset(pos as
                                  isize).offset(4 as libc::c_int as isize));
    pos = pos.wrapping_add(8 as libc::c_int as libc::c_uint);
    if dend.wrapping_sub(klen) < pos ||
           dend.wrapping_sub(vlen) < pos.wrapping_add(klen) {
        *__errno_location() = 71 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*cdbp).cdb_kpos = pos;
    (*cdbp).cdb_klen = klen;
    (*cdbp).cdb_vpos = pos.wrapping_add(klen);
    (*cdbp).cdb_vlen = vlen;
    *cptr = pos.wrapping_add(klen).wrapping_add(vlen);
    return 1 as libc::c_int;
}
