#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]


use libc;
use crate::{bnst_compare, BNST_DATA, dic, FEATURE, TAG_DATA, tnode_t, tools};
use crate::case_analysis::{get_closest_case_component, MatchPP, pp_code_to_kstr, pp_hstr_to_code, pp_kstr_to_code};
use crate::ctools::{check_feature, Class, exit, fprintf, free, get_mrph_rep_from_f, sprintf, stderr, strcat, strcmp, strcpy, strdup, strlen, strncmp, strtok};
use crate::lib_bgh::bgh_match_check;
use crate::lib_sm::sm2code;
use crate::structs::CDB_FILE;
use crate::tools::{OptAnaphora, OptCaseFlag, Thesaurus};
use crate::types::{CASE_FRAME, CF_PRED_MGR, DBM_FILE, SENTENCE_DATA};

#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut FukugojiYomiTable: [*mut libc::c_char; 43] =
    [b"\xe3\x82\x92\xe9\x99\xa4\xe3\x81\x8f\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xae\xe3\x81\x9e\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x80\x9a\xe3\x81\x98\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x98\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x80\x9a\xe3\x81\x9a\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x98\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x80\x9a\xe3\x81\x99\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x98\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe5\x90\xab\xe3\x82\x81\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xb5\xe3\x81\x8f\xe3\x82\x81\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe5\xa7\x8b\xe3\x82\x81\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xaf\xe3\x81\x98\xe3\x82\x81\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe7\xb5\xa1\xe3\x82\x80\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x89\xe3\x82\x80\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe6\xb2\xbf\xe3\x81\x86\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x9d\xe3\x81\x86\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x90\x91\xe3\x81\x91\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x80\xe3\x81\x91\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xbc\xb4\xe3\x81\x86\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa8\xe3\x82\x82\xe3\x81\xaa\xe3\x81\x86\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x9f\xba\xe3\x81\xa5\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x82\xe3\x81\xa8\xe3\x81\xa5\xe3\x81\x8f\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\xaf\xbe\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x9f\xe3\x81\x84\xe3\x81\x99\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x96\xa2\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x93\xe3\x81\x99\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xbb\xa3\xe3\x82\x8f\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x8f\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x8a\xa0\xe3\x81\x88\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x8f\xe3\x81\x88\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x99\x90\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe7\xb6\x9a\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\xa5\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x90\x88\xe3\x82\x8f\xe3\x81\x9b\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8f\xe3\x81\x9b\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe6\xaf\x94\xe3\x81\xb9\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x89\xe3\x81\xb9\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xb8\xa6\xe3\x81\xb6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xaa\xe3\x82\x89\xe3\x81\xb6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x99\x90\xe3\x82\x8b\xe3\x81\xac\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\xe3\x81\xac\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut FukugojiCanonicalTable: [*mut libc::c_char; 55] =
    [b"\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8b\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x88\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x82\x81\xe3\x81\x90\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe5\xb7\xa1\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xae\xe3\x81\x9e\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x99\xa4\xe3\x81\x84\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x98\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x80\x9a\xe3\x81\x98\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x9a\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe9\x80\x9a\xe3\x81\x98\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xb5\xe3\x81\x8f\xe3\x82\x81\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe5\x90\xab\xe3\x82\x81\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe3\x81\xaf\xe3\x81\x98\xe3\x82\x81\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x92\xe5\xa7\x8b\xe3\x82\x81\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x89\xe3\x82\x80\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe7\xb5\xa1\xe3\x82\x93\xe3\x81\xa7\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x9d\xe3\x81\x86\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe6\xb2\xbf\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x80\xe3\x81\x91\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x90\x91\xe3\x81\x91\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa8\xe3\x82\x82\xe3\x81\xaa\xe3\x81\x86\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xbc\xb4\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x82\xe3\x81\xa8\xe3\x81\xa5\xe3\x81\x8f\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x9f\xba\xe3\x81\xa5\xe3\x81\x84\xe3\x81\xa6\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x9f\xe3\x81\x84\xe3\x81\x99\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\xaf\xbe\xe3\x81\x97\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x93\xe3\x81\x99\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x96\xa2\xe3\x81\x97\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x8f\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xbb\xa3\xe3\x82\x8f\xe3\x81\xa3\xe3\x81\xa6\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8a\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8a\xe3\x81\x84\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\x84\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa8\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa8\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x8f\xe3\x81\x88\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x8a\xa0\xe3\x81\x88\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x99\x90\xe3\x81\xa3\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\xa5\xe3\x81\x8f\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe7\xb6\x9a\xe3\x81\x84\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8f\xe3\x81\x9b\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe5\x90\x88\xe3\x82\x8f\xe3\x81\x9b\xe3\x81\xa6\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x89\xe3\x81\xb9\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe6\xaf\x94\xe3\x81\xb9\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\xaa\xe3\x82\x89\xe3\x81\xb6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe4\xb8\xa6\xe3\x82\x93\xe3\x81\xa7\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xa8\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xa8\xe3\x81\x97\xe3\x81\xa6\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8b\xe3\x81\xac\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x82\x88\xe3\x82\x89\xe3\x81\x9a\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\xe3\x81\xac\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xab\xe9\x99\x90\xe3\x82\x89\xe3\x81\x9a\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
/*====================================================================

			  格構造解析: 入力側

                                               S.Kurohashi 93. 5.31

    $Id$
====================================================================*/
#[no_mangle]
pub static mut fukugoji_string: [libc::c_char; 128] = [0; 128];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn extract_fukugoji_string(mut b_ptr: *mut BNST_DATA,
                                                 mut pre_b_ptr:
                                                 *mut BNST_DATA)
                                                 -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* 付属語がないとき */
    if (*b_ptr).num < 1 as libc::c_int ||
        (*pre_b_ptr).mrph_num == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    fukugoji_string[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* 前の文節の助詞 */
    strcat(fukugoji_string.as_mut_ptr(),
           (*(*pre_b_ptr).mrph_ptr.offset((*pre_b_ptr).mrph_num as
               isize).offset(-(1 as libc::c_int
               as
               isize))).Goi.as_mut_ptr());
    /* この文節 */
    i = 0 as libc::c_int;
    while i < (*b_ptr).mrph_num {
        if !check_feature((*(*b_ptr).mrph_ptr.offset(i as isize)).f,
                          b"\xe8\x87\xaa\xe7\xab\x8b\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            strcmp(Class[(*(*b_ptr).mrph_ptr.offset(i as isize)).Hinshi as
                usize][0 as libc::c_int as usize].id as
                       *const libc::c_char,
                   b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                       *const libc::c_char) != 0 {
            strcat(fukugoji_string.as_mut_ptr(),
                   (*(*b_ptr).mrph_ptr.offset(i as isize)).Goi.as_mut_ptr());
            if (i + 1 as libc::c_int) < (*b_ptr).mrph_num &&
                strcmp((*(*b_ptr).mrph_ptr.offset(i as
                    isize).offset(1 as
                    libc::c_int
                    as
                    isize)).Goi.as_mut_ptr(),
                       b"\xe3\x81\xac\x00" as *const u8 as
                           *const libc::c_char) == 0 &&
                strcmp(Class[(*(*b_ptr).mrph_ptr.offset(i as
                    isize).offset(1
                    as
                    libc::c_int
                    as
                    isize)).Hinshi
                    as usize][0 as libc::c_int as usize].id as
                           *const libc::c_char,
                       b"\xe5\x8a\xa9\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                           *const u8 as *const libc::c_char) == 0 {
                strcat(fukugoji_string.as_mut_ptr(),
                       (*(*b_ptr).mrph_ptr.offset(i as
                           isize).offset(1 as
                           libc::c_int
                           as
                           isize)).Goi.as_mut_ptr());
            }
            break;
        } else { i += 1 }
    }
    /* 原形の読みに統一 */
    i = 0 as libc::c_int;
    while *FukugojiYomiTable[i as usize] != 0 {
        if strcmp(fukugoji_string.as_mut_ptr(), FukugojiYomiTable[i as usize])
            == 0 {
            strcpy(fukugoji_string.as_mut_ptr(),
                   FukugojiYomiTable[(i + 1 as libc::c_int) as usize]);
            break;
        } else { i += 2 as libc::c_int }
    }
    return fukugoji_string.as_mut_ptr();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn canonicalize_fukugoji(mut str: *mut libc::c_char)
                                               -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* 複合辞のいわば代表表記を返す */
    i = 0 as libc::c_int;
    while *FukugojiCanonicalTable[i as usize] != 0 {
        if strcmp(str, FukugojiCanonicalTable[i as usize]) == 0 {
            return FukugojiCanonicalTable[(i + 1 as libc::c_int) as usize];
        }
        i += 2 as libc::c_int
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_fukugoji_id(mut b_ptr: *mut BNST_DATA)
                                          -> *mut libc::c_char
/*==================================================================*/
{
    // let mut fc: libc::c_int = 0;
    let mut fukugoji_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canonical_str: *mut libc::c_char = 0 as *mut libc::c_char;
    fukugoji_str =
        extract_fukugoji_string(b_ptr,
                                b_ptr.offset(-(1 as libc::c_int as isize)));
    return if !fukugoji_str.is_null() &&
        {
            canonical_str = canonicalize_fukugoji(fukugoji_str);
            !canonical_str.is_null()
        } {
        sprintf(fukugoji_string.as_mut_ptr(),
                b"ID:\xe3\x80\x9c%s\x00" as *const u8 as *const libc::c_char,
                canonical_str);
        fukugoji_string.as_mut_ptr()
    } else { 0 as *mut libc::c_char };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_fukugoji_case_string(mut b_ptr: *mut TAG_DATA)
                                                   -> *mut libc::c_char
/*==================================================================*/
{
    let mut fc: libc::c_int = 0;
    let mut fukugoji_str: *mut libc::c_char = 0 as *mut libc::c_char;
    fukugoji_str =
        extract_fukugoji_string(b_ptr as *mut BNST_DATA,
                                b_ptr.offset(-(1 as libc::c_int as isize)) as
                                    *mut BNST_DATA);
    return if !fukugoji_str.is_null() &&
        {
            fc = pp_hstr_to_code(fukugoji_str);
            (fc) != -(10 as libc::c_int)
        } {
        sprintf(fukugoji_string.as_mut_ptr(),
                b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-%s\x00" as
                    *const u8 as *const libc::c_char, pp_code_to_kstr(fc));
        fukugoji_string.as_mut_ptr()
    } else { 0 as *mut libc::c_char };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_cc_relation(mut cpm_ptr: *mut CF_PRED_MGR,
                                           mut b_ptr: *mut TAG_DATA,
                                           mut pp_str: *mut libc::c_char)
                                           -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if cpm_ptr.is_null() ||
        (*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if !(*cpm_ptr).elem_b_ptr[i as usize].is_null() &&
            (*(*cpm_ptr).elem_b_ptr[i as usize]).num == (*b_ptr).num &&
            MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).pp[(*cpm_ptr).cmm[0
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
                usize][0 as
                libc::c_int
                as
                usize],
                    pp_str) != 0 {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_data_from_feature_to_pp(mut cpm_ptr:
                                                       *mut CF_PRED_MGR,
                                                       mut b_ptr:
                                                       *mut TAG_DATA,
                                                       mut pp_num:
                                                       *mut libc::c_int,
                                                       mut fcp:
                                                       *mut libc::c_char)
                                                       -> libc::c_int
/*==================================================================*/
{
    let mut c_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    let mut cc: libc::c_int = 0;
    /* 用言の項となるもの */
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
        if strncmp(fcp,
                   b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\x00" as
                       *const u8 as *const libc::c_char,
                   strlen(b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\x00"
                       as *const u8 as *const libc::c_char)) == 0 {
            if OptAnaphora == 0 &&
                strcmp(fcp,
                       b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\xef\xbc\x8a\x00"
                           as *const u8 as *const libc::c_char) == 0 {
                /* 省略解析用: 割り当てなしを考える印 (格解析では無視) */
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            cc =
                pp_kstr_to_code(fcp.offset(strlen(b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\x00"
                    as *const u8 as
                    *const libc::c_char) as
                    isize));
            if cc == -(10 as libc::c_int) {
                fprintf(stderr,
                        b";; case <%s> in a rule is unknown!\n\x00" as
                            *const u8 as *const libc::c_char,
                        fcp.offset(strlen(b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\x00"
                            as *const u8 as
                            *const libc::c_char) as isize));
                exit(1 as libc::c_int);
            }
            let fresh0 = *pp_num;
            *pp_num = *pp_num + 1;
            (*c_ptr).pp[(*c_ptr).element_num as usize][fresh0 as usize] = cc;
            if *pp_num >= 10 as libc::c_int {
                fprintf(stderr,
                        b";; not enough pp_num (%d)!\n\x00" as *const u8 as
                            *const libc::c_char, 10 as libc::c_int);
                exit(1 as libc::c_int);
            }
        } else if strcmp(fcp,
                         b"\xef\xbc\xb4\xe5\xbf\x85\xe9\xa0\x88\xe6\xa0\xbc\x00"
                             as *const u8 as *const libc::c_char) == 0 {
            (*c_ptr).oblig[(*c_ptr).element_num as usize] =
                (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(fcp,
                         b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                             as *const u8 as *const libc::c_char) == 0 {
            /* 「〜を〜に」のとき */
            if (*(*cpm_ptr).pred_b_ptr).num != (*b_ptr).num {
                return 0 as libc::c_int;
            }
        }
    } else if strcmp(fcp,
                     b"\xef\xbc\xb4\xe5\x90\x8d\xe8\xa9\x9e\xe9\xa0\x85\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        /* 名詞の項となるもの */
        /* 条件: 同格ではない 
	             連体修飾節の場合はその関係が外の関係 */
        if (*b_ptr).dpnd_type as libc::c_int != 'A' as i32 &&
            (check_feature((*b_ptr).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
                check_cc_relation((*b_ptr).cpm_ptr, (*cpm_ptr).pred_b_ptr,
                                  b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char) != 0) {
            let fresh1 = *pp_num;
            *pp_num = *pp_num + 1;
            (*c_ptr).pp[(*c_ptr).element_num as usize][fresh1 as usize] =
                0 as libc::c_int;
            (*c_ptr).pp_str[(*c_ptr).element_num as usize] =
                0 as *mut libc::c_char
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_data_cframe_pp(mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut b_ptr: *mut TAG_DATA,
                                              mut flag: libc::c_int)
                                              -> *mut TAG_DATA
/*==================================================================*/
{
    let mut pp_num: libc::c_int = 0 as libc::c_int;
    let mut cc: libc::c_int = 0;
    let mut not_flag: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    /* flag == TRUE:  格要素
       flag == FALSE: 被連体修飾詞 */
    /* 格要素 */
    if flag == (0 as libc::c_int == 0) as libc::c_int {
        if (*b_ptr).num > 0 as libc::c_int &&
            !check_feature((*b_ptr).f,
                           b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\xe8\xa1\xa8\xe8\xa8\x98\xe7\x9b\xb4\xe5\x89\x8d\xe5\x8f\x82\xe7\x85\xa7\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            b_ptr = b_ptr.offset(-1)
        }
        /* 「〜のNだ。」禁止 (★=>ルールへ) */
        if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
            !check_feature((*b_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                           b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            return 0 as *mut TAG_DATA;
        }
        (*c_ptr).oblig[(*c_ptr).element_num as usize] = 0 as libc::c_int;
        /* 係り先をみる場合 */
        start_cp =
            check_feature((*b_ptr).f,
                          b"\xe4\xbf\x82\xe3\x83\x81\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char); /* 2: OK, 1: 未定, 0: NG */
        if !start_cp.is_null() {
            buffer =
                strdup(start_cp.offset(strlen(b"\xe4\xbf\x82\xe3\x83\x81:\x00"
                    as *const u8 as
                    *const libc::c_char) as
                    isize));
            start_cp = buffer;
            loop_cp = start_cp;
            flag = 1 as libc::c_int;
            not_flag = 0 as libc::c_int;
            while *loop_cp != 0 {
                if flag == 1 as libc::c_int &&
                    *loop_cp as libc::c_int == '&' as i32 &&
                    *loop_cp.offset(1 as libc::c_int as isize) as
                        libc::c_int == '&' as i32 {
                    *loop_cp = '\u{0}' as i32 as libc::c_char;
                    if not_flag == 0 &&
                        check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                      start_cp).is_null() ||
                        not_flag != 0 &&
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           start_cp).is_null() {
                        flag = 0 as libc::c_int
                        /* NG */
                    }
                    loop_cp = loop_cp.offset(2 as libc::c_int as isize);
                    start_cp = loop_cp;
                    not_flag = 0 as libc::c_int
                } else if flag < 2 as libc::c_int &&
                    *loop_cp as libc::c_int == '|' as i32 &&
                    *loop_cp.offset(1 as libc::c_int as isize) as
                        libc::c_int == '|' as i32 {
                    if flag == 1 as libc::c_int {
                        *loop_cp = '\u{0}' as i32 as libc::c_char;
                        if not_flag == 0 &&
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           start_cp).is_null() ||
                            not_flag != 0 &&
                                check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                              start_cp).is_null() {
                            flag = 2 as libc::c_int
                            /* OK */
                        }
                    } else {
                        flag = 1 as libc::c_int
                        /* 0 -> 1 */
                    }
                    loop_cp = loop_cp.offset(2 as libc::c_int as isize);
                    start_cp = loop_cp;
                    not_flag = 0 as libc::c_int
                } else if *loop_cp as libc::c_int == ':' as i32 {
                    *loop_cp = '\u{0}' as i32 as libc::c_char;
                    if flag == 2 as libc::c_int ||
                        (flag == 1 as libc::c_int &&
                            (not_flag == 0 &&
                                !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                               start_cp).is_null()) ||
                            not_flag != 0 &&
                                check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                              start_cp).is_null()) {
                        if _make_data_from_feature_to_pp(cpm_ptr, b_ptr,
                                                         &mut pp_num,
                                                         loop_cp.offset(1 as
                                                             libc::c_int
                                                             as
                                                             isize))
                            == 0 as libc::c_int {
                            free(buffer as *mut libc::c_void);
                            return 0 as *mut TAG_DATA;
                        }
                    }
                    break;
                } else {
                    if *loop_cp as libc::c_int == '^' as i32 {
                        not_flag = 1 as libc::c_int
                    }
                    loop_cp = loop_cp.offset(1)
                }
            }
            free(buffer as *mut libc::c_void);
        }
        if !check_feature((*b_ptr).f,
                          b"\xef\xbc\xb4\xe6\xa0\xbc\xe7\x9b\xb4\xe5\xbe\x8c\xe5\x8f\x82\xe7\x85\xa7\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            /* 「〜の(方)」などの格は「方」の方の格をみる */
            fp = (*b_ptr.offset(1 as libc::c_int as isize)).f
        } else { fp = (*b_ptr).f }
        /* featureから格へ */
        while !fp.is_null() {
            if _make_data_from_feature_to_pp(cpm_ptr, b_ptr, &mut pp_num,
                                             (*fp).cp) == 0 as libc::c_int {
                return 0 as *mut TAG_DATA;
            }
            fp = (*fp).next
        }
    } else if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
        fp = (*b_ptr).f;
        (*c_ptr).oblig[(*c_ptr).element_num as usize] = 0 as libc::c_int;
        while !fp.is_null() {
            if strncmp((*fp).cp,
                       b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe9\x80\xa3\xe6\xa0\xbc-\x00"
                           as *const u8 as *const libc::c_char,
                       strlen(b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe9\x80\xa3\xe6\xa0\xbc-\x00"
                           as *const u8 as *const libc::c_char)) == 0 {
                cc =
                    pp_kstr_to_code((*fp).cp.offset(strlen(b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe9\x80\xa3\xe6\xa0\xbc-\x00"
                        as *const u8 as
                        *const libc::c_char)
                        as isize));
                if cc == -(10 as libc::c_int) {
                    fprintf(stderr,
                            b";; case <%s> in a rule is unknown!\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*fp).cp.offset(11 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
                let fresh2 = pp_num;
                pp_num = pp_num + 1;
                (*c_ptr).pp[(*c_ptr).element_num as usize][fresh2 as usize] =
                    cc;
                if pp_num >= 10 as libc::c_int {
                    fprintf(stderr,
                            b";; not enough pp_num (%d)!\n\x00" as *const u8
                                as *const libc::c_char, 10 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            fp = (*fp).next
        }
    }
    return if pp_num != 0 {
        (*c_ptr).pp[(*c_ptr).element_num as usize][pp_num as usize] =
            -(10 as libc::c_int);
        b_ptr
    } else { 0 as *mut TAG_DATA };
}
/* 被連体修飾詞 (とりあえず用言のときのみ) */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_data_cframe_sm(mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut b_ptr: *mut TAG_DATA)
/*==================================================================*/
{
    let mut sm_num: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0;
    let mut c_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    if Thesaurus == 2 as libc::c_int {
        size = 12 as libc::c_int
    } else if Thesaurus == 1 as libc::c_int { size = 11 as libc::c_int }
    /* 格要素 -- 文 */
    if !check_feature((*b_ptr).f,
                      b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        strcpy((*c_ptr).sm[(*c_ptr).element_num as
            usize].offset((size * sm_num) as isize),
               sm2code(b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char));
        sm_num += 1
    } else {
        /* 修飾 *
    else if (check_feature(b_ptr->f, "修飾")) {
	strcpy(c_ptr->sm[c_ptr->element_num]+size*sm_num, 
	       sm2code("修飾"));
	sm_num++;
	} */
        if !check_feature((*b_ptr).f,
                          b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            strcpy((*c_ptr).sm[(*c_ptr).element_num as
                usize].offset((size * sm_num) as isize),
                   sm2code(b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char));
            sm_num += 1
        }
        if !check_feature((*b_ptr).f,
                          b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            strcpy(
                (*c_ptr).sm[(*c_ptr).element_num as usize].offset((size * sm_num) as isize),
                sm2code(b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
            );
            sm_num += 1
        }
        /* 固有名詞 => 主体 */
        if !check_feature((*b_ptr).f,
                          b"\xe4\xba\xba\xe5\x90\x8d\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            !check_feature((*b_ptr).f,
                           b"\xe7\xb5\x84\xe7\xb9\x94\xe5\x90\x8d\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            strcpy(
                (*c_ptr).sm[(*c_ptr).element_num as usize].offset((size * sm_num) as isize),
                sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
            );
            sm_num += 1
        }
        /* 主体 */
        if Thesaurus == 2 as libc::c_int {
            /* いろいろ使えるので意味素すべてコピー */
            strcpy((*c_ptr).sm[(*c_ptr).element_num as
                usize].offset((size * sm_num) as isize),
                   (*b_ptr).SM_code.as_mut_ptr());
            sm_num =
                (sm_num as
                    libc::c_ulong).wrapping_add(strlen((*b_ptr).SM_code.as_mut_ptr()).wrapping_div(size
                    as
                    libc::c_ulong))
                    as libc::c_int as libc::c_int
        } else if Thesaurus == 1 as libc::c_int {
            if bgh_match_check(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                *const u8 as *const libc::c_char as
                *mut libc::c_char),
                               (*b_ptr).BGH_code.as_mut_ptr()) != 0 {
                strcpy((*c_ptr).sm[(*c_ptr).element_num as
                    usize].offset((size * sm_num) as
                    isize),
                       sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char));
                sm_num += 1
            }
        }
    }
    *(*c_ptr).sm[(*c_ptr).element_num as
        usize].offset((size * sm_num) as isize) =
        '\u{0}' as i32 as libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_data_cframe_ex(mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut b_ptr: *mut TAG_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut c_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 2 as libc::c_int;
    if Thesaurus == 1 as libc::c_int {
        strcpy((*c_ptr).ex[(*c_ptr).element_num as usize],
               (*b_ptr).BGH_code.as_mut_ptr());
    } else if Thesaurus == 2 as libc::c_int {
        strcpy((*c_ptr).ex[(*c_ptr).element_num as usize],
               (*b_ptr).SM_code.as_mut_ptr());
    }
    if OptCaseFlag & 32 as libc::c_int != 0 &&
        {
            cp = get_mrph_rep_from_f((*b_ptr).head_ptr, 0 as libc::c_int);
            !cp.is_null()
        } {
        strcpy(*(*c_ptr).ex_list[(*c_ptr).element_num as
            usize].offset(0 as libc::c_int as isize),
               cp);
    } else {
        strcpy(*(*c_ptr).ex_list[(*c_ptr).element_num as
            usize].offset(0 as libc::c_int as isize),
               (*(*b_ptr).head_ptr).Goi.as_mut_ptr());
    }
    (*c_ptr).ex_num[(*c_ptr).element_num as usize] = 1 as libc::c_int;
    *(*c_ptr).ex_freq[(*c_ptr).element_num as
        usize].offset(0 as libc::c_int as isize) =
        1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_data_cf_type(mut cpm_ptr: *mut CF_PRED_MGR)
/*==================================================================*/
{
    let mut b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    (*cpm_ptr).cf.type_0 = 1 as libc::c_int;
    (*cpm_ptr).cf.type_flag = 0 as libc::c_int;
    (*cpm_ptr).cf.voice = (*b_ptr).voice;
    vtype =
        check_feature((*b_ptr).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
    if !vtype.is_null() {
        vtype =
            vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as *const u8
                as *const libc::c_char) as isize);
        strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(), vtype);
    } else {
        vtype =
            check_feature((*b_ptr).f,
                          b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !vtype.is_null() {
            vtype =
                vtype.offset(strlen(b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90:\x00"
                    as *const u8 as *const libc::c_char)
                    as isize);
            strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(), vtype);
        } else if !check_feature((*b_ptr).f,
                                 b"\xe6\xba\x96\xe7\x94\xa8\xe8\xa8\x80\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(),
                   b"\xe6\xba\x96\x00" as *const u8 as *const libc::c_char);
        } else if !check_feature((*b_ptr).f,
                                 b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(),
                   b"\xe5\x90\x8d\x00" as *const u8 as *const libc::c_char);
            (*cpm_ptr).cf.type_0 = 2 as libc::c_int
        } else {
            (*cpm_ptr).cf.pred_type[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        }
    }
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
        (!check_feature((*b_ptr).f,
                        b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8 as
                            *const libc::c_char as
                            *mut libc::c_char).is_null() ||
            !check_feature((*b_ptr).f,
                           b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null()) {
        (*cpm_ptr).cf.type_flag = 1 as libc::c_int
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_data_cframe_child(mut sp: *mut SENTENCE_DATA,
                                                mut cpm_ptr: *mut CF_PRED_MGR,
                                                mut child_ptr: *mut TAG_DATA,
                                                mut child_num: libc::c_int,
                                                mut closest_flag: libc::c_int)
                                                -> libc::c_int
/*==================================================================*/
{
    let mut cel_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    cel_b_ptr =
        _make_data_cframe_pp(cpm_ptr, child_ptr,
                             (0 as libc::c_int == 0) as libc::c_int);
    if !cel_b_ptr.is_null() {
        /* 「みかん三個を食べる」 ひとつ前の名詞を格要素とするとき
	   「みかんを三個食べる」 の場合はそのまま両方格要素になる
	*/
        if !check_feature((*cel_b_ptr).f,
                          b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (!check_feature((*cel_b_ptr).f,
                            b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*cel_b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) &&
            (*cel_b_ptr).num > 0 as libc::c_int &&
            (!check_feature((*(*sp).tag_data.offset((*cel_b_ptr).num as isize).offset(-(1 as libc::c_int as isize))).f,
                            b"\xe4\xbf\x82:\xe9\x9a\xa3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).tag_data.offset((*cel_b_ptr).num as isize).offset(-(1 as libc::c_int as isize))).f,
                               b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe6\x9c\xaa\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null()) &&
            check_feature((*(*sp).tag_data.offset((*cel_b_ptr).num as isize).offset(-(1 as libc::c_int as isize))).f,
                          b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            check_feature((*(*sp).tag_data.offset((*cel_b_ptr).num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).f,
                          b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            _make_data_cframe_sm(cpm_ptr,
                                 (*sp).tag_data.offset((*cel_b_ptr).num as
                                     isize).offset(-(1
                                     as
                                     libc::c_int
                                     as
                                     isize)));
            _make_data_cframe_ex(cpm_ptr,
                                 (*sp).tag_data.offset((*cel_b_ptr).num as
                                     isize).offset(-(1
                                     as
                                     libc::c_int
                                     as
                                     isize)));
            (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] =
                (*sp).tag_data.offset((*cel_b_ptr).num as
                    isize).offset(-(1 as libc::c_int as
                    isize));
            (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                0 as libc::c_int
        } else {
            /* 直前格のマーク (厳しい版: 完全に直前のみ) */
            if closest_flag != 0 {
                (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            } else {
                (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                    0 as libc::c_int
            } /* 並列要素格納用 */
            _make_data_cframe_sm(cpm_ptr, cel_b_ptr);
            _make_data_cframe_ex(cpm_ptr, cel_b_ptr);
            (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] =
                cel_b_ptr
        }
        (*(*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize]).next =
            0 as *mut tnode_t;
        (*cpm_ptr).para_b_ptr[(*cpm_ptr).cf.element_num as usize] =
            0 as *mut TAG_DATA;
        /* 格が明示されていないことをマーク */
        if !check_feature((*cel_b_ptr).f,
                          b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            !check_feature((*cel_b_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            (*cel_b_ptr).inum > 0 as libc::c_int {
            (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                -(1 as libc::c_int)
        } else {
            (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                child_num
        }
        (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
            0 as libc::c_int;
        (*cpm_ptr).cf.element_num += 1;
        if (*cpm_ptr).cf.element_num >= 24 as libc::c_int {
            (*cpm_ptr).cf.element_num = 0 as libc::c_int
        }
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_data_cframe_rentai(mut sp: *mut SENTENCE_DATA,
                                                 mut cpm_ptr:
                                                 *mut CF_PRED_MGR)
                                                 -> libc::c_int
/*==================================================================*/
{
    let mut b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    let mut cel_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut renkaku_exception_p: libc::c_int = 0 as libc::c_int;
    if !check_feature((*b_ptr).f,
                      b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\xe6\x8c\x87\xe5\xae\x9a:2\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        renkaku_exception_p = 1 as libc::c_int
    }
    /* 被連体修飾詞 */
    if !check_feature((*b_ptr).f,
                      b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        ((*b_ptr).para_type as libc::c_int != 1 as libc::c_int ||
            (*b_ptr).num == (*(*b_ptr).parent).num) ||
        (*b_ptr).para_type as libc::c_int == 1 as libc::c_int &&
            !check_feature((*b_ptr).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            (*(*b_ptr).parent).para_top_p as libc::c_int != 0 &&
            !check_feature((*(*(*b_ptr).parent).child[0 as libc::c_int as
                usize]).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
        renkaku_exception_p != 0 {
        /* para_type == PARA_NORMAL は「Vし,Vした PARA N」のとき
	   このときは親(PARA)の親(N)を格要素とする．

	   親がpara_top_pかどうかをみても「VしたNとN PARA」の
	   時と区別ができない
        */
        /* 用言が並列ではないとき */
        if (*b_ptr).para_type as libc::c_int != 1 as libc::c_int {
            if !(*b_ptr).parent.is_null() {
                /* 〜のは */
                if renkaku_exception_p != 0 &&
                    !(*(*b_ptr).parent).parent.is_null() {
                    if !check_feature((*(*(*b_ptr).parent).parent).f,
                                      b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                        cel_b_ptr = (*(*b_ptr).parent).parent;
                        _make_data_cframe_pp(cpm_ptr, b_ptr,
                                             0 as libc::c_int);
                    }
                } else {
                    cel_b_ptr = (*b_ptr).parent;
                    _make_data_cframe_pp(cpm_ptr, b_ptr, 0 as libc::c_int);
                }
                if !cel_b_ptr.is_null() {
                    _make_data_cframe_sm(cpm_ptr, cel_b_ptr);
                    _make_data_cframe_ex(cpm_ptr, cel_b_ptr);
                    (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize]
                        = cel_b_ptr;
                    (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize]
                        = -(1 as libc::c_int);
                    (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                        0 as libc::c_int;
                    (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize]
                        = 0 as libc::c_int;
                    (*cpm_ptr).cf.element_num += 1
                }
            }
        } else {
            /* 用言が並列のとき */
            cel_b_ptr = b_ptr; /* ★不正確★ */
            while (*(*cel_b_ptr).parent).para_type as libc::c_int ==
                1 as libc::c_int {
                cel_b_ptr = (*cel_b_ptr).parent
            }
            if !(*cel_b_ptr).parent.is_null() &&
                !(*(*cel_b_ptr).parent).parent.is_null() {
                _make_data_cframe_pp(cpm_ptr, (*cel_b_ptr).parent,
                                     0 as libc::c_int);
                _make_data_cframe_sm(cpm_ptr, (*(*cel_b_ptr).parent).parent);
                _make_data_cframe_ex(cpm_ptr, (*(*cel_b_ptr).parent).parent);
                (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] =
                    (*(*cel_b_ptr).parent).parent;
                (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                    -(1 as libc::c_int);
                (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                    2 as libc::c_int;
                (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                    0 as libc::c_int;
                (*cpm_ptr).cf.element_num += 1
            }
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_data_cframe(mut sp: *mut SENTENCE_DATA,
                                          mut cpm_ptr: *mut CF_PRED_MGR)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    let mut cel_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut i: libc::c_int = 0;
    let mut child_num: libc::c_int = 0;
    // let mut first: libc::c_int = 0;
    let mut closest: libc::c_int = 0;
    let mut orig_child_num: libc::c_int = -(1 as libc::c_int);
    // let mut renkaku_exception_p: libc::c_int = 0;
    (*cpm_ptr).cf.samecase[0 as libc::c_int as
        usize][0 as libc::c_int as usize] =
        -(10 as libc::c_int);
    (*cpm_ptr).cf.samecase[0 as libc::c_int as
        usize][1 as libc::c_int as usize] =
        -(10 as libc::c_int);
    (*cpm_ptr).cf.pred_b_ptr = b_ptr;
    (*b_ptr).cpm_ptr = cpm_ptr;
    /* 表層格 etc. の設定 */
    (*cpm_ptr).cf.element_num = 0 as libc::c_int;
    /* 連体修飾 */
    make_data_cframe_rentai(sp, cpm_ptr);
    child_num = 0 as libc::c_int;
    while !(*b_ptr).child[child_num as usize].is_null() { child_num += 1 }
    /* 自分(用言)が複合名詞内 */
    if (*b_ptr).inum > 0 as libc::c_int {
        // let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
        /* 文節のheadに係る名詞の取り扱い *

	t_ptr = b_ptr->parent;
	while (1) {
	    if (t_ptr->cpm_ptr) { * 別の格解析対象 *
		t_ptr = NULL;
		break;
	    }
	    if (t_ptr->inum == 0) {
		break;
	    }
	    t_ptr = t_ptr->parent;
	}

	* ... n3 n2 n1
	   複合名詞内の(うしろからみて)最初の用言(格解析対象)に対して
	   n1の子供をとってくる *
	if (t_ptr) {
	    orig_child_num = child_num;
	    for (i = 0; t_ptr->child[i]; i++) {
		* 文節内部以外 *
		if (t_ptr->child[i]->inum == 0) {
		    b_ptr->child[child_num++] = t_ptr->child[i];
		}
	    }
	}
	*/
        if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
            check_feature((*b_ptr).f,
                          b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if check_feature((*(*b_ptr).parent).f,
                             b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() ||
                !check_feature((*b_ptr).f,
                               b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                _make_data_cframe_pp(cpm_ptr, b_ptr, 0 as libc::c_int);
            } else {
                (*cpm_ptr).cf.pp[(*cpm_ptr).cf.element_num as
                    usize][0 as libc::c_int as usize] =
                    pp_hstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
                (*cpm_ptr).cf.pp[(*cpm_ptr).cf.element_num as
                    usize][1 as libc::c_int as usize] =
                    -(10 as libc::c_int);
                (*cpm_ptr).cf.oblig[(*cpm_ptr).cf.element_num as usize] =
                    0 as libc::c_int
            }
            _make_data_cframe_sm(cpm_ptr, (*b_ptr).parent);
            _make_data_cframe_ex(cpm_ptr, (*b_ptr).parent);
            (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] =
                (*b_ptr).parent;
            (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                -(1 as libc::c_int);
            (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                0 as libc::c_int;
            (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                0 as libc::c_int;
            (*cpm_ptr).cf.element_num += 1
        }
    }
    /* 自分(用言)が複合名詞内のときの親 : 被連体修飾詞扱い
	   ※ 連格のとき(「〜したのは」)はすでに扱っている */
    /* 子供を格要素に */
    i = child_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if make_data_cframe_child(sp, cpm_ptr, (*b_ptr).child[i as usize], i,
                                  if i == 0 as libc::c_int &&
                                      (*b_ptr).num ==
                                          (*(*b_ptr).child[i as usize]).num
                                              + 1 as libc::c_int &&
                                      check_feature((*b_ptr).f,
                                                    b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                                                        as *const u8 as
                                                        *const libc::c_char
                                                        as
                                                        *mut libc::c_char).is_null()
                                  {
                                      (0 as libc::c_int == 0) as libc::c_int
                                  } else { 0 as libc::c_int }) != 0 {
            if (*cpm_ptr).cf.element_num == 0 as libc::c_int {
                /* 子供が作れるはずなのに、作れなかった */
                return -(1 as libc::c_int);
            }
        }
        i -= 1
    }
    /* 複合名詞: 子供をもとにもどす */
    if orig_child_num >= 0 as libc::c_int {
        (*b_ptr).child[orig_child_num as usize] = 0 as *mut tnode_t
    }
    /* 用言文節が「（〜を）〜に」のとき 
       「する」の格フレームに対してニ格(同文節)を設定
       ヲ格は子供の処理で扱われる */
    if !check_feature((*b_ptr).f,
                      b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        if !_make_data_cframe_pp(cpm_ptr, b_ptr,
                                 (0 as libc::c_int == 0) as
                                     libc::c_int).is_null() {
            _make_data_cframe_sm(cpm_ptr, b_ptr);
            _make_data_cframe_ex(cpm_ptr, b_ptr);
            (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] = b_ptr;
            (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                child_num;
            (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                0 as libc::c_int;
            (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                (0 as libc::c_int == 0) as libc::c_int;
            (*cpm_ptr).cf.element_num += 1
        }
    }
    /* 用言が並列のとき、格要素を expand する */
    if (*b_ptr).para_type as libc::c_int == 1 as libc::c_int &&
        !(*b_ptr).parent.is_null() &&
        (*(*b_ptr).parent).para_top_p as libc::c_int != 0 {
        child_num = 0 as libc::c_int;
        /* <PARA>に係る子供をチェック */
        i = 0 as libc::c_int;
        while !(*(*b_ptr).parent).child[i as usize].is_null() {
            if (*(*(*b_ptr).parent).child[i as usize]).para_type as
                libc::c_int == 1 as libc::c_int {
                child_num += 1
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while !(*(*b_ptr).parent).child[i as usize].is_null() {
            if (*(*(*b_ptr).parent).child[i as usize]).para_type as
                libc::c_int == 0 as libc::c_int &&
                (*(*(*b_ptr).parent).child[i as usize]).num < (*b_ptr).num
            {
                cel_b_ptr =
                    _make_data_cframe_pp(cpm_ptr,
                                         (*(*b_ptr).parent).child[i as usize],
                                         (0 as libc::c_int == 0) as
                                             libc::c_int);
                if !cel_b_ptr.is_null() {
                    _make_data_cframe_sm(cpm_ptr, cel_b_ptr);
                    _make_data_cframe_ex(cpm_ptr, cel_b_ptr);
                    (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize]
                        = cel_b_ptr;
                    /* 格が明示されていないことをマーク */
                    if !check_feature((*cel_b_ptr).f,
                                      b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() ||
                        !check_feature((*cel_b_ptr).f,
                                       b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null() {
                        (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as
                            usize] = -(1 as libc::c_int)
                    } else {
                        (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as
                            usize] = i
                    }
                    (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                        child_num;
                    (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize]
                        = 0 as libc::c_int;
                    (*cpm_ptr).cf.element_num += 1
                }
                if (*cpm_ptr).cf.element_num >= 24 as libc::c_int {
                    (*cpm_ptr).cf.element_num = 0 as libc::c_int;
                    return -(1 as libc::c_int);
                }
            }
            i += 1
        }
    }
    /* 直前格要素の取得 */
    closest = get_closest_case_component(sp, cpm_ptr);
    /* 直前格要素のひとつ手前のノ格
       ※ <数量>以外: 一五％の株式を V
          <時間>以外: */
    if OptCaseFlag & 4 as libc::c_int != 0 && closest > -(1 as libc::c_int) &&
        (*(*cpm_ptr).elem_b_ptr[closest as usize]).num > 0 as libc::c_int
        &&
        check_feature((*(*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[closest
            as
            usize]).num
            as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).f,
                      b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        check_feature((*(*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[closest
            as
            usize]).num
            as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).f,
                      b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        !check_feature((*(*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[closest
            as
            usize]).num
            as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).f,
                       b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        let mut bp: *mut TAG_DATA = 0 as *mut TAG_DATA;
        bp =
            (*sp).tag_data.offset((*(*cpm_ptr).elem_b_ptr[closest as
                usize]).num as
                isize).offset(-(1 as libc::c_int as
                isize));
        /* 割り当てる格は格フレームによって動的に変わる */
        (*cpm_ptr).cf.pp[(*cpm_ptr).cf.element_num as
            usize][0 as libc::c_int as usize] =
            pp_hstr_to_code(b"\xe6\x9c\xaa\x00" as *const u8 as
                *const libc::c_char as
                *mut libc::c_char); /* 表層格 */
        (*cpm_ptr).cf.pp[(*cpm_ptr).cf.element_num as
            usize][1 as libc::c_int as usize] =
            -(10 as libc::c_int); /* 以下は削除する予定 */
        (*cpm_ptr).cf.sp[(*cpm_ptr).cf.element_num as usize] =
            pp_hstr_to_code(b"\xe3\x81\xae\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char);
        (*cpm_ptr).cf.oblig[(*cpm_ptr).cf.element_num as usize] =
            0 as libc::c_int;
        _make_data_cframe_sm(cpm_ptr, bp);
        _make_data_cframe_ex(cpm_ptr, bp);
        (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] = bp;
        (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
            -(1 as libc::c_int);
        (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
            0 as libc::c_int;
        (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
            0 as libc::c_int;
        if (*cpm_ptr).cf.element_num < 24 as libc::c_int {
            (*cpm_ptr).cf.element_num += 1
        }
    }
    return closest;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_pred_voice(mut ptr: *mut BNST_DATA)
/*==================================================================*/
{
    /* ヴォイスの設定 */
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ptr).voice = 0 as libc::c_int;
    cp =
        check_feature((*ptr).f,
                      b"\xe6\x85\x8b\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char);
    if !cp.is_null() {
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        str =
            strdup(cp.offset(strlen(b"\xe6\x85\x8b:\x00" as *const u8 as
                *const libc::c_char) as isize));
        token = strtok(str, b"|\x00" as *const u8 as *const libc::c_char);
        while !token.is_null() {
            if strcmp(token,
                      b"\xe5\x8f\x97\xe5\x8b\x95\x00" as *const u8 as
                          *const libc::c_char) == 0 {
                (*ptr).voice |= 2 as libc::c_int
            } else if strcmp(token,
                             b"\xe4\xbd\xbf\xe5\xbd\xb9\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                (*ptr).voice |= 1 as libc::c_int
            } else if strcmp(token,
                             b"\xe3\x82\x82\xe3\x82\x89\xe3\x81\x86\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                (*ptr).voice |= 8 as libc::c_int
            } else if strcmp(token,
                             b"\xe3\x81\xbb\xe3\x81\x97\xe3\x81\x84\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                (*ptr).voice |= 16 as libc::c_int
            } else if strcmp(token,
                             b"\xe4\xbd\xbf\xe5\xbd\xb9&\xe5\x8f\x97\xe5\x8b\x95\x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                (*ptr).voice |= 4 as libc::c_int
            }
            /* 「可能」は未扱い */
            token =
                strtok(0 as *mut libc::c_char,
                       b"|\x00" as *const u8 as *const libc::c_char)
        }
        free(str as *mut libc::c_void);
    };
}
/*====================================================================
                               END
====================================================================*/
