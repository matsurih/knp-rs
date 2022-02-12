#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{atoi, fclose, fopen, fprintf, fputs, free, strcmp, usage};
use crate::configfile::check_rule_filename;
use crate::ctools::{car, cdr, exit, LineNo, LineNoForError, malloc_data, Outfp, realloc, s_feof, s_read, stderr};
use crate::feature::{list2feature, list2feature_pattern};
use crate::regexp::{store_regexpbnsts, store_regexpmrphs};
use crate::structs::{BnstRule, CDB_FILE, DpndRule, FEATURE_PATTERN, GeneralRuleType, HomoRule, KoouRule, MrphRule, REGEXPBNSTS, REGEXPMRPHS};
use crate::tools::{Case_name, OptDisplay};
use crate::types::{CELL, DBM_FILE, FEATURE, FILE, RuleVector};

#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
/* global variable declaration */
#[no_mangle]
pub static mut HomoRuleArray: [HomoRule; 128] = [HomoRule { pre_pattern: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS, pattern: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS, f: 0 as *const FEATURE as *mut FEATURE }; 128];
#[no_mangle]
pub static mut CurHomoRuleSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut KoouRuleArray: [KoouRule; 124] = [KoouRule { start_pattern: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS, end_pattern: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS, uke_pattern: 0 as *const REGEXPMRPHS as *mut REGEXPMRPHS, dpnd_type: 0 }; 124];
#[no_mangle]
pub static mut CurKoouRuleSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut DpndRuleArray: [DpndRule; 128] = [DpndRule { dependant: FEATURE_PATTERN { fp: [0 as *const FEATURE as *mut FEATURE; 16] }, governor: [FEATURE_PATTERN { fp: [0 as *const FEATURE as *mut FEATURE; 16] }; 35], dpnd_type: [0; 35], barrier: FEATURE_PATTERN { fp: [0 as *const FEATURE as *mut FEATURE; 16] }, preference: 0, decide: 0 }; 128];
#[no_mangle]
pub static mut CurDpndRuleSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ContRuleArray: [BnstRule; 256] = [BnstRule { pre_pattern: 0 as *const REGEXPBNSTS as *mut REGEXPBNSTS, self_pattern: 0 as *const REGEXPBNSTS as *mut REGEXPBNSTS, post_pattern: 0 as *const REGEXPBNSTS as *mut REGEXPBNSTS, f: 0 as *const FEATURE as *mut FEATURE }; 256];
#[no_mangle]
pub static mut ContRuleSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ExistEtcRule: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut GeneralRuleArray: *mut GeneralRuleType = 0 as *const GeneralRuleType as *mut GeneralRuleType;
#[no_mangle]
pub static mut GeneralRuleNum: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut GeneralRuleMax: libc::c_int = 0 as libc::c_int;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_mrph_rule(mut file_name: *mut libc::c_char,
                                        mut rp: *mut MrphRule,
                                        mut count: *mut libc::c_int,
                                        mut max: libc::c_int)
