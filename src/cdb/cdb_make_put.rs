#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int)
     -> __off64_t;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn _cdb_make_fullwrite(fd: libc::c_int, buf: *const libc::c_uchar,
                           len: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn _cdb_make_flush(cdbmp: *mut cdb_make) -> libc::c_int;
    #[no_mangle]
    fn _cdb_make_add(cdbmp: *mut cdb_make, hval: libc::c_uint,
                     key: *const libc::c_void, klen: libc::c_uint,
                     val: *const libc::c_void, vlen: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn cdb_pack(num: libc::c_uint, buf: *mut libc::c_uchar);
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
    #[no_mangle]
    fn cdb_hash(buf: *const libc::c_void, len: libc::c_uint) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type cdb_put_mode = libc::c_uint;
pub const CDB_FIND_FILL0: cdb_put_mode = 4;
pub const CDB_PUT_REPLACE0: cdb_put_mode = 4;
pub const CDB_PUT_WARN: cdb_put_mode = 3;
pub const CDB_PUT_INSERT: cdb_put_mode = 2;
pub const CDB_FIND_REMOVE: cdb_put_mode = 1;
pub const CDB_PUT_REPLACE: cdb_put_mode = 1;
pub const CDB_FIND: cdb_put_mode = 0;
pub const CDB_PUT_ADD: cdb_put_mode = 0;
/* cdb_make_put.c: "advanced" cdb_make_put routine
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
unsafe extern "C" fn fixup_rpos(mut cdbmp: *mut cdb_make,
                                mut rpos: libc::c_uint,
                                mut rlen: libc::c_uint) {
    let mut i: libc::c_uint = 0; /* it was the last record, nothing to do */
    let mut rl: *mut cdb_rl = 0 as *mut cdb_rl;
    let mut rp: *mut cdb_rec = 0 as *mut cdb_rec;
    let mut rs: *mut cdb_rec = 0 as *mut cdb_rec;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        rl = (*cdbmp).cdb_rec[i as usize];
        's_22:
            while !rl.is_null() {
                rs = (*rl).rec.as_mut_ptr();
                rp = rs.offset((*rl).cnt as isize);
                loop  {
                    rp = rp.offset(-1);
                    if !(rp >= rs) { break ; }
                    if (*rp).rpos <= rpos { break 's_22 ; }
                    (*rp).rpos = (*rp).rpos.wrapping_sub(rlen)
                }
                rl = (*rl).next
            }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn remove_record(mut cdbmp: *mut cdb_make,
                                   mut rpos: libc::c_uint,
                                   mut rlen: libc::c_uint) -> libc::c_int {
    let mut pos: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut r: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    len = (*cdbmp).cdb_dpos.wrapping_sub(rpos).wrapping_sub(rlen);
    (*cdbmp).cdb_dpos = (*cdbmp).cdb_dpos.wrapping_sub(rlen);
    if len == 0 { return 0 as libc::c_int }
    pos = rpos;
    fd = (*cdbmp).cdb_fd;
    loop  {
        r =
            if len as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                       libc::c_ulong {
                ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                    libc::c_ulong
            } else { len as libc::c_ulong } as libc::c_int;
        if lseek(fd, pos.wrapping_add(rlen) as __off64_t, 0 as libc::c_int) <
               0 as libc::c_int as libc::c_long ||
               {
                   r =
                       read(fd,
                            (*cdbmp).cdb_buf.as_mut_ptr() as
                                *mut libc::c_void, r as size_t) as
                           libc::c_int;
                   (r) <= 0 as libc::c_int
               } {
            return -(1 as libc::c_int)
        }
        if lseek(fd, pos as __off64_t, 0 as libc::c_int) <
               0 as libc::c_int as libc::c_long ||
               _cdb_make_fullwrite(fd, (*cdbmp).cdb_buf.as_mut_ptr(),
                                   r as libc::c_uint) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        pos = pos.wrapping_add(r as libc::c_uint);
        len = len.wrapping_sub(r as libc::c_uint);
        if !(len != 0) { break ; }
    }
    if (*cdbmp).cdb_dpos == pos {
    } else {
        __assert_fail(b"cdbmp->cdb_dpos == pos\x00" as *const u8 as
                          *const libc::c_char,
                      b"cdb_make_put.c\x00" as *const u8 as
                          *const libc::c_char,
                      48 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"int remove_record(struct cdb_make *, unsigned int, unsigned int)\x00")).as_ptr());
    }
    fixup_rpos(cdbmp, rpos, rlen);
    return 0 as libc::c_int;
}
unsafe extern "C" fn zerofill_record(mut cdbmp: *mut cdb_make,
                                     mut rpos: libc::c_uint,
                                     mut rlen: libc::c_uint) -> libc::c_int {
    if rpos.wrapping_add(rlen) == (*cdbmp).cdb_dpos {
        (*cdbmp).cdb_dpos = rpos;
        return 0 as libc::c_int
    }
    if lseek((*cdbmp).cdb_fd, rpos as __off64_t, 0 as libc::c_int) <
           0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int)
    }
    memset((*cdbmp).cdb_buf.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong);
    cdb_pack(rlen.wrapping_sub(8 as libc::c_int as libc::c_uint),
             (*cdbmp).cdb_buf.as_mut_ptr().offset(4 as libc::c_int as isize));
    loop  {
        rpos =
            if rlen as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                       libc::c_ulong {
                ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                    libc::c_ulong
            } else { rlen as libc::c_ulong } as libc::c_uint;
        if _cdb_make_fullwrite((*cdbmp).cdb_fd, (*cdbmp).cdb_buf.as_mut_ptr(),
                               rpos) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        rlen = rlen.wrapping_sub(rpos);
        if rlen == 0 { return 0 as libc::c_int }
        memset((*cdbmp).cdb_buf.as_mut_ptr().offset(4 as libc::c_int as isize)
                   as *mut libc::c_void, 0 as libc::c_int,
               4 as libc::c_int as libc::c_ulong);
    };
}
/* return: 0 = not found, 1 = error, or record length */
unsafe extern "C" fn match_0(mut cdbmp: *mut cdb_make, mut pos: libc::c_uint,
                             mut key: *const libc::c_char,
                             mut klen: libc::c_uint) -> libc::c_uint {
    let mut len: libc::c_int = 0;
    let mut rlen: libc::c_uint = 0;
    if lseek((*cdbmp).cdb_fd, pos as __off64_t, 0 as libc::c_int) <
           0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int as libc::c_uint
    }
    if read((*cdbmp).cdb_fd,
            (*cdbmp).cdb_buf.as_mut_ptr() as *mut libc::c_void,
            8 as libc::c_int as size_t) != 8 as libc::c_int as libc::c_long {
        return 1 as libc::c_int as libc::c_uint
    }
    if cdb_unpack((*cdbmp).cdb_buf.as_mut_ptr() as *const libc::c_uchar) !=
           klen {
        return 0 as libc::c_int as libc::c_uint
    }
    /* record length; check its validity */
    rlen =
        cdb_unpack((*cdbmp).cdb_buf.as_mut_ptr().offset(4 as libc::c_int as
                                                            isize) as
                       *const libc::c_uchar); /* someone changed our file? */
    if rlen >
           (*cdbmp).cdb_dpos.wrapping_sub(pos).wrapping_sub(klen).wrapping_sub(8
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
       {
        *__errno_location() = 71 as libc::c_int;
        return 1 as libc::c_int as libc::c_uint
    }
    rlen =
        rlen.wrapping_add(klen.wrapping_add(8 as libc::c_int as
                                                libc::c_uint));
    while klen != 0 {
        len =
            if klen as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                       libc::c_ulong {
                ::std::mem::size_of::<[libc::c_uchar; 4096]>() as
                    libc::c_ulong
            } else { klen as libc::c_ulong } as libc::c_int;
        len =
            read((*cdbmp).cdb_fd,
                 (*cdbmp).cdb_buf.as_mut_ptr() as *mut libc::c_void,
                 len as size_t) as libc::c_int;
        if len <= 0 as libc::c_int { return 1 as libc::c_int as libc::c_uint }
        if memcmp((*cdbmp).cdb_buf.as_mut_ptr() as *const libc::c_void,
                  key as *const libc::c_void, len as libc::c_ulong) !=
               0 as libc::c_int {
            return 0 as libc::c_int as libc::c_uint
        }
        key = key.offset(len as isize);
        klen = klen.wrapping_sub(len as libc::c_uint)
    }
    return rlen;
}
unsafe extern "C" fn findrec(mut cdbmp: *mut cdb_make,
                             mut key: *const libc::c_void,
                             mut klen: libc::c_uint, mut hval: libc::c_uint,
                             mut mode: cdb_put_mode) -> libc::c_int {
    let mut rl: *mut cdb_rl = 0 as *mut cdb_rl;
    let mut rp: *mut cdb_rec = 0 as *mut cdb_rec;
    let mut rs: *mut cdb_rec = 0 as *mut cdb_rec;
    let mut r: libc::c_uint = 0;
    let mut seeked: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    rl =
        (*cdbmp).cdb_rec[(hval & 255 as libc::c_int as libc::c_uint) as
                             usize];
    's_18:
        while !rl.is_null() {
            rs = (*rl).rec.as_mut_ptr();
            rp = rs.offset((*rl).cnt as isize);
            loop  {
                rp = rp.offset(-1);
                if !(rp >= rs) { break ; }
                if (*rp).hval != hval { continue ; }
                /*XXX this explicit flush may be unnecessary having
       * smarter match() that looks into cdb_buf too, but
       * most of a time here spent in finding hash values
       * (above), not keys */
                if seeked == 0 && _cdb_make_flush(cdbmp) < 0 as libc::c_int {
                    return -(1 as libc::c_int)
                }
                seeked = 1 as libc::c_int;
                r =
                    match_0(cdbmp, (*rp).rpos, key as *const libc::c_char,
                            klen);
                if r == 0 { continue ; }
                if r == 1 as libc::c_int as libc::c_uint {
                    return -(1 as libc::c_int)
                }
                ret = 1 as libc::c_int;
                match mode as libc::c_uint {
                    1 => {
                        if remove_record(cdbmp, (*rp).rpos, r) <
                               0 as libc::c_int {
                            return -(1 as libc::c_int)
                        }
                    }
                    4 => {
                        if zerofill_record(cdbmp, (*rp).rpos, r) <
                               0 as libc::c_int {
                            return -(1 as libc::c_int)
                        }
                    }
                    _ => { break 's_18 ; }
                }
                memmove(rp as *mut libc::c_void,
                        rp.offset(1 as libc::c_int as isize) as
                            *const libc::c_void,
                        (rs.offset((*rl).cnt as
                                       isize).offset(-(1 as libc::c_int as
                                                           isize)).wrapping_offset_from(rp)
                             as libc::c_long as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cdb_rec>()
                                                             as
                                                             libc::c_ulong));
                (*rl).cnt = (*rl).cnt.wrapping_sub(1);
                (*cdbmp).cdb_rcnt = (*cdbmp).cdb_rcnt.wrapping_sub(1)
            }
            rl = (*rl).next
        }
    if seeked != 0 &&
           lseek((*cdbmp).cdb_fd, (*cdbmp).cdb_dpos as __off64_t,
                 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_find(mut cdbmp: *mut cdb_make,
                                       mut key: *const libc::c_void,
                                       mut klen: libc::c_uint,
                                       mut mode: cdb_put_mode)
 -> libc::c_int {
    return findrec(cdbmp, key, klen, cdb_hash(key, klen), mode);
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_exists(mut cdbmp: *mut cdb_make,
                                         mut key: *const libc::c_void,
                                         mut klen: libc::c_uint)
 -> libc::c_int {
    return cdb_make_find(cdbmp, key, klen, CDB_FIND);
}
#[no_mangle]
pub unsafe extern "C" fn cdb_make_put(mut cdbmp: *mut cdb_make,
                                      mut key: *const libc::c_void,
                                      mut klen: libc::c_uint,
                                      mut val: *const libc::c_void,
                                      mut vlen: libc::c_uint,
                                      mut mode: cdb_put_mode) -> libc::c_int {
    let mut hval: libc::c_uint = cdb_hash(key, klen);
    let mut r: libc::c_int = 0;
    match mode as libc::c_uint {
        1 | 2 | 3 | 4 => {
            r = findrec(cdbmp, key, klen, hval, mode);
            if r < 0 as libc::c_int { return -(1 as libc::c_int) }
            if r != 0 &&
                   mode as libc::c_uint ==
                       CDB_PUT_INSERT as libc::c_int as libc::c_uint {
                *__errno_location() = 17 as libc::c_int;
                return 1 as libc::c_int
            }
        }
        0 => { r = 0 as libc::c_int }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    if _cdb_make_add(cdbmp, hval, key, klen, val, vlen) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return r;
}
