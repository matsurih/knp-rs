#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::db::{db_close, db_get};
use crate::{case_analysis, Chi_dpnd_matrix, Chi_pa_matrix, Chi_pos_matrix, ctools, Dpnd_matrix, Mask_matrix, Quote_matrix, tools, tree_conv};
use crate::case_analysis::{assign_nil_assigned_components, call_case_analysis, ClearCPMcache, InitCPMcache, noun_lexical_disambiguation_by_case_analysis, verb_lexical_disambiguation_by_case_analysis};
use crate::case_ipal::{check_examples, get_chi_pa, malloc_db_buf};
use crate::configfile::open_dict;
use crate::context::OptUseSmfix;
use crate::ctools::{assign_cfeature, check_feature, Language, OptAnalysis, OptChiPos, Outfp, stderr};
use crate::feature::{feature_pattern_match, print_feature};
use crate::lib_print::{print_kakari, print_result};
use crate::lib_sm::{assign_ga_subject, fix_sm_place, specify_sm_from_cf};
use crate::read_data::{assign_general_feature, calc_bnst_length, dpnd_info_to_tag_pm, get_mrph_rep, get_mrph_rep_length};
use crate::read_rule::{case2num, CurDpndRuleSize, DpndRuleArray};
use crate::structs::{CDB_FILE, DPND, DpndRule, MRPH_DATA, TOTAL_MGR};
use crate::tools::{Chi_root_prob_matrix, Chi_word_pos, Chi_word_type, left_arg, OptCaseFlag, OptChiGenerative, OptCKY, OptDisplay, OptEllipsis, OptExpress, OptInput, OptNbest, OptSemanticHead, right_arg};
use crate::tree_conv::{bnst_to_tag_tree, find_head_mrph_from_dpnd_bnst, make_dpnd_tree};
use crate::types::{BNST_DATA, CF_PRED_MGR, CHECK_DATA, DBM_FILE, FEATURE, SENTENCE_DATA, TAG_DATA};


#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double { return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char); }
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int { return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char, 10 as libc::c_int) as libc::c_int; }
#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
/*====================================================================

                                 依存構造解析
 
                                                S.Kurohashi 93. 5.31

       $Id$

====================================================================*/
/* DB file for Chinese dpnd rule */
#[no_mangle]
pub static mut chi_dpnd_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut CHIDpndExist: libc::c_int = 0;
#[no_mangle]
pub static mut chi_dpnd_prob_db: DBM_FILE =
    0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut CHIDpndProbExist: libc::c_int = 0;
#[no_mangle]
pub static mut chi_dis_comma_db: DBM_FILE =
    0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut CHIDisCommaExist: libc::c_int = 0;
