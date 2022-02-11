#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{configfile, ctools, db, structs, tools, types};
/*====================================================================

			 名詞・動詞相互情報量

                                        Daisuke Kawahara 2008. 10. 13

    $Id$
====================================================================*/
#[no_mangle]
pub static mut nv_mi_db: types::DBM_FILE = 0 as *const structs::CDB_FILE as *mut structs::CDB_FILE;
#[no_mangle]
pub static mut NV_MI_Exist: libc::c_int = 0;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_nv_mi() 
 /*==================================================================*/
 {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*tools::DICT.as_mut_ptr().offset(37 as libc::c_int as isize)).is_null() {
        filename =
            configfile::check_dict_filename(
                *tools::DICT.as_mut_ptr().offset(37 as libc::c_int as isize),
                (0 as libc::c_int == 0) as libc::c_int
            )
    } else {
        filename =
            configfile::check_dict_filename(
                b"auto/nv_mi.db\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int
            )
    }
    if tools::OptDisplay == 3 as libc::c_int {
        fprintf(
            ctools::Outfp,
            b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
            filename
        );
    }
    nv_mi_db = db::db_read_open(filename);
    if nv_mi_db.is_null() {
        if tools::OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char, ctools::Outfp);
        }
        NV_MI_Exist = 0 as libc::c_int
    } else {
        if tools::OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, ctools::Outfp);
        }
        NV_MI_Exist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn close_nv_mi() {
    if NV_MI_Exist == (0 as libc::c_int == 0) as libc::c_int {
        db::db_close(nv_mi_db);
    };
}

#[no_mangle]
pub unsafe extern "C" fn lookup_nv_mi(mut str: *mut libc::c_char) -> *mut libc::c_char {
    return db::db_get(nv_mi_db, str);
}

#[no_mangle]
pub unsafe extern "C" fn check_nv_mi(mut n_ptr: *mut types::TAG_DATA, mut v_ptr: *mut types::TAG_DATA) -> libc::c_int {
    // let mut ret: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    let mut given_verb_length: libc::c_int = 0;
    let mut dic_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut given_verb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut given_noun: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score: libc::c_double = 0.;
    if NV_MI_Exist == 0 as libc::c_int || n_ptr.is_null() ||
           (*n_ptr).head_ptr.is_null() || v_ptr.is_null() ||
           (*v_ptr).head_ptr.is_null() {
        return 2147483647 as libc::c_int
    }
    /* チェックする名詞からDBを引く */
    given_noun = ctools::get_mrph_rep_from_f((*n_ptr).head_ptr, 0 as libc::c_int);
    if given_noun.is_null() { return 2147483647 as libc::c_int }
    dic_str = lookup_nv_mi(given_noun);
    if !dic_str.is_null() {
        given_verb = ctools::get_mrph_rep_from_f((*v_ptr).head_ptr, 0 as libc::c_int);
        if given_verb.is_null() {
            /* チェックする動詞 */
            free(dic_str as *mut libc::c_void);
            return 2147483647 as libc::c_int
        }
        given_verb_length = strlen(given_verb) as libc::c_int;
        token = strtok(dic_str, b"|\x00" as *const u8 as *const libc::c_char);
        while !token.is_null() {
            if strncmp(token, given_verb, given_verb_length as libc::c_ulong)
                   == 0 {
                /* 与えられた動詞とDBがマッチ */
                num =
                    sscanf(token,
                           b"%*[^,],%d,%f\x00" as *const u8 as
                               *const libc::c_char,
                           &mut rank as *mut libc::c_int,
                           &mut score as *mut libc::c_double);
                if num != 2 as libc::c_int {
                    fprintf(ctools::stderr,
                            b";;; Invalid string in NV MI db <%s>.\n\x00" as
                                *const u8 as *const libc::c_char, token);
                    break ;
                } else { free(dic_str as *mut libc::c_void); return rank }
            } else {
                token =
                    strtok(0 as *mut libc::c_char,
                           b"|\x00" as *const u8 as *const libc::c_char)
            }
        }
        free(dic_str as *mut libc::c_void);
    }
    return 2147483647 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn check_nv_mi_parent_and_children(mut v_ptr: *mut types::TAG_DATA, mut rank_threshold: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut rank: libc::c_int = 2147483647 as libc::c_int;
    /* 子供の格要素をチェック */
    i = 0 as libc::c_int;
    while !(*v_ptr).child[i as usize].is_null() {
        if !ctools::check_feature((*(*v_ptr).child[i as usize]).f,
                          b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            rank = check_nv_mi((*v_ptr).child[i as usize], v_ptr);
            if rank < rank_threshold {
                return (0 as libc::c_int == 0) as libc::c_int
            }
        }
        i += 1
    }
    /* 連体修飾の親をチェック */
    if !(*v_ptr).parent.is_null() &&
           !ctools::check_feature((*v_ptr).f,
                          b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
        rank = check_nv_mi((*v_ptr).parent, v_ptr);
        if rank < rank_threshold {
            return (0 as libc::c_int == 0) as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*====================================================================
                               END
====================================================================*/
