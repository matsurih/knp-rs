#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! 分類語彙表  検索プログラム
//!
//!  増補改訂版(2004) 9万6千語
//!  例）
//!  原因,げんいん1.1112,04,02,01
//!  投票,とうひょう1.1532,16,03,02
//!  投票,とうひょう1.3630,17,01,01
//!  テレビ,てれび1.4620,02,01,03
//!
//!  類似度:以下のようにコード化し7レベルの類似度に
//!  1.2345.66.7777 (7777は最後の2レベルをcat)
//!
use libc;

use crate::{fprintf, fputs, free, Outfp, strncmp};
use crate::ctools::{check_dict_filename, db_close, db_get, db_read_open, DICT, SM2CODEExist, THESAURUS, Thesaurus};
use crate::structs::CDB_FILE;
use crate::tools::{OptDisplay, sm2code_db};
use crate::types::DBM_FILE;

#[no_mangle]
pub static mut bgh_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut BGHExist: libc::c_int = 0;

/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_bgh()
/*==================================================================*/
{
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*DICT.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(1 as libc::c_int as
                isize),
                                        (0 as libc::c_int == 0) as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"scode/bgh/bgh.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                filename);
    }
    bgh_db = db_read_open(filename);
    if bgh_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        BGHExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        BGHExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
    (*THESAURUS.as_mut_ptr().offset(1 as libc::c_int as isize)).exist =
        BGHExist;
    /* 意味素 => 意味素コード */
    if Thesaurus == 1 as libc::c_int {
        if !(*DICT.as_mut_ptr().offset(3 as libc::c_int as isize)).is_null() {
            filename =
                check_dict_filename(*DICT.as_mut_ptr().offset(3 as libc::c_int as isize),
                                    (0 as libc::c_int == 0) as libc::c_int)
        } else {
            filename =
                check_dict_filename(b"scode/bgh/sm2code.db\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int)
        }
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Opening %s ... \x00" as *const u8 as
                        *const libc::c_char, filename);
        }
        sm2code_db = db_read_open(filename);
        if sm2code_db.is_null() {
            if OptDisplay == 3 as libc::c_int {
                fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                      Outfp);
            }
            SM2CODEExist = 0 as libc::c_int
        } else {
            if OptDisplay == 3 as libc::c_int {
                fputs(b"done.\n\x00" as *const u8 as *const libc::c_char,
                      Outfp);
            }
            SM2CODEExist = (0 as libc::c_int == 0) as libc::c_int
        }
        free(filename as *mut libc::c_void);
    };
}

#[no_mangle]
pub unsafe extern "C" fn close_bgh() {
    if BGHExist == (0 as libc::c_int == 0) as libc::c_int {
        db_close(bgh_db);
    };
}

#[no_mangle]
pub unsafe extern "C" fn _get_bgh(mut cp: *mut libc::c_char, mut arg: *mut libc::c_char) -> *mut libc::c_char {
    return db_get(bgh_db, cp);
}

#[no_mangle]
pub unsafe extern "C" fn bgh_code_match(mut c1: *mut libc::c_char, mut c2: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut point: libc::c_int = 0 as libc::c_int;
    /* 1桁目一致 -> 1,2,3,4,5,6-7,8-11桁目比較

       1桁目不一致 -> 1桁目が4(その他)以外なら 2〜4桁目比較 
       		      2桁目以降一致の場合 1桁目は一致とみなす */
    /* sm-***の形で記述される汎化された意味情報を無視する */
    if *c1.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32 ||
        *c2.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32
    {
        return point;
    }
    if *c1.offset(0 as libc::c_int as isize) as libc::c_int ==
        *c2.offset(0 as libc::c_int as isize) as libc::c_int {
        point = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while *c1.offset(i as isize) as libc::c_int ==
            *c2.offset(i as isize) as libc::c_int &&
            i < 11 as libc::c_int {
            if i != 5 as libc::c_int && i != 7 as libc::c_int &&
                i != 8 as libc::c_int && i != 9 as libc::c_int {
                point += 1
            }
            i += 1
        }
    } else if *c1.offset(0 as libc::c_int as isize) as libc::c_int !=
        '4' as i32 &&
        *c2.offset(0 as libc::c_int as isize) as libc::c_int !=
            '4' as i32 &&
        *c1.offset(1 as libc::c_int as isize) as libc::c_int ==
            *c2.offset(1 as libc::c_int as isize) as libc::c_int {
        point = 2 as libc::c_int;
        i = 2 as libc::c_int;
        while *c1.offset(i as isize) as libc::c_int ==
            *c2.offset(i as isize) as libc::c_int &&
            i < 4 as libc::c_int {
            point += 1;
            i += 1
        }
    }
    return point;
}

#[no_mangle]
pub unsafe extern "C" fn bgh_code_match_for_case(mut cp1: *mut libc::c_char, mut cp2: *mut libc::c_char) -> libc::c_int {
    /* 例の分類語彙表コードのマッチング度の計算 */
    let mut match_0: libc::c_int = 0 as libc::c_int;
    /* 単位の項目は無視 */
    if strncmp(cp1, b"11960\x00" as *const u8 as *const libc::c_char,
               5 as libc::c_int as libc::c_ulong) == 0 ||
        strncmp(cp2, b"11960\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong) == 0 {
        return 0 as libc::c_int;
    }
    /* 比較 */
    match_0 = bgh_code_match(cp1, cp2);
    /* 代名詞の項目は類似度を押さえる */
    if (strncmp(cp1, b"12000\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong) == 0 ||
        strncmp(cp2, b"12000\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong) == 0) &&
        match_0 > 3 as libc::c_int {
        return 3 as libc::c_int;
    }
    return match_0;
}

#[no_mangle]
pub unsafe extern "C" fn comp_bgh(mut cpp: *mut libc::c_char, mut cpd: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if *cpp.offset(0 as libc::c_int as isize) as libc::c_int ==
        *cpd.offset(0 as libc::c_int as isize) as libc::c_int {
        i = 1 as libc::c_int;
        while i < 11 as libc::c_int {
            if *cpp.offset(i as isize) as libc::c_int == '*' as i32 {
                return i;
            } else {
                if *cpp.offset(i as isize) as libc::c_int != *cpd.offset(i as isize) as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            i += 1
        }
    } else if *cpp.offset(0 as libc::c_int as isize) as libc::c_int != '4' as i32 && *cpd.offset(0 as libc::c_int as isize) as libc::c_int !=
        '4' as i32 &&
        *cpp.offset(1 as libc::c_int as isize) as libc::c_int ==
            *cpd.offset(1 as libc::c_int as isize) as libc::c_int {
        i = 2 as libc::c_int;
        while i < 4 as libc::c_int {
            if *cpp.offset(i as isize) as libc::c_int == '*' as i32 {
                return i;
            } else {
                if *cpp.offset(i as isize) as libc::c_int !=
                    *cpd.offset(i as isize) as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            i += 1
        }
    } else { return 0 as libc::c_int; }
    return 11 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn bgh_match_check(mut pat: *mut libc::c_char, mut codes: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if codes.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while *codes.offset(i as isize) != 0 {
        if comp_bgh(pat, codes.offset(i as isize)) > 0 as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 11 as libc::c_int
    }
    return 0 as libc::c_int;
}
