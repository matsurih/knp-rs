#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! 設定ファイル関連

use libc;

use crate::{atof, atoi, ctools, FILE, fopen, fprintf, fputs, free, sprintf, strcasecmp, strcat, strchr, strcmp, strcpy, strdup, strlen};
use crate::case_analysis::pp_kstr_to_code;
use crate::context::{AntecedentDecideThresholdForGa, AntecedentDecideThresholdForNi, AntecedentDecideThresholdPredGeneral, DiscAddedCases, loc_code_to_str, loc_name_to_code, LocationLimit, LocationOrder, PrevSentenceLimit};
use crate::corefer::SynonymFile;
use crate::ctools::{__xstat, car, cdr, CRFFileNE, DistSimDB, DistSimFile, DistSimWordList, exit, getenv, Jumangram_Dirname, LineNo, LineNoForError, malloc_data, Outfp, s_feof, s_read, stderr, strtok, THESAURUS};
use crate::db::db_read_open;
use crate::dic::{used_auto_dic_features, used_auto_dic_features_num};
use crate::juman::lisp::{set_cha_getc, unset_cha_getc};
use crate::lib_dt::DTFile;
use crate::structs::{CDB_FILE, stat, timespec};
use crate::tools::{CurrentRuleNum, DICT, knp_dict_file_already_defined, Knpdict_Dirname, Knprule_Dirname, OptDiscNounMethod, OptDiscPredMethod, OptDisplay, OptEllipsis, OptNE, OptNECRF, ParaThesaurus, realloc_data, RULE, RuleNumMax, Thesaurus};
use crate::types::{CELL, DBM_FILE, RuleVector, time_t};


/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_duplicated(mut value: libc::c_int, mut string: *mut libc::c_char) {
    /* 値が 0 でないときはエラー */
    if value != 0 {
        fprintf(stderr, b"%s is duplicately specified in .knprc\n\x00" as *const u8 as *const libc::c_char, string);
        exit(0 as libc::c_int);
    }
}

