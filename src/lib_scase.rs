#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! 表層格情報
use libc;

use crate::{_FEATURE, BNST_DATA, Class, FEATURE, fprintf, fputs, free, MRPH_DATA, sprintf, sscanf, strcat, strcmp, strcpy, strlen, strncmp, TAG_DATA, tnode_b};
use crate::ctools::{assign_cfeature, case2num, check_dict_filename, check_feature, db_close, db_get, db_read_open, DICT, make_pred_string, malloc_data, Outfp, stderr};
use crate::structs::CDB_FILE;
use crate::tools::{hiragana2katakana, OptCaseFlag, OptDisplay};
use crate::types::DBM_FILE;

#[no_mangle]
pub static mut scase_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut ScaseDicExist: libc::c_int = 0;
#[no_mangle]
pub static mut OptUseScase: libc::c_int = 0;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_scase()
/*==================================================================*/
{
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptUseScase == 0 as libc::c_int {
        ScaseDicExist = 0 as libc::c_int;
        return;
    }
    if !(*DICT.as_mut_ptr().offset(5 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(5 as libc::c_int as
                isize),
                                0 as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"gcf/scase.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                filename);
    }
    scase_db = db_read_open(filename);
    if scase_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        ScaseDicExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        ScaseDicExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_scase()
/*==================================================================*/
{
    if ScaseDicExist == (0 as libc::c_int == 0) as libc::c_int {
        db_close(scase_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_scase(mut cp: *mut libc::c_char)
                                   -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if ScaseDicExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
    value = db_get(scase_db, cp);
    return if !value.is_null() {
        i = 0 as libc::c_int;
        while *value.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
            let ref mut fresh0 = *value.offset(i as isize);
            *fresh0 = (*fresh0 as libc::c_int - '0' as i32) as libc::c_char;
            i += 1
        }
        value
    } else { 0 as *mut libc::c_char };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mrph2case(mut bp: *mut BNST_DATA)
                                   -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = (*bp).mrph_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                          b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                usize][0 as libc::c_int as usize].id as
                          *const libc::c_char,
                      b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                          *const libc::c_char) == 0 &&
                strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                    usize][(*(*bp).mrph_ptr.offset(i as
                    isize)).Bunrui
                    as usize].id as
                           *const libc::c_char,
                       b"\xe6\xa0\xbc\xe5\x8a\xa9\xe8\xa9\x9e\x00" as
                           *const u8 as *const libc::c_char) == 0 {
                return (*(*bp).mrph_ptr.offset(i as isize)).Goi.as_mut_ptr();
            }
        } else { return 0 as *mut libc::c_char; }
        i -= 1
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_pred_string_for_scase(mut bp: *mut BNST_DATA)
                                                    -> *mut libc::c_char
