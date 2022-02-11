#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{dic, structs, tools, types};
use crate::ctools::{exit, stderr};
use crate::feature::{feature_pattern_match, list2feature_pattern};
use crate::structs::{BnstRule, FEATURE_PATTERN, MrphRule, REGEXPBNST, REGEXPBNSTS, REGEXPMRPH, REGEXPMRPHS};
use crate::types::{CELL, DBM_FILE, FEATURE, TAG_DATA};

#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const tools::CDB_FILE as *mut tools::CDB_FILE;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
#[no_mangle]
pub static mut matched_ptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;

/* マッチした形態素or文節のポインタの記憶 */
// Initialized in run_static_initializers
#[no_mangle]
pub static mut RegexpMrphInitValue: REGEXPMRPH =
    REGEXPMRPH {
        type_flag: 0,
        ast_flag: 0,
        Hinshi_not: 0,
        Hinshi: [0; 64],
        Bunrui_not: 0,
        Bunrui: [0; 64],
        Kata_not: 0,
        Katuyou_Kata: [0; 64],
        Kei_not: 0,
        Katuyou_Kei:
        [0 as *const libc::c_char as *mut libc::c_char; 64],
        Goi_not: 0,
        Goi: [0 as *const libc::c_char as *mut libc::c_char; 64],
        f_pattern: FEATURE_PATTERN { fp: [0 as *mut FEATURE; 16] },
    };
#[no_mangle]
pub static mut RegexpmrphsInitValue: REGEXPMRPHS =
    {
        let mut init =
            REGEXPMRPHS {
                mrph: 0 as *const REGEXPMRPH as *mut REGEXPMRPH,
                mrphsize: 0 as libc::c_int as libc::c_char,
            };
        init
    };
// Initialized in run_static_initializers
#[no_mangle]
pub static mut RegexpBnstInitValue: REGEXPBNST =
    REGEXPBNST {
        type_flag: 0,
        ast_flag: 0,
        mrphs: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS,
        f_pattern: FEATURE_PATTERN { fp: [0 as *mut FEATURE; 16] },
    };
/*==================================================================*/
/*   		              ストア                                */
/*==================================================================*/
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpmrph_alloc(mut size: libc::c_int)
                                          -> *mut REGEXPMRPH