#[no_mangle]
pub static mut chi_case_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut CHICaseExist: libc::c_int = 0;
#[no_mangle]
pub static mut chi_pos_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut CHIPosExist: libc::c_int = 0;
#[no_mangle]
pub static mut Possibility: libc::c_int = 0;
/* 依存構造の可能性の何番目か */
static mut dpndID: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut giga_weight: libc::c_double = 0.4f64;
#[no_mangle]
pub static mut prob_bk_weight_1: libc::c_double = 0.8f64;
#[no_mangle]
pub static mut prob_bk_weight_2: libc::c_double = 0.5f64;
#[no_mangle]
pub static mut fprob_LtoR: libc::c_double = 0.;
#[no_mangle]
pub static mut fprob_RtoL: libc::c_double = 0.;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_dpnd_rule(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut r_ptr: *mut DpndRule = 0 as *mut DpndRule;
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        r_ptr = DpndRuleArray.as_mut_ptr();
        while j < CurDpndRuleSize {
            if feature_pattern_match(&mut (*r_ptr).dependant, (*b_ptr).f,
                                     0 as *mut libc::c_void,
                                     b_ptr as *mut libc::c_void) ==
                   (0 as libc::c_int == 0) as libc::c_int {
                (*b_ptr).dpnd_rule = r_ptr;
                break ;
            } else { j += 1; r_ptr = r_ptr.offset(1) }
        }
        if (*b_ptr).dpnd_rule.is_null() {
            fprintf(stderr,
                    b";; No DpndRule for %dth bnst (\x00" as *const u8 as
                        *const libc::c_char, i);
            print_feature((*b_ptr).f, stderr);
            fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
            /* DpndRuleArray[0] はマッチしない時用 */
            (*b_ptr).dpnd_rule = DpndRuleArray.as_mut_ptr()
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_chi_dpnd_db() 
 /*==================================================================*/
 {
    if OptChiGenerative == 0 {
        db_close(chi_dpnd_db);
    } else {
        db_close(chi_dpnd_prob_db);
        db_close(chi_dis_comma_db);
        db_close(chi_case_db);
        db_close(chi_pos_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_chi_dpnd_db() 
 /*==================================================================*/
 {
    if OptChiGenerative == 0 {
        chi_dpnd_db =
            open_dict(25 as libc::c_int,
                      b"ebcf/chidpnd.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHIDpndExist)
    } else {
        chi_dpnd_prob_db =
            open_dict(33 as libc::c_int,
                      b"ebcf/chidpnd_prob.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHIDpndProbExist);
        chi_dis_comma_db =
            open_dict(34 as libc::c_int,
                      b"ebcf/chi_dis_comma.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHIDisCommaExist);
        chi_case_db =
            open_dict(35 as libc::c_int,
                      b"ebcf/chi_case.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHICaseExist);
        chi_pos_db =
            open_dict(36 as libc::c_int,
                      b"ebcf/chi_pos.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHIPosExist)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn free_chi_type() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int {
        if !(*Chi_word_type.as_mut_ptr().offset(i as isize)).is_null() {
            free(*Chi_word_type.as_mut_ptr().offset(i as isize) as
                     *mut libc::c_void);
            let ref mut fresh0 =
                *Chi_word_type.as_mut_ptr().offset(i as isize);
            *fresh0 = 0 as *mut libc::c_char
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_chi_type() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int {
        let ref mut fresh1 = *Chi_word_type.as_mut_ptr().offset(i as isize);
        *fresh1 =
            malloc((::std::mem::size_of::<libc::c_char>() as
                        libc::c_ulong).wrapping_mul((8 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        libc::c_ulong)) as
                *mut libc::c_char;
        i += 1
    }
    strcpy(*Chi_word_type.as_mut_ptr().offset(0 as libc::c_int as isize),
           b"adv\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(1 as libc::c_int as isize),
           b"verbMarker\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(2 as libc::c_int as isize),
           b"ba\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(3 as libc::c_int as isize),
           b"coor\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(4 as libc::c_int as isize),
           b"num\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(5 as libc::c_int as isize),
           b"subord\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(6 as libc::c_int as isize),
           b"relDe\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(7 as libc::c_int as isize),
           b"assoDe\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(8 as libc::c_int as isize),
           b"vDe\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(9 as libc::c_int as isize),
           b"vDe\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(10 as libc::c_int as isize),
           b"det\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(11 as libc::c_int as isize),
           b"nounMarker\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(12 as libc::c_int as isize),
           b"foreign\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(13 as libc::c_int as isize),
           b"other\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(14 as libc::c_int as isize),
           b"adj\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(15 as libc::c_int as isize),
           b"lBei\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(16 as libc::c_int as isize),
           b"post\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(17 as libc::c_int as isize),
           b"measure\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(18 as libc::c_int as isize),
           b"particle\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(19 as libc::c_int as isize),
           b"noun\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(20 as libc::c_int as isize),
           b"noun\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(21 as libc::c_int as isize),
           b"tempNoun\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(22 as libc::c_int as isize),
           b"num\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(23 as libc::c_int as isize),
           b"sound\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(24 as libc::c_int as isize),
           b"prep\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(25 as libc::c_int as isize),
           b"noun\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(26 as libc::c_int as isize),
           b"punc\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(27 as libc::c_int as isize),
           b"sBei\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(28 as libc::c_int as isize),
           b"sentMarker\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(29 as libc::c_int as isize),
           b"verb\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(30 as libc::c_int as isize),
           b"verb\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(31 as libc::c_int as isize),
           b"verb\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_type.as_mut_ptr().offset(32 as libc::c_int as isize),
           b"verb\x00" as *const u8 as *const libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn free_chi_pos() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int {
        if !(*Chi_word_pos.as_mut_ptr().offset(i as isize)).is_null() {
            free(*Chi_word_pos.as_mut_ptr().offset(i as isize) as
                     *mut libc::c_void);
            let ref mut fresh2 =
                *Chi_word_pos.as_mut_ptr().offset(i as isize);
            *fresh2 = 0 as *mut libc::c_char
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_chi_pos() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int {
        let ref mut fresh3 = *Chi_word_pos.as_mut_ptr().offset(i as isize);
        *fresh3 =
            malloc((::std::mem::size_of::<libc::c_char>() as
                        libc::c_ulong).wrapping_mul((3 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        libc::c_ulong)) as
                *mut libc::c_char;
        i += 1
    }
    strcpy(*Chi_word_pos.as_mut_ptr().offset(0 as libc::c_int as isize),
           b"AD\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(1 as libc::c_int as isize),
           b"AS\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(2 as libc::c_int as isize),
           b"BA\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(3 as libc::c_int as isize),
           b"CC\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(4 as libc::c_int as isize),
           b"CD\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(5 as libc::c_int as isize),
           b"CS\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(6 as libc::c_int as isize),
           b"DEC\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(7 as libc::c_int as isize),
           b"DEG\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(8 as libc::c_int as isize),
           b"DER\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(9 as libc::c_int as isize),
           b"DEV\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(10 as libc::c_int as isize),
           b"DT\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(11 as libc::c_int as isize),
           b"ETC\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(12 as libc::c_int as isize),
           b"FW\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(13 as libc::c_int as isize),
           b"IJ\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(14 as libc::c_int as isize),
           b"JJ\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(15 as libc::c_int as isize),
           b"LB\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(16 as libc::c_int as isize),
           b"LC\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(17 as libc::c_int as isize),
           b"M\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(18 as libc::c_int as isize),
           b"MSP\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(19 as libc::c_int as isize),
           b"NN\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(20 as libc::c_int as isize),
           b"NR\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(21 as libc::c_int as isize),
           b"NT\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(22 as libc::c_int as isize),
           b"OD\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(23 as libc::c_int as isize),
           b"ON\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(24 as libc::c_int as isize),
           b"P\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(25 as libc::c_int as isize),
           b"PN\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(26 as libc::c_int as isize),
           b"PU\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(27 as libc::c_int as isize),
           b"SB\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(28 as libc::c_int as isize),
           b"SP\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(29 as libc::c_int as isize),
           b"VA\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(30 as libc::c_int as isize),
           b"VC\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(31 as libc::c_int as isize),
           b"VE\x00" as *const u8 as *const libc::c_char);
    strcpy(*Chi_word_pos.as_mut_ptr().offset(32 as libc::c_int as isize),
           b"VV\x00" as *const u8 as *const libc::c_char);
}
/* get dpnd rule for Chinese */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_chi_dpnd_rule(mut word1: *mut libc::c_char,
                                           mut pos1: *mut libc::c_char,
                                           mut word2: *mut libc::c_char,
                                           mut pos2: *mut libc::c_char,
                                           mut distance: libc::c_int,
                                           mut comma: libc::c_int)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptChiGenerative == 0 && CHIDpndExist == 0 as libc::c_int ||
           OptChiGenerative != 0 && CHIDpndProbExist == 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    if OptChiGenerative != 0 {
        if strcmp(word2, b"ROOT\x00" as *const u8 as *const libc::c_char) == 0
               && OptChiGenerative != 0 {
            key =
                malloc_db_buf(strlen(word1).wrapping_add(strlen(pos1)).wrapping_add(7
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                  as libc::c_int);
            sprintf(key,
                    b"%s_%s_ROOT\x00" as *const u8 as *const libc::c_char,
                    pos1, word1);
        } else if distance == 0 as libc::c_int {
            key =
                malloc_db_buf(strlen(word1).wrapping_add(strlen(word2)).wrapping_add(strlen(pos1)).wrapping_add(strlen(pos2)).wrapping_add(4
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulong)
                                  as libc::c_int);
            sprintf(key,
                    b"%s_%s_%s_%s\x00" as *const u8 as *const libc::c_char,
                    pos1, word1, pos2, word2);
        } else {
            key =
                malloc_db_buf(strlen(word1).wrapping_add(strlen(word2)).wrapping_add(strlen(pos1)).wrapping_add(strlen(pos2)).wrapping_add(8
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulong)
                                  as libc::c_int);
            sprintf(key,
                    b"%s_%s_%s_%s_%d_%d\x00" as *const u8 as
                        *const libc::c_char, pos1, word1, pos2, word2,
                    distance, comma);
        }
    } else {
        key =
            malloc_db_buf(strlen(word1).wrapping_add(strlen(word2)).wrapping_add(strlen(pos1)).wrapping_add(strlen(pos2)).wrapping_add(6
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_ulong)
                              as libc::c_int);
        sprintf(key,
                b"%s_%s_%s_%s_%d\x00" as *const u8 as *const libc::c_char,
                pos1, word1, pos2, word2, distance);
    }
    return if OptChiGenerative == 0 {
        db_get(chi_dpnd_db, key)
    } else if distance == 0 as libc::c_int ||
        strcmp(word2,
               b"ROOT\x00" as *const u8 as *const libc::c_char) == 0
    {
        db_get(chi_dpnd_prob_db, key)
    } else { db_get(chi_dis_comma_db, key) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_dpnd_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* store different directions for each type */
    let mut j: libc::c_int = 0; /* store different directions for each type */
    let mut k: libc::c_int = 0; /* store different directions for each type */
    let mut l: libc::c_int = 0; /* store different directions for each type */
    // let mut s: libc::c_int = 0; /* store different probability for each type */
    let mut value: libc::c_int =
        0; /* store different probability for each type */
    let mut first_uke_flag: libc::c_int =
        0; /* store different probability for each type */
    let mut k_ptr: *mut BNST_DATA =
        0 as *mut BNST_DATA; /* store different probability for each type */
    let mut u_ptr: *mut BNST_DATA =
        0 as *mut BNST_DATA; /* store different dpnd type */
    let mut lex_rule: *mut libc::c_char =
        0 as *mut libc::c_char; /* store different dpnd type */
    let mut pos_rule_1: *mut libc::c_char =
        0 as *mut libc::c_char; /* store different dpnd type */
    let mut pos_rule_2: *mut libc::c_char =
        0 as *mut libc::c_char; /* store different dpnd type */
    let mut pos_rule: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut type_0: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut probL: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut probR: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut occur: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut dpnd: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut count: libc::c_int =
        0; /* store occur time of different dpnd type */
    let mut rule: *mut libc::c_char =
        0 as *mut libc::c_char; /* store occur time of different dpnd type */
    let mut curRule: [*mut libc::c_char; 10] =
        [0 as *mut libc::c_char; 10]; /* number of dpnd type */
    let mut appear_LtoR_2: libc::c_int = 0; /* number of dpnd type */
    let mut appear_RtoL_2: libc::c_int = 0; /* number of dpnd type */
    let mut appear_LtoR_3: libc::c_int = 0; /* number of dpnd type */
    let mut appear_RtoL_3: libc::c_int = 0; /* parameter of each dpnd type */
    let mut total_2: libc::c_int = 0; /* parameter of each dpnd type */
    let mut total_3: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    let mut direction_1: [libc::c_char; 10] = [0; 10];
    let mut direction_2: [libc::c_char; 10] = [0; 10];
    let mut direction_3: [libc::c_char; 10] = [0; 10];
    let mut direction_4: [libc::c_char; 10] = [0; 10];
    let mut prob_LtoR_1: [libc::c_double; 10] = [0.; 10];
    let mut prob_RtoL_1: [libc::c_double; 10] = [0.; 10];
    let mut prob_LtoR_2: [libc::c_double; 10] = [0.; 10];
    let mut prob_RtoL_2: [libc::c_double; 10] = [0.; 10];
    let mut prob_LtoR_3: [libc::c_double; 10] = [0.; 10];
    let mut prob_RtoL_3: [libc::c_double; 10] = [0.; 10];
    let mut prob_LtoR_4: [libc::c_double; 10] = [0.; 10];
    let mut prob_RtoL_4: [libc::c_double; 10] = [0.; 10];
    let mut type_1: [[libc::c_char; 10]; 10] = [[0; 10]; 10];
    let mut type_2: [[libc::c_char; 10]; 10] = [[0; 10]; 10];
    let mut type_3: [[libc::c_char; 10]; 10] = [[0; 10]; 10];
    let mut type_4: [[libc::c_char; 10]; 10] = [[0; 10]; 10];
    let mut occur_1: [libc::c_double; 10] = [0.; 10];
    let mut occur_2: [libc::c_double; 10] = [0.; 10];
    let mut occur_3: [libc::c_double; 10] = [0.; 10];
    let mut occur_4: [libc::c_double; 10] = [0.; 10];
    let mut occur_RtoL_1: [libc::c_double; 10] = [0.; 10];
    let mut occur_RtoL_2: [libc::c_double; 10] = [0.; 10];
    let mut occur_RtoL_3: [libc::c_double; 10] = [0.; 10];
    let mut occur_RtoL_4: [libc::c_double; 10] = [0.; 10];
    let mut count_1: libc::c_int = 0;
    let mut count_2: libc::c_int = 0;
    let mut count_3: libc::c_int = 0;
    let mut count_4: libc::c_int = 0;
    let mut lamda1: [libc::c_double; 10] = [0.; 10];
    let mut lamda2: [libc::c_double; 10] = [0.; 10];
    /* initialization */
    lex_rule = 0 as *mut libc::c_char;
    pos_rule_1 = 0 as *mut libc::c_char;
    pos_rule_2 = 0 as *mut libc::c_char;
    pos_rule = 0 as *mut libc::c_char;
    rule = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        prob_LtoR_1[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_LtoR_2[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_LtoR_3[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_LtoR_4[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_1[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_2[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_3[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_4[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_RtoL_1[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_RtoL_2[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_RtoL_3[i as usize] = 0 as libc::c_int as libc::c_double;
        prob_RtoL_4[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_RtoL_1[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_RtoL_2[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_RtoL_3[i as usize] = 0 as libc::c_int as libc::c_double;
        occur_RtoL_4[i as usize] = 0 as libc::c_int as libc::c_double;
        direction_1[i as usize] = 0 as libc::c_int as libc::c_char;
        direction_2[i as usize] = 0 as libc::c_int as libc::c_char;
        direction_3[i as usize] = 0 as libc::c_int as libc::c_char;
        direction_4[i as usize] = 0 as libc::c_int as libc::c_char;
        lamda1[i as usize] = 0 as libc::c_int as libc::c_double;
        lamda2[i as usize] = 0 as libc::c_int as libc::c_double;
        i += 1
    }
    count_1 = 0 as libc::c_int;
    count_2 = 0 as libc::c_int;
    count_3 = 0 as libc::c_int;
    count_4 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        k_ptr = (*sp).bnst_data.offset(i as isize);
        first_uke_flag = 1 as libc::c_int;
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            u_ptr = (*sp).bnst_data.offset(j as isize);
            lex_rule = 0 as *mut libc::c_char;
            pos_rule_1 = 0 as *mut libc::c_char;
            pos_rule_2 = 0 as *mut libc::c_char;
            pos_rule = 0 as *mut libc::c_char;
            (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int;
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].count
                = 0 as libc::c_int;
            if Language != 2 as libc::c_int {
                k = 0 as libc::c_int;
                while (*(*k_ptr).dpnd_rule).dpnd_type[k as usize] != 0 {
                    value =
                        feature_pattern_match(&mut *(*(*k_ptr).dpnd_rule).governor.as_mut_ptr().offset(k
                                                                                                           as
                                                                                                           isize),
                                              (*u_ptr).f,
                                              k_ptr as *mut libc::c_void,
                                              u_ptr as *mut libc::c_void);
                    if value == (0 as libc::c_int == 0) as libc::c_int {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            =
                            (*(*k_ptr).dpnd_rule).dpnd_type[k as usize] as
                                libc::c_int;
                        first_uke_flag = 0 as libc::c_int;
                        break ;
                    } else { k += 1 }
                }
            } else {
                if j == i + 1 as libc::c_int {
                    distance = 1 as libc::c_int
                } else { distance = 2 as libc::c_int }
                /* read dpnd rule from DB for Chinese */
                lex_rule =
                    get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(),
                                      (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                                      (*(*u_ptr).head_ptr).Goi.as_mut_ptr(),
                                      (*(*u_ptr).head_ptr).Pos.as_mut_ptr(),
                                      distance, 0 as libc::c_int);
                pos_rule_1 =
                    get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(),
                                      (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                                      b"X\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      (*(*u_ptr).head_ptr).Pos.as_mut_ptr(),
                                      distance, 0 as libc::c_int);
                pos_rule_2 =
                    get_chi_dpnd_rule(b"X\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                                      (*(*u_ptr).head_ptr).Goi.as_mut_ptr(),
                                      (*(*u_ptr).head_ptr).Pos.as_mut_ptr(),
                                      distance, 0 as libc::c_int);
                pos_rule =
                    get_chi_dpnd_rule(b"X\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                                      b"X\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      (*(*u_ptr).head_ptr).Pos.as_mut_ptr(),
                                      distance, 0 as libc::c_int);
                if !lex_rule.is_null() {
                    count = 0 as libc::c_int;
                    rule = 0 as *mut libc::c_char;
                    rule =
                        strtok(lex_rule,
                               b":\x00" as *const u8 as *const libc::c_char);
                    while !rule.is_null() {
                        curRule[count as usize] =
                            malloc(strlen(rule).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy(curRule[count as usize], rule);
                        count += 1;
                        rule = 0 as *mut libc::c_char;
                        rule =
                            strtok(0 as *mut libc::c_char,
                                   b":\x00" as *const u8 as
                                       *const libc::c_char)
                    }
                    count_1 = count;
                    k = 0 as libc::c_int;
                    while k < count {
                        type_0 = 0 as *mut libc::c_char;
                        probL = 0 as *mut libc::c_char;
                        probR = 0 as *mut libc::c_char;
                        occur = 0 as *mut libc::c_char;
                        type_0 =
                            strtok(curRule[k as usize],
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probR =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probL =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        occur =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        dpnd =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        if strcmp(type_0,
                                  b"R\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            direction_1[k as usize] =
                                'R' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"L\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_1[k as usize] =
                                'L' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"B\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_1[k as usize] =
                                'B' as i32 as libc::c_char
                        }
                        occur_1[k as usize] = atof(occur);
                        strcpy(type_1[k as usize].as_mut_ptr(), dpnd);
                        prob_LtoR_1[k as usize] = atof(probR);
                        prob_RtoL_1[k as usize] = atof(probL);
                        if !curRule[k as usize].is_null() {
                            free(curRule[k as usize] as *mut libc::c_void);
                            curRule[k as usize] = 0 as *mut libc::c_char
                        }
                        k += 1
                    }
                }
                if !pos_rule_1.is_null() {
                    count = 0 as libc::c_int;
                    rule = 0 as *mut libc::c_char;
                    rule =
                        strtok(pos_rule_1,
                               b":\x00" as *const u8 as *const libc::c_char);
                    while !rule.is_null() {
                        curRule[count as usize] =
                            malloc(strlen(rule).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy(curRule[count as usize], rule);
                        count += 1;
                        rule = 0 as *mut libc::c_char;
                        rule =
                            strtok(0 as *mut libc::c_char,
                                   b":\x00" as *const u8 as
                                       *const libc::c_char)
                    }
                    count_2 = count;
                    k = 0 as libc::c_int;
                    while k < count {
                        type_0 = 0 as *mut libc::c_char;
                        probL = 0 as *mut libc::c_char;
                        probR = 0 as *mut libc::c_char;
                        occur = 0 as *mut libc::c_char;
                        type_0 =
                            strtok(curRule[k as usize],
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probR =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probL =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        occur =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        dpnd =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        if strcmp(type_0,
                                  b"R\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            direction_2[k as usize] =
                                'R' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"L\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_2[k as usize] =
                                'L' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"B\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_2[k as usize] =
                                'B' as i32 as libc::c_char
                        }
                        occur_2[k as usize] = atof(occur);
                        strcpy(type_2[k as usize].as_mut_ptr(), dpnd);
                        prob_LtoR_2[k as usize] = atof(probR);
                        prob_RtoL_2[k as usize] = atof(probL);
                        if !curRule[k as usize].is_null() {
                            free(curRule[k as usize] as *mut libc::c_void);
                            curRule[k as usize] = 0 as *mut libc::c_char
                        }
                        k += 1
                    }
                }
                if !pos_rule_2.is_null() {
                    count = 0 as libc::c_int;
                    rule = 0 as *mut libc::c_char;
                    rule =
                        strtok(pos_rule_2,
                               b":\x00" as *const u8 as *const libc::c_char);
                    while !rule.is_null() {
                        curRule[count as usize] =
                            malloc(strlen(rule).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy(curRule[count as usize], rule);
                        count += 1;
                        rule = 0 as *mut libc::c_char;
                        rule =
                            strtok(0 as *mut libc::c_char,
                                   b":\x00" as *const u8 as
                                       *const libc::c_char)
                    }
                    count_3 = count;
                    k = 0 as libc::c_int;
                    while k < count {
                        type_0 = 0 as *mut libc::c_char;
                        probL = 0 as *mut libc::c_char;
                        probR = 0 as *mut libc::c_char;
                        occur = 0 as *mut libc::c_char;
                        type_0 =
                            strtok(curRule[k as usize],
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probR =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probL =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        occur =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        dpnd =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        if strcmp(type_0,
                                  b"R\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            direction_3[k as usize] =
                                'R' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"L\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_3[k as usize] =
                                'L' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"B\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_3[k as usize] =
                                'B' as i32 as libc::c_char
                        }
                        occur_3[k as usize] = atof(occur);
                        strcpy(type_3[k as usize].as_mut_ptr(), dpnd);
                        prob_LtoR_3[k as usize] = atof(probR);
                        prob_RtoL_3[k as usize] = atof(probL);
                        if !curRule[k as usize].is_null() {
                            free(curRule[k as usize] as *mut libc::c_void);
                            curRule[k as usize] = 0 as *mut libc::c_char
                        }
                        k += 1
                    }
                }
                if !pos_rule.is_null() {
                    count = 0 as libc::c_int;
                    rule = 0 as *mut libc::c_char;
                    rule =
                        strtok(pos_rule,
                               b":\x00" as *const u8 as *const libc::c_char);
                    while !rule.is_null() {
                        curRule[count as usize] =
                            malloc(strlen(rule).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy(curRule[count as usize], rule);
                        count += 1;
                        rule = 0 as *mut libc::c_char;
                        rule =
                            strtok(0 as *mut libc::c_char,
                                   b":\x00" as *const u8 as
                                       *const libc::c_char)
                    }
                    count_4 = count;
                    k = 0 as libc::c_int;
                    while k < count {
                        type_0 = 0 as *mut libc::c_char;
                        probL = 0 as *mut libc::c_char;
                        probR = 0 as *mut libc::c_char;
                        occur = 0 as *mut libc::c_char;
                        type_0 =
                            strtok(curRule[k as usize],
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probR =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        probL =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        occur =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        dpnd =
                            strtok(0 as *mut libc::c_char,
                                   b"_\x00" as *const u8 as
                                       *const libc::c_char);
                        if strcmp(type_0,
                                  b"R\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            direction_4[k as usize] =
                                'R' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"L\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_4[k as usize] =
                                'L' as i32 as libc::c_char
                        } else if strcmp(type_0,
                                         b"B\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            direction_4[k as usize] =
                                'B' as i32 as libc::c_char
                        }
                        occur_4[k as usize] = atof(occur);
                        strcpy(type_4[k as usize].as_mut_ptr(), dpnd);
                        prob_LtoR_4[k as usize] = atof(probR);
                        prob_RtoL_4[k as usize] = atof(probL);
                        if !curRule[k as usize].is_null() {
                            free(curRule[k as usize] as *mut libc::c_void);
                            curRule[k as usize] = 0 as *mut libc::c_char
                        }
                        k += 1
                    }
                }
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].prob_pos_LtoR
                    = 0 as libc::c_int as libc::c_double;
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].prob_pos_RtoL
                    = 0 as libc::c_int as libc::c_double;
                /* calculate pos probability */
                if !pos_rule.is_null() {
                    k = 0 as libc::c_int;
                    while k < count_4 {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_pos_LtoR
                            += prob_LtoR_4[k as usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_pos_RtoL
                            += prob_RtoL_4[k as usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].occur_pos
                            += occur_4[k as usize];
                        k += 1
                    }
                }
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].prob_pos_LtoR
                    =
                    1.0f64 *
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_pos_LtoR
                        /
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].occur_pos;
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].prob_pos_RtoL
                    =
                    1.0f64 *
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_pos_RtoL
                        /
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].occur_pos;
                /* calculate probability */
                if !lex_rule.is_null() {
                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize].count
                        = count_1;
                    k = 0 as libc::c_int;
                    while k <
                              (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                        isize))[j
                                                                                    as
                                                                                    usize].count
                          {
                        lamda1[k as usize] =
                            1.0f64 * occur_1[k as usize] /
                                (occur_1[k as usize] +
                                     1 as libc::c_int as libc::c_double);
                        appear_LtoR_2 = 0 as libc::c_int;
                        appear_RtoL_2 = 0 as libc::c_int;
                        appear_LtoR_3 = 0 as libc::c_int;
                        appear_RtoL_3 = 0 as libc::c_int;
                        total_2 = 0 as libc::c_int;
                        total_3 = 0 as libc::c_int;
                        l = 0 as libc::c_int;
                        while l < count_2 {
                            if strcmp(type_1[k as usize].as_mut_ptr(),
                                      type_2[l as usize].as_mut_ptr()) == 0 {
                                appear_LtoR_2 =
                                    prob_LtoR_2[l as usize] as libc::c_int;
                                appear_RtoL_2 =
                                    prob_RtoL_2[l as usize] as libc::c_int;
                                total_2 = occur_2[l as usize] as libc::c_int;
                                break ;
                            } else { l += 1 }
                        }
                        l = 0 as libc::c_int;
                        while l < count_3 {
                            if strcmp(type_1[k as usize].as_mut_ptr(),
                                      type_3[l as usize].as_mut_ptr()) == 0 {
                                appear_LtoR_3 =
                                    prob_LtoR_3[l as usize] as libc::c_int;
                                appear_RtoL_3 =
                                    prob_RtoL_3[l as usize] as libc::c_int;
                                total_3 = occur_3[l as usize] as libc::c_int;
                                break ;
                            } else { l += 1 }
                        }
                        if total_2 != 0 as libc::c_int ||
                               total_3 != 0 as libc::c_int {
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda1[k as usize] *
                                    (1.0f64 * prob_LtoR_1[k as usize] /
                                         occur_1[k as usize]) +
                                    1.0f64 *
                                        (1 as libc::c_int as libc::c_double -
                                             lamda1[k as usize]) *
                                        (1.0f64 *
                                             (appear_LtoR_2 + appear_LtoR_3)
                                                 as libc::c_double /
                                             (total_2 + total_3) as
                                                 libc::c_double);
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda1[k as usize] *
                                    (1.0f64 * prob_RtoL_1[k as usize] /
                                         occur_1[k as usize]) +
                                    1.0f64 *
                                        (1 as libc::c_int as libc::c_double -
                                             lamda1[k as usize]) *
                                        (1.0f64 *
                                             (appear_RtoL_2 + appear_RtoL_3)
                                                 as libc::c_double /
                                             (total_2 + total_3) as
                                                 libc::c_double)
                        } else {
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda1[k as usize] *
                                    (1.0f64 * prob_LtoR_1[k as usize] /
                                         occur_1[k as usize]);
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda1[k as usize] *
                                    (1.0f64 * prob_RtoL_1[k as usize] /
                                         occur_1[k as usize])
                        }
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].direction[k
                                                                                                   as
                                                                                                   usize]
                            = direction_1[k as usize];
                        strcpy((*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                         isize))[j
                                                                                     as
                                                                                     usize].type_0[k
                                                                                                       as
                                                                                                       usize].as_mut_ptr(),
                               type_1[k as usize].as_mut_ptr());
                        k += 1
                    }
                } else if !pos_rule_1.is_null() || !pos_rule_2.is_null() {
                    if !pos_rule_1.is_null() {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].count
                            = count_2;
                        k = 0 as libc::c_int;
                        while k <
                                  (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                            isize))[j
                                                                                        as
                                                                                        usize].count
                              {
                            appear_LtoR_2 =
                                prob_LtoR_2[k as usize] as libc::c_int;
                            appear_RtoL_2 =
                                prob_RtoL_2[k as usize] as libc::c_int;
                            appear_LtoR_3 = 0 as libc::c_int;
                            appear_RtoL_3 = 0 as libc::c_int;
                            total_2 = occur_2[k as usize] as libc::c_int;
                            total_3 = 0 as libc::c_int;
                            l = 0 as libc::c_int;
                            while l < count_3 {
                                if strcmp(type_2[k as usize].as_mut_ptr(),
                                          type_3[l as usize].as_mut_ptr()) ==
                                       0 {
                                    appear_LtoR_3 =
                                        prob_LtoR_3[l as usize] as
                                            libc::c_int;
                                    appear_RtoL_3 =
                                        prob_RtoL_3[l as usize] as
                                            libc::c_int;
                                    total_3 =
                                        occur_3[l as usize] as libc::c_int;
                                    break ;
                                } else { l += 1 }
                            }
                            lamda2[k as usize] =
                                1.0f64 * (total_2 + total_3) as libc::c_double
                                    /
                                    (total_2 + total_3 + 1 as libc::c_int) as
                                        libc::c_double;
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda2[k as usize] *
                                    (1.0f64 *
                                         (appear_LtoR_2 + appear_LtoR_3) as
                                             libc::c_double /
                                         (total_2 + total_3) as
                                             libc::c_double);
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda2[k as usize] *
                                    (1.0f64 *
                                         (appear_RtoL_2 + appear_RtoL_3) as
                                             libc::c_double /
                                         (total_2 + total_3) as
                                             libc::c_double);
                            strcpy((*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                             isize))[j
                                                                                         as
                                                                                         usize].type_0[k
                                                                                                           as
                                                                                                           usize].as_mut_ptr(),
                                   type_2[k as usize].as_mut_ptr());
                            l = 0 as libc::c_int;
                            while l < count_4 {
                                if strcmp(type_2[k as usize].as_mut_ptr(),
                                          type_4[l as usize].as_mut_ptr()) ==
                                       0 {
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                              isize))[j
                                                                                          as
                                                                                          usize].prob_LtoR[k
                                                                                                               as
                                                                                                               usize]
                                        +=
                                        (1 as libc::c_int as libc::c_double -
                                             lamda2[k as usize]) *
                                            (1.0f64 * prob_LtoR_4[l as usize]
                                                 / occur_4[l as usize]);
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                              isize))[j
                                                                                          as
                                                                                          usize].prob_RtoL[k
                                                                                                               as
                                                                                                               usize]
                                        +=
                                        (1 as libc::c_int as libc::c_double -
                                             lamda2[k as usize]) *
                                            (1.0f64 * prob_RtoL_4[l as usize]
                                                 / occur_4[l as usize]);
                                    break ;
                                } else { l += 1 }
                            }
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].direction[k
                                                                                                       as
                                                                                                       usize]
                                = direction_2[k as usize];
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                *= prob_bk_weight_1;
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                *= prob_bk_weight_1;
                            k += 1
                        }
                    } else if !pos_rule_2.is_null() {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].count
                            = count_3;
                        k = 0 as libc::c_int;
                        while k <
                                  (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                            isize))[j
                                                                                        as
                                                                                        usize].count
                              {
                            lamda2[k as usize] =
                                1.0f64 * occur_3[k as usize] /
                                    (occur_3[k as usize] +
                                         1 as libc::c_int as libc::c_double);
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda2[k as usize] *
                                    (1.0f64 * prob_LtoR_3[k as usize] /
                                         occur_3[k as usize]);
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                =
                                1.0f64 * lamda2[k as usize] *
                                    (1.0f64 * prob_RtoL_3[k as usize] /
                                         occur_3[k as usize]);
                            strcpy((*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                             isize))[j
                                                                                         as
                                                                                         usize].type_0[k
                                                                                                           as
                                                                                                           usize].as_mut_ptr(),
                                   type_3[k as usize].as_mut_ptr());
                            l = 0 as libc::c_int;
                            while l < count_4 {
                                if strcmp(type_3[k as usize].as_mut_ptr(),
                                          type_4[l as usize].as_mut_ptr()) ==
                                       0 {
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                              isize))[j
                                                                                          as
                                                                                          usize].prob_LtoR[k
                                                                                                               as
                                                                                                               usize]
                                        +=
                                        (1 as libc::c_int as libc::c_double -
                                             lamda2[k as usize]) *
                                            (1.0f64 * prob_LtoR_4[l as usize]
                                                 / occur_4[k as usize]);
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                              isize))[j
                                                                                          as
                                                                                          usize].prob_RtoL[k
                                                                                                               as
                                                                                                               usize]
                                        +=
                                        (1 as libc::c_int as libc::c_double -
                                             lamda2[k as usize]) *
                                            (1.0f64 * prob_RtoL_4[l as usize]
                                                 / occur_4[k as usize]);
                                    break ;
                                } else { l += 1 }
                            }
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].direction[k
                                                                                                       as
                                                                                                       usize]
                                = direction_3[k as usize];
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_LtoR[k
                                                                                                       as
                                                                                                       usize]
                                *= prob_bk_weight_1;
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[k
                                                                                                       as
                                                                                                       usize]
                                *= prob_bk_weight_1;
                            k += 1
                        }
                    }
                } else {
                    (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize].count
                        = count_4;
                    k = 0 as libc::c_int;
                    while k <
                              (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                        isize))[j
                                                                                    as
                                                                                    usize].count
                          {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_LtoR[k
                                                                                                   as
                                                                                                   usize]
                            =
                            1.0f64 * prob_LtoR_4[k as usize] /
                                occur_4[k as usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_RtoL[k
                                                                                                   as
                                                                                                   usize]
                            =
                            1.0f64 * prob_RtoL_4[k as usize] /
                                occur_4[k as usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].direction[k
                                                                                                   as
                                                                                                   usize]
                            = direction_4[k as usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_LtoR[k
                                                                                                   as
                                                                                                   usize]
                            *= prob_bk_weight_2;
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_RtoL[k
                                                                                                   as
                                                                                                   usize]
                            *= prob_bk_weight_2;
                        strcpy((*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                         isize))[j
                                                                                     as
                                                                                     usize].type_0[k
                                                                                                       as
                                                                                                       usize].as_mut_ptr(),
                               type_4[k as usize].as_mut_ptr());
                        k += 1
                    }
                }
                if !lex_rule.is_null() || !pos_rule_1.is_null() ||
                       !pos_rule_2.is_null() || !pos_rule.is_null() {
                    first_uke_flag = 0 as libc::c_int;
                    if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                 isize))[j as
                                                                             usize].count
                           > 0 as libc::c_int {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 'O' as i32
                    }
                }
            }
            j += 1
        }
        i += 1
    }
    if Language == 2 as libc::c_int {
        /* free memory */
        if !lex_rule.is_null() {
            free(lex_rule as *mut libc::c_void);
            lex_rule = 0 as *mut libc::c_char
        }
        if !pos_rule_1.is_null() {
            free(pos_rule_1 as *mut libc::c_void);
            pos_rule_1 = 0 as *mut libc::c_char
        }
        if !pos_rule_2.is_null() {
            free(pos_rule_2 as *mut libc::c_void);
            pos_rule_2 = 0 as *mut libc::c_char
        }
        if !pos_rule.is_null() {
            free(pos_rule as *mut libc::c_void);
            pos_rule = 0 as *mut libc::c_char
        }
        if !rule.is_null() {
            free(rule as *mut libc::c_void);
            rule = 0 as *mut libc::c_char
        }
    };
}
/* calculate dpnd and dis_comma probability */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_prob(mut sp: *mut SENTENCE_DATA,
                                  mut left: libc::c_int,
                                  mut right: libc::c_int,
                                  mut distance: libc::c_int,
                                  mut comma: libc::c_int) 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0; /* parameter of each dpnd type */
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lex_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos_rule_1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos_rule_2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut direction: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut k_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut u_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut prob_LtoR_1: [libc::c_double; 2] = [0.; 2];
    let mut prob_LtoR_2: [libc::c_double; 2] = [0.; 2];
    let mut prob_LtoR_3: [libc::c_double; 2] = [0.; 2];
    let mut prob_LtoR_4: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR_1: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR_2: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR_3: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR_4: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL_1: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL_2: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL_3: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL_4: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL_1: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL_2: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL_3: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL_4: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    /* initialization */
    k_ptr = (*sp).bnst_data.offset(left as isize);
    u_ptr = (*sp).bnst_data.offset(right as isize);
    lex_rule = 0 as *mut libc::c_char;
    pos_rule_1 = 0 as *mut libc::c_char;
    pos_rule_2 = 0 as *mut libc::c_char;
    pos_rule = 0 as *mut libc::c_char;
    prob_LtoR_1[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_1[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_2[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_2[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_3[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_3[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_4[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_LtoR_4[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_1[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_1[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_2[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_2[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_3[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_3[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_4[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR_4[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_1[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_1[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_2[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_2[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_3[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_3[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_4[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL_4[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_1[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_1[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_2[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_2[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_3[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_3[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_4[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL_4[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lamda = 0 as libc::c_int as libc::c_double;
    fprob_LtoR = 0.0f64;
    fprob_RtoL = 0.0f64;
    /* read rule from DB for Chinese */
  /* for each pair, [0] store TRAIN, [1] store GIGA */
    lex_rule =
        get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(),
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          (*(*u_ptr).head_ptr).Goi.as_mut_ptr(),
                          (*(*u_ptr).head_ptr).Pos.as_mut_ptr(), distance,
                          comma);
    pos_rule_1 =
        get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(),
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*(*u_ptr).head_ptr).Pos.as_mut_ptr(), distance,
                          comma);
    pos_rule_2 =
        get_chi_dpnd_rule(b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          (*(*u_ptr).head_ptr).Goi.as_mut_ptr(),
                          (*(*u_ptr).head_ptr).Pos.as_mut_ptr(), distance,
                          comma);
    pos_rule =
        get_chi_dpnd_rule(b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*(*u_ptr).head_ptr).Pos.as_mut_ptr(), distance,
                          comma);
    if !lex_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(lex_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR_1[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_1[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_1[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_1[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR_1[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_1[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_1[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_1[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    if !pos_rule_1.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule =
            strtok(pos_rule_1, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR_2[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_2[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_2[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_2[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR_2[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_2[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_2[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_2[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    if !pos_rule_2.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule =
            strtok(pos_rule_2, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR_3[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_3[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_3[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_3[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR_3[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_3[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_3[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_3[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    if !pos_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(pos_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR_4[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_4[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_4[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_4[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR_4[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL_4[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR_4[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL_4[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lamda = 0.0f64;
        prob[k as usize] = 0.0f64;
        /* prob_LtoR */
        if prob_LtoR_1[k as usize] > 0.0000000000000001f64 {
            lamda =
                prob_LtoR_1[k as usize] /
                    (prob_LtoR_1[k as usize] +
                         1 as libc::c_int as libc::c_double);
            if prob_LtoR_2[k as usize] > 0.0000000000000001f64 ||
                   prob_LtoR_3[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    lamda * prob_LtoR_1[k as usize] / occur_LtoR_1[k as usize]
                        +
                        (1 as libc::c_int as libc::c_double - lamda) *
                            (prob_LtoR_2[k as usize] +
                                 prob_LtoR_3[k as usize]) /
                            (occur_LtoR_2[k as usize] +
                                 occur_LtoR_3[k as usize])
            } else {
                prob[k as usize] =
                    lamda * prob_LtoR_1[k as usize] / occur_LtoR_1[k as usize]
            }
        } else if prob_LtoR_2[k as usize] > 0.0000000000000001f64 ||
                      prob_LtoR_3[k as usize] > 0.0000000000000001f64 {
            lamda =
                (prob_LtoR_2[k as usize] + prob_LtoR_3[k as usize]) /
                    (prob_LtoR_2[k as usize] + prob_LtoR_3[k as usize] +
                         1 as libc::c_int as libc::c_double);
            if prob_LtoR_4[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    prob_bk_weight_1 *
                        (lamda *
                             (prob_LtoR_2[k as usize] +
                                  prob_LtoR_3[k as usize]) /
                             (occur_LtoR_2[k as usize] +
                                  occur_LtoR_3[k as usize]) +
                             (1 as libc::c_int as libc::c_double - lamda) *
                                 (prob_LtoR_4[k as usize] /
                                      occur_LtoR_4[k as usize]))
            } else {
                prob[k as usize] =
                    prob_bk_weight_1 *
                        (lamda *
                             (prob_LtoR_2[k as usize] +
                                  prob_LtoR_3[k as usize]) /
                             (occur_LtoR_2[k as usize] +
                                  occur_LtoR_3[k as usize]))
            }
        } else if prob_LtoR_4[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                prob_bk_weight_2 * prob_LtoR_4[k as usize] /
                    occur_LtoR_4[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        fprob_LtoR =
            prob[0 as libc::c_int as usize] * giga_weight +
                prob[1 as libc::c_int as usize] *
                    (1 as libc::c_int as libc::c_double - giga_weight)
    } else { fprob_LtoR = prob[1 as libc::c_int as usize] }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lamda = 0.0f64;
        prob[k as usize] = 0.0f64;
        /* prob_RtoL */
        if prob_RtoL_1[k as usize] > 0.0000000000000001f64 {
            lamda =
                prob_RtoL_1[k as usize] /
                    (prob_RtoL_1[k as usize] +
                         1 as libc::c_int as libc::c_double);
            if prob_RtoL_2[k as usize] > 0.0000000000000001f64 ||
                   prob_RtoL_3[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    lamda * prob_RtoL_1[k as usize] / occur_RtoL_1[k as usize]
                        +
                        (1 as libc::c_int as libc::c_double - lamda) *
                            (prob_RtoL_2[k as usize] +
                                 prob_RtoL_3[k as usize]) /
                            (occur_RtoL_2[k as usize] +
                                 occur_RtoL_3[k as usize])
            } else {
                prob[k as usize] =
                    lamda * prob_RtoL_1[k as usize] / occur_RtoL_1[k as usize]
            }
        } else if prob_RtoL_2[k as usize] > 0.0000000000000001f64 ||
                      prob_RtoL_3[k as usize] > 0.0000000000000001f64 {
            lamda =
                (prob_RtoL_2[k as usize] + prob_RtoL_3[k as usize]) /
                    (prob_RtoL_2[k as usize] + prob_RtoL_3[k as usize] +
                         1 as libc::c_int as libc::c_double);
            if prob_RtoL_4[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    prob_bk_weight_1 *
                        (lamda *
                             (prob_RtoL_2[k as usize] +
                                  prob_RtoL_3[k as usize]) /
                             (occur_RtoL_2[k as usize] +
                                  occur_RtoL_3[k as usize]) +
                             (1 as libc::c_int as libc::c_double - lamda) *
                                 (prob_RtoL_4[k as usize] /
                                      occur_RtoL_4[k as usize]))
            } else {
                prob[k as usize] =
                    prob_bk_weight_1 *
                        (lamda *
                             (prob_RtoL_2[k as usize] +
                                  prob_RtoL_3[k as usize]) /
                             (occur_RtoL_2[k as usize] +
                                  occur_RtoL_3[k as usize]))
            }
        } else if prob_RtoL_4[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                prob_bk_weight_2 * prob_RtoL_4[k as usize] /
                    occur_RtoL_4[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        fprob_RtoL =
            prob[0 as libc::c_int as usize] * giga_weight +
                prob[1 as libc::c_int as usize] *
                    (1 as libc::c_int as libc::c_double - giga_weight)
    } else { fprob_RtoL = prob[1 as libc::c_int as usize] }
    /* free memory */
    if !pos_rule.is_null() {
        free(pos_rule as *mut libc::c_void);
        pos_rule = 0 as *mut libc::c_char
    }
    if !lex_rule.is_null() {
        free(lex_rule as *mut libc::c_void);
        lex_rule = 0 as *mut libc::c_char
    }
    if !pos_rule_1.is_null() {
        free(pos_rule_1 as *mut libc::c_void);
        pos_rule_1 = 0 as *mut libc::c_char
    }
    if !pos_rule_2.is_null() {
        free(pos_rule_2 as *mut libc::c_void);
        pos_rule_2 = 0 as *mut libc::c_char
    };
}
/* calculate dpnd and dis_comma probability for model with pos-tagging */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_prob_wpos(mut sp: *mut SENTENCE_DATA,
                                       mut left: libc::c_int,
                                       mut right: libc::c_int,
                                       mut distance: libc::c_int,
                                       mut comma: libc::c_int,
                                       mut pos_left: *mut libc::c_char,
                                       mut pos_right: *mut libc::c_char) 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0; /* parameter of each dpnd type */
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut direction: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut k_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut u_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut prob_LtoR: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    /* initialization */
    k_ptr = (*sp).bnst_data.offset(left as isize);
    u_ptr = (*sp).bnst_data.offset(right as isize);
    pos_rule = 0 as *mut libc::c_char;
    prob_LtoR[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    prob_LtoR[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    occur_LtoR[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    prob_RtoL[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    occur_RtoL[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lamda = 0 as libc::c_int as libc::c_double;
    fprob_LtoR = 0.0f64;
    fprob_RtoL = 0.0f64;
    /* read rule from DB for Chinese */
  /* for each pair, [0] store TRAIN, [1] store GIGA */
    pos_rule =
        get_chi_dpnd_rule(b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, pos_left,
                          b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, pos_right, distance, comma);
    if !pos_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(pos_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            lamda = 0.0f64;
            prob[k as usize] = 0.0f64;
            /* prob_LtoR */
            if prob_LtoR[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    prob_bk_weight_2 * prob_LtoR[k as usize] /
                        occur_LtoR[k as usize]
            }
            k += 1
        }
        if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
            fprob_LtoR =
                prob[0 as libc::c_int as usize] * giga_weight +
                    prob[1 as libc::c_int as usize] *
                        (1 as libc::c_int as libc::c_double - giga_weight)
        } else { fprob_LtoR = prob[1 as libc::c_int as usize] }
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            lamda = 0.0f64;
            prob[k as usize] = 0.0f64;
            /* prob_RtoL */
            if prob_RtoL[k as usize] > 0.0000000000000001f64 {
                prob[k as usize] =
                    prob_bk_weight_2 * prob_RtoL[k as usize] /
                        occur_RtoL[k as usize]
            }
            k += 1
        }
        if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
            fprob_RtoL =
                prob[0 as libc::c_int as usize] * giga_weight +
                    prob[1 as libc::c_int as usize] *
                        (1 as libc::c_int as libc::c_double - giga_weight)
        } else { fprob_RtoL = prob[1 as libc::c_int as usize] }
    }
    /* free memory */
    if !pos_rule.is_null() {
        free(pos_rule as *mut libc::c_void);
        pos_rule = 0 as *mut libc::c_char
    };
}
/* calculate dpnd and dis_comma probability for model with pos-tagging */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_lex_prob_wpos(mut sp: *mut SENTENCE_DATA,
                                           mut left: libc::c_int,
                                           mut right: libc::c_int,
                                           mut distance: libc::c_int,
                                           mut comma: libc::c_int,
                                           mut pos_left: *mut libc::c_char,
                                           mut pos_right: *mut libc::c_char) 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0; /* parameter of each dpnd type */
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lex_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut direction: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut k_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut u_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut prob_LtoR: [libc::c_double; 2] = [0.; 2];
    let mut occur_LtoR: [libc::c_double; 2] = [0.; 2];
    let mut prob_RtoL: [libc::c_double; 2] = [0.; 2];
    let mut occur_RtoL: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    /* initialization */
    k_ptr = (*sp).bnst_data.offset(left as isize);
    u_ptr = (*sp).bnst_data.offset(right as isize);
    lex_rule = 0 as *mut libc::c_char;
    prob_LtoR[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    prob_LtoR[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    occur_LtoR[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_LtoR[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob_RtoL[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    prob_RtoL[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    occur_RtoL[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    occur_RtoL[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lamda = 0 as libc::c_int as libc::c_double;
    fprob_LtoR = 0.0f64;
    fprob_RtoL = 0.0f64;
    /* read rule from DB for Chinese */
  /* for each pair, [0] store TRAIN, [1] store GIGA */
    lex_rule =
        get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(), pos_left,
                          (*(*k_ptr).head_ptr).Goi.as_mut_ptr(), pos_right,
                          distance, comma);
    if !lex_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(lex_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            direction = 0 as *mut libc::c_char;
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            probRtoL = 0 as *mut libc::c_char;
            occurRtoL = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            direction =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            probLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            probRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurRtoL =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                occur_LtoR[0 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL[0 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR[0 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL[0 as libc::c_int as usize] = atof(probRtoL)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                occur_LtoR[1 as libc::c_int as usize] = atof(occurLtoR);
                occur_RtoL[1 as libc::c_int as usize] = atof(occurRtoL);
                prob_LtoR[1 as libc::c_int as usize] = atof(probLtoR);
                prob_RtoL[1 as libc::c_int as usize] = atof(probRtoL)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lamda = 0.0f64;
        prob[k as usize] = 0.0f64;
        /* prob_LtoR */
        if prob_LtoR[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                prob_bk_weight_2 * prob_LtoR[k as usize] /
                    occur_LtoR[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        fprob_LtoR =
            prob[0 as libc::c_int as usize] * giga_weight +
                prob[1 as libc::c_int as usize] *
                    (1 as libc::c_int as libc::c_double - giga_weight)
    } else { fprob_LtoR = prob[1 as libc::c_int as usize] }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lamda = 0.0f64;
        prob[k as usize] = 0.0f64;
        /* prob_RtoL */
        if prob_RtoL[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                prob_bk_weight_2 * prob_RtoL[k as usize] /
                    occur_RtoL[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        fprob_RtoL =
            prob[0 as libc::c_int as usize] * giga_weight +
                prob[1 as libc::c_int as usize] *
                    (1 as libc::c_int as libc::c_double - giga_weight)
    } else { fprob_RtoL = prob[1 as libc::c_int as usize] }
    /* free memory */
    if !lex_rule.is_null() {
        free(lex_rule as *mut libc::c_void);
        lex_rule = 0 as *mut libc::c_char
    };
}
/* calculate root probability */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_root_prob(mut sp: *mut SENTENCE_DATA,
                                       mut root: libc::c_int)
 -> libc::c_double 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lex_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut probRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut occurRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut direction: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut root_prob: libc::c_double = 0.;
    let mut lex_root_prob: [libc::c_double; 2] = [0.; 2];
    let mut pos_root_prob: [libc::c_double; 2] = [0.; 2];
    let mut lex_root_occur: [libc::c_double; 2] = [0.; 2];
    let mut pos_root_occur: [libc::c_double; 2] = [0.; 2];
    let mut lamda_root: libc::c_double = 0.;
    let mut k_ptr: *mut BNST_DATA = (*sp).bnst_data.offset(root as isize);
    /* get rule for root */
  /* initialization */
    lex_rule = 0 as *mut libc::c_char;
    pos_rule = 0 as *mut libc::c_char;
    lex_root_prob[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lex_root_prob[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lex_root_occur[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    lex_root_occur[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_prob[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_prob[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_occur[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_occur[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob[0 as libc::c_int as usize] = 0.0f64;
    prob[1 as libc::c_int as usize] = 0.0f64;
    /* get root rule */
    lex_rule =
        get_chi_dpnd_rule((*(*k_ptr).head_ptr).Goi.as_mut_ptr(),
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          b"ROOT\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
    pos_rule =
        get_chi_dpnd_rule(b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*(*k_ptr).head_ptr).Pos.as_mut_ptr(),
                          b"ROOT\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
    if !lex_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(lex_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            probLtoR =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                lex_root_prob[0 as libc::c_int as usize] = atof(probLtoR);
                lex_root_occur[0 as libc::c_int as usize] = atof(occurLtoR)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                lex_root_prob[1 as libc::c_int as usize] = atof(probLtoR);
                lex_root_occur[1 as libc::c_int as usize] = atof(occurLtoR)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    if !pos_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(pos_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            probLtoR =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                pos_root_prob[0 as libc::c_int as usize] = atof(probLtoR);
                pos_root_occur[0 as libc::c_int as usize] = atof(occurLtoR)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                pos_root_prob[1 as libc::c_int as usize] = atof(probLtoR);
                pos_root_occur[1 as libc::c_int as usize] = atof(occurLtoR)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    /* calculate root prob */
    lamda_root = 0.5f64;
    k = 0 as libc::c_int;
    while k < 1 as libc::c_int {
        if lex_root_prob[k as usize] > 0.0000000000000001f64 &&
               pos_root_prob[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                lamda_root * lex_root_prob[k as usize] /
                    lex_root_occur[k as usize] +
                    (1 as libc::c_int as libc::c_double - lamda_root) *
                        (pos_root_prob[k as usize] /
                             pos_root_occur[k as usize])
        } else if pos_root_prob[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                (1 as libc::c_int as libc::c_double - lamda_root) *
                    (pos_root_prob[k as usize] / pos_root_occur[k as usize])
        } else { prob[k as usize] = 0 as libc::c_int as libc::c_double }
        k += 1
    }
    root_prob = prob[0 as libc::c_int as usize];
    /* free memory */
    if !lex_rule.is_null() {
        free(lex_rule as *mut libc::c_void);
        lex_rule = 0 as *mut libc::c_char
    }
    if !pos_rule.is_null() {
        free(pos_rule as *mut libc::c_void);
        pos_rule = 0 as *mut libc::c_char
    }
    return root_prob;
}
/* calculate root probability for model with pos-tagging */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_root_prob_wpos(mut sp: *mut SENTENCE_DATA,
                                            mut root: libc::c_int,
                                            mut root_pos: *mut libc::c_char)
 -> libc::c_double 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut probRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occurLtoR: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut occurRtoL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut direction: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut root_prob: libc::c_double = 0.;
    let mut pos_root_prob: [libc::c_double; 2] = [0.; 2];
    let mut pos_root_occur: [libc::c_double; 2] = [0.; 2];
    /* get rule for root */
  /* initialization */
    pos_rule = 0 as *mut libc::c_char;
    pos_root_prob[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_prob[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_occur[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    pos_root_occur[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_double;
    prob[0 as libc::c_int as usize] = 0.0f64;
    prob[1 as libc::c_int as usize] = 0.0f64;
    /* get root rule */
    pos_rule =
        get_chi_dpnd_rule(b"XX\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, root_pos,
                          b"ROOT\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
    if !pos_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(pos_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong)) as
                    *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            probLtoR = 0 as *mut libc::c_char;
            occurLtoR = 0 as *mut libc::c_char;
            dpnd = 0 as *mut libc::c_char;
            probLtoR =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            occurLtoR =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            dpnd =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(dpnd, b"TRAIN\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                pos_root_prob[0 as libc::c_int as usize] = atof(probLtoR);
                pos_root_occur[0 as libc::c_int as usize] = atof(occurLtoR)
            } else if strcmp(dpnd,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                pos_root_prob[1 as libc::c_int as usize] = atof(probLtoR);
                pos_root_occur[1 as libc::c_int as usize] = atof(occurLtoR)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    /* calculate root prob */
    k = 0 as libc::c_int;
    while k < 1 as libc::c_int {
        if pos_root_prob[k as usize] > 0.0000000000000001f64 {
            prob[k as usize] =
                pos_root_prob[k as usize] / pos_root_occur[k as usize]
        } else { prob[k as usize] = 0 as libc::c_int as libc::c_double }
        k += 1
    }
    root_prob = prob[0 as libc::c_int as usize];
    /* free memory */
    if !pos_rule.is_null() {
        free(pos_rule as *mut libc::c_void);
        pos_rule = 0 as *mut libc::c_char
    }
    return root_prob;
}
/* calculate dpnd probability & root probability for Chinese probabilistic model */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_chi_dpnd_matrix_forProbModel(mut sp:
                                                               *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut comma: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    let mut total_word_prob: libc::c_double = 0.;
    let mut total_LtoR: libc::c_double = 0.;
    let mut total_RtoL: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        /* get root prob */
        (*Chi_root_prob_matrix.as_mut_ptr().offset(i as
                                                       isize)).prob[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
            = get_root_prob(sp, i);
        /* get dpnd rule for word pair */
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            /* get comma and distance */
            comma = 0 as libc::c_int;
            k = i + 1 as libc::c_int;
            while k < j {
                if !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                  b"PU\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                       (strcmp((*(*(*sp).bnst_data.offset(k as
                                                              isize)).head_ptr).Goi.as_mut_ptr(),
                               b",\x00" as *const u8 as *const libc::c_char)
                            == 0 ||
                            strcmp((*(*(*sp).bnst_data.offset(k as
                                                                  isize)).head_ptr).Goi.as_mut_ptr(),
                                   b"\xef\xbc\x9a\x00" as *const u8 as
                                       *const libc::c_char) == 0 ||
                            strcmp((*(*(*sp).bnst_data.offset(k as
                                                                  isize)).head_ptr).Goi.as_mut_ptr(),
                                   b":\x00" as *const u8 as
                                       *const libc::c_char) == 0 ||
                            strcmp((*(*(*sp).bnst_data.offset(k as
                                                                  isize)).head_ptr).Goi.as_mut_ptr(),
                                   b"\xef\xbc\x9b\x00" as *const u8 as
                                       *const libc::c_char) == 0 ||
                            strcmp((*(*(*sp).bnst_data.offset(k as
                                                                  isize)).head_ptr).Goi.as_mut_ptr(),
                                   b"\xef\xbc\x8c\x00" as *const u8 as
                                       *const libc::c_char) == 0) {
                    comma = 1 as libc::c_int;
                    break ;
                } else { k += 1 }
            }
            if j == i + 1 as libc::c_int {
                distance = 1 as libc::c_int
            } else { distance = 2 as libc::c_int }
            /* get dis_comma prob */
            fprob_LtoR = 0.0f64;
            fprob_RtoL = 0.0f64;
            get_prob(sp, i, j, distance, comma);
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].prob_dis_comma_LtoR[0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                = fprob_LtoR;
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].prob_dis_comma_RtoL[0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                = fprob_RtoL;
            total_LtoR = 0.0f64;
            total_RtoL = 0.0f64;
            /* get dpnd prob */
            fprob_LtoR = 0.0f64;
            fprob_RtoL = 0.0f64;
            get_prob(sp, i, j, 0 as libc::c_int, 0 as libc::c_int);
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].prob_LtoR[0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                = fprob_LtoR;
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].prob_RtoL[0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                = fprob_RtoL;
            /* direction */
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].direction[0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                = 0 as libc::c_int as libc::c_char;
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].count
                = 0 as libc::c_int;
            if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                         isize))[j as
                                                                     usize].prob_LtoR[0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize]
                   > 0.0000000000000001f64 &&
                   (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                             isize))[j as
                                                                         usize].prob_RtoL[0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize]
                       > 0.0000000000000001f64 &&
                   (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                             isize))[j as
                                                                         usize].prob_dis_comma_LtoR[0
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                       > 0.0000000000000001f64 &&
                   (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                             isize))[j as
                                                                         usize].prob_dis_comma_RtoL[0
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                       > 0.0000000000000001f64 {
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].direction[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                    = 'B' as i32 as libc::c_char;
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].count
                    = 1 as libc::c_int
            } else if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                isize))[j as
                                                                            usize].prob_LtoR[0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                          > 0.0000000000000001f64 &&
                          (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                    isize))[j
                                                                                as
                                                                                usize].prob_dis_comma_LtoR[0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               usize]
                              > 0.0000000000000001f64 {
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].direction[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                    = 'R' as i32 as libc::c_char;
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].count
                    = 1 as libc::c_int
            } else if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                isize))[j as
                                                                            usize].prob_RtoL[0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                          > 0.0000000000000001f64 &&
                          (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                    isize))[j
                                                                                as
                                                                                usize].prob_dis_comma_RtoL[0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               usize]
                              > 0.0000000000000001f64 {
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].direction[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                    = 'L' as i32 as libc::c_char;
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[j as
                                                                      usize].count
                    = 1 as libc::c_int
            }
            if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                         isize))[j as
                                                                     usize].count
                   > 0 as libc::c_int {
                (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    'O' as i32
            }
            j += 1
        }
        i += 1
    }
    /* normalize prob_dpnd */
  /* p(wi|wr) = F(wi|wr)/(+F(wj|wr)) */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        total_word_prob = 0.0f64;
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            if !(j == i) {
                if j < i {
                    if (*Chi_dpnd_matrix.as_mut_ptr().offset(j as
                                                                 isize))[i as
                                                                             usize].prob_LtoR[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize]
                           > 0.0000000000000001f64 {
                        total_word_prob +=
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(j as
                                                                      isize))[i
                                                                                  as
                                                                                  usize].prob_LtoR[0
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       usize]
                    }
                } else if j > i {
                    if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                 isize))[j as
                                                                             usize].prob_RtoL[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize]
                           > 0.0000000000000001f64 {
                        total_word_prob +=
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].prob_RtoL[0
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       usize]
                    }
                }
            }
            j += 1
        }
        if !(total_word_prob < 0.0000000000000001f64) {
            j = 0 as libc::c_int;
            while j < (*sp).Bnst_num {
                if !(j == i) {
                    if j < i {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(j as
                                                                  isize))[i as
                                                                              usize].prob_LtoR[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                            /= total_word_prob
                    } else if j > i {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_RtoL[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                            /= total_word_prob
                    }
                }
                j += 1
            }
        }
        i += 1
    };
}
/* calculate dpnd probability & root probability & pos probability for Chinese probabilistic model with pos-tagging */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_chi_dpnd_matrix_wpos(mut sp:
                                                       *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut comma: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    // let mut total_word_prob: libc::c_double = 0.;
    let mut total_LtoR: libc::c_double = 0.;
    let mut total_RtoL: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as isize))[i as usize].count =
            0 as libc::c_int;
        m = 0 as libc::c_int;
        while m < 2 as libc::c_int &&
                  m <
                      (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                               isize)).pos_max
              {
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[i as
                                                                  usize].left_pos_index[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                  as
                                                                                                                                  isize))[i
                                                                                                                                              as
                                                                                                                                              usize].count
                                                                                            as
                                                                                            usize]
                =
                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                         isize)).pos_index[m
                                                                               as
                                                                               usize];
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[i as
                                                                  usize].right_pos_index[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                   as
                                                                                                                                   isize))[i
                                                                                                                                               as
                                                                                                                                               usize].count
                                                                                             as
                                                                                             usize]
                =
                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                         isize)).pos_index[m
                                                                               as
                                                                               usize];
            let ref mut fresh4 =
                (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                          isize))[i as
                                                                      usize].count;
            *fresh4 += 1;
            m += 1
        }
        /* get dpnd rule for word pair */
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[j as
                                                                  usize].count
                = 0 as libc::c_int;
            /* get comma and distance */
            comma = 0 as libc::c_int;
            k = i + 1 as libc::c_int;
            while k < j {
                if strcmp((*(*(*sp).bnst_data.offset(k as
                                                         isize)).head_ptr).Goi.as_mut_ptr(),
                          b",\x00" as *const u8 as *const libc::c_char) == 0
                       ||
                       strcmp((*(*(*sp).bnst_data.offset(k as
                                                             isize)).head_ptr).Goi.as_mut_ptr(),
                              b"\xef\xbc\x9a\x00" as *const u8 as
                                  *const libc::c_char) == 0 ||
                       strcmp((*(*(*sp).bnst_data.offset(k as
                                                             isize)).head_ptr).Goi.as_mut_ptr(),
                              b":\x00" as *const u8 as *const libc::c_char) ==
                           0 ||
                       strcmp((*(*(*sp).bnst_data.offset(k as
                                                             isize)).head_ptr).Goi.as_mut_ptr(),
                              b"\xef\xbc\x9b\x00" as *const u8 as
                                  *const libc::c_char) == 0 ||
                       strcmp((*(*(*sp).bnst_data.offset(k as
                                                             isize)).head_ptr).Goi.as_mut_ptr(),
                              b"\xef\xbc\x8c\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                    comma = 1 as libc::c_int;
                    break ;
                } else { k += 1 }
            }
            if j == i + 1 as libc::c_int {
                distance = 1 as libc::c_int
            } else { distance = 2 as libc::c_int }
            m = 0 as libc::c_int;
            while m < 2 as libc::c_int &&
                      m <
                          (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                   isize)).pos_max
                  {
                /* get root prob */
                (*Chi_root_prob_matrix.as_mut_ptr().offset(i as
                                                               isize)).prob[m
                                                                                as
                                                                                usize]
                    =
                    get_root_prob_wpos(sp, i,
                                       (*Chi_pos_matrix.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).pos[m
                                                                                                as
                                                                                                usize]);
                (*Chi_root_prob_matrix.as_mut_ptr().offset(i as
                                                               isize)).pos_index[m
                                                                                     as
                                                                                     usize]
                    = m;
                n = 0 as libc::c_int;
                while n < 2 as libc::c_int &&
                          n <
                              (*Chi_pos_matrix.as_mut_ptr().offset(j as
                                                                       isize)).pos_max
                      {
                    /* get dpnd prob */
                    fprob_LtoR = 0.0f64;
                    fprob_RtoL = 0.0f64;
                    get_prob_wpos(sp, i, j, 0 as libc::c_int,
                                  0 as libc::c_int,
                                  (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                           isize)).pos[m
                                                                                           as
                                                                                           usize],
                                  (*Chi_pos_matrix.as_mut_ptr().offset(j as
                                                                           isize)).pos[n
                                                                                           as
                                                                                           usize]);
                    //	  get_lex_prob_wpos(sp, i, j, distance, comma, Chi_pos_matrix[i].pos[m], Chi_pos_matrix[j].pos[n]);
                    if fprob_LtoR > 0.0000000000000001f64 ||
                           fprob_RtoL > 0.0000000000000001f64 {
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize))[j
                                                                                                                                                     as
                                                                                                                                                     usize].count
                                                                                                   as
                                                                                                   usize]
                            = fprob_LtoR;
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize))[j
                                                                                                                                                     as
                                                                                                                                                     usize].count
                                                                                                   as
                                                                                                   usize]
                            = fprob_RtoL;
                        total_LtoR = 0.0f64;
                        total_RtoL = 0.0f64;
                        /* get dis_comma prob */
                        fprob_LtoR = 0.0f64;
                        fprob_RtoL = 0.0f64;
                        get_prob_wpos(sp, i, j, distance, comma,
                                      (*Chi_pos_matrix.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)).pos[m
                                                                                               as
                                                                                               usize],
                                      (*Chi_pos_matrix.as_mut_ptr().offset(j
                                                                               as
                                                                               isize)).pos[n
                                                                                               as
                                                                                               usize]);
                        //	    get_lex_prob_wpos(sp, i, j, 0, 0, Chi_pos_matrix[i].pos[m], Chi_pos_matrix[j].pos[n]);
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_dis_comma_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                   as
                                                                                                                                                   isize))[j
                                                                                                                                                               as
                                                                                                                                                               usize].count
                                                                                                             as
                                                                                                             usize]
                            = fprob_LtoR;
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].prob_dis_comma_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                   as
                                                                                                                                                   isize))[j
                                                                                                                                                               as
                                                                                                                                                               usize].count
                                                                                                             as
                                                                                                             usize]
                            = fprob_RtoL;
                        total_LtoR = 0.0f64;
                        total_RtoL = 0.0f64;
                        /* 	    /\* get lex_dis_comma prob *\/ */
/* 	    fprob_LtoR = 0.0; */
/* 	    fprob_RtoL = 0.0; */
/* 	    get_lex_prob_wpos(sp, i, j, distance, comma, Chi_pos_matrix[i].pos[m], Chi_pos_matrix[j].pos[n]); */
/* 	    Chi_dpnd_matrix[i][j].lex_prob_dis_comma_LtoR[Chi_dpnd_matrix[i][j].count] = fprob_LtoR; */
/* 	    Chi_dpnd_matrix[i][j].lex_prob_dis_comma_RtoL[Chi_dpnd_matrix[i][j].count] = fprob_RtoL; */
                        /* 	    total_LtoR = 0.0; */
/* 	    total_RtoL = 0.0; */
                        /* 	    /\* get dpnd prob *\/ */
/* 	    fprob_LtoR = 0.0; */
/* 	    fprob_RtoL = 0.0; */
/* 	    get_lex_prob_wpos(sp, i, j, 0, 0, Chi_pos_matrix[i].pos[m], Chi_pos_matrix[j].pos[n]); */
/* 	    Chi_dpnd_matrix[i][j].lex_prob_LtoR[Chi_dpnd_matrix[i][j].count] = fprob_LtoR; */
/* 	    Chi_dpnd_matrix[i][j].lex_prob_RtoL[Chi_dpnd_matrix[i][j].count] = fprob_RtoL; */
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].left_pos_index[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                              as
                                                                                                                                              isize))[j
                                                                                                                                                          as
                                                                                                                                                          usize].count
                                                                                                        as
                                                                                                        usize]
                            =
                            (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                     isize)).pos_index[m
                                                                                           as
                                                                                           usize];
                        (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize].right_pos_index[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                               as
                                                                                                                                               isize))[j
                                                                                                                                                           as
                                                                                                                                                           usize].count
                                                                                                         as
                                                                                                         usize]
                            =
                            (*Chi_pos_matrix.as_mut_ptr().offset(j as
                                                                     isize)).pos_index[n
                                                                                           as
                                                                                           usize];
                        /* direction */
                        if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                     isize))[j
                                                                                 as
                                                                                 usize].prob_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                            as
                                                                                                                                            isize))[j
                                                                                                                                                        as
                                                                                                                                                        usize].count
                                                                                                      as
                                                                                                      usize]
                               > 0.0000000000000001f64 &&
                               (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                         isize))[j
                                                                                     as
                                                                                     usize].prob_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                as
                                                                                                                                                isize))[j
                                                                                                                                                            as
                                                                                                                                                            usize].count
                                                                                                          as
                                                                                                          usize]
                                   > 0.0000000000000001f64 &&
                               (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                         isize))[j
                                                                                     as
                                                                                     usize].prob_dis_comma_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                          as
                                                                                                                                                          isize))[j
                                                                                                                                                                      as
                                                                                                                                                                      usize].count
                                                                                                                    as
                                                                                                                    usize]
                                   > 0.0000000000000001f64 &&
                               (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                         isize))[j
                                                                                     as
                                                                                     usize].prob_dis_comma_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                          as
                                                                                                                                                          isize))[j
                                                                                                                                                                      as
                                                                                                                                                                      usize].count
                                                                                                                    as
                                                                                                                    usize]
                                   > 0.0000000000000001f64 {
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].direction[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                             as
                                                                                                                                             isize))[j
                                                                                                                                                         as
                                                                                                                                                         usize].count
                                                                                                       as
                                                                                                       usize]
                                = 'B' as i32 as libc::c_char
                        } else if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                            isize))[j
                                                                                        as
                                                                                        usize].prob_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                   as
                                                                                                                                                   isize))[j
                                                                                                                                                               as
                                                                                                                                                               usize].count
                                                                                                             as
                                                                                                             usize]
                                      > 0.0000000000000001f64 &&
                                      (*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                as
                                                                                isize))[j
                                                                                            as
                                                                                            usize].prob_dis_comma_LtoR[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                                 as
                                                                                                                                                                 isize))[j
                                                                                                                                                                             as
                                                                                                                                                                             usize].count
                                                                                                                           as
                                                                                                                           usize]
                                          > 0.0000000000000001f64 {
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].direction[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                             as
                                                                                                                                             isize))[j
                                                                                                                                                         as
                                                                                                                                                         usize].count
                                                                                                       as
                                                                                                       usize]
                                = 'R' as i32 as libc::c_char
                        } else if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                            isize))[j
                                                                                        as
                                                                                        usize].prob_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                   as
                                                                                                                                                   isize))[j
                                                                                                                                                               as
                                                                                                                                                               usize].count
                                                                                                             as
                                                                                                             usize]
                                      > 0.0000000000000001f64 &&
                                      (*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                as
                                                                                isize))[j
                                                                                            as
                                                                                            usize].prob_dis_comma_RtoL[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                                                 as
                                                                                                                                                                 isize))[j
                                                                                                                                                                             as
                                                                                                                                                                             usize].count
                                                                                                                           as
                                                                                                                           usize]
                                          > 0.0000000000000001f64 {
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].direction[(*Chi_dpnd_matrix.as_mut_ptr().offset(i
                                                                                                                                             as
                                                                                                                                             isize))[j
                                                                                                                                                         as
                                                                                                                                                         usize].count
                                                                                                       as
                                                                                                       usize]
                                = 'L' as i32 as libc::c_char
                        }
                        let ref mut fresh5 =
                            (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize].count;
                        *fresh5 += 1;
                        if (*Chi_dpnd_matrix.as_mut_ptr().offset(i as
                                                                     isize))[j
                                                                                 as
                                                                                 usize].count
                               > 0 as libc::c_int {
                            (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[j as
                                                                              usize]
                                = 'O' as i32
                        }
                    }
                    n += 1
                }
                m += 1
            }
            j += 1
        }
        i += 1
    };
}
/* get pos rule for Chinese */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_chi_pos_rule(mut word: *mut libc::c_char,
                                          mut pos: *mut libc::c_char)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptChiPos != 0 && CHIPosExist == 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    key =
        malloc_db_buf(strlen(word).wrapping_add(strlen(pos)).wrapping_add(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                          as libc::c_int);
    sprintf(key, b"%s_%s\x00" as *const u8 as *const libc::c_char, pos, word);
    return db_get(chi_pos_db, key);
}
/* calculate pos probability for Chinese probabilistic model */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_chi_pos_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut pos_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dpnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut count: libc::c_int = 0;
    let mut pos_prob: [libc::c_double; 2] = [0.; 2];
    let mut fprob: libc::c_double = 0.;
    let mut tmp_prob: [libc::c_double; 2] = [0.; 2];
    let mut tmp_occur: [libc::c_double; 2] = [0.; 2];
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        /* initialization */
        j = 0 as libc::c_int;
        while j < 33 as libc::c_int {
            (*Chi_pos_matrix.as_mut_ptr().offset(i as isize)).prob[j as usize]
                = 0.0f64;
            (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                     isize)).prob_pos_index[j
                                                                                as
                                                                                usize]
                = 0.0f64;
            (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                     isize)).pos_index[j as
                                                                           usize]
                = -(1 as libc::c_int);
            let ref mut fresh6 =
                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                         isize)).pos[j as
                                                                         usize];
            *fresh6 =
                malloc((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul((3 as libc::c_int +
                                                             1 as libc::c_int)
                                                            as libc::c_ulong))
                    as *mut libc::c_char;
            j += 1
        }
        (*Chi_pos_matrix.as_mut_ptr().offset(i as isize)).pos_max =
            0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 33 as libc::c_int {
            /* initialization */
            pos_rule = 0 as *mut libc::c_char;
            tmp_prob[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_double;
            tmp_prob[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_double;
            tmp_occur[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_double;
            tmp_occur[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_double;
            /* read rule from DB for Chinese */
      /* for each pair, [0] store TRAIN, [1] store GIGA */
            pos_rule =
                get_chi_pos_rule((*(*(*sp).bnst_data.offset(i as
                                                                isize)).head_ptr).Goi.as_mut_ptr(),
                                 *Chi_word_pos.as_mut_ptr().offset(j as
                                                                       isize));
            if !pos_rule.is_null() {
                count = 0 as libc::c_int;
                rule = 0 as *mut libc::c_char;
                rule =
                    strtok(pos_rule,
                           b":\x00" as *const u8 as *const libc::c_char);
                while !rule.is_null() {
                    curRule[count as usize] =
                        malloc(strlen(rule).wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong))
                            as *mut libc::c_char;
                    strcpy(curRule[count as usize], rule);
                    count += 1;
                    rule = 0 as *mut libc::c_char;
                    rule =
                        strtok(0 as *mut libc::c_char,
                               b":\x00" as *const u8 as *const libc::c_char)
                }
                k = 0 as libc::c_int;
                while k < count {
                    prob = 0 as *mut libc::c_char;
                    occur = 0 as *mut libc::c_char;
                    dpnd = 0 as *mut libc::c_char;
                    prob =
                        strtok(curRule[k as usize],
                               b"_\x00" as *const u8 as *const libc::c_char);
                    occur =
                        strtok(0 as *mut libc::c_char,
                               b"_\x00" as *const u8 as *const libc::c_char);
                    dpnd =
                        strtok(0 as *mut libc::c_char,
                               b"_\x00" as *const u8 as *const libc::c_char);
                    if strcmp(dpnd,
                              b"TRAIN\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                        tmp_occur[0 as libc::c_int as usize] = atof(occur);
                        tmp_prob[0 as libc::c_int as usize] = atof(prob)
                    } else if strcmp(dpnd,
                                     b"GIGA\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        tmp_occur[0 as libc::c_int as usize] = atof(occur);
                        tmp_prob[0 as libc::c_int as usize] = atof(prob)
                    }
                    if !curRule[k as usize].is_null() {
                        free(curRule[k as usize] as *mut libc::c_void);
                        curRule[k as usize] = 0 as *mut libc::c_char
                    }
                    k += 1
                }
                k = 0 as libc::c_int;
                while k < 2 as libc::c_int {
                    pos_prob[k as usize] = 0.0f64;
                    if tmp_prob[k as usize] > 0.0000000000000001f64 {
                        pos_prob[k as usize] =
                            tmp_prob[k as usize] / tmp_occur[k as usize]
                    }
                    k += 1
                }
                if pos_prob[0 as libc::c_int as usize] > 0.0000000000000001f64
                   {
                    fprob =
                        pos_prob[0 as libc::c_int as usize] * giga_weight +
                            pos_prob[1 as libc::c_int as usize] *
                                (1 as libc::c_int as libc::c_double -
                                     giga_weight)
                } else { fprob = pos_prob[1 as libc::c_int as usize] }
                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                         isize)).prob_pos_index[j
                                                                                    as
                                                                                    usize]
                    = fprob;
                if (*Chi_pos_matrix.as_mut_ptr().offset(i as isize)).pos_max
                       == 0 as libc::c_int {
                    (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                             isize)).prob[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                        = fprob;
                    (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                             isize)).pos_index[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                        = j;
                    strcpy((*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                    isize)).pos[0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize],
                           *Chi_word_pos.as_mut_ptr().offset(j as isize));
                    let ref mut fresh7 =
                        (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                 isize)).pos_max;
                    *fresh7 += 1
                } else {
                    k = 0 as libc::c_int;
                    while k <
                              (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                       isize)).pos_max
                          {
                        if fprob >
                               (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                        isize)).prob[k
                                                                                         as
                                                                                         usize]
                           {
                            l =
                                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                         isize)).pos_max;
                            while l > k {
                                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                         isize)).prob[l
                                                                                          as
                                                                                          usize]
                                    =
                                    (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                             isize)).prob[(l
                                                                                               -
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              usize];
                                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                         isize)).pos_index[l
                                                                                               as
                                                                                               usize]
                                    =
                                    (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                             isize)).pos_index[(l
                                                                                                    -
                                                                                                    1
                                                                                                        as
                                                                                                        libc::c_int)
                                                                                                   as
                                                                                                   usize];
                                strcpy((*Chi_pos_matrix.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).pos[l
                                                                                                as
                                                                                                usize],
                                       (*Chi_pos_matrix.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).pos[(l
                                                                                                 -
                                                                                                 1
                                                                                                     as
                                                                                                     libc::c_int)
                                                                                                as
                                                                                                usize]);
                                l -= 1
                            }
                            (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                     isize)).prob[k
                                                                                      as
                                                                                      usize]
                                = fprob;
                            (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                     isize)).pos_index[k
                                                                                           as
                                                                                           usize]
                                = j;
                            strcpy((*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                            isize)).pos[k
                                                                                            as
                                                                                            usize],
                                   *Chi_word_pos.as_mut_ptr().offset(j as
                                                                         isize));
                            let ref mut fresh8 =
                                (*Chi_pos_matrix.as_mut_ptr().offset(i as
                                                                         isize)).pos_max;
                            *fresh8 += 1;
                            break ;
                        } else { k += 1 }
                    }
                }
            }
            /* free memory */
            if !pos_rule.is_null() {
                free(pos_rule as *mut libc::c_void);
                pos_rule = 0 as *mut libc::c_char
            }
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn relax_dpnd_matrix(mut sp: *mut SENTENCE_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 係り先がない場合の緩和

  括弧によるマスクは優先し，その制限内で末尾に係れるように変更

  ○ Ａ‥‥「‥‥‥‥‥」‥‥Ｂ (文末)
  ○ ‥‥‥「Ａ‥‥‥Ｂ」‥‥‥ (括弧終)
  ○ ‥‥‥「Ａ‥Ｂ．‥」‥‥‥ (係:文末)
  × Ａ‥‥‥‥Ｂ「‥‥‥‥Ｃ」 (Ｂに係り得るとはしない．
  Ｃとの関係は解析で対処)
  */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ok_flag: libc::c_int = 0;
    let mut relax_flag: libc::c_int = 0;
    let mut last_possibility: libc::c_int = 0;
    relax_flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num - 1 as libc::c_int {
        ok_flag = 0 as libc::c_int;
        last_possibility = i;
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            if (*Quote_matrix.as_mut_ptr().offset(i as isize))[j as usize] !=
                   0 {
                if (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                       > 0 as libc::c_int {
                    ok_flag = (0 as libc::c_int == 0) as libc::c_int;
                    break ;
                } else if !check_feature((*(*sp).bnst_data.offset(j as
                                                                      isize)).f,
                                         b"\xe4\xbf\x82:\xe6\x96\x87\xe6\x9c\xab\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() {
                    last_possibility = j;
                    break ;
                } else { last_possibility = j }
            }
            j += 1
        }
        if ok_flag == 0 as libc::c_int {
            if !check_feature((*(*sp).bnst_data.offset(last_possibility as
                                                           isize)).f,
                              b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
                   !check_feature((*(*sp).bnst_data.offset(last_possibility as
                                                               isize)).f,
                                  b"\xe4\xbf\x82:\xe6\x96\x87\xe6\x9c\xab\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                   !check_feature((*(*sp).bnst_data.offset(last_possibility as
                                                               isize)).f,
                                  b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                      isize))[last_possibility
                                                                  as usize] =
                    'R' as i32;
                relax_flag = (0 as libc::c_int == 0) as libc::c_int
            }
        }
        i += 1
    }
    return relax_flag;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_uncertain_d_condition(mut sp:
                                                         *mut SENTENCE_DATA,
                                                     mut dp: *mut DPND,
                                                     mut gvnr: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 後方チ(ェック)の d の係り受けを許す条件

  ・ 次の可能な係り先(D)が３つ以上後ろ ( d - - D など )
  ・ 係り元とdの後ろが同じ格	例) 日本で最初に京都で行われた
  ・ d(係り先)とdの後ろが同じ格	例) 東京で計画中に京都に変更された

  ※ 「dに読点がある」ことでdを係り先とするのは不適切
  例) 「うすい板を木目が直角になるように、何枚もはり合わせたもの。」
  */
    let mut i: libc::c_int = 0;
    let mut next_D: libc::c_int = 0;
    let mut dpnd_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gvnr_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    next_D = 0 as libc::c_int;
    i = gvnr + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if (*Mask_matrix.as_mut_ptr().offset((*dp).pos as isize))[i as usize]
               != 0 &&
               (*Quote_matrix.as_mut_ptr().offset((*dp).pos as
                                                      isize))[i as usize] != 0
               && (*dp).mask[i as usize] != 0 &&
               (*Dpnd_matrix.as_mut_ptr().offset((*dp).pos as
                                                     isize))[i as usize] ==
                   'D' as i32 {
            next_D = i;
            break ;
        } else { i += 1 }
    }
    dpnd_cp =
        check_feature((*(*sp).bnst_data.offset((*dp).pos as isize)).f,
                      b"\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char);
    gvnr_cp =
        check_feature((*(*sp).bnst_data.offset(gvnr as isize)).f,
                      b"\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char);
    if gvnr < (*sp).Bnst_num - 1 as libc::c_int {
        next_cp =
            check_feature((*(*sp).bnst_data.offset((gvnr + 1 as libc::c_int)
                                                       as isize)).f,
                          b"\xe4\xbf\x82\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char)
    } else { next_cp = 0 as *mut libc::c_char }
    return if next_D == 0 as libc::c_int || (gvnr + 2 as libc::c_int) < next_D ||
        gvnr + 2 as libc::c_int == next_D &&
            gvnr < (*sp).Bnst_num - 1 as libc::c_int &&
            !check_feature((*(*sp).bnst_data.offset((gvnr +
                1 as libc::c_int)
                as isize)).f,
                           b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            (!dpnd_cp.is_null() && !next_cp.is_null() &&
                strcmp(dpnd_cp, next_cp) == 0 ||
                !gvnr_cp.is_null() && !next_cp.is_null() &&
                    strcmp(gvnr_cp, next_cp) == 0) {
        /* fprintf(stderr, "%d -> %d OK\n", i, j); */
        1 as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn compare_dpnd(mut sp: *mut SENTENCE_DATA,
                                      mut new_mgr: *mut TOTAL_MGR,
                                      mut best_mgr: *mut TOTAL_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0;
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_info_to_bnst(mut sp: *mut SENTENCE_DATA,
                                           mut dp: *mut DPND) 
 /*==================================================================*/
 {
    /* 係り受けに関する種々の情報を DPND から BNST_DATA にコピー */
    let mut i: libc::c_int = 0;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if Language != 2 as libc::c_int &&
               ((*dp).type_0[i as usize] as libc::c_int == 'd' as i32 ||
                    (*dp).type_0[i as usize] as libc::c_int == 'R' as i32) {
            (*b_ptr).dpnd_head = (*dp).head[i as usize];
            (*b_ptr).dpnd_type = 'D' as i32 as libc::c_char
            /* relaxした場合もDに */
        } else {
            (*b_ptr).dpnd_head = (*dp).head[i as usize];
            (*b_ptr).dpnd_type = (*dp).type_0[i as usize]
        }
        (*b_ptr).dpnd_dflt = (*dp).dflt[i as usize];
        i += 1;
        b_ptr = b_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_info_to_tag_raw(mut sp: *mut SENTENCE_DATA,
                                              mut dp: *mut DPND) 
 /*==================================================================*/
 {
    /* 係り受けに関する種々の情報を DPND から TAG_DATA にコピー */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut last_b: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut check_ac: libc::c_int = 0;
    let mut rep_length: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut ht_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        /* もっとも近い文節行を記憶 */
        if (*t_ptr).bnum >= 0 as libc::c_int { last_b = (*t_ptr).bnum }
        /* 文末 */
        if i == (*sp).Tag_num - 1 as libc::c_int {
            (*t_ptr).dpnd_head = -(1 as libc::c_int);
            (*t_ptr).dpnd_type = 'D' as i32 as libc::c_char
        } else if (*t_ptr).inum != 0 as libc::c_int {
            (*t_ptr).dpnd_head = (*t_ptr).num + 1 as libc::c_int;
            (*t_ptr).dpnd_type = 'D' as i32 as libc::c_char
        } else {
            /* 隣にかける */
            /* 文節内最後のタグ単位 (inum == 0) */
            if check_feature((*(*sp).bnst_data.offset(last_b as isize)).f,
                             b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\xe7\x84\xa1\xe8\xa6\x96\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                   {
                       cp =
                           check_feature((*(*sp).bnst_data.offset((*dp).head[last_b
                                                                                 as
                                                                                 usize]
                                                                      as
                                                                      isize)).f,
                                         b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                       !cp.is_null()
                   } {
                offset =
                    atoi(cp.offset(strlen(b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97:\x00"
                                              as *const u8 as
                                              *const libc::c_char) as isize));
                if offset > 0 as libc::c_int ||
                       (*(*sp).bnst_data.offset((*dp).head[last_b as usize] as
                                                    isize)).tag_num <=
                           -(1 as libc::c_int) * offset {
                    offset = 0 as libc::c_int
                }
            } else { offset = 0 as libc::c_int }
            /* ＡのＢＣなどがあった場合は、Ｃの格フレームにＡが存在せず、
	 かつ、Ｂの格フレームにＡが存在した場合は、ＡがＢにかかると考える */
            if check_feature((*(*sp).bnst_data.offset(last_b as isize)).f,
                             b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\xe7\x84\xa1\xe8\xa6\x96\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                   !check_feature((*t_ptr).f,
                                  b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                   (*dp).head[last_b as usize] - last_b == 1 as libc::c_int {
                if OptCaseFlag & 32 as libc::c_int != 0 {
                    strp = get_mrph_rep((*t_ptr).head_ptr);
                    rep_length = get_mrph_rep_length(strp)
                } else {
                    strp = (*(*t_ptr).head_ptr).Goi2.as_mut_ptr();
                    rep_length = strlen(strp) as libc::c_int
                }
                /* 「ＡのＣ」をチェック */
                ht_ptr =
                    (*(*sp).bnst_data.offset((*dp).head[last_b as usize] as
                                                 isize)).tag_ptr.offset((*(*sp).bnst_data.offset((*dp).head[last_b
                                                                                                                as
                                                                                                                usize]
                                                                                                     as
                                                                                                     isize)).tag_num
                                                                            as
                                                                            isize).offset(-(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)).offset(offset
                                                                                                                   as
                                                                                                                   isize);
                if !(*ht_ptr).cf_ptr.is_null() {
                    check_ac =
                        check_examples(strp, rep_length,
                                       (*(*ht_ptr).cf_ptr).ex_list[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize],
                                       (*(*ht_ptr).cf_ptr).ex_num[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize])
                } else { check_ac = -(1 as libc::c_int) }
                if OptDisplay == 3 as libc::c_int {
                    fprintf(Outfp,
                            b"\xe2\x98\x86\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97\xe5\x88\xa4\xe5\xae\x9aAC: %6s \xe3\x81\xae %6s: %d\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*(*t_ptr).head_ptr).Goi2.as_mut_ptr(),
                            (*(*ht_ptr).head_ptr).Goi2.as_mut_ptr(),
                            check_ac);
                }
                /* Ｂが複数タグから成る場合のためのループ */
                j = 0 as libc::c_int;
                ht_ptr =
                    (*(*sp).bnst_data.offset((*dp).head[last_b as usize] as
                                                 isize)).tag_ptr;
                while j <
                          (*(*sp).bnst_data.offset((*dp).head[last_b as usize]
                                                       as isize)).tag_num -
                              1 as libc::c_int + offset {
                    if OptCaseFlag & 32 as libc::c_int != 0 {
                        strp = get_mrph_rep((*t_ptr).head_ptr);
                        rep_length = get_mrph_rep_length(strp)
                    } else {
                        strp = (*(*t_ptr).head_ptr).Goi2.as_mut_ptr();
                        rep_length = strlen(strp) as libc::c_int
                    }
                    /* 「ＡのＢ」をチェック */
                    if OptDisplay == 3 as libc::c_int &&
                           !(*ht_ptr).cf_ptr.is_null() {
                        fprintf(Outfp,
                                b"\xe2\x98\x86\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97\xe5\x88\xa4\xe5\xae\x9aAB: %6s \xe3\x81\xae %6s: %d\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*(*t_ptr).head_ptr).Goi2.as_mut_ptr(),
                                (*(*ht_ptr).head_ptr).Goi2.as_mut_ptr(),
                                check_examples(strp, rep_length,
                                               (*(*ht_ptr).cf_ptr).ex_list[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize],
                                               (*(*ht_ptr).cf_ptr).ex_num[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]));
                    }
                    if check_ac == -(1 as libc::c_int) &&
                           !(*ht_ptr).cf_ptr.is_null() &&
                           check_examples(strp, rep_length,
                                          (*(*ht_ptr).cf_ptr).ex_list[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize],
                                          (*(*ht_ptr).cf_ptr).ex_num[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize])
                               != -(1 as libc::c_int) {
                        offset =
                            j -
                                ((*(*sp).bnst_data.offset((*dp).head[last_b as
                                                                         usize]
                                                              as
                                                              isize)).tag_num
                                     - 1 as libc::c_int);
                        sprintf(buf.as_mut_ptr(),
                                b"\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97:%d\x00"
                                    as *const u8 as *const libc::c_char,
                                offset);
                        assign_cfeature(&mut (*(*sp).bnst_data.offset(*(*dp).head.as_mut_ptr().offset(last_b
                                                                                                          as
                                                                                                          isize)
                                                                          as
                                                                          isize)).f,
                                        buf.as_mut_ptr(), 0 as libc::c_int);
                        assign_cfeature(&mut (*ht_ptr).f, buf.as_mut_ptr(),
                                        0 as libc::c_int);
                        break ;
                    } else { j += 1; ht_ptr = ht_ptr.offset(1) }
                }
            }
            if (*dp).head[last_b as usize] == -(1 as libc::c_int) {
                (*t_ptr).dpnd_head = -(1 as libc::c_int)
            } else {
                (*t_ptr).dpnd_head =
                    (*(*(*sp).bnst_data.offset((*dp).head[last_b as usize] as
                                                   isize)).tag_ptr.offset((*(*sp).bnst_data.offset((*dp).head[last_b
                                                                                                                  as
                                                                                                                  usize]
                                                                                                       as
                                                                                                       isize)).tag_num
                                                                              as
                                                                              isize).offset(-(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize)).offset(offset
                                                                                                                     as
                                                                                                                     isize)).num
            }
            if Language != 2 as libc::c_int &&
                   ((*dp).type_0[last_b as usize] as libc::c_int == 'd' as i32
                        ||
                        (*dp).type_0[last_b as usize] as libc::c_int ==
                            'R' as i32) {
                (*t_ptr).dpnd_type = 'D' as i32 as libc::c_char
            } else { (*t_ptr).dpnd_type = (*dp).type_0[last_b as usize] }
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_info_to_tag(mut sp: *mut SENTENCE_DATA,
                                          mut dp: *mut DPND) 
 /*==================================================================*/
 {
    if OptInput == 0 as libc::c_int || OptInput & 2 as libc::c_int != 0 ||
           OptInput & 4 as libc::c_int != 0 {
        dpnd_info_to_tag_raw(sp, dp);
    } else {
        /* 解析済み (OPT_INPUT_PARSED) */
        dpnd_info_to_tag_pm(sp);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_semantic_head(mut m_ptr: *mut MRPH_DATA,
                                             mut t_ptr: *mut TAG_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp =
        check_feature((*t_ptr).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        /* 用言の場合のみ対象 */
        if strcmp(cp,
                  b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as *const u8 as
                      *const libc::c_char) == 0 &&
               !check_feature((*t_ptr).f,
                              b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            /* 判定詞がある場合はその判定詞 */
            if (*m_ptr).num > (*(*t_ptr).head_ptr).num &&
                   strcmp(Class[(*m_ptr).Hinshi as
                                    usize][0 as libc::c_int as usize].id as
                              *const libc::c_char,
                          b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as
                              *const u8 as *const libc::c_char) == 0 {
                return (0 as libc::c_int == 0) as libc::c_int
            }
        } else if (*m_ptr).num == (*(*t_ptr).head_ptr).num {
            return (0 as libc::c_int == 0) as libc::c_int
        }
    }
    return if (*m_ptr).inum == 0 as libc::c_int {
        /* 判定詞以外は(準)?内容語 */
        /* バックアップ: 基本句末尾の形態素 (上記の判定詞ルールにマッチしない場合用) */
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_syntactic_head(mut m_ptr: *mut MRPH_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 基本句末尾の形態素が句読点の場合は一つ前、それ以外は基本句末尾の形態素 */
    return if (*m_ptr).inum == 1 as libc::c_int &&
        strcmp(Class[(*m_ptr.offset(1 as libc::c_int as isize)).Hinshi as
            usize][0 as libc::c_int as usize].id as
                   *const libc::c_char,
               b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                   *const libc::c_char) == 0 &&
        (strcmp(Class[(*m_ptr.offset(1 as libc::c_int as isize)).Hinshi as
            usize][(*m_ptr.offset(1 as libc::c_int as
            isize)).Bunrui as
            usize].id as *const libc::c_char,
                b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                    *const libc::c_char) == 0 ||
            strcmp(Class[(*m_ptr.offset(1 as libc::c_int as isize)).Hinshi
                as
                usize][(*m_ptr.offset(1 as libc::c_int as
                isize)).Bunrui as
                usize].id as *const libc::c_char,
                   b"\xe5\x8f\xa5\xe7\x82\xb9\x00" as *const u8 as
                       *const libc::c_char) == 0) {
        (0 as libc::c_int == 0) as libc::c_int
    } else if (*m_ptr).inum == 0 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_info_to_mrph(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* root形態素 */
    let mut j: libc::c_int = 0;
    let mut last_t: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut proj_table: [libc::c_int; 200] = [0; 200];
    let mut appear_semantic_head_flag: libc::c_int = 0 as libc::c_int;
    let mut appear_outer_word_flag: libc::c_int = 0 as libc::c_int;
    let mut last_content_m: libc::c_int = 0;
    let mut this_is_semantic_head_flag: libc::c_int = 0;
    let mut appear_syntactic_head_flag: libc::c_int = 0 as libc::c_int;
    let mut this_is_syntactic_head_flag: libc::c_int = 0;
    let mut root_m: libc::c_int = -(1 as libc::c_int);
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut head_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut last_content_m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    /* initialize proj_table */
    memset(proj_table.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul(200 as libc::c_int as
                                                libc::c_ulong));
    i = 0 as libc::c_int;
    m_ptr = (*sp).mrph_data;
    while i < (*sp).Mrph_num {
        /* もっとも近い基本句行を記憶 */
        if (*m_ptr).tnum >= 0 as libc::c_int {
            last_t = (*m_ptr).tnum;
            appear_semantic_head_flag = 0 as libc::c_int;
            appear_syntactic_head_flag = 0 as libc::c_int;
            appear_outer_word_flag = 0 as libc::c_int;
            last_content_m_ptr = 0 as *mut MRPH_DATA
        }
        /* この形態素がsemantic headかどうかをチェック */
        if OptSemanticHead != 0 &&
               appear_semantic_head_flag == 0 as libc::c_int &&
               check_semantic_head(m_ptr,
                                   (*sp).tag_data.offset(last_t as isize)) !=
                   0 {
            appear_semantic_head_flag =
                (0 as libc::c_int == 0) as libc::c_int;
            last_content_m = (*m_ptr).num;
            last_content_m_ptr = m_ptr;
            this_is_semantic_head_flag =
                (0 as libc::c_int == 0) as libc::c_int
        } else { this_is_semantic_head_flag = 0 as libc::c_int }
        /* この形態素がsyntactic headかどうかをチェック */
        if appear_syntactic_head_flag == 0 as libc::c_int &&
               check_syntactic_head(m_ptr) != 0 {
            appear_syntactic_head_flag =
                (0 as libc::c_int == 0) as libc::c_int;
            this_is_syntactic_head_flag =
                (0 as libc::c_int == 0) as libc::c_int;
            if OptSemanticHead == 0 {
                /* 句点の係り先用に、headを記憶 */
                last_content_m = (*m_ptr).num
            }
        } else { this_is_syntactic_head_flag = 0 as libc::c_int }
        /* この形態素が句間要素なら、これ以降は常に親にする (for OptSemanticHead) */
        if appear_outer_word_flag == 0 as libc::c_int &&
               (*(*sp).tag_data.offset(last_t as isize)).dpnd_head !=
                   -(1 as libc::c_int) &&
               !check_feature((*m_ptr).f,
                              b"\xef\xbc\xb4\xe5\x8f\xa5\xe9\x96\x93\xe8\xa6\x81\xe7\xb4\xa0\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            appear_outer_word_flag = (0 as libc::c_int == 0) as libc::c_int
        }
        /* 隣にかける */
        if appear_syntactic_head_flag == 0 && this_is_semantic_head_flag == 0
               &&
               (OptSemanticHead != 0 &&
                    (appear_semantic_head_flag == 0 as libc::c_int ||
                         appear_outer_word_flag ==
                             (0 as libc::c_int == 0) as libc::c_int) ||
                    OptSemanticHead == 0 &&
                        check_feature((*m_ptr).f,
                                      b"\xef\xbc\xb4\xe5\x8f\xa5\xe5\x86\x85\xe8\xa6\x81\xe7\xb4\xa0\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null()) {
            /* syntactic headかつ句内要素ではない */
            (*m_ptr).out_head_flag = 0 as libc::c_int;
            (*m_ptr).dpnd_head = (*m_ptr).num + 1 as libc::c_int;
            (*m_ptr).dpnd_type = 'D' as i32 as libc::c_char;
            if !last_content_m_ptr.is_null() {
                /* semantic headからこの形態素にかける */
                (*last_content_m_ptr).dpnd_head = (*m_ptr).num;
                (*last_content_m_ptr).dpnd_type = 'D' as i32 as libc::c_char;
                last_content_m_ptr = 0 as *mut MRPH_DATA
            }
        } else if appear_outer_word_flag == 0 as libc::c_int &&
                      this_is_semantic_head_flag == 0 &&
                      (OptSemanticHead != 0 &&
                           appear_semantic_head_flag ==
                               (0 as libc::c_int == 0) as libc::c_int &&
                           !(appear_syntactic_head_flag != 0 &&
                                 this_is_syntactic_head_flag == 0) ||
                           !check_feature((*m_ptr).f,
                                          b"\xef\xbc\xb4\xe5\x8f\xa5\xe5\x86\x85\xe8\xa6\x81\xe7\xb4\xa0\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() ||
                           (*(*sp).tag_data.offset(last_t as isize)).dpnd_head
                               == -(1 as libc::c_int) &&
                               appear_syntactic_head_flag != 0 &&
                               this_is_syntactic_head_flag == 0) {
            /* 基本句内の内容語(前側にある)にかける */
            /* semantic head時もしくは句内要素もしくは文末の句点 */
            (*m_ptr).out_head_flag = 0 as libc::c_int;
            (*m_ptr).dpnd_head = last_content_m;
            (*m_ptr).dpnd_type = 'D' as i32 as libc::c_char
        } else {
            /* 基本句内最後の形態素(inum == 0) もしくは semantic head */
            (*m_ptr).out_head_flag =
                1 as
                    libc::c_int; /* 基本句外に出る形態素であることを示す */
            if !last_content_m_ptr.is_null() &&
                   this_is_semantic_head_flag == 0 &&
                   !(appear_syntactic_head_flag != 0 &&
                         this_is_syntactic_head_flag == 0) {
                (*last_content_m_ptr).dpnd_head = (*m_ptr).num;
                (*last_content_m_ptr).dpnd_type = 'D' as i32 as libc::c_char;
                (*last_content_m_ptr).out_head_flag = 0 as libc::c_int;
                last_content_m_ptr = 0 as *mut MRPH_DATA
            }
            if i == (*sp).Mrph_num - 1 as libc::c_int {
                if root_m >= 0 as libc::c_int {
                    /* avoid multiple root */
                    (*m_ptr).dpnd_head = root_m
                } else {
                    (*m_ptr).dpnd_head = -(1 as libc::c_int);
                    root_m = (*m_ptr).num
                }
                (*m_ptr).dpnd_type = 'D' as i32 as libc::c_char
            } else {
                offset = 0 as libc::c_int;
                head = (*(*sp).tag_data.offset(last_t as isize)).dpnd_head;
                (*m_ptr).dpnd_type =
                    (*(*sp).tag_data.offset(last_t as isize)).dpnd_type;
                if head == -(1 as libc::c_int) {
                    if root_m >= 0 as libc::c_int {
                        /* avoid multiple root */
                        (*m_ptr).dpnd_head = root_m
                    } else {
                        (*m_ptr).dpnd_head = -(1 as libc::c_int);
                        root_m = (*m_ptr).num
                    }
                } else {
                    /* m_ptr->dpnd_head = (sp->tag_data + head)->head_ptr->num; */
                    head_ptr =
                        find_head_mrph_from_dpnd_bnst((*sp).tag_data.offset(last_t
                                                                                as
                                                                                isize)
                                                          as *mut BNST_DATA,
                                                      (*sp).tag_data.offset(head
                                                                                as
                                                                                isize)
                                                          as *mut BNST_DATA);
                    /* check projectivity */
                    if proj_table[i as usize] != 0 &&
                           proj_table[i as usize] < (*head_ptr).num {
                        if OptDisplay == 3 as libc::c_int {
                            fprintf(stderr,
                                    b";; violation of projectivity in mrph tree (%s: modified %dth mrph: %d -> %d)\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    if !(*sp).KNPSID.is_null() {
                                        (*sp).KNPSID.offset(5 as libc::c_int
                                                                as isize) as
                                            *const libc::c_char
                                    } else {
                                        b"?\x00" as *const u8 as
                                            *const libc::c_char
                                    }, i, (*head_ptr).num,
                                    proj_table[i as usize]);
                        }
                        (*m_ptr).dpnd_head = proj_table[i as usize]
                    } else {
                        (*m_ptr).dpnd_head = (*head_ptr).num;
                        /* update proj_table */
                        j = i + 1 as libc::c_int;
                        while j < (*m_ptr).dpnd_head {
                            if proj_table[j as usize] == 0 ||
                                   proj_table[j as usize] > (*m_ptr).dpnd_head
                               {
                                proj_table[j as usize] = (*m_ptr).dpnd_head
                            }
                            j += 1
                        }
                    }
                }
            }
        }
        i += 1;
        m_ptr = m_ptr.offset(1)
    };
}
/* 文末 */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_para_info(mut sp: *mut SENTENCE_DATA,
                                        mut dst: *mut BNST_DATA,
                                        mut src: *mut BNST_DATA) 
 /*==================================================================*/
 {
    (*dst).para_num = (*src).para_num;
    (*dst).para_key_type = (*src).para_key_type;
    (*dst).para_top_p = (*src).para_top_p;
    (*dst).para_type = (*src).para_type;
    (*dst).to_para_p = (*src).to_para_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn tag_bnst_postprocess(mut sp: *mut SENTENCE_DATA,
                                              mut flag: libc::c_int) 
 /*==================================================================*/
 {
    /* タグ単位・文節を後処理して、機能的なタグ単位をマージ
     flag == 0: num, dpnd_head の番号の付け替えはしない */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = -(1 as libc::c_int);
    let mut t_table: [libc::c_int; 200] = [0; 200];
    let mut b_table: [libc::c_int; 200] = [0; 200];
    let mut merge_to: libc::c_int = 0;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* タグ後処理用ルールの適用 
     FEATUREの伝搬はこの中で行う */
    assign_general_feature((*sp).tag_data as *mut libc::c_void, (*sp).Tag_num,
                           14 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
    /* マージするタグ・文節の処理 */
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        /* もとのnum, mrph_numを保存 */
        (*t_ptr).preserve_mrph_num = (*t_ptr).mrph_num;
        if (*t_ptr).bnum >= 0 as libc::c_int {
            /* 文節区切りでもあるとき */
            (*(*t_ptr).b_ptr).preserve_mrph_num = (*(*t_ptr).b_ptr).mrph_num
        } /* 無効なタグ単位である印をつけておく */
        if !check_feature((*t_ptr).f,
                          b"\xef\xbc\xb4\xe3\x83\x9e\xe3\x83\xbc\xe3\x82\xb8\xe2\x86\x90\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            merge_to = i - 1 as libc::c_int;
            while merge_to >= 0 as libc::c_int {
                if (*(*sp).tag_data.offset(merge_to as isize)).num >=
                       0 as libc::c_int {
                    break ;
                }
                merge_to -= 1
            }
            (*t_ptr).num = -(1 as libc::c_int);
            (*(*sp).tag_data.offset(merge_to as isize)).mrph_num +=
                (*t_ptr).mrph_num;
            (*(*sp).tag_data.offset(merge_to as isize)).dpnd_head =
                (*t_ptr).dpnd_head;
            (*(*sp).tag_data.offset(merge_to as isize)).dpnd_type =
                (*t_ptr).dpnd_type;
            j = 0 as libc::c_int;
            while j < (*t_ptr).mrph_num {
                let ref mut fresh9 =
                    (*(*sp).tag_data.offset(merge_to as isize)).length;
                *fresh9 =
                    (*fresh9 as
                         libc::c_ulong).wrapping_add(strlen((*(*t_ptr).mrph_ptr.offset(j
                                                                                           as
                                                                                           isize)).Goi2.as_mut_ptr()))
                        as libc::c_int as libc::c_int;
                j += 1
            }
            assign_cfeature(&mut (*(*sp).tag_data.offset(merge_to as
                                                             isize)).f,
                            b"\xe5\xbe\x8c\xe5\x87\xa6\xe7\x90\x86-\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe3\x83\x9e\xe3\x83\xbc\xe3\x82\xb8\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            /* <文節始>や<タグ単位始>のfeature消去はしない */
            if (*t_ptr).bnum >= 0 as libc::c_int {
                /* 文節区切りでもあるとき */
                merge_to = -(1 as libc::c_int);
                while (*(*t_ptr).b_ptr).num + merge_to >= 0 as libc::c_int {
                    if (*(*t_ptr).b_ptr.offset(merge_to as isize)).num >=
                           0 as libc::c_int {
                        break ;
                    }
                    merge_to -= 1
                }
                copy_para_info(sp, (*t_ptr).b_ptr.offset(merge_to as isize),
                               (*t_ptr).b_ptr);
                (*(*t_ptr).b_ptr).num = -(1 as libc::c_int);
                (*(*t_ptr).b_ptr.offset(merge_to as isize)).mrph_num +=
                    (*(*t_ptr).b_ptr).mrph_num;
                (*(*t_ptr).b_ptr.offset(merge_to as isize)).dpnd_head =
                    (*(*t_ptr).b_ptr).dpnd_head;
                (*(*t_ptr).b_ptr.offset(merge_to as isize)).dpnd_type =
                    (*(*t_ptr).b_ptr).dpnd_type;
                j = 0 as libc::c_int;
                while j < (*t_ptr).mrph_num {
                    let ref mut fresh10 =
                        (*(*t_ptr).b_ptr.offset(merge_to as isize)).length;
                    *fresh10 =
                        (*fresh10 as
                             libc::c_ulong).wrapping_add(strlen((*(*(*t_ptr).b_ptr).mrph_ptr.offset(j
                                                                                                        as
                                                                                                        isize)).Goi2.as_mut_ptr()))
                            as libc::c_int as libc::c_int;
                    j += 1
                }
            }
        } else { count += 1 }
        t_table[i as usize] = count;
        i += 1;
        t_ptr = t_ptr.offset(1)
    }
    if flag == 0 as libc::c_int { return }
    count = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if (*b_ptr).num != -(1 as libc::c_int) { count += 1 }
        b_table[i as usize] = count;
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    /* タグ単位番号の更新 */
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        if (*t_ptr).num != -(1 as libc::c_int) {
            /* numの更新 (★どこかで tag_data + num をするとだめ) */
            (*t_ptr).num = t_table[i as usize];
            if (*t_ptr).dpnd_head != -(1 as libc::c_int) {
                (*t_ptr).dpnd_head = t_table[(*t_ptr).dpnd_head as usize]
            }
        }
        if (*t_ptr).bnum >= 0 as libc::c_int {
            /* bnumの更新 (★どこかで bnst_data + bnum をするとだめ) */
            (*t_ptr).bnum = b_table[(*t_ptr).bnum as usize]
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
    }
    /* 文節番号の更新 */
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if (*b_ptr).num != -(1 as libc::c_int) {
            (*b_ptr).num = b_table[i as usize];
            if (*b_ptr).dpnd_head != -(1 as libc::c_int) {
                (*b_ptr).dpnd_head = b_table[(*b_ptr).dpnd_head as usize]
            }
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn undo_tag_bnst_postprocess(mut sp:
                                                       *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut b_count: libc::c_int = 0 as libc::c_int;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* nbestオプションなどでprint_result()が複数回呼ばれるときのために
     変更したnum, mrph_num, lengthを元に戻しておく */
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        (*t_ptr).num = i;
        (*t_ptr).mrph_num = (*t_ptr).preserve_mrph_num;
        calc_bnst_length(sp, t_ptr as *mut BNST_DATA);
        if (*t_ptr).bnum >= 0 as libc::c_int {
            /* 文節区切りでもあるとき */
            let fresh11 = b_count;
            b_count = b_count + 1;
            (*(*t_ptr).b_ptr).num = fresh11;
            (*t_ptr).bnum = (*(*t_ptr).b_ptr).num;
            (*(*t_ptr).b_ptr).mrph_num = (*(*t_ptr).b_ptr).preserve_mrph_num;
            calc_bnst_length(sp, (*t_ptr).b_ptr);
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_postprocess(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"\xe3\x80\x9c\xe3\x81\xa8\xe3\x81\xbf\xe3\x82\x89\xe3\x82\x8c\xe3\x82\x8b\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
               (*(*sp).bnst_data.offset(i as isize)).para_num !=
                   -(1 as libc::c_int) &&
               (*(*sp).para_data.offset((*(*sp).bnst_data.offset(i as
                                                                     isize)).para_num
                                            as isize)).status as libc::c_int
                   != 'x' as i32 {
            assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97:30\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            assign_cfeature(&mut (*(*(*sp).bnst_data.offset(i as
                                                                isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                                                                                                                    as
                                                                                                                    isize)).tag_num
                                                                                           as
                                                                                           isize).offset(-(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize))).f,
                            b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97:30\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_evaluation(mut sp: *mut SENTENCE_DATA,
                                         mut dpnd: DPND,
                                         mut eos_flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut one_score: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut rentai: libc::c_int = 0;
    let mut vacant_slot_num: libc::c_int = 0;
    let mut topic_score: libc::c_int = 0;
    let mut scase_check: [libc::c_int; 11] = [0; 11];
    let mut ha_check: libc::c_int = 0;
    let mut un_count: libc::c_int = 0;
    let mut pred_p: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut d_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* 依存構造だけを評価する場合の関数
     (各文節について，そこに係っている文節の評価点を計算)

     評価基準
     ========
     0. 係り先のdefault位置との差をペナルティに(kakari_uke.rule)

     1. 「〜は」(提題,係:未格)の係り先は優先されるものがある
     (bnst_etc.ruleで指定，並列のキーは並列解析後プログラムで指定)

     2. 「〜は」は一述語に一つ係ることを優先(時間,数量は別)

     3. すべての格要素は同一表層格が一述語に一つ係ることを優先(ガガは別)

     4. 未格，連体修飾先はガ,ヲ,ニ格の余っているスロット数だけ点数付与
  */
    score = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        g_ptr = (*sp).bnst_data.offset(i as isize);
        one_score = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < 11 as libc::c_int {
            scase_check[k as usize] = 0 as libc::c_int;
            k += 1
        }
        ha_check = 0 as libc::c_int;
        un_count = 0 as libc::c_int;
        if !check_feature((*g_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() ||
               !check_feature((*g_ptr).f,
                              b"\xe6\xba\x96\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            pred_p = 1 as libc::c_int
        } else { pred_p = 0 as libc::c_int }
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            d_ptr = (*sp).bnst_data.offset(j as isize);
            if dpnd.head[j as usize] == i {
                /* 係り先のDEFAULTの位置との差をペナルティに
	   ※ 提題はC,B'を求めて遠くに係ることがあるが，それが
	   他の係り先に影響しないよう,ペナルティに差をつける */
                if !check_feature((*d_ptr).f,
                                  b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    one_score -= dpnd.dflt[j as usize]
                } else {
                    one_score -= dpnd.dflt[j as usize] * 2 as libc::c_int
                }
                /* 読点をもつものが隣にかかることを防ぐ */
                if j + 1 as libc::c_int == i &&
                       !check_feature((*d_ptr).f,
                                      b"\xe8\xaa\xad\xe7\x82\xb9\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                    one_score -= 5 as libc::c_int
                }
                if pred_p != 0 &&
                       {
                           cp =
                               check_feature((*d_ptr).f,
                                             b"\xe4\xbf\x82\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut libc::c_char);
                           !cp.is_null()
                       } {
                    /* 未格 提題(「〜は」)の扱い */
                    if !check_feature((*d_ptr).f,
                                      b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() &&
                           strcmp(cp,
                                  b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                                      as *const u8 as *const libc::c_char) ==
                               0 {
                        /* 文末, 「〜が」など, 並列末, C, B'に係ることを優先 */
                        cp2 =
                            check_feature((*g_ptr).f,
                                          b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                        if !cp2.is_null() {
                            sscanf(cp2,
                                   b"%*[^:]:%d\x00" as *const u8 as
                                       *const libc::c_char,
                                   &mut topic_score as *mut libc::c_int);
                            one_score += topic_score
                        }
                        /* else {one_score -= 15;} */
                        /* 一つめの提題にだけ点を与える (時間,数量は別)
	       → 複数の提題が同一述語に係ることを防ぐ */
                        if !check_feature((*d_ptr).f,
                                          b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                               ||
                               !check_feature((*d_ptr).f,
                                              b"\xe6\x95\xb0\xe9\x87\x8f\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                           {
                            one_score += 10 as libc::c_int
                        } else if ha_check == 0 as libc::c_int {
                            one_score += 10 as libc::c_int;
                            ha_check = 1 as libc::c_int
                        }
                    }
                    k = case2num(cp.offset(3 as libc::c_int as isize));
                    /* 格要素一般の扱い */
                    /* 未格 : 数えておき，後で空スロットを調べる (時間,数量は別) */
                    if strcmp(cp,
                              b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char) == 0 {
                        if !check_feature((*d_ptr).f,
                                          b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                               ||
                               !check_feature((*d_ptr).f,
                                              b"\xe6\x95\xb0\xe9\x87\x8f\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                           {
                            one_score += 10 as libc::c_int
                        } else { un_count += 1 }
                    } else if strcmp(cp,
                                     b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                         as *const u8 as *const libc::c_char)
                                  == 0 {
                        if check_feature((*g_ptr).f,
                                         b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                            one_score += 10 as libc::c_int;
                            break ;
                        }
                    } else if strcmp(cp,
                                     b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                         as *const u8 as *const libc::c_char)
                                  == 0 {
                        if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char)
                                                   as usize] as libc::c_int !=
                               0 &&
                               scase_check[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) as
                                               usize] == 0 as libc::c_int {
                            one_score += 10 as libc::c_int;
                            scase_check[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char) as
                                            usize] = 1 as libc::c_int
                        } else if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char)
                                                          as usize] as
                                      libc::c_int != 0 &&
                                      scase_check[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char)
                                                      as usize] ==
                                          0 as libc::c_int {
                            one_score += 10 as libc::c_int;
                            scase_check[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char) as
                                            usize] = 1 as libc::c_int
                        }
                    } else if k != -(1 as libc::c_int) {
                        if scase_check[k as usize] == 0 as libc::c_int {
                            scase_check[k as usize] = 1 as libc::c_int;
                            one_score += 10 as libc::c_int
                        }
                    }
                    /* ノ格 : 体言以外なら break 
	     → それより前の格要素には点を与えない．
	     → ノ格がかかればそれより前の格はかからない

	     ※ 「体言」というのは判定詞のこと，ただし
	     文末などでは用言:動となっていることも
	     あるので，「体言」でチェック */
                    /* ガ格 : ガガ構文があるので少し複雑 */
                    /* 他の格 : 各格1つは点数をあたえる
	     ※ ニ格の場合，時間とそれ以外は区別する方がいいかも？ */
                    /* 「〜するのは〜だ」にボーナス 01/01/11
	     ほとんどの場合改善．

	     改善例)
	     「抗議したのも 任官を 拒否される 理由の 一つらしい」

	     「使うのは 恐ろしい ことだ。」
	     「円満決着に なるかどうかは 微妙な ところだ。」
	     ※ これらの例は「こと/ところだ」に係ると扱う

	     「他人に 教えるのが 好きになる やり方です」
	     ※ この例は曖昧だが，文脈上正しい

	     副作用例)
	     「だれが ＭＶＰか 分からない 試合でしょう」
	     「〜 殴るなど した 疑い。」
	     「ビザを 取るのも 大変な 時代。」
	     「波が 高まるのは 避けられそうにない 雲行きだ。」
	     「あまり 役立つとは 思われない 論理だ。」
	     「どう 折り合うかが 問題視されてきた 法だ。」
	     「認められるかどうかが 争われた 裁判で」

	     ※問題※
	     「あの戦争」が〜 のような場合も用言とみなされるのが問題
	  */
                    if !check_feature((*d_ptr).f,
                                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() &&
                           (!check_feature((*d_ptr).f,
                                           b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null() ||
                                !check_feature((*d_ptr).f,
                                               b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null())
                           &&
                           !check_feature((*g_ptr).f,
                                          b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                        one_score += 3 as libc::c_int
                    }
                }
            }
            j -= 1
        }
        /* 用言の場合，最終的に未格,ガ格,ヲ格,ニ格,連体修飾に対して
       ガ格,ヲ格,ニ格のスロット分だけ点数を与える */
        if pred_p != 0 {
            /* 連体修飾の場合，係先が
	 ・形式名詞,副詞的名詞
	 ・「予定」,「見込み」など
	 でなければ一つの格要素と考える */
            if !check_feature((*g_ptr).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                if !check_feature((*(*sp).bnst_data.offset(dpnd.head[i as
                                                                         usize]
                                                               as isize)).f,
                                  b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                       !check_feature((*(*sp).bnst_data.offset(dpnd.head[i as
                                                                             usize]
                                                                   as
                                                                   isize)).f,
                                      b"\xe3\x83\xab\xe3\x83\xbc\xe3\x83\xab\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                    rentai = 0 as libc::c_int;
                    one_score += 10 as libc::c_int
                    /* 外の関係ならここで加点 */
                } else {
                    rentai = 1 as libc::c_int
                    /* それ以外なら後で空きスロットをチェック */
                }
            } else { rentai = 0 as libc::c_int }
            /* 空いているガ格,ヲ格,ニ格,ガ２ */
            vacant_slot_num = 0 as libc::c_int;
            if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) as usize]
                   as libc::c_int -
                   scase_check[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char) as usize] ==
                   1 as libc::c_int {
                vacant_slot_num += 1
            }
            if (*g_ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) as usize]
                   as libc::c_int -
                   scase_check[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char) as usize] ==
                   1 as libc::c_int {
                vacant_slot_num += 1
            }
            if (*g_ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) as usize]
                   as libc::c_int -
                   scase_check[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char) as usize] ==
                   1 as libc::c_int && rentai == 1 as libc::c_int &&
                   !check_feature((*g_ptr).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                vacant_slot_num += 1
                /* ニ格は動詞で連体修飾の場合だけ考慮，つまり連体
	   修飾に割り当てるだけで，未格のスロットとはしない */
            }
            if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) as usize]
                   as libc::c_int -
                   scase_check[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char) as usize] ==
                   1 as libc::c_int {
                vacant_slot_num += 1
            }
            /* 空きスロット分だけ連体修飾，未格にスコアを与える */
            if rentai + un_count <= vacant_slot_num {
                one_score += (rentai + un_count) * 10 as libc::c_int
            } else { one_score += vacant_slot_num * 10 as libc::c_int }
        }
        score += one_score;
        if OptDisplay == 3 as libc::c_int {
            if i == 1 as libc::c_int {
                fprintf(Outfp,
                        b"Score:    \x00" as *const u8 as
                            *const libc::c_char);
            }
            if pred_p != 0 {
                fprintf(Outfp,
                        b"%2d*\x00" as *const u8 as *const libc::c_char,
                        one_score);
            } else {
                fprintf(Outfp,
                        b"%2d \x00" as *const u8 as *const libc::c_char,
                        one_score);
            }
        }
        i += 1
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp, b"=%d\n\x00" as *const u8 as *const libc::c_char,
                score);
    }
    if OptDisplay == 3 as libc::c_int ||
           OptNbest == (0 as libc::c_int == 0) as libc::c_int {
        dpnd_info_to_bnst(sp, &mut dpnd);
        if OptExpress & 2 as libc::c_int == 0 {
            dpnd_info_to_tag(sp, &mut dpnd);
        }
        if make_dpnd_tree(sp) != 0 {
            if OptExpress & 2 as libc::c_int == 0 {
                tree_conv::bnst_to_tag_tree(sp);
                /* タグ単位の木へ */
            }
            if OptNbest == (0 as libc::c_int == 0) as libc::c_int {
                (*sp).score = score as libc::c_double;
                print_result(sp, 0 as libc::c_int, eos_flag);
            } else {
                print_kakari(sp,
                             if OptExpress & 2 as libc::c_int != 0 {
                                 3 as libc::c_int
                             } else { 1 as libc::c_int }, eos_flag);
            }
        }
    }
    if score as libc::c_double > (*(*sp).Best_mgr).score {
        (*(*sp).Best_mgr).dpnd = dpnd;
        (*(*sp).Best_mgr).score = score as libc::c_double;
        (*(*sp).Best_mgr).ID = dpndID;
        Possibility += 1
    }
    dpndID += 1;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn count_dpnd_candidates(mut sp: *mut SENTENCE_DATA,
                                               mut dpnd: *mut DPND,
                                               mut pos: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut d_possibility: libc::c_int = 1 as libc::c_int;
    let mut b_ptr: *mut BNST_DATA = (*sp).bnst_data.offset(pos as isize);
    if pos == -(1 as libc::c_int) { return }
    if pos < (*sp).Bnst_num - 2 as libc::c_int {
        i = pos + 2 as libc::c_int;
        while i < (*dpnd).head[(pos + 1 as libc::c_int) as usize] {
            (*dpnd).mask[i as usize] = 0 as libc::c_int;
            i += 1
        }
    }
    i = pos + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if (*Quote_matrix.as_mut_ptr().offset(pos as isize))[i as usize] != 0
               && (*dpnd).mask[i as usize] != 0 {
            if (OptCKY != 0 || d_possibility != 0) &&
                   (*Dpnd_matrix.as_mut_ptr().offset(pos as
                                                         isize))[i as usize]
                       == 'd' as i32 {
                if OptCKY != 0 ||
                       check_uncertain_d_condition(sp, dpnd, i) != 0 {
                    (*dpnd).check[pos as usize].pos[count as usize] = i;
                    count += 1
                }
                d_possibility = 0 as libc::c_int
            } else if (*Dpnd_matrix.as_mut_ptr().offset(pos as
                                                            isize))[i as
                                                                        usize]
                          != 0 &&
                          (*Dpnd_matrix.as_mut_ptr().offset(pos as
                                                                isize))[i as
                                                                            usize]
                              != 'd' as i32 {
                (*dpnd).check[pos as usize].pos[count as usize] = i;
                count += 1;
                d_possibility = 0 as libc::c_int
            }
            /* バリアのチェック */
            if count != 0 &&
                   !(*(*b_ptr).dpnd_rule).barrier.fp[0 as libc::c_int as
                                                         usize].is_null() &&
                   feature_pattern_match(&mut (*(*b_ptr).dpnd_rule).barrier,
                                         (*(*sp).bnst_data.offset(i as
                                                                      isize)).f,
                                         b_ptr as *mut libc::c_void,
                                         (*sp).bnst_data.offset(i as isize) as
                                             *mut libc::c_void) ==
                       (0 as libc::c_int == 0) as libc::c_int {
                break ; /* 候補数 */
            }
        }
        i += 1
    }
    if count != 0 {
        (*dpnd).check[pos as usize].num = count;
        (*dpnd).check[pos as usize].def =
            if (*(*b_ptr).dpnd_rule).preference == -(1 as libc::c_int) {
                count
            } else { (*(*b_ptr).dpnd_rule).preference }
        /* デフォルトの位置 */
    }
    count_dpnd_candidates(sp, dpnd, pos - 1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn call_count_dpnd_candidates(mut sp:
                                                        *mut SENTENCE_DATA,
                                                    mut dpnd: *mut DPND) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        (*dpnd).mask[i as usize] = 1 as libc::c_int;
        memset(&mut *(*dpnd).check.as_mut_ptr().offset(i as isize) as
                   *mut CHECK_DATA as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<CHECK_DATA>() as libc::c_ulong);
        (*dpnd).check[i as usize].num = -(1 as libc::c_int);
        i += 1
    }
    count_dpnd_candidates(sp, dpnd, (*sp).Bnst_num - 1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn decide_dpnd(mut sp: *mut SENTENCE_DATA,
                                     mut dpnd: DPND,
                                     mut eos_flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut possibilities: [libc::c_int; 200] = [0; 200];
    let mut default_pos: libc::c_int = 0;
    let mut d_possibility: libc::c_int = 0;
    let mut MaskFlag: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    if OptDisplay == 3 as libc::c_int {
        if dpnd.pos == (*sp).Bnst_num - 1 as libc::c_int {
            fprintf(Outfp, b"------\x00" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < (*sp).Bnst_num {
                fprintf(Outfp,
                        b"-%02d\x00" as *const u8 as *const libc::c_char, i);
                i += 1
            }
            fputc('\n' as i32, Outfp);
        }
        fprintf(Outfp, b"In %2d:\x00" as *const u8 as *const libc::c_char,
                dpnd.pos);
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            fprintf(Outfp, b" %2d\x00" as *const u8 as *const libc::c_char,
                    dpnd.head[i as usize]);
            i += 1
        }
        fputc('\n' as i32, Outfp);
    }
    dpnd.pos -= 1;
    /* 文頭まで解析が終わったら評価関数をよぶ */
    if dpnd.pos == -(1 as libc::c_int) {
        /* 無格従属: 前の文節の係り受けに従う場合 */
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num - 1 as libc::c_int {
            if dpnd.head[i as usize] < 0 as libc::c_int {
                /* ありえない係り受け */
                if i >= dpnd.head[(i + dpnd.head[i as usize]) as usize] {
                    return
                }
                dpnd.head[i as usize] =
                    dpnd.head[(i + dpnd.head[i as usize]) as usize];
                dpnd.check[i as usize].pos[0 as libc::c_int as usize] =
                    dpnd.head[i as usize]
            }
            i += 1
        }
        if OptAnalysis == 2 as libc::c_int || OptAnalysis == 6 as libc::c_int
           {
            dpnd_evaluation(sp, dpnd, eos_flag);
        } else if OptAnalysis == 1 as libc::c_int {
            call_case_analysis(sp, dpnd, eos_flag);
        }
        return
    }
    b_ptr = (*sp).bnst_data.offset(dpnd.pos as isize);
    dpnd.f[dpnd.pos as usize] = (*b_ptr).f;
    /* (前の係りによる)非交差条件の設定 (dpnd.mask が 0 なら係れない) */
    if dpnd.pos < (*sp).Bnst_num - 2 as libc::c_int {
        i = dpnd.pos + 2 as libc::c_int;
        while i < dpnd.head[(dpnd.pos + 1 as libc::c_int) as usize] {
            dpnd.mask[i as usize] = 0 as libc::c_int;
            i += 1
        }
    }
    /* 並列構造のキー文節, 部分並列の文節<I>
     (すでに行われた並列構造解析の結果をマークするだけ) */
    i = dpnd.pos + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if (*Mask_matrix.as_mut_ptr().offset(dpnd.pos as isize))[i as usize]
               == 2 as libc::c_int {
            dpnd.head[dpnd.pos as usize] = i;
            dpnd.type_0[dpnd.pos as usize] = 'P' as i32 as libc::c_char;
            /* チェック用 */
      /* 並列の場合は一意に決まっているので、候補を挙げるのは意味がない */
            dpnd.check[dpnd.pos as usize].num = 1 as libc::c_int;
            dpnd.check[dpnd.pos as usize].pos[0 as libc::c_int as usize] = i;
            decide_dpnd(sp, dpnd, eos_flag);
            return
        } else {
            if (*Mask_matrix.as_mut_ptr().offset(dpnd.pos as
                                                     isize))[i as usize] ==
                   3 as libc::c_int {
                dpnd.head[dpnd.pos as usize] = i;
                dpnd.type_0[dpnd.pos as usize] = 'I' as i32 as libc::c_char;
                dpnd.check[dpnd.pos as usize].num = 1 as libc::c_int;
                dpnd.check[dpnd.pos as usize].pos[0 as libc::c_int as usize] =
                    i;
                decide_dpnd(sp, dpnd, eos_flag);
                return
            }
        }
        i += 1
    }
    /* 前の文節の係り受けに従う場合  例) 「〜大統領は一日，〜」 */
    cp =
        check_feature((*b_ptr).f,
                      b"\xe4\xbf\x82:\xe7\x84\xa1\xe6\xa0\xbc\xe5\xbe\x93\xe5\xb1\x9e\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() {
        sscanf(cp,
               b"%*[^:]:%*[^:]:%d\x00" as *const u8 as *const libc::c_char,
               &mut *dpnd.head.as_mut_ptr().offset(dpnd.pos as isize) as
                   *mut libc::c_int);
        dpnd.type_0[dpnd.pos as usize] = 'D' as i32 as libc::c_char;
        dpnd.dflt[dpnd.pos as usize] = 0 as libc::c_int;
        dpnd.check[dpnd.pos as usize].num = 1 as libc::c_int;
        decide_dpnd(sp, dpnd, eos_flag);
        return
    }
    /* 通常の係り受け解析 */
    /* 係り先の候補を調べる */
    count = 0 as libc::c_int;
    d_possibility = 1 as libc::c_int;
    i = dpnd.pos + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if (*Mask_matrix.as_mut_ptr().offset(dpnd.pos as isize))[i as usize]
               != 0 &&
               (*Quote_matrix.as_mut_ptr().offset(dpnd.pos as
                                                      isize))[i as usize] != 0
               && dpnd.mask[i as usize] != 0 {
            if d_possibility != 0 &&
                   (*Dpnd_matrix.as_mut_ptr().offset(dpnd.pos as
                                                         isize))[i as usize]
                       == 'd' as i32 {
                if check_uncertain_d_condition(sp, &mut dpnd, i) != 0 {
                    possibilities[count as usize] = i;
                    count += 1
                }
                d_possibility = 0 as libc::c_int
            } else if (*Dpnd_matrix.as_mut_ptr().offset(dpnd.pos as
                                                            isize))[i as
                                                                        usize]
                          != 0 &&
                          (*Dpnd_matrix.as_mut_ptr().offset(dpnd.pos as
                                                                isize))[i as
                                                                            usize]
                              != 'd' as i32 {
                possibilities[count as usize] = i;
                count += 1;
                d_possibility = 0 as libc::c_int
            }
            /* バリアのチェック */
            if count != 0 &&
                   !(*(*b_ptr).dpnd_rule).barrier.fp[0 as libc::c_int as
                                                         usize].is_null() &&
                   feature_pattern_match(&mut (*(*b_ptr).dpnd_rule).barrier,
                                         (*(*sp).bnst_data.offset(i as
                                                                      isize)).f,
                                         b_ptr as *mut libc::c_void,
                                         (*sp).bnst_data.offset(i as isize) as
                                             *mut libc::c_void) ==
                       (0 as libc::c_int == 0) as libc::c_int {
                break ;
            }
        } else { MaskFlag = 1 as libc::c_int }
        i += 1
    }
    /* 実際に候補をつくっていく(この関数の再帰的呼び出し) */
    if count != 0 {
        /* preference は一番近く:1, 二番目:2, 最後:-1
       default_pos は一番近く:1, 二番目:2, 最後:count に変更 */
        default_pos =
            if (*(*b_ptr).dpnd_rule).preference == -(1 as libc::c_int) {
                count
            } else { (*(*b_ptr).dpnd_rule).preference }; /* 候補数 */
        dpnd.check[dpnd.pos as usize].num =
            count; /* デフォルトの位置 */
        dpnd.check[dpnd.pos as usize].def = default_pos;
        i = 0 as libc::c_int;
        while i < count {
            dpnd.check[dpnd.pos as usize].pos[i as usize] =
                possibilities[i as usize];
            i += 1
        }
        /* 一意に決定する場合 */
        if (*(*b_ptr).dpnd_rule).barrier.fp[0 as libc::c_int as
                                                usize].is_null() ||
               (*(*b_ptr).dpnd_rule).decide != 0 {
            if default_pos <= count {
                dpnd.head[dpnd.pos as usize] =
                    possibilities[(default_pos - 1 as libc::c_int) as usize]
            } else {
                dpnd.head[dpnd.pos as usize] =
                    possibilities[(count - 1 as libc::c_int) as usize]
                /* default_pos が 2 なのに，countが 1 しかない場合 */
            }
            dpnd.type_0[dpnd.pos as usize] =
                (*Dpnd_matrix.as_mut_ptr().offset(dpnd.pos as
                                                      isize))[dpnd.head[dpnd.pos
                                                                            as
                                                                            usize]
                                                                  as usize] as
                    libc::c_char;
            dpnd.dflt[dpnd.pos as usize] = 0 as libc::c_int;
            decide_dpnd(sp, dpnd, eos_flag);
        } else {
            /* すべての可能性をつくり出す場合 */
    /* 節間の係り受けの場合は一意に決めるべき */
            i = 0 as libc::c_int;
            while i < count {
                dpnd.head[dpnd.pos as usize] = possibilities[i as usize];
                dpnd.type_0[dpnd.pos as usize] =
                    (*Dpnd_matrix.as_mut_ptr().offset(dpnd.pos as
                                                          isize))[dpnd.head[dpnd.pos
                                                                                as
                                                                                usize]
                                                                      as
                                                                      usize]
                        as libc::c_char;
                dpnd.dflt[dpnd.pos as usize] =
                    abs(default_pos - 1 as libc::c_int - i);
                decide_dpnd(sp, dpnd, eos_flag);
                i += 1
            }
        }
    } else if (*Mask_matrix.as_mut_ptr().offset(dpnd.pos as
                                                    isize))[((*sp).Bnst_num -
                                                                 1 as
                                                                     libc::c_int)
                                                                as usize] != 0
     {
        dpnd.head[dpnd.pos as usize] = (*sp).Bnst_num - 1 as libc::c_int;
        dpnd.type_0[dpnd.pos as usize] = 'D' as i32 as libc::c_char;
        dpnd.dflt[dpnd.pos as usize] = 10 as libc::c_int;
        dpnd.check[dpnd.pos as usize].num = 1 as libc::c_int;
        dpnd.check[dpnd.pos as usize].pos[0 as libc::c_int as usize] =
            (*sp).Bnst_num - 1 as libc::c_int;
        decide_dpnd(sp, dpnd, eos_flag);
    };
}
/* 係り先がない場合
     文末が並列にマスクされていなければ，文末に係るとする */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn when_no_dpnd_struct(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    (*(*sp).Best_mgr).dpnd.head[((*sp).Bnst_num - 1 as libc::c_int) as usize]
        = -(1 as libc::c_int);
    (*(*sp).Best_mgr).dpnd.type_0[((*sp).Bnst_num - 1 as libc::c_int) as
                                      usize] = 'D' as i32 as libc::c_char;
    i = (*sp).Bnst_num - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        (*(*sp).Best_mgr).dpnd.head[i as usize] = i + 1 as libc::c_int;
        (*(*sp).Best_mgr).dpnd.type_0[i as usize] =
            'D' as i32 as libc::c_char;
        (*(*sp).Best_mgr).dpnd.check[i as usize].num = 1 as libc::c_int;
        (*(*sp).Best_mgr).dpnd.check[i as
                                         usize].pos[0 as libc::c_int as usize]
            = i + 1 as libc::c_int;
        i -= 1
    }
    (*(*sp).Best_mgr).score = 0 as libc::c_int as libc::c_double;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn after_decide_dpnd(mut sp: *mut SENTENCE_DATA,
                                           mut eos_flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut check_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* 解析済: 構造は与えられたもの1つのみ */
    if OptInput & 1 as libc::c_int != 0 { Possibility = 1 as libc::c_int }
    return if Possibility != 0 as libc::c_int {
        /* 依存構造決定後 格解析を行う場合 */
        if OptAnalysis == 6 as libc::c_int {
            (*(*sp).Best_mgr).score =
                -(10000 as libc::c_int) as libc::c_double;
            if call_case_analysis(sp, (*(*sp).Best_mgr).dpnd, eos_flag) ==
                0 as libc::c_int {
                return 0 as libc::c_int
            }
        }
        /* 依存構造・格構造決定後の処理 */
        /* 格解析結果の情報をfeatureへ */
        if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int
        {
            /* 格解析結果を用言基本句featureへ */
            i = 0 as libc::c_int; /* 未対応格要素の処理 */
            while i < (*(*sp).Best_mgr).pred_num {
                assign_nil_assigned_components(sp,
                                               &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                                   as
                                                   isize));
                /* assign_case_component_feature(sp, &(sp->Best_mgr->cpm[i]), FALSE); <格要素-??>feature */
                /* 格フレームの意味情報を用言基本句featureへ */
                j = 0 as libc::c_int;
                while j < (*(*(*sp).Best_mgr).cpm[i as usize].cmm[0 as libc::c_int as usize].cf_ptr).element_num {
                    case_analysis::append_cf_feature(
                        &mut (*(*(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as isize)).pred_b_ptr).f,
                        &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as isize), (*(*sp).Best_mgr).cpm[i as usize].cmm[0 as libc::c_int as usize].cf_ptr,
                        j
                    );
                    j += 1
                }
                i += 1
            }
        }
        /* 木を作成 */
        dpnd_info_to_bnst(sp,
                          &mut (*(*sp).Best_mgr).dpnd); /* タグ単位の木へ */
        if make_dpnd_tree(sp) == 0 as libc::c_int { return 0 as libc::c_int }
        bnst_to_tag_tree(sp);
        if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int
        {
            /* 格解析の結果を用言文節へ */
            i = 0 as libc::c_int;
            while i < (*(*sp).Best_mgr).pred_num {
                (*(*(*sp).Best_mgr).cpm[i as usize].pred_b_ptr).cpm_ptr =
                    &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as
                        isize)
                        as *mut CF_PRED_MGR;
                /* ※ 暫定的
	   並列のときに make_dpnd_tree() を呼び出すと cpm_ptr がなくなるので、
	   ここでコピーしておく */
                check_b_ptr = (*(*sp).Best_mgr).cpm[i as usize].pred_b_ptr;
                while !(*check_b_ptr).parent.is_null() &&
                    (*(*check_b_ptr).parent).para_top_p as libc::c_int
                        == (0 as libc::c_int == 0) as libc::c_int &&
                    (*(*check_b_ptr).parent).cpm_ptr.is_null() {
                    (*(*check_b_ptr).parent).cpm_ptr =
                        &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as
                            isize)
                            as *mut CF_PRED_MGR;
                    check_b_ptr = (*check_b_ptr).parent
                }
                /* 各格要素の親用言を設定
	   ※ 文脈解析のときに格フレームを決定してなくても格解析は行っているので
	   これは成功する */
                j = 0 as libc::c_int;
                while j < (*(*sp).Best_mgr).cpm[i as usize].cf.element_num {
                    /* 省略解析の結果 or 連体修飾は除く */
                    if !((*(*sp).Best_mgr).cpm[i as
                        usize].elem_b_num[j as
                        usize]
                        <= -(2 as libc::c_int) ||
                        (*(*(*sp).Best_mgr).cpm[i as
                            usize].elem_b_ptr[j
                            as
                            usize]).num
                            >
                            (*(*(*sp).Best_mgr).cpm[i as
                                usize].pred_b_ptr).num)
                    {
                        (*(*(*sp).Best_mgr).cpm[i as
                            usize].elem_b_ptr[j as
                            usize]).pred_b_ptr
                            = (*(*sp).Best_mgr).cpm[i as usize].pred_b_ptr
                    }
                    j += 1
                }
                /* 格フレームがある場合 */
                if (*(*sp).Best_mgr).cpm[i as usize].result_num !=
                    0 as libc::c_int &&
                    (*(*(*sp).Best_mgr).cpm[i as
                        usize].cmm[0 as libc::c_int
                        as
                        usize].cf_ptr).cf_address
                        != -(1 as libc::c_int) as libc::c_ulonglong &&
                    (OptCaseFlag & 16 as libc::c_int != 0 &&
                        (*(*sp).Best_mgr).cpm[i as
                            usize].cmm[0 as
                            libc::c_int
                            as
                            usize].score
                            != -(1001 as libc::c_int) as libc::c_double ||
                        OptCaseFlag & 16 as libc::c_int == 0 &&
                            (*(*sp).Best_mgr).cpm[i as
                                usize].cmm[0 as
                                libc::c_int
                                as
                                usize].score
                                != -(2 as libc::c_int) as libc::c_double)
                {
                    /* 文脈解析のときは格フレーム決定している用言についてのみ */
                    if OptEllipsis == 0 ||
                        (*(*sp).Best_mgr).cpm[i as usize].decided ==
                            2 as libc::c_int {
                        if OptCaseFlag & 2 as libc::c_int != 0 {
                            assign_ga_subject(sp,
                                              &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                                  as
                                                  isize));
                        }
                        fix_sm_place(sp,
                                     &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                         as
                                         isize));
                        if OptUseSmfix ==
                            (0 as libc::c_int == 0) as libc::c_int {
                            specify_sm_from_cf(sp,
                                               &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                                   as
                                                   isize));
                        }
                        /* マッチした用例をfeatureに出力 *
	       record_match_ex(sp, &(sp->Best_mgr->cpm[i])); */
                        /* 直前格のマッチスコアをfeatureに出力 *
	       record_closest_cc_match(sp, &(sp->Best_mgr->cpm[i])); */
                        /* 格解析の結果を用いて形態素曖昧性を解消 */
                        verb_lexical_disambiguation_by_case_analysis(&mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                            as
                            isize));
                        noun_lexical_disambiguation_by_case_analysis(&mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                            as
                            isize));
                    } else if (*(*sp).Best_mgr).cpm[i as usize].decided ==
                        1 as libc::c_int {
                        if OptCaseFlag & 2 as libc::c_int != 0 {
                            assign_ga_subject(sp,
                                              &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                                  as
                                                  isize));
                        }
                    }
                    if (*(*sp).Best_mgr).cpm[i as usize].decided ==
                        2 as libc::c_int {
                        assign_cfeature(&mut (*(*(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                            as
                            isize)).pred_b_ptr).f,
                                        b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe6\xb1\xba\xe5\xae\x9a\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                    }
                }
                i += 1
            }
        }
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn detect_dpnd_case_struct(mut sp: *mut SENTENCE_DATA,
                                                 mut eos_flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int =
        0; /* スコアは「より大きい」時に入れ換えるので，
				   初期値は十分小さくしておく */
    let mut dpnd: DPND =
        DPND{head: [0; 200],
             type_0: [0; 200],
             dflt: [0; 200],
             mask: [0; 200],
             pos: 0,
             check: [CHECK_DATA{num: 0, def: 0, pos: [0; 200],}; 200],
             f: [0 as *mut FEATURE; 200],};
    (*(*sp).Best_mgr).score = -(10000 as libc::c_int) as libc::c_double;
    (*(*sp).Best_mgr).dflt = 0 as libc::c_int;
    (*(*sp).Best_mgr).ID = -(1 as libc::c_int);
    Possibility = 0 as libc::c_int;
    dpndID = 0 as libc::c_int;
    /* 係り状態の初期化 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        dpnd.head[i as usize] = -(1 as libc::c_int);
        dpnd.type_0[i as usize] = 'D' as i32 as libc::c_char;
        dpnd.dflt[i as usize] = 0 as libc::c_int;
        dpnd.mask[i as usize] = 1 as libc::c_int;
        memset(&mut *dpnd.check.as_mut_ptr().offset(i as isize) as
                   *mut CHECK_DATA as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<CHECK_DATA>() as libc::c_ulong);
        dpnd.check[i as usize].num = -(1 as libc::c_int);
        dpnd.f[i as usize] = 0 as *mut FEATURE;
        i += 1
    }
    dpnd.pos = (*sp).Bnst_num - 1 as libc::c_int;
    /* 格解析キャッシュの初期化 */
    if OptAnalysis == 1 as libc::c_int { InitCPMcache(); }
    /* 依存構造解析 --> 格構造解析 */
    decide_dpnd(sp, dpnd, eos_flag);
    /* 格解析キャッシュの初期化 */
    if OptAnalysis == 1 as libc::c_int { ClearCPMcache(); }
    return after_decide_dpnd(sp, eos_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_candidates(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b2t_table: [libc::c_int; 200] = [0; 200];
    let mut tm: *mut TOTAL_MGR = (*sp).Best_mgr;
    let mut b_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut t_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut tmp_b_buffer: [libc::c_char; 128] = [0; 128];
    let mut tmp_t_buffer: [libc::c_char; 128] = [0; 128];
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        /* 文節番号->基本句番号のテーブル */
        b2t_table[(*(*sp).bnst_data.offset(i as isize)).num as usize] =
            (*(*(*sp).bnst_data.offset(i as
                                           isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                                                                                               as
                                                                                               isize)).tag_num
                                                                      as
                                                                      isize).offset(-(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize))).num;
        i += 1
    }
    /* 各文節ごとにチェック用の feature を与える */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if (*tm).dpnd.check[i as usize].num != -(1 as libc::c_int) {
            /* 係り側 -> 係り先 */
            sprintf(b_buffer.as_mut_ptr(),
                    b"\xe5\x80\x99\xe8\xa3\x9c\x00" as *const u8 as
                        *const libc::c_char);
            sprintf(t_buffer.as_mut_ptr(),
                    b"\xe5\x80\x99\xe8\xa3\x9c\x00" as *const u8 as
                        *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*tm).dpnd.check[i as usize].num {
                /* 候補たち */
                sprintf(tmp_b_buffer.as_mut_ptr(),
                        b":%d\x00" as *const u8 as *const libc::c_char,
                        (*tm).dpnd.check[i as usize].pos[j as usize]);
                sprintf(tmp_t_buffer.as_mut_ptr(),
                        b":%d\x00" as *const u8 as *const libc::c_char,
                        b2t_table[(*tm).dpnd.check[i as usize].pos[j as usize]
                                      as usize]);
                if strlen(t_buffer.as_mut_ptr()).wrapping_add(strlen(tmp_t_buffer.as_mut_ptr()))
                       >= 5120 as libc::c_int as libc::c_ulong {
                    fprintf(stderr,
                            b";; Too long string <%s> (%d) in check_candidates. (%s)\n\x00"
                                as *const u8 as *const libc::c_char,
                            t_buffer.as_mut_ptr(),
                            (*tm).dpnd.check[i as usize].num,
                            if !(*sp).KNPSID.is_null() {
                                (*sp).KNPSID.offset(5 as libc::c_int as isize)
                                    as *const libc::c_char
                            } else {
                                b"?\x00" as *const u8 as *const libc::c_char
                            });
                    return
                }
                strcat(b_buffer.as_mut_ptr(), tmp_b_buffer.as_mut_ptr());
                strcat(t_buffer.as_mut_ptr(), tmp_t_buffer.as_mut_ptr());
                j += 1
            }
            assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            b_buffer.as_mut_ptr(), 0 as libc::c_int);
            assign_cfeature(&mut (*(*(*sp).bnst_data.offset(i as
                                                                isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                                                                                                                    as
                                                                                                                    isize)).tag_num
                                                                                           as
                                                                                           isize).offset(-(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize))).f,
                            t_buffer.as_mut_ptr(), 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn memo_by_program(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    /*
   *  プログラムによるメモへの書き込み
   */
    /* 緩和をメモに記録する場合
     int i;

     for (i = 0; i < sp->Bnst_num - 1; i++) {
     if (sp->Best_mgr->dpnd.type[i] == 'd') {
     strcat(PM_Memo, " 緩和d");
     sprintf(PM_Memo+strlen(PM_Memo), "(%d)", i);
     } else if (sp->Best_mgr->dpnd.type[i] == 'R') {
     strcat(PM_Memo, " 緩和R");
     sprintf(PM_Memo+strlen(PM_Memo), "(%d)", i);
     }
     }
  */
    /* 遠い係り受けをメモに記録する場合

  for (i = 0; i < sp->Bnst_num - 1; i++) {
  if (sp->Best_mgr->dpnd.head[i] > i + 3 &&
  !check_feature(sp->bnst_data[i].f, "ハ") &&
  !check_feature(sp->bnst_data[i].f, "読点") &&
  !check_feature(sp->bnst_data[i].f, "用言") &&
  !check_feature(sp->bnst_data[i].f, "係:ガ格") &&
  !check_feature(sp->bnst_data[i].f, "用言:無") &&
  !check_feature(sp->bnst_data[i].f, "並キ") &&
  !check_feature(sp->bnst_data[i+1].f, "括弧始")) {
  strcat(PM_Memo, " 遠係");
  sprintf(PM_Memo+strlen(PM_Memo), "(%d)", i);
  }
  }
  */
}
/* get gigaword pa count for Chinese, for cell (i,j), i is the position of argument, j is the position of predicate */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_gigaword_pa_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dis: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Chi_pa_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int as libc::c_double;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"PU\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
                   !check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                  b"PU\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                (*Chi_pa_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int as libc::c_double;
                (*Chi_pa_matrix.as_mut_ptr().offset(j as isize))[i as usize] =
                    0 as libc::c_int as libc::c_double
            } else {
                if j == i + 1 as libc::c_int {
                    dis = 1 as libc::c_int
                } else { dis = 2 as libc::c_int }
                (*Chi_pa_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    get_chi_pa((*sp).bnst_data.offset(i as isize),
                               (*sp).bnst_data.offset(j as isize), dis);
                (*Chi_pa_matrix.as_mut_ptr().offset(j as isize))[i as usize] =
                    get_chi_pa((*sp).bnst_data.offset(j as isize),
                               (*sp).bnst_data.offset(i as isize), dis)
            }
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_prob(mut sp: *mut SENTENCE_DATA,
                                       mut head: libc::c_int,
                                       mut left_arg_num: libc::c_int,
                                       mut right_arg_num: libc::c_int)
 -> libc::c_double 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut case_prob: libc::c_double = 0.0f64;
    let mut lex_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bk_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_len: libc::c_int = 0 as libc::c_int;
    let mut right_arg_len: libc::c_int = 0 as libc::c_int;
    let mut lex_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bk_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lex_occur: [libc::c_double; 2] = [0.; 2];
    let mut lex_total: [libc::c_double; 2] = [0.; 2];
    let mut bk_occur: [libc::c_double; 2] = [0.; 2];
    let mut bk_total: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut total: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut count: libc::c_int = 0;
    let mut lamda_ctb: libc::c_double = 0.;
    let mut lamda_giga: libc::c_double = 0.;
    //printf("\nhead:%d left:", head);
    if OptChiGenerative != 0 && CHICaseExist == 0 as libc::c_int {
        return case_prob
    }
    if left_arg_num >= 30 as libc::c_int || right_arg_num >= 30 as libc::c_int
       {
        fprintf(stderr,
                b";;; number of arguments exceeded maximum\n\x00" as *const u8
                    as *const libc::c_char);
        return case_prob
    }
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            left_arg_len =
                (left_arg_len as
                     libc::c_ulong).wrapping_add(strlen((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                     as
                                                                                                                     isize)
                                                                                       as
                                                                                       isize)).head_ptr).Type.as_mut_ptr()).wrapping_add(1
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i += 1
        }
    } else { left_arg_len = 4 as libc::c_int }
    left_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((left_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", left_arg[i]);
            if i == left_arg_num - 1 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                as
                                                                                                                                isize)
                                                                                                  as
                                                                                                  isize)).head_ptr).Type.as_mut_ptr().offset(2
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)
                                                       as
                                                       isize)).head_ptr).Type.as_mut_ptr());
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                as
                                                                                                                                isize)
                                                                                                  as
                                                                                                  isize)).head_ptr).Type.as_mut_ptr().offset(1
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        (*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)
                                                       as
                                                       isize)).head_ptr).Type.as_mut_ptr());
            }
            if i == 0 as libc::c_int {
                strcpy(left_arg_key, tmp_key);
            } else { strcat(left_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i += 1
        }
    } else {
        strcpy(left_arg_key, b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    //printf(" right:");
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            right_arg_len =
                (right_arg_len as
                     libc::c_ulong).wrapping_add(strlen((*(*(*sp).bnst_data.offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                      as
                                                                                                                      isize)
                                                                                       as
                                                                                       isize)).head_ptr).Type.as_mut_ptr()).wrapping_add(1
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i -= 1
        }
    } else { right_arg_len = 4 as libc::c_int }
    right_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((right_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", right_arg[i]);
            if i == 0 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*(*(*sp).bnst_data.offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                 as
                                                                                                                                 isize)
                                                                                                  as
                                                                                                  isize)).head_ptr).Type.as_mut_ptr().offset(2
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*(*sp).bnst_data.offset(*right_arg.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize)
                                                       as
                                                       isize)).head_ptr).Type.as_mut_ptr());
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*(*(*sp).bnst_data.offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                 as
                                                                                                                                 isize)
                                                                                                  as
                                                                                                  isize)).head_ptr).Type.as_mut_ptr().offset(1
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        (*(*(*sp).bnst_data.offset(*right_arg.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize)
                                                       as
                                                       isize)).head_ptr).Type.as_mut_ptr());
            }
            if i == right_arg_num - 1 as libc::c_int {
                strcpy(right_arg_key, tmp_key);
            } else { strcat(right_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i -= 1
        }
    } else {
        strcpy(right_arg_key,
               b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    /* get lex rule */
    lex_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul(((left_arg_len +
                                                      right_arg_len) as
                                                     libc::c_ulong).wrapping_add(strlen((*(*(*sp).bnst_data.offset(head
                                                                                                                       as
                                                                                                                       isize)).head_ptr).Goi.as_mut_ptr())).wrapping_add(strlen((*(*(*sp).bnst_data.offset(head
                                                                                                                                                                                                               as
                                                                                                                                                                                                               isize)).head_ptr).Type.as_mut_ptr())).wrapping_add(9
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                      libc::c_ulong)))
            as *mut libc::c_char;
    sprintf(lex_key,
            b"(%s_%s)_(%s)_(%s)\x00" as *const u8 as *const libc::c_char,
            (*(*(*sp).bnst_data.offset(head as
                                           isize)).head_ptr).Type.as_mut_ptr(),
            (*(*(*sp).bnst_data.offset(head as
                                           isize)).head_ptr).Goi.as_mut_ptr(),
            left_arg_key, right_arg_key);
    lex_rule = db_get(chi_case_db, lex_key);
    //printf("\nlex_key: %s", lex_key);
  //printf("\nlex_rule: %s", lex_rule);
    /* get bk rule */
    bk_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul(((left_arg_len +
                                                      right_arg_len) as
                                                     libc::c_ulong).wrapping_add(strlen((*(*(*sp).bnst_data.offset(head
                                                                                                                       as
                                                                                                                       isize)).head_ptr).Type.as_mut_ptr())).wrapping_add(11
                                                                                                                                                                              as
                                                                                                                                                                              libc::c_int
                                                                                                                                                                              as
                                                                                                                                                                              libc::c_ulong)))
            as *mut libc::c_char;
    sprintf(bk_key,
            b"(%s_XX)_(%s)_(%s)\x00" as *const u8 as *const libc::c_char,
            (*(*(*sp).bnst_data.offset(head as
                                           isize)).head_ptr).Type.as_mut_ptr(),
            left_arg_key, right_arg_key);
    bk_rule = db_get(chi_case_db, bk_key);
    //printf("\nbk_key: %s", bk_key);
  //printf("\nbk_rule: %s\n", bk_rule);
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lex_occur[k as usize] = 0 as libc::c_int as libc::c_double;
        lex_total[k as usize] = 0 as libc::c_int as libc::c_double;
        bk_occur[k as usize] = 0 as libc::c_int as libc::c_double;
        bk_total[k as usize] = 0 as libc::c_int as libc::c_double;
        k += 1
    }
    if !lex_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(lex_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(strlen(rule).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)))
                    as *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            occur = 0 as *mut libc::c_char;
            total = 0 as *mut libc::c_char;
            type_0 = 0 as *mut libc::c_char;
            occur =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            total =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            type_0 =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(type_0,
                      b"TRAIN\x00" as *const u8 as *const libc::c_char) == 0 {
                lex_occur[0 as libc::c_int as usize] = atof(occur);
                lex_total[0 as libc::c_int as usize] = atof(total)
            } else if strcmp(type_0,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                lex_occur[1 as libc::c_int as usize] = atof(occur);
                lex_total[1 as libc::c_int as usize] = atof(total)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    if !bk_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(bk_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(strlen(rule).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)))
                    as *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            occur = 0 as *mut libc::c_char;
            total = 0 as *mut libc::c_char;
            type_0 = 0 as *mut libc::c_char;
            occur =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            total =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            type_0 =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(type_0,
                      b"TRAIN\x00" as *const u8 as *const libc::c_char) == 0 {
                bk_occur[0 as libc::c_int as usize] = atof(occur);
                bk_total[0 as libc::c_int as usize] = atof(total)
            } else if strcmp(type_0,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                bk_occur[1 as libc::c_int as usize] = atof(occur);
                bk_total[1 as libc::c_int as usize] = atof(total)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        prob[k as usize] = 0.0f64;
        if lex_total[k as usize] > 0.0000000000000001f64 {
            lamda =
                lex_occur[k as usize] /
                    (lex_occur[k as usize] +
                         1 as libc::c_int as libc::c_double);
            prob[k as usize] =
                lamda * (lex_occur[k as usize] / lex_total[k as usize]);
            prob[k as usize] +=
                (1 as libc::c_int as libc::c_double - lamda) *
                    (bk_occur[k as usize] / bk_total[k as usize])
        } else if bk_total[k as usize] > 0.0000000000000001f64 {
            lamda =
                bk_occur[k as usize] /
                    (bk_occur[k as usize] +
                         1 as libc::c_int as libc::c_double);
            prob[k as usize] =
                lamda * bk_occur[k as usize] / bk_total[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
            //	lamda = log(lex_occur[0] + bk_occur[0]) / (log(lex_occur[0] + bk_occur[0]) + 1);
            lamda_ctb =
                (lex_occur[0 as libc::c_int as usize] +
                     bk_occur[0 as libc::c_int as usize]) /
                    (lex_occur[0 as libc::c_int as usize] +
                         bk_occur[0 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda_giga =
                (lex_occur[1 as libc::c_int as usize] +
                     bk_occur[1 as libc::c_int as usize]) /
                    (lex_occur[1 as libc::c_int as usize] +
                         bk_occur[1 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda =
                if lamda_ctb < lamda_giga {
                    (lamda_ctb) / lamda_giga
                } else { (lamda_giga) / lamda_ctb };
            case_prob =
                prob[0 as libc::c_int as usize] * lamda +
                    prob[1 as libc::c_int as usize] *
                        (1 as libc::c_int as libc::c_double - lamda)
        } else { case_prob = prob[0 as libc::c_int as usize] }
    } else if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
        //	lamda = (lex_occur[1] + bk_occur[1]) / (lex_occur[1] + bk_occur[1] + 1);
        lamda =
            log(lex_occur[1 as libc::c_int as usize] +
                    bk_occur[1 as libc::c_int as usize]) /
                (log(lex_occur[1 as libc::c_int as usize] +
                         bk_occur[1 as libc::c_int as usize]) +
                     1 as libc::c_int as libc::c_double);
        case_prob = prob[1 as libc::c_int as usize] * lamda
    } else { case_prob = 0 as libc::c_int as libc::c_double }
    //printf("prob_ctb: %f, prob_giga: %f, lamda: %f", prob[0], prob[1], lamda);
  //printf("prob: %f\n\n", case_prob);
    if !left_arg_key.is_null() {
        free(left_arg_key as *mut libc::c_void);
        left_arg_key = 0 as *mut libc::c_char
    }
    if !right_arg_key.is_null() {
        free(right_arg_key as *mut libc::c_void);
        right_arg_key = 0 as *mut libc::c_char
    }
    if !lex_key.is_null() {
        free(lex_key as *mut libc::c_void);
        lex_key = 0 as *mut libc::c_char
    }
    if !bk_key.is_null() {
        free(bk_key as *mut libc::c_void);
        bk_key = 0 as *mut libc::c_char
    }
    if !lex_rule.is_null() {
        free(lex_rule as *mut libc::c_void);
        lex_rule = 0 as *mut libc::c_char
    }
    if !bk_rule.is_null() {
        free(bk_rule as *mut libc::c_void);
        bk_rule = 0 as *mut libc::c_char
    }
    return case_prob;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_prob_wpos(mut sp: *mut SENTENCE_DATA,
                                            mut head: libc::c_int,
                                            mut left_arg_num: libc::c_int,
                                            mut right_arg_num: libc::c_int,
                                            mut pos_index_pre: libc::c_int)
 -> libc::c_double 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut case_prob: libc::c_double = 0.0f64;
    let mut bk_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_len: libc::c_int = 0 as libc::c_int;
    let mut right_arg_len: libc::c_int = 0 as libc::c_int;
    let mut bk_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bk_occur: [libc::c_double; 2] = [0.; 2];
    let mut bk_total: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut total: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut count: libc::c_int = 0;
    let mut lamda_ctb: libc::c_double = 0.;
    let mut lamda_giga: libc::c_double = 0.;
    //printf("\nhead:%d left:", head);
    if OptChiGenerative != 0 && CHICaseExist == 0 as libc::c_int {
        return case_prob
    }
    if left_arg_num >= 30 as libc::c_int || right_arg_num >= 30 as libc::c_int
       {
        fprintf(stderr,
                b";;; number of arguments exceeded maximum\n\x00" as *const u8
                    as *const libc::c_char);
        return case_prob
    }
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            left_arg_len =
                (left_arg_len as
                     libc::c_ulong).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                             as
                                                                                                                             isize)
                                                                                               as
                                                                                               isize)).wrapping_add(1
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i += 1
        }
    } else { left_arg_len = 4 as libc::c_int }
    left_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((left_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", left_arg[i]);
            if i == left_arg_num - 1 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize)
                                                                                                           as
                                                                                                           isize)).offset(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                               as isize));
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize)
                                                                                                           as
                                                                                                           isize)).offset(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                               as isize));
            }
            if i == 0 as libc::c_int {
                strcpy(left_arg_key, tmp_key);
            } else { strcat(left_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i += 1
        }
    } else {
        strcpy(left_arg_key, b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    //printf(" right:");
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            right_arg_len =
                (right_arg_len as
                     libc::c_ulong).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                              as
                                                                                                                              isize)
                                                                                               as
                                                                                               isize)).wrapping_add(1
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i -= 1
        }
    } else { right_arg_len = 4 as libc::c_int }
    right_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((right_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", right_arg[i]);
            if i == 0 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                           as
                                                                                                           isize)).offset(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)
                                                               as isize));
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                           as
                                                                                                           isize)).offset(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)
                                                               as isize));
            }
            if i == right_arg_num - 1 as libc::c_int {
                strcpy(right_arg_key, tmp_key);
            } else { strcat(right_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i -= 1
        }
    } else {
        strcpy(right_arg_key,
               b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    /* get bk rule */
    bk_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul(((left_arg_len +
                                                      right_arg_len) as
                                                     libc::c_ulong).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(pos_index_pre
                                                                                                                               as
                                                                                                                               isize))).wrapping_add(11
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong)))
            as *mut libc::c_char;
    sprintf(bk_key,
            b"(%s_XX)_(%s)_(%s)\x00" as *const u8 as *const libc::c_char,
            *Chi_word_type.as_mut_ptr().offset(pos_index_pre as isize),
            left_arg_key, right_arg_key);
    bk_rule = db_get(chi_case_db, bk_key);
    //printf("\nbk_key: %s", bk_key);
  //printf("\nbk_rule: %s\n", bk_rule);
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        bk_occur[k as usize] = 0 as libc::c_int as libc::c_double;
        bk_total[k as usize] = 0 as libc::c_int as libc::c_double;
        k += 1
    }
    if !bk_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(bk_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(strlen(rule).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)))
                    as *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            occur = 0 as *mut libc::c_char;
            total = 0 as *mut libc::c_char;
            type_0 = 0 as *mut libc::c_char;
            occur =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            total =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            type_0 =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(type_0,
                      b"TRAIN\x00" as *const u8 as *const libc::c_char) == 0 {
                bk_occur[0 as libc::c_int as usize] = atof(occur);
                bk_total[0 as libc::c_int as usize] = atof(total)
            } else if strcmp(type_0,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                bk_occur[1 as libc::c_int as usize] = atof(occur);
                bk_total[1 as libc::c_int as usize] = atof(total)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        prob[k as usize] = 0.0f64;
        if bk_total[k as usize] > 0.0000000000000001f64 {
            lamda =
                bk_occur[k as usize] /
                    (bk_occur[k as usize] +
                         1 as libc::c_int as libc::c_double);
            prob[k as usize] =
                lamda * bk_occur[k as usize] / bk_total[k as usize]
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
            lamda_ctb =
                bk_occur[0 as libc::c_int as usize] /
                    (bk_occur[0 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda_giga =
                bk_occur[1 as libc::c_int as usize] /
                    (bk_occur[1 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda =
                if lamda_ctb < lamda_giga {
                    (lamda_ctb) / lamda_giga
                } else { (lamda_giga) / lamda_ctb };
            case_prob =
                prob[0 as libc::c_int as usize] * lamda +
                    prob[1 as libc::c_int as usize] *
                        (1 as libc::c_int as libc::c_double - lamda)
        } else { case_prob = prob[0 as libc::c_int as usize] }
    } else if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
        lamda =
            log(bk_occur[1 as libc::c_int as usize]) /
                (log(bk_occur[1 as libc::c_int as usize]) +
                     1 as libc::c_int as libc::c_double);
        case_prob = prob[1 as libc::c_int as usize] * lamda
    } else { case_prob = 0 as libc::c_int as libc::c_double }
    //printf("prob_ctb: %f, prob_giga: %f, lamda: %f", prob[0], prob[1], lamda);
  //printf("prob: %f\n\n", case_prob);
    if !left_arg_key.is_null() {
        free(left_arg_key as *mut libc::c_void);
        left_arg_key = 0 as *mut libc::c_char
    }
    if !right_arg_key.is_null() {
        free(right_arg_key as *mut libc::c_void);
        right_arg_key = 0 as *mut libc::c_char
    }
    if !bk_key.is_null() {
        free(bk_key as *mut libc::c_void);
        bk_key = 0 as *mut libc::c_char
    }
    if !bk_rule.is_null() {
        free(bk_rule as *mut libc::c_void);
        bk_rule = 0 as *mut libc::c_char
    }
    return case_prob;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_lex_case_prob_wpos(mut sp: *mut SENTENCE_DATA,
                                                mut head: libc::c_int,
                                                mut left_arg_num: libc::c_int,
                                                mut right_arg_num:
                                                    libc::c_int,
                                                mut pos_index_pre:
                                                    libc::c_int)
 -> libc::c_double 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut case_prob: libc::c_double = 0.0f64;
    let mut lex_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right_arg_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_arg_len: libc::c_int = 0 as libc::c_int;
    let mut right_arg_len: libc::c_int = 0 as libc::c_int;
    let mut lex_rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lex_occur: [libc::c_double; 2] = [0.; 2];
    let mut lex_total: [libc::c_double; 2] = [0.; 2];
    let mut lamda: libc::c_double = 0.;
    let mut prob: [libc::c_double; 2] = [0.; 2];
    let mut rule: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut occur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut total: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curRule: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut count: libc::c_int = 0;
    let mut lamda_ctb: libc::c_double = 0.;
    let mut lamda_giga: libc::c_double = 0.;
    //printf("\nhead:%d left:", head);
    if OptChiGenerative != 0 && CHICaseExist == 0 as libc::c_int {
        return case_prob
    }
    if left_arg_num >= 30 as libc::c_int || right_arg_num >= 30 as libc::c_int
       {
        fprintf(stderr,
                b";;; number of arguments exceeded maximum\n\x00" as *const u8
                    as *const libc::c_char);
        return case_prob
    }
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            left_arg_len =
                (left_arg_len as
                     libc::c_ulong).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                             as
                                                                                                                             isize)
                                                                                               as
                                                                                               isize)).wrapping_add(1
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i += 1
        }
    } else { left_arg_len = 4 as libc::c_int }
    left_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((left_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if left_arg_num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < left_arg_num {
            if *left_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", left_arg[i]);
            if i == left_arg_num - 1 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize)
                                                                                                           as
                                                                                                           isize)).offset(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                               as isize));
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize)
                                                                                                           as
                                                                                                           isize)).offset(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*left_arg.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                               as isize));
            }
            if i == 0 as libc::c_int {
                strcpy(left_arg_key, tmp_key);
            } else { strcat(left_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i += 1
        }
    } else {
        strcpy(left_arg_key, b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    //printf(" right:");
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            right_arg_len =
                (right_arg_len as
                     libc::c_ulong).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                              as
                                                                                                                              isize)
                                                                                               as
                                                                                               isize)).wrapping_add(1
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i -= 1
        }
    } else { right_arg_len = 4 as libc::c_int }
    right_arg_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((right_arg_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut libc::c_char;
    if right_arg_num > 0 as libc::c_int {
        i = right_arg_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *right_arg.as_mut_ptr().offset(i as isize) ==
                   -(1 as libc::c_int) {
                break ;
            }
            //printf("%d,", right_arg[i]);
            if i == 0 as libc::c_int {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                           as
                                                                                                           isize)).offset(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)
                                                               as isize));
            } else {
                tmp_key =
                    malloc((::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong).wrapping_mul(strlen((*Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                           as
                                                                                                           isize)).offset(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))))
                        as *mut libc::c_char;
                sprintf(tmp_key,
                        b"%s_\x00" as *const u8 as *const libc::c_char,
                        *Chi_word_type.as_mut_ptr().offset(*right_arg.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)
                                                               as isize));
            }
            if i == right_arg_num - 1 as libc::c_int {
                strcpy(right_arg_key, tmp_key);
            } else { strcat(right_arg_key, tmp_key); }
            if !tmp_key.is_null() {
                free(tmp_key as *mut libc::c_void);
                tmp_key = 0 as *mut libc::c_char
            }
            i -= 1
        }
    } else {
        strcpy(right_arg_key,
               b"NULL\x00" as *const u8 as *const libc::c_char);
    }
    /* get lex rule */
    lex_key =
        malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul(((left_arg_len +
                                                      right_arg_len) as
                                                     libc::c_ulong).wrapping_add(strlen((*(*(*sp).bnst_data.offset(head
                                                                                                                       as
                                                                                                                       isize)).head_ptr).Goi.as_mut_ptr())).wrapping_add(strlen(*Chi_word_type.as_mut_ptr().offset(pos_index_pre
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       isize))).wrapping_add(9
                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                 libc::c_ulong)))
            as *mut libc::c_char;
    sprintf(lex_key,
            b"(%s_%s)_(%s)_(%s)\x00" as *const u8 as *const libc::c_char,
            *Chi_word_type.as_mut_ptr().offset(pos_index_pre as isize),
            (*(*(*sp).bnst_data.offset(head as
                                           isize)).head_ptr).Goi.as_mut_ptr(),
            left_arg_key, right_arg_key);
    lex_rule = db_get(chi_case_db, lex_key);
    //printf("\nlex_key: %s", lex_key);
  //printf("\nlex_rule: %s", lex_rule);
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        lex_occur[k as usize] = 0 as libc::c_int as libc::c_double;
        lex_total[k as usize] = 0 as libc::c_int as libc::c_double;
        k += 1
    }
    if !lex_rule.is_null() {
        count = 0 as libc::c_int;
        rule = 0 as *mut libc::c_char;
        rule = strtok(lex_rule, b":\x00" as *const u8 as *const libc::c_char);
        while !rule.is_null() {
            curRule[count as usize] =
                malloc((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(strlen(rule).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)))
                    as *mut libc::c_char;
            strcpy(curRule[count as usize], rule);
            count += 1;
            rule = 0 as *mut libc::c_char;
            rule =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char)
        }
        k = 0 as libc::c_int;
        while k < count {
            occur = 0 as *mut libc::c_char;
            total = 0 as *mut libc::c_char;
            type_0 = 0 as *mut libc::c_char;
            occur =
                strtok(curRule[k as usize],
                       b"_\x00" as *const u8 as *const libc::c_char);
            total =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            type_0 =
                strtok(0 as *mut libc::c_char,
                       b"_\x00" as *const u8 as *const libc::c_char);
            if strcmp(type_0,
                      b"TRAIN\x00" as *const u8 as *const libc::c_char) == 0 {
                lex_occur[0 as libc::c_int as usize] = atof(occur);
                lex_total[0 as libc::c_int as usize] = atof(total)
            } else if strcmp(type_0,
                             b"GIGA\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                lex_occur[1 as libc::c_int as usize] = atof(occur);
                lex_total[1 as libc::c_int as usize] = atof(total)
            }
            if !curRule[k as usize].is_null() {
                free(curRule[k as usize] as *mut libc::c_void);
                curRule[k as usize] = 0 as *mut libc::c_char
            }
            k += 1
        }
    }
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        prob[k as usize] = 0.0f64;
        if lex_total[k as usize] > 0.0000000000000001f64 {
            lamda =
                lex_occur[k as usize] /
                    (lex_occur[k as usize] +
                         1 as libc::c_int as libc::c_double);
            prob[k as usize] =
                lamda * (lex_occur[k as usize] / lex_total[k as usize]);
            prob[k as usize] +=
                (1 as libc::c_int as libc::c_double - lamda) *
                    (lex_occur[k as usize] / lex_total[k as usize])
        }
        k += 1
    }
    if prob[0 as libc::c_int as usize] > 0.0000000000000001f64 {
        if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
            lamda_ctb =
                lex_occur[0 as libc::c_int as usize] /
                    (lex_occur[0 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda_giga =
                lex_occur[1 as libc::c_int as usize] /
                    (lex_occur[1 as libc::c_int as usize] +
                         1 as libc::c_int as libc::c_double);
            lamda =
                if lamda_ctb < lamda_giga {
                    (lamda_ctb) / lamda_giga
                } else { (lamda_giga) / lamda_ctb };
            case_prob =
                prob[0 as libc::c_int as usize] * lamda +
                    prob[1 as libc::c_int as usize] *
                        (1 as libc::c_int as libc::c_double - lamda)
        } else { case_prob = prob[0 as libc::c_int as usize] }
    } else if prob[1 as libc::c_int as usize] > 0.0000000000000001f64 {
        lamda =
            log(lex_occur[1 as libc::c_int as usize]) /
                (log(lex_occur[1 as libc::c_int as usize]) +
                     1 as libc::c_int as libc::c_double);
        case_prob = prob[1 as libc::c_int as usize] * lamda
    } else { case_prob = 0 as libc::c_int as libc::c_double }
    //printf("prob_ctb: %f, prob_giga: %f, lamda: %f", prob[0], prob[1], lamda);
  //printf("prob: %f\n\n", case_prob);
    if !left_arg_key.is_null() {
        free(left_arg_key as *mut libc::c_void);
        left_arg_key = 0 as *mut libc::c_char
    }
    if !right_arg_key.is_null() {
        free(right_arg_key as *mut libc::c_void);
        right_arg_key = 0 as *mut libc::c_char
    }
    if !lex_key.is_null() {
        free(lex_key as *mut libc::c_void);
        lex_key = 0 as *mut libc::c_char
    }
    if !lex_rule.is_null() {
        free(lex_rule as *mut libc::c_void);
        lex_rule = 0 as *mut libc::c_char
    }
    return case_prob;
}
/*====================================================================
                                  END
====================================================================*/
