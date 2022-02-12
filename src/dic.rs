#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{db, fprintf, fputs, free, MRPH_DATA, sprintf, strcat, strcmp, strlen, strncmp};
use crate::ctools::{assign_cfeature, check_dict_filename, get_mrph_rep_from_f, malloc_data, Outfp, strtok};
use crate::structs::CDB_FILE;
use crate::tools::{DICT, OptDisplay};
use crate::types::DBM_FILE;

#[no_mangle]
pub static mut auto_dic_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut AutoDicExist: libc::c_int = 0;
#[no_mangle]
pub static mut used_auto_dic_features: [*mut libc::c_char; 10] = [0 as *const libc::c_char as *mut libc::c_char; 10];
#[no_mangle]
pub static mut used_auto_dic_features_num: libc::c_int = 0 as libc::c_int;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_auto_dic()
/*==================================================================*/
{
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*DICT.as_mut_ptr().offset(26 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(26 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"auto/auto.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                filename);
    }
    auto_dic_db = db::db_read_open(filename);
    if auto_dic_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        AutoDicExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        AutoDicExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_auto_dic()
/*==================================================================*/
{
    if AutoDicExist == (0 as libc::c_int == 0) as libc::c_int {
        db::db_close(auto_dic_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn lookup_auto_dic(mut str: *mut libc::c_char)
                                         -> *mut libc::c_char
/*==================================================================*/
{
    return db::db_get(auto_dic_db, str);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_auto_dic(mut m_ptr: *mut MRPH_DATA,
                                        mut assign_pos: libc::c_int,
                                        mut m_length: libc::c_int,
                                        mut rule_value: *mut libc::c_char,
                                        mut temp_assign_flag: libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    // let mut length: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dic_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 5120] = [0; 5120];
    if AutoDicExist == 0 as libc::c_int { return 0 as libc::c_int; }
    if used_auto_dic_features_num > 0 as libc::c_int {
        /* 使用する自動獲得属性が指定されているとき */
        flag = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < used_auto_dic_features_num {
            if strcmp(used_auto_dic_features[i as usize], rule_value) == 0 {
                flag = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else { i += 1 }
        }
        if flag == 0 as libc::c_int {
            /* マッチしなかった */
            return 0 as libc::c_int;
        }
    }
    /* 後側をひとつずつ短くしていく */
    while m_length > 0 as libc::c_int {
        key[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < m_length {
            /* 辞書になければ、次のループに入り、後側を短くする */
            /* 形態素列からキーを作る */
            if i != 0 {
                strcat(key.as_mut_ptr(),
                       b"+\x00" as *const u8 as *const libc::c_char);
            }
            rep_str =
                get_mrph_rep_from_f(m_ptr.offset(i as isize),
                                    0 as libc::c_int);
            if !rep_str.is_null() {
                if strlen(key.as_mut_ptr()).wrapping_add(strlen(rep_str)).wrapping_add(2
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    > 5120 as libc::c_int as libc::c_ulong {
                    return 0 as libc::c_int;
                }
                strcat(key.as_mut_ptr(), rep_str);
            } else {
                /* 助詞、助動詞などは代表表記がない */
                strcat(key.as_mut_ptr(),
                       (*m_ptr.offset(i as isize)).Goi2.as_mut_ptr());
                /* 表記 */
            } /* strncmpの長さ */
            i += 1
        } /* 辞書項目を区切る (dict/auto/Makefile.amで指定) */
        dic_str = lookup_auto_dic(key.as_mut_ptr());
        if !dic_str.is_null() {
            let mut cmp_length: libc::c_int =
                strlen(rule_value) as libc::c_int;
            let mut token: *mut libc::c_char =
                strtok(dic_str, b"|\x00" as *const u8 as *const libc::c_char);
            ret = 0 as *mut libc::c_char;
            while !token.is_null() {
                if strncmp(token, rule_value, cmp_length as libc::c_ulong) ==
                    0 {
                    /* 辞書項目とルールから与えられた文字列がマッチ */
                    ret =
                        malloc_data(strlen(token).wrapping_add(9 as
                            libc::c_int
                            as
                            libc::c_ulong),
                                    b"check_auto_dic\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) as
                            *mut libc::c_char;
                    sprintf(ret,
                            b"%s:%d-%d\x00" as *const u8 as
                                *const libc::c_char, token, (*m_ptr).num,
                            (*m_ptr.offset(m_length as
                                isize).offset(-(1 as
                                libc::c_int
                                as
                                isize))).num);
                    break;
                } else {
                    token =
                        strtok(0 as *mut libc::c_char,
                               b"|\x00" as *const u8 as *const libc::c_char)
                }
            }
            free(dic_str as *mut libc::c_void);
            if !ret.is_null() {
                /* マッチすれば、featureをassign_posの形態素に付与して終了 */
                assign_cfeature(&mut (*m_ptr.offset(assign_pos as isize)).f,
                                ret, temp_assign_flag);
                free(ret as *mut libc::c_void);
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        if assign_pos != 0 {
            /* 末尾にfeatureを付与する場合は、一つ前にずらす */
            assign_pos -= 1
        }
        m_length -= 1
    }
    return 0 as libc::c_int;
}
/*====================================================================
                               END
====================================================================*/
