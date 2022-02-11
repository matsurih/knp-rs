#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int)
     -> __off64_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
/* cdb_make.c: basic cdb creation routines
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_pack(mut num: libc::c_uint,
                                  mut buf: *mut libc::c_uchar) {
    *buf.offset(0 as libc::c_int as isize) =
        (num & 255 as libc::c_int as libc::c_uint) as
            libc::c_uchar; /* hash table counts */
    num >>= 8 as libc::c_int; /* hash table positions */
    *buf.offset(1 as libc::c_int as isize) =
        (num & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
    num >>= 8 as libc::c_int;
    *buf.offset(2 as libc::c_int as isize) =
        (num & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
    *buf.offset(3 as libc::c_int as isize) =
        (num >> 8 as libc::c_int) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_start(mut cdbmp: *mut cdb_make,
                                        mut fd: libc::c_int) -> libc::c_int {
    memset(cdbmp as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<cdb_make>() as libc::c_ulong);
    (*cdbmp).cdb_fd = fd;
    (*cdbmp).cdb_dpos = 2048 as libc::c_int as libc::c_uint;
    (*cdbmp).cdb_bpos =
        (*cdbmp).cdb_buf.as_mut_ptr().offset(2048 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cdb_make_fullwrite(mut fd: libc::c_int,
                                             mut buf: *const libc::c_uchar,
                                             mut len: libc::c_uint)
 -> libc::c_int {
    while len != 0 {
        let mut l: libc::c_int =
            write(fd, buf as *const libc::c_void, len as size_t) as
                libc::c_int;
        if l > 0 as libc::c_int {
            len = len.wrapping_sub(l as libc::c_uint);
            buf = buf.offset(l as isize)
        } else if l < 0 as libc::c_int &&
                      *__errno_location() != 4 as libc::c_int {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cdb_make_flush(mut cdbmp: *mut cdb_make)
 -> libc::c_int {
    let mut len: libc::c_uint =
        (*cdbmp).cdb_bpos.wrapping_offset_from((*cdbmp).cdb_buf.as_mut_ptr())
            as libc::c_long as libc::c_uint;
    if len != 0 {
        if _cdb_make_fullwrite((*cdbmp).cdb_fd, (*cdbmp).cdb_buf.as_mut_ptr(),
                               len) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        (*cdbmp).cdb_bpos = (*cdbmp).cdb_buf.as_mut_ptr()
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cdb_make_write(mut cdbmp: *mut cdb_make,
                                         mut ptr: *const libc::c_uchar,
                                         mut len: libc::c_uint)
 -> libc::c_int {
    let mut l: libc::c_uint =
        (::std::mem::size_of::<[libc::c_uchar; 4096]>() as
             libc::c_ulong).wrapping_sub((*cdbmp).cdb_bpos.wrapping_offset_from((*cdbmp).cdb_buf.as_mut_ptr())
                                             as libc::c_long as libc::c_ulong)
            as libc::c_uint;
    (*cdbmp).cdb_dpos = (*cdbmp).cdb_dpos.wrapping_add(len);
    if len > l {
        memcpy((*cdbmp).cdb_bpos as *mut libc::c_void,
               ptr as *const libc::c_void, l as libc::c_ulong);
        (*cdbmp).cdb_bpos = (*cdbmp).cdb_bpos.offset(l as isize);
        if _cdb_make_flush(cdbmp) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        ptr = ptr.offset(l as isize);
        len = len.wrapping_sub(l);
        l =
            (len as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                                 as libc::c_ulong) as
                libc::c_uint;
        if l != 0 {
            l =
                (l as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                                     as libc::c_ulong) as
                    libc::c_uint as libc::c_uint;
            if _cdb_make_fullwrite((*cdbmp).cdb_fd, ptr, l) < 0 as libc::c_int
               {
                return -(1 as libc::c_int)
            }
            ptr = ptr.offset(l as isize);
            len = len.wrapping_sub(l)
        }
    }
    if len != 0 {
        memcpy((*cdbmp).cdb_bpos as *mut libc::c_void,
               ptr as *const libc::c_void, len as libc::c_ulong);
        (*cdbmp).cdb_bpos = (*cdbmp).cdb_bpos.offset(len as isize)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cdb_make_finish_internal(mut cdbmp: *mut cdb_make)
 -> libc::c_int {
    let mut hcnt: [libc::c_uint; 256] = [0; 256];
    let mut hpos: [libc::c_uint; 256] = [0; 256];
    let mut htab: *mut cdb_rec = 0 as *mut cdb_rec;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rl: *mut cdb_rl = 0 as *mut cdb_rl;
    let mut hsize: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if ((0xffffffff as libc::c_uint).wrapping_sub((*cdbmp).cdb_dpos) >>
            3 as libc::c_int) < (*cdbmp).cdb_rcnt {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int)
    }
    /* count htab sizes and reorder reclists */
    hsize = 0 as libc::c_int as libc::c_uint;
    t = 0 as libc::c_int as libc::c_uint;
    while t < 256 as libc::c_int as libc::c_uint {
        let mut rlt: *mut cdb_rl = 0 as *mut cdb_rl;
        i = 0 as libc::c_int as libc::c_uint;
        rl = (*cdbmp).cdb_rec[t as usize];
        while !rl.is_null() {
            let mut rln: *mut cdb_rl = (*rl).next;
            (*rl).next = rlt;
            rlt = rl;
            i = i.wrapping_add((*rl).cnt);
            rl = rln
        }
        (*cdbmp).cdb_rec[t as usize] = rlt;
        hcnt[t as usize] = i << 1 as libc::c_int;
        if hsize < hcnt[t as usize] { hsize = hcnt[t as usize] }
        t = t.wrapping_add(1)
    }
    /* allocate memory to hold max htable */
    htab =
        malloc((hsize.wrapping_add(2 as libc::c_int as libc::c_uint) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cdb_rec>()
                                                    as libc::c_ulong)) as
            *mut cdb_rec;
    if htab.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    p = htab as *mut libc::c_uchar;
    htab = htab.offset(2 as libc::c_int as isize);
    /* build hash tables */
    t = 0 as libc::c_int as libc::c_uint;
    while t < 256 as libc::c_int as libc::c_uint {
        let mut len: libc::c_uint = 0;
        let mut hi: libc::c_uint = 0;
        hpos[t as usize] = (*cdbmp).cdb_dpos;
        len = hcnt[t as usize];
        if !(len == 0 as libc::c_int as libc::c_uint) {
            i = 0 as libc::c_int as libc::c_uint;
            while i < len {
                let ref mut fresh0 = (*htab.offset(i as isize)).rpos;
                *fresh0 = 0 as libc::c_int as libc::c_uint;
                (*htab.offset(i as isize)).hval = *fresh0;
                i = i.wrapping_add(1)
            }
            rl = (*cdbmp).cdb_rec[t as usize];
            while !rl.is_null() {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*rl).cnt {
                    hi =
                        ((*rl).rec[i as usize].hval >>
                             8 as libc::c_int).wrapping_rem(len);
                    while (*htab.offset(hi as isize)).rpos != 0 {
                        hi = hi.wrapping_add(1);
                        if hi == len { hi = 0 as libc::c_int as libc::c_uint }
                    }
                    *htab.offset(hi as isize) = (*rl).rec[i as usize];
                    i = i.wrapping_add(1)
                }
                rl = (*rl).next
            }
            i = 0 as libc::c_int as libc::c_uint;
            while i < len {
                cdb_pack((*htab.offset(i as isize)).hval,
                         p.offset((i << 3 as libc::c_int) as isize));
                cdb_pack((*htab.offset(i as isize)).rpos,
                         p.offset((i << 3 as libc::c_int) as
                                      isize).offset(4 as libc::c_int as
                                                        isize));
                i = i.wrapping_add(1)
            }
            if _cdb_make_write(cdbmp, p, len << 3 as libc::c_int) <
                   0 as libc::c_int {
                free(p as *mut libc::c_void);
                return -(1 as libc::c_int)
            }
        }
        t = t.wrapping_add(1)
    }
    free(p as *mut libc::c_void);
    if _cdb_make_flush(cdbmp) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    p = (*cdbmp).cdb_buf.as_mut_ptr();
    t = 0 as libc::c_int as libc::c_uint;
    while t < 256 as libc::c_int as libc::c_uint {
        cdb_pack(hpos[t as usize],
                 p.offset((t << 3 as libc::c_int) as isize));
        cdb_pack(hcnt[t as usize],
                 p.offset((t << 3 as libc::c_int) as
                              isize).offset(4 as libc::c_int as isize));
        t = t.wrapping_add(1)
    }
    if lseek((*cdbmp).cdb_fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int)
           != 0 as libc::c_int as libc::c_long ||
           _cdb_make_fullwrite((*cdbmp).cdb_fd, p,
                               2048 as libc::c_int as libc::c_uint) !=
               0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cdb_make_free(mut cdbmp: *mut cdb_make) {
    let mut t: libc::c_uint = 0;
    t = 0 as libc::c_int as libc::c_uint;
    while t < 256 as libc::c_int as libc::c_uint {
        let mut rl: *mut cdb_rl = (*cdbmp).cdb_rec[t as usize];
        while !rl.is_null() {
            let mut tm: *mut cdb_rl = rl;
            rl = (*rl).next;
            free(tm as *mut libc::c_void);
        }
        t = t.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_finish(mut cdbmp: *mut cdb_make)
 -> libc::c_int {
    let mut r: libc::c_int = cdb_make_finish_internal(cdbmp);
    cdb_make_free(cdbmp);
    return r;
}