/*==================================================================*/
{
    let mut buffer: *mut libc::c_char =
        0 as *mut libc::c_char; /* OptCaseFlag & OPT_CASE_USE_REP_CF */
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cbp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    verb =
        make_pred_string(bp as *mut TAG_DATA, 0 as *mut MRPH_DATA,
                         0 as *mut libc::c_char, 0 as libc::c_int,
                         0 as libc::c_int, 0 as libc::c_int);
    /* cbp = get_quasi_closest_case_component((TAG_DATA *)bp, 
       bp->num < 1 ? NULL : (TAG_DATA *)(bp - 1)); */
    return if (*bp).num > 0 as libc::c_int {
        cbp = bp.offset(-(1 as libc::c_int as isize));
        pp = mrph2case(cbp);
        if !pp.is_null() {
            let mut pp_katakana: *mut libc::c_char =
                hiragana2katakana(pp as *mut libc::c_uchar) as
                    *mut libc::c_char;
            buffer =
                malloc_data(strlen((*(*cbp).head_ptr).Goi.as_mut_ptr()).wrapping_add(strlen(pp_katakana)).wrapping_add(strlen(verb)).wrapping_add(10
                    as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"make_pred_string_for_scase\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            sprintf(buffer,
                    b"%s:%s-%s\x00" as *const u8 as *const libc::c_char,
                    (*(*cbp).head_ptr).Goi.as_mut_ptr(), pp_katakana, verb);
            free(verb as *mut libc::c_void);
            free(pp_katakana as *mut libc::c_void);
            buffer
        } else { verb }
    } else { verb };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn or_scase_code(mut dst: *mut *mut libc::c_char,
                                       mut src: *mut libc::c_char)
/*==================================================================*/
{
    if (*dst).is_null() {
        *dst = src
    } else if !src.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while *(*dst).offset(i as isize) as libc::c_int != '\u{0}' as i32 {
            let ref mut fresh1 = *(*dst).offset(i as isize);
            *fresh1 =
                (*fresh1 as libc::c_int |
                    *src.offset(i as isize) as libc::c_int) as libc::c_char;
            i += 1
        }
        free(src as *mut libc::c_void);
    };
    /* *dstがあってsrcがないときはなにもしない */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_scase_code(mut ptr: *mut BNST_DATA)
/*==================================================================*/
{
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ans: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut anscp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut voice: [libc::c_char; 4] = [0; 4];
    /* 初期化: init_bnst でもしている */
    i = 0 as libc::c_int;
    cp = (*ptr).SCASE_code.as_mut_ptr();
    while i < 11 as libc::c_int {
        *cp = 0 as libc::c_int as libc::c_char;
        i += 1;
        cp = cp.offset(1)
    }
    if ScaseDicExist == (0 as libc::c_int == 0) as libc::c_int &&
        {
            vtype =
                check_feature((*ptr).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8
                                  as *const libc::c_char as
                                  *mut libc::c_char);
            !vtype.is_null()
        } &&
        strcmp(vtype,
               b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as *const u8 as
                   *const libc::c_char) != 0 {
        /* 判定詞ではない場合 */
        vtype =
            vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as *const u8
                as *const libc::c_char) as isize);
        voice[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        if (*ptr).voice & 2 as libc::c_int != 0 {
            strcpy(voice.as_mut_ptr(),
                   b":P\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).voice & 1 as libc::c_int != 0 {
            strcpy(voice.as_mut_ptr(),
                   b":C\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).voice & 4 as libc::c_int != 0 {
            strcpy(voice.as_mut_ptr(),
                   b":PC\x00" as *const u8 as *const libc::c_char);
        }
        /* まず、直前格要素との組で検索 *
	str_buffer = make_pred_string_for_scase(ptr);
	strcat(str_buffer, ":");
	strcat(str_buffer, vtype);
	if (voice[0]) strcat(str_buffer, voice);

	ans = get_scase(str_buffer);
	*/
        if ans.is_null() {
            /* なければ、用言だけで検索 */
            if !str_buffer.is_null() {
                free(str_buffer as *mut libc::c_void);
            }
            str_buffer =
                make_pred_string(ptr as *mut TAG_DATA, 0 as *mut MRPH_DATA,
                                 0 as *mut libc::c_char,
                                 OptCaseFlag & 32 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int);
            strcat(str_buffer, b":\x00" as *const u8 as *const libc::c_char);
            strcat(str_buffer, vtype);
            if voice[0 as libc::c_int as usize] != 0 {
                strcat(str_buffer, voice.as_mut_ptr());
            }
            ans = get_scase(str_buffer);
            /* 代表表記が曖昧な用言の場合 */
            if !check_feature((*(*ptr).head_ptr).f,
                              b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                let mut fp: *mut FEATURE = 0 as *mut FEATURE;
                let mut m: MRPH_DATA =
                    MRPH_DATA {
                        type_0: 0,
                        num: 0,
                        parent: 0 as *mut tnode_b,
                        child: [0 as *mut tnode_b; 32],
                        length: 0,
                        space: 0,
                        dpnd_head: 0,
                        dpnd_type: 0,
                        dpnd_dflt: 0,
                        para_num: 0,
                        para_key_type: 0,
                        para_top_p: 0,
                        para_type: 0,
                        to_para_p: 0,
                        tnum: 0,
                        inum: 0,
                        out_head_flag: 0,
                        Goi: [0; 129],
                        Yomi: [0; 129],
                        Goi2: [0; 129],
                        Hinshi: 0,
                        Bunrui: 0,
                        Katuyou_Kata: 0,
                        Katuyou_Kei: 0,
                        Imi: [0; 1024],
                        f: 0 as *mut _FEATURE,
                        Num: 0,
                        SM: 0 as *mut libc::c_char,
                        Pos: [0; 4],
                        Type: [0; 9],
                    };
                // let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut new_ans: *mut libc::c_char = 0 as *mut libc::c_char;
                fp = (*(*ptr).head_ptr).f;
                while !fp.is_null() {
                    if strncmp((*fp).cp,
                               b"ALT-\x00" as *const u8 as
                                   *const libc::c_char,
                               4 as libc::c_int as libc::c_ulong) == 0 {
                        sscanf((*fp).cp.offset(4 as libc::c_int as isize),
                               b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00" as
                                   *const u8 as *const libc::c_char,
                               m.Goi2.as_mut_ptr(), m.Yomi.as_mut_ptr(),
                               m.Goi.as_mut_ptr(),
                               &mut m.Hinshi as *mut libc::c_int,
                               &mut m.Bunrui as *mut libc::c_int,
                               &mut m.Katuyou_Kata as *mut libc::c_int,
                               &mut m.Katuyou_Kei as *mut libc::c_int,
                               m.Imi.as_mut_ptr());
                        free(str_buffer as *mut libc::c_void);
                        str_buffer =
                            make_pred_string(ptr as *mut TAG_DATA, &mut m,
                                             0 as *mut libc::c_char,
                                             OptCaseFlag & 32 as libc::c_int,
                                             0 as libc::c_int,
                                             0 as libc::c_int);
                        strcat(str_buffer,
                               b":\x00" as *const u8 as *const libc::c_char);
                        strcat(str_buffer, vtype);
                        if voice[0 as libc::c_int as usize] != 0 {
                            strcat(str_buffer, voice.as_mut_ptr());
                        }
                        new_ans = get_scase(str_buffer);
                        or_scase_code(&mut ans, new_ans);
                        /* ORで足す */
                    }
                    fp = (*fp).next
                }
            }
        }
        if !ans.is_null() {
            /* DEBUG 表示 */
            if OptDisplay == 3 as libc::c_int {
                let mut print_buffer: *mut libc::c_char =
                    0 as *mut libc::c_char;
                print_buffer =
                    malloc_data(strlen(str_buffer).wrapping_add(10 as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"get_scase_code\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                sprintf(print_buffer,
                        b"SCASEUSE:%s\x00" as *const u8 as
                            *const libc::c_char, str_buffer);
                assign_cfeature(&mut (*ptr).f, print_buffer,
                                0 as libc::c_int);
                free(print_buffer as *mut libc::c_void);
            }
            cp = (*ptr).SCASE_code.as_mut_ptr();
            anscp = ans;
            i = 0 as libc::c_int;
            while i < 11 as libc::c_int {
                let fresh2 = anscp;
                anscp = anscp.offset(1);
                let fresh3 = cp;
                cp = cp.offset(1);
                *fresh3 = *fresh2;
                i += 1
            }
            free(ans as *mut libc::c_void);
            free(str_buffer as *mut libc::c_void);
            current_block = 17189961260979847415;
        } else {
            if OptDisplay == 3 as libc::c_int {
                fprintf(stderr,
                        b";; Cannot find SCASE: %s\n\x00" as *const u8 as
                            *const libc::c_char, str_buffer);
            }
            free(str_buffer as *mut libc::c_void);
            current_block = 16203797167131938757;
        }
    } else { current_block = 16203797167131938757; }
    match current_block {
        16203797167131938757 => {
            /* 判定詞などの場合,
       表層格辞書がない場合, 
       または辞書にない用言の場合 */
            if !check_feature((*ptr).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                (*ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char
            } else if !check_feature((*ptr).f,
                                     b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_char).is_null() {
                (*ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char;
                (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char
                /* 形容詞の表層格の付与は副作用が多いので制限
	ptr->SCASE_code[case2num("ヨリ格")] = 1;
	ptr->SCASE_code[case2num("ト格")] = 1;
	*/
            } else if !check_feature((*ptr).f,
                                     b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_char).is_null() {
                (*ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char;
                (*ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char;
                (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char;
                (*ptr).SCASE_code[case2num(b"\xe3\x83\x98\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char;
                (*ptr).SCASE_code[case2num(b"\xe3\x83\x88\xe6\xa0\xbc\x00" as
                    *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) as usize] =
                    1 as libc::c_int as libc::c_char
            }
        }
        _ => {}
    }
    /* ヴォイスによる修正 */
    if (*ptr).voice & 1 as libc::c_int != 0 {
        (*ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char as
            *mut libc::c_char) as usize] =
            1 as libc::c_int as libc::c_char;
        (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char as
            *mut libc::c_char) as usize] =
            1 as libc::c_int as libc::c_char
    } else if (*ptr).voice & 2 as libc::c_int != 0 ||
        (*ptr).voice & 4 as libc::c_int != 0 {
        (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char as
            *mut libc::c_char) as usize] =
            1 as libc::c_int as libc::c_char
    } else if (*ptr).voice & 8 as libc::c_int != 0 ||
        (*ptr).voice & 16 as libc::c_int != 0 {
        (*ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char as
            *mut libc::c_char) as usize] =
            1 as libc::c_int as libc::c_char;
        (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char as
            *mut libc::c_char) as usize] =
            1 as libc::c_int as libc::c_char
    };
}
/*====================================================================
                               END
====================================================================*/