#[no_mangle]
pub unsafe extern "C" fn clear_rule_configuration() {
    if CurrentRuleNum != 0 {
        free(RULE as *mut libc::c_void);
        RULE = 0 as *mut RuleVector;
        CurrentRuleNum = 0 as libc::c_int;
        RuleNumMax = 0 as libc::c_int
    }
    if !Knprule_Dirname.is_null() {
        free(Knprule_Dirname as *mut libc::c_void);
        Knprule_Dirname = 0 as *mut libc::c_char
    }
    if !Knpdict_Dirname.is_null() {
        free(Knpdict_Dirname as *mut libc::c_void);
        Knpdict_Dirname = 0 as *mut libc::c_char
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_tilde(mut file: *mut libc::c_char)
                                     -> *mut libc::c_char
/*==================================================================*/
{
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if *file as libc::c_int == '~' as i32 &&
        {
            home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
            !home.is_null()
        } {
        ret =
            malloc_data(strlen(home).wrapping_add(strlen(file)),
                        b"check_tilde\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut libc::c_char;
        sprintf(ret, b"%s%s\x00" as *const u8 as *const libc::c_char, home,
                strchr(file, '/' as i32));
    } else { ret = strdup(file) }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn str2ints(mut str: *mut libc::c_char) -> *mut libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret_size: libc::c_int = 1 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '\"' as i32 {
        start = str.offset(1 as libc::c_int as isize);
        cp = strchr(start, '\"' as i32);
        if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
    } else { start = str }
    ret =
        malloc_data(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(ret_size as libc::c_ulong),
            b"str2ints\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) as *mut libc::c_int;
    token = strtok(start, b",\x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        if count >= ret_size - 1 as libc::c_int {
            ret_size <<= 1 as libc::c_int;
            ret =
                realloc_data(
                    ret as *mut libc::c_void,
                    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(ret_size as libc::c_ulong),
                    b"str2ints\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) as *mut libc::c_int
        }
        let fresh0 = count;
        count = count + 1;
        *ret.offset(fresh0 as isize) = atoi(token);
        token = strtok(0 as *mut libc::c_char, b",\x00" as *const u8 as *const libc::c_char)
    }
    *ret.offset(count as isize) = 0 as libc::c_int;
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_rc_file(mut opfile: *mut libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if !opfile.is_null() {
        fp = fopen(opfile, b"r\x00" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            fprintf(stderr, b"not found rc file <%s>.\n\x00" as *const u8 as *const libc::c_char, opfile);
            exit(1 as libc::c_int);
        }
    } else {
        let mut user_home: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        user_home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if user_home.is_null() {
            filename = 0 as *mut libc::c_char
        } else {
            filename = malloc_data(
                strlen(user_home).wrapping_add(strlen(b"/.knprc\x00" as *const u8 as *const libc::c_char)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"find_rc_file\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as *mut libc::c_char;
            sprintf(
                filename,
                b"%s/.knprc\x00" as *const u8 as *const libc::c_char,
                user_home,
            );
        }
        if filename.is_null() ||
            {
                fp =
                    fopen(filename,
                          b"r\x00" as *const u8 as *const libc::c_char);
                fp.is_null()
            } {
            fp =
                fopen(b"/usr/local/etc/knprc\x00" as *const u8 as
                          *const libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char);
            if fp.is_null() {
                fprintf(stderr,
                        b"not found <.knprc> and KNP_RC_DEFAULT(<%s>).\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"/usr/local/etc/knprc\x00" as *const u8 as
                            *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
        if !filename.is_null() { free(filename as *mut libc::c_void); }
    }
    return fp;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_rc(mut in_0: *mut FILE)
/*==================================================================*/
{
    let mut cell1: *mut CELL = 0 as *mut CELL;
    let mut cell2: *mut CELL = 0 as *mut CELL;
    let mut dicttype: *mut libc::c_char = 0 as *mut libc::c_char;
    LineNo = 0 as libc::c_int;
    *Jumangram_Dirname.as_mut_ptr().offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    while s_feof(in_0) == 0 {
        LineNoForError = LineNo;
        cell1 = s_read(in_0);
        if strcmp(b"JUMAN\xe6\x96\x87\xe6\xb3\x95\xe3\x83\x87\xe3\x82\xa3\xe3\x83\xac\xe3\x82\xaf\xe3\x83\x88\xe3\x83\xaa\x00"
                      as *const u8 as *const libc::c_char,
                  (*car(cell1)).value.atom as *const libc::c_char) == 0 {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                strcpy(Jumangram_Dirname.as_mut_ptr(),
                       (*cell2).value.atom as *const libc::c_char);
            }
        } else if strcmp(b"KNP\xe3\x83\xab\xe3\x83\xbc\xe3\x83\xab\xe3\x83\x87\xe3\x82\xa3\xe3\x83\xac\xe3\x82\xaf\xe3\x83\x88\xe3\x83\xaa\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() && {
                cell2 = car(cdr(cell1));
                ((*cell2).tag) == 1 as libc::c_int
            }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                Knprule_Dirname =
                    check_tilde((*cell2).value.atom as *mut libc::c_char)
            }
        } else if strcmp(b"KNP\xe3\x83\xab\xe3\x83\xbc\xe3\x83\xab\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00" as *const u8 as *const libc::c_char, (*car(cell1)).value.atom as *const libc::c_char) == 0 {
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                if CurrentRuleNum >= RuleNumMax {
                    RuleNumMax += 10 as libc::c_int;
                    RULE = realloc_data(
                        RULE as *mut libc::c_void,
                        (::std::mem::size_of::<RuleVector>() as libc::c_ulong).wrapping_mul(RuleNumMax as libc::c_ulong),
                        b"read_rc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) as *mut RuleVector
                }
                /* KNP ルールディレクトリ */
                /* KNP ルールファイル */
                /* デフォルト値設定 */
                let ref mut fresh1 = (*RULE.offset(CurrentRuleNum as isize)).file;
                *fresh1 = strdup((*car(car(cell1))).value.atom as *const libc::c_char);
                (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
                (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
                (*RULE.offset(CurrentRuleNum as isize)).type_0 = 0 as libc::c_int;
                (*RULE.offset(CurrentRuleNum as isize)).direction = 0 as libc::c_int;
                cell2 = cdr(car(cell1));
                while !car(cell2).is_null() {
                    if strcmp((*car(cell2)).value.atom as *const libc::c_char, b"\xe5\x90\x8c\xe5\xbd\xa2\xe7\x95\xb0\xe7\xbe\xa9\xe8\xaa\x9e\x00" as *const u8 as *const libc::c_char) == 0 {
                        check_duplicated(
                            (*RULE.offset(CurrentRuleNum as isize)).type_0,
                            b"Rule type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 3 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as *const libc::c_char, b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\x00" as *const u8 as *const libc::c_char) == 0 {
                        check_duplicated(
                            (*RULE.offset(CurrentRuleNum as isize)).type_0,
                            b"Rule type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 1 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as *const libc::c_char, b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0-\xe5\x89\x8d\xe5\x87\xa6\xe7\x90\x86\x00" as *const u8 as *const libc::c_char) == 0 {
                        check_duplicated(
                            (*RULE.offset(CurrentRuleNum as isize)).type_0,
                            b"Rule type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 16 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as *const libc::c_char, b"\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00" as *const u8 as *const libc::c_char) == 0 {
                        check_duplicated(
                            (*RULE.offset(CurrentRuleNum as isize)).type_0,
                            b"Rule type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            11 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5-\xe6\xa7\x8b\xe9\x80\xa0\xe6\xb1\xba\xe5\xae\x9a\xe5\xbe\x8c\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            13 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5-\xe5\xbe\x8c\xe5\x87\xa6\xe7\x90\x86\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            14 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe6\x96\x87\xe7\xaf\x80\x00" as
                                         *const u8 as *const libc::c_char) ==
                        0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            2 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe6\x96\x87\xe7\xaf\x80-\xe6\xa7\x8b\xe9\x80\xa0\xe6\xb1\xba\xe5\xae\x9a\xe5\xbe\x8c\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            12 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe4\xbf\x82\xe3\x82\x8a\xe5\x8f\x97\xe3\x81\x91\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            4 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x91\xbc\xe5\xbf\x9c\x00" as
                                         *const u8 as *const libc::c_char) ==
                        0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            5 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9b\xba\xe6\x9c\x89\xe8\xa1\xa8\xe7\x8f\xbe\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            6 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9b\xba\xe6\x9c\x89\xe8\xa1\xa8\xe7\x8f\xbe\xe5\x8f\xa5-PRE\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            7 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9b\xba\xe6\x9c\x89\xe8\xa1\xa8\xe7\x8f\xbe\xe5\x8f\xa5\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            8 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe5\x9b\xba\xe6\x9c\x89\xe8\xa1\xa8\xe7\x8f\xbe\xe5\x8f\xa5-AUX\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            9 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe6\x96\x87\xe8\x84\x88\x00" as
                                         *const u8 as *const libc::c_char) ==
                        0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).type_0,
                                         b"Rule type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).type_0 =
                            10 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe3\x83\xab\xe3\x83\xbc\xe3\x83\xab\xe3\x83\xab\xe3\x83\xbc\xe3\x83\x97\xe5\x85\x88\xe8\xa1\x8c\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        (*RULE.offset(CurrentRuleNum as isize)).mode =
                            1 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"BREAK\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        /* RLOOP_BREAK_NONE は 0 なのでひっかからない */
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).breakmode,
                                         b"Break mode\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).breakmode =
                            1 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"BREAKJUMP\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        /* RLOOP_BREAK_NONE は 0 なのでひっかからない */
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).breakmode,
                                         b"Break mode\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).breakmode =
                            2 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe9\xa0\x86\xe6\x96\xb9\xe5\x90\x91\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).direction,
                                         b"Direction\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).direction =
                            1 as libc::c_int
                    } else if strcmp((*car(cell2)).value.atom as
                                         *const libc::c_char,
                                     b"\xe9\x80\x86\xe6\x96\xb9\xe5\x90\x91\x00"
                                         as *const u8 as *const libc::c_char)
                        == 0 {
                        check_duplicated((*RULE.offset(CurrentRuleNum as
                            isize)).direction,
                                         b"Direction\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                        (*RULE.offset(CurrentRuleNum as isize)).direction =
                            -(1 as libc::c_int)
                    } else {
                        fprintf(stderr,
                                b"%s is invalid in .knprc\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*car(cell2)).value.atom);
                        exit(0 as libc::c_int);
                    }
                    cell2 = cdr(cell2)
                }
                /* ルールのタイプが指定されていないとき */
                if (*RULE.offset(CurrentRuleNum as isize)).type_0 == 0 {
                    fprintf(stderr,
                            b"Rule type for \'%s\' is not specified in .knprc\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*RULE.offset(CurrentRuleNum as isize)).file);
                    exit(0 as libc::c_int);
                }
                /* デフォルトの方向 */
                if (*RULE.offset(CurrentRuleNum as isize)).direction == 0 {
                    (*RULE.offset(CurrentRuleNum as isize)).direction =
                        1 as libc::c_int
                }
                CurrentRuleNum += 1;
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe8\xbe\x9e\xe6\x9b\xb8\xe3\x83\x87\xe3\x82\xa3\xe3\x83\xac\xe3\x82\xaf\xe3\x83\x88\xe3\x83\xaa\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                Knpdict_Dirname =
                    check_tilde((*cell2).value.atom as *mut libc::c_char)
            }
        } else if strcmp(b"KNP\xe8\xbe\x9e\xe6\x9b\xb8\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
            && knp_dict_file_already_defined == 0 {
            cell1 = cdr(cell1);
            knp_dict_file_already_defined = 1 as libc::c_int;
            while !car(cell1).is_null() {
                dicttype =
                    (*car(cdr(car(cell1)))).value.atom as *mut libc::c_char;
                if strcmp(dicttype,
                          b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0INDEXDB\x00"
                              as *const u8 as *const libc::c_char) == 0 {
                    DICT[6 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0DATA\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[7 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0SIMDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[16 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0CFPDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[20 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0CFCASEDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[17 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0CASEDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[19 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0RENYOUDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[21 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0ADVERBDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[22 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0INDEXDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[14 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0DATA\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[15 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0ID\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x97DB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[38 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82DATA\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[39 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe5\x88\x86\xe9\xa1\x9e\xe8\xaa\x9e\xe5\xbd\x99\xe8\xa1\xa8DB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[1 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"\xe8\xa1\xa8\xe5\xb1\xa4\xe6\xa0\xbcDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[5 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"NTT\xe5\x8d\x98\xe8\xaa\x9eDB\x00" as
                                     *const u8 as *const libc::c_char) == 0 {
                    DICT[2 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"NTT\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0DB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[3 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else if strcmp(dicttype,
                                 b"NTT\xe5\x9b\xba\xe6\x9c\x89\xe5\x90\x8d\xe8\xa9\x9e\xe5\xa4\x89\xe6\x8f\x9b\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xabDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DICT[4 as libc::c_int as usize] =
                        strdup((*car(car(cell1))).value.atom as
                            *const libc::c_char)
                } else {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*car(cdr(car(cell1)))).value.atom);
                    exit(0 as libc::c_int);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe3\x82\xb7\xe3\x82\xbd\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut i: libc::c_int = 0;
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                THESAURUS[0 as libc::c_int as usize].path =
                    strdup((*car(car(cell1))).value.atom as
                        *const libc::c_char);
                THESAURUS[0 as libc::c_int as usize].name =
                    strdup((*car(cdr(car(cell1)))).value.atom as
                        *const libc::c_char);
                THESAURUS[0 as libc::c_int as usize].format =
                    str2ints((*car(cdr(cdr(car(cell1))))).value.atom as
                        *mut libc::c_char);
                i = 0 as libc::c_int;
                while *THESAURUS[0 as libc::c_int as
                    usize].format.offset(i as isize) != 0 {
                    THESAURUS[0 as libc::c_int as usize].code_size +=
                        *THESAURUS[0 as libc::c_int as
                            usize].format.offset(i as isize);
                    i += 1
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe3\x82\xb7\xe3\x82\xbd\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                let mut i_0: libc::c_int = 0;
                Thesaurus = -(1 as libc::c_int);
                if strcasecmp((*cell2).value.atom as *const libc::c_char,
                              b"NONE\x00" as *const u8 as *const libc::c_char)
                    != 0 {
                    /* KNP 辞書ディレクトリ */
                    /* KNP 辞書ファイル */
                    /* 新たなシソーラス */
                    /* 格解析用シソーラス */
                    /* NONEではないとき */
                    i_0 = 0 as libc::c_int;
                    while !THESAURUS[i_0 as usize].name.is_null() &&
                        i_0 < 3 as libc::c_int {
                        if strcasecmp((*cell2).value.atom as
                                          *const libc::c_char,
                                      THESAURUS[i_0 as usize].name) == 0 {
                            Thesaurus = i_0;
                            if OptDisplay == 3 as libc::c_int {
                                fprintf(Outfp,
                                        b"Thesaurus for case analysis ... %s\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        THESAURUS[i_0 as usize].name);
                            }
                            break;
                        } else { i_0 += 1 }
                    }
                    if Thesaurus == -(1 as libc::c_int) {
                        fprintf(stderr,
                                b"%s is invalid in .knprc\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*cell2).value.atom);
                        exit(0 as libc::c_int);
                    }
                }
            }
        } else if strcmp(b"KNP\xe4\xb8\xa6\xe5\x88\x97\xe8\xa7\xa3\xe6\x9e\x90\xe3\x82\xb7\xe3\x82\xbd\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                let mut i_1: libc::c_int = 0;
                ParaThesaurus = -(1 as libc::c_int);
                if strcasecmp((*cell2).value.atom as *const libc::c_char,
                              b"NONE\x00" as *const u8 as *const libc::c_char)
                    != 0 {
                    /* 並列解析用シソーラス */
                    /* NONEではないとき */
                    i_1 = 0 as libc::c_int;
                    while !THESAURUS[i_1 as usize].name.is_null() &&
                        i_1 < 3 as libc::c_int {
                        if strcasecmp((*cell2).value.atom as
                                          *const libc::c_char,
                                      THESAURUS[i_1 as usize].name) == 0 {
                            ParaThesaurus = i_1;
                            if OptDisplay == 3 as libc::c_int {
                                fprintf(Outfp,
                                        b"Thesaurus for para analysis ... %s\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        THESAURUS[i_1 as usize].name);
                            }
                            break;
                        } else { i_1 += 1 }
                    }
                    if ParaThesaurus == -(1 as libc::c_int) {
                        fprintf(stderr,
                                b"%s is invalid in .knprc\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*cell2).value.atom);
                        exit(0 as libc::c_int);
                    }
                }
            }
        } else if strcmp(b"KNP\xe8\x87\xaa\xe5\x8b\x95\xe7\x8d\xb2\xe5\xbe\x97\xe8\xbe\x9e\xe6\x9b\xb8\xe9\x81\xa9\xe7\x94\xa8\xe5\xb1\x9e\xe6\x80\xa7\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype = (*car(car(cell1))).value.atom as *mut libc::c_char;
                if used_auto_dic_features_num >= 10 as libc::c_int {
                    fprintf(stderr,
                            b";; the number of auto dic features exceeded AUTO_DIC_FEATURES_MAX\n\x00"
                                as *const u8 as *const libc::c_char);
                    exit(0 as libc::c_int);
                }
                let fresh2 = used_auto_dic_features_num;
                used_auto_dic_features_num = used_auto_dic_features_num + 1;
                let ref mut fresh3 =
                    *used_auto_dic_features.as_mut_ptr().offset(fresh2 as
                        isize);
                *fresh3 = strdup(dicttype);
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Auto dic feature: %s\n\x00" as *const u8 as
                                *const libc::c_char, dicttype);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut n: libc::c_int = 0 as libc::c_int;
            let mut cn: libc::c_int = 0;
            if cdr(cell1).is_null() {
                fprintf(stderr,
                        b"error in .knprc: %s\n\x00" as *const u8 as
                            *const libc::c_char, (*car(cell1)).value.atom);
                exit(0 as libc::c_int);
            }
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                cn =
                    pp_kstr_to_code((*car(cell1)).value.atom as
                        *mut libc::c_char);
                if cn == -(10 as libc::c_int) {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*car(cell1)).value.atom);
                    exit(0 as libc::c_int);
                }
                let fresh4 = n;
                n = n + 1;
                *DiscAddedCases.as_mut_ptr().offset(fresh4 as isize) = cn;
                cell1 = cdr(cell1)
            }
            *DiscAddedCases.as_mut_ptr().offset(n as isize) =
                -(10 as libc::c_int)
        } else if strcmp(b"KNP\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\x8e\xa2\xe7\xb4\xa2\xe7\xaf\x84\xe5\x9b\xb2\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut pp: libc::c_int = 0;
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype = (*car(car(cell1))).value.atom as *mut libc::c_char;
                pp = pp_kstr_to_code(dicttype);
                if pp == 0 as libc::c_int || pp == -(10 as libc::c_int) {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char, dicttype);
                    exit(0 as libc::c_int);
                }
                *LocationLimit.as_mut_ptr().offset(pp as isize) =
                    atoi((*car(cdr(car(cell1)))).value.atom as
                        *const libc::c_char);
                if *LocationLimit.as_mut_ptr().offset(pp as isize) <=
                    0 as libc::c_int {
                    *LocationLimit.as_mut_ptr().offset(pp as isize) =
                        -(10 as libc::c_int)
                }
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Location category order limit ... %d for %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            *LocationLimit.as_mut_ptr().offset(pp as isize),
                            dicttype);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\x8e\xa2\xe7\xb4\xa2\xe9\x96\xbe\xe5\x80\xa4\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut pp_0: libc::c_int = 0;
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype = (*car(car(cell1))).value.atom as *mut libc::c_char;
                pp_0 = pp_kstr_to_code(dicttype);
                if pp_0 == 1 as libc::c_int {
                    AntecedentDecideThresholdForGa =
                        atof((*car(cdr(car(cell1)))).value.atom as
                            *const libc::c_char) as libc::c_float
                } else if pp_0 == 2 as libc::c_int {
                    AntecedentDecideThresholdPredGeneral =
                        atof((*car(cdr(car(cell1)))).value.atom as
                            *const libc::c_char) as libc::c_float
                } else if pp_0 == 3 as libc::c_int {
                    AntecedentDecideThresholdForNi =
                        atof((*car(cdr(car(cell1)))).value.atom as
                            *const libc::c_char) as libc::c_float
                } else {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char, dicttype);
                    exit(0 as libc::c_int);
                }
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Antecedent dicide th ... %s for %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*car(cdr(car(cell1)))).value.atom, dicttype);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"KNP\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\x8e\xa2\xe7\xb4\xa2\xe6\x96\x87\xe6\x95\xb0\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                PrevSentenceLimit =
                    atoi((*cell2).value.atom as *const libc::c_char);
                if PrevSentenceLimit < 0 as libc::c_int ||
                    PrevSentenceLimit >= 512 as libc::c_int {
                    fprintf(stderr,
                            b"%d is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char, PrevSentenceLimit);
                    exit(0 as libc::c_int);
                }
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Previous sentence limit ... %d\n\x00" as
                                *const u8 as *const libc::c_char,
                            PrevSentenceLimit);
                }
            }
        } else if strcmp(b"KNP\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\x8e\xa2\xe7\xb4\xa2\xe9\xa0\x86\xe5\xba\x8f\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut pp_1: libc::c_int = 0;
            let mut count: libc::c_int = 0;
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype = (*car(car(cell1))).value.atom as *mut libc::c_char;
                pp_1 = pp_kstr_to_code(dicttype);
                if pp_1 == 0 as libc::c_int || pp_1 == -(10 as libc::c_int) {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char, dicttype);
                    exit(0 as libc::c_int);
                }
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Location category order for %s:\x00" as
                                *const u8 as *const libc::c_char, dicttype);
                }
                count = 0 as libc::c_int;
                cell2 = cdr(car(cell1));
                while !car(cell2).is_null() {
                    (*LocationOrder.as_mut_ptr().offset(pp_1 as
                        isize))[count as
                        usize]
                        =
                        loc_name_to_code((*car(cell2)).value.atom as
                            *mut libc::c_char);
                    if (*LocationOrder.as_mut_ptr().offset(pp_1 as
                        isize))[count
                        as
                        usize]
                        < 0 as libc::c_int {
                        (*LocationOrder.as_mut_ptr().offset(pp_1 as
                            isize))[count
                            as
                            usize]
                            = -(10 as libc::c_int)
                    }
                    if OptDisplay == 3 as libc::c_int {
                        fprintf(Outfp,
                                b" %s\x00" as *const u8 as
                                    *const libc::c_char,
                                loc_code_to_str((*LocationOrder.as_mut_ptr().offset(pp_1
                                    as
                                    isize))[count
                                    as
                                    usize]));
                    }
                    count += 1;
                    cell2 = cdr(cell2)
                }
                (*LocationOrder.as_mut_ptr().offset(pp_1 as
                    isize))[count as
                    usize] =
                    -(10 as libc::c_int);
                if OptDisplay == 3 as libc::c_int {
                    fputs(b"\n\x00" as *const u8 as *const libc::c_char,
                          Outfp);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"NE\xe3\x83\xa2\xe3\x83\x87\xe3\x83\xab\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                CRFFileNE =
                    check_tilde((*cell2).value.atom as *mut libc::c_char);
                if OptNE != 0 && OptNECRF != 0 &&
                    OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"NE model file ... %s\n\x00" as *const u8 as
                                *const libc::c_char, CRFFileNE);
                }
            }
        } else if strcmp(b"\xe5\x90\x8c\xe7\xbe\xa9\xe8\xa1\xa8\xe7\x8f\xbe\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .knprc\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                SynonymFile =
                    check_tilde((*cell2).value.atom as *mut libc::c_char);
                if OptEllipsis & 8 as libc::c_int != 0 &&
                    OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"Synonym db file ... %s\n\x00" as *const u8 as
                                *const libc::c_char, SynonymFile);
                }
            }
        } else if strcmp(b"\xe5\x88\x86\xe5\xb8\x83\xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype =
                    (*car(cdr(car(cell1)))).value.atom as *mut libc::c_char;
                if strcmp(dicttype,
                          b"\xe5\x88\x86\xe5\xb8\x83\xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6MIDB\x00"
                              as *const u8 as *const libc::c_char) == 0 {
                    DistSimFile =
                        check_tilde((*car(car(cell1))).value.atom as
                            *mut libc::c_char);
                    if OptDisplay == 3 as libc::c_int {
                        fprintf(Outfp,
                                b"Distsim midb file ... %s\n\x00" as *const u8
                                    as *const libc::c_char, DistSimFile);
                    }
                } else if strcmp(dicttype,
                                 b"\xe5\x88\x86\xe5\xb8\x83\xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6SIMDB\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DistSimDB =
                        check_tilde((*car(car(cell1))).value.atom as
                            *mut libc::c_char);
                    if OptDisplay == 3 as libc::c_int {
                        fprintf(Outfp,
                                b"Distsim simdb file ... %s\n\x00" as
                                    *const u8 as *const libc::c_char,
                                DistSimDB);
                    }
                } else if strcmp(dicttype,
                                 b"\xe5\x88\x86\xe5\xb8\x83\xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6WORDLIST\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    DistSimWordList =
                        check_tilde((*car(car(cell1))).value.atom as
                            *mut libc::c_char);
                    if OptDisplay == 3 as libc::c_int {
                        fprintf(Outfp,
                                b"Distsim wordlist file ... %s\n\x00" as
                                    *const u8 as *const libc::c_char,
                                DistSimWordList);
                    }
                } else {
                    fprintf(stderr,
                            b"%s is invalid in .knprc\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*car(cdr(car(cell1)))).value.atom);
                    exit(0 as libc::c_int);
                }
                cell1 = cdr(cell1)
            }
        } else if strcmp(b"\xe6\xb1\xba\xe5\xae\x9a\xe6\x9c\xa8\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                             as *const u8 as *const libc::c_char,
                         (*car(cell1)).value.atom as *const libc::c_char) == 0
        {
            let mut pp_2: libc::c_int = 0;
            cell1 = cdr(cell1);
            while !car(cell1).is_null() {
                dicttype =
                    (*car(cdr(car(cell1)))).value.atom as *mut libc::c_char;
                if strcmp(dicttype,
                          b"ALL\x00" as *const u8 as *const libc::c_char) == 0
                {
                    pp_2 = 0 as libc::c_int
                } else {
                    pp_2 = pp_kstr_to_code(dicttype);
                    if pp_2 == 0 as libc::c_int ||
                        pp_2 == -(10 as libc::c_int) {
                        fprintf(stderr,
                                b"%s is invalid in .knprc\n\x00" as *const u8
                                    as *const libc::c_char, dicttype);
                        exit(0 as libc::c_int);
                    }
                }
                let ref mut fresh5 =
                    *DTFile.as_mut_ptr().offset(pp_2 as isize);
                *fresh5 =
                    check_tilde((*car(car(cell1))).value.atom as
                        *mut libc::c_char);
                if (OptDiscPredMethod == 3 as libc::c_int ||
                    OptDiscNounMethod == 3 as libc::c_int) &&
                    OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"DT file ... %s for %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            *DTFile.as_mut_ptr().offset(pp_2 as isize),
                            dicttype);
                }
                cell1 = cdr(cell1)
            }
        }
    }
    /* 自動獲得辞書適用属性 */
    /* 省略解析格 */
    /* 省略解析探索範囲 */
    /* 省略解析探索閾値 */
    /* 省略解析探索文数 */
    /* 省略解析探索順序 */
    /* 分布類似度ファイル */
    /* knprc にルールが指定されていない場合のデフォルトルール */
    if CurrentRuleNum == 0 as libc::c_int {
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Setting default rules ... \x00" as *const u8 as
                        *const libc::c_char);
        }
        RuleNumMax = 12 as libc::c_int;
        RULE =
            realloc_data(RULE as *mut libc::c_void,
                         (::std::mem::size_of::<RuleVector>() as
                             libc::c_ulong).wrapping_mul(RuleNumMax as libc::c_ulong),
                         b"read_rc\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char) as *mut RuleVector;
        /* mrph_homo 同形異義語 */
        let ref mut fresh6 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh6 =
            strdup(b"mrph_homo\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 3 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* mrph_filter 形態素-前処理 ルールループ先行 */
        let ref mut fresh7 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh7 =
            strdup(b"mrph_filter\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 16 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* mrph_auto_dic 形態素 ルールループ先行 */
        let ref mut fresh8 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh8 =
            strdup(b"mrph_auto_dic\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* mrph_basic 形態素 ルールループ先行 */
        let ref mut fresh9 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh9 =
            strdup(b"mrph_basic\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* bnst_basic 文節 逆方向 ルールループ先行 */
        let ref mut fresh10 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh10 =
            strdup(b"bnst_basic\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 2 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        /* bnst_type 文節 逆方向 BREAK */
        let ref mut fresh11 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh11 =
            strdup(b"bnst_type\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 2 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        /* bnst_etc 文節 逆方向 ルールループ先行 */
        let ref mut fresh12 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh12 =
            strdup(b"bnst_etc\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 2 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        /* kakari_uke 係り受け */
        let ref mut fresh13 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh13 =
            strdup(b"kakari_uke\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 4 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* koou 呼応 */
        let ref mut fresh14 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh14 = strdup(b"koou\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 5 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction = 1 as libc::c_int;
        CurrentRuleNum += 1;
        /* bnst_basic 基本句 逆方向 ルールループ先行 */
        let ref mut fresh15 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh15 =
            strdup(b"bnst_basic\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 1 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 11 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        /* case_analysis 基本句 逆方向 */
        let ref mut fresh16 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh16 =
            strdup(b"case_analysis\x00" as *const u8 as *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 11 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        /* tag_postprocess 基本句-後処理 逆方向 */
        let ref mut fresh17 = (*RULE.offset(CurrentRuleNum as isize)).file;
        *fresh17 =
            strdup(b"tag_postprocess\x00" as *const u8 as
                *const libc::c_char);
        (*RULE.offset(CurrentRuleNum as isize)).mode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).breakmode = 0 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).type_0 = 14 as libc::c_int;
        (*RULE.offset(CurrentRuleNum as isize)).direction =
            -(1 as libc::c_int);
        CurrentRuleNum += 1;
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"done.\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn server_read_rc(mut fp: *mut FILE) {
    clear_rule_configuration();
    set_cha_getc();
    read_rc(fp);
    unset_cha_getc();
}

#[no_mangle]
pub unsafe extern "C" fn check_data_newer_than_rule(mut data: time_t, mut datapath: *mut libc::c_char) {
    /* ルールファイルとの時間チェック */
    let mut rulename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: libc::c_int = 0;
    let mut sb: stat =
        stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
    rulename = strdup(datapath);
    *rulename.offset(strlen(rulename) as
        isize).offset(-(5 as libc::c_int as isize)) =
        '\u{0}' as i32 as libc::c_char;
    strcat(rulename, b".rule\x00" as *const u8 as *const libc::c_char);
    status = ctools::stat(rulename, &mut sb);
    if status == 0 {
        if data < sb.st_mtim.tv_sec {
            fprintf(stderr,
                    b";; %s: older than rule file!\n\x00" as *const u8 as
                        *const libc::c_char, datapath);
        }
    }
    free(rulename as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn check_rule_filename(mut file: *mut libc::c_char) -> *mut libc::c_char {
    /* ルールファイル (*.data) の fullpath を返す関数 */
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: libc::c_int = 0;
    let mut sb: stat =
        stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
    if Knprule_Dirname.is_null() {
        Knprule_Dirname =
            strdup(b"/usr/local/share/knp/rule\x00" as *const u8 as
                *const libc::c_char)
    }
    fullname =
        malloc_data(strlen(Knprule_Dirname).wrapping_add(strlen(file)).wrapping_add(7
            as
            libc::c_int
            as
            libc::c_ulong),
                    b"check_rule_filename\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    sprintf(fullname, b"%s/%s.data\x00" as *const u8 as *const libc::c_char,
            Knprule_Dirname, file);
    /* dir + filename + ".data" */
    status = ctools::stat(fullname, &mut sb);
    if status < 0 as libc::c_int {
        *fullname.offset(strlen(fullname) as
            isize).offset(-(5 as libc::c_int as isize)) =
            '\u{0}' as i32 as libc::c_char;
        /* dir + filename */
        status = ctools::stat(fullname, &mut sb);
        if status < 0 as libc::c_int {
            /* filename + ".data" */
            if *file as libc::c_int == '~' as i32 &&
                {
                    home =
                        getenv(b"HOME\x00" as *const u8 as
                            *const libc::c_char);
                    !home.is_null()
                } {
                free(fullname as *mut libc::c_void);
                fullname =
                    malloc_data(strlen(home).wrapping_add(strlen(file)).wrapping_add(6
                        as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"check_rule_filename\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                sprintf(fullname,
                        b"%s%s.data\x00" as *const u8 as *const libc::c_char,
                        home, strchr(file, '/' as i32));
            } else {
                sprintf(fullname,
                        b"%s.data\x00" as *const u8 as *const libc::c_char,
                        file);
            }
            status = ctools::stat(fullname, &mut sb);
            if status < 0 as libc::c_int {
                *fullname.offset(strlen(fullname) as
                    isize).offset(-(5 as libc::c_int as
                    isize)) =
                    '\u{0}' as i32 as libc::c_char;
                /* filename */
                status = ctools::stat(fullname, &mut sb);
                if status < 0 as libc::c_int {
                    fprintf(stderr,
                            b"%s: No such file.\n\x00" as *const u8 as
                                *const libc::c_char, fullname);
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    /* ルールファイルとの時間チェック */
    check_data_newer_than_rule(sb.st_mtim.tv_sec, fullname);
    return fullname;
}

#[no_mangle]
pub unsafe extern "C" fn check_dict_filename(mut file: *mut libc::c_char, mut flag: libc::c_int) -> *mut libc::c_char {
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: libc::c_int = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if Knpdict_Dirname.is_null() {
        Knpdict_Dirname =
            strdup(b"/usr/local/share/knp/dict\x00" as *const u8 as
                *const libc::c_char)
    }
    fullname =
        malloc_data(strlen(Knpdict_Dirname).wrapping_add(strlen(file)).wrapping_add(2
            as
            libc::c_int
            as
            libc::c_ulong),
                    b"check_dict_filename\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    sprintf(fullname, b"%s/%s\x00" as *const u8 as *const libc::c_char,
            Knpdict_Dirname, file);
    /* dir + filename */
    status = ctools::stat(fullname, &mut sb);
    if status < 0 as libc::c_int {
        free(fullname as *mut libc::c_void);
        if *file as libc::c_int == '~' as i32 &&
            {
                home =
                    getenv(b"HOME\x00" as *const u8 as
                        *const libc::c_char);
                !home.is_null()
            } {
            fullname =
                malloc_data(strlen(home).wrapping_add(strlen(file)),
                            b"check_dict_filename\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            sprintf(fullname, b"%s%s\x00" as *const u8 as *const libc::c_char,
                    home, strchr(file, '/' as i32));
        } else { fullname = strdup(file) }
        status = ctools::stat(fullname, &mut sb);
        if status < 0 as libc::c_int {
            /* flag が FALSE のときはファイルが存在するかどうかチェックしない */
            if flag == 0 as libc::c_int { return fullname; }
            fprintf(stderr,
                    b"%s: No such file.\n\x00" as *const u8 as
                        *const libc::c_char, fullname);
            exit(1 as libc::c_int);
        }
    }
    return fullname;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn open_dict(mut dic_num: libc::c_int,
                                   mut dic_name: *mut libc::c_char,
                                   mut exist: *mut libc::c_int) -> DBM_FILE
/*==================================================================*/
{
    let mut index_db_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut db: DBM_FILE = 0 as *mut CDB_FILE;
    if !DICT[dic_num as usize].is_null() {
        index_db_filename =
            check_dict_filename(DICT[dic_num as usize],
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        index_db_filename = check_dict_filename(dic_name, 0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                index_db_filename);
    }
    db = db_read_open(index_db_filename);
    if db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        *exist = 0 as libc::c_int
    } else {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
        }
        *exist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(index_db_filename as *mut libc::c_void);
    return db;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_configfile(mut opfile: *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 39 as libc::c_int {
        DICT[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    THESAURUS[0 as libc::c_int as usize].path = 0 as *mut libc::c_char;
    THESAURUS[0 as libc::c_int as usize].name = 0 as *mut libc::c_char;
    THESAURUS[0 as libc::c_int as usize].format = 0 as *mut libc::c_int;
    THESAURUS[0 as libc::c_int as usize].exist = 0 as libc::c_int;
    THESAURUS[1 as libc::c_int as usize].path = 0 as *mut libc::c_char;
    THESAURUS[1 as libc::c_int as usize].name =
        strdup(b"BGH\x00" as *const u8 as *const libc::c_char);
    THESAURUS[1 as libc::c_int as usize].format = 0 as *mut libc::c_int;
    THESAURUS[1 as libc::c_int as usize].code_size = 11 as libc::c_int;
    THESAURUS[2 as libc::c_int as usize].path = 0 as *mut libc::c_char;
    THESAURUS[2 as libc::c_int as usize].name =
        strdup(b"NTT\x00" as *const u8 as *const libc::c_char);
    THESAURUS[2 as libc::c_int as usize].format = 0 as *mut libc::c_int;
    THESAURUS[2 as libc::c_int as usize].code_size = 12 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 44 as libc::c_int {
        let ref mut fresh18 = *DTFile.as_mut_ptr().offset(i as isize);
        *fresh18 = 0 as *mut libc::c_char;
        i += 1
    }
    read_rc(find_rc_file(opfile));
}
/*====================================================================
				 END
====================================================================*/
