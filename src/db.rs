#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! Database

use libc;

use crate::{close, fprintf, free, memset, strerror, strlen};
use crate::ctools::{__errno_location, cdb_find, cdb_init, cdb_make_add, cdb_make_finish, cdb_make_start, cdb_read, deflate, deflateEnd, deflateInit_, internal_state, malloc, malloc_data, memcpy, open, realloc, stderr};
use crate::structs::CDB_FILE;
use crate::types::{Bytef, DBM_FILE, size_t, uInt, z_stream};

#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn compress_string(mut str: *mut libc::c_char,
                                         mut compressed_size:
                                         *mut libc::c_int,
                                         mut compressionlevel: libc::c_int)
                                         -> *mut libc::c_char {
    let mut ret: libc::c_int = 0;
    let mut out_capacity: libc::c_int = 0 as libc::c_int;
    let mut cur_out_size: libc::c_int = 0 as libc::c_int;
    let mut buffer: [libc::c_char; 32768] = [0; 32768];
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zs: z_stream =
        z_stream {
            next_in: 0 as *mut Bytef,
            avail_in: 0,
            total_in: 0,
            next_out: 0 as *mut Bytef,
            avail_out: 0,
            total_out: 0,
            msg: 0 as *mut libc::c_char,
            state: 0 as *mut internal_state,
            zalloc: None,
            zfree: None,
            opaque: 0 as *mut libc::c_void,
            data_type: 0,
            adler: 0,
            reserved: 0,
        };
    out_capacity = 100000 as libc::c_int;
    out = malloc(out_capacity as libc::c_ulong) as *mut libc::c_char;
    if out.is_null() {
        fprintf(stderr,
                b"malloc error\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    *out.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    memset(&mut zs as *mut z_stream as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<z_stream>() as libc::c_ulong);
    if deflateInit_(&mut zs, compressionlevel,
                    b"1.2.11\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<z_stream>() as libc::c_ulong as
                        libc::c_int) != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    zs.next_in = str as *mut Bytef;
    zs.avail_in = strlen(str) as uInt;
    loop {
        zs.next_out = buffer.as_mut_ptr() as *mut Bytef;
        zs.avail_out =
            ::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong as
                uInt;
        ret = deflate(&mut zs, 4 as libc::c_int);
        if (cur_out_size as libc::c_ulong) < zs.total_out {
            if (out_capacity as libc::c_ulong) <
                zs.total_out.wrapping_sub(1 as libc::c_int as
                    libc::c_ulong) {
                let mut tmp_out: *mut libc::c_char = 0 as *mut libc::c_char;
                out_capacity += 100000 as libc::c_int;
                tmp_out =
                    realloc(out as *mut libc::c_void,
                            out_capacity as libc::c_ulong) as
                        *mut libc::c_char;
                if tmp_out.is_null() {
                    fprintf(stderr,
                            b"realloc error\n\x00" as *const u8 as
                                *const libc::c_char);
                    free(out as *mut libc::c_void);
                    return 0 as *mut libc::c_char;
                } else { out = tmp_out }
            }
            memcpy(out.offset(cur_out_size as isize) as *mut libc::c_void,
                   buffer.as_mut_ptr() as *const libc::c_void,
                   zs.total_out.wrapping_sub(cur_out_size as libc::c_ulong));
            // strncat(out, buffer, zs.total_out - strlen(out));
            cur_out_size = zs.total_out as libc::c_int;
            *out.offset(cur_out_size as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        if !(ret == 0 as libc::c_int) { break; }
    }
    deflateEnd(&mut zs);
    *compressed_size = zs.total_out as libc::c_int;
    if ret != 1 as libc::c_int {
        fprintf(stderr,
                b"Exception during zlib compression: (%d) %s\n\x00" as
                    *const u8 as *const libc::c_char, ret, zs.msg);
        return 0 as *mut libc::c_char;
    }
    return out;
}
/* BerkeleyDB 3 */
/* DB open for reading */
#[no_mangle]
pub unsafe extern "C" fn db_read_open(mut filename: *mut libc::c_char)
                                      -> DBM_FILE {
    let mut db: DBM_FILE = 0 as *mut CDB_FILE;
    db =
        malloc_data(::std::mem::size_of::<CDB_FILE>() as libc::c_ulong,
                    b"db_read_open\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as DBM_FILE;
    (*db).mode = 0 as libc::c_int;
    (*db).fd = open(filename, (*db).mode);
    if (*db).fd < 0 as libc::c_int {
        free(db as *mut libc::c_void);
        return 0 as DBM_FILE;
    }
    if cdb_init(&mut (*db).cdb, (*db).fd) != 0 as libc::c_int {
        /* mmap error? */
        free(db as *mut libc::c_void);
        return 0 as DBM_FILE;
    }
    return db;
}
/* DB open for writing */
#[no_mangle]
pub unsafe extern "C" fn db_write_open(mut filename: *mut libc::c_char)
                                       -> DBM_FILE {
    let mut db: DBM_FILE = 0 as *mut CDB_FILE;
    db =
        malloc_data(::std::mem::size_of::<CDB_FILE>() as libc::c_ulong,
                    b"db_write_open\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) as DBM_FILE;
    (*db).mode =
        0o100 as libc::c_int | 0o2 as libc::c_int | 0o1000 as libc::c_int;
    (*db).fd =
        open(filename, (*db).mode,
             0o400 as libc::c_int | 0o200 as libc::c_int |
                 0o400 as libc::c_int >> 3 as libc::c_int |
                 0o400 as libc::c_int >> 3 as libc::c_int >>
                     3 as libc::c_int);
    if (*db).fd < 0 as libc::c_int {
        fprintf(stderr,
                b"db_write_open: %s: %s\n\x00" as *const u8 as
                    *const libc::c_char, filename,
                strerror(*__errno_location()));
        return 0 as DBM_FILE;
    }
    cdb_make_start(&mut (*db).cdbm, (*db).fd);
    return db;
}
/* DB close */
#[no_mangle]
pub unsafe extern "C" fn db_close(mut db: DBM_FILE) {
    if (*db).mode & 0o100 as libc::c_int != 0 {
        cdb_make_finish(&mut (*db).cdbm);
    }
    close((*db).fd);
    free(db as *mut libc::c_void);
}
/* DB get */
#[no_mangle]
pub unsafe extern "C" fn db_get(mut db: DBM_FILE, mut buf: *mut libc::c_char)
                                -> *mut libc::c_char {
    if cdb_find(&mut (*db).cdb, buf as *const libc::c_void,
                strlen(buf) as libc::c_uint) > 0 as libc::c_int {
        let mut rbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut datalen: libc::c_uint = 0;
        datalen = (*db).cdb.cdb_vlen;
        rbuf =
            malloc_data(datalen.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as size_t,
                        b"db_get\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char) as *mut libc::c_char;
        cdb_read(&mut (*db).cdb, rbuf as *mut libc::c_void, datalen,
                 (*db).cdb.cdb_vpos);
        *rbuf.offset(datalen as isize) = '\u{0}' as i32 as libc::c_char;
        return rbuf;
    }
    return 0 as *mut libc::c_char;
}
/* DB put */
#[no_mangle]
pub unsafe extern "C" fn db_put(mut db: DBM_FILE, mut buf: *mut libc::c_char,
                                mut value: *mut libc::c_char,
                                mut Separator: *mut libc::c_char,
                                mut mode: libc::c_int) -> libc::c_int {
    /* overwrite anytime ignoring the mode (CDB doesn't support rewriting) */
    if mode == 5 as libc::c_int {
        /* compress value */
        let mut compressed_size: libc::c_int = 0; /* the size of resuting compressed data */
        let mut compressed_value: *mut libc::c_char = compress_string(value, &mut compressed_size, -(1 as libc::c_int)); /* Z_BEST_SPEED */
        cdb_make_add(&mut (*db).cdbm, buf as *const libc::c_void, strlen(buf) as libc::c_uint, compressed_value as *const libc::c_void, compressed_size as libc::c_uint);
        free(compressed_value as *mut libc::c_void);
    } else {
        cdb_make_add(&mut (*db).cdbm, buf as *const libc::c_void, strlen(buf) as libc::c_uint, value as *const libc::c_void, strlen(value) as libc::c_uint);
    }
    return 0 as libc::c_int;
}
