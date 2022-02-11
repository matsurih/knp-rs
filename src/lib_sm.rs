#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
//! NTT  検索プログラム

use crate::case_analysis::MatchPP;
use crate::case_match::{_sm_match_score, cf_match_element, comp_sm, sms_match};
use crate::consts::VERBOSE2;
use crate::ctools::{assign_cfeature, check_dict_filename, check_feature, malloc_data, Outfp, stderr};
use crate::db::{db_close, db_get, db_read_open};
use crate::dic::DICT;
use crate::structs::sm_list;
use crate::thesaurus::{code_depth, get_most_similar_code};
use crate::tools::{code2sm_db, CODE2SMExist, cont_str, hash, OptDisplay, sm2code_db, SM2CODEExist, sm_db, SMExist, smlist, smp2smg_db, SMP2SMGExist, Thesaurus, VerboseLevel};
use crate::types::{BNST_DATA, CF_PRED_MGR, FILE, SENTENCE_DATA, SMLIST};

#[no_mangle]
pub unsafe extern "C" fn init_ntt() {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    /* **  データベースオープン  ***/
    /* 単語 <=> 意味素コード */
    // ファイル名を指定する
    if !(*DICT.as_mut_ptr().offset(2 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(2 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
        // .knprc で定義されているとき   → SM_DB は const.h で定義されている
        //                                  DICT[SM_DB] は configfile.c で指定されている
    } else {
        filename =
            check_dict_filename(b"scode/ntt/word2code.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
        // .knprc で定義されていないとき → path.h の default値(SM_DB_NAME) を使う
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp, b"Opening %s ... \x00" as *const u8 as *const libc::c_char, filename);
    }
    sm_db = db_read_open(filename);
    if sm_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        SMExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        SMExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
    (*THESAURUS.as_mut_ptr().offset(2 as libc::c_int as isize)).exist =
        SMExist;
    /* 意味素 => 意味素コード */
    if Thesaurus == 2 as libc::c_int {
        if !(*DICT.as_mut_ptr().offset(3 as libc::c_int as isize)).is_null() {
            filename =
                check_dict_filename(*DICT.as_mut_ptr().offset(3 as libc::c_int
                    as isize),
                                    (0 as libc::c_int == 0) as libc::c_int)
        } else {
            filename =
                check_dict_filename(b"scode/ntt/sm2code.db\x00" as *const u8
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
    }
    /* 意味素コード => 意味素 */
    if !(*DICT.as_mut_ptr().offset(12 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(12 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"scode/ntt/code2sm.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                filename);
    }
    code2sm_db = db_read_open(filename);
    if code2sm_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        CODE2SMExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        CODE2SMExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
    /* 固有名詞体系 <=> 一般名詞体系 */
    if !(*DICT.as_mut_ptr().offset(4 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(4 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"scode/ntt/smp2smg.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                filename);
    }
    smp2smg_db = db_read_open(filename);
    if smp2smg_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        SMP2SMGExist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        SMP2SMGExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_ntt()
/*==================================================================*/
{
    if SMExist == (0 as libc::c_int == 0) as libc::c_int { db_close(sm_db); }
    if SM2CODEExist == (0 as libc::c_int == 0) as libc::c_int {
        db_close(sm2code_db);
    }
    if SMP2SMGExist == (0 as libc::c_int == 0) as libc::c_int {
        db_close(smp2smg_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ClearSMList()
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut smp: *mut SMLIST = 0 as *mut SMLIST;
    let mut next: *mut SMLIST = 0 as *mut SMLIST;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !smlist[i as usize].key.is_null() {
            free(smlist[i as usize].key as *mut libc::c_void);
            free(smlist[i as usize].sm as *mut libc::c_void);
            smlist[i as usize].key = 0 as *mut libc::c_char
        }
        smp = smlist[i as usize].next;
        while !smp.is_null() {
            free((*smp).key as *mut libc::c_void);
            free((*smp).sm as *mut libc::c_void);
            next = (*smp).next;
            free(smp as *mut libc::c_void);
            smp = next
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ne_check_all_sm(mut code: *mut libc::c_char)
                                         -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* すべての意味属性が固有名詞なら TRUE */
    i = 0 as libc::c_int;
    while *code.offset(i as isize) != 0 {
        if *code.offset(i as isize) as libc::c_int != '2' as i32 {
            return 0 as libc::c_int;
        }
        i += 12 as libc::c_int
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_ntt(mut cp: *mut libc::c_char,
                                  mut arg: *mut libc::c_char)
                                  -> *mut libc::c_char
/*==================================================================*/
{
    /* データベースから取り出した code を処理する */
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    code = db_get(sm_db, cp);
    if !code.is_null() {
        /* 溢れたら、縮める */
        if strlen(code) >
            (12 as libc::c_int * 256 as libc::c_int) as libc::c_ulong {
            *code.offset((12 as libc::c_int * 256 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        pos = 0 as libc::c_int;
        /* すべての意味属性が固有名詞のとき */
        if ne_check_all_sm(code) == (0 as libc::c_int == 0) as libc::c_int {
            i = 0 as libc::c_int;
            while *code.offset(i as isize) != 0 {
                if *code.offset(i as isize) as libc::c_int == '2' as i32 &&
                    strncmp(code.offset(i as isize),
                            b"2001030\x00" as *const u8 as
                                *const libc::c_char,
                            7 as libc::c_int as libc::c_ulong) != 0 {
                    /* 大字 ではない */
                    strncpy(code.offset(pos as isize),
                            code.offset(i as isize),
                            12 as libc::c_int as libc::c_ulong);
                    pos += 12 as libc::c_int
                }
                i += 12 as libc::c_int
            }
        } else {
            /* 意味素を付与する品詞 */
            i = 0 as libc::c_int;
            while *code.offset(i as isize) != 0 {
                if *arg as libc::c_int != 0 &&
                    *code.offset(i as isize) as libc::c_int ==
                        *arg as libc::c_int ||
                    *code.offset(i as isize) as libc::c_int == '3' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == '4' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == '5' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == '6' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == '7' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == '9' as i32
                    ||
                    *code.offset(i as isize) as libc::c_int == 'a' as i32 {
                    /* 代名 */
                    strncpy(code.offset(pos as isize),
                            code.offset(i as isize),
                            12 as libc::c_int as libc::c_ulong);
                    pos += 12 as libc::c_int
                }
                i += 12 as libc::c_int
            }
        }
        *code.offset(pos as isize) = '\u{0}' as i32 as libc::c_char
    }
    return code;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm2code(mut cp: *mut libc::c_char)
                                 -> *mut libc::c_char
/*==================================================================*/
{
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    /* sm と code は 1:1 対応 
       -> cont_str は溢れない */
    if SM2CODEExist == 0 as libc::c_int {
        cont_str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        return cont_str.as_mut_ptr();
    }
    code = db_get(sm2code_db, cp);
    if !code.is_null() {
        strcpy(cont_str.as_mut_ptr(), code);
        free(code as *mut libc::c_void);
    } else if Thesaurus == 2 as libc::c_int {
        if strncmp(cp,
                   b"ORGANIZATION\x00" as *const u8 as *const libc::c_char,
                   12 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne1*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"PERSON\x00" as *const u8 as *const libc::c_char,
                          6 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne2*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"LOCATION\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne3*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"ARTIFACT\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne4*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp, b"DATE\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne5*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp, b"TIME\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne6*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"MONEY\x00" as *const u8 as *const libc::c_char,
                          5 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne7*********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"PERCENT\x00" as *const u8 as *const libc::c_char,
                          7 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne8*********\x00" as *const u8 as *const libc::c_char);
        }
    } else if Thesaurus == 1 as libc::c_int {
        if strncmp(cp,
                   b"ORGANIZATION\x00" as *const u8 as *const libc::c_char,
                   12 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne1********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"PERSON\x00" as *const u8 as *const libc::c_char,
                          6 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne2********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"LOCATION\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne3********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"ARTIFACT\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne4********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp, b"DATE\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne5********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp, b"TIME\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne6********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"MONEY\x00" as *const u8 as *const libc::c_char,
                          5 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne7********\x00" as *const u8 as *const libc::c_char);
        } else if strncmp(cp,
                          b"PERCENT\x00" as *const u8 as *const libc::c_char,
                          7 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(cont_str.as_mut_ptr(),
                   b"ne8********\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        cont_str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    return cont_str.as_mut_ptr();
}
/* NEの場合は例外 */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn code2sm(mut cp: *mut libc::c_char)
                                 -> *mut libc::c_char
/*==================================================================*/
{
    let mut sm: *mut libc::c_char = 0 as *mut libc::c_char;
    /* sm と code は 1:1 対応 
       -> cont_str は溢れない */
    if CODE2SMExist == 0 as libc::c_int {
        cont_str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        return cont_str.as_mut_ptr();
    }
    sm = db_get(code2sm_db, cp);
    if !sm.is_null() {
        strcpy(cont_str.as_mut_ptr(), sm);
        free(sm as *mut libc::c_void);
    } else {
        cont_str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    return cont_str.as_mut_ptr();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn codes2sm_print(mut fp: *mut FILE,
                                        mut cp: *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut sm: [libc::c_char; 13] = [0; 13];
    i = 0 as libc::c_int;
    while *cp.offset(i as isize) != 0 {
        if i != 0 as libc::c_int { fputc(',' as i32, fp); }
        strncpy(sm.as_mut_ptr(), cp.offset(i as isize),
                12 as libc::c_int as libc::c_ulong);
        sm[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
        sm[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        fputs(code2sm(sm.as_mut_ptr()), fp);
        i += 12 as libc::c_int
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _smp2smg(mut cp: *mut libc::c_char)
                                  -> *mut libc::c_char
/*==================================================================*/
{
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 13] = [0; 13];
    /* 値は長くても 52 bytes ぐらい */
    if SMP2SMGExist == 0 as libc::c_int {
        cont_str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        return cont_str.as_mut_ptr();
    }
    strncpy(key.as_mut_ptr(), cp, 12 as libc::c_int as libc::c_ulong);
    key[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    code = db_get(smp2smg_db, key.as_mut_ptr());
    return code;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn smp2smg(mut cpd: *mut libc::c_char,
                                 mut flag: libc::c_int) -> *mut libc::c_char
/*==================================================================*/
{
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut storep: libc::c_int = 0 as libc::c_int;
    let mut inc: libc::c_int = 0;
    let mut use_0: libc::c_int = 1 as libc::c_int;
    if SMP2SMGExist == 0 as libc::c_int {
        fprintf(stderr,
                b";;; Cannot open smp2smg table!\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    start = _smp2smg(cpd);
    if start.is_null() { return 0 as *mut libc::c_char; }
    cp = start;
    while *cp != 0 {
        use_0 = 1 as libc::c_int;
        if *cp.offset(12 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            inc = 1 as libc::c_int
        } else if strncmp(cp.offset(12 as libc::c_int as isize),
                          b" side-effect\x00" as *const u8 as
                              *const libc::c_char,
                          12 as libc::c_int as libc::c_ulong) == 0 {
            if *cp.offset(12 as libc::c_int as
                isize).offset(12 as libc::c_int as isize) as
                libc::c_int == '/' as i32 {
                inc = 13 as libc::c_int
            } else {
                /* 今回で終わり */
                inc = 0 as libc::c_int
            }
            /* flag == FALSE の場合 side-effect を使わない */
            if flag == 0 as libc::c_int { use_0 = 0 as libc::c_int }
        } else if *cp.offset(12 as libc::c_int as isize) as libc::c_int !=
            '\u{0}' as i32 {
            fprintf(stderr,
                    b";;; Invalid delimiter! <%c> (%s)\n\x00" as *const u8 as
                        *const libc::c_char,
                    *cp.offset(12 as libc::c_int as isize) as libc::c_int,
                    b"smp2smg\x00" as *const u8 as *const libc::c_char);
            inc = 1 as libc::c_int
        } else {
            /* 今回で終わり '\0' */
            inc = 0 as libc::c_int
        }
        if use_0 != 0 {
            strncpy(start.offset(storep as isize), cp,
                    12 as libc::c_int as libc::c_ulong);
            storep += 12 as libc::c_int
        }
        if !(inc != 0) { break; }
        cp = cp.offset(inc as isize);
        cp = cp.offset(12 as libc::c_int as isize)
    }
    if storep != 0 {
        *start.offset(storep as isize) = '\u{0}' as i32 as libc::c_char;
        return start;
    }
    free(start as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn merge_smp2smg(mut bp: *mut BNST_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    /* smp2smg の結果をくっつける */
    if (*bp).SM_code[0 as libc::c_int as usize] as libc::c_int ==
        '\u{0}' as i32 {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*bp).SM_num {
        if (*bp).SM_code[(i * 12 as libc::c_int) as usize] as libc::c_int ==
            '2' as i32 {
            p =
                smp2smg(&mut *(*bp).SM_code.as_mut_ptr().offset((i *
                    12 as
                        libc::c_int)
                    as isize),
                        0 as libc::c_int);
            if !p.is_null() {
                /* 溢れた場合 */
                if strlen((*bp).SM_code.as_mut_ptr()).wrapping_add(strlen(p)).wrapping_div(12
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    > 256 as libc::c_int as libc::c_ulong {
                    return;
                }
                strcat((*bp).SM_code.as_mut_ptr(), p);
                free(p as *mut libc::c_void);
            }
        }
        i += 1
    }
    (*bp).SM_num =
        strlen((*bp).SM_code.as_mut_ptr()).wrapping_div(12 as libc::c_int as
            libc::c_ulong) as
            libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _ntt_code_match(mut c1: *mut libc::c_char,
                                         mut c2: *mut libc::c_char)
                                         -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    if *c1 as libc::c_int == '2' as i32 && *c2 as libc::c_int != '2' as i32 ||
        *c1 as libc::c_int != '2' as i32 &&
            *c2 as libc::c_int == '2' as i32 {
        return 0 as libc::c_int as libc::c_float;
    }
    d1 = code_depth(c1, 12 as libc::c_int);
    d2 = code_depth(c2, 12 as libc::c_int);
    if d1 + d2 == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    min = if d1 < d2 { d1 } else { d2 };
    if min == 0 as libc::c_int { return 0 as libc::c_int as libc::c_float; }
    i = 1 as libc::c_int;
    while i <= min {
        if *c1.offset(i as isize) as libc::c_int !=
            *c2.offset(i as isize) as libc::c_int {
            return 2 as libc::c_int as libc::c_float *
                (i - 1 as libc::c_int) as libc::c_float /
                (d1 + d2) as libc::c_float;
        }
        i += 1
    }
    return 2 as libc::c_int as libc::c_float * min as libc::c_float /
        (d1 + d2) as libc::c_float;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ntt_code_match(mut c1: *mut libc::c_char,
                                        mut c2: *mut libc::c_char,
                                        mut flag: libc::c_int)
                                        -> libc::c_float
/*==================================================================*/
{
    return if flag == 2 as libc::c_int {
        let mut score: libc::c_float = 0.;
        let mut maxscore: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut f1: libc::c_int = 0 as libc::c_int;
        let mut f2: libc::c_int = 0 as libc::c_int;
        let mut c1num: libc::c_int = 1 as libc::c_int;
        let mut c2num: libc::c_int = 1 as libc::c_int;
        if *c1 as libc::c_int == '2' as i32 {
            c1 = smp2smg(c1, 0 as libc::c_int);
            if c1.is_null() { return 0 as libc::c_int as libc::c_float; }
            f1 = 1 as libc::c_int;
            c1num =
                strlen(c1).wrapping_div(12 as libc::c_int as libc::c_ulong) as
                    libc::c_int
        }
        if *c2 as libc::c_int == '2' as i32 {
            c2 = smp2smg(c2, 0 as libc::c_int);
            if c2.is_null() {
                if f1 == 1 as libc::c_int { free(c1 as *mut libc::c_void); }
                return 0 as libc::c_int as libc::c_float;
            }
            f2 = 1 as libc::c_int;
            c2num =
                strlen(c2).wrapping_div(12 as libc::c_int as libc::c_ulong) as
                    libc::c_int
        }
        cp1 = c1;
        i = 0 as libc::c_int;
        while i < c1num {
            cp2 = c2;
            j = 0 as libc::c_int;
            while j < c2num {
                score = _ntt_code_match(cp1, cp2);
                if score > maxscore { maxscore = score }
                cp2 = cp2.offset(12 as libc::c_int as isize);
                j += 1
            }
            cp1 = cp1.offset(12 as libc::c_int as isize);
            i += 1
        }
        if f1 == 1 as libc::c_int { free(c1 as *mut libc::c_void); }
        if f2 == 1 as libc::c_int { free(c2 as *mut libc::c_void); }
        maxscore
    } else if flag == 4 as libc::c_int {
        let mut score_0: libc::c_float = 0.;
        let mut maxscore_0: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut cp2_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_0: libc::c_int = 0;
        let mut f2_0: libc::c_int = 0 as libc::c_int;
        let mut c2num_0: libc::c_int = 1 as libc::c_int;
        /* PATTERN: 固有名詞 */
        if *c1 as libc::c_int == '2' as i32 { return _ntt_code_match(c1, c2); }
        /* PATTERN: 普通名詞 */
        if *c2 as libc::c_int == '2' as i32 {
            c2 = smp2smg(c2, 0 as libc::c_int);
            if c2.is_null() { return 0 as libc::c_int as libc::c_float; }
            f2_0 = 1 as libc::c_int;
            c2num_0 =
                strlen(c2).wrapping_div(12 as libc::c_int as libc::c_ulong) as
                    libc::c_int
        }
        cp2_0 = c2;
        i_0 = 0 as libc::c_int;
        while i_0 < c2num_0 {
            score_0 = _ntt_code_match(c1, cp2_0);
            if score_0 > maxscore_0 { maxscore_0 = score_0 }
            cp2_0 = cp2_0.offset(12 as libc::c_int as isize);
            i_0 += 1
        }
        if f2_0 == 1 as libc::c_int { free(c2 as *mut libc::c_void); }
        maxscore_0
    } else { _ntt_code_match(c1, c2) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm_match_check(mut pat: *mut libc::c_char,
                                        mut codes: *mut libc::c_char,
                                        mut expand: libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if codes.is_null() { return 0 as libc::c_int; }
    i = 0 as libc::c_int;
    while *codes.offset(i as isize) != 0 {
        if _sm_match_score(pat, codes.offset(i as isize), expand) >
            0 as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_sm(mut bp: *mut BNST_DATA,
                                   mut cp: *mut libc::c_char) -> libc::c_int
/*==================================================================*/
{
    let mut target_code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_p: *mut libc::c_int = 0 as *mut libc::c_int;
    target_code = sm2code(cp);
    if Thesaurus == 1 as libc::c_int {
        code = (*bp).BGH_code.as_mut_ptr();
        num_p = &mut (*bp).BGH_num
    } else if Thesaurus == 2 as libc::c_int {
        code = (*bp).SM_code.as_mut_ptr();
        num_p = &mut (*bp).SM_num
    } else { return 0 as libc::c_int; }
    /* すでにその意味属性をもっているとき */
    if sms_match(target_code, code, 1 as libc::c_int) ==
        (0 as libc::c_int == 0) as libc::c_int {
        return 0 as libc::c_int;
    }
    /* ★溢れる?★ */
    strcat(code, target_code);
    *num_p += 1;
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm_check_match_max(mut exd: *mut libc::c_char,
                                            mut exp: *mut libc::c_char,
                                            mut expand: libc::c_int,
                                            mut target: *mut libc::c_char)
                                            -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 12 as libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut tempscore: libc::c_float = 0.;
    /* どちらかに用例のコードがないとき */
    if !(!exd.is_null() && !exp.is_null() && *exd as libc::c_int != 0 &&
        *exp as libc::c_int != 0) {
        return 0 as libc::c_int;
    }
    if expand != 1 as libc::c_int { expand = 4 as libc::c_int }
    /* 最大マッチスコアを求める */
    j = 0 as libc::c_int;
    while *exp.offset(j as isize) != 0 {
        i = 0 as libc::c_int;
        while *exd.offset(i as isize) != 0 {
            tempscore =
                ntt_code_match(exp.offset(j as isize), exd.offset(i as isize),
                               expand);
            if tempscore > score {
                score = tempscore;
                /* 両方 target 意味素に属す */
                if sm_match_check(target, exd, expand) != 0 &&
                    sm_match_check(target, exp, expand) != 0 {
                    flag = (0 as libc::c_int == 0) as libc::c_int
                } else { flag = 0 as libc::c_int }
            }
            i += step
        }
        j += step
    }
    return flag;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm_fix(mut bp: *mut BNST_DATA,
                                mut targets: *mut libc::c_char)
                                -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut codes: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*bp).SM_code[0 as libc::c_int as usize] as libc::c_int ==
        '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    codes = (*bp).SM_code.as_mut_ptr();
    i = 0 as libc::c_int;
    while *codes.offset(i as isize) != 0 {
        j = 0 as libc::c_int;
        while *targets.offset(j as isize) != 0 {
            if _sm_match_score(targets.offset(j as isize),
                               codes.offset(i as isize), 1 as libc::c_int) >
                0 as libc::c_int {
                strncpy(codes.offset(pos as isize), codes.offset(i as isize),
                        12 as libc::c_int as libc::c_ulong);
                pos += 12 as libc::c_int;
                break;
            } else { j += 12 as libc::c_int }
        }
        i += 12 as libc::c_int
    }
    /* match しない場合ってどんなとき? */
    if pos != 0 as libc::c_int {
        *codes.offset(pos as isize) = '\u{0}' as i32 as libc::c_char;
        (*bp).SM_num =
            strlen(codes).wrapping_div(12 as libc::c_int as libc::c_ulong) as
                libc::c_int
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm_all_match(mut c: *mut libc::c_char,
                                      mut target: *mut libc::c_char)
                                      -> libc::c_int
/*==================================================================*/
{
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_char = 0 as libc::c_int as libc::c_char;
    /* 固有名詞のとき以外で、すべての意味属性が時間であれば TRUE */
    p = c;
    while *p != 0 {
        /* 固有名詞のときをのぞく */
        if !(*p as libc::c_int == '2' as i32) {
            /* 意味素のチェック */
            if comp_sm(target, p, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            } else {
                if flag == 0 { flag = 1 as libc::c_int as libc::c_char }
            }
        }
        p = p.offset(12 as libc::c_int as isize)
    }
    return if flag != 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_time_feature(mut bp: *mut BNST_DATA)
/*==================================================================*/
{
    /* <時間> の意味素しかもっていなければ <時間> を与える */
    if check_feature((*bp).f,
                     b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char).is_null()
        &&
        sm_all_match((*bp).SM_code.as_mut_ptr(),
                     sm2code(b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                         as *const libc::c_char as
                         *mut libc::c_char)) != 0 {
        assign_cfeature(&mut (*bp).f,
                        b"\xe6\x99\x82\xe9\x96\x93\xe5\x88\xa4\xe5\xae\x9a\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char, 0 as libc::c_int);
        assign_cfeature(&mut (*bp).f,
                        b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_sm_aux_feature(mut bp: *mut BNST_DATA)
/*==================================================================*/
{
    /* ルールに入れた */
    if Thesaurus != 2 as libc::c_int { return; }
    /* <時間>属性を付与する */
    assign_time_feature(bp);
    /* <抽象>属性を付与する */
    if sm_all_match((*bp).SM_code.as_mut_ptr(),
                    sm2code(b"\xe6\x8a\xbd\xe8\xb1\xa1\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char)) !=
        0 {
        assign_cfeature(&mut (*bp).f,
                        b"\xe6\x8a\xbd\xe8\xb1\xa1\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_matched_sm(mut sm: *mut libc::c_char,
                                           mut del: *mut libc::c_char)
                                           -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *sm.offset(i as isize) != 0 {
        flag = 1 as libc::c_int;
        /* 固有ではないときチェック */
        if *sm.offset(i as isize) as libc::c_int != '2' as i32 {
            j = 0 as libc::c_int;
            while *del.offset(j as isize) != 0 {
                if _sm_match_score(sm.offset(i as isize),
                                   del.offset(j as isize), 1 as libc::c_int) >
                    0 as libc::c_int {
                    flag = 0 as libc::c_int;
                    break;
                } else { j += 12 as libc::c_int }
            }
        }
        if flag != 0 {
            strncpy(sm.offset(pos as isize), sm.offset(i as isize),
                    12 as libc::c_int as libc::c_ulong);
            pos += 12 as libc::c_int
        }
        i += 12 as libc::c_int
    }
    *sm.offset(pos as isize) = '\u{0}' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_specified_sm(mut sm: *mut libc::c_char,
                                             mut del: *mut libc::c_char)
                                             -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *sm.offset(i as isize) != 0 {
        flag = 1 as libc::c_int;
        /* 固有ではないときを対象とする */
        if *sm.offset(i as isize) as libc::c_int != '2' as i32 {
            j = 0 as libc::c_int;
            while *del.offset(j as isize) != 0 {
                if strncmp(sm.offset(i as
                    isize).offset(1 as libc::c_int as
                    isize),
                           del.offset(j as
                               isize).offset(1 as libc::c_int as
                               isize),
                           (12 as libc::c_int - 1 as libc::c_int) as
                               libc::c_ulong) == 0 {
                    flag = 0 as libc::c_int;
                    break;
                } else { j += 12 as libc::c_int }
            }
        }
        if flag != 0 {
            strncpy(sm.offset(pos as isize), sm.offset(i as isize),
                    12 as libc::c_int as libc::c_ulong);
            pos += 12 as libc::c_int
        }
        i += 12 as libc::c_int
    }
    *sm.offset(pos as isize) = '\u{0}' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn fix_sm_person(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if Thesaurus != 2 as libc::c_int { return; }
    /* 人名のとき: 
       o 一般名詞体系の<主体>以下の意味素を削除
       o 固有名詞体系の意味素の一般名詞体系へのマッピングを禁止 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe4\xba\xba\xe5\x90\x8d\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            /* 固有の意味素だけ残したい */
            delete_matched_sm((*(*sp).bnst_data.offset(i as
                isize)).SM_code.as_mut_ptr(),
                              b"100*********\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char); /* <主体>の意味素 */
            assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn fix_sm_place(mut sp: *mut SENTENCE_DATA,
                                      mut cpm_ptr: *mut CF_PRED_MGR)
/*==================================================================*/
{
    /* そのうち汎用化する
       現在は <場所> のみ */
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if Thesaurus != 2 as libc::c_int { return; }
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                usize].result_lists_d[0 as libc::c_int as
                usize].flag[i as
                usize];
        /* 省略格要素ではない割り当てがあったとき */
        if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int) &&
            !(*cpm_ptr).elem_b_ptr[i as usize].is_null() &&
            num >= 0 as libc::c_int &&
            MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).pp[num as
                usize][0 as
                libc::c_int
                as
                usize],
                    b"\xe3\x83\x87\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) != 0 &&
            cf_match_element((*(*cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).sm[num as
                usize],
                             b"\xe5\xa0\xb4\xe6\x89\x80\x00" as *const u8
                                 as *const libc::c_char as
                                 *mut libc::c_char,
                             (0 as libc::c_int == 0) as libc::c_int) != 0 {
            /* 固有→一般変換しておく */
            merge_smp2smg((*cpm_ptr).elem_b_ptr[i as usize] as
                *mut BNST_DATA);
            /* <場所>のみに限定する */
            sm_fix((*cpm_ptr).elem_b_ptr[i as usize] as *mut BNST_DATA,
                   b"101*********20**********\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
            assign_cfeature(&mut (**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(i
                as
                isize)).f,
                            b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            assign_cfeature(&mut (**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(i
                as
                isize)).f,
                            b"\xe9\x9d\x9e\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            break;
        } else { i += 1 }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn register_noun_sm(mut key: *mut libc::c_char,
                                          mut sm: *mut libc::c_char)
/*==================================================================*/
{
    let mut slp: *mut SMLIST = 0 as *mut SMLIST;
    if key.is_null() { return; }
    slp =
        &mut *smlist.as_mut_ptr().offset((hash as
            unsafe extern "C" fn(_:
                                 *mut libc::c_uchar,
                                 _:
                                 libc::c_int)
                                 ->
                                 libc::c_int)(key as
                                                  *mut libc::c_uchar,
                                              (strlen as
                                                  unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                                       ->
                                                                       libc::c_ulong)(key)
                                                  as
                                                  libc::c_int)
            as isize) as *mut SMLIST;
    if !(*slp).key.is_null() {
        let mut slpp: *mut *mut SMLIST = 0 as *mut *mut SMLIST;
        slpp = &mut slp;
        loop {
            if strcmp((**slpp).key, key) == 0 {
                /* すでにあるsmを上書き */
                free((**slpp).sm as *mut libc::c_void);
                (**slpp).sm = strdup(sm);
                return;
            }
            slpp = &mut (**slpp).next;
            if (*slpp).is_null() { break; }
        }
        *slpp =
            malloc_data(::std::mem::size_of::<SMLIST>() as libc::c_ulong,
                        b"register_noun_sm\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut SMLIST;
        (**slpp).key = strdup(key);
        (**slpp).sm = strdup(sm);
        (**slpp).next = 0 as *mut sm_list
    } else {
        (*slp).key = strdup(key);
        (*slp).sm = strdup(sm)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_noun_sm(mut key: *mut libc::c_char)
                                       -> *mut libc::c_char
/*==================================================================*/
{
    let mut slp: *mut SMLIST = 0 as *mut SMLIST;
    slp =
        &mut *smlist.as_mut_ptr().offset((hash as
            unsafe extern "C" fn(_:
                                 *mut libc::c_uchar,
                                 _:
                                 libc::c_int)
                                 ->
                                 libc::c_int)(key as
                                                  *mut libc::c_uchar,
                                              (strlen as
                                                  unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                                       ->
                                                                       libc::c_ulong)(key)
                                                  as
                                                  libc::c_int)
            as isize) as *mut SMLIST;
    if (*slp).key.is_null() { return 0 as *mut libc::c_char; }
    while !slp.is_null() {
        if strcmp((*slp).key, key) == 0 {
            let mut newsm: *mut libc::c_char = 0 as *mut libc::c_char;
            newsm = strdup((*slp).sm);
            if VerboseLevel as libc::c_uint >=
                VERBOSE2 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b";; Cache hit!: %s [\x00" as *const u8 as
                            *const libc::c_char, key);
                codes2sm_print(stderr, newsm);
                fprintf(stderr,
                        b"]\n\x00" as *const u8 as *const libc::c_char);
            }
            return newsm;
        }
        slp = (*slp).next
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn specify_sm_from_cf(mut sp: *mut SENTENCE_DATA,
                                            mut cpm_ptr: *mut CF_PRED_MGR)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut new_code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sm_codes: *mut libc::c_char = 0 as *mut libc::c_char;
    if Thesaurus != 2 as libc::c_int { return; }
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if !((*cpm_ptr).elem_b_ptr[i as usize].is_null() ||
            (*(*cpm_ptr).elem_b_ptr[i as
                usize]).SM_code[0 as libc::c_int
                as usize] ==
                0) {
            num =
                (*cpm_ptr).cmm[0 as libc::c_int as
                    usize].result_lists_d[0 as libc::c_int as
                    usize].flag[i as
                    usize];
            /* 省略格要素ではない割り当てがあったとき */
            if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int) &&
                num >= 0 as libc::c_int &&
                !(*(*cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).ex[num as
                    usize].is_null()
                &&
                (*cpm_ptr).cmm[0 as libc::c_int as
                    usize].result_lists_p[0 as libc::c_int
                    as
                    usize].pos[(*cpm_ptr).cmm[0
                    as
                    libc::c_int
                    as
                    usize].result_lists_d[0
                    as
                    libc::c_int
                    as
                    usize].flag[i
                    as
                    usize]
                    as
                    usize]
                    != -(1 as libc::c_int) &&
                (*cpm_ptr).cmm[0 as libc::c_int as
                    usize].result_lists_d[0 as libc::c_int
                    as
                    usize].score[i
                    as
                    usize]
                    > 7 as libc::c_int as libc::c_double {
                /* 格フレームとある程度マッチするとき */
                if !(*(*cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).sm_specify[num as
                    usize].is_null()
                {
                    sm_codes =
                        strdup((*(*cpm_ptr).cmm[0 as libc::c_int as
                            usize].cf_ptr).sm_specify[num
                            as
                            usize])
                } else {
                    sm_codes =
                        strdup((*(*cpm_ptr).cmm[0 as libc::c_int as
                            usize].cf_ptr).ex[num as
                            usize]);
                    if !(*(*cpm_ptr).cmm[0 as libc::c_int as
                        usize].cf_ptr).sm_delete[num as
                        usize].is_null()
                    {
                        delete_specified_sm(sm_codes,
                                            (*(*cpm_ptr).cmm[0 as libc::c_int
                                                as
                                                usize].cf_ptr).sm_delete[num
                                                as
                                                usize]);
                    }
                }
                /* もっとも類似している意味属性に決定 */
                new_code =
                    get_most_similar_code((*(*cpm_ptr).elem_b_ptr[i as
                        usize]).SM_code.as_mut_ptr(),
                                          sm_codes);
                if !new_code.is_null() {
                    if strcmp((*(*cpm_ptr).elem_b_ptr[i as
                        usize]).SM_code.as_mut_ptr(),
                              new_code) != 0 {
                        /* 意味素更新 */
                        if VerboseLevel as libc::c_uint >=
                            VERBOSE2 as libc::c_int as libc::c_uint {
                            fprintf(stderr,
                                    b";;; %s %d %s [\x00" as *const u8 as
                                        *const libc::c_char,
                                    if !(*sp).KNPSID.is_null() {
                                        (*sp).KNPSID as *const libc::c_char
                                    } else {
                                        b"?\x00" as *const u8 as
                                            *const libc::c_char
                                    },
                                    (*(*cpm_ptr).elem_b_ptr[i as usize]).num,
                                    (*(*(*cpm_ptr).elem_b_ptr[i as
                                        usize]).head_ptr).Goi.as_mut_ptr());
                            codes2sm_print(stderr,
                                           (*(*cpm_ptr).elem_b_ptr[i as
                                               usize]).SM_code.as_mut_ptr());
                            fprintf(stderr,
                                    b"] -> [\x00" as *const u8 as
                                        *const libc::c_char);
                            codes2sm_print(stderr, new_code);
                            fprintf(stderr,
                                    b"]\n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        strcpy((*(*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[i
                            as
                            usize]).num
                            as
                            isize)).SM_code.as_mut_ptr(),
                               new_code);
                        (*(*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[i as
                            usize]).num
                            as isize)).SM_num =
                            strlen(new_code).wrapping_div(12 as libc::c_int as
                                libc::c_ulong)
                                as libc::c_int;
                        /* 意味素登録 */
                        register_noun_sm((*(*(*cpm_ptr).elem_b_ptr[i as
                            usize]).head_ptr).Goi.as_mut_ptr(),
                                         new_code);
                    }
                    free(new_code as *mut libc::c_void);
                }
                free(sm_codes as *mut libc::c_void);
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_ga_subject(mut sp: *mut SENTENCE_DATA,
                                           mut cpm_ptr: *mut CF_PRED_MGR)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if Thesaurus != 2 as libc::c_int { return; }
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                usize].result_lists_d[0 as libc::c_int as
                usize].flag[i as
                usize];
        /* 省略格要素ではない割り当てがあったとき */
        if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int) &&
            !(*cpm_ptr).elem_b_ptr[i as usize].is_null() &&
            (*cpm_ptr).cmm[0 as libc::c_int as
                usize].result_lists_d[0 as libc::c_int as
                usize].flag[i as
                usize]
                >= 0 as libc::c_int &&
            MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).pp[num as
                usize][0 as
                libc::c_int
                as
                usize],
                    b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) != 0 {
            /* o すでに主体付与されていない
	       o <数量> ではない (<数量>のとき意味属性がない)
	       o <用言:動>である 
	       o 格フレームが<主体>をもつ, <主体準>ではない
	       o 入力側が意味素がないか、(固有名詞と推定)
	         <抽象物> or <事>という意味素をもつ (つまり、<抽象的関係>だけではない)
	    */
            if check_feature((*(*cpm_ptr).elem_b_ptr[i as usize]).f,
                             b"\xe4\xb8\xbb\xe4\xbd\x93\xe4\xbb\x98\xe4\xb8\x8e\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                check_feature((*(*cpm_ptr).elem_b_ptr[i as usize]).f,
                              b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8
                                  as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                               b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                cf_match_element((*(*cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).sm[num
                    as
                    usize],
                                 b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                                 (0 as libc::c_int == 0) as libc::c_int) !=
                    0 &&
                ((*(*cpm_ptr).elem_b_ptr[i as usize]).SM_num ==
                    0 as libc::c_int ||
                    sm_match_check(sm2code(b"\xe5\x85\xb7\xe4\xbd\x93\x00"
                        as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char),
                                   (*(*cpm_ptr).elem_b_ptr[i as
                                       usize]).SM_code.as_mut_ptr(),
                                   1 as libc::c_int) != 0 ||
                    sm_match_check(sm2code(b"\xe5\x9c\xb0\xe5\x90\x8d\x00"
                        as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char),
                                   (*(*cpm_ptr).elem_b_ptr[i as
                                       usize]).SM_code.as_mut_ptr(),
                                   1 as libc::c_int) != 0 ||
                    sm_match_check(sm2code(b"\xe6\x8a\xbd\xe8\xb1\xa1\xe7\x89\xa9\x00"
                        as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char),
                                   (*(*cpm_ptr).elem_b_ptr[i as
                                       usize]).SM_code.as_mut_ptr(),
                                   1 as libc::c_int) != 0 ||
                    sm_match_check(sm2code(b"\xe4\xba\x8b\x00" as
                        *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char),
                                   (*(*cpm_ptr).elem_b_ptr[i as
                                       usize]).SM_code.as_mut_ptr(),
                                   1 as libc::c_int) != 0) {
                assign_sm((*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[i as
                    usize]).num
                    as isize) as
                              *mut BNST_DATA,
                          b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
                assign_cfeature(&mut (*(*sp).tag_data.offset((**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(i
                    as
                    isize)).num
                    as isize)).f,
                                b"\xe4\xb8\xbb\xe4\xbd\x93\xe4\xbb\x98\xe4\xb8\x8e\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
            }
            break;
        } else { i += 1 }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sm2feature(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feature_buffer: [libc::c_char; 3332] = [0; 3332];
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        /* thesaurus.c: get_bnst_code() で与えられたfeatureを上書き */
        cp =
            check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"NTT\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"%s:%s\x00" as *const u8 as *const libc::c_char, cp,
                    (*(*sp).tag_data.offset(i as
                        isize)).SM_code.as_mut_ptr());
            assign_cfeature(&mut (*(*sp).tag_data.offset(i as isize)).f,
                            feature_buffer.as_mut_ptr(), 0 as libc::c_int);
        }
        i += 1
    };
}
/*====================================================================
                               END
====================================================================*/
