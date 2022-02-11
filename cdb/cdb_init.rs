#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off64_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
 -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
/* cdb_init.c: cdb_init, cdb_free and cdb_read routines
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_init(mut cdbp: *mut cdb, mut fd: libc::c_int)
 -> libc::c_int {
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut mem: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fsize: libc::c_uint = 0;
    let mut dend: libc::c_uint = 0;
    /* get file size */
    if fstat(fd, &mut st) < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* trivial sanity check: at least toc should be here */
    if st.st_size < 2048 as libc::c_int as libc::c_long {
        *__errno_location() = 71 as libc::c_int;
        return -(1 as libc::c_int)
    }
    fsize =
        if st.st_size < 0xffffffff as libc::c_uint as libc::c_long {
            st.st_size
        } else { 0xffffffff as libc::c_uint as libc::c_long } as libc::c_uint;
    /* memory-map file */
    mem =
        mmap(0 as *mut libc::c_void, fsize as size_t, 0x1 as libc::c_int,
             0x1 as libc::c_int, fd, 0 as libc::c_int as __off64_t) as
            *mut libc::c_uchar;
    if mem == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_uchar {
        return -(1 as libc::c_int)
    }
    /* _WIN32 */
    (*cdbp).cdb_fd = fd;
    (*cdbp).cdb_fsize = fsize;
    (*cdbp).cdb_mem = mem;
    (*cdbp).cdb_vlen = 0 as libc::c_int as libc::c_uint;
    (*cdbp).cdb_vpos = (*cdbp).cdb_vlen;
    (*cdbp).cdb_klen = 0 as libc::c_int as libc::c_uint;
    (*cdbp).cdb_kpos = (*cdbp).cdb_klen;
    dend = cdb_unpack(mem as *const libc::c_uchar);
    if dend < 2048 as libc::c_int as libc::c_uint {
        dend = 2048 as libc::c_int as libc::c_uint
    } else if dend >= fsize { dend = fsize }
    (*cdbp).cdb_dend = dend;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_free(mut cdbp: *mut cdb) {
    if !(*cdbp).cdb_mem.is_null() {
        munmap((*cdbp).cdb_mem as *mut libc::c_void,
               (*cdbp).cdb_fsize as size_t);
        /* _WIN32 */
        (*cdbp).cdb_mem = 0 as *const libc::c_uchar
    }
    (*cdbp).cdb_fsize = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_get(mut cdbp: *const cdb, mut len: libc::c_uint,
                                 mut pos: libc::c_uint)
 -> *const libc::c_void {
    if pos > (*cdbp).cdb_fsize || (*cdbp).cdb_fsize.wrapping_sub(pos) < len {
        *__errno_location() = 71 as libc::c_int;
        return 0 as *const libc::c_void
    }
    return (*cdbp).cdb_mem.offset(pos as isize) as *const libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn cdb_read(mut cdbp: *const cdb,
                                  mut buf: *mut libc::c_void,
                                  mut len: libc::c_uint,
                                  mut pos: libc::c_uint) -> libc::c_int {
    let mut data: *const libc::c_void = cdb_get(cdbp, len, pos);
    if data.is_null() { return -(1 as libc::c_int) }
    memcpy(buf, data, len as libc::c_ulong);
    return 0 as libc::c_int;
}
