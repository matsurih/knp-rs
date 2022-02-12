#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use std::env::args;
use crate::ctools::{_IO_codecvt, _IO_marker, _IO_wide_data, cdb_rl};
use crate::juman::ctools::error;

extern "C" {
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char, __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn cdb_unpack(buf_0: *const libc::c_uchar) -> libc::c_uint;
    #[no_mangle]
    fn cdb_init(cdbp: *mut cdb, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cdb_read(cdbp: *const cdb, buf_0: *mut libc::c_void, len: libc::c_uint, pos: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn cdb_findinit(cdbfp: *mut cdb_find, cdbp: *mut cdb, key: *const libc::c_void, klen: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn cdb_findnext(cdbfp: *mut cdb_find) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_start(cdbmp: *mut cdb_make, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_put(cdbmp: *mut cdb_make, key: *const libc::c_void, klen: libc::c_uint, val: *const libc::c_void, vlen: libc::c_uint, mode: cdb_put_mode) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_finish(cdbmp: *mut cdb_make) -> libc::c_int;
}

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type cdb_put_mode = libc::c_uint;
pub const CDB_FIND_FILL0: cdb_put_mode = 4;
pub const CDB_PUT_REPLACE0: cdb_put_mode = 4;
pub const CDB_PUT_WARN: cdb_put_mode = 3;
pub const CDB_PUT_INSERT: cdb_put_mode = 2;
pub const CDB_FIND_REMOVE: cdb_put_mode = 1;
pub const CDB_PUT_REPLACE: cdb_put_mode = 1;
pub const CDB_FIND: cdb_put_mode = 0;
pub const CDB_PUT_ADD: cdb_put_mode = 0;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
/* map format (or else CDB native format) */
/* Silly defines just to suppress silly compiler warnings.
 * The thing is, trivial routines like strlen(), fgets() etc expects
 * char* argument, and GCC>=4 complains about using unsigned char* here.
 * Silly silly silly.
 */
#[inline]
unsafe extern "C" fn ustrlen(mut s: *const libc::c_uchar) -> size_t {
    return strlen(s as *const libc::c_char);
}
#[inline]
unsafe extern "C" fn ufgets(mut s: *mut libc::c_uchar, mut size: libc::c_int,
                            mut f: *mut FILE) -> *mut libc::c_uchar {
    return fgets(s as *mut libc::c_char, size, f) as *mut libc::c_uchar;
}
static mut buf: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut blen: libc::c_uint = 0;

unsafe extern "C" fn allocbuf(mut len: libc::c_uint) {
    if blen < len {
        buf =
            if !buf.is_null() {
                realloc(buf as *mut libc::c_void, len as libc::c_ulong)
            } else { malloc(len as libc::c_ulong) } as *mut libc::c_uchar;
        if buf.is_null() {
            error(12 as libc::c_int,
                  b"unable to allocate %u bytes\x00" as *const u8 as *mut libc::c_char, len);
        }
        blen = len
    };
}
unsafe extern "C" fn qmode(mut dbname: *mut libc::c_char,
                           mut key: *const libc::c_char, mut num: libc::c_int,
                           mut flags: libc::c_int) -> libc::c_int {
    let mut c: cdb =
        cdb{cdb_fd: 0,
            cdb_fsize: 0,
            cdb_dend: 0,
            cdb_mem: 0 as *const libc::c_uchar,
            cdb_vpos: 0,
            cdb_vlen: 0,
            cdb_kpos: 0,
            cdb_klen: 0,};
    let mut cf: cdb_find =
        cdb_find{cdb_cdbp: 0 as *mut cdb,
                 cdb_hval: 0,
                 cdb_htp: 0 as *const libc::c_uchar,
                 cdb_htab: 0 as *const libc::c_uchar,
                 cdb_htend: 0 as *const libc::c_uchar,
                 cdb_httodo: 0,
                 cdb_key: 0 as *const libc::c_void,
                 cdb_klen: 0,};
    let mut r: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    r = open(dbname, 0 as libc::c_int);
    if r < 0 as libc::c_int || cdb_init(&mut c, r) != 0 as libc::c_int {
        error(*__errno_location(),
              b"unable to open database `%s\'\x00" as *const u8 as
                  *mut libc::c_char, dbname);
    }
    r =
        cdb_findinit(&mut cf, &mut c, key as *const libc::c_void,
                     strlen(key) as libc::c_uint);
    if r == 0 {
        return 100 as libc::c_int
    } else {
        if r < 0 as libc::c_int {
            error(*__errno_location(),
                  b"%s\x00" as *const u8 as *mut libc::c_char, key);
        }
    }
    n = 0 as libc::c_int;
    found = 0 as libc::c_int;
    loop  {
        r = cdb_findnext(&mut cf);
        if !(r > 0 as libc::c_int) { break ; }
        n += 1;
        if num != 0 && num != n { continue ; }
        found += 1;
        allocbuf(c.cdb_vlen);
        if cdb_read(&mut c, buf as *mut libc::c_void, c.cdb_vlen, c.cdb_vpos)
               != 0 as libc::c_int {
            error(*__errno_location(),
                  b"unable to read value\x00" as *const u8 as
                      *mut libc::c_char);
        }
        fwrite(buf as *const libc::c_void, 1 as libc::c_int as libc::c_ulong,
               c.cdb_vlen as libc::c_ulong, stdout);
        if flags & 0x1000 as libc::c_int != 0 { putchar('\n' as i32); }
        if num != 0 { break ; }
    }
    if r < 0 as libc::c_int {
        error(0 as libc::c_int, b"%s\x00" as *const u8 as *mut libc::c_char,
              key);
    }
    return if found != 0 { 0 as libc::c_int } else { 100 as libc::c_int };
}
unsafe extern "C" fn fget(mut f: *mut FILE, mut b: *mut libc::c_uchar,
                          mut len: libc::c_uint, mut posp: *mut libc::c_uint,
                          mut limit: libc::c_uint) {
    if !posp.is_null() && limit.wrapping_sub(*posp) < len {
        error(71 as libc::c_int,
              b"invalid database format\x00" as *const u8 as
                  *mut libc::c_char);
    }
    if fread(b as *mut libc::c_void, 1 as libc::c_int as libc::c_ulong,
             len as libc::c_ulong, f) != len as libc::c_ulong {
        if ferror(f) != 0 {
            error(*__errno_location(),
                  b"unable to read\x00" as *const u8 as *mut libc::c_char);
        }
        fprintf(stderr,
                b"%s: unable to read: short file\n\x00" as *const u8 as
                    *const libc::c_char, program_invocation_short_name);
        exit(2 as libc::c_int);
    }
    if !posp.is_null() { *posp = (*posp).wrapping_add(len) };
}
unsafe extern "C" fn fcpy(mut fi: *mut FILE, mut fo: *mut FILE,
                          mut len: libc::c_uint, mut posp: *mut libc::c_uint,
                          mut limit: libc::c_uint) -> libc::c_int {
    while len > blen {
        fget(fi, buf, blen, posp, limit);
        if !fo.is_null() &&
               fwrite(buf as *const libc::c_void,
                      1 as libc::c_int as libc::c_ulong,
                      blen as libc::c_ulong, fo) != blen as libc::c_ulong {
            return -(1 as libc::c_int)
        }
        len = len.wrapping_sub(blen)
    }
    if len != 0 {
        fget(fi, buf, len, posp, limit);
        if !fo.is_null() &&
               fwrite(buf as *const libc::c_void,
                      1 as libc::c_int as libc::c_ulong, len as libc::c_ulong,
                      fo) != len as libc::c_ulong {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dmode(mut dbname: *mut libc::c_char,
                           mut mode: libc::c_char, mut flags: libc::c_int)
 -> libc::c_int {
    let mut eod: libc::c_uint = 0;
    let mut klen: libc::c_uint = 0;
    let mut vlen: libc::c_uint = 0;
    let mut pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut f: *mut FILE = 0 as *mut FILE;
    if strcmp(dbname, b"-\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        f = stdin
    } else {
        f = fopen(dbname, b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            error(*__errno_location(),
                  b"open %s\x00" as *const u8 as *mut libc::c_char, dbname);
        }
    }
    allocbuf(2048 as libc::c_int as libc::c_uint);
    fget(f, buf, 2048 as libc::c_int as libc::c_uint, &mut pos,
         2048 as libc::c_int as libc::c_uint);
    eod = cdb_unpack(buf as *const libc::c_uchar);
    while pos < eod {
        fget(f, buf, 8 as libc::c_int as libc::c_uint, &mut pos, eod);
        klen = cdb_unpack(buf as *const libc::c_uchar);
        vlen =
            cdb_unpack(buf.offset(4 as libc::c_int as isize) as
                           *const libc::c_uchar);
        if flags & 0x1000 as libc::c_int == 0 {
            if printf((if mode as libc::c_int == 'd' as i32 {
                           b"+%u,%u:\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"+%u:\x00" as *const u8 as *const libc::c_char
                       }), klen, vlen) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
        }
        if fcpy(f, stdout, klen, &mut pos, eod) != 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if mode as libc::c_int == 'd' as i32 {
            if fputs((if flags & 0x1000 as libc::c_int != 0 {
                          b" \x00" as *const u8 as *const libc::c_char
                      } else {
                          b"->\x00" as *const u8 as *const libc::c_char
                      }), stdout) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
        }
        if fcpy(f,
                (if mode as libc::c_int == 'd' as i32 {
                     stdout
                 } else { 0 as *mut FILE }), vlen, &mut pos, eod) !=
               0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if putc('\n' as i32, stdout) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
    }
    if pos != eod {
        error(71 as libc::c_int,
              b"invalid cdb file format\x00" as *const u8 as
                  *mut libc::c_char);
    }
    if flags & 0x1000 as libc::c_int == 0 {
        if putc('\n' as i32, stdout) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn smode(mut dbname: *mut libc::c_char) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut pos: libc::c_uint = 0;
    let mut eod: libc::c_uint = 0;
    let mut cnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut kmin: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut kmax: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ktot: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vmin: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vmax: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vtot: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hmin: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hmax: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut htot: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hcnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dist: [libc::c_uint; 11] = [0; 11];
    let mut toc: [libc::c_uchar; 2048] = [0; 2048];
    let mut k: libc::c_uint = 0;
    if strcmp(dbname, b"-\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        f = stdin
    } else {
        f = fopen(dbname, b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            error(*__errno_location(),
                  b"open %s\x00" as *const u8 as *mut libc::c_char, dbname);
        }
    }
    pos = 0 as libc::c_int as libc::c_uint;
    fget(f, toc.as_mut_ptr(), 2048 as libc::c_int as libc::c_uint, &mut pos,
         2048 as libc::c_int as libc::c_uint);
    allocbuf(2048 as libc::c_int as libc::c_uint);
    eod = cdb_unpack(toc.as_mut_ptr() as *const libc::c_uchar);
    while pos < eod {
        let mut klen: libc::c_uint = 0;
        let mut vlen: libc::c_uint = 0;
        fget(f, buf, 8 as libc::c_int as libc::c_uint, &mut pos, eod);
        klen = cdb_unpack(buf as *const libc::c_uchar);
        vlen =
            cdb_unpack(buf.offset(4 as libc::c_int as isize) as
                           *const libc::c_uchar);
        fcpy(f, 0 as *mut FILE, klen, &mut pos, eod);
        fcpy(f, 0 as *mut FILE, vlen, &mut pos, eod);
        cnt = cnt.wrapping_add(1);
        ktot = ktot.wrapping_add(klen);
        if kmin == 0 || kmin > klen { kmin = klen }
        if kmax < klen { kmax = klen }
        vtot = vtot.wrapping_add(vlen);
        if vmin == 0 || vmin > vlen { vmin = vlen }
        if vmax < vlen { vmax = vlen }
        vlen = vlen.wrapping_add(klen)
    }
    if pos != eod {
        error(71 as libc::c_int,
              b"invalid cdb file format\x00" as *const u8 as
                  *mut libc::c_char);
    }
    k = 0 as libc::c_int as libc::c_uint;
    while k < 11 as libc::c_int as libc::c_uint {
        dist[k as usize] = 0 as libc::c_int as libc::c_uint;
        k = k.wrapping_add(1)
    }
    k = 0 as libc::c_int as libc::c_uint;
    while k < 256 as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint =
            cdb_unpack(toc.as_mut_ptr().offset((k << 3 as libc::c_int) as
                                                   isize) as
                           *const libc::c_uchar);
        let mut hlen: libc::c_uint =
            cdb_unpack(toc.as_mut_ptr().offset((k << 3 as libc::c_int) as
                                                   isize).offset(4 as
                                                                     libc::c_int
                                                                     as isize)
                           as *const libc::c_uchar);
        if i != pos {
            error(71 as libc::c_int,
                  b"invalid cdb hash table\x00" as *const u8 as
                      *mut libc::c_char);
        }
        if !(hlen == 0) {
            i = 0 as libc::c_int as libc::c_uint;
            while i < hlen {
                let mut h: libc::c_uint = 0;
                fget(f, buf, 8 as libc::c_int as libc::c_uint, &mut pos,
                     0xffffffff as libc::c_uint);
                if !(cdb_unpack(buf.offset(4 as libc::c_int as isize) as
                                    *const libc::c_uchar) == 0) {
                    h =
                        (cdb_unpack(buf as *const libc::c_uchar) >>
                             8 as libc::c_int).wrapping_rem(hlen);
                    if h == i {
                        h = 0 as libc::c_int as libc::c_uint
                    } else {
                        if h < i {
                            h = i.wrapping_sub(h)
                        } else { h = hlen.wrapping_sub(h).wrapping_add(i) }
                        if h >= 11 as libc::c_int as libc::c_uint {
                            h =
                                (11 as libc::c_int - 1 as libc::c_int) as
                                    libc::c_uint
                        }
                    }
                    dist[h as usize] = dist[h as usize].wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            if hmin == 0 || hmin > hlen { hmin = hlen }
            if hmax < hlen { hmax = hlen }
            htot = htot.wrapping_add(hlen);
            hcnt = hcnt.wrapping_add(1)
        }
        k = k.wrapping_add(1)
    }
    printf(b"number of records: %u\n\x00" as *const u8 as *const libc::c_char,
           cnt);
    printf(b"key min/avg/max length: %u/%u/%u\n\x00" as *const u8 as
               *const libc::c_char, kmin,
           if cnt != 0 {
               ktot.wrapping_add(cnt.wrapping_div(2 as libc::c_int as
                                                      libc::c_uint)).wrapping_div(cnt)
           } else { 0 as libc::c_int as libc::c_uint }, kmax);
    printf(b"val min/avg/max length: %u/%u/%u\n\x00" as *const u8 as
               *const libc::c_char, vmin,
           if cnt != 0 {
               vtot.wrapping_add(cnt.wrapping_div(2 as libc::c_int as
                                                      libc::c_uint)).wrapping_div(cnt)
           } else { 0 as libc::c_int as libc::c_uint }, vmax);
    printf(b"hash tables/entries/collisions: %u/%u/%u\n\x00" as *const u8 as
               *const libc::c_char, hcnt, htot,
           cnt.wrapping_sub(dist[0 as libc::c_int as usize]));
    printf(b"hash table min/avg/max length: %u/%u/%u\n\x00" as *const u8 as
               *const libc::c_char, hmin,
           if hcnt != 0 {
               htot.wrapping_add(hcnt.wrapping_div(2 as libc::c_int as
                                                       libc::c_uint)).wrapping_div(hcnt)
           } else { 0 as libc::c_int as libc::c_uint }, hmax);
    printf(b"hash table distances:\n\x00" as *const u8 as
               *const libc::c_char);
    k = 0 as libc::c_int as libc::c_uint;
    while k < 11 as libc::c_int as libc::c_uint {
        printf(b" %c%u: %6u %2u%%\n\x00" as *const u8 as *const libc::c_char,
               if k == (11 as libc::c_int - 1 as libc::c_int) as libc::c_uint
                  {
                   '>' as i32
               } else { 'd' as i32 },
               if k == (11 as libc::c_int - 1 as libc::c_int) as libc::c_uint
                  {
                   k.wrapping_sub(1 as libc::c_int as libc::c_uint)
               } else { k }, dist[k as usize],
               if cnt != 0 {
                   dist[k as
                            usize].wrapping_mul(100 as libc::c_int as
                                                    libc::c_uint).wrapping_div(cnt)
               } else { 0 as libc::c_int as libc::c_uint });
        k = k.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn badinput(mut fn_0: *const libc::c_char) {
    fprintf(stderr,
            b"%s: %s: bad format\n\x00" as *const u8 as *const libc::c_char,
            program_invocation_short_name, fn_0);
    exit(2 as libc::c_int);
}
unsafe extern "C" fn getnum(mut f: *mut FILE, mut np: *mut libc::c_uint,
                            mut fn_0: *const libc::c_char) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut c: libc::c_int = getc(f);
    if c < '0' as i32 || c > '9' as i32 { badinput(fn_0); }
    n = (c - '0' as i32) as libc::c_uint;
    loop  {
        c = getc(f);
        if !(c >= '0' as i32 && c <= '9' as i32) { break ; }
        c -= '0' as i32;
        if (0xffffffff as
                libc::c_uint).wrapping_div(10 as libc::c_int as
                                               libc::c_uint).wrapping_sub(c as
                                                                              libc::c_uint)
               < n {
            badinput(fn_0);
        }
        n =
            n.wrapping_mul(10 as libc::c_int as
                               libc::c_uint).wrapping_add(c as libc::c_uint)
    }
    *np = n;
    return c;
}
unsafe extern "C" fn addrec(mut cdbmp: *mut cdb_make,
                            mut key: *const libc::c_uchar,
                            mut klen: libc::c_uint,
                            mut val: *const libc::c_uchar,
                            mut vlen: libc::c_uint, mut flags: libc::c_int) {
    let mut r: libc::c_int =
        cdb_make_put(cdbmp, key as *const libc::c_void, klen,
                     val as *const libc::c_void, vlen,
                     (flags & 0xf as libc::c_int) as cdb_put_mode);
    if r < 0 as libc::c_int {
        error(*__errno_location(),
              b"cdb_make_put\x00" as *const u8 as *mut libc::c_char);
    } else {
        if r != 0 && flags & 0x100 as libc::c_int != 0 {
            fprintf(stderr,
                    b"%s: key `\x00" as *const u8 as *const libc::c_char,
                    program_invocation_short_name);
            fwrite(key as *const libc::c_void,
                   1 as libc::c_int as libc::c_ulong, klen as libc::c_ulong,
                   stderr);
            fputs(b"\' duplicated\n\x00" as *const u8 as *const libc::c_char,
                  stderr);
            if flags & 0x200 as libc::c_int != 0 { exit(1 as libc::c_int); }
        }
    };
}
unsafe extern "C" fn dofile_cdb(mut cdbmp: *mut cdb_make, mut f: *mut FILE,
                                mut fn_0: *const libc::c_char,
                                mut flags: libc::c_int) {
    let mut klen: libc::c_uint = 0;
    let mut vlen: libc::c_uint = 0;
    let mut c: libc::c_int = 0;
    loop  {
        c = getc(f);
        if !(c == '+' as i32) { break ; }
        c = getnum(f, &mut klen, fn_0);
        if c != ',' as i32 ||
               { c = getnum(f, &mut vlen, fn_0); (c) != ':' as i32 } ||
               (0xffffffff as libc::c_uint).wrapping_sub(klen) < vlen {
            badinput(fn_0);
        }
        allocbuf(klen.wrapping_add(vlen));
        fget(f, buf, klen, 0 as *mut libc::c_uint,
             0 as libc::c_int as libc::c_uint);
        if getc(f) != '-' as i32 || getc(f) != '>' as i32 { badinput(fn_0); }
        fget(f, buf.offset(klen as isize), vlen, 0 as *mut libc::c_uint,
             0 as libc::c_int as libc::c_uint);
        if getc(f) != '\n' as i32 { badinput(fn_0); }
        addrec(cdbmp, buf, klen, buf.offset(klen as isize), vlen, flags);
    }
    if c != '\n' as i32 { badinput(fn_0); };
}
unsafe extern "C" fn dofile_ln(mut cdbmp: *mut cdb_make, mut f: *mut FILE,
                               mut flags: libc::c_int) {
    let mut k: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    while !ufgets(buf, blen as libc::c_int, f).is_null() {
        let mut l: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        loop  {
            l =
                (l as
                     libc::c_ulong).wrapping_add(ustrlen(buf.offset(l as
                                                                        isize)))
                    as libc::c_uint as libc::c_uint;
            v = buf.offset(l as isize);
            if v > buf &&
                   *v.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                       '\n' as i32 {
                *v.offset(-(1 as libc::c_int) as isize) =
                    '\u{0}' as i32 as libc::c_uchar;
                break ;
            } else {
                if l < blen {
                    allocbuf(l.wrapping_add(512 as libc::c_int as
                                                libc::c_uint));
                }
                if ufgets(buf.offset(l as isize),
                          blen.wrapping_sub(l) as libc::c_int, f).is_null() {
                    break ;
                }
            }
        }
        k = buf;
        while *k as libc::c_int == ' ' as i32 ||
                  *k as libc::c_int == '\t' as i32 {
            k = k.offset(1)
        }
        if *k == 0 || *k as libc::c_int == '#' as i32 { continue ; }
        v = k;
        while *v as libc::c_int != 0 && *v as libc::c_int != ' ' as i32 &&
                  *v as libc::c_int != '\t' as i32 {
            v = v.offset(1)
        }
        if *v != 0 {
            let fresh0 = v;
            v = v.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_uchar
        }
        while *v as libc::c_int == ' ' as i32 ||
                  *v as libc::c_int == '\t' as i32 {
            v = v.offset(1)
        }
        addrec(cdbmp, k, ustrlen(k) as libc::c_uint, v,
               ustrlen(v) as libc::c_uint, flags);
    };
}
unsafe extern "C" fn dofile(mut cdbmp: *mut cdb_make, mut f: *mut FILE,
                            mut fn_0: *const libc::c_char,
                            mut flags: libc::c_int) {
    if flags & 0x1000 as libc::c_int != 0 {
        dofile_ln(cdbmp, f, flags);
    } else { dofile_cdb(cdbmp, f, fn_0, flags); }
    if ferror(f) != 0 {
        error(*__errno_location(),
              b"read error\x00" as *const u8 as *mut libc::c_char);
    };
}
unsafe extern "C" fn cmode(mut dbname: *mut libc::c_char,
                           mut tmpname: *mut libc::c_char,
                           mut argc: libc::c_int,
                           mut argv: *mut *mut libc::c_char,
                           mut flags: libc::c_int, mut perms: libc::c_int)
 -> libc::c_int {
    let mut cdb: cdb_make =
        cdb_make{cdb_fd: 0,
                 cdb_dpos: 0,
                 cdb_rcnt: 0,
                 cdb_buf: [0; 4096],
                 cdb_bpos: 0 as *mut libc::c_uchar,
                 cdb_rec: [0 as *mut cdb_rl; 256],};
    let mut fd: libc::c_int = 0;
    if tmpname.is_null() {
        tmpname =
            malloc(strlen(dbname).wrapping_add(5 as libc::c_int as
                                                   libc::c_ulong)) as
                *mut libc::c_char;
        if tmpname.is_null() {
            error(12 as libc::c_int,
                  b"unable to allocate memory\x00" as *const u8 as
                      *mut libc::c_char);
        }
        /* OpenBSD compiler complains about strcat() and strcpy() usage,
     * and suggests to replace them with (non-standard) strlcat() and
     * strlcpy().  This is silly, since it's obvious that usage of
     * original str*() routines here is correct.
     * This is compiler/environment bug, not tinycdb bug, so please
     * fix it in proper place, and don't send patches to me.  Thank you.
     */
        strcat(strcpy(tmpname, dbname),
               b".tmp\x00" as *const u8 as *const libc::c_char);
    } else if strcmp(tmpname, b"-\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int ||
                  strcmp(tmpname, dbname) == 0 as libc::c_int {
        tmpname = dbname
    }
    if perms >= 0 as libc::c_int { umask(0 as libc::c_int as __mode_t); }
    unlink(tmpname);
    fd =
        open(tmpname,
             0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
                 | 0o400000 as libc::c_int,
             if perms >= 0 as libc::c_int {
                 perms
             } else { 0o666 as libc::c_int });
    if fd < 0 as libc::c_int {
        error(*__errno_location(),
              b"unable to create %s\x00" as *const u8 as *mut libc::c_char,
              tmpname);
    }
    cdb_make_start(&mut cdb, fd);
    allocbuf(4096 as libc::c_int as libc::c_uint);
    if argc != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < argc {
            if strcmp(*argv.offset(i as isize),
                      b"-\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                dofile(&mut cdb, stdin,
                       b"(stdin)\x00" as *const u8 as *const libc::c_char,
                       flags);
            } else {
                let mut f: *mut FILE =
                    fopen(*argv.offset(i as isize),
                          b"r\x00" as *const u8 as *const libc::c_char);
                if f.is_null() {
                    error(*__errno_location(),
                          b"%s\x00" as *const u8 as *mut libc::c_char,
                          *argv.offset(i as isize));
                }
                dofile(&mut cdb, f, *argv.offset(i as isize), flags);
                fclose(f);
            }
            i += 1
        }
    } else {
        dofile(&mut cdb, stdin,
               b"(stdin)\x00" as *const u8 as *const libc::c_char, flags);
    }
    if cdb_make_finish(&mut cdb) != 0 as libc::c_int {
        error(*__errno_location(),
              b"cdb_make_finish\x00" as *const u8 as *mut libc::c_char);
    }
    close(fd);
    if tmpname != dbname {
        if rename(tmpname, dbname) != 0 as libc::c_int {
            error(*__errno_location(),
                  b"rename %s->%s\x00" as *const u8 as *mut libc::c_char,
                  tmpname, dbname);
        }
    }
    return 0 as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut mode: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut tmpname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut perms: libc::c_int = -(1 as libc::c_int);
    extern "C" {
        #[link_name = "optarg"]
        static mut optarg_0: *mut libc::c_char;
    }
    extern "C" {
        #[link_name = "optind"]
        static mut optind_0: libc::c_int;
    }
    let ref mut fresh1 = *argv.offset(0 as libc::c_int as isize);
    *fresh1 = program_invocation_short_name;
    if argc <= 1 as libc::c_int {
        error(0 as libc::c_int,
              b"no arguments given\x00" as *const u8 as *mut libc::c_char);
    }
    loop  {
        c =
            getopt(argc, argv,
                   b"qdlcsht:n:mwruep:0\x00" as *const u8 as
                       *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            113 | 100 | 108 | 99 | 115 => {
                if mode as libc::c_int != 0 && mode as libc::c_int != c {
                    error(0 as libc::c_int,
                          b"different modes of operation requested\x00" as
                              *const u8 as *mut libc::c_char);
                }
                mode = c as libc::c_char
            }
            116 => { tmpname = optarg }
            119 => { flags |= 0x100 as libc::c_int }
            101 => { flags |= 0x100 as libc::c_int | 0x200 as libc::c_int }
            114 => {
                flags =
                    flags & !(0xf as libc::c_int) |
                        CDB_PUT_REPLACE as libc::c_int
            }
            117 => {
                flags =
                    flags & !(0xf as libc::c_int) |
                        CDB_PUT_INSERT as libc::c_int
            }
            48 => {
                flags =
                    flags & !(0xf as libc::c_int) |
                        CDB_PUT_REPLACE0 as libc::c_int
            }
            109 => { flags |= 0x1000 as libc::c_int }
            112 => {
                let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
                perms =
                    strtol(optarg, &mut ep, 0 as libc::c_int) as libc::c_int;
                if perms < 0 as libc::c_int || perms > 0o777 as libc::c_int ||
                       !ep.is_null() && *ep as libc::c_int != 0 {
                    error(0 as libc::c_int,
                          b"invalid permissions `%s\'\x00" as *const u8 as
                              *mut libc::c_char, optarg);
                }
            }
            110 => {
                let mut ep_0: *mut libc::c_char = 0 as *mut libc::c_char;
                num =
                    strtol(optarg, &mut ep_0, 0 as libc::c_int) as
                        libc::c_int;
                if num <= 0 as libc::c_int ||
                       !ep_0.is_null() && *ep_0 as libc::c_int != 0 {
                    error(0 as libc::c_int,
                          b"invalid record number `%s\'\x00" as *const u8 as
                              *mut libc::c_char, optarg);
                }
            }
            104 => {
                printf(b"%s: Constant DataBase (CDB) tool version 0.78. Usage is:\n query:  %s -q [-m] [-n recno|-a] cdbfile key\n dump:   %s -d [-m] [cdbfile|-]\n list:   %s -l [-m] [cdbfile|-]\n create: %s -c [-m] [-wrue0] [-t tempfile|-] [-p perms] cdbfile [infile...]\n stats:  %s -s [cdbfile|-]\n help:   %s -h\n\x00"
                           as *const u8 as *const libc::c_char,
                       program_invocation_short_name,
                       program_invocation_short_name,
                       program_invocation_short_name,
                       program_invocation_short_name,
                       program_invocation_short_name,
                       program_invocation_short_name,
                       program_invocation_short_name);
                return 0 as libc::c_int
            }
            _ => { error(0 as libc::c_int, 0 as *mut libc::c_char); }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    match mode as libc::c_int {
        113 => {
            if argc < 2 as libc::c_int {
                error(0 as libc::c_int,
                      b"no database or key to query specified\x00" as
                          *const u8 as *mut libc::c_char);
            }
            if argc > 2 as libc::c_int {
                error(0 as libc::c_int,
                      b"extra arguments in command line\x00" as *const u8 as
                          *mut libc::c_char);
            }
            r =
                qmode(*argv.offset(0 as libc::c_int as isize),
                      *argv.offset(1 as libc::c_int as isize), num, flags)
        }
        99 => {
            if argc == 0 {
                error(0 as libc::c_int,
                      b"no database name specified\x00" as *const u8 as
                          *mut libc::c_char);
            }
            if flags & 0x100 as libc::c_int != 0 &&
                   flags & 0xf as libc::c_int == 0 {
                flags |= CDB_PUT_WARN as libc::c_int
            }
            r =
                cmode(*argv.offset(0 as libc::c_int as isize), tmpname,
                      argc - 1 as libc::c_int,
                      argv.offset(1 as libc::c_int as isize), flags, perms)
        }
        100 | 108 => {
            if argc > 1 as libc::c_int {
                error(0 as libc::c_int,
                      b"extra arguments for dump/list\x00" as *const u8 as
                          *mut libc::c_char);
            }
            r =
                dmode(if argc != 0 {
                          *argv.offset(0 as libc::c_int as isize) as
                              *const libc::c_char
                      } else { b"-\x00" as *const u8 as *const libc::c_char }
                          as *mut libc::c_char, mode, flags)
        }
        115 => {
            if argc > 1 as libc::c_int {
                error(0 as libc::c_int,
                      b"extra argument(s) for stats\x00" as *const u8 as
                          *mut libc::c_char);
            }
            r =
                smode(if argc != 0 {
                          *argv.offset(0 as libc::c_int as isize) as
                              *const libc::c_char
                      } else { b"-\x00" as *const u8 as *const libc::c_char }
                          as *mut libc::c_char)
        }
        _ => {
            error(0 as libc::c_int,
                  b"no -q, -c, -d, -l or -s option specified\x00" as *const u8
                      as *mut libc::c_char);
        }
    }
    if r < 0 as libc::c_int || fflush(stdout) < 0 as libc::c_int {
        error(*__errno_location(),
              b"unable to write: %d\x00" as *const u8 as *mut libc::c_char,
              c);
    }
    return r;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
