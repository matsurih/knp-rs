use crate::juman::ctools::{_IO_codecvt, _IO_marker, _IO_wide_data};
use crate::juman::types::{__blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t, __syscall_slong_t, __time_t, __uid_t, BIN, caddr_t, CELL, size_t};

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BIN {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CELL {
    pub tag: libc::c_int,
    pub value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub cons: BIN,
    pub atom: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CELLTABLE {
    pub pre: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub max: libc::c_int,
    pub n: libc::c_int,
    pub cell: *mut CELL,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MRPH {
    pub midasi: [libc::c_uchar; 129],
    pub midasi2: [libc::c_uchar; 129],
    pub yomi: [libc::c_uchar; 129],
    pub imis: [libc::c_uchar; 1024],
    pub imi: *mut CELL,
    pub hinsi: libc::c_char,
    pub bunrui: libc::c_char,
    pub katuyou1: libc::c_char,
    pub katuyou2: libc::c_char,
    pub weight: libc::c_uchar,
    pub con_tbl: libc::c_int,
    pub length: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RENSETU_PAIR {
    pub i_pos: libc::c_int,
    pub j_pos: libc::c_int,
    pub hinsi: libc::c_int,
    pub bunrui: libc::c_int,
    pub type_0: libc::c_int,
    pub form: libc::c_int,
    pub goi: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CLASS {
    pub id: *mut libc::c_uchar,
    pub cost: libc::c_int,
    pub kt: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cons: BIN,
    pub atom: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TYPE {
    pub name: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FORM {
    pub name: *mut libc::c_uchar,
    pub gobi: *mut libc::c_uchar,
    pub gobi_yomi: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pat_index_list {
    pub next: *mut pat_index_list,
    pub index: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pat_node {
    pub il: pat_index_list,
    pub checkbit: libc::c_short,
    pub right: *mut pat_node,
    pub left: *mut pat_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dic_t {
    pub used: libc::c_int,
    pub fd: libc::c_int,
    pub size: libc::off_t,
    pub addr: caddr_t,
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

