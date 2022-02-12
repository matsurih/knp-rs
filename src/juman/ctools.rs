use libc;
use crate::juman::structs::{pat_index_list, pat_node, tm, stat};
use crate::juman::types::{__int32_t, __off_t, CELL, CLASS, FILE, FORM, size_t, time_t, TYPE};

pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum cdb_rl {}
pub enum internal_state {}

extern "C" {
    #[no_mangle]
    pub static mut CurPath: [libc::c_char; 4096];
    #[no_mangle]
    pub static mut JumanPath: [libc::c_char; 4096];
    #[no_mangle]
    pub static mut ProgName: *mut libc::c_char;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub static mut LineNo: libc::c_int;
    #[no_mangle]
    pub static mut LineNoForError: libc::c_int;
    #[no_mangle]
    pub static mut Jumangram_Dirname: [libc::c_char; 4096];
    #[no_mangle]
    pub static mut Class: [[CLASS; 129]; 129];
    #[no_mangle]
    pub static mut Type: [TYPE; 128];
    #[no_mangle]
    pub static mut Form: [[FORM; 128]; 128];
    #[no_mangle]
    pub fn error(errno: libc::c_int, msg: *mut libc::c_char, _: ...);
    #[no_mangle]
    pub fn my_exit(exit_code: libc::c_int);
    #[no_mangle]
    pub fn my_alloc(n: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    pub fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn pathfopen(filename: *mut libc::c_char, mode: *mut libc::c_char, path: *mut libc::c_char, filename_path: *mut libc::c_char) -> *mut FILE;
    #[no_mangle]
    pub fn getpath(cur_path: *mut libc::c_char, juman_path: *mut libc::c_char);
    #[no_mangle]
    pub fn print_current_time(fp: *mut FILE);
    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    pub fn s_feof(fp: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn car(cell: *mut CELL) -> *mut CELL;
    #[no_mangle]
    pub fn cdr(cell: *mut CELL) -> *mut CELL;
    #[no_mangle]
    pub fn s_read(fp: *mut FILE) -> *mut CELL;
    #[no_mangle]
    pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    pub fn perror(__s: *const libc::c_char);
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char, __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    pub fn asctime(__tp: *const tm) -> *mut libc::c_char;
    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn regcomp(__preg: *mut libc::regex_t, __pattern: *const libc::c_char, __cflags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn regexec(__preg: *const libc::regex_t, __String: *const libc::c_char, __nmatch: size_t, __pmatch: *mut libc::regmatch_t, __eflags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    pub fn pat_init_tree_top(_: *mut pat_node);
    #[no_mangle]
    pub fn com_l(_: *mut libc::c_char, _: *mut pat_node);
    #[no_mangle]
    pub fn my_fopen(filename: *mut libc::c_char, mode: *mut libc::c_char) -> *mut FILE;
    #[no_mangle]
    pub fn my_realloc(ptr: *mut libc::c_void, n: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    pub fn grammar(fp_out: *mut FILE);
    #[no_mangle]
    pub fn katuyou(fp: *mut FILE);
    #[no_mangle]
    pub fn connect_table(fp_out: *mut FILE);
    #[no_mangle]
    pub fn check_table_for_undef(hinsi: libc::c_int, bunrui: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn connect_matrix(fp_out: *mut FILE);
    #[no_mangle]
    pub fn check_matrix(postcon: libc::c_int, precon: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn check_matrix_left(precon: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn check_matrix_right(postcon: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn push_darts_file(basename: *mut libc::c_char);
    #[no_mangle]
    pub fn da_search(dic_no: libc::c_int, str: *mut libc::c_char, rslt: *mut libc::c_char) -> size_t;
    #[no_mangle]
    pub fn da_traverse(dic_no: libc::c_int, str: *mut libc::c_char, node_pos: *mut size_t, key_pos: size_t, key_length: size_t, key_type: libc::c_char, deleted_bytes: libc::c_char, rslt: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn close_darts();
    #[no_mangle]
    pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int, __flags: libc::c_int, __fd: libc::c_int, __offset: __off_t) -> *mut libc::c_void;
    #[no_mangle]
    pub fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    pub fn malloc_pat_node() -> *mut pat_node;
    #[no_mangle]
    pub fn malloc_pat_index_list() -> *mut pat_index_list;
    #[no_mangle]
    pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t);
    #[no_mangle]
    pub fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t);
}

#[inline]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else { __c };
}
#[inline]
pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else { __c };
}
#[inline]
pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}

