#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;

    #[no_mangle]
    static mut Class: [[CLASS; 129]; 129];
    #[no_mangle]
    static mut Type: [TYPE; 128];
    #[no_mangle]
    static mut Form: [[FORM; 128]; 128];
    #[no_mangle]
    static mut rensetu_tbl: *mut RENSETU_PAIR;
    #[no_mangle]
    static mut rensetu_mtr: *mut libc::c_uchar;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut libc::c_char,
    pub __translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits =
               "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits =
               "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits =
               "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
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
pub type _ExitCode = libc::c_uint;
pub const OtherError: _ExitCode = 11;
pub const UnknownId: _ExitCode = 10;
pub const SyntaxError: _ExitCode = 9;
pub const ProgramError: _ExitCode = 8;
pub const ConfigError: _ExitCode = 7;
pub const ConnError: _ExitCode = 6;
pub const DicError: _ExitCode = 5;
pub const GramError: _ExitCode = 4;
pub const AllocateError: _ExitCode = 3;
pub const OpenError: _ExitCode = 2;
pub const SystemError: _ExitCode = 1;
pub const NormalExit: _ExitCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BIN {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}
pub type BIN = _BIN;
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
pub type CELL = _CELL;
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
pub type MRPH = _MRPH;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CLASS {
    pub id: *mut libc::c_uchar,
    pub cost: libc::c_int,
    pub kt: libc::c_int,
}
pub type CLASS = _CLASS;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TYPE {
    pub name: *mut libc::c_uchar,
}
pub type TYPE = _TYPE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FORM {
    pub name: *mut libc::c_uchar,
    pub gobi: *mut libc::c_uchar,
    pub gobi_yomi: *mut libc::c_uchar,
}
pub type FORM = _FORM;
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
pub type RENSETU_PAIR = _RENSETU_PAIR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _process_buffer {
    pub mrph_p: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub score: libc::c_int,
    pub path: [libc::c_int; 500],
    pub connect: libc::c_int,
}
pub type PROCESS_BUFFER = _process_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chk_connect_wk {
    pub pre_p: libc::c_int,
    pub score: libc::c_int,
}
pub type CHK_CONNECT_WK = _chk_connect_wk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _connect_cost {
    pub p_no: libc::c_short,
    pub pos: libc::c_short,
    pub cost: libc::c_int,
    pub opt: libc::c_char,
}
pub type CONNECT_COST = _connect_cost;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _char_node {
    pub next: *mut _char_node,
    pub chr: [libc::c_char; 7],
    pub type_0: libc::c_char,
    pub da_node_pos: [size_t; 10],
    pub node_type: [libc::c_char; 10],
    pub deleted_bytes: [libc::c_char; 10],
    pub p_buffer: [*mut libc::c_char; 10],
    pub da_node_pos_num: size_t,
}
pub type CHAR_NODE = _char_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DIC_FILES {
    pub number: libc::c_int,
    pub now: libc::c_int,
    pub dic: [*mut FILE; 5],
    pub tree_top: [pat_node; 5],
}
pub type DIC_FILES = _DIC_FILES;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cost_omomi {
    pub rensetsu: libc::c_int,
    pub keitaiso: libc::c_int,
    pub cost_haba: libc::c_int,
}
pub type COST_OMOMI = _cost_omomi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MRPH_PATTERN {
    pub regex: [libc::c_char; 64],
    pub preg: regex_t,
    pub weight: libc::c_double,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub static mut dakuon: [*mut libc::c_uchar; 51] =
    [b"\xe3\x81\x8c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xae\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x90\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x92\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb2\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x94\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb4\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x96\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb6\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x98\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb8\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xba\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xbc\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xbe\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x80\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa2\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x85\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x87\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x89\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x90\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x93\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb6\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x96\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x99\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbc\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x9c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x91\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb4\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x94\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x97\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xba\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x9a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbd\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x9d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut seion: [*mut libc::c_uchar; 41] =
    [b"\xe3\x81\x8b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xab\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xad\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xaf\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x91\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x93\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x95\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x97\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x99\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xb9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xbb\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xbd\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xbf\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x81\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa4\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa6\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa8\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xaf\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb2\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x92\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x95\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb8\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x98\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbb\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x9b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut lowercase: [*mut libc::c_uchar; 24] =
    [b"\xe3\x81\x81\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x83\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x85\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x87\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x89\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xb5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xa1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xa3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xa5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xa7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\xa9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xae\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\x83\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x93\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xb3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x83\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xa3\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x85\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xa5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x87\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x83\xa7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut uppercase: [*mut libc::c_uchar; 8] =
    [b"\xe3\x81\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut pre_prolonged: [*mut libc::c_uchar; 48] =
    [b"\xe3\x81\x8b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbe\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x83\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x97\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xab\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb2\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x98\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x91\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb8\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x81\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x92\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xba\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x99\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa4\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x90\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x85\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x93\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa8\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xae\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbb\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x94\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbc\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbd\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x87\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xad\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut prolonged2chr: [*mut libc::c_uchar; 48] =
    [b"\xe3\x81\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut pre_lower_start: [libc::c_int; 5] =
    [0 as libc::c_int, 14 as libc::c_int, 23 as libc::c_int,
     30 as libc::c_int, 37 as libc::c_int];
#[no_mangle]
pub static mut pre_lower_end: [libc::c_int; 5] =
    [14 as libc::c_int, 23 as libc::c_int, 30 as libc::c_int,
     37 as libc::c_int, 45 as libc::c_int];
#[no_mangle]
pub static mut pre_lower: [*mut libc::c_uchar; 46] =
    [b"\xe3\x81\x8b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x95\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xaa\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xaf\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xbe\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x89\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x96\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb0\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb1\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x97\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xab\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8a\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x98\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xad\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x8f\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x99\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xb5\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x80\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x91\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9b\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa6\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x81\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9c\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa7\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x93\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xae\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x82\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x88\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x82\x8d\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\x9e\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\xe3\x81\xa9\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_uchar,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar];
#[no_mangle]
pub static mut m_pattern: *mut MRPH_PATTERN =
    0 as *const MRPH_PATTERN as *mut MRPH_PATTERN;
#[no_mangle]
pub static mut mrph_pattern: [*mut libc::c_char; 8] =
    [b"\xef\xbc\xa8\xe3\x81\xa3\xef\xbc\xa8\xe3\x82\x8a    30\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xa8\xe3\x81\xa3\xef\xbc\xa8\xef\xbc\xb9\xe3\x82\x8a  30\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xab\xe3\x83\x83\xef\xbc\xab\xe3\x83\xaa    30\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xab\xe3\x83\x83\xef\xbc\xab\xef\xbc\xb9\xe3\x83\xaa  30\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xa8\xef\xbc\xa8\xe3\x81\xa3\xe3\x81\xa8    24\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xab\xef\xbc\xab\xe3\x81\xa3\xe3\x81\xa8    20\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xef\xbc\xab\xef\xbc\xab\xe3\x83\x83\xe3\x81\xa8    20\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut ProgName: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut db: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut DicFile: DIC_FILES =
    DIC_FILES{number: 0,
              now: 0,
              dic: [0 as *const FILE as *mut FILE; 5],
              tree_top:
                  [pat_node{il:
                                pat_index_list{next:
                                                   0 as *const pat_index_list
                                                       as *mut pat_index_list,
                                               index: 0,},
                            checkbit: 0,
                            right: 0 as *const pat_node as *mut pat_node,
                            left: 0 as *const pat_node as *mut pat_node,};
                      5],};
#[no_mangle]
pub static mut cost_omomi: COST_OMOMI =
    COST_OMOMI{rensetsu: 0, keitaiso: 0, cost_haba: 0,};
#[no_mangle]
pub static mut Jumangram_Dirname: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut LineNo: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut LineNoForError: libc::c_int = 0;
#[no_mangle]
pub static mut pat_buffer: [libc::c_char; 500000] = [0; 500000];
#[no_mangle]
pub static mut Show_Opt1: libc::c_int = 0;
#[no_mangle]
pub static mut Show_Opt2: libc::c_int = 0;
#[no_mangle]
pub static mut Show_Opt_tag: [libc::c_char; 129] = [0; 129];
#[no_mangle]
pub static mut Show_Opt_jumanrc: libc::c_int = 0;
#[no_mangle]
pub static mut Show_Opt_debug: libc::c_int = 0;
#[no_mangle]
pub static mut Rendaku_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut Repetition_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut Onomatopoeia_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut LowercaseRep_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut LowercaseDel_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut LongSoundRep_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut LongSoundDel_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut UseGivenSegmentation_Opt: libc::c_int = 0;
#[no_mangle]
pub static mut String: [libc::c_uchar; 50000] = [0; 50000];
#[no_mangle]
pub static mut NormalizedString: [libc::c_uchar; 50000] = [0; 50000];
#[no_mangle]
pub static mut String2Length: [libc::c_int; 50000] = [0; 50000];
#[no_mangle]
pub static mut CharLatticeUsedFlag: libc::c_int = 0;
#[no_mangle]
pub static mut CharLattice: [CHAR_NODE; 16000] =
    [CHAR_NODE{next: 0 as *const _char_node as *mut _char_node,
               chr: [0; 7],
               type_0: 0,
               da_node_pos: [0; 10],
               node_type: [0; 10],
               deleted_bytes: [0; 10],
               p_buffer: [0 as *const libc::c_char as *mut libc::c_char; 10],
               da_node_pos_num: 0,}; 16000];
#[no_mangle]
pub static mut CharRootNode: CHAR_NODE =
    CHAR_NODE{next: 0 as *const _char_node as *mut _char_node,
              chr: [0; 7],
              type_0: 0,
              da_node_pos: [0; 10],
              node_type: [0; 10],
              deleted_bytes: [0; 10],
              p_buffer: [0 as *const libc::c_char as *mut libc::c_char; 10],
              da_node_pos_num: 0,};
#[no_mangle]
pub static mut CharNum: size_t = 0;
#[no_mangle]
pub static mut MostDistantPosition: libc::c_int = 0;
#[no_mangle]
pub static mut Unkword_Pat_Num: libc::c_int = 0;
#[no_mangle]
pub static mut pre_m_buffer_num: libc::c_int = 0;
#[no_mangle]
pub static mut m_buffer_num: libc::c_int = 0;
#[no_mangle]
pub static mut Jiritsu_buffer: [libc::c_int; 129] = [0; 129];
#[no_mangle]
pub static mut undef_hinsi: libc::c_int = 0;
#[no_mangle]
pub static mut undef_kata_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut undef_alph_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut undef_etc_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut undef_kata_con_tbl: libc::c_int = 0;
#[no_mangle]
pub static mut undef_alph_con_tbl: libc::c_int = 0;
#[no_mangle]
pub static mut undef_etc_con_tbl: libc::c_int = 0;
#[no_mangle]
pub static mut suusi_hinsi: libc::c_int = 0;
#[no_mangle]
pub static mut suusi_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut kakko_hinsi: libc::c_int = 0;
#[no_mangle]
pub static mut kakko_bunrui1: libc::c_int = 0;
#[no_mangle]
pub static mut kakko_bunrui2: libc::c_int = 0;
#[no_mangle]
pub static mut kuuhaku_hinsi: libc::c_int = 0;
#[no_mangle]
pub static mut kuuhaku_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut kuuhaku_con_tbl: libc::c_int = 0;
#[no_mangle]
pub static mut onomatopoeia_hinsi: libc::c_int = 0;
#[no_mangle]
pub static mut onomatopoeia_bunrui: libc::c_int = 0;
#[no_mangle]
pub static mut onomatopoeia_con_tbl: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_hinsi1: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_hinsi2: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_hinsi3: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_hinsi4: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_renyou: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui2_1: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui2_2: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui2_3: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui4_1: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui4_2: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui4_3: libc::c_int = 0;
#[no_mangle]
pub static mut rendaku_bunrui4_4: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_interjection: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_copula: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_hinsi1: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_hinsi2: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_hinsi3: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_hinsi4: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_bunrui4_1: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_bunrui4_2: libc::c_int = 0;
#[no_mangle]
pub static mut prolong_ng_bunrui4_3: libc::c_int = 0;
#[no_mangle]
pub static mut jiritsu_num: libc::c_int = 0;
#[no_mangle]
pub static mut p_buffer_num: libc::c_int = 0;
#[no_mangle]
pub static mut connect_cache: [CONNECT_COST; 1000] =
    [CONNECT_COST{p_no: 0, pos: 0, cost: 0, opt: 0,}; 1000];
#[no_mangle]
pub static mut mrph_buffer_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut m_buffer: *mut MRPH = 0 as *const MRPH as *mut MRPH;
#[no_mangle]
pub static mut m_check_buffer: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut process_buffer_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut p_buffer: *mut PROCESS_BUFFER =
    0 as *const PROCESS_BUFFER as *mut PROCESS_BUFFER;
#[no_mangle]
pub static mut path_buffer: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut match_pbuf: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut kigou: [libc::c_uchar; 129] = [0; 129];
#[no_mangle]
pub static mut midasi1: [libc::c_uchar; 50000] = [0; 50000];
#[no_mangle]
pub static mut midasi2: [libc::c_uchar; 129] = [0; 129];
#[no_mangle]
pub static mut yomi: [libc::c_uchar; 129] = [0; 129];
#[no_mangle]
pub unsafe extern "C" fn changeDictionary(mut number: libc::c_int) {
    db = DicFile.dic[number as usize];
    DicFile.now = number;
}
#[no_mangle]
pub unsafe extern "C" fn push_dic_file_for_win(mut dic_file_name:
                                                   *mut libc::c_char,
                                               mut num: libc::c_int)
 -> libc::c_int {
    let mut full_file_name: [libc::c_char; 1025] = [0; 1025];
    if *dic_file_name.offset(strlen(dic_file_name).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                 as isize) as libc::c_int != '\\' as i32 {
        strcat(dic_file_name, b"\\\x00" as *const u8 as *const libc::c_char);
    }
    sprintf(full_file_name.as_mut_ptr(),
            b"%s%s\x00" as *const u8 as *const libc::c_char, dic_file_name,
            b"jumandic.pat\x00" as *const u8 as *const libc::c_char);
    strcat(dic_file_name,
           b"jumandic.dat\x00" as *const u8 as *const libc::c_char);
    DicFile.dic[num as usize] =
        my_fopen(dic_file_name,
                 b"rb\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    pat_init_tree_top(&mut *DicFile.tree_top.as_mut_ptr().offset(num as
                                                                     isize));
    com_l(full_file_name.as_mut_ptr(),
          &mut *DicFile.tree_top.as_mut_ptr().offset(num as isize));
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn juman_init_rc(mut fp: *mut FILE) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut win32_decided: libc::c_int = 0 as libc::c_int;
    let mut dic_file_name: [libc::c_char; 1025] = [0; 1025];
    let mut full_file_name: [libc::c_char; 1025] = [0; 1025];
    let mut cell1: *mut CELL = 0 as *mut CELL;
    let mut cell2: *mut CELL = 0 as *mut CELL;
    LineNo = 0 as libc::c_int;
    cost_omomi.keitaiso = 1 as libc::c_int;
    cost_omomi.rensetsu = 100 as libc::c_int * 10 as libc::c_int;
    cost_omomi.cost_haba = 20 as libc::c_int * 10 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        cell1 = s_read(fp);
        if win32_decided == 0 &&
               strcmp(b"\xe6\x96\x87\xe6\xb3\x95\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                          as *const u8 as *const libc::c_char,
                      (*car(cell1)).value.atom as *const libc::c_char) == 0 {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                     {
                         cell2 = car(cdr(cell1));
                         ((*cell2).tag) == 1 as libc::c_int
                     }) {
                return 0 as libc::c_int
            } else {
                strcpy(Jumangram_Dirname.as_mut_ptr(),
                       (*cell2).value.atom as *const libc::c_char);
                grammar(0 as *mut FILE);
                katuyou(0 as *mut FILE);
                connect_table(0 as *mut FILE);
                connect_matrix(0 as *mut FILE);
            }
        } else if win32_decided == 0 &&
                      strcmp(b"\xe8\xbe\x9e\xe6\x9b\xb8\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                                 as *const u8 as *const libc::c_char,
                             (*car(cell1)).value.atom as *const libc::c_char)
                          == 0 {
            cell2 = cdr(cell1);
            num = 0 as libc::c_int;
            while !car(cell2).is_null() {
                if !(!car(cell2).is_null() &&
                         (*car(cell2)).tag == 1 as libc::c_int) {
                    return 0 as libc::c_int
                } else {
                    if num >= 5 as libc::c_int {
                        error(ConfigError as libc::c_int,
                              b"Too many dictionary files.\x00" as *const u8
                                  as *const libc::c_char as *mut libc::c_char,
                              -(1 as libc::c_int) as *mut libc::c_char);
                    } else {
                        strcpy(dic_file_name.as_mut_ptr(),
                               (*car(cell2)).value.atom as
                                   *const libc::c_char);
                        if dic_file_name[strlen(dic_file_name.as_mut_ptr()).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                             as usize] as libc::c_int !=
                               '/' as i32 {
                            strcat(dic_file_name.as_mut_ptr(),
                                   b"/\x00" as *const u8 as
                                       *const libc::c_char);
                        }
                        cell2 = cdr(cell2);
                        push_darts_file(dic_file_name.as_mut_ptr());
                        sprintf(full_file_name.as_mut_ptr(),
                                b"%s%s\x00" as *const u8 as
                                    *const libc::c_char,
                                dic_file_name.as_mut_ptr(),
                                b"jumandic.pat\x00" as *const u8 as
                                    *const libc::c_char);
                        strcat(dic_file_name.as_mut_ptr(),
                               b"jumandic.dat\x00" as *const u8 as
                                   *const libc::c_char);
                        DicFile.dic[num as usize] =
                            my_fopen(dic_file_name.as_mut_ptr(),
                                     b"r\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char);
                        if check_filesize(DicFile.dic[num as usize]) == 0 as libc::c_int {
                            num -= 1
                        }
                    }
                    num += 1
                }
            }
            DicFile.number = num;
            changeDictionary(0 as libc::c_int);
        } else if strcmp(b"\xe9\x80\xa3\xe6\x8e\xa5\xe3\x82\xb3\xe3\x82\xb9\xe3\x83\x88\xe9\x87\x8d\xe3\x81\xbf\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
         {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                     {
                         cell2 = car(cdr(cell1));
                         ((*cell2).tag) == 1 as libc::c_int
                     }) {
                return 0 as libc::c_int
            } else {
                cost_omomi.rensetsu =
                    atoi((*cell2).value.atom as *const libc::c_char) *
                        10 as libc::c_int
            }
        } else if strcmp(b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe3\x82\xb3\xe3\x82\xb9\xe3\x83\x88\xe9\x87\x8d\xe3\x81\xbf\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
         {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                     {
                         cell2 = car(cdr(cell1));
                         ((*cell2).tag) == 1 as libc::c_int
                     }) {
                return 0 as libc::c_int
            } else {
                cost_omomi.keitaiso =
                    atoi((*cell2).value.atom as *const libc::c_char)
            }
        } else if strcmp(b"\xe3\x82\xb3\xe3\x82\xb9\xe3\x83\x88\xe5\xb9\x85\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
         {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                     {
                         cell2 = car(cdr(cell1));
                         ((*cell2).tag) == 1 as libc::c_int
                     }) {
                return 0 as libc::c_int
            } else {
                cost_omomi.cost_haba =
                    atoi((*cell2).value.atom as *const libc::c_char) *
                        10 as libc::c_int
            }
        } else if strcmp(b"\xe5\x93\x81\xe8\xa9\x9e\xe3\x82\xb3\xe3\x82\xb9\xe3\x83\x88\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
         {
            read_class_cost(cdr(cell1));
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compile_unkword_patterns() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *mrph_pattern[i as usize] != 0 { i += 1 }
    m_pattern =
        malloc((::std::mem::size_of::<MRPH_PATTERN>() as
                    libc::c_ulong).wrapping_mul(i as libc::c_ulong)) as
            *mut MRPH_PATTERN;
    i = 0 as libc::c_int;
    while *mrph_pattern[i as usize] != 0 {
        flag = 0 as libc::c_int;
        (*m_pattern.offset(i as isize)).weight =
            10 as libc::c_int as libc::c_double;
        sprintf((*m_pattern.offset(i as isize)).regex.as_mut_ptr(),
                b"^\x00" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while *mrph_pattern[i as usize].offset(j as isize) != 0 {
            if *mrph_pattern[i as
                                 usize].offset(j as
                                                   isize).offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                   as libc::c_int == ' ' as i32 ||
                   *mrph_pattern[i as
                                     usize].offset(j as
                                                       isize).offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                       as libc::c_int == '\t' as i32 {
                flag = 1 as libc::c_int;
                j -= 1 as libc::c_int
            } else if flag != 0 {
                (*m_pattern.offset(i as isize)).weight =
                    atof(mrph_pattern[i as usize].offset(j as isize));
                break ;
            } else {
                if strlen((*m_pattern.offset(i as isize)).regex.as_mut_ptr())
                       >=
                       (64 as libc::c_int - 3 as libc::c_int) as libc::c_ulong
                   {
                    printf(b"too long pattern: \"%s\"\n\x00" as *const u8 as
                               *const libc::c_char, mrph_pattern[i as usize]);
                    exit(1 as libc::c_int);
                }
                if strncmp(mrph_pattern[i as usize].offset(j as isize),
                           b"\xef\xbc\xa8\x00" as *const u8 as
                               *const libc::c_char,
                           3 as libc::c_int as libc::c_ulong) == 0 {
                    strcat((*m_pattern.offset(i as isize)).regex.as_mut_ptr(),
                           b"\xe3(\x81[\x82-\xbf]|\x82[\x80-\x8f])\x00" as
                               *const u8 as *const libc::c_char);
                } else if strncmp(mrph_pattern[i as usize].offset(j as isize),
                                  b"\xef\xbc\xab\x00" as *const u8 as
                                      *const libc::c_char,
                                  3 as libc::c_int as libc::c_ulong) == 0 {
                    strcat((*m_pattern.offset(i as isize)).regex.as_mut_ptr(),
                           b"\xe3(\x82[\xa0-\xbf]|\x83[\x80-\xba])\x00" as
                               *const u8 as *const libc::c_char);
                } else if strncmp(mrph_pattern[i as usize].offset(j as isize),
                                  b"\xef\xbc\xb9\x00" as *const u8 as
                                      *const libc::c_char,
                                  3 as libc::c_int as libc::c_ulong) == 0 {
                    strcat((*m_pattern.offset(i as isize)).regex.as_mut_ptr(),
                           b"\xe3(\x82[\x83\x85\x87]|\x83[\xa3\xa5\xa7])\x00"
                               as *const u8 as *const libc::c_char);
                } else {
                    strncat((*m_pattern.offset(i as
                                                   isize)).regex.as_mut_ptr(),
                            mrph_pattern[i as usize].offset(j as isize),
                            3 as libc::c_int as libc::c_ulong);
                }
            }
            j += 3 as libc::c_int
        }
        if regcomp(&mut (*m_pattern.offset(i as isize)).preg,
                   (*m_pattern.offset(i as isize)).regex.as_mut_ptr(),
                   1 as libc::c_int) != 0 as libc::c_int {
            printf(b"regex compile failed\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        i += 1
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn juman_close() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < DicFile.number { fclose(DicFile.dic[i as usize]); i += 1 }
    close_darts();
    free(rensetu_tbl as *mut libc::c_void);
    free(rensetu_mtr as *mut libc::c_void);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn realloc_mrph_buffer() {
    mrph_buffer_max += 1000 as libc::c_int;
    m_buffer =
        my_realloc(m_buffer as *mut libc::c_void,
                   (::std::mem::size_of::<MRPH>() as
                        libc::c_ulong).wrapping_mul(mrph_buffer_max as
                                                        libc::c_ulong) as
                       libc::c_int) as *mut MRPH;
    m_check_buffer =
        my_realloc(m_check_buffer as *mut libc::c_void,
                   (::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(mrph_buffer_max as
                                                        libc::c_ulong) as
                       libc::c_int) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn realloc_process_buffer() {
    process_buffer_max += 1000 as libc::c_int;
    p_buffer =
        my_realloc(p_buffer as *mut libc::c_void,
                   (::std::mem::size_of::<PROCESS_BUFFER>() as
                        libc::c_ulong).wrapping_mul(process_buffer_max as
                                                        libc::c_ulong) as
                       libc::c_int) as *mut PROCESS_BUFFER;
    path_buffer =
        my_realloc(path_buffer as *mut libc::c_void,
                   (::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(process_buffer_max as
                                                        libc::c_ulong) as
                       libc::c_int) as *mut libc::c_int;
    match_pbuf =
        my_realloc(match_pbuf as *mut libc::c_void,
                   (::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(process_buffer_max as
                                                        libc::c_ulong) as
                       libc::c_int) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_class_cost(mut cell: *mut CELL) {
    let mut pos_cell: *mut CELL = 0 as *mut CELL;
    let mut hinsi: libc::c_int = 0;
    let mut bunrui: libc::c_int = 0;
    let mut cost: libc::c_int = 0;
    while !car(cell).is_null() {
        pos_cell = car(car(cell));
        cost = atoi((*car(cdr(car(cell)))).value.atom as *const libc::c_char);
        if strcmp((*car(pos_cell)).value.atom as *const libc::c_char,
                  b"*\x00" as *const u8 as *const libc::c_char) == 0 {
            hinsi = 1 as libc::c_int;
            while !Class[hinsi as
                             usize][0 as libc::c_int as usize].id.is_null() {
                bunrui = 0 as libc::c_int;
                while !Class[hinsi as usize][bunrui as usize].id.is_null() {
                    Class[hinsi as usize][bunrui as usize].cost = cost;
                    bunrui += 1
                }
                hinsi += 1
            }
        } else {
            hinsi = get_hinsi_id((*car(pos_cell)).value.atom);
            if car(cdr(pos_cell)).is_null() ||
                   strcmp((*car(cdr(pos_cell))).value.atom as
                              *const libc::c_char,
                          b"*\x00" as *const u8 as *const libc::c_char) == 0 {
                bunrui = 0 as libc::c_int;
                while !Class[hinsi as usize][bunrui as usize].id.is_null() {
                    Class[hinsi as usize][bunrui as usize].cost = cost;
                    bunrui += 1
                }
            } else {
                bunrui =
                    get_bunrui_id((*car(cdr(pos_cell))).value.atom, hinsi);
                Class[hinsi as usize][bunrui as usize].cost = cost
            }
        }
        cell = cdr(cell)
    }
    hinsi = 1 as libc::c_int;
    while !Class[hinsi as usize][0 as libc::c_int as usize].id.is_null() {
        bunrui = 0 as libc::c_int;
        while !Class[hinsi as usize][bunrui as usize].id.is_null() {
            if Class[hinsi as usize][bunrui as usize].cost == 0 as libc::c_int
               {
                Class[hinsi as usize][bunrui as usize].cost =
                    10 as libc::c_int
            }
            bunrui += 1
        }
        hinsi += 1
    }
    Class[0 as libc::c_int as usize][0 as libc::c_int as usize].cost =
        0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn register_nodes_by_deletion(mut src_buf:
                                                        *mut libc::c_char,
                                                    mut pat_buf:
                                                        *mut libc::c_char,
                                                    mut node_type:
                                                        libc::c_char,
                                                    mut deleted_bytes:
                                                        libc::c_char) {
    let mut length: libc::c_int = 0;
    let mut start_buf: *mut libc::c_char = src_buf;
    let mut current_pat_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    deleted_bytes += 1;
    while *src_buf != 0 {
        if *src_buf as libc::c_int == '\n' as i32 {
            current_pat_buf = pat_buf.offset(strlen(pat_buf) as isize);
            length =
                (src_buf.wrapping_offset_from(start_buf) as libc::c_long +
                     1 as libc::c_int as libc::c_long) as libc::c_int;
            strncat(pat_buf, start_buf, length as libc::c_ulong);
            *current_pat_buf =
                (node_type as libc::c_int + 11 as libc::c_int) as
                    libc::c_char;
            *current_pat_buf.offset(1 as libc::c_int as isize) =
                (deleted_bytes as libc::c_int + 11 as libc::c_int) as
                    libc::c_char;
            *current_pat_buf.offset(length as isize) =
                '\u{0}' as i32 as libc::c_char;
            start_buf = src_buf.offset(1 as libc::c_int as isize)
        }
        src_buf = src_buf.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn da_search_one_step(mut dic_no: libc::c_int,
                                            mut left_position: libc::c_int,
                                            mut right_position: libc::c_int,
                                            mut pat_buf: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut current_da_node_pos: size_t = 0;
    let mut left_char_node: *mut CHAR_NODE = 0 as *mut CHAR_NODE;
    let mut right_char_node: *mut CHAR_NODE = 0 as *mut CHAR_NODE;
    let mut current_pat_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_node_type: libc::c_char = 0;
    if left_position < 0 as libc::c_int {
        left_char_node = &mut CharRootNode
    } else {
        left_char_node =
            &mut *CharLattice.as_mut_ptr().offset(left_position as isize) as
                *mut CHAR_NODE
    }
    while !left_char_node.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*left_char_node).da_node_pos_num {
            right_char_node =
                &mut *CharLattice.as_mut_ptr().offset(right_position as isize)
                    as *mut CHAR_NODE;
            while !right_char_node.is_null() {
                if (*right_char_node).chr[0 as libc::c_int as usize] as
                       libc::c_int == '\u{0}' as i32 {
                    if left_position >= 0 as libc::c_int {
                        (*right_char_node).node_type[(*right_char_node).da_node_pos_num
                                                         as usize] =
                            ((*left_char_node).node_type[i as usize] as
                                 libc::c_int |
                                 (*right_char_node).type_0 as libc::c_int) as
                                libc::c_char;
                        (*right_char_node).deleted_bytes[(*right_char_node).da_node_pos_num
                                                             as usize] =
                            ((*left_char_node).deleted_bytes[i as usize] as
                                 libc::c_ulong).wrapping_add(strlen(CharLattice[right_position
                                                                                    as
                                                                                    usize].chr.as_mut_ptr()))
                                as libc::c_char;
                        if !(*left_char_node).p_buffer[i as usize].is_null() {
                            if (*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                               as
                                                               usize].is_null()
                               {
                                (*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                                as usize] =
                                    malloc(50000 as libc::c_int as
                                               libc::c_ulong) as
                                        *mut libc::c_char;
                                *(*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                                 as
                                                                 usize].offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                    = '\u{0}' as i32 as libc::c_char
                            }
                            strcat((*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                                   as usize],
                                   (*left_char_node).p_buffer[i as usize]);
                            register_nodes_by_deletion((*left_char_node).p_buffer[i
                                                                                      as
                                                                                      usize],
                                                       pat_buf,
                                                       ((*right_char_node).node_type[(*right_char_node).da_node_pos_num
                                                                                         as
                                                                                         usize]
                                                            as libc::c_int |
                                                            32 as libc::c_int)
                                                           as libc::c_char,
                                                       (*right_char_node).deleted_bytes[(*right_char_node).da_node_pos_num
                                                                                            as
                                                                                            usize]);
                        }
                        let fresh0 = (*right_char_node).da_node_pos_num;
                        (*right_char_node).da_node_pos_num =
                            (*right_char_node).da_node_pos_num.wrapping_add(1);
                        (*right_char_node).da_node_pos[fresh0 as usize] =
                            (*left_char_node).da_node_pos[i as usize];
                        if MostDistantPosition < right_position {
                            MostDistantPosition = right_position
                        }
                    }
                } else if !((*right_char_node).type_0 as libc::c_int &
                                4 as libc::c_int != 0 ||
                                (*right_char_node).type_0 as libc::c_int &
                                    16 as libc::c_int != 0) ||
                              (*right_char_node).type_0 as libc::c_int &
                                  4 as libc::c_int != 0 &&
                                  left_position < 0 as libc::c_int ||
                              (*right_char_node).type_0 as libc::c_int &
                                  16 as libc::c_int != 0 &&
                                  left_position >= 0 as libc::c_int {
                    current_node_type =
                        ((*left_char_node).node_type[i as usize] as
                             libc::c_int |
                             (*right_char_node).type_0 as libc::c_int) as
                            libc::c_char;
                    current_da_node_pos =
                        (*left_char_node).da_node_pos[i as usize];
                    current_pat_buf =
                        pat_buf.offset(strlen(pat_buf) as isize);
                    status =
                        da_traverse(dic_no,
                                    (*right_char_node).chr.as_mut_ptr(),
                                    &mut current_da_node_pos,
                                    0 as libc::c_int as size_t,
                                    strlen((*right_char_node).chr.as_mut_ptr()),
                                    current_node_type,
                                    (*left_char_node).deleted_bytes[i as
                                                                        usize],
                                    pat_buf);
                    if status > 0 as libc::c_int {
                        if (*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                           as usize].is_null()
                           {
                            (*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                            as usize] =
                                malloc(50000 as libc::c_int as libc::c_ulong)
                                    as *mut libc::c_char;
                            *(*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                             as
                                                             usize].offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                = '\u{0}' as i32 as libc::c_char
                        }
                        strcat((*right_char_node).p_buffer[(*right_char_node).da_node_pos_num
                                                               as usize],
                               current_pat_buf);
                    } else { (status) == -(1 as libc::c_int); }
                    if status > 0 as libc::c_int ||
                           status == -(1 as libc::c_int) {
                        (*right_char_node).node_type[(*right_char_node).da_node_pos_num
                                                         as usize] =
                            current_node_type;
                        (*right_char_node).deleted_bytes[(*right_char_node).da_node_pos_num
                                                             as usize] =
                            (*left_char_node).deleted_bytes[i as usize];
                        let fresh1 = (*right_char_node).da_node_pos_num;
                        (*right_char_node).da_node_pos_num =
                            (*right_char_node).da_node_pos_num.wrapping_add(1);
                        (*right_char_node).da_node_pos[fresh1 as usize] =
                            current_da_node_pos;
                        if MostDistantPosition < right_position {
                            MostDistantPosition = right_position
                        }
                    }
                }
                if (*right_char_node).da_node_pos_num >=
                       10 as libc::c_int as libc::c_ulong {
                    (*right_char_node).da_node_pos_num =
                        (10 as libc::c_int - 1 as libc::c_int) as size_t
                }
                right_char_node = (*right_char_node).next
            }
            i += 1
        }
        left_char_node = (*left_char_node).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn da_search_from_position(mut dic_no: libc::c_int,
                                                 mut position: libc::c_int,
                                                 mut pat_buf:
                                                     *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut char_node: *mut CHAR_NODE = 0 as *mut CHAR_NODE;
    MostDistantPosition = position - 1 as libc::c_int;
    i = position;
    while (i as libc::c_ulong) < CharNum {
        char_node =
            &mut *CharLattice.as_mut_ptr().offset(i as isize) as
                *mut CHAR_NODE;
        while !char_node.is_null() {
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < (*char_node).da_node_pos_num {
                if !(*char_node).p_buffer[j as usize].is_null() {
                    free((*char_node).p_buffer[j as usize] as
                             *mut libc::c_void);
                }
                (*char_node).p_buffer[j as usize] = 0 as *mut libc::c_char;
                j += 1
            }
            (*char_node).da_node_pos_num = 0 as libc::c_int as size_t;
            char_node = (*char_node).next
        }
        i += 1
    }
    da_search_one_step(dic_no, -(1 as libc::c_int), position, pat_buf);
    i = position + 1 as libc::c_int;
    while (i as libc::c_ulong) < CharNum {
        if MostDistantPosition < i - 1 as libc::c_int { break ; }
        da_search_one_step(dic_no, i - 1 as libc::c_int, i, pat_buf);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn search_all(mut position: libc::c_int,
                                    mut position_in_char: libc::c_int)
 -> libc::c_int {
    let mut dic_no: libc::c_int = 0;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    dic_no = 0 as libc::c_int;
    while dic_no < DicFile.number {
        changeDictionary(dic_no);
        pat_buffer[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        if CharLatticeUsedFlag != 0 {
            da_search_from_position(dic_no, position_in_char,
                                    pat_buffer.as_mut_ptr());
        } else {
            da_search(dic_no,
                      String.as_mut_ptr().offset(position as isize) as
                          *mut libc::c_char, pat_buffer.as_mut_ptr());
        }
        pbuf = pat_buffer.as_mut_ptr();
        while *pbuf as libc::c_int != '\u{0}' as i32 {
            if take_data(position, position_in_char, &mut pbuf,
                         0 as libc::c_int as libc::c_char) == 0 as libc::c_int
               {
                return 0 as libc::c_int
            }
        }
        dic_no += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn recognize_onomatopoeia(mut pos: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut next_code: libc::c_int = 0;
    let mut key_length: libc::c_int =
        strlen(String.as_mut_ptr().offset(pos as isize) as
                   *const libc::c_char) as libc::c_int;
    let mut pmatch: [regmatch_t; 1] = [regmatch_t{rm_so: 0, rm_eo: 0,}; 1];
    code = check_code(String.as_mut_ptr(), pos);
    if code != 0xa5a0 as libc::c_int && code != 0xa6a0 as libc::c_int {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while *lowercase[i as usize] != 0 {
        if strncmp(String.as_mut_ptr().offset(pos as isize) as
                       *const libc::c_char,
                   lowercase[i as usize] as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            return 0 as libc::c_int
        }
        i += 1
    }
    if Onomatopoeia_Opt != 0 &&
           code == check_code(String.as_mut_ptr(), pos + 3 as libc::c_int) {
        i = 0 as libc::c_int;
        while i < Unkword_Pat_Num {
            if regexec(&mut (*m_pattern.offset(i as isize)).preg,
                       String.as_mut_ptr().offset(pos as isize) as
                           *const libc::c_char, 1 as libc::c_int as size_t,
                       pmatch.as_mut_ptr(), 0 as libc::c_int) ==
                   0 as libc::c_int {
                (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                    onomatopoeia_hinsi as libc::c_char;
                (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                    onomatopoeia_bunrui as libc::c_char;
                (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                    onomatopoeia_con_tbl;
                (*m_buffer.offset(m_buffer_num as isize)).katuyou1 =
                    0 as libc::c_int as libc::c_char;
                (*m_buffer.offset(m_buffer_num as isize)).katuyou2 =
                    0 as libc::c_int as libc::c_char;
                (*m_buffer.offset(m_buffer_num as isize)).length =
                    pmatch[0 as libc::c_int as usize].rm_eo -
                        pmatch[0 as libc::c_int as usize].rm_so;
                strncpy((*m_buffer.offset(m_buffer_num as
                                              isize)).midasi.as_mut_ptr() as
                            *mut libc::c_char,
                        String.as_mut_ptr().offset(pos as isize) as
                            *const libc::c_char,
                        (*m_buffer.offset(m_buffer_num as isize)).length as
                            libc::c_ulong);
                (*m_buffer.offset(m_buffer_num as
                                      isize)).midasi[(*m_buffer.offset(m_buffer_num
                                                                           as
                                                                           isize)).length
                                                         as usize] =
                    '\u{0}' as i32 as libc::c_uchar;
                strncpy((*m_buffer.offset(m_buffer_num as
                                              isize)).yomi.as_mut_ptr() as
                            *mut libc::c_char,
                        String.as_mut_ptr().offset(pos as isize) as
                            *const libc::c_char,
                        (*m_buffer.offset(m_buffer_num as isize)).length as
                            libc::c_ulong);
                (*m_buffer.offset(m_buffer_num as
                                      isize)).yomi[(*m_buffer.offset(m_buffer_num
                                                                         as
                                                                         isize)).length
                                                       as usize] =
                    '\u{0}' as i32 as libc::c_uchar;
                (*m_buffer.offset(m_buffer_num as isize)).weight =
                    (*m_pattern.offset(i as isize)).weight as libc::c_uchar;
                strcpy((*m_buffer.offset(m_buffer_num as
                                             isize)).midasi2.as_mut_ptr() as
                           *mut libc::c_char,
                       (*m_buffer.offset(m_buffer_num as
                                             isize)).midasi.as_mut_ptr() as
                           *const libc::c_char);
                strcpy((*m_buffer.offset(m_buffer_num as
                                             isize)).imis.as_mut_ptr() as
                           *mut libc::c_char,
                       b"\"\x00" as *const u8 as *const libc::c_char);
                strcat((*m_buffer.offset(m_buffer_num as
                                             isize)).imis.as_mut_ptr() as
                           *mut libc::c_char,
                       b"\xe8\x87\xaa\xe5\x8b\x95\xe8\xaa\x8d\xe8\xad\x98 \xe3\x82\xb9\xe3\x83\xab\xe6\x8e\xa5\xe7\xb6\x9a\xe5\x8f\xaf\xe8\x83\xbd\x00"
                           as *const u8 as *const libc::c_char);
                strcat((*m_buffer.offset(m_buffer_num as
                                             isize)).imis.as_mut_ptr() as
                           *mut libc::c_char,
                       b"\"\x00" as *const u8 as *const libc::c_char);
                check_connect(pos, m_buffer_num,
                              0 as libc::c_int as libc::c_char);
                m_buffer_num += 1;
                if m_buffer_num == mrph_buffer_max { realloc_mrph_buffer(); }
                break ;
            } else { i += 1 }
        }
    }
    if Repetition_Opt != 0 {
        let mut current_block_64: u64;
        len = 2 as libc::c_int;
        while len < 5 as libc::c_int {
            next_code =
                check_code(String.as_mut_ptr(),
                           pos + len * 3 as libc::c_int - 3 as libc::c_int);
            if next_code == 0xa1bc as libc::c_int { next_code = code }
            if key_length < len * 2 as libc::c_int * 3 as libc::c_int ||
                   code != next_code {
                break ;
            }
            code = next_code;
            if !(strncmp(String.as_mut_ptr().offset(pos as isize) as
                             *const libc::c_char,
                         String.as_mut_ptr().offset(pos as
                                                        isize).offset((len *
                                                                           3
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          isize)
                             as *const libc::c_char,
                         (len * 3 as libc::c_int) as libc::c_ulong) != 0) {
                if !(strncmp(String.as_mut_ptr().offset(pos as isize) as
                                 *const libc::c_char,
                             String.as_mut_ptr().offset(pos as
                                                            isize).offset(3 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                                 as *const libc::c_char,
                             3 as libc::c_int as libc::c_ulong) == 0 &&
                         strncmp(String.as_mut_ptr().offset(pos as isize) as
                                     *const libc::c_char,
                                 String.as_mut_ptr().offset(pos as
                                                                isize).offset((2
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   3
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize)
                                     as *const libc::c_char,
                                 3 as libc::c_int as libc::c_ulong) == 0) {
                    (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                        onomatopoeia_hinsi as libc::c_char;
                    (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                        onomatopoeia_bunrui as libc::c_char;
                    (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                        onomatopoeia_con_tbl;
                    (*m_buffer.offset(m_buffer_num as isize)).katuyou1 =
                        0 as libc::c_int as libc::c_char;
                    (*m_buffer.offset(m_buffer_num as isize)).katuyou2 =
                        0 as libc::c_int as libc::c_char;
                    (*m_buffer.offset(m_buffer_num as isize)).length =
                        len * 2 as libc::c_int * 3 as libc::c_int;
                    strncpy((*m_buffer.offset(m_buffer_num as
                                                  isize)).midasi.as_mut_ptr()
                                as *mut libc::c_char,
                            String.as_mut_ptr().offset(pos as isize) as
                                *const libc::c_char,
                            (len * 2 as libc::c_int * 3 as libc::c_int) as
                                libc::c_ulong);
                    (*m_buffer.offset(m_buffer_num as
                                          isize)).midasi[(len *
                                                              2 as libc::c_int
                                                              *
                                                              3 as
                                                                  libc::c_int)
                                                             as usize] =
                        '\u{0}' as i32 as libc::c_uchar;
                    strncpy((*m_buffer.offset(m_buffer_num as
                                                  isize)).yomi.as_mut_ptr() as
                                *mut libc::c_char,
                            String.as_mut_ptr().offset(pos as isize) as
                                *const libc::c_char,
                            (len * 2 as libc::c_int * 3 as libc::c_int) as
                                libc::c_ulong);
                    (*m_buffer.offset(m_buffer_num as
                                          isize)).yomi[(len * 2 as libc::c_int
                                                            *
                                                            3 as libc::c_int)
                                                           as usize] =
                        '\u{0}' as i32 as libc::c_uchar;
                    (*m_buffer.offset(m_buffer_num as isize)).weight =
                        (13 as libc::c_int * len) as libc::c_uchar;
                    i = 17 as libc::c_int;
                    while i < 23 as libc::c_int {
                        if !strstr((*m_buffer.offset(m_buffer_num as
                                                         isize)).midasi.as_mut_ptr()
                                       as *const libc::c_char,
                                   lowercase[i as usize] as
                                       *const libc::c_char).is_null() {
                            break ;
                        }
                        i += 1
                    }
                    if i < 23 as libc::c_int {
                        if len == 2 as libc::c_int {
                            current_block_64 = 2891135413264362348;
                        } else {
                            let ref mut fresh2 =
                                (*m_buffer.offset(m_buffer_num as
                                                      isize)).weight;
                            *fresh2 =
                                (*fresh2 as libc::c_int -
                                     (13 as libc::c_int + 4 as libc::c_int))
                                    as libc::c_uchar;
                            current_block_64 = 8545136480011357681;
                        }
                    } else { current_block_64 = 8545136480011357681; }
                    match current_block_64 {
                        2891135413264362348 => { }
                        _ => {
                            i = 0 as libc::c_int;
                            while *dakuon[i as usize] != 0 {
                                if !strstr((*m_buffer.offset(m_buffer_num as
                                                                 isize)).midasi.as_mut_ptr()
                                               as *const libc::c_char,
                                           dakuon[i as usize] as
                                               *const libc::c_char).is_null()
                                   {
                                    break ;
                                }
                                i += 1
                            }
                            if *dakuon[i as usize] != 0 {
                                let ref mut fresh3 =
                                    (*m_buffer.offset(m_buffer_num as
                                                          isize)).weight;
                                *fresh3 =
                                    (*fresh3 as libc::c_int -
                                         1 as libc::c_int) as libc::c_uchar;
                                if strncmp((*m_buffer.offset(m_buffer_num as
                                                                 isize)).midasi.as_mut_ptr()
                                               as *const libc::c_char,
                                           dakuon[i as usize] as
                                               *const libc::c_char,
                                           3 as libc::c_int as libc::c_ulong)
                                       == 0 {
                                    let ref mut fresh4 =
                                        (*m_buffer.offset(m_buffer_num as
                                                              isize)).weight;
                                    *fresh4 =
                                        (*fresh4 as libc::c_int -
                                             1 as libc::c_int) as
                                            libc::c_uchar
                                }
                            }
                            if code == 0xa6a0 as libc::c_int {
                                let ref mut fresh5 =
                                    (*m_buffer.offset(m_buffer_num as
                                                          isize)).weight;
                                *fresh5 =
                                    (*fresh5 as libc::c_int -
                                         2 as libc::c_int) as libc::c_uchar
                            }
                            strcpy((*m_buffer.offset(m_buffer_num as
                                                         isize)).midasi2.as_mut_ptr()
                                       as *mut libc::c_char,
                                   (*m_buffer.offset(m_buffer_num as
                                                         isize)).midasi.as_mut_ptr()
                                       as *const libc::c_char);
                            strcpy((*m_buffer.offset(m_buffer_num as
                                                         isize)).imis.as_mut_ptr()
                                       as *mut libc::c_char,
                                   b"\"\x00" as *const u8 as
                                       *const libc::c_char);
                            strcat((*m_buffer.offset(m_buffer_num as
                                                         isize)).imis.as_mut_ptr()
                                       as *mut libc::c_char,
                                   b"\xe8\x87\xaa\xe5\x8b\x95\xe8\xaa\x8d\xe8\xad\x98 \xe3\x82\xb9\xe3\x83\xab\xe6\x8e\xa5\xe7\xb6\x9a\xe5\x8f\xaf\xe8\x83\xbd\x00"
                                       as *const u8 as *const libc::c_char);
                            strcat((*m_buffer.offset(m_buffer_num as
                                                         isize)).imis.as_mut_ptr()
                                       as *mut libc::c_char,
                                   b"\"\x00" as *const u8 as
                                       *const libc::c_char);
                            check_connect(pos, m_buffer_num,
                                          0 as libc::c_int as libc::c_char);
                            m_buffer_num += 1;
                            if m_buffer_num == mrph_buffer_max {
                                realloc_mrph_buffer();
                            }
                            break ;
                        }
                    }
                }
            }
            len += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn take_data(mut pos: libc::c_int,
                                   mut pos_in_char: libc::c_int,
                                   mut pbuf: *mut *mut libc::c_char,
                                   mut opt: libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    // let mut f: libc::c_int = 0;
    let mut component_num: libc::c_int = 0;
    let mut rengo_con_tbl: libc::c_int = 0;
    let mut rengo_weight: libc::c_int = 0;
    let mut mrph: MRPH =
        MRPH{midasi: [0; 129],
             midasi2: [0; 129],
             yomi: [0; 129],
             imis: [0; 1024],
             imi: 0 as *mut CELL,
             hinsi: 0,
             bunrui: 0,
             katuyou1: 0,
             katuyou2: 0,
             weight: 0,
             con_tbl: 0,
             length: 0,};
    let mut c_mrph_buf: [MRPH; 20] =
        [MRPH{midasi: [0; 129],
              midasi2: [0; 129],
              yomi: [0; 129],
              imis: [0; 1024],
              imi: 0 as *mut CELL,
              hinsi: 0,
              bunrui: 0,
              katuyou1: 0,
              katuyou2: 0,
              weight: 0,
              con_tbl: 0,
              length: 0,}; 20];
    let mut c_mrph_p: *mut MRPH = 0 as *mut MRPH;
    let mut con_tbl_bak: libc::c_int = 0;
    let mut pnum_bak: libc::c_int = 0;
    let mut c_weight: libc::c_int = 0;
    let mut new_mrph_num: libc::c_int = 0;
    let mut deleted_bytes: libc::c_int = 0;
    s = *pbuf as *mut libc::c_uchar;
    let fresh6 = s;
    s = s.offset(1);
    opt = (*fresh6 as libc::c_int - 11 as libc::c_int) as libc::c_char;
    let fresh7 = s;
    s = s.offset(1);
    deleted_bytes =
        *fresh7 as libc::c_int - 1 as libc::c_int - 11 as libc::c_int;
    s =
        _take_data(s as *mut libc::c_char, &mut mrph, deleted_bytes, &mut opt)
            as *mut libc::c_uchar;
    if mrph.hinsi as libc::c_int == 127 as libc::c_int {
        if Show_Opt_debug != 0 {
            printf(b"\\begin{\xe9\x80\xa3\xe8\xaa\x9e} %s(%s)\n\x00" as
                       *const u8 as *const libc::c_char,
                   mrph.midasi.as_mut_ptr(), mrph.midasi2.as_mut_ptr());
        }
        component_num = mrph.bunrui as libc::c_int;
        rengo_con_tbl = mrph.con_tbl;
        rengo_weight = mrph.weight as libc::c_int;
        i = 0 as libc::c_int;
        while i < component_num {
            s =
                _take_data(s as *mut libc::c_char,
                           &mut *c_mrph_buf.as_mut_ptr().offset(i as isize),
                           0 as libc::c_int, &mut opt) as *mut libc::c_uchar;
            if opt as libc::c_int != 1 as libc::c_int {
                c_mrph_buf[i as usize].weight =
                    255 as libc::c_int as libc::c_uchar
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while i < component_num {
            *m_buffer.offset(m_buffer_num as isize) = c_mrph_buf[i as usize];
            c_mrph_p =
                &mut *m_buffer.offset(m_buffer_num as isize) as *mut MRPH;
            if UseGivenSegmentation_Opt != 0 &&
                   (*c_mrph_p).length != String2Length[pos as usize] {
                s = s.offset(1);
                *pbuf = s as *mut libc::c_char;
                return (0 as libc::c_int == 0) as libc::c_int
            }
            if i == 0 as libc::c_int {
                con_tbl_bak = (*c_mrph_p).con_tbl;
                if rengo_con_tbl != -(1 as libc::c_int) &&
                       check_matrix_left(rengo_con_tbl) ==
                           (0 as libc::c_int == 0) as libc::c_int {
                    (*c_mrph_p).con_tbl = rengo_con_tbl
                }
                pnum_bak = p_buffer_num;
                check_connect(pos, m_buffer_num, opt);
                if p_buffer_num == pnum_bak { break ; }
                (*c_mrph_p).con_tbl = con_tbl_bak;
                (*p_buffer.offset(pnum_bak as isize)).end =
                    pos + (*c_mrph_p).length;
                (*p_buffer.offset(pnum_bak as isize)).connect =
                    0 as libc::c_int;
                (*p_buffer.offset(pnum_bak as isize)).score =
                    (*p_buffer.offset(pnum_bak as isize)).score +
                        Class[(*c_mrph_p).hinsi as
                                  usize][(*c_mrph_p).bunrui as usize].cost *
                            (*c_mrph_p).weight as libc::c_int *
                            cost_omomi.keitaiso *
                            (rengo_weight - 10 as libc::c_int) /
                            10 as libc::c_int;
                if Show_Opt_debug != 0 {
                    printf(b"----- \xe9\x80\xa3\xe8\xaa\x9e\xe5\x86\x85 %s %d\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*c_mrph_p).midasi.as_mut_ptr(),
                           (*p_buffer.offset(pnum_bak as isize)).score);
                }
            } else {
                (*p_buffer.offset(p_buffer_num as isize)).mrph_p =
                    m_buffer_num;
                (*p_buffer.offset(p_buffer_num as isize)).start = pos;
                (*p_buffer.offset(p_buffer_num as isize)).end =
                    pos + (*c_mrph_p).length;
                (*p_buffer.offset(p_buffer_num as
                                      isize)).path[0 as libc::c_int as usize]
                    = p_buffer_num - 1 as libc::c_int;
                (*p_buffer.offset(p_buffer_num as
                                      isize)).path[1 as libc::c_int as usize]
                    = -(1 as libc::c_int);
                c_weight =
                    check_matrix((*m_buffer.offset((*p_buffer.offset((p_buffer_num
                                                                          -
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)).mrph_p
                                                       as isize)).con_tbl,
                                 (*c_mrph_p).con_tbl);
                if c_weight == 0 as libc::c_int ||
                       c_weight > 10 as libc::c_int {
                    c_weight = 10 as libc::c_int
                }
                (*p_buffer.offset(p_buffer_num as isize)).score =
                    (*p_buffer.offset((p_buffer_num - 1 as libc::c_int) as
                                          isize)).score +
                        (Class[(*c_mrph_p).hinsi as
                                   usize][(*c_mrph_p).bunrui as usize].cost *
                             (*c_mrph_p).weight as libc::c_int *
                             cost_omomi.keitaiso +
                             c_weight * cost_omomi.rensetsu) * rengo_weight /
                            10 as libc::c_int;
                if i < component_num - 1 as libc::c_int {
                    (*p_buffer.offset(p_buffer_num as isize)).connect =
                        0 as libc::c_int
                } else {
                    (*p_buffer.offset(p_buffer_num as isize)).connect =
                        (0 as libc::c_int == 0) as libc::c_int;
                    if rengo_con_tbl != -(1 as libc::c_int) &&
                           check_matrix_right(rengo_con_tbl) ==
                               (0 as libc::c_int == 0) as libc::c_int {
                        (*c_mrph_p).con_tbl = rengo_con_tbl
                    }
                }
                if Show_Opt_debug != 0 {
                    printf(b"----- \xe9\x80\xa3\xe8\xaa\x9e\xe5\x86\x85 %s %d\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*c_mrph_p).midasi.as_mut_ptr(),
                           (*p_buffer.offset(p_buffer_num as isize)).score);
                }
                p_buffer_num += 1;
                if p_buffer_num == process_buffer_max {
                    realloc_process_buffer();
                }
            }
            pos += (*c_mrph_p).length;
            m_buffer_num += 1;
            if m_buffer_num == mrph_buffer_max { realloc_mrph_buffer(); }
            i += 1
        }
        if Show_Opt_debug != 0 {
            printf(b"\\end{\xe9\x80\xa3\xe8\xaa\x9e}\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    } else {
        if UseGivenSegmentation_Opt != 0 &&
               mrph.length != String2Length[pos as usize] {
            mrph.weight = 255 as libc::c_int as libc::c_uchar
        }
        if mrph.weight as libc::c_int == 255 as libc::c_int {
            s = s.offset(1);
            *pbuf = s as *mut libc::c_char;
            return (0 as libc::c_int == 0) as libc::c_int
        }
        if opt as libc::c_int & 4 as libc::c_int != 0 {
            let mut code: libc::c_int = check_code(String.as_mut_ptr(), pos);
            if pos < 3 as libc::c_int ||
                   code == 0xa6a0 as libc::c_int &&
                       (check_code(String.as_mut_ptr(),
                                   pos - 3 as libc::c_int) ==
                            0xa6a0 as libc::c_int ||
                            check_code(String.as_mut_ptr(),
                                       pos - 3 as libc::c_int) ==
                                0xa1bc as libc::c_int) {
                s = s.offset(1);
                *pbuf = s as *mut libc::c_char;
                return (0 as libc::c_int == 0) as libc::c_int
            }
        }
        if opt as libc::c_int & 2 as libc::c_int == 0 ||
               (strlen(mrph.midasi.as_mut_ptr() as *const libc::c_char) >
                    3 as libc::c_int as libc::c_ulong ||
                    strncmp(mrph.midasi.as_mut_ptr() as *const libc::c_char,
                            uppercase[6 as libc::c_int as usize] as
                                *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong) == 0) {
            *m_buffer.offset(m_buffer_num as isize) = mrph;
            check_connect(pos, m_buffer_num, opt);
            new_mrph_num = m_buffer_num;
            m_buffer_num += 1;
            if m_buffer_num == mrph_buffer_max { realloc_mrph_buffer(); }
            if suusi_word(pos, new_mrph_num) == 0 as libc::c_int {
                return 0 as libc::c_int
            }
            if through_word(pos, new_mrph_num) == 0 as libc::c_int {
                return 0 as libc::c_int
            }
        }
    }
    s = s.offset(1);
    *pbuf = s as *mut libc::c_char;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _take_data(mut s: *mut libc::c_char,
                                    mut mrph: *mut MRPH,
                                    mut deleted_bytes: libc::c_int,
                                    mut opt: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut imi_length: libc::c_int = 0;
    // let mut c: libc::c_char = 0;
    // let mut rep: *mut libc::c_char = 0 as *mut libc::c_char;
    string_decode(&mut s, (*mrph).midasi.as_mut_ptr() as *mut libc::c_char);
    (*mrph).hinsi = numeral_decode(&mut s) as libc::c_char;
    (*mrph).bunrui = numeral_decode(&mut s) as libc::c_char;
    (*mrph).katuyou1 = numeral_decode(&mut s) as libc::c_char;
    (*mrph).katuyou2 = numeral_decode(&mut s) as libc::c_char;
    (*mrph).weight = numeral_decode(&mut s) as libc::c_uchar;
    (*mrph).con_tbl = numeral_decode(&mut s);
    string_decode(&mut s, (*mrph).yomi.as_mut_ptr() as *mut libc::c_char);
    string_decode(&mut s, (*mrph).midasi2.as_mut_ptr() as *mut libc::c_char);
    if (*mrph).midasi2[0 as libc::c_int as usize] == 0 {
        strcpy((*mrph).midasi2.as_mut_ptr() as *mut libc::c_char,
               (*mrph).midasi.as_mut_ptr() as *const libc::c_char);
    }
    (*mrph).length =
        strlen((*mrph).midasi.as_mut_ptr() as *const libc::c_char) as
            libc::c_int;
    imi_length = numeral_decode(&mut s);
    if imi_length != 0 {
        i = 0 as libc::c_int;
        while i < imi_length {
            let fresh8 = s;
            s = s.offset(1);
            (*mrph).imis[i as usize] = *fresh8 as libc::c_uchar;
            i += 1
        }
        (*mrph).imis[i as usize] = '\u{0}' as i32 as libc::c_uchar
    } else {
        strcpy((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
               b"NIL\x00" as *const u8 as *const libc::c_char);
    }
    if imi_length > 0 as libc::c_int &&
           (*mrph).imis[(imi_length - 2 as libc::c_int) as usize] as
               libc::c_int == 'D' as i32 {
        (*mrph).imis[(imi_length - 2 as libc::c_int) as usize] =
            '\"' as i32 as libc::c_uchar;
        (*mrph).imis[(imi_length - 1 as libc::c_int) as usize] =
            '\u{0}' as i32 as libc::c_uchar;
        if *opt as libc::c_int != 1 as libc::c_int {
            (*mrph).weight = 255 as libc::c_int as libc::c_uchar
        } else { *opt = 4 as libc::c_int as libc::c_char }
    }
    if *opt as libc::c_int & 2 as libc::c_int != 0 &&
           (*mrph).weight as libc::c_int != 255 as libc::c_int {
        (*mrph).weight =
            ((*mrph).weight as libc::c_int + 6 as libc::c_int) as
                libc::c_uchar;
        if imi_length == 0 as libc::c_int {
            strcpy((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
                   b"\"\x00" as *const u8 as *const libc::c_char);
        } else {
            (*mrph).imis[strlen((*mrph).imis.as_mut_ptr() as
                                    *const libc::c_char).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                             as usize] = ' ' as i32 as libc::c_uchar
        }
        strcat((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
               b"\xe9\x9d\x9e\xe6\xa8\x99\xe6\xba\x96\xe8\xa1\xa8\xe8\xa8\x98\x00"
                   as *const u8 as *const libc::c_char);
        strcat((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
               b"\"\x00" as *const u8 as *const libc::c_char);
        imi_length =
            strlen((*mrph).imis.as_mut_ptr() as *const libc::c_char) as
                libc::c_int
    }
    if *opt as libc::c_int & 8 as libc::c_int != 0 &&
           (*mrph).weight as libc::c_int != 255 as libc::c_int {
        if *opt as libc::c_int & 32 as libc::c_int != 0 &&
               (*mrph).hinsi as libc::c_int == prolong_ng_hinsi2 &&
               deleted_bytes == 3 as libc::c_int &&
               check_code((*mrph).midasi.as_mut_ptr(), 0 as libc::c_int) ==
                   0xa6a0 as libc::c_int &&
               check_code((*mrph).midasi.as_mut_ptr(),
                          (*mrph).length - 3 as libc::c_int) ==
                   0xa6a0 as libc::c_int {
            let mut m: libc::c_int = 0;
            let mut rep_pos: *mut libc::c_char =
                strstr((*mrph).imis.as_mut_ptr() as *const libc::c_char,
                       b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                           as *const u8 as *const libc::c_char);
            if !rep_pos.is_null() &&
                   check_code(rep_pos.offset(13 as libc::c_int as isize) as
                                  *mut libc::c_uchar, 0 as libc::c_int) ==
                       0xffff as libc::c_int {
                (*mrph).weight = 255 as libc::c_int as libc::c_uchar
            } else {
                m = pre_m_buffer_num;
                while m < m_buffer_num {
                    if (*m_buffer.offset(m as isize)).length ==
                           (*mrph).length + 3 as libc::c_int &&
                           check_code((*m_buffer.offset(m as
                                                            isize)).midasi2.as_mut_ptr(),
                                      (*m_buffer.offset(m as isize)).length -
                                          3 as libc::c_int) ==
                               0xa1bc as libc::c_int &&
                           strcmp((*m_buffer.offset(m as
                                                        isize)).imis.as_mut_ptr()
                                      as *const libc::c_char,
                                  (*mrph).imis.as_mut_ptr() as
                                      *const libc::c_char) == 0 as libc::c_int
                       {
                        (*mrph).weight = 255 as libc::c_int as libc::c_uchar;
                        break ;
                    } else { m += 1 }
                }
            }
        } else if ((*mrph).hinsi as libc::c_int == prolong_ng_hinsi1 ||
                       (*mrph).hinsi as libc::c_int == prolong_ng_hinsi2 ||
                       (*mrph).hinsi as libc::c_int == prolong_ng_hinsi3 ||
                       (*mrph).hinsi as libc::c_int == prolong_ng_hinsi4 &&
                           (*mrph).bunrui as libc::c_int ==
                               prolong_ng_bunrui4_1 ||
                       (*mrph).hinsi as libc::c_int == prolong_ng_hinsi4 &&
                           (*mrph).bunrui as libc::c_int ==
                               prolong_ng_bunrui4_2 &&
                           (*mrph).length == 3 as libc::c_int ||
                       (*mrph).hinsi as libc::c_int == prolong_ng_hinsi4 &&
                           (*mrph).bunrui as libc::c_int ==
                               prolong_ng_bunrui4_3 &&
                           (*mrph).length == 3 as libc::c_int) &&
                      strstr((*mrph).imis.as_mut_ptr() as *const libc::c_char,
                             b"\xe9\x95\xb7\xe9\x9f\xb3\xe6\x8c\xbf\xe5\x85\xa5\xe5\x8f\xaf\x00"
                                 as *const u8 as
                                 *const libc::c_char).is_null() {
            (*mrph).weight = 255 as libc::c_int as libc::c_uchar
        } else {
            (*mrph).weight =
                ((*mrph).weight as libc::c_int +
                     if (*mrph).hinsi as libc::c_int == prolong_interjection {
                         6 as libc::c_int
                     } else if (*mrph).hinsi as libc::c_int == prolong_copula
                      {
                         50 as libc::c_int
                     } else { 9 as libc::c_int }) as libc::c_uchar;
            if imi_length == 0 as libc::c_int {
                strcpy((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
                       b"\"\x00" as *const u8 as *const libc::c_char);
            } else {
                (*mrph).imis[strlen((*mrph).imis.as_mut_ptr() as
                                        *const libc::c_char).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                 as usize] = ' ' as i32 as libc::c_uchar
            }
            strcat((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
                   b"\xe9\x95\xb7\xe9\x9f\xb3\xe6\x8c\xbf\xe5\x85\xa5\x00" as
                       *const u8 as *const libc::c_char);
            strcat((*mrph).imis.as_mut_ptr() as *mut libc::c_char,
                   b"\"\x00" as *const u8 as *const libc::c_char);
        }
    }
    (*mrph).length += deleted_bytes;
    if (*mrph).length >= 129 as libc::c_int {
        (*mrph).weight = 255 as libc::c_int as libc::c_uchar
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn numeral_decode(mut str: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    s = *str as *mut libc::c_uchar;
    *str = s.offset(2 as libc::c_int as isize) as *mut libc::c_char;
    return (*s as libc::c_int - 0x20 as libc::c_int) *
               (0x100 as libc::c_int - 0x20 as libc::c_int) +
               *s.offset(1 as libc::c_int as isize) as libc::c_int -
               0x20 as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string_decode(mut str: *mut *mut libc::c_char,
                                       mut out: *mut libc::c_char) {
    while **str as libc::c_int != 0x20 as libc::c_int &&
              **str as libc::c_int != '\t' as i32 {
        let fresh9 = *str;
        *str = (*str).offset(1);
        let fresh10 = out;
        out = out.offset(1);
        *fresh10 = *fresh9
    }
    *str = (*str).offset(1);
    *out = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn trim_space(mut pos: libc::c_int) -> libc::c_int {
    while String[pos as usize] as libc::c_int == 0xa1 as libc::c_int &&
              String[(pos + 1 as libc::c_int) as usize] as libc::c_int ==
                  0xa1 as libc::c_int {
        pos += 2 as libc::c_int
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn undef_word(mut pos: libc::c_int) -> libc::c_int {
    // let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut next_code: libc::c_int = 0;
    let mut cur_bytes: libc::c_int = 0;
    cur_bytes = utf8_bytes(String.as_mut_ptr().offset(pos as isize));
    code = check_code(String.as_mut_ptr(), pos);
    if code == 0xa5a0 as libc::c_int || code == 0xffff as libc::c_int ||
           code == 0xa3b0 as libc::c_int {
        end = pos + cur_bytes
    } else if code == 0x20 as libc::c_int {
        end = pos + 1 as libc::c_int
    } else {
        end = pos;
        while !(end - pos >= 129 as libc::c_int - cur_bytes) {
            end += cur_bytes;
            next_code = check_code(String.as_mut_ptr(), end);
            cur_bytes = utf8_bytes(String.as_mut_ptr().offset(end as isize));
            if !(next_code == code ||
                     code == 0xa6a0 as libc::c_int &&
                         next_code == 0xa1bc as libc::c_int ||
                     code == 0xa4a0 as libc::c_int &&
                         next_code == 0xa1a5 as libc::c_int) {
                break ;
            }
        }
    }
    if UseGivenSegmentation_Opt != 0 {
        end = pos + String2Length[pos as usize]
    }
    match code {
        32 => {
            (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                kuuhaku_hinsi as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                kuuhaku_bunrui as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                kuuhaku_con_tbl
        }
        42656 => {
            (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                undef_hinsi as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                undef_kata_bunrui as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                undef_kata_con_tbl
        }
        42144 => {
            (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                undef_hinsi as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                undef_alph_bunrui as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                undef_alph_con_tbl
        }
        _ => {
            (*m_buffer.offset(m_buffer_num as isize)).hinsi =
                undef_hinsi as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).bunrui =
                undef_etc_bunrui as libc::c_char;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                undef_etc_con_tbl
        }
    }
    (*m_buffer.offset(m_buffer_num as isize)).katuyou1 =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(m_buffer_num as isize)).katuyou2 =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(m_buffer_num as isize)).length = end - pos;
    if end - pos >= 129 as libc::c_int {
        fprintf(stderr,
                b"Too long undef_word<%s>\n\x00" as *const u8 as
                    *const libc::c_char, String.as_mut_ptr());
        return 0 as libc::c_int
    }
    if code == 0x20 as libc::c_int {
        strcpy((*m_buffer.offset(m_buffer_num as isize)).midasi.as_mut_ptr()
                   as *mut libc::c_char,
               b"\\ \x00" as *const u8 as *const libc::c_char);
        strcpy((*m_buffer.offset(m_buffer_num as isize)).yomi.as_mut_ptr() as
                   *mut libc::c_char,
               b"\\ \x00" as *const u8 as *const libc::c_char);
    } else {
        strncpy((*m_buffer.offset(m_buffer_num as isize)).midasi.as_mut_ptr()
                    as *mut libc::c_char,
                String.as_mut_ptr().offset(pos as isize) as
                    *const libc::c_char, (end - pos) as libc::c_ulong);
        (*m_buffer.offset(m_buffer_num as isize)).midasi[(end - pos) as usize]
            = '\u{0}' as i32 as libc::c_uchar;
        strncpy((*m_buffer.offset(m_buffer_num as isize)).yomi.as_mut_ptr() as
                    *mut libc::c_char,
                String.as_mut_ptr().offset(pos as isize) as
                    *const libc::c_char, (end - pos) as libc::c_ulong);
        (*m_buffer.offset(m_buffer_num as isize)).yomi[(end - pos) as usize] =
            '\u{0}' as i32 as libc::c_uchar
    }
    (*m_buffer.offset(m_buffer_num as isize)).weight =
        10 as libc::c_int as libc::c_uchar;
    strcpy((*m_buffer.offset(m_buffer_num as isize)).midasi2.as_mut_ptr() as
               *mut libc::c_char,
           (*m_buffer.offset(m_buffer_num as isize)).midasi.as_mut_ptr() as
               *const libc::c_char);
    strcpy((*m_buffer.offset(m_buffer_num as isize)).imis.as_mut_ptr() as
               *mut libc::c_char,
           b"NIL\x00" as *const u8 as *const libc::c_char);
    check_connect(pos, m_buffer_num, 0 as libc::c_int as libc::c_char);
    m_buffer_num += 1;
    if m_buffer_num == mrph_buffer_max { realloc_mrph_buffer(); }
    if Repetition_Opt != 0 || Onomatopoeia_Opt != 0 {
        recognize_onomatopoeia(pos);
    }
    if code == 0x20 as libc::c_int {
        return through_word(pos, m_buffer_num - 1 as libc::c_int)
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_unicode_char_type(mut code: libc::c_int)
 -> libc::c_int {
    return if code > 0x303f as libc::c_int && code < 0x30a0 as libc::c_int {
        0xa5a0 as libc::c_int
    } else if code > 0x309f as libc::c_int && code < 0x30fb as libc::c_int {
        0xa6a0 as libc::c_int
    } else if code == 0x30fc as libc::c_int {
        0xa1bc as libc::c_int
    } else if code == 0xff0e as libc::c_int {
        0xa1a5 as libc::c_int
    } else if code > 0xff0f as libc::c_int && code < 0xff1a as libc::c_int ||
        code > 0x2f as libc::c_int && code < 0x3a as libc::c_int {
        0xa3c0 as libc::c_int
    } else if code > 0x40 as libc::c_int && code < 0x5b as libc::c_int ||
        code > 0x60 as libc::c_int && code < 0x7b as libc::c_int ||
        code > 0xbf as libc::c_int && code < 0x100 as libc::c_int ||
        code > 0xff20 as libc::c_int && code < 0xff3b as libc::c_int
        ||
        code > 0xff40 as libc::c_int && code < 0xff5b as libc::c_int
    {
        0xa4a0 as libc::c_int
    } else if code > 0x4dff as libc::c_int && code < 0xa000 as libc::c_int ||
        code == 0x3005 as libc::c_int {
        0xffff as libc::c_int
    } else if code > 0x36f as libc::c_int && code < 0x400 as libc::c_int {
        0xb0a0 as libc::c_int
    } else { 0xa3b0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn check_utf8_char_type(mut ucp: *mut libc::c_uchar)
 -> libc::c_int {
    let mut code: libc::c_int = 0 as libc::c_int;
    // let mut length: libc::c_int = strlen(ucp as *const libc::c_char) as libc::c_int;
    let mut unicode: libc::c_int = 0;
    let mut c: libc::c_uchar = *ucp;
    if c as libc::c_int > 0xfb as libc::c_int {
        code = 0 as libc::c_int
    } else if c as libc::c_int > 0xf7 as libc::c_int {
        code = 0 as libc::c_int
    } else if c as libc::c_int > 0xef as libc::c_int {
        code = 0 as libc::c_int
    } else if c as libc::c_int > 0xdf as libc::c_int {
        unicode =
            (c as libc::c_int & 0xf as libc::c_int) << 12 as libc::c_int;
        c = *ucp.offset(1 as libc::c_int as isize);
        unicode +=
            (c as libc::c_int & 0x3f as libc::c_int) << 6 as libc::c_int;
        c = *ucp.offset(2 as libc::c_int as isize);
        unicode += c as libc::c_int & 0x3f as libc::c_int;
        code = check_unicode_char_type(unicode)
    } else if c as libc::c_int > 0x7f as libc::c_int {
        unicode =
            (c as libc::c_int & 0x1f as libc::c_int) << 6 as libc::c_int;
        c = *ucp.offset(1 as libc::c_int as isize);
        unicode += c as libc::c_int & 0x3f as libc::c_int;
        code = check_unicode_char_type(unicode)
    } else { code = check_unicode_char_type(c as libc::c_int) }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn check_code(mut cp: *mut libc::c_uchar,
                                    mut pos: libc::c_int) -> libc::c_int {
    // let mut code: libc::c_int = 0;
    if *cp.offset(pos as isize) as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int
    } else {
        if *cp.offset(pos as isize) as libc::c_int == 0x20 as libc::c_int {
            return 0x20 as libc::c_int
        }
    }
    return check_utf8_char_type(cp.offset(pos as isize));
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <juman_init_etc> 未定義語処理，数詞処理，透過処理等の準備
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn juman_init_etc() {
    // let mut i: libc::c_int = 0;
    // let mut flag: libc::c_int = 0;
    /* 未定義語処理の準備 */
    undef_hinsi =
        get_hinsi_id(b"\xe6\x9c\xaa\xe5\xae\x9a\xe7\xbe\xa9\xe8\xaa\x9e\x00"
                         as *const u8 as *const libc::c_char);
    undef_kata_bunrui =
        get_bunrui_id(b"\xe3\x82\xab\xe3\x82\xbf\xe3\x82\xab\xe3\x83\x8a\x00"
                          as *const u8 as *const libc::c_char, undef_hinsi);
    undef_alph_bunrui =
        get_bunrui_id(b"\xe3\x82\xa2\xe3\x83\xab\xe3\x83\x95\xe3\x82\xa1\xe3\x83\x99\xe3\x83\x83\xe3\x83\x88\x00"
                          as *const u8 as *const libc::c_char, undef_hinsi);
    undef_etc_bunrui =
        get_bunrui_id(b"\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8
                          as *const libc::c_char, undef_hinsi);
    undef_kata_con_tbl =
        check_table_for_undef(undef_hinsi, undef_kata_bunrui);
    undef_alph_con_tbl =
        check_table_for_undef(undef_hinsi, undef_alph_bunrui);
    undef_etc_con_tbl = check_table_for_undef(undef_hinsi, undef_etc_bunrui);
    /* 数詞処理の準備 */
    suusi_hinsi =
        get_hinsi_id(b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    suusi_bunrui =
        get_bunrui_id(b"\xe6\x95\xb0\xe8\xa9\x9e\x00" as *const u8 as
                          *const libc::c_char, suusi_hinsi);
    /* 透過処理の準備 */
    kakko_hinsi =
        get_hinsi_id(b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                         *const libc::c_char);
    kakko_bunrui1 =
        get_bunrui_id(b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00" as *const u8
                          as *const libc::c_char, kakko_hinsi);
    kakko_bunrui2 =
        get_bunrui_id(b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00" as *const u8
                          as *const libc::c_char, kakko_hinsi);
    kuuhaku_hinsi =
        get_hinsi_id(b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                         *const libc::c_char);
    kuuhaku_bunrui =
        get_bunrui_id(b"\xe7\xa9\xba\xe7\x99\xbd\x00" as *const u8 as
                          *const libc::c_char, kuuhaku_hinsi);
    kuuhaku_con_tbl = check_table_for_undef(kuuhaku_hinsi, kuuhaku_bunrui);
    /* オノマトペ自動認識の準備 */
    onomatopoeia_hinsi =
        get_hinsi_id(b"\xe5\x89\xaf\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    onomatopoeia_bunrui = 0 as libc::c_int;
    onomatopoeia_con_tbl =
        check_table_for_undef(onomatopoeia_hinsi, onomatopoeia_bunrui);
    /* 連濁処理・小書き文字・長音記号処理の準備 */
    rendaku_hinsi1 =
        get_hinsi_id(b"\xe5\x8b\x95\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char); /* 母音動詞(type=1)の基本連用形のform_idを基本連用形の汎用idとみなす */
    rendaku_renyou =
        get_form_id(b"\xe5\x9f\xba\xe6\x9c\xac\xe9\x80\xa3\xe7\x94\xa8\xe5\xbd\xa2\x00"
                        as *const u8 as *const libc::c_char,
                    1 as libc::c_int); /* 直前が形式名詞は不可 */
    rendaku_hinsi2 =
        get_hinsi_id(b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    rendaku_bunrui2_1 =
        get_bunrui_id(b"\xe6\x99\xae\xe9\x80\x9a\xe5\x90\x8d\xe8\xa9\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi2);
    rendaku_bunrui2_2 =
        get_bunrui_id(b"\xe3\x82\xb5\xe5\xa4\x89\xe5\x90\x8d\xe8\xa9\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi2);
    rendaku_bunrui2_3 =
        get_bunrui_id(b"\xe5\xbd\xa2\xe5\xbc\x8f\xe5\x90\x8d\xe8\xa9\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi2);
    rendaku_hinsi3 =
        get_hinsi_id(b"\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\x00" as *const u8
                         as *const libc::c_char);
    rendaku_hinsi4 =
        get_hinsi_id(b"\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00" as *const u8
                         as *const libc::c_char);
    rendaku_bunrui4_1 =
        get_bunrui_id(b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x80\xa7\xe8\xbf\xb0\xe8\xaa\x9e\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi4);
    rendaku_bunrui4_2 =
        get_bunrui_id(b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x80\xa7\xe5\x90\x8d\xe8\xa9\x9e\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi4);
    rendaku_bunrui4_3 =
        get_bunrui_id(b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x80\xa7\xe5\x90\x8d\xe8\xa9\x9e\xe5\x8a\xa9\xe6\x95\xb0\xe8\xbe\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi4);
    rendaku_bunrui4_4 =
        get_bunrui_id(b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x80\xa7\xe7\x89\xb9\xe6\xae\x8a\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      rendaku_hinsi4);
    prolong_interjection =
        get_hinsi_id(b"\xe6\x84\x9f\xe5\x8b\x95\xe8\xa9\x9e\x00" as *const u8
                         as *const libc::c_char);
    prolong_copula =
        get_hinsi_id(b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as *const u8
                         as *const libc::c_char);
    prolong_ng_hinsi1 =
        get_hinsi_id(b"\xe5\x8b\x95\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    prolong_ng_hinsi2 =
        get_hinsi_id(b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    prolong_ng_hinsi3 =
        get_hinsi_id(b"\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00" as *const u8
                         as *const libc::c_char);
    prolong_ng_hinsi4 =
        get_hinsi_id(b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                         *const libc::c_char);
    prolong_ng_bunrui4_1 =
        get_bunrui_id(b"\xe6\xa0\xbc\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8
                          as *const libc::c_char, prolong_ng_hinsi4);
    prolong_ng_bunrui4_2 =
        get_bunrui_id(b"\xe5\x89\xaf\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8
                          as *const libc::c_char, prolong_ng_hinsi4);
    prolong_ng_bunrui4_3 =
        get_bunrui_id(b"\xe6\x8e\xa5\xe7\xb6\x9a\xe5\x8a\xa9\xe8\xa9\x9e\x00"
                          as *const u8 as *const libc::c_char,
                      prolong_ng_hinsi4);
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <suusi_word> 数詞処理
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn suusi_word(mut pos: libc::c_int,
                                    mut m_num: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut new_mrph: *mut MRPH = 0 as *mut MRPH;
    let mut pre_mrph: *mut MRPH = 0 as *mut MRPH;
    new_mrph = &mut *m_buffer.offset(m_num as isize) as *mut MRPH;
    if (*new_mrph).hinsi as libc::c_int != suusi_hinsi ||
           (*new_mrph).bunrui as libc::c_int != suusi_bunrui {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    j = 0 as libc::c_int;
    loop  {
        i = *match_pbuf.offset(j as isize);
        if !(i >= 0 as libc::c_int) { break ; }
        pre_mrph =
            &mut *m_buffer.offset((*p_buffer.offset(i as isize)).mrph_p as
                                      isize) as *mut MRPH;
        if (*pre_mrph).hinsi as libc::c_int == suusi_hinsi &&
               (*pre_mrph).bunrui as libc::c_int == suusi_bunrui &&
               check_matrix((*pre_mrph).con_tbl, (*new_mrph).con_tbl) !=
                   0 as libc::c_int {
            if strlen((*pre_mrph).midasi.as_mut_ptr() as
                          *const libc::c_char).wrapping_add(strlen((*new_mrph).midasi.as_mut_ptr()
                                                                       as
                                                                       *const libc::c_char))
                   >= 108 as libc::c_int as libc::c_ulong ||
                   strlen((*pre_mrph).yomi.as_mut_ptr() as
                              *const libc::c_char).wrapping_add(strlen((*new_mrph).yomi.as_mut_ptr()
                                                                           as
                                                                           *const libc::c_char))
                       >= 108 as libc::c_int as libc::c_ulong {
                /* 零が連続した際に読みの候補数が爆発するため，より短い SUUSI_MIDASI_MAX, SUUSI_YOMI_MAX を設定 2014/08/14 */
		/* MIDASI_MAX、YOMI_MAXを越える数詞は分割するように変更 08/01/15 */
		/* fprintf(stderr, "Too long suusi<%s>\n", String);
		   return FALSE; */
                return (0 as libc::c_int == 0) as libc::c_int
            }
            *m_buffer.offset(m_buffer_num as isize) = *pre_mrph;
            strcat((*m_buffer.offset(m_buffer_num as
                                         isize)).midasi.as_mut_ptr() as
                       *mut libc::c_char,
                   (*new_mrph).midasi.as_mut_ptr() as *const libc::c_char);
            strcat((*m_buffer.offset(m_buffer_num as isize)).yomi.as_mut_ptr()
                       as *mut libc::c_char,
                   (*new_mrph).yomi.as_mut_ptr() as *const libc::c_char);
            strcat((*m_buffer.offset(m_buffer_num as
                                         isize)).midasi2.as_mut_ptr() as
                       *mut libc::c_char,
                   (*new_mrph).midasi2.as_mut_ptr() as *const libc::c_char);
            let ref mut fresh11 =
                (*m_buffer.offset(m_buffer_num as isize)).length;
            *fresh11 =
                (*fresh11 as
                     libc::c_ulong).wrapping_add(strlen((*new_mrph).midasi.as_mut_ptr()
                                                            as
                                                            *const libc::c_char))
                    as libc::c_int as libc::c_int;
            /* コストは後ろ側の方を継承 */
            (*m_buffer.offset(m_buffer_num as isize)).weight =
                (*new_mrph).weight;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                (*new_mrph).con_tbl;
            *p_buffer.offset(p_buffer_num as isize) =
                *p_buffer.offset(i as isize);
            (*p_buffer.offset(p_buffer_num as isize)).end =
                (pos as
                     libc::c_ulong).wrapping_add(strlen((*new_mrph).midasi.as_mut_ptr()
                                                            as
                                                            *const libc::c_char))
                    as libc::c_int;
            (*p_buffer.offset(p_buffer_num as isize)).mrph_p = m_buffer_num;
            (*p_buffer.offset(p_buffer_num as isize)).score +=
                ((*new_mrph).weight as libc::c_int -
                     (*pre_mrph).weight as libc::c_int) *
                    Class[(*new_mrph).hinsi as
                              usize][(*new_mrph).bunrui as usize].cost *
                    cost_omomi.keitaiso;
            m_buffer_num += 1;
            if m_buffer_num == mrph_buffer_max {
                realloc_mrph_buffer();
                new_mrph = &mut *m_buffer.offset(m_num as isize) as *mut MRPH
                /* fixed by kuro 99/09/01 */
            }
            p_buffer_num += 1;
            if p_buffer_num == process_buffer_max {
                realloc_process_buffer();
            }
        }
        j += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <through_word> 括弧，空白の透過処理
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn through_word(mut pos: libc::c_int,
                                      mut m_num: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut sc: libc::c_int = 0;
    let mut scmin: libc::c_int = 0;
    let mut tmp_path: libc::c_int = 0;
    let mut now_mrph: *mut MRPH = 0 as *mut MRPH;
    let mut mrph_p: *mut MRPH = 0 as *mut MRPH;
    now_mrph = &mut *m_buffer.offset(m_num as isize) as *mut MRPH;
    if is_through(now_mrph) == 0 {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    l = 0 as libc::c_int;
    loop  {
        i = *match_pbuf.offset(l as isize);
        if !(i >= 0 as libc::c_int) { break ; }
        j = 0 as libc::c_int;
        while j < m_buffer_num {
            mrph_p = &mut *m_buffer.offset(j as isize) as *mut MRPH;
            if (*mrph_p).hinsi as libc::c_int ==
                   (*now_mrph).hinsi as libc::c_int &&
                   (*mrph_p).bunrui as libc::c_int ==
                       (*now_mrph).bunrui as libc::c_int &&
                   (*mrph_p).con_tbl ==
                       (*m_buffer.offset((*p_buffer.offset(i as isize)).mrph_p
                                             as isize)).con_tbl &&
                   (*mrph_p).weight as libc::c_int ==
                       (*now_mrph).weight as libc::c_int &&
                   strcmp((*mrph_p).midasi.as_mut_ptr() as
                              *const libc::c_char,
                          (*now_mrph).midasi.as_mut_ptr() as
                              *const libc::c_char) == 0 as libc::c_int &&
                   strcmp((*mrph_p).yomi.as_mut_ptr() as *const libc::c_char,
                          (*now_mrph).yomi.as_mut_ptr() as
                              *const libc::c_char) == 0 as libc::c_int {
                break ;
            }
            j += 1
        }
        n = j;
        if n == m_buffer_num {
            /* その文の中で初めて出現したタイプの括弧であれば，直前の
	       形態素のcon_tblを透過させるようにcon_tblを変化させて，
	       m_bufferに追加する */
            *m_buffer.offset(m_buffer_num as isize) = *now_mrph;
            (*m_buffer.offset(m_buffer_num as isize)).con_tbl =
                (*m_buffer.offset((*p_buffer.offset(i as isize)).mrph_p as
                                      isize)).con_tbl;
            m_buffer_num += 1;
            if m_buffer_num == mrph_buffer_max {
                realloc_mrph_buffer();
                now_mrph = &mut *m_buffer.offset(m_num as isize) as *mut MRPH
                /* fixed by kuro 99/09/01 */
            }
        }
        /* 透過規則に基づく連接 */
        sc =
            (*now_mrph).weight as libc::c_int * cost_omomi.keitaiso *
                Class[(*now_mrph).hinsi as
                          usize][(*now_mrph).bunrui as usize].cost;
        j = 0 as libc::c_int;
        while j < p_buffer_num {
            if (*p_buffer.offset(j as isize)).mrph_p == n &&
                   (*p_buffer.offset(j as isize)).start == pos {
                break ;
            }
            j += 1
        }
        nn = j;
        if nn == p_buffer_num {
            /* 初めて現われるような透過なら，p_bufferに新たに追加する */
            (*p_buffer.offset(p_buffer_num as isize)).score =
                (*p_buffer.offset(i as isize)).score + sc;
            (*p_buffer.offset(p_buffer_num as isize)).mrph_p = n;
            (*p_buffer.offset(p_buffer_num as isize)).start = pos;
            (*p_buffer.offset(p_buffer_num as isize)).end =
                pos + (*now_mrph).length;
            (*p_buffer.offset(p_buffer_num as
                                  isize)).path[0 as libc::c_int as usize] = i;
            (*p_buffer.offset(p_buffer_num as
                                  isize)).path[1 as libc::c_int as usize] =
                -(1 as libc::c_int);
            (*p_buffer.offset(p_buffer_num as isize)).connect =
                (0 as libc::c_int == 0) as libc::c_int;
            p_buffer_num += 1;
            if p_buffer_num == process_buffer_max {
                realloc_process_buffer();
            }
        } else {
            /* p_bufferの爆発を防ぐため，以前のp_bufferの使い回しをする */
            j = 0 as libc::c_int;
            while (*p_buffer.offset(nn as isize)).path[j as usize] !=
                      -(1 as libc::c_int) {
                j += 1
            }
            (*p_buffer.offset(nn as isize)).path[j as usize] = i;
            (*p_buffer.offset(nn as
                                  isize)).path[(j + 1 as libc::c_int) as
                                                   usize] =
                -(1 as libc::c_int);
            /* コストが高いものは枝刈りする */
            scmin = 2147483647 as libc::c_int;
            j = 0 as libc::c_int;
            while (*p_buffer.offset(nn as isize)).path[j as usize] !=
                      -(1 as libc::c_int) {
                if scmin >
                       (*p_buffer.offset((*p_buffer.offset(nn as
                                                               isize)).path[j
                                                                                as
                                                                                usize]
                                             as isize)).score {
                    scmin =
                        (*p_buffer.offset((*p_buffer.offset(nn as
                                                                isize)).path[j
                                                                                 as
                                                                                 usize]
                                              as isize)).score
                }
                j += 1
            }
            j = 0 as libc::c_int;
            while (*p_buffer.offset(nn as isize)).path[j as usize] !=
                      -(1 as libc::c_int) {
                if (*p_buffer.offset((*p_buffer.offset(nn as
                                                           isize)).path[j as
                                                                            usize]
                                         as isize)).score >
                       scmin + cost_omomi.cost_haba {
                    k = j;
                    while (*p_buffer.offset(nn as isize)).path[k as usize] !=
                              -(1 as libc::c_int) {
                        (*p_buffer.offset(nn as isize)).path[k as usize] =
                            (*p_buffer.offset(nn as
                                                  isize)).path[(k +
                                                                    1 as
                                                                        libc::c_int)
                                                                   as usize];
                        k += 1
                    }
                    j -= 1
                }
                j += 1
            }
            /* もっともコストが小さいpathを0番目にもってくる */
            j = 1 as libc::c_int;
            while (*p_buffer.offset(nn as isize)).path[j as usize] !=
                      -(1 as libc::c_int) {
                if (*p_buffer.offset((*p_buffer.offset(nn as
                                                           isize)).path[j as
                                                                            usize]
                                         as isize)).score == scmin {
                    tmp_path =
                        (*p_buffer.offset(nn as
                                              isize)).path[0 as libc::c_int as
                                                               usize];
                    (*p_buffer.offset(nn as
                                          isize)).path[0 as libc::c_int as
                                                           usize] =
                        (*p_buffer.offset(nn as isize)).path[j as usize];
                    (*p_buffer.offset(nn as isize)).path[j as usize] =
                        tmp_path;
                    break ;
                } else { j += 1 }
            }
            (*p_buffer.offset(nn as isize)).score = scmin + sc
        }
        l += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <is_through>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn is_through(mut mrph_p: *mut MRPH) -> libc::c_int {
    return if (*mrph_p).hinsi as libc::c_int == kakko_hinsi &&
        (*mrph_p).bunrui as libc::c_int == kakko_bunrui2 ||
        (*mrph_p).hinsi as libc::c_int == kuuhaku_hinsi &&
            (*mrph_p).bunrui as libc::c_int == kuuhaku_bunrui {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
#[no_mangle]
pub static mut OutputAV: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut OutputAVnum: libc::c_int = 0;
#[no_mangle]
pub static mut OutputAVmax: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn prepare_path_mrph(mut path_num: libc::c_int,
                                           mut para_flag: libc::c_int)
 -> *mut MRPH {
    let mut mrph_p: *mut MRPH = 0 as *mut MRPH;
    // let mut j: libc::c_int = 0;
    mrph_p =
        &mut *m_buffer.offset((*p_buffer.offset(path_num as isize)).mrph_p as
                                  isize) as *mut MRPH;
    if para_flag != 0 as libc::c_int &&
           is_through(mrph_p) == (0 as libc::c_int == 0) as libc::c_int {
        return 0 as *mut MRPH
    }
    if para_flag != 0 {
        strcpy(kigou.as_mut_ptr() as *mut libc::c_char,
               b"@ \x00" as *const u8 as *const libc::c_char);
    } else {
        kigou[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_uchar
    }
    strcpy(midasi1.as_mut_ptr() as *mut libc::c_char,
           (*mrph_p).midasi.as_mut_ptr() as *const libc::c_char);
    strcpy(midasi2.as_mut_ptr() as *mut libc::c_char,
           if *(*mrph_p).midasi2.as_mut_ptr() as libc::c_int != 0 {
               (*mrph_p).midasi2.as_mut_ptr()
           } else { (*mrph_p).midasi.as_mut_ptr() } as *const libc::c_char);
    strcpy(yomi.as_mut_ptr() as *mut libc::c_char,
           (*mrph_p).yomi.as_mut_ptr() as *const libc::c_char);
    /* 連濁、小書き文字、長音記号処理用 */
    if strcmp(midasi1.as_mut_ptr() as *const libc::c_char,
              b"\\ \x00" as *const u8 as *const libc::c_char) != 0 &&
           strncmp(midasi1.as_mut_ptr() as *const libc::c_char,
                   String.as_mut_ptr().offset((*p_buffer.offset(path_num as
                                                                    isize)).start
                                                  as isize) as
                       *const libc::c_char, (*mrph_p).length as libc::c_ulong)
               != 0 {
        strncpy(midasi1.as_mut_ptr() as *mut libc::c_char,
                String.as_mut_ptr().offset((*p_buffer.offset(path_num as
                                                                 isize)).start
                                               as isize) as
                    *const libc::c_char,
                (*mrph_p).length as
                    libc::c_ulong); /* 隙間 10, 改行 1, 終端 1 */
        midasi1[(*mrph_p).length as usize] = '\u{0}' as i32 as libc::c_uchar
    }
    return mrph_p;
}
#[no_mangle]
pub unsafe extern "C" fn get_path_mrph(mut path_num: libc::c_int,
                                       mut para_flag: libc::c_int)
 -> *mut libc::c_char {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut mrph_p: *mut MRPH = 0 as *mut MRPH;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    mrph_p = prepare_path_mrph(path_num, para_flag);
    if mrph_p.is_null() { return 0 as *mut libc::c_char }
    len =
        strlen(kigou.as_mut_ptr() as
                   *const libc::c_char).wrapping_add(strlen(midasi1.as_mut_ptr()
                                                                as
                                                                *const libc::c_char)).wrapping_add(strlen(yomi.as_mut_ptr()
                                                                                                              as
                                                                                                              *const libc::c_char)).wrapping_add(strlen(midasi2.as_mut_ptr()
                                                                                                                                                            as
                                                                                                                                                            *const libc::c_char)).wrapping_add(strlen(Class[(*mrph_p).hinsi
                                                                                                                                                                                                                as
                                                                                                                                                                                                                usize][0
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           usize].id
                                                                                                                                                                                                          as
                                                                                                                                                                                                          *const libc::c_char)).wrapping_add(((*mrph_p).hinsi
                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                  /
                                                                                                                                                                                                                                                  10
                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                      libc::c_int)
                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                 libc::c_ulong).wrapping_add(1
                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                 libc::c_ulong)
            as libc::c_int;
    if (*mrph_p).bunrui != 0 {
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(Class[(*mrph_p).hinsi as
                                                              usize][(*mrph_p).bunrui
                                                                         as
                                                                         usize].id
                                                        as
                                                        *const libc::c_char))
                as libc::c_int as libc::c_int
    } else { len += 1 as libc::c_int }
    len +=
        (*mrph_p).bunrui as libc::c_int / 10 as libc::c_int +
            1 as libc::c_int;
    if (*mrph_p).katuyou1 != 0 {
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(Type[(*mrph_p).katuyou1 as
                                                             usize].name as
                                                        *const libc::c_char))
                as libc::c_int as libc::c_int
    } else { len += 1 as libc::c_int }
    len +=
        (*mrph_p).katuyou1 as libc::c_int / 10 as libc::c_int +
            1 as libc::c_int;
    if (*mrph_p).katuyou2 != 0 {
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(Form[(*mrph_p).katuyou1 as
                                                             usize][(*mrph_p).katuyou2
                                                                        as
                                                                        usize].name
                                                        as
                                                        *const libc::c_char))
                as libc::c_int as libc::c_int
    } else { len += 1 as libc::c_int }
    len +=
        (*mrph_p).katuyou2 as libc::c_int / 10 as libc::c_int +
            1 as libc::c_int;
    len += 12 as libc::c_int;
    match Show_Opt2 {
        4 => {
            len =
                (len as
                     libc::c_ulong).wrapping_add(strlen((*mrph_p).imis.as_mut_ptr()
                                                            as
                                                            *const libc::c_char).wrapping_add(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_ulong))
                    as libc::c_int as libc::c_int;
            ret = malloc(len as libc::c_ulong) as *mut libc::c_char;
            sprintf(ret,
                    b"%s%s %s %s %s %d %s %d %s %d %s %d %s\n\x00" as
                        *const u8 as *const libc::c_char, kigou.as_mut_ptr(),
                    midasi1.as_mut_ptr(), yomi.as_mut_ptr(),
                    midasi2.as_mut_ptr(),
                    Class[(*mrph_p).hinsi as
                              usize][0 as libc::c_int as usize].id,
                    (*mrph_p).hinsi as libc::c_int,
                    if (*mrph_p).bunrui as libc::c_int != 0 {
                        Class[(*mrph_p).hinsi as
                                  usize][(*mrph_p).bunrui as usize].id
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).bunrui as libc::c_int,
                    if (*mrph_p).katuyou1 as libc::c_int != 0 {
                        Type[(*mrph_p).katuyou1 as usize].name
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).katuyou1 as libc::c_int,
                    if (*mrph_p).katuyou2 as libc::c_int != 0 {
                        Form[(*mrph_p).katuyou1 as
                                 usize][(*mrph_p).katuyou2 as usize].name
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).katuyou2 as libc::c_int,
                    (*mrph_p).imis.as_mut_ptr());
        }
        1 => {
            ret = malloc(len as libc::c_ulong) as *mut libc::c_char;
            sprintf(ret,
                    b"%s%s %s %s %s %d %s %d %s %d %s %d\n\x00" as *const u8
                        as *const libc::c_char, kigou.as_mut_ptr(),
                    midasi1.as_mut_ptr(), yomi.as_mut_ptr(),
                    midasi2.as_mut_ptr(),
                    Class[(*mrph_p).hinsi as
                              usize][0 as libc::c_int as usize].id,
                    (*mrph_p).hinsi as libc::c_int,
                    if (*mrph_p).bunrui as libc::c_int != 0 {
                        Class[(*mrph_p).hinsi as
                                  usize][(*mrph_p).bunrui as usize].id
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).bunrui as libc::c_int,
                    if (*mrph_p).katuyou1 as libc::c_int != 0 {
                        Type[(*mrph_p).katuyou1 as usize].name
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).katuyou1 as libc::c_int,
                    if (*mrph_p).katuyou2 as libc::c_int != 0 {
                        Form[(*mrph_p).katuyou1 as
                                 usize][(*mrph_p).katuyou2 as usize].name
                    } else {
                        b"*\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_uchar
                    }, (*mrph_p).katuyou2 as libc::c_int);
        }
        _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_best_path_num() -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    j = 0 as libc::c_int;
    last = p_buffer_num - 1 as libc::c_int;
    loop  {
        last =
            (*p_buffer.offset(last as isize)).path[0 as libc::c_int as usize];
        *path_buffer.offset(j as isize) = last;
        j += 1;
        if !((*p_buffer.offset(last as isize)).path[0 as libc::c_int as usize]
                 != 0) {
            break ;
        }
    }
    return j;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <print_path_mrph> 形態素の表示      >>> changed by yamaji <<<
------------------------------------------------------------------------------
*/
/*
  サーバーモード対応のため、引数outputを増やして出力先の変更を可能にする。
  NACSIS 吉岡
*/
/* para_flag != 0 なら @出力 */
#[no_mangle]
pub unsafe extern "C" fn print_path_mrph(mut output: *mut FILE,
                                         mut path_num: libc::c_int,
                                         mut para_flag: libc::c_int) {
    let mut proc_p: *mut PROCESS_BUFFER = 0 as *mut PROCESS_BUFFER;
    let mut mrph_p: *mut MRPH = 0 as *mut MRPH;
    // let mut newDicNo: libc::c_int = 0;
    // let mut now_r_buffer_num: libc::c_int = 0;
    // let mut r_last_mrph: MRPH =
    //     MRPH{midasi: [0; 129],
    //          midasi2: [0; 129],
    //          yomi: [0; 129],
    //          imis: [0; 1024],
    //          imi: 0 as *mut CELL,
    //          hinsi: 0,
    //          bunrui: 0,
    //          katuyou1: 0,
    //          katuyou2: 0,
    //          weight: 0,
    //          con_tbl: 0,
    //          length: 0,};
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    mrph_p = prepare_path_mrph(path_num, para_flag);
    if mrph_p.is_null() { return }
    proc_p = &mut *p_buffer.offset(path_num as isize) as *mut PROCESS_BUFFER;
    pos = (*proc_p).start;
    fputs(kigou.as_mut_ptr() as *const libc::c_char, output);
    let mut current_block_43: u64;
    match Show_Opt2 {
        0 => {
            len =
                strlen(yomi.as_mut_ptr() as *const libc::c_char) as
                    libc::c_int;
            yomi[len as usize] = ')' as i32 as libc::c_uchar;
            yomi[(len + 1 as libc::c_int) as usize] =
                '\u{0}' as i32 as libc::c_uchar;
            fprintf(output,
                    b"%-12.12s(%-12.12s%-10.10s %-14.14s\x00" as *const u8 as
                        *const libc::c_char, midasi1.as_mut_ptr(),
                    yomi.as_mut_ptr(), midasi2.as_mut_ptr(),
                    Class[(*mrph_p).hinsi as
                              usize][(*mrph_p).bunrui as usize].id);
            if (*mrph_p).katuyou1 != 0 {
                fprintf(output,
                        b" %-14.14s %-12.12s\x00" as *const u8 as
                            *const libc::c_char,
                        Type[(*mrph_p).katuyou1 as usize].name,
                        Form[(*mrph_p).katuyou1 as
                                 usize][(*mrph_p).katuyou2 as usize].name);
            }
            fputc('\n' as i32, output);
            current_block_43 = 12497913735442871383;
        }
        2 => {
            fprintf(output,
                    b"%s %s %s %d %d %d %d\n\x00" as *const u8 as
                        *const libc::c_char, midasi1.as_mut_ptr(),
                    yomi.as_mut_ptr(), midasi2.as_mut_ptr(),
                    (*mrph_p).hinsi as libc::c_int,
                    (*mrph_p).bunrui as libc::c_int,
                    (*mrph_p).katuyou1 as libc::c_int,
                    (*mrph_p).katuyou2 as libc::c_int);
            current_block_43 = 12497913735442871383;
        }
        3 => {
            /* ラティスのつながりを表示 */
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    path_num);
            i = 0 as libc::c_int;
            while (*proc_p).path[i as usize] != -(1 as libc::c_int) {
                if i != 0 {
                    fprintf(output,
                            b";\x00" as *const u8 as *const libc::c_char);
                }
                fprintf(output, b"%d\x00" as *const u8 as *const libc::c_char,
                        (*proc_p).path[i as usize]);
                i += 1
            }
            fprintf(output, b" \x00" as *const u8 as *const libc::c_char);
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    pos);
            if strcmp(midasi1.as_mut_ptr() as *const libc::c_char,
                      b"\\ \x00" as *const u8 as *const libc::c_char) == 0 {
                pos += 1
            } else {
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(strlen(midasi1.as_mut_ptr()
                                                                as
                                                                *const libc::c_char))
                        as libc::c_int as libc::c_int
            }
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    pos);
            current_block_43 = 14883255393027039194;
        }
        1 | 4 => { current_block_43 = 14883255393027039194; }
        5 => {
            fprintf(output, b"%s \x00" as *const u8 as *const libc::c_char,
                    midasi2.as_mut_ptr());
            current_block_43 = 12497913735442871383;
        }
        _ => { current_block_43 = 12497913735442871383; }
    }
    match current_block_43 {
        14883255393027039194 =>
        /* -E オプションはこの部分だけ追加,
	   その後は -e -e2 オプションと同様の出力をするので break しない */
        {
            fprintf(output,
                    b"%s %s %s \x00" as *const u8 as *const libc::c_char,
                    midasi1.as_mut_ptr(), yomi.as_mut_ptr(),
                    midasi2.as_mut_ptr()); /* -e では imis は出力しない */
            fprintf(output, b"%s \x00" as *const u8 as *const libc::c_char,
                    Class[(*mrph_p).hinsi as
                              usize][0 as libc::c_int as usize].id);
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    (*mrph_p).hinsi as libc::c_int);
            if (*mrph_p).bunrui != 0 {
                fprintf(output,
                        b"%s \x00" as *const u8 as *const libc::c_char,
                        Class[(*mrph_p).hinsi as
                                  usize][(*mrph_p).bunrui as usize].id);
            } else {
                fprintf(output,
                        b"* \x00" as *const u8 as *const libc::c_char);
            }
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    (*mrph_p).bunrui as libc::c_int);
            if (*mrph_p).katuyou1 != 0 {
                fprintf(output,
                        b"%s \x00" as *const u8 as *const libc::c_char,
                        Type[(*mrph_p).katuyou1 as usize].name);
            } else {
                fprintf(output,
                        b"* \x00" as *const u8 as *const libc::c_char);
            }
            fprintf(output, b"%d \x00" as *const u8 as *const libc::c_char,
                    (*mrph_p).katuyou1 as libc::c_int);
            if (*mrph_p).katuyou2 != 0 {
                fprintf(output,
                        b"%s \x00" as *const u8 as *const libc::c_char,
                        Form[(*mrph_p).katuyou1 as
                                 usize][(*mrph_p).katuyou2 as usize].name);
            } else {
                fprintf(output,
                        b"* \x00" as *const u8 as *const libc::c_char);
            }
            fprintf(output, b"%d\x00" as *const u8 as *const libc::c_char,
                    (*mrph_p).katuyou2 as libc::c_int);
            if Show_Opt2 == 1 as libc::c_int {
                fprintf(output,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            } else {
                /* for SRI
	   fprintf(stdout, "\n");
	   if (para_flag) fprintf(stdout , "@ ");
	   */
                fprintf(output,
                        b" %s\n\x00" as *const u8 as *const libc::c_char,
                        (*mrph_p).imis.as_mut_ptr());
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn process_path_mrph(mut output: *mut FILE,
                                           mut path_num: libc::c_int,
                                           mut para_flag: libc::c_int) {
    if !output.is_null() {
        print_path_mrph(output, path_num, para_flag);
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if OutputAVnum == 0 as libc::c_int {
            OutputAVmax = 10 as libc::c_int;
            OutputAV =
                malloc((::std::mem::size_of::<*mut libc::c_char>() as
                            libc::c_ulong).wrapping_mul(OutputAVmax as
                                                            libc::c_ulong)) as
                    *mut *mut libc::c_char
        } else if OutputAVnum >= OutputAVmax - 1 as libc::c_int {
            OutputAVmax <<= 1 as libc::c_int;
            OutputAV =
                realloc(OutputAV as *mut libc::c_void,
                        (::std::mem::size_of::<*mut libc::c_char>() as
                             libc::c_ulong).wrapping_mul(OutputAVmax as
                                                             libc::c_ulong))
                    as *mut *mut libc::c_char
        }
        p = get_path_mrph(path_num, para_flag);
        if !p.is_null() {
            let fresh12 = OutputAVnum;
            OutputAVnum = OutputAVnum + 1;
            let ref mut fresh13 = *OutputAV.offset(fresh12 as isize);
            *fresh13 = p;
            let ref mut fresh14 = *OutputAV.offset(OutputAVnum as isize);
            *fresh14 = 0 as *mut libc::c_char
        }
    };
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <print_best_path> 後方最長一致のPATHを調べる
------------------------------------------------------------------------------
*/
/*
  サーバーモード対応のため、引数outputを増やして出力先の変更を可能にする。
  NACSIS 吉岡
*/
#[no_mangle]
pub unsafe extern "C" fn print_best_path(mut output: *mut FILE)
 -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    // let mut mrph_p: *mut MRPH = 0 as *mut MRPH;
    // let mut mrph_p1: *mut MRPH = 0 as *mut MRPH;
    j = 0 as libc::c_int;
    last = p_buffer_num - 1 as libc::c_int;
    loop  {
        last =
            (*p_buffer.offset(last as isize)).path[0 as libc::c_int as usize];
        *path_buffer.offset(j as isize) = last;
        j += 1;
        if !((*p_buffer.offset(last as isize)).path[0 as libc::c_int as usize]
                 != 0) {
            break ;
        }
    }
    /* 結果を buffer に入れる場合 */
    if output.is_null() {
        OutputAVnum = 0 as libc::c_int;
        OutputAVmax = 0 as libc::c_int
    }
    i = j - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        process_path_mrph(output, *path_buffer.offset(i as isize),
                          0 as libc::c_int);
        i -= 1
    }
    return OutputAV;
    /* 後でちゃんと free すること */
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <print_all_mrph> 正しい解析結果に含まれる全ての形態素
------------------------------------------------------------------------------
*/
/*
  サーバーモード対応のため、引数outputを増やして出力先の変更を可能にする。
  NACSIS 吉岡
*/
#[no_mangle]
pub unsafe extern "C" fn print_all_mrph(mut output: *mut FILE)
 -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    // let mut mrph: MRPH =
    //     MRPH{midasi: [0; 129],
    //          midasi2: [0; 129],
    //          yomi: [0; 129],
    //          imis: [0; 1024],
    //          imi: 0 as *mut CELL,
    //          hinsi: 0,
    //          bunrui: 0,
    //          katuyou1: 0,
    //          katuyou2: 0,
    //          weight: 0,
    //          con_tbl: 0,
    //          length: 0,};
    i = 0 as libc::c_int;
    while i < m_buffer_num {
        *m_check_buffer.offset(i as isize) = 0 as libc::c_int;
        i += 1
    }
    _print_all_mrph(output, p_buffer_num - 1 as libc::c_int);
    *m_check_buffer.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    /* 結果を buffer に入れる場合 */
    if output.is_null() {
        OutputAVnum = 0 as libc::c_int;
        OutputAVmax = 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < m_buffer_num {
        if *m_check_buffer.offset(i as isize) != 0 {
            process_path_mrph(output, i, 0 as libc::c_int);
        }
        i += 1
    }
    return OutputAV;
    /* 後でちゃんと free すること */
}
#[no_mangle]
pub unsafe extern "C" fn _print_all_mrph(mut output: *mut FILE,
                                         mut path_num: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (*p_buffer.offset(path_num as isize)).path[i as usize] !=
              -(1 as libc::c_int) {
        if *m_check_buffer.offset((*p_buffer.offset(path_num as
                                                        isize)).path[i as
                                                                         usize]
                                      as isize) == 0 {
            *m_check_buffer.offset((*p_buffer.offset(path_num as
                                                         isize)).path[i as
                                                                          usize]
                                       as isize) = 1 as libc::c_int;
            _print_all_mrph(output,
                            (*p_buffer.offset(path_num as
                                                  isize)).path[i as usize]);
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <print_all_path> 正しい解析結果に含まれる全てのPATH
------------------------------------------------------------------------------
*/
/*
  サーバーモード対応のため、引数outputを増やして出力先の変更を可能にする。
  NACSIS 吉岡
*/
#[no_mangle]
pub unsafe extern "C" fn print_all_path(mut output: *mut FILE)
 -> *mut *mut libc::c_char {
    /*  int i,j;
    for (i = 0 ; i < p_buffer_num ; i++) {
	printf("%d %s %d %d --- " , i , m_buffer[p_buffer[i].mrph_p].midasi ,
	       p_buffer[i].start , p_buffer[i].end);
	for (j = 0 ; p_buffer[i].path[j] != -1 ; j++)
	  printf("%d ",p_buffer[i].path[j]);
	printf("\n");
    }*/
    /* 結果を buffer に入れる場合 */
    if output.is_null() {
        OutputAVnum = 0 as libc::c_int;
        OutputAVmax = 0 as libc::c_int
    }
    _print_all_path(output, p_buffer_num - 1 as libc::c_int,
                    0 as libc::c_int);
    return OutputAV;
    /* 後でちゃんと free すること */
}
#[no_mangle]
pub unsafe extern "C" fn _print_all_path(mut output: *mut FILE,
                                         mut path_num: libc::c_int,
                                         mut pathes: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (*p_buffer.offset(path_num as isize)).path[i as usize] !=
              -(1 as libc::c_int) {
        if (*p_buffer.offset(path_num as
                                 isize)).path[0 as libc::c_int as usize] ==
               0 as libc::c_int {
            j = pathes - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                process_path_mrph(output, *path_buffer.offset(j as isize),
                                  0 as libc::c_int);
                j -= 1
            }
            if !output.is_null() {
                fprintf(output,
                        b"EOP\n\x00" as *const u8 as *const libc::c_char);
            }
        } else {
            *path_buffer.offset(pathes as isize) =
                (*p_buffer.offset(path_num as isize)).path[i as usize];
            _print_all_path(output,
                            (*p_buffer.offset(path_num as
                                                  isize)).path[i as usize],
                            pathes + 1 as libc::c_int);
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <print_homograph_path> 複数の候補をとれる形態素は列挙する
------------------------------------------------------------------------------
*/
/*
  サーバーモード対応のため、引数outputを増やして出力先の変更を可能にする。
  NACSIS 吉岡
*/
#[no_mangle]
pub unsafe extern "C" fn print_homograph_path(mut output: *mut FILE)
 -> *mut *mut libc::c_char {
    *path_buffer.offset(0 as libc::c_int as isize) =
        p_buffer_num - 1 as libc::c_int;
    *path_buffer.offset(1 as libc::c_int as isize) = -(1 as libc::c_int);
    /* 結果を buffer に入れる場合 */
    if output.is_null() {
        OutputAVnum = 0 as libc::c_int;
        OutputAVmax = 0 as libc::c_int
    }
    _print_homograph_path(output, 0 as libc::c_int, 2 as libc::c_int);
    return OutputAV;
    /* 後でちゃんと free すること */
}
#[no_mangle]
pub unsafe extern "C" fn _print_homograph_path(mut output: *mut FILE,
                                               mut pbuf_start: libc::c_int,
                                               mut new_p: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut now_pos: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut pt: libc::c_int = 0;
    let mut pt2: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    if (*p_buffer.offset(*path_buffer.offset(pbuf_start as isize) as
                             isize)).path[0 as libc::c_int as usize] ==
           0 as libc::c_int {
        /* 先頭までリンクをたどり終わった */
        j = new_p - 2 as libc::c_int;
        while j >= 1 as libc::c_int {
            /* 同音異義語群を一気に出力 */
            while *path_buffer.offset(j as isize) >= 0 as libc::c_int {
                j -= 1
            }
            k = j + 1 as libc::c_int;
            l = 0 as libc::c_int;
            while *path_buffer.offset(k as isize) >= 0 as libc::c_int {
                let fresh15 = l;
                l = l + 1;
                process_path_mrph(output, *path_buffer.offset(k as isize),
                                  fresh15);
                k += 1
            }
            j -= 1
        }
        if Show_Opt1 == 3 as libc::c_int { return 1 as libc::c_int }
        if !output.is_null() {
            fprintf(output, b"EOP\n\x00" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int
    }
    /* 特殊優先規則
           3文字の複合語は 2-1
	   4文字の複合語は 2-2
	   5文字の複合語は 3-2
       と分割されることが多いので，この分割を優先的に出力するようにする．
       そのため，
       ・2文字語の左に2,3,4文字語が連接している場合は2文字語を優先
       ・それ以外ならば文字数の少ない語を優先
       とする． */
    f = 0 as libc::c_int;
    now_pos =
        (*p_buffer.offset(*path_buffer.offset(pbuf_start as isize) as
                              isize)).start;
    j = pbuf_start;
    while *path_buffer.offset(j as isize) >= 0 as libc::c_int {
        i = 0 as libc::c_int;
        loop  {
            pt =
                (*p_buffer.offset(*path_buffer.offset(j as isize) as
                                      isize)).path[i as usize];
            if !(pt != -(1 as libc::c_int)) { break ; }
            // bestよりコストが高いパスは探索しない(コスト幅を使ったときにスコアの低い変なパスを選ばないため)
            if (*p_buffer.offset(pt as isize)).score >
                   (*p_buffer.offset((*p_buffer.offset(*path_buffer.offset(j
                                                                               as
                                                                               isize)
                                                           as
                                                           isize)).path[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                         as isize)).score {
                break ;
            }
            /* 2文字語を探す */
            if (*p_buffer.offset(pt as isize)).start ==
                   now_pos - 2 as libc::c_int * 3 as libc::c_int {
                k = 0 as libc::c_int;
                loop  {
                    pt2 = (*p_buffer.offset(pt as isize)).path[k as usize];
                    if !(pt2 != -(1 as libc::c_int)) { break ; }
                    /* 2文字語の左に連接している語が2〜4文字語か？ */
                    if (*p_buffer.offset(pt2 as isize)).start <=
                           now_pos -
                               (2 as libc::c_int + 2 as libc::c_int) *
                                   3 as libc::c_int &&
                           (*p_buffer.offset(pt2 as isize)).start >=
                               now_pos -
                                   (2 as libc::c_int + 4 as libc::c_int) *
                                       3 as libc::c_int {
                        f = 1 as libc::c_int
                    }
                    k += 1
                }
            }
            i += 1
        }
        j += 1
    }
    ll = 1 as libc::c_int;
    while ll <= now_pos {
        /* 半角文字のため，1byteごとに処理 */
        len = ll;
        if f != 0 {
            /* 1文字語と2文字語の優先順位を入れ替える */
            if ll == 3 as libc::c_int {
                len = 3 as libc::c_int * 2 as libc::c_int
            } else if ll == 3 as libc::c_int * 2 as libc::c_int {
                len = 3 as libc::c_int
            }
        }
        /* 同じ長さの形態素を全てリストアップする */
        l = new_p;
        j = pbuf_start;
        while *path_buffer.offset(j as isize) >= 0 as libc::c_int {
            i = 0 as libc::c_int;
            loop  {
                pt =
                    (*p_buffer.offset(*path_buffer.offset(j as isize) as
                                          isize)).path[i as usize];
                if !(pt != -(1 as libc::c_int)) { break ; }
                if (*p_buffer.offset(pt as isize)).score >
                       (*p_buffer.offset((*p_buffer.offset(*path_buffer.offset(j
                                                                                   as
                                                                                   isize)
                                                               as
                                                               isize)).path[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                             as isize)).score {
                    break ;
                }
                if (*p_buffer.offset(pt as isize)).start == now_pos - len {
                    /* 同音異義語群を求める(但し重複しないようにする) */
                    k = new_p; /* 同音異義語群のエンドマーク */
                    while k < l {
                        if *path_buffer.offset(k as isize) == pt { break ; }
                        k += 1
                    }
                    if k == l {
                        let fresh16 = l;
                        l = l + 1;
                        *path_buffer.offset(fresh16 as isize) = pt;
                        if l >= process_buffer_max {
                            realloc_process_buffer();
                        }
                    }
                }
                i += 1
            }
            j += 1
        }
        *path_buffer.offset(l as isize) = -(1 as libc::c_int);
        if l != new_p {
            if _print_homograph_path(output, new_p, l + 1 as libc::c_int) != 0
               {
                return 1 as libc::c_int
            }
        }
        ll += 1
    }
    return 0 as libc::c_int;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <pos_match_process> <pos_right_process>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn pos_match_process(mut pos: libc::c_int,
                                           mut p_start: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* 位置がマッチする p_buffer を抽出して，match_pbuf に書き並べる */
    j = 0 as libc::c_int;
    i = p_start;
    while i < p_buffer_num {
        if (*p_buffer.offset(i as isize)).end <= pos ||
               (*p_buffer.offset(i as isize)).connect == 0 as libc::c_int {
            if i == p_start {
                /* p_start 以前の p_buffer は十分 pos が小さいので，探索を省略 */
                p_start += 1
            }
            if (*p_buffer.offset(i as isize)).end == pos &&
                   (*p_buffer.offset(i as isize)).connect ==
                       (0 as libc::c_int == 0) as libc::c_int {
                let fresh17 = j;
                j = j + 1;
                *match_pbuf.offset(fresh17 as isize) = i
            }
        }
        i += 1
    }
    *match_pbuf.offset(j as isize) = -(1 as libc::c_int);
    return p_start;
}
#[no_mangle]
pub unsafe extern "C" fn pos_right_process(mut pos: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < p_buffer_num {
        if (*p_buffer.offset(i as isize)).end > pos {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <check_connect>			Changed by yamaji
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn check_connect(mut pos: libc::c_int,
                                       mut m_num: libc::c_int,
                                       mut opt: libc::c_char) -> libc::c_int {
    static mut chk_connect: [CHK_CONNECT_WK; 5000] =
        [CHK_CONNECT_WK{pre_p: 0, score: 0,};
            5000]; /* maximam value of int */
    let mut chk_con_num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pathes: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut best_score: libc::c_int = 0;
    let mut haba_score: libc::c_int = 0;
    let mut best_score_num: libc::c_int = 0;
    let mut c_score: libc::c_int = 0;
    let mut class_score: libc::c_int = 0;
    let mut new_mrph: *mut MRPH = 0 as *mut MRPH;
    let mut pre_mrph: *mut MRPH = 0 as *mut MRPH;
    let mut left_con: libc::c_int = 0;
    let mut right_con: libc::c_int = 0;
    let mut c_cache: *mut CONNECT_COST = 0 as *mut CONNECT_COST;
    new_mrph = &mut *m_buffer.offset(m_num as isize) as *mut MRPH;
    best_score = 2147483647 as libc::c_int;
    chk_con_num = 0 as libc::c_int;
    best_score_num = 0 as libc::c_int;
    class_score = best_score_num;
    score = class_score;
    class_score =
        Class[(*new_mrph).hinsi as usize][(*new_mrph).bunrui as usize].cost *
            (*new_mrph).weight as libc::c_int * cost_omomi.keitaiso;
    /* 連接キャッシュにヒットすれば，探索を行わない */
    c_cache =
        &mut *connect_cache.as_mut_ptr().offset((*rensetu_tbl.offset((*new_mrph).con_tbl
                                                                         as
                                                                         isize)).j_pos
                                                    as isize) as
            *mut CONNECT_COST;
    if Show_Opt_debug == 0 as libc::c_int {
        if (*c_cache).pos as libc::c_int == pos &&
               (*c_cache).p_no as libc::c_int > 0 as libc::c_int &&
               (*c_cache).opt as libc::c_int == opt as libc::c_int {
            (*p_buffer.offset(p_buffer_num as isize)).score =
                (*c_cache).cost + class_score;
            (*p_buffer.offset(p_buffer_num as isize)).mrph_p = m_num;
            (*p_buffer.offset(p_buffer_num as isize)).start = pos;
            (*p_buffer.offset(p_buffer_num as isize)).end =
                pos + (*new_mrph).length;
            i = 0 as libc::c_int;
            loop  {
                let ref mut fresh18 =
                    (*p_buffer.offset(p_buffer_num as
                                          isize)).path[i as usize];
                *fresh18 =
                    (*p_buffer.offset((*c_cache).p_no as
                                          isize)).path[i as usize];
                if !(*fresh18 >= 0 as libc::c_int) { break ; }
                i += 1
            }
            (*p_buffer.offset(p_buffer_num as isize)).path[i as usize] =
                -(1 as libc::c_int);
            (*p_buffer.offset(p_buffer_num as isize)).connect =
                (0 as libc::c_int == 0) as libc::c_int;
            p_buffer_num += 1;
            if p_buffer_num == process_buffer_max {
                realloc_process_buffer();
            }
            return (0 as libc::c_int == 0) as libc::c_int
        }
    }
    i = 0 as libc::c_int;
    loop  {
        j = *match_pbuf.offset(i as isize);
        if !(j >= 0 as libc::c_int) { break ; }
        /* 以前の形態素のcon_tblを取り出す */
        left_con =
            (*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p as
                                  isize)).con_tbl;
        /* 新しい形態素のcon_tblを取り出す */
        right_con = (*new_mrph).con_tbl;
        c_score = check_matrix(left_con, right_con);
        /* 濁音化するのは直前の形態素が名詞、または動詞の連用形、名詞性接尾辞の場合のみ */
	/* 接尾辞の場合を除き直前の形態素が平仮名1文字となるものは不可 */
        if opt as libc::c_int & 4 as libc::c_int != 0 &&
               (!((*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p as
                                        isize)).hinsi as libc::c_int ==
                      rendaku_hinsi1 &&
                      (*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p
                                            as isize)).katuyou2 as libc::c_int
                          == rendaku_renyou ||
                      (*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p
                                            as isize)).hinsi as libc::c_int ==
                          rendaku_hinsi2 &&
                          (*m_buffer.offset((*p_buffer.offset(j as
                                                                  isize)).mrph_p
                                                as isize)).bunrui as
                              libc::c_int != rendaku_bunrui2_3 ||
                      (*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p
                                            as isize)).hinsi as libc::c_int ==
                          rendaku_hinsi4 &&
                          ((*m_buffer.offset((*p_buffer.offset(j as
                                                                   isize)).mrph_p
                                                 as isize)).bunrui as
                               libc::c_int == rendaku_bunrui4_1 ||
                               (*m_buffer.offset((*p_buffer.offset(j as
                                                                       isize)).mrph_p
                                                     as isize)).bunrui as
                                   libc::c_int == rendaku_bunrui4_2 ||
                               (*m_buffer.offset((*p_buffer.offset(j as
                                                                       isize)).mrph_p
                                                     as isize)).bunrui as
                                   libc::c_int == rendaku_bunrui4_3 ||
                               (*m_buffer.offset((*p_buffer.offset(j as
                                                                       isize)).mrph_p
                                                     as isize)).bunrui as
                                   libc::c_int == rendaku_bunrui4_4)) ||
                    (*m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p as
                                          isize)).hinsi as libc::c_int !=
                        rendaku_hinsi4 &&
                        check_code((*m_buffer.offset((*p_buffer.offset(j as
                                                                           isize)).mrph_p
                                                         as
                                                         isize)).midasi.as_mut_ptr(),
                                   0 as libc::c_int) == 0xa5a0 as libc::c_int
                        &&
                        (*m_buffer.offset((*p_buffer.offset(j as
                                                                isize)).mrph_p
                                              as isize)).length ==
                            3 as libc::c_int) {
            c_score = 0 as libc::c_int
        }
        if c_score != 0 {
            chk_connect[chk_con_num as usize].pre_p = j;
            /* calculate the score */
            score =
                (*p_buffer.offset(j as isize)).score +
                    c_score * cost_omomi.rensetsu;
            chk_connect[chk_con_num as usize].score = score;
            if score < best_score {
                best_score = score;
                best_score_num = chk_con_num
            }
            chk_con_num += 1;
            if chk_con_num >= 5000 as libc::c_int { break ; }
        }
        /* デバグ用コスト表示 */
        if Show_Opt_debug == 2 as libc::c_int ||
               Show_Opt_debug == 1 as libc::c_int && c_score != 0 {
            fprintf(stderr, b"%3d \x00" as *const u8 as *const libc::c_char,
                    pos);
            pre_mrph =
                &mut *m_buffer.offset((*p_buffer.offset(j as isize)).mrph_p as
                                          isize) as *mut MRPH;
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    (*pre_mrph).midasi.as_mut_ptr());
            if !Class[(*pre_mrph).hinsi as
                          usize][0 as libc::c_int as usize].id.is_null() {
                fprintf(stderr, b"(\x00" as *const u8 as *const libc::c_char);
                if strcmp((*pre_mrph).midasi.as_mut_ptr() as
                              *const libc::c_char,
                          (*pre_mrph).midasi2.as_mut_ptr() as
                              *const libc::c_char) != 0 {
                    fprintf(stderr,
                            b"%s:\x00" as *const u8 as *const libc::c_char,
                            (*pre_mrph).midasi2.as_mut_ptr());
                }
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        Class[(*pre_mrph).hinsi as
                                  usize][0 as libc::c_int as usize].id);
                if (*pre_mrph).bunrui != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Class[(*pre_mrph).hinsi as
                                      usize][(*pre_mrph).bunrui as usize].id);
                }
                if (*pre_mrph).katuyou1 != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Type[(*pre_mrph).katuyou1 as usize].name);
                }
                if (*pre_mrph).katuyou2 != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Form[(*pre_mrph).katuyou1 as
                                     usize][(*pre_mrph).katuyou2 as
                                                usize].name);
                }
                fprintf(stderr, b")\x00" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"[= %d]\x00" as *const u8 as *const libc::c_char,
                    (*p_buffer.offset(j as isize)).score);
            if c_score != 0 {
                fprintf(stderr,
                        b"--[+%d*%d]--\x00" as *const u8 as
                            *const libc::c_char, c_score,
                        cost_omomi.rensetsu);
            } else {
                fprintf(stderr,
                        b"--XXX--\x00" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    (*new_mrph).midasi.as_mut_ptr());
            if !Class[(*new_mrph).hinsi as
                          usize][0 as libc::c_int as usize].id.is_null() {
                fprintf(stderr, b"(\x00" as *const u8 as *const libc::c_char);
                if strcmp((*new_mrph).midasi.as_mut_ptr() as
                              *const libc::c_char,
                          (*new_mrph).midasi2.as_mut_ptr() as
                              *const libc::c_char) != 0 {
                    fprintf(stderr,
                            b"%s:\x00" as *const u8 as *const libc::c_char,
                            (*new_mrph).midasi2.as_mut_ptr());
                }
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        Class[(*new_mrph).hinsi as
                                  usize][0 as libc::c_int as usize].id);
                if (*new_mrph).bunrui != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Class[(*new_mrph).hinsi as
                                      usize][(*new_mrph).bunrui as usize].id);
                }
                if (*new_mrph).katuyou1 != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Type[(*new_mrph).katuyou1 as usize].name);
                }
                if (*new_mrph).katuyou2 != 0 {
                    fprintf(stderr,
                            b"-%s\x00" as *const u8 as *const libc::c_char,
                            Form[(*new_mrph).katuyou1 as
                                     usize][(*new_mrph).katuyou2 as
                                                usize].name);
                }
                fprintf(stderr, b")\x00" as *const u8 as *const libc::c_char);
            }
            if c_score == 0 as libc::c_int {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(stderr,
                        b"[+%d*%d.%d*%d = %d]\n\x00" as *const u8 as
                            *const libc::c_char,
                        Class[(*new_mrph).hinsi as
                                  usize][(*new_mrph).bunrui as usize].cost,
                        (*new_mrph).weight as libc::c_int / 10 as libc::c_int,
                        (*new_mrph).weight as libc::c_int % 10 as libc::c_int,
                        cost_omomi.keitaiso * 10 as libc::c_int,
                        (*p_buffer.offset(j as isize)).score +
                            c_score * cost_omomi.rensetsu + class_score);
            }
        }
        i += 1
    }
    /* return immidiately, because if best_score is
       INT_MAX then no path exists. */
    if best_score == 2147483647 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    /* 今回の連接の様子を連接キャッシュに入れる */
    (*c_cache).p_no = p_buffer_num as libc::c_short;
    (*c_cache).cost = best_score;
    (*c_cache).pos = pos as libc::c_short;
    (*c_cache).opt = opt;
    /* 取り敢えずベストパスの1つを0番に登録 */
    (*p_buffer.offset(p_buffer_num as isize)).path[0 as libc::c_int as usize]
        = chk_connect[best_score_num as usize].pre_p;
    pathes = 1 as libc::c_int;
    haba_score = best_score + cost_omomi.cost_haba;
    j = 0 as libc::c_int;
    while j < chk_con_num {
        /* それ以外のパスも追加 */
        if chk_connect[j as usize].score <= haba_score && j != best_score_num
           {
            if pathes >= 500 as libc::c_int - 1 as libc::c_int { break ; }
            let fresh19 = pathes;
            pathes = pathes + 1;
            (*p_buffer.offset(p_buffer_num as isize)).path[fresh19 as usize] =
                chk_connect[j as usize].pre_p
        }
        j += 1
    }
    (*p_buffer.offset(p_buffer_num as isize)).path[pathes as usize] =
        -(1 as libc::c_int);
    (*p_buffer.offset(p_buffer_num as isize)).score =
        best_score + class_score;
    (*p_buffer.offset(p_buffer_num as isize)).mrph_p = m_num;
    (*p_buffer.offset(p_buffer_num as isize)).start = pos;
    (*p_buffer.offset(p_buffer_num as isize)).end = pos + (*new_mrph).length;
    (*p_buffer.offset(p_buffer_num as isize)).connect =
        (0 as libc::c_int == 0) as libc::c_int;
    p_buffer_num += 1;
    if p_buffer_num == process_buffer_max { realloc_process_buffer(); }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn make_new_node(mut current_char_node_ptr:
                                           *mut *mut CHAR_NODE,
                                       mut chr: *mut libc::c_char,
                                       mut type_0: libc::c_int)
 -> *mut CHAR_NODE {
    let mut i: libc::c_int = 0;
    let mut new_char_node: *mut CHAR_NODE =
        malloc(::std::mem::size_of::<CHAR_NODE>() as libc::c_ulong) as
            *mut CHAR_NODE;
    strcpy((*new_char_node).chr.as_mut_ptr(), chr);
    (*new_char_node).type_0 = type_0 as libc::c_char;
    (*new_char_node).da_node_pos_num = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        (*new_char_node).p_buffer[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    (**current_char_node_ptr).next = new_char_node;
    *current_char_node_ptr = new_char_node;
    CharLatticeUsedFlag = (0 as libc::c_int == 0) as libc::c_int;
    return new_char_node;
}
/*
------------------------------------------------------------------------------
  PROCEDURE: <juman_sent> 一文を形態素解析する    by T.Utsuro
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn juman_sent() -> libc::c_int {
    let mut i: libc::c_int =
        0; /* 直前の文字が削除文字(長音, 小文字) */
    let mut j: libc::c_int = 0;
    let mut pre_code: libc::c_int = 0;
    let mut post_code: libc::c_int = 0;
    // let mut pos_end: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut pre_p_buffer_num: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut next_pos: libc::c_int = 0 as libc::c_int;
    let mut pre_byte_length: libc::c_int = 0 as libc::c_int;
    let mut local_deleted_num: libc::c_int = 0 as libc::c_int;
    let mut p_start: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut code: libc::c_int = 0;
    let mut next_pre_is_deleted: libc::c_int = 0;
    let mut pre_is_deleted: libc::c_int = 0 as libc::c_int;
    let mut current_char_node: *mut CHAR_NODE = 0 as *mut CHAR_NODE;
    let mut new_char_node: *mut CHAR_NODE = 0 as *mut CHAR_NODE;
    if mrph_buffer_max == 0 as libc::c_int {
        m_buffer =
            my_alloc((::std::mem::size_of::<MRPH>() as
                          libc::c_ulong).wrapping_mul(1000 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int) as *mut MRPH;
        m_check_buffer =
            my_alloc((::std::mem::size_of::<libc::c_int>() as
                          libc::c_ulong).wrapping_mul(1000 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int) as *mut libc::c_int;
        mrph_buffer_max += 1000 as libc::c_int
    }
    if process_buffer_max == 0 as libc::c_int {
        p_buffer =
            my_alloc((::std::mem::size_of::<PROCESS_BUFFER>() as
                          libc::c_ulong).wrapping_mul(1000 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int) as *mut PROCESS_BUFFER;
        path_buffer =
            my_alloc((::std::mem::size_of::<libc::c_int>() as
                          libc::c_ulong).wrapping_mul(1000 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int) as *mut libc::c_int;
        match_pbuf =
            my_alloc((::std::mem::size_of::<libc::c_int>() as
                          libc::c_ulong).wrapping_mul(1000 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int) as *mut libc::c_int;
        process_buffer_max += 1000 as libc::c_int
    }
    /* 与えられた区切りを使う場合 */
    if UseGivenSegmentation_Opt != 0 {
        let mut token: *mut libc::c_char =
            0 as *mut libc::c_char; /* 空行はスキップ */
        let mut dup_String: *mut libc::c_char =
            strdup(String.as_mut_ptr() as *const libc::c_char);
        length = strlen(dup_String) as libc::c_int;
        String[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_uchar;
        pos = 0 as libc::c_int;
        while pos < length {
            String2Length[pos as usize] = 0 as libc::c_int;
            pos += 1
        }
        token =
            strtok(dup_String, b":\x00" as *const u8 as *const libc::c_char);
        while !token.is_null() {
            String2Length[count as usize] = strlen(token) as libc::c_int;
            count =
                (count as libc::c_ulong).wrapping_add(strlen(token)) as
                    libc::c_int as libc::c_int;
            strcat(String.as_mut_ptr() as *mut libc::c_char, token);
            token =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        free(dup_String as *mut libc::c_void);
    }
    length =
        strlen(String.as_mut_ptr() as *const libc::c_char) as libc::c_int;
    if length == 0 as libc::c_int { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        connect_cache[i as usize].p_no = 0 as libc::c_int as libc::c_short;
        i += 1
    }
    /* 文頭処理 */
    (*p_buffer.offset(0 as libc::c_int as isize)).end = 0 as libc::c_int;
    (*p_buffer.offset(0 as libc::c_int as
                          isize)).path[0 as libc::c_int as usize] =
        -(1 as libc::c_int);
    (*p_buffer.offset(0 as libc::c_int as isize)).score = 0 as libc::c_int;
    (*p_buffer.offset(0 as libc::c_int as isize)).mrph_p = 0 as libc::c_int;
    (*p_buffer.offset(0 as libc::c_int as isize)).connect =
        (0 as libc::c_int == 0) as libc::c_int;
    (*m_buffer.offset(0 as libc::c_int as isize)).hinsi =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(0 as libc::c_int as isize)).bunrui =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(0 as libc::c_int as isize)).con_tbl = 0 as libc::c_int;
    (*m_buffer.offset(0 as libc::c_int as isize)).weight =
        10 as libc::c_int as libc::c_uchar;
    strcpy((*m_buffer.offset(0 as libc::c_int as isize)).midasi.as_mut_ptr()
               as *mut libc::c_char,
           b"(\xe6\x96\x87\xe9\xa0\xad)\x00" as *const u8 as
               *const libc::c_char);
    m_buffer_num = 1 as libc::c_int;
    p_buffer_num = 1 as libc::c_int;
    /* initialization for root node (starting node for looking up double array) */
    CharRootNode.next = 0 as *mut _char_node;
    CharRootNode.da_node_pos[0 as libc::c_int as usize] =
        0 as libc::c_int as size_t;
    CharRootNode.deleted_bytes[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    CharRootNode.p_buffer[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    CharRootNode.da_node_pos_num = 1 as libc::c_int as size_t;
    CharLatticeUsedFlag = 0 as libc::c_int;
    CharNum = 0 as libc::c_int as size_t;
    pos = 0 as libc::c_int;
    while pos < length {
        current_char_node =
            &mut *CharLattice.as_mut_ptr().offset(CharNum as isize) as
                *mut CHAR_NODE;
        if String[pos as usize] as libc::c_int & 0x80 as libc::c_int != 0 {
            /* 全角の場合 */
            /* 長音記号による置換 */
        /* (String[pos]がーもしくは〜) && (posが文最後の文字列か次の文字が記号かひらがな) && 二文字目以降 */
            if LongSoundRep_Opt != 0 &&
                   (strncmp(String.as_mut_ptr().offset(pos as isize) as
                                *const libc::c_char,
                            b"\xe3\x83\xbc\x00" as *const u8 as
                                *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong) == 0 ||
                        strncmp(String.as_mut_ptr().offset(pos as isize) as
                                    *const libc::c_char,
                                b"\xe3\x80\x9c\x00" as *const u8 as
                                    *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong) == 0) &&
                   (String[(pos + 3 as libc::c_int) as usize] == 0 ||
                        check_code(String.as_mut_ptr(),
                                   pos + 3 as libc::c_int) ==
                            0xa3b0 as libc::c_int ||
                        check_code(String.as_mut_ptr(),
                                   pos + 3 as libc::c_int) ==
                            0xa5a0 as libc::c_int) && pos > 0 as libc::c_int {
                /* 2文字目以降 */
                /* 次の文字が"ー","〜"でない */
                /*  strncmp(String + pos + BYTES4CHAR, DEF_PROLONG_SYMBOL1, BYTES4CHAR) && */
                /*  strncmp(String + pos + BYTES4CHAR, DEF_PROLONG_SYMBOL2, BYTES4CHAR)) */
                i = 0 as libc::c_int;
                while *pre_prolonged[i as usize] != 0 {
                    if strncmp(String.as_mut_ptr().offset(pos as
                                                              isize).offset(-(pre_byte_length
                                                                                  as
                                                                                  isize))
                                   as *const libc::c_char,
                               pre_prolonged[i as usize] as
                                   *const libc::c_char,
                               pre_byte_length as libc::c_ulong) == 0 {
                        new_char_node =
                            make_new_node(&mut current_char_node,
                                          prolonged2chr[i as usize] as
                                              *mut libc::c_char,
                                          2 as libc::c_int |
                                              16 as libc::c_int);
                        break ;
                    } else { i += 1 }
                }
            } else if LowercaseRep_Opt != 0 {
                i = 0 as libc::c_int;
                while i < 7 as libc::c_int {
                    if strncmp(String.as_mut_ptr().offset(pos as isize) as
                                   *const libc::c_char,
                               lowercase[i as usize] as *const libc::c_char,
                               3 as libc::c_int as libc::c_ulong) == 0 {
                        new_char_node =
                            make_new_node(&mut current_char_node,
                                          uppercase[i as usize] as
                                              *mut libc::c_char,
                                          2 as libc::c_int);
                        break ;
                    } else { i += 1 }
                }
            }
            /* 小書き文字による置換 */
            /* 濁音化した文字の場合に、清音も文字を追加 (辞書展開によりオフに) */
            if Rendaku_Opt != 0 {
                code = check_code(String.as_mut_ptr(), pos);
                if pos > 0 as libc::c_int &&
                       (code == 0xa5a0 as libc::c_int ||
                            code == 0xa6a0 as libc::c_int &&
                                check_code(String.as_mut_ptr(),
                                           pos - pre_byte_length) !=
                                    0xa6a0 as libc::c_int &&
                                check_code(String.as_mut_ptr(),
                                           pos - pre_byte_length) !=
                                    0xa1bc as libc::c_int) {
                    i = 0 as libc::c_int;
                    while i < 40 as libc::c_int {
                        if strncmp(String.as_mut_ptr().offset(pos as isize) as
                                       *const libc::c_char,
                                   dakuon[i as usize] as *const libc::c_char,
                                   3 as libc::c_int as libc::c_ulong) == 0 {
                            new_char_node =
                                make_new_node(&mut current_char_node,
                                              seion[i as usize] as
                                                  *mut libc::c_char,
                                              4 as libc::c_int);
                            break ;
                        } else { i += 1 }
                    }
                }
            }
            next_pos = utf8_bytes(String.as_mut_ptr().offset(pos as isize))
        } else { next_pos = 1 as libc::c_int }
        strncpy(CharLattice[CharNum as usize].chr.as_mut_ptr(),
                String.as_mut_ptr().offset(pos as isize) as
                    *const libc::c_char, next_pos as libc::c_ulong);
        CharLattice[CharNum as usize].chr[next_pos as usize] =
            '\u{0}' as i32 as libc::c_char;
        CharLattice[CharNum as usize].type_0 =
            1 as libc::c_int as libc::c_char;
        next_pre_is_deleted = 0 as libc::c_int;
        /* 長音挿入 */
        if (LongSoundDel_Opt != 0 || LowercaseDel_Opt != 0) &&
               next_pos == 3 as libc::c_int {
            pre_code =
                if pos > 0 as libc::c_int {
                    check_code(String.as_mut_ptr(), pos - pre_byte_length)
                } else { -(1 as libc::c_int) }; /* 文末の場合は0 */
            post_code = check_code(String.as_mut_ptr(), pos + next_pos);
            if LongSoundDel_Opt != 0 && pre_code > 0 as libc::c_int &&
                   ((16 as libc::c_int + local_deleted_num + 1 as libc::c_int)
                        * 3 as libc::c_int) < 129 as libc::c_int &&
                   ((pre_is_deleted != 0 || pre_code == 0xa5a0 as libc::c_int
                         || pre_code == 0xa6a0 as libc::c_int ||
                         pre_code == 0xffff as libc::c_int &&
                             post_code == 0xa5a0 as libc::c_int) &&
                        (strncmp(String.as_mut_ptr().offset(pos as isize) as
                                     *const libc::c_char,
                                 b"\xe3\x83\xbc\x00" as *const u8 as
                                     *const libc::c_char,
                                 3 as libc::c_int as libc::c_ulong) == 0 ||
                             strncmp(String.as_mut_ptr().offset(pos as isize)
                                         as *const libc::c_char,
                                     b"\xe3\x80\x9c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3 as libc::c_int as libc::c_ulong) == 0))
                   ||
                   pre_is_deleted != 0 &&
                       strncmp(String.as_mut_ptr().offset(pos as isize) as
                                   *const libc::c_char,
                               b"\xe3\x81\xa3\x00" as *const u8 as
                                   *const libc::c_char,
                               3 as libc::c_int as libc::c_ulong) == 0 &&
                       (post_code == 0 as libc::c_int ||
                            post_code == 0xa3b0 as libc::c_int) {
                local_deleted_num += 1;
                next_pre_is_deleted = 1 as libc::c_int;
                new_char_node =
                    make_new_node(&mut current_char_node,
                                  b"\x00" as *const u8 as *const libc::c_char
                                      as *mut libc::c_char, 8 as libc::c_int)
            } else if LowercaseDel_Opt != 0 && pre_code > 0 as libc::c_int &&
                          ((16 as libc::c_int + local_deleted_num +
                                1 as libc::c_int) * 3 as libc::c_int) <
                              129 as libc::c_int {
                /* 小書き文字による長音化 */
                i = 0 as libc::c_int;
                while i < 5 as libc::c_int {
                    if strncmp(String.as_mut_ptr().offset(pos as isize) as
                                   *const libc::c_char,
                               lowercase[i as usize] as *const libc::c_char,
                               3 as libc::c_int as libc::c_ulong) == 0 {
                        j = pre_lower_start[i as usize];
                        while j < pre_lower_end[i as usize] {
                            if strncmp(String.as_mut_ptr().offset(pos as
                                                                      isize).offset(-(pre_byte_length
                                                                                          as
                                                                                          isize))
                                           as *const libc::c_char,
                                       pre_lower[j as usize] as
                                           *const libc::c_char,
                                       pre_byte_length as libc::c_ulong) == 0
                               {
                                break ;
                            }
                            j += 1
                        }
                        /* 直前の文字が対応する平仮名、または、削除された同一の小書き文字の場合は削除 */
                        if j < pre_lower_end[i as usize] ||
                               pre_is_deleted != 0 &&
                                   strncmp(String.as_mut_ptr().offset(pos as
                                                                          isize).offset(-(pre_byte_length
                                                                                              as
                                                                                              isize))
                                               as *const libc::c_char,
                                           String.as_mut_ptr().offset(pos as
                                                                          isize)
                                               as *const libc::c_char,
                                           pre_byte_length as libc::c_ulong)
                                       == 0 {
                            local_deleted_num += 1;
                            next_pre_is_deleted = 1 as libc::c_int;
                            new_char_node =
                                make_new_node(&mut current_char_node,
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              8 as libc::c_int);
                            break ;
                        }
                    }
                    i += 1
                }
            } else { local_deleted_num = 0 as libc::c_int }
        }
        pre_is_deleted = next_pre_is_deleted;
        pre_byte_length = next_pos;
        (*current_char_node).next = 0 as *mut _char_node;
        CharNum = CharNum.wrapping_add(1);
        pos += next_pos
    }
    count = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < length {
        p_start = pos_match_process(pos, p_start);
        if *match_pbuf.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
            pre_m_buffer_num = m_buffer_num;
            pre_p_buffer_num = p_buffer_num;
            if UseGivenSegmentation_Opt == 0 ||
                   String2Length[pos as usize] != 0 {
                if search_all(pos, count) == 0 as libc::c_int {
                    return 0 as libc::c_int
                }
                if undef_word(pos) == 0 as libc::c_int {
                    return 0 as libc::c_int
                }
            }
        }
        next_pos = utf8_bytes(String.as_mut_ptr().offset(pos as isize));
        count += 1;
        pos += next_pos
    }
    /* 文末処理 */
    strcpy((*m_buffer.offset(m_buffer_num as isize)).midasi.as_mut_ptr() as
               *mut libc::c_char,
           b"(\xe6\x96\x87\xe6\x9c\xab)\x00" as *const u8 as
               *const libc::c_char);
    (*m_buffer.offset(m_buffer_num as isize)).hinsi =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(m_buffer_num as isize)).bunrui =
        0 as libc::c_int as libc::c_char;
    (*m_buffer.offset(m_buffer_num as isize)).con_tbl = 0 as libc::c_int;
    (*m_buffer.offset(m_buffer_num as isize)).weight =
        10 as libc::c_int as libc::c_uchar;
    m_buffer_num += 1;
    if m_buffer_num == mrph_buffer_max { realloc_mrph_buffer(); }
    pos_match_process(pos, p_start);
    if check_connect(length, m_buffer_num - 1 as libc::c_int,
                     0 as libc::c_int as libc::c_char) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