/*==================================================================*/
{
    let mut tmp: *mut REGEXPMRPH = 0 as *mut REGEXPMRPH;
    let mut i: libc::c_int = 0;
    tmp =
        malloc((size as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<REGEXPMRPH>()
            as libc::c_ulong)) as
            *mut REGEXPMRPH;
    if tmp.is_null() {
        fprintf(stderr,
                b"Can\'t allocate memory for REGEXPMRPH\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    i = 0 as libc::c_int;
    while i < size {
        *tmp.offset(i as isize) = RegexpMrphInitValue;
        i += 1
    }
    return tmp;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpmrphs_alloc() -> *mut REGEXPMRPHS
/*==================================================================*/
{
    let mut tmp: *mut REGEXPMRPHS = 0 as *mut REGEXPMRPHS;
    tmp =
        malloc(::std::mem::size_of::<REGEXPMRPHS>() as libc::c_ulong) as
            *mut REGEXPMRPHS;
    if tmp.is_null() {
        fprintf(stderr,
                b"Can\'t allocate memory for REGEXPMRPHS\n\x00" as *const u8
                    as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    return tmp;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpbnst_alloc(mut size: libc::c_int)
                                          -> *mut REGEXPBNST
/*==================================================================*/
{
    let mut tmp: *mut REGEXPBNST = 0 as *mut REGEXPBNST;
    let mut i: libc::c_int = 0;
    tmp =
        malloc((size as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<REGEXPBNST>()
            as libc::c_ulong)) as
            *mut REGEXPBNST;
    if tmp.is_null() {
        fprintf(stderr,
                b"Can\'t allocate memory for REGEXPBNST\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    i = 0 as libc::c_int;
    while i < size {
        *tmp.offset(i as isize) = RegexpBnstInitValue;
        i += 1
    }
    return tmp;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpbnsts_alloc() -> *mut REGEXPBNSTS
/*==================================================================*/
{
    let mut tmp: *mut REGEXPBNSTS = 0 as *mut REGEXPBNSTS;
    tmp =
        malloc(::std::mem::size_of::<REGEXPBNSTS>() as libc::c_ulong) as
            *mut REGEXPBNSTS;
    if tmp.is_null() {
        fprintf(stderr,
                b"Can\'t allocate memory for REGEXPBNSTS\n\x00" as *const u8
                    as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    return tmp;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_mrph_nflag(mut mp: *mut REGEXPMRPH,
                                          mut type_0: libc::c_int)
/*==================================================================*/
{
    match type_0 {
        1 => { (*mp).Hinshi_not = '^' as i32 as libc::c_char }
        2 => { (*mp).Bunrui_not = '^' as i32 as libc::c_char }
        3 => { (*mp).Kata_not = '^' as i32 as libc::c_char }
        4 => { (*mp).Kei_not = '^' as i32 as libc::c_char }
        5 => { (*mp).Goi_not = '^' as i32 as libc::c_char }
        _ => {}
    };
}
/*==================================================================*/
unsafe extern "C" fn store_mrph_item(mut mp: *mut REGEXPMRPH,
                                     mut mcell: *mut CELL,
                                     mut type_0: libc::c_int) -> *mut CELL
/*==================================================================*/
{
    let mut nth: libc::c_int = 0; /* "(...)" */
    let mut list_cell: *mut CELL = 0 as *mut CELL;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if car(mcell).is_null() { return 0 as *mut CELL; }
    if !car(mcell).is_null() && (*car(mcell)).tag == 1 as libc::c_int {
        tmp = (*car(mcell)).value.atom as *mut libc::c_char;
        if strcmp(tmp, b"*\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "*" */
            return cdr(mcell);
        } else {
            if strcmp(tmp, b"^\x00" as *const u8 as *const libc::c_char) == 0
            {
                /* "^" */
                store_mrph_nflag(mp, type_0);
                mcell = cdr(mcell);
                if !car(mcell).is_null() &&
                    (*car(mcell)).tag == 1 as libc::c_int {
                    list_cell =
                        cons(car(mcell) as *mut libc::c_void,
                             0 as *mut libc::c_void)
                } else { list_cell = car(mcell) }
            } else if strncmp(tmp,
                              b"^\x00" as *const u8 as *const libc::c_char,
                              strlen(b"^\x00" as *const u8 as
                                  *const libc::c_char)) == 0 {
                /* "^atom" */
                store_mrph_nflag(mp, type_0);
                let ref mut fresh0 = (*car(mcell)).value.atom;
                *fresh0 =
                    (*fresh0).offset(strlen(b"^\x00" as *const u8 as
                        *const libc::c_char) as
                        isize);
                list_cell =
                    cons(car(mcell) as *mut libc::c_void,
                         0 as *mut libc::c_void)
            } else {
                /* "atom" */
                list_cell =
                    cons(car(mcell) as *mut libc::c_void,
                         0 as *mut libc::c_void)
            }
        }
    } else { list_cell = car(mcell) }
    if type_0 == 2 as libc::c_int &&
        (*mp).Hinshi[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        fprintf(stderr,
                b"Cannot restrict Bunrui for multiple Hinshis.\n\x00" as
                    *const u8 as *const libc::c_char);
        error_in_lisp();
    }
    nth = 0 as libc::c_int;
    while !car(list_cell).is_null() {
        match type_0 {
            1 => {
                (*mp).Hinshi[nth as usize] =
                    get_hinsi_id((*car(list_cell)).value.atom)
            }
            2 => {
                (*mp).Bunrui[nth as usize] =
                    get_bunrui_id((*car(list_cell)).value.atom,
                                  (*mp).Hinshi[0 as libc::c_int as usize])
            }
            3 => {
                (*mp).Katuyou_Kata[nth as usize] =
                    get_type_id((*car(list_cell)).value.atom)
            }
            4 => {
                tmp = (*car(list_cell)).value.atom as *mut libc::c_char;
                (*mp).Katuyou_Kei[nth as usize] =
                    malloc(strlen(tmp).wrapping_add(1 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                        as
                        libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*mp).Katuyou_Kei[nth as usize], tmp);
            }
            5 => {
                tmp = (*car(list_cell)).value.atom as *mut libc::c_char;
                (*mp).Goi[nth as usize] =
                    malloc(strlen(tmp).wrapping_add(1 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                        as
                        libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*mp).Goi[nth as usize], tmp);
            }
            _ => {}
        }
        list_cell = cdr(list_cell);
        nth += 1
    }
    return cdr(mcell);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_regexpmrph(mut mp: *mut REGEXPMRPH,
                                          mut mcell: *mut CELL)
                                          -> libc::c_char
/*==================================================================*/
{
    /* 形態素が特殊文字による指定の場合 */
    return if !mcell.is_null() && (*mcell).tag == 1 as libc::c_int {
        if strcmp((*mcell).value.atom as *const libc::c_char,
                  b"^\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "^" */
            (*mp).type_flag = '^' as i32 as libc::c_char;
            '^' as i32 as libc::c_char;
        } else if strcmp((*mcell).value.atom as *const libc::c_char,
                         b"?\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "?" */
            (*mp).type_flag = '?' as i32 as libc::c_char;
            '?' as i32 as libc::c_char;
        } else if strcmp((*mcell).value.atom as *const libc::c_char,
                         b"*\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "*" */
            (*mp.offset(-(1 as libc::c_int as isize))).ast_flag =
                '*' as i32 as libc::c_char;
            '*' as i32 as libc::c_char;
        } else if strcmp((*mcell).value.atom as *const libc::c_char,
                         b"?*\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "?*" */
            (*mp).type_flag = '?' as i32 as libc::c_char;
            (*mp).ast_flag = '*' as i32 as libc::c_char;
            '?' as i32 as libc::c_char;
        } else {
            fprintf(stderr,
                    b"Invalid string for meta mrph (%s).\n\x00" as *const u8
                        as *const libc::c_char, (*mcell).value.atom);
            error_in_lisp();
            0 as *mut libc::c_void as libc::c_char;
        }
    } else {
        /* 形態素が通常の指定の場合 */
        (*mp).f_pattern.fp[0 as libc::c_int as usize] = 0 as *mut FEATURE;
        mcell = store_mrph_item(mp, mcell, 1 as libc::c_int);
        if mcell.is_null() { return '\u{0}' as i32 as libc::c_char; }
        mcell = store_mrph_item(mp, mcell, 2 as libc::c_int);
        if mcell.is_null() { return '\u{0}' as i32 as libc::c_char; }
        mcell = store_mrph_item(mp, mcell, 3 as libc::c_int);
        if mcell.is_null() { return '\u{0}' as i32 as libc::c_char; }
        mcell = store_mrph_item(mp, mcell, 4 as libc::c_int);
        if mcell.is_null() { return '\u{0}' as i32 as libc::c_char; }
        mcell = store_mrph_item(mp, mcell, 5 as libc::c_int);
        if mcell.is_null() { return '\u{0}' as i32 as libc::c_char; }
        list2feature_pattern(&mut (*mp).f_pattern, car(mcell));
        if cdr(mcell).is_null() {
            '\u{0}' as i32 as libc::c_char
        } else {
            fprintf(stderr,
                    b"Invalid string for NOT_FLAG.\n\x00" as *const u8 as
                        *const libc::c_char);
            error_in_lisp();
            0 as *mut libc::c_void as libc::c_char
        };
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_regexpmrphs(mut mspp: *mut *mut REGEXPMRPHS,
                                           mut cell: *mut CELL)
/*==================================================================*/
{
    let mut mrph_num: libc::c_int = 0 as libc::c_int;
    if cell.is_null() {
        *mspp = 0 as *mut REGEXPMRPHS;
        return; }
    *mspp = regexpmrphs_alloc();
    (**mspp).mrph = regexpmrph_alloc(length(cell));
    while !cell.is_null() {
        match store_regexpmrph((**mspp).mrph.offset(mrph_num as isize),
                               car(cell)) as libc::c_int {
            0 | 63 => { mrph_num += 1 }
            94 | 42 | _ => {}
        }
        cell = cdr(cell)
    }
    (**mspp).mrphsize = mrph_num as libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_regexpbnst(mut bp: *mut REGEXPBNST,
                                          mut cell: *mut CELL)
                                          -> libc::c_char
/*==================================================================*/
{
    /* 文節が特殊文字による指定の場合 */
    return if !cell.is_null() && (*cell).tag == 1 as libc::c_int {
        if strcmp((*cell).value.atom as *const libc::c_char,
                  b"^\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "^" */
            (*bp).type_flag = '^' as i32 as libc::c_char;
            '^' as i32 as libc::c_char;
        } else if strcmp((*cell).value.atom as *const libc::c_char,
                         b"?\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "?" */
            (*bp).type_flag = '?' as i32 as libc::c_char;
            '?' as i32 as libc::c_char;
        } else if strcmp((*cell).value.atom as *const libc::c_char,
                         b"*\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "*" */
            (*bp.offset(-(1 as libc::c_int as isize))).ast_flag =
                '*' as i32 as libc::c_char;
            '*' as i32 as libc::c_char;
        } else if strcmp((*cell).value.atom as *const libc::c_char,
                         b"?*\x00" as *const u8 as *const libc::c_char) == 0 {
            /* "?*" */
            (*bp).type_flag = '?' as i32 as libc::c_char;
            (*bp).ast_flag = '*' as i32 as libc::c_char;
            '?' as i32 as libc::c_char;
        } else {
            fprintf(stderr,
                    b"Invalid string for meta bnst (%s).\n\x00" as *const u8
                        as *const libc::c_char, (*cell).value.atom);
            error_in_lisp();
            0 as *mut libc::c_void as libc::c_char;
        }
    } else {
        /* 文節が通常の指定の場合 */
        store_regexpmrphs(&mut (*bp).mrphs, car(cell));
        if !cdr(cell).is_null() {
            list2feature_pattern(&mut (*bp).f_pattern, car(cdr(cell)));
        } else {
            (*bp).f_pattern.fp[0 as libc::c_int as usize] = 0 as *mut FEATURE
        }
        '\u{0}' as i32 as libc::c_char;
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_regexpbnsts(mut bspp: *mut *mut REGEXPBNSTS,
                                           mut cell: *mut CELL)
/*==================================================================*/
{
    let mut bnst_num: libc::c_int = 0 as libc::c_int;
    if cell.is_null() {
        *bspp = 0 as *mut REGEXPBNSTS;
        return; }
    *bspp = regexpbnsts_alloc();
    (**bspp).bnst = regexpbnst_alloc(length(cell));
    while !cell.is_null() {
        match store_regexpbnst((**bspp).bnst.offset(bnst_num as isize),
                               car(cell)) as libc::c_int {
            0 | 63 => { bnst_num += 1 }
            94 | 42 | _ => {}
        }
        cell = cdr(cell)
    }
    (**bspp).bnstsize = bnst_num as libc::c_char;
}
/*==================================================================*/
/*   		            マッチング                              */
/*==================================================================*/
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn rule_HBK_cmp(mut flg: libc::c_char,
                                      mut r_data: *mut libc::c_int,
                                      mut data: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* 品詞、細分類，活用型のマッチング */
    let mut i: libc::c_int = 0;
    let mut tmp_ret: libc::c_int = 0 as libc::c_int;
    return if *r_data.offset(0 as libc::c_int as isize) == -(1 as libc::c_int) ||
        *r_data.offset(0 as libc::c_int as isize) == 0 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while *r_data.offset(i as isize) != -(1 as libc::c_int) {
            if *r_data.offset(i as isize) == data {
                tmp_ret = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else { i += 1 }
        }
        if flg as libc::c_int == '\u{0}' as i32 &&
            tmp_ret == (0 as libc::c_int == 0) as libc::c_int ||
            flg as libc::c_int == '^' as i32 && tmp_ret == 0 as libc::c_int
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn rule_Kei_cmp(mut flg: libc::c_char,
                                      mut r_string: *mut *mut libc::c_char,
                                      mut kata: libc::c_int,
                                      mut kei: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* 活用形のマッチング */
    let mut i: libc::c_int = 0;
    let mut tmp_ret: libc::c_int = 0 as libc::c_int;
    return if (*r_string.offset(0 as libc::c_int as isize)).is_null() ||
        strcmp(*r_string.offset(0 as libc::c_int as isize),
               b"*\x00" as *const u8 as *const libc::c_char) == 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else if kata == 0 as libc::c_int || kei == 0 as libc::c_int ||
        kata >= 128 as libc::c_int || kei >= 128 as libc::c_int ||
        Form[kata as usize][kei as usize].name.is_null() {
        0 as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while !(*r_string.offset(i as isize)).is_null() {
            if strcmp(*r_string.offset(i as isize),
                      Form[kata as usize][kei as usize].name as
                          *const libc::c_char) == 0 {
                tmp_ret = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else { i += 1 }
        }
        if flg as libc::c_int == '\u{0}' as i32 &&
            tmp_ret == (0 as libc::c_int == 0) as libc::c_int ||
            flg as libc::c_int == '^' as i32 && tmp_ret == 0 as libc::c_int
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    };
}

#[no_mangle]
pub unsafe extern "C" fn mrph_check_function(mut rule: *mut libc::c_char, mut data: *mut libc::c_char) -> libc::c_int {
    // let mut i: libc::c_int = 0;
    // let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    fprintf(
        stderr,
        b";; Invalid Mrph-Feature-Function (%s)\n\x00" as *const u8 as *const libc::c_char,
        rule,
    );
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn rule_Goi_cmp(mut flg: libc::c_char, mut r_string: *mut *mut libc::c_char, mut d_string: *mut libc::c_char) -> libc::c_int {
    /* 語彙のマッチング */
    let mut i: libc::c_int = 0;
    let mut tmp_ret: libc::c_int = 0 as libc::c_int;
    return if (*r_string.offset(0 as libc::c_int as isize)).is_null() ||
        strcmp(*r_string.offset(0 as libc::c_int as isize),
               b"*\x00" as *const u8 as *const libc::c_char) == 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while !(*r_string.offset(i as isize)).is_null() {
            /* 関数呼び出し
	    if (r_string[i][0] == '&')
		if (mrph_check_function(r_string[i], d_string)) {
		    tmp_ret = TRUE;
		    break;
		} */
            if strcmp(*r_string.offset(i as isize), d_string) == 0 {
                tmp_ret = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else { i += 1 }
        }
        if flg as libc::c_int == '\u{0}' as i32 &&
            tmp_ret == (0 as libc::c_int == 0) as libc::c_int ||
            flg as libc::c_int == '^' as i32 && tmp_ret == 0 as libc::c_int
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpmrph_match(mut ptr1: *mut REGEXPMRPH, mut ptr2: *mut structs::MRPH_DATA) -> libc::c_int {
    /* 形態素のマッチング */
    let mut ret_mrph: libc::c_int = 0;
    /* '?' */
    return if (*ptr1).type_flag as libc::c_int == '?' as i32 {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        if rule_HBK_cmp((*ptr1).Hinshi_not, (*ptr1).Hinshi.as_mut_ptr(),
                        (*ptr2).Hinshi) != 0 &&
            rule_HBK_cmp((*ptr1).Bunrui_not, (*ptr1).Bunrui.as_mut_ptr(),
                         (*ptr2).Bunrui) != 0 &&
            rule_HBK_cmp((*ptr1).Kata_not,
                         (*ptr1).Katuyou_Kata.as_mut_ptr(),
                         (*ptr2).Katuyou_Kata) != 0 &&
            rule_Kei_cmp((*ptr1).Kei_not, (*ptr1).Katuyou_Kei.as_mut_ptr(),
                         (*ptr2).Katuyou_Kata, (*ptr2).Katuyou_Kei) != 0 &&
            rule_Goi_cmp((*ptr1).Goi_not, (*ptr1).Goi.as_mut_ptr(),
                         (*ptr2).Goi.as_mut_ptr()) != 0 {
            ret_mrph = (0 as libc::c_int == 0) as libc::c_int
        } else { ret_mrph = 0 as libc::c_int }
        if (*ptr1).type_flag as libc::c_int == '\u{0}' as i32 &&
            ret_mrph == (0 as libc::c_int == 0) as libc::c_int ||
            (*ptr1).type_flag as libc::c_int == '^' as i32 &&
                ret_mrph == 0 as libc::c_int {
            feature_pattern_match(&mut (*ptr1).f_pattern, (*ptr2).f,
                                  0 as *mut libc::c_void,
                                  ptr2 as *mut libc::c_void)
        } else { 0 as libc::c_int }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpmrphs_match(mut r_ptr: *mut REGEXPMRPH,
                                           mut r_num: libc::c_int,
                                           mut d_ptr: *mut structs::MRPH_DATA,
                                           mut d_num: libc::c_int,
                                           mut fw_or_bw: libc::c_int,
                                           mut all_or_part: libc::c_int,
                                           mut short_or_long: libc::c_int)
                                           -> libc::c_int
/*==================================================================*/
{
    /* 形態素列に対してもfeatureを与えられるように変更 99/04/09 */
    let mut step: libc::c_int = 0;
    let mut return_num: libc::c_int = 0;
    if fw_or_bw == 0 as libc::c_int {
        step = 1 as libc::c_int
    } else { step = -(1 as libc::c_int) };
    return if r_num == 0 as libc::c_int {
        if d_num == 0 as libc::c_int || all_or_part == 1 as libc::c_int {
            d_num
        } else { -(1 as libc::c_int) }
    } else if (*r_ptr).ast_flag as libc::c_int == '*' as i32 {
        /* 
	       パターンに"condition*"がある場合，次の可能性を調べる

	        1. パターンのみ進める(パターンの"*"をスキップ)
	        2. データのみ進める(conditionがデータとマッチすれば)

	       1を先にすればSHORT_MATCHING, 2を先にすればLONG_MATCHING
	    */
        if short_or_long == 0 as libc::c_int {
            return_num =
                regexpmrphs_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int, d_ptr, d_num,
                                  fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else if d_num != 0 && regexpmrph_match(r_ptr, d_ptr) != 0 &&
                {
                    return_num =
                        regexpmrphs_match(r_ptr, r_num,
                                          d_ptr.offset(step as
                                              isize),
                                          d_num - 1 as libc::c_int,
                                          fw_or_bw, all_or_part,
                                          short_or_long);
                    (return_num) != -(1 as libc::c_int)
                } {
                return_num
            } else { -(1 as libc::c_int) }
        } else if d_num != 0 && regexpmrph_match(r_ptr, d_ptr) != 0 &&
            {
                return_num =
                    regexpmrphs_match(r_ptr, r_num,
                                      d_ptr.offset(step as isize),
                                      d_num - 1 as libc::c_int,
                                      fw_or_bw, all_or_part,
                                      short_or_long);
                (return_num) != -(1 as libc::c_int)
            } {
            return_num
        } else {
            return_num =
                regexpmrphs_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int, d_ptr, d_num,
                                  fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else { -(1 as libc::c_int) }
        }
    } else if d_num != 0 && regexpmrph_match(r_ptr, d_ptr) != 0 &&
        {
            return_num =
                regexpmrphs_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int,
                                  d_ptr.offset(step as isize),
                                  d_num - 1 as libc::c_int,
                                  fw_or_bw, all_or_part,
                                  short_or_long);
            (return_num) != -(1 as libc::c_int)
        } {
        return_num
    } else { -(1 as libc::c_int) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpmrphrule_match(mut r_ptr: *mut MrphRule,
                                              mut d_ptr: *mut structs::MRPH_DATA,
                                              mut bw_length: libc::c_int,
                                              mut fw_length: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    /* 
       pre_pattern  (shortest match でよい)
       self_pattern (longest match  がよい)
       post_pattern (shortest match でよい)
       
       まず，pre_patternを調べ，次にself_patternのlongest matchから
       順に，その後でpost_patternを調べる
    */
    let mut match_length: libc::c_int =
        0; /* マッチした形態素or文節のポインタの記憶の初期化 */
    let mut match_rest: libc::c_int = 0;
    matched_ptr = 0 as *mut libc::c_void;
    /* まず，pre_patternを調べる */
    if (*r_ptr).pre_pattern.is_null() && bw_length != 0 as libc::c_int ||
        !(*r_ptr).pre_pattern.is_null() &&
            regexpmrphs_match((*(*r_ptr).pre_pattern).mrph.offset((*(*r_ptr).pre_pattern).mrphsize
                as
                libc::c_int
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize)),
                              (*(*r_ptr).pre_pattern).mrphsize as
                                  libc::c_int,
                              d_ptr.offset(-(1 as libc::c_int as isize)),
                              bw_length, 1 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) ==
                -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    /* 次にself_patternのlongest matchから順に，その後でpost_patternを調べる
       match_length は self_pattern の match の(可能性の)長さ */
    match_length = fw_length; /* 違い */
    while match_length > 0 as libc::c_int {
        if (*r_ptr).self_pattern.is_null() {
            match_length = 1 as libc::c_int
            /* self_pattern がなければ
				   マッチの長さは1にしておく */
        } else {
            match_rest =
                regexpmrphs_match((*(*r_ptr).self_pattern).mrph,
                                  (*(*r_ptr).self_pattern).mrphsize as
                                      libc::c_int, d_ptr, match_length,
                                  0 as libc::c_int, 1 as libc::c_int,
                                  1 as libc::c_int);
            if match_rest != -(1 as libc::c_int) {
                match_length -= match_rest
            } else { return -(1 as libc::c_int); }
        }
        if (*r_ptr).post_pattern.is_null() ||
            regexpmrphs_match((*(*r_ptr).post_pattern).mrph,
                              (*(*r_ptr).post_pattern).mrphsize as
                                  libc::c_int,
                              d_ptr.offset(match_length as isize),
                              fw_length - match_length, 0 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) !=
                -(1 as libc::c_int) {
            return match_length;
        }
        match_length -= 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _regexpbnst_match(mut r_ptr: *mut REGEXPMRPHS,
                                           mut b_ptr: *mut types::BNST_DATA)
                                           -> libc::c_int
/*==================================================================*/
{
    /* 将来はいらない */
    return regexpmrphs_match((*r_ptr).mrph, (*r_ptr).mrphsize as libc::c_int,
                             (*b_ptr).mrph_ptr, (*b_ptr).mrph_num,
                             0 as libc::c_int, 0 as libc::c_int,
                             0 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpbnst_match(mut ptr1: *mut REGEXPBNST,
                                          mut ptr2: *mut types::BNST_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    /* 文節のマッチング */
    let mut ret_mrph: libc::c_int = 0;
    /* '?' */
    return if (*ptr1).type_flag as libc::c_int == '?' as i32 {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        if regexpmrphs_match((*(*ptr1).mrphs).mrph,
                             (*(*ptr1).mrphs).mrphsize as libc::c_int,
                             (*ptr2).mrph_ptr, (*ptr2).mrph_num,
                             0 as libc::c_int, 0 as libc::c_int,
                             0 as libc::c_int) != -(1 as libc::c_int) {
            ret_mrph = (0 as libc::c_int == 0) as libc::c_int
        } else { ret_mrph = 0 as libc::c_int }
        if (*ptr1).type_flag as libc::c_int == '\u{0}' as i32 &&
            ret_mrph == (0 as libc::c_int == 0) as libc::c_int ||
            (*ptr1).type_flag as libc::c_int == '^' as i32 &&
                ret_mrph == 0 as libc::c_int {
            feature_pattern_match(&mut (*ptr1).f_pattern, (*ptr2).f,
                                  0 as *mut libc::c_void,
                                  ptr2 as *mut libc::c_void)
        } else { 0 as libc::c_int }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpbnsts_match(mut r_ptr: *mut REGEXPBNST,
                                           mut r_num: libc::c_int,
                                           mut d_ptr: *mut types::BNST_DATA,
                                           mut d_num: libc::c_int,
                                           mut fw_or_bw: libc::c_int,
                                           mut all_or_part: libc::c_int,
                                           mut short_or_long: libc::c_int)
                                           -> libc::c_int
/*==================================================================*/
{
    let mut step: libc::c_int = 0;
    let mut return_num: libc::c_int = 0;
    if fw_or_bw == 0 as libc::c_int {
        step = 1 as libc::c_int
    } else { step = -(1 as libc::c_int) };
    return if r_num == 0 as libc::c_int {
        if d_num == 0 as libc::c_int || all_or_part == 1 as libc::c_int {
            d_num
        } else { -(1 as libc::c_int) }
    } else if (*r_ptr).ast_flag as libc::c_int == '*' as i32 {
        /* 
	       パターンに"condition*"がある場合，次の可能性を調べる

	        1. パターンのみ進める(パターンの"*"をスキップ)
	        2. データのみ進める(conditionがデータとマッチすれば)

	       1を先にすればSHORT_MATCHING, 2を先にすればLONG_MATCHING
	    */
        if short_or_long == 0 as libc::c_int {
            return_num =
                regexpbnsts_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int, d_ptr, d_num,
                                  fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else if d_num != 0 && regexpbnst_match(r_ptr, d_ptr) != 0 &&
                {
                    return_num =
                        regexpbnsts_match(r_ptr, r_num,
                                          d_ptr.offset(step as
                                              isize),
                                          d_num - 1 as libc::c_int,
                                          fw_or_bw, all_or_part,
                                          short_or_long);
                    (return_num) != -(1 as libc::c_int)
                } {
                return_num
            } else { -(1 as libc::c_int) }
        } else if d_num != 0 && regexpbnst_match(r_ptr, d_ptr) != 0 &&
            {
                return_num =
                    regexpbnsts_match(r_ptr, r_num,
                                      d_ptr.offset(step as isize),
                                      d_num - 1 as libc::c_int,
                                      fw_or_bw, all_or_part,
                                      short_or_long);
                (return_num) != -(1 as libc::c_int)
            } {
            return_num
        } else {
            return_num =
                regexpbnsts_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int, d_ptr, d_num,
                                  fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else { -(1 as libc::c_int) }
        }
    } else if d_num != 0 && regexpbnst_match(r_ptr, d_ptr) != 0 &&
        {
            return_num =
                regexpbnsts_match(r_ptr.offset(step as isize),
                                  r_num - 1 as libc::c_int,
                                  d_ptr.offset(step as isize),
                                  d_num - 1 as libc::c_int,
                                  fw_or_bw, all_or_part,
                                  short_or_long);
            (return_num) != -(1 as libc::c_int)
        } {
        return_num
    } else { -(1 as libc::c_int) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexpbnstrule_match(mut r_ptr: *mut BnstRule,
                                              mut d_ptr: *mut types::BNST_DATA,
                                              mut bw_length: libc::c_int,
                                              mut fw_length: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    /* 
       pre_pattern  (shortest match でよい)
       self_pattern (longest match  がよい)
       post_pattern (shortest match でよい)
       
       まず，pre_patternを調べ，次にself_patternのlongest matchから
       順に，その後でpost_patternを調べる
    */
    let mut match_length: libc::c_int =
        0; /* マッチした形態素or文節のポインタの記憶の初期化 */
    let mut match_rest: libc::c_int = 0;
    matched_ptr = 0 as *mut libc::c_void;
    /* まず，pre_patternを調べる */
    if (*r_ptr).pre_pattern.is_null() && bw_length != 0 as libc::c_int ||
        !(*r_ptr).pre_pattern.is_null() &&
            regexpbnsts_match((*(*r_ptr).pre_pattern).bnst.offset((*(*r_ptr).pre_pattern).bnstsize
                as
                libc::c_int
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize)),
                              (*(*r_ptr).pre_pattern).bnstsize as
                                  libc::c_int,
                              d_ptr.offset(-(1 as libc::c_int as isize)),
                              bw_length, 1 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) ==
                -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    /* 次にself_patternのlongest matchから順に，その後でpost_patternを調べる
       match_length は self_pattern の match の(可能性の)長さ */
    match_length = fw_length; /* 違い */
    while match_length > 0 as libc::c_int {
        if (*r_ptr).self_pattern.is_null() {
            match_length = 1 as libc::c_int
            /* self_pattern がなければ
				   マッチの長さは1にしておく */
        } else {
            match_rest =
                regexpbnsts_match((*(*r_ptr).self_pattern).bnst,
                                  (*(*r_ptr).self_pattern).bnstsize as
                                      libc::c_int, d_ptr, match_length,
                                  0 as libc::c_int, 1 as libc::c_int,
                                  1 as libc::c_int);
            if match_rest != -(1 as libc::c_int) {
                match_length -= match_rest
            } else { return -(1 as libc::c_int); }
        }
        if (*r_ptr).post_pattern.is_null() ||
            regexpbnsts_match((*(*r_ptr).post_pattern).bnst,
                              (*(*r_ptr).post_pattern).bnstsize as
                                  libc::c_int,
                              d_ptr.offset(match_length as isize),
                              fw_length - match_length, 0 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) !=
                -(1 as libc::c_int) {
            return match_length;
        }
        match_length -= 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexptags_match(mut r_ptr: *mut REGEXPBNST,
                                          mut r_num: libc::c_int,
                                          mut d_ptr: *mut TAG_DATA,
                                          mut d_num: libc::c_int,
                                          mut fw_or_bw: libc::c_int,
                                          mut all_or_part: libc::c_int,
                                          mut short_or_long: libc::c_int)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut step: libc::c_int = 0;
    let mut return_num: libc::c_int = 0;
    if fw_or_bw == 0 as libc::c_int {
        step = 1 as libc::c_int
    } else { step = -(1 as libc::c_int) };
    return if r_num == 0 as libc::c_int {
        if d_num == 0 as libc::c_int || all_or_part == 1 as libc::c_int {
            d_num
        } else { -(1 as libc::c_int) }
    } else if (*r_ptr).ast_flag as libc::c_int == '*' as i32 {
        /* 
	       パターンに"condition*"がある場合，次の可能性を調べる

	        1. パターンのみ進める(パターンの"*"をスキップ)
	        2. データのみ進める(conditionがデータとマッチすれば)

	       1を先にすればSHORT_MATCHING, 2を先にすればLONG_MATCHING
	    */
        if short_or_long == 0 as libc::c_int {
            return_num =
                regexptags_match(r_ptr.offset(step as isize),
                                 r_num - 1 as libc::c_int, d_ptr, d_num,
                                 fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else if d_num != 0 &&
                regexpbnst_match(r_ptr, d_ptr as *mut types::BNST_DATA) !=
                    0 &&
                {
                    return_num =
                        regexptags_match(r_ptr, r_num,
                                         d_ptr.offset(step as
                                             isize),
                                         d_num - 1 as libc::c_int,
                                         fw_or_bw, all_or_part,
                                         short_or_long);
                    (return_num) != -(1 as libc::c_int)
                } {
                return_num
            } else { -(1 as libc::c_int) }
        } else if d_num != 0 &&
            regexpbnst_match(r_ptr, d_ptr as *mut types::BNST_DATA) != 0 &&
            {
                return_num =
                    regexptags_match(r_ptr, r_num,
                                     d_ptr.offset(step as isize),
                                     d_num - 1 as libc::c_int,
                                     fw_or_bw, all_or_part,
                                     short_or_long);
                (return_num) != -(1 as libc::c_int)
            } {
            return_num
        } else {
            return_num =
                regexptags_match(r_ptr.offset(step as isize),
                                 r_num - 1 as libc::c_int, d_ptr, d_num,
                                 fw_or_bw, all_or_part, short_or_long);
            if return_num != -(1 as libc::c_int) {
                return_num
            } else { -(1 as libc::c_int) }
        }
    } else if d_num != 0 &&
        regexpbnst_match(r_ptr, d_ptr as *mut types::BNST_DATA) != 0 &&
        {
            return_num =
                regexptags_match(r_ptr.offset(step as isize),
                                 r_num - 1 as libc::c_int,
                                 d_ptr.offset(step as isize),
                                 d_num - 1 as libc::c_int, fw_or_bw,
                                 all_or_part, short_or_long);
            (return_num) != -(1 as libc::c_int)
        } {
        return_num
    } else { -(1 as libc::c_int) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn regexptagrule_match(mut r_ptr: *mut BnstRule,
                                             mut d_ptr: *mut TAG_DATA,
                                             mut bw_length: libc::c_int,
                                             mut fw_length: libc::c_int)
                                             -> libc::c_int
/*==================================================================*/
{
    /* 
       pre_pattern  (shortest match でよい)
       self_pattern (longest match  がよい)
       post_pattern (shortest match でよい)
       
       まず，pre_patternを調べ，次にself_patternのlongest matchから
       順に，その後でpost_patternを調べる
    */
    let mut match_length: libc::c_int =
        0; /* マッチした形態素or文節のポインタの記憶の初期化 */
    let mut match_rest: libc::c_int = 0;
    matched_ptr = 0 as *mut libc::c_void;
    /* まず，pre_patternを調べる */
    if (*r_ptr).pre_pattern.is_null() && bw_length != 0 as libc::c_int ||
        !(*r_ptr).pre_pattern.is_null() &&
            regexptags_match((*(*r_ptr).pre_pattern).bnst.offset((*(*r_ptr).pre_pattern).bnstsize
                as
                libc::c_int
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize)),
                             (*(*r_ptr).pre_pattern).bnstsize as
                                 libc::c_int,
                             d_ptr.offset(-(1 as libc::c_int as isize)),
                             bw_length, 1 as libc::c_int, 0 as libc::c_int,
                             0 as libc::c_int) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    /* 次にself_patternのlongest matchから順に，その後でpost_patternを調べる
       match_length は self_pattern の match の(可能性の)長さ */
    match_length = fw_length; /* 違い */
    while match_length > 0 as libc::c_int {
        if (*r_ptr).self_pattern.is_null() {
            match_length = 1 as libc::c_int
            /* self_pattern がなければ
				   マッチの長さは1にしておく */
        } else {
            match_rest =
                regexptags_match((*(*r_ptr).self_pattern).bnst,
                                 (*(*r_ptr).self_pattern).bnstsize as
                                     libc::c_int, d_ptr, match_length,
                                 0 as libc::c_int, 1 as libc::c_int,
                                 1 as libc::c_int);
            if match_rest != -(1 as libc::c_int) {
                match_length -= match_rest
            } else { return -(1 as libc::c_int); }
        }
        if (*r_ptr).post_pattern.is_null() ||
            regexptags_match((*(*r_ptr).post_pattern).bnst,
                             (*(*r_ptr).post_pattern).bnstsize as
                                 libc::c_int,
                             d_ptr.offset(match_length as isize),
                             fw_length - match_length, 0 as libc::c_int,
                             0 as libc::c_int, 0 as libc::c_int) !=
                -(1 as libc::c_int) {
            return match_length;
        }
        match_length -= 1
    }
    return -(1 as libc::c_int);
}

unsafe extern "C" fn run_static_initializers() {
    RegexpMrphInitValue =
        {
            let mut init =
                REGEXPMRPH {
                    type_flag: '\u{0}' as i32 as libc::c_char,
                    ast_flag: 0 as *mut libc::c_void as libc::c_char,
                    Hinshi_not: 0 as *mut libc::c_void as libc::c_char,
                    Hinshi:
                    [0 as libc::c_int, -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int)],
                    Bunrui_not: 0 as *mut libc::c_void as libc::c_char,
                    Bunrui:
                    [0 as libc::c_int, -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int)],
                    Kata_not: 0 as *mut libc::c_void as libc::c_char,
                    Katuyou_Kata:
                    [0 as libc::c_int, -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int),
                        -(1 as libc::c_int), -(1 as libc::c_int)],
                    Kei_not: 0 as *mut libc::c_void as libc::c_char,
                    Katuyou_Kei:
                    [0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char],
                    Goi_not: 0 as *mut libc::c_void as libc::c_char,
                    Goi:
                    [0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char],
                    f_pattern:
                    FEATURE_PATTERN {
                        fp:
                        [0 as *mut FEATURE;
                            16],
                    },
                };
            init
        };
    RegexpBnstInitValue =
        {
            let mut init =
                REGEXPBNST {
                    type_flag: '\u{0}' as i32 as libc::c_char,
                    ast_flag: 0 as *mut libc::c_void as libc::c_char,
                    mrphs:
                    0 as *mut libc::c_void as libc::c_char as
                        *mut REGEXPMRPHS,
                    f_pattern:
                    FEATURE_PATTERN {
                        fp:
                        [0 as *mut FEATURE;
                            16],
                    },
                };
            init
        }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/*====================================================================
				 END
====================================================================*/
