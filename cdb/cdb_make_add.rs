#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn cdb_hash(buf: *const libc::c_void, len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn cdb_pack(num: libc::c_uint, buf: *mut libc::c_uchar);
    #[no_mangle]
    fn _cdb_make_write(cdbmp: *mut cdb_make, ptr: *const libc::c_uchar,
                       len: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb_make {
    pub cdb_fd: libc::c_int,
    pub cdb_dpos: libc::c_uint,
    pub cdb_rcnt: libc::c_uint,
    pub cdb_buf: [libc::c_uchar; 4096],
    pub cdb_bpos: *mut libc::c_uchar,
    pub cdb_rec: [*mut cdb_rl; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb_rl {
    pub next: *mut cdb_rl,
    pub cnt: libc::c_uint,
    pub rec: [cdb_rec; 254],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb_rec {
    pub hval: libc::c_uint,
    pub rpos: libc::c_uint,
}
/* cdb_make_add.c: basic cdb_make_add routine
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
/* for malloc */
#[no_mangle]
pub unsafe extern "C" fn _cdb_make_add(mut cdbmp: *mut cdb_make,
                                       mut hval: libc::c_uint,
                                       mut key: *const libc::c_void,
                                       mut klen: libc::c_uint,
                                       mut val: *const libc::c_void,
                                       mut vlen: libc::c_uint)
 -> libc::c_int {
    let mut rlen: [libc::c_uchar; 8] = [0; 8];
    let mut rl: *mut cdb_rl = 0 as *mut cdb_rl;
    let mut i: libc::c_uint = 0;
    if klen >
           (0xffffffff as
                libc::c_uint).wrapping_sub((*cdbmp).cdb_dpos.wrapping_add(8 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint))
           ||
           vlen >
               (0xffffffff as
                    libc::c_uint).wrapping_sub((*cdbmp).cdb_dpos.wrapping_add(klen).wrapping_add(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint))
       {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int)
    }
    i = hval & 255 as libc::c_int as libc::c_uint;
    rl = (*cdbmp).cdb_rec[i as usize];
    if rl.is_null() ||
           (*rl).cnt as libc::c_ulong >=
               (::std::mem::size_of::<[cdb_rec; 254]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<cdb_rec>()
                                                    as libc::c_ulong) {
        rl =
            malloc(::std::mem::size_of::<cdb_rl>() as libc::c_ulong) as
                *mut cdb_rl;
        if rl.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int)
        }
        (*rl).cnt = 0 as libc::c_int as libc::c_uint;
        (*rl).next = (*cdbmp).cdb_rec[i as usize];
        (*cdbmp).cdb_rec[i as usize] = rl
    }
    let fresh0 = (*rl).cnt;
    (*rl).cnt = (*rl).cnt.wrapping_add(1);
    i = fresh0;
    (*rl).rec[i as usize].hval = hval;
    (*rl).rec[i as usize].rpos = (*cdbmp).cdb_dpos;
    (*cdbmp).cdb_rcnt = (*cdbmp).cdb_rcnt.wrapping_add(1);
    cdb_pack(klen, rlen.as_mut_ptr());
    cdb_pack(vlen, rlen.as_mut_ptr().offset(4 as libc::c_int as isize));
    if _cdb_make_write(cdbmp, rlen.as_mut_ptr(),
                       8 as libc::c_int as libc::c_uint) < 0 as libc::c_int ||
           _cdb_make_write(cdbmp, key as *const libc::c_uchar, klen) <
               0 as libc::c_int ||
           _cdb_make_write(cdbmp, val as *const libc::c_uchar, vlen) <
               0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_add(mut cdbmp: *mut cdb_make,
                                      mut key: *const libc::c_void,
                                      mut klen: libc::c_uint,
                                      mut val: *const libc::c_void,
                                      mut vlen: libc::c_uint) -> libc::c_int {
    return _cdb_make_add(cdbmp, cdb_hash(key, klen), key, klen, val, vlen);
}