/*==================================================================*/
{
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    /* 重複してルールファイルが指定されているとき */
    if *count != 0 {
        fprintf(stderr,
                b";; Mrph rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        store_regexpmrphs(&mut (*rp).pre_pattern, car(body_cell));
        store_regexpmrphs(&mut (*rp).self_pattern, car(cdr(body_cell)));
        store_regexpmrphs(&mut (*rp).post_pattern, car(cdr(cdr(body_cell))));
        (*rp).f = 0 as *mut FEATURE;
        list2feature(cdr(cdr(cdr(body_cell))), &mut (*rp).f);
        *count += 1;
        if *count == max {
            fprintf(stderr,
                    b";; Too many Rule for %s.\n\x00" as *const u8 as
                        *const libc::c_char, file_name);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    free(file_name as *mut libc::c_void);
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn case2num(mut cp: *mut libc::c_char) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while **Case_name.as_mut_ptr().offset(i as isize) != 0 {
        if strcmp(cp, *Case_name.as_mut_ptr().offset(i as isize)) == 0 {
            return i;
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_koou_rule(mut file_name: *mut libc::c_char)
/*==================================================================*/
{
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    let mut rp: *mut KoouRule = KoouRuleArray.as_mut_ptr();
    /* 重複してルールファイルが指定されているとき */
    if CurKoouRuleSize != 0 {
        fprintf(stderr,
                b";; Koou rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name); /* default */
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    free(file_name as *mut libc::c_void);
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        store_regexpmrphs(&mut (*rp).start_pattern, car(body_cell));
        body_cell = cdr(body_cell);
        store_regexpmrphs(&mut (*rp).end_pattern, car(body_cell));
        (*rp).dpnd_type = 'D' as i32 as libc::c_char;
        body_cell = cdr(body_cell);
        if !car(body_cell).is_null() {
            store_regexpmrphs(&mut (*rp).uke_pattern, car(body_cell));
        } else { (*rp).uke_pattern = 0 as *mut REGEXPMRPHS }
        body_cell = cdr(body_cell);
        if !car(body_cell).is_null() {
            (*rp).dpnd_type = *(*car(body_cell)).value.atom as libc::c_char
        }
        CurKoouRuleSize += 1;
        if CurKoouRuleSize == 124 as libc::c_int {
            fprintf(stderr,
                    b";; Too many KoouRule.\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_homo_rule(mut file_name: *mut libc::c_char)
/*==================================================================*/
{
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    let mut rp: *mut HomoRule = HomoRuleArray.as_mut_ptr();
    /* 重複してルールファイルが指定されているとき */
    if CurHomoRuleSize != 0 {
        fprintf(stderr,
                b";; Homo rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    free(file_name as *mut libc::c_void);
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        /* 前の形態素列ルールの読込 */
        store_regexpmrphs(&mut (*rp).pre_pattern, car(body_cell));
        /* homoの形態素列ルールの読込 */
        store_regexpmrphs(&mut (*rp).pattern, car(cdr(body_cell)));
        list2feature(cdr(cdr(body_cell)), &mut (*rp).f);
        CurHomoRuleSize += 1;
        if CurHomoRuleSize == 128 as libc::c_int {
            fprintf(stderr,
                    b";; Too many HomoRule.\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_bnst_rule(mut file_name: *mut libc::c_char,
                                        mut rp: *mut BnstRule,
                                        mut count: *mut libc::c_int,
                                        mut max: libc::c_int)
/*==================================================================*/
{
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    /* 重複してルールファイルが指定されているとき */
    if *count != 0 {
        fprintf(stderr,
                b";; Bnst rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    free(file_name as *mut libc::c_void);
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        store_regexpbnsts(&mut (*rp).pre_pattern, car(body_cell));
        store_regexpbnsts(&mut (*rp).self_pattern, car(cdr(body_cell)));
        store_regexpbnsts(&mut (*rp).post_pattern, car(cdr(cdr(body_cell))));
        (*rp).f = 0 as *mut FEATURE;
        list2feature(cdr(cdr(cdr(body_cell))), &mut (*rp).f);
        *count += 1;
        if *count == max {
            fprintf(stderr,
                    b";; Too many BnstRule.\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_dpnd_rule(mut file_name: *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    let mut loop_cell: *mut CELL = 0 as *mut CELL;
    let mut rp: *mut DpndRule = DpndRuleArray.as_mut_ptr();
    /* 重複してルールファイルが指定されているとき */
    if CurDpndRuleSize != 0 {
        fprintf(stderr,
                b";; Dpnd rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char,
                file_name); /* dpnd_type[i] != 0 がgovernorのある印 */
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    free(file_name as *mut libc::c_void);
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        list2feature_pattern(&mut (*rp).dependant, car(body_cell));
        loop_cell = car(cdr(body_cell));
        i = 0 as libc::c_int;
        while !car(loop_cell).is_null() {
            list2feature_pattern(&mut *(*rp).governor.as_mut_ptr().offset(i as
                isize),
                                 car(car(loop_cell)));
            (*rp).dpnd_type[i as usize] =
                *(*car(cdr(car(loop_cell)))).value.atom as libc::c_char;
            loop_cell = cdr(loop_cell);
            i += 1;
            if i == 35 as libc::c_int {
                fprintf(stderr,
                        b";; Too many Governors in a DpndRule.\x00" as
                            *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
        (*rp).dpnd_type[i as usize] = 0 as libc::c_int as libc::c_char;
        list2feature_pattern(&mut (*rp).barrier, car(cdr(cdr(body_cell))));
        (*rp).preference =
            atoi((*car(cdr(cdr(cdr(body_cell))))).value.atom as
                *const libc::c_char);
        /* 一意に決定するかどうか */
        if !car(cdr(cdr(cdr(cdr(body_cell))))).is_null() &&
            strcmp((*car(cdr(cdr(cdr(cdr(body_cell)))))).value.atom as
                       *const libc::c_char,
                   b"U\x00" as *const u8 as *const libc::c_char) == 0 {
            (*rp).decide = 1 as libc::c_int
        } else { (*rp).decide = 0 as libc::c_int }
        CurDpndRuleSize += 1;
        if CurDpndRuleSize == 128 as libc::c_int {
            fprintf(stderr,
                    b";; Too many DpndRule.\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_dpnd_rule_for_chinese(mut file_name:
                                                    *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut body_cell: *mut CELL = 0 as *mut CELL;
    let mut loop_cell: *mut CELL = 0 as *mut CELL;
    // let mut prob_cell: *mut CELL = 0 as *mut CELL;
    let mut rp: *mut DpndRule = DpndRuleArray.as_mut_ptr();
    /* 重複してルールファイルが指定されているとき */
    if CurDpndRuleSize != 0 {
        fprintf(stderr,
                b";; Dpnd rule is duplicated (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    file_name = check_rule_filename(file_name);
    fp = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(stderr,
                b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Reading %s ... \x00" as *const u8 as *const libc::c_char,
                file_name);
    }
    free(file_name as *mut libc::c_void);
    LineNo = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        body_cell = s_read(fp);
        list2feature_pattern(&mut (*rp).dependant, car(body_cell));
        loop_cell = car(cdr(body_cell));
        i = 0 as libc::c_int;
        while !car(loop_cell).is_null() {
            list2feature_pattern(&mut *(*rp).governor.as_mut_ptr().offset(i as
                isize),
                                 car(loop_cell));
            /* 	    strcpy(rp->gov_word[i], _Atom(car(car(car(cdr(car(car(loop_cell)))))))); */
            /* 	    rp->dpnd_type[i] = *(_Atom(car(cdr(car(loop_cell))))); */
            /* 	    prob_cell = car(cdr(cdr(car(loop_cell)))); */
            /* 	    rp->prob_LtoR[i] = atof(_Atom(car(car(prob_cell)))); */
            /* 	    rp->prob_RtoL[i] = atof(_Atom(car(car(cdr(prob_cell))))); */
            /* 	    rp->count[i] = atoi(_Atom(car(cdr(cdr(cdr(car(loop_cell))))))); */
            /* 	    strcpy(rp->dpnd_relation[i], _Atom(car(cdr(cdr(cdr(cdr(car(loop_cell)))))))); */
            loop_cell =
                cdr(loop_cell); /* dpnd_type[i] != 0 がgovernorのある印 */
            i += 1;
            if i == 35 as libc::c_int {
                fprintf(stderr,
                        b";; Too many Governors in a DpndRule.\x00" as
                            *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
        (*rp).dpnd_type[i as usize] = 0 as libc::c_int as libc::c_char;
        num += 1;
        list2feature_pattern(&mut (*rp).barrier, car(cdr(cdr(body_cell))));
        (*rp).preference =
            atoi((*car(cdr(cdr(cdr(body_cell))))).value.atom as
                *const libc::c_char);
        /* 一意に決定するかどうか */
        if !car(cdr(cdr(cdr(cdr(body_cell))))).is_null() &&
            strcmp((*car(cdr(cdr(cdr(cdr(body_cell)))))).value.atom as
                       *const libc::c_char,
                   b"U\x00" as *const u8 as *const libc::c_char) == 0 {
            (*rp).decide = 1 as libc::c_int
        } else { (*rp).decide = 0 as libc::c_int }
        CurDpndRuleSize += 1;
        if CurDpndRuleSize == 128 as libc::c_int {
            fprintf(stderr,
                    b";; Too many DpndRule.\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        rp = rp.offset(1)
    }
    if OptDisplay == 3 as libc::c_int {
        fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_etc_rule(mut flag: libc::c_int)
/*==================================================================*/
{
    if ExistEtcRule != 0 { usage(); }
    ExistEtcRule = flag;
    if flag == 1 as libc::c_int || flag == 3 as libc::c_int {
        EtcRuleArray =
            malloc_data((::std::mem::size_of::<MrphRule>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"init_etc_rule\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut MrphRule as *mut libc::c_void
    } else if flag == 2 as libc::c_int {
        EtcRuleArray =
            malloc_data((::std::mem::size_of::<BnstRule>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"init_etc_rule\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut BnstRule as *mut libc::c_void
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_etc_rule(mut file_name: *mut libc::c_char,
                                       mut rp: *mut libc::c_void,
                                       mut count: *mut libc::c_int,
                                       mut max: libc::c_int)
/*==================================================================*/
{
    if ExistEtcRule == 1 as libc::c_int || ExistEtcRule == 3 as libc::c_int {
        read_mrph_rule(file_name, rp as *mut MrphRule, count, max);
    } else if ExistEtcRule == 2 as libc::c_int {
        read_bnst_rule(file_name, rp as *mut BnstRule, count, max);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_general_rule(mut rule: *mut RuleVector)
/*==================================================================*/
{
    if GeneralRuleNum >= GeneralRuleMax {
        GeneralRuleMax += 10 as libc::c_int;
        GeneralRuleArray =
            realloc(GeneralRuleArray as *mut libc::c_void,
                    (::std::mem::size_of::<GeneralRuleType>() as
                        libc::c_ulong).wrapping_mul(GeneralRuleMax as
                        libc::c_ulong)) as
                *mut GeneralRuleType
    }
    /* 各種タイプ, モードの伝播 */
    (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0 =
        (*rule).type_0;
    (*GeneralRuleArray.offset(GeneralRuleNum as isize)).mode = (*rule).mode;
    (*GeneralRuleArray.offset(GeneralRuleNum as isize)).breakmode =
        (*rule).breakmode;
    (*GeneralRuleArray.offset(GeneralRuleNum as isize)).direction =
        (*rule).direction;
    (*GeneralRuleArray.offset(GeneralRuleNum as isize)).CurRuleSize =
        0 as libc::c_int;
    if (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0 ==
        1 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0 ==
            16 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0 ==
            6 as libc::c_int {
        let ref mut fresh0 =
            (*GeneralRuleArray.offset(GeneralRuleNum as isize)).RuleArray;
        *fresh0 =
            malloc_data((::std::mem::size_of::<MrphRule>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"read_general_rule\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut MrphRule as *mut libc::c_void;
        read_mrph_rule((*rule).file,
                       (*GeneralRuleArray.offset(GeneralRuleNum as
                           isize)).RuleArray as
                           *mut MrphRule,
                       &mut (*GeneralRuleArray.offset(GeneralRuleNum as
                           isize)).CurRuleSize,
                       1024 as libc::c_int);
    } else if (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0 ==
        11 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0
            == 2 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0
            == 12 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0
            == 13 as libc::c_int ||
        (*GeneralRuleArray.offset(GeneralRuleNum as isize)).type_0
            == 14 as libc::c_int {
        let ref mut fresh1 =
            (*GeneralRuleArray.offset(GeneralRuleNum as isize)).RuleArray;
        *fresh1 =
            malloc_data((::std::mem::size_of::<BnstRule>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"read_general_rule\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut BnstRule as *mut libc::c_void;
        read_bnst_rule((*rule).file,
                       (*GeneralRuleArray.offset(GeneralRuleNum as
                           isize)).RuleArray as
                           *mut BnstRule,
                       &mut (*GeneralRuleArray.offset(GeneralRuleNum as
                           isize)).CurRuleSize,
                       1024 as libc::c_int);
    }
    GeneralRuleNum += 1;
}
/*====================================================================
                               END
====================================================================*/
