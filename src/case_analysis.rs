#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types, ptr_wrapping_offset_from, register_tool)]
//! 格構造解析
use crate::{_FEATURE, BNST_DATA, case_data, CHECK_DATA, context, cpm_def, DPND, Dpnd_matrix, feature, FEATURE, Mask_matrix, MRPH_DATA, Quote_matrix, TAG_DATA, tnode_b, TOTAL_MGR, tree_conv};
use crate::bnst_compare::subordinate_level_check;
use crate::case_data::make_data_cframe;
use crate::case_ipal::{_make_ipal_cframe_pp, calc_adv_modifying_num_probability, calc_adv_modifying_probability, calc_vp_modifying_num_probability, calc_vp_modifying_probability, check_cf_case, init_mgr_cf, MAX_Case_frame_num};
use crate::case_match::{_calc_similarity_sm_cf, case_frame_match, cf_match_element, cf_match_exactly, count_assigned_adjacent_element, count_pat_element, EX_match_exact};
use crate::case_print::{print_crrspnd, print_data_cframe};
use crate::context::ETAG_name;
use crate::ctools::{assign_cfeature, check_feature, exit, malloc_data, OptAnalysis, stderr, stdout};
use crate::dpnd_analysis::{compare_dpnd, dpnd_info_to_bnst, dpnd_info_to_tag, Possibility};
use crate::feature::{check_str_type, delete_cfeature};
use crate::lib_print::{print_kakari, print_result};
use crate::read_data::{assign_feature_alt_mrph, assign_general_feature, copy_mrph, delete_existing_features, get_mrph_rep, get_mrph_rep_length, make_mrph_rn};
use crate::structs::{CDB_FILE, CF_ALIGNMENT, CF_MATCH_MGR, cpm_cache, LIST, PP_STR_TO_CODE, sentence};
use crate::thesaurus::get_str_code_with_len;
use crate::tools::{hash, OptCaseFlag, OptCFMode, OptCKY, OptDisplay, OptEllipsis, OptNbest, realloc_data, Thesaurus};
use crate::tree_conv::{bnst_to_tag_tree, make_dpnd_tree};
use crate::types::{CASE_FRAME, CF_PRED_MGR, CPM_CACHE, DBM_FILE, ELLIPSIS_COMPONENT, ELLIPSIS_MGR, SENTENCE_DATA, size_t};


/*====================================================================
		       格助詞の文字−コード対応
====================================================================*/

#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
#[no_mangle]
pub static mut Cf_match_mgr: *mut CF_MATCH_MGR = 0 as *const CF_MATCH_MGR as *mut CF_MATCH_MGR;
/* 作業領域 */
#[no_mangle]
pub static mut Work_mgr: TOTAL_MGR = TOTAL_MGR{
    dpnd: DPND{
        head: [0; 200],
        type_0: [0; 200],
        dflt: [0; 200],
        mask: [0; 200],
        pos: 0,
        check: [CHECK_DATA{num: 0, def: 0, pos: [0; 200],}; 200],
        f: [0 as *const FEATURE as *mut FEATURE; 200],
    },
    pssb: 0,
    dflt: 0,
    score: 0.,
    pred_num: 0,
    cpm: [
        CF_PRED_MGR{
            cf: CASE_FRAME{
                type_0: 0,
                type_flag: 0,
                element_num: 0,
                oblig: [0; 24],
                adjacent: [0; 24],
                pp: [[0; 10]; 24],
                sp: [0; 24],
                pp_str: [0 as *const libc::c_char as *mut libc::c_char; 24],
                sm: [0 as *const libc::c_char as *mut libc::c_char; 24],
                sm_delete: [0 as *const libc::c_char as *mut libc::c_char; 24],
                sm_delete_size: [0; 24],
                sm_delete_num: [0; 24],
                sm_specify: [0 as *const libc::c_char as *mut libc::c_char; 24],
                sm_specify_size: [0; 24],
                sm_specify_num: [0; 24],
                ex: [0 as *const libc::c_char as *mut libc::c_char; 24],
                ex_list: [0 as *const *mut libc::c_char as *mut *mut libc::c_char; 24],
                ex_freq: [0 as *const libc::c_int as *mut libc::c_int; 24],
                ex_size: [0; 24],
                ex_num: [0; 24],
                freq: [0; 24],
                semantics: [0 as *const libc::c_char as *mut libc::c_char; 24],
                gex_list: [0 as *const *mut libc::c_char as *mut *mut libc::c_char; 24],
                gex_freq: [0 as *const libc::c_double as *mut libc::c_double; 24],
                gex_size: [0; 24],
                gex_num: [0; 24],
                voice: 0,
                cf_address: 0,
                cf_size: 0,
                cf_id: [0; 280],
                pred_type: [0; 4],
                entry: 0 as *const libc::c_char as *mut libc::c_char,
                imi: [0; 128],
                etcflag: 0,
                feature: 0 as *const libc::c_char as *mut libc::c_char,
                weight: [0; 24],
                samecase: [[0; 2]; 24],
                cf_align: [
                    CF_ALIGNMENT{
                        cf_id: 0 as *const libc::c_char as *mut libc::c_char,
                        aligned_case: [[0; 2]; 24],
                    };
                    5
                ],
                pred_b_ptr: 0 as *const TAG_DATA as *mut TAG_DATA,
                cf_similarity: 0.,
            },
            pred_b_ptr: 0 as *const TAG_DATA as *mut TAG_DATA,
            elem_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
            para_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
            elem_s_ptr: [0 as *const sentence as *mut sentence; 24],
            elem_b_num: [0; 24],
            score: 0.,
            result_num: 0,
            tie_num: 0,
            cmm: [
                CF_MATCH_MGR{
                    cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p: [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24], }; 10],
                    result_lists_d: [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],}; 10],
                    cpm: 0 as *const cpm_def as *mut cpm_def,
                };
                5
            ],
            decided: 0,
        };
        64
    ],
    ID: 0,
};
#[no_mangle]
pub static mut DISTANCE_STEP: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub static mut RENKAKU_STEP: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut STRONG_V_COST: libc::c_int = 8 as libc::c_int;
#[no_mangle]
pub static mut ADJACENT_TOUTEN_COST: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub static mut LEVELA_COST: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub static mut TEIDAI_STEP: libc::c_int = 2 as libc::c_int;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn realloc_cmm() 
 /*==================================================================*/
 {
    Cf_match_mgr =
        realloc_data(Cf_match_mgr as *mut libc::c_void,
                     (::std::mem::size_of::<CF_MATCH_MGR>() as
                          libc::c_ulong).wrapping_mul(MAX_Case_frame_num as
                                                          libc::c_ulong),
                     b"realloc_cmm\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char) as *mut CF_MATCH_MGR;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_case_frame(mut cf: *mut CASE_FRAME) 
 /*==================================================================*/
 {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 24 as libc::c_int {
        if Thesaurus == 1 as libc::c_int {
            (*cf).ex[j as usize] =
                malloc_data((::std::mem::size_of::<libc::c_char>() as
                                 libc::c_ulong).wrapping_mul(256 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(11
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong),
                            b"init_case_frame\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char
        } else if Thesaurus == 2 as libc::c_int {
            (*cf).ex[j as usize] =
                malloc_data((::std::mem::size_of::<libc::c_char>() as
                                 libc::c_ulong).wrapping_mul(256 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(12
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong),
                            b"init_case_frame\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char
        }
        (*cf).sm[j as usize] =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                             libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                             libc::c_ulong).wrapping_mul(12
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong),
                        b"init_case_frame\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        (*cf).ex_list[j as usize] =
            malloc_data(::std::mem::size_of::<*mut libc::c_char>() as
                            libc::c_ulong,
                        b"init_case_frame\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut libc::c_char;
        let ref mut fresh0 =
            *(*cf).ex_list[j as usize].offset(0 as libc::c_int as isize);
        *fresh0 =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                             libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                             libc::c_ulong),
                        b"init_case_frame\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        (*cf).ex_freq[j as usize] =
            malloc_data(::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        b"init_case_frame\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_int;
        j += 1
    }
    (*cf).cf_id[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*cf).imi[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_case_analysis_cmm() 
 /*==================================================================*/
 {
    if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int {
        /* 作業cmm領域確保 */
        Cf_match_mgr =
            malloc_data((::std::mem::size_of::<CF_MATCH_MGR>() as
                             libc::c_ulong).wrapping_mul(1024 as libc::c_int
                                                             as
                                                             libc::c_ulong),
                        b"init_case_analysis_cmm\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut CF_MATCH_MGR;
        init_mgr_cf(&mut Work_mgr);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_case_frame(mut cf: *mut CASE_FRAME) 
 /*==================================================================*/
 {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 24 as libc::c_int {
        free((*cf).ex[j as usize] as *mut libc::c_void);
        free((*cf).sm[j as usize] as *mut libc::c_void);
        free(*(*cf).ex_list[j as usize].offset(0 as libc::c_int as isize) as
                 *mut libc::c_void);
        free((*cf).ex_list[j as usize] as *mut libc::c_void);
        free((*cf).ex_freq[j as usize] as *mut libc::c_void);
        j += 1
    };
}
#[no_mangle]
pub static mut PP_str_to_code: [PP_STR_TO_CODE; 50] =
    [{
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xcf\x86\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xcf\x86\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\x8c\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x82\xac\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\x8b\xe3\x82\x89\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x82\xab\xe3\x83\xa9\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xb8\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x98\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x88\xe3\x82\x8a\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xa8\xe3\x83\xaa\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xa8\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x88\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xa7\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x87\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x82\x88\xe3\x81\xa3\xe3\x81\xa6\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\x83\xe3\x83\x86\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x82\x81\xe3\x81\x90\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\xa1\xe3\x82\xb0\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x98\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\x84\xe3\x82\xa6\xe3\x82\xb8\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x81\xa4\xe3\x81\x86\xe3\x81\x9a\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\x84\xe3\x82\xa6\xe3\x82\xba\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x81\xb5\xe3\x81\x8f\xe3\x82\x81\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\x95\xe3\x82\xaf\xe3\x83\xa1\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x81\xaf\xe3\x81\x98\xe3\x82\x81\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\x8f\xe3\x82\xb8\xe3\x83\xa1\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x89\xe3\x82\x80\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xab\xe3\x83\xa9\xe3\x83\xa0\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x9d\xe3\x81\x86\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xbd\xe3\x82\xa6\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 16 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x82\x80\xe3\x81\x91\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\xa0\xe3\x82\xb1\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 17 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\xa8\xe3\x82\x82\xe3\x81\xaa\xe3\x81\x86\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\x88\xe3\x83\xa2\xe3\x83\x8a\xe3\x82\xa6\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 18 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x82\x82\xe3\x81\xa8\xe3\x81\xa5\xe3\x81\x8f\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\xa2\xe3\x83\x88\xe3\x83\x85\xe3\x82\xaf\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 19 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x82\x92\xe3\x81\xae\xe3\x81\x9e\xe3\x81\x8f\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\xb2\xe3\x83\x8e\xe3\x82\xbe\xe3\x82\xaf\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 20 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8b\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\xab\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 21 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x9f\xe3\x81\x84\xe3\x81\x99\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xbf\xe3\x82\xa4\xe3\x82\xb9\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 22 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x93\xe3\x81\x99\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xab\xe3\x83\xb3\xe3\x82\xb9\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 23 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8b\xe3\x82\x8f\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xab\xe3\x83\xaf\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 24 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8a\xe3\x81\x8f\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xaa\xe3\x82\xaf\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 25 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\x8f\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\x84\xe3\x82\xaf\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 26 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\xa8\xe3\x82\x8b\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\x88\xe3\x83\xab\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 27 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x8f\xe3\x81\x88\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xaf\xe3\x83\xaf\xe3\x82\xa8\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 28 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xab\xe3\x82\xae\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 29 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\xa4\xe3\x81\xa5\xe3\x81\x8f\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\x84\xe3\x83\x85\xe3\x82\xaf\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 30 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8f\xe3\x81\x9b\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xa2\xe3\x83\xaf\xe3\x82\xbb\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 31 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8f\xe3\x82\x89\xe3\x81\xb9\xe3\x82\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xaf\xe3\x83\xa9\xe3\x83\x99\xe3\x83\xab\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\xaa\xe3\x82\x89\xe3\x81\xb6\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\x8a\xe3\x83\xa9\xe3\x83\x96\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 33 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xa8\xe3\x81\x99\xe3\x82\x8b\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x88\xe3\x82\xb9\xe3\x83\xab\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 34 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8b\xe3\x81\xac\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\xab\xe3\x83\x8c\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 35 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xab\xe3\x81\x8b\xe3\x81\x8e\xe3\x82\x8b\xe3\x81\xac\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8b\xe3\x82\xab\xe3\x82\xae\xe3\x83\xab\xe3\x83\x8c\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 36 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xa8\xe3\x81\x84\xe3\x81\x86\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x88\xe3\x82\xa4\xe3\x82\xa6\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 37 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 38 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xbe\xe3\x81\xa7\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x9e\xe3\x83\x87\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 39 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 40 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xae\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8e\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 41 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\x8c\xef\xbc\x92\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 42 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 43 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\x8c\xe3\x81\x8c\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe3\x82\xac\xe3\x82\xac\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 42 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            kstr:
                                b"\xe5\xa4\x96\xe3\x83\x8e\xe9\x96\xa2\xe4\xbf\x82\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                            code: 43 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe3\x81\xaf\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe3\x83\x8f\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xe6\x9c\xaa\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xe6\x9c\xaa\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                b"\xef\xbc\x8a\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            kstr:
                                b"\xef\xbc\x8a\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            code: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             PP_STR_TO_CODE{hstr:
                                0 as *const libc::c_char as *mut libc::c_char,
                            kstr:
                                0 as *const libc::c_char as *mut libc::c_char,
                            code: -(1 as libc::c_int),};
         init
     }];
/* ※ 格の最大数を変えたら、PP_NUMBER(const.h)を変えること */
/*====================================================================
			 文字−コード対応関数
====================================================================*/
#[no_mangle]
pub unsafe extern "C" fn pp_kstr_to_code(mut cp: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if strcmp(cp, b"NIL\x00" as *const u8 as *const libc::c_char) == 0 {
        return -(10 as libc::c_int)
    }
    i = 0 as libc::c_int;
    while !PP_str_to_code[i as usize].kstr.is_null() {
        if strcmp(PP_str_to_code[i as usize].kstr, cp) == 0 {
            return PP_str_to_code[i as usize].code
        }
        i += 1
    }
    if strcmp(cp,
              b"\xe3\x83\x8b\xe3\x83\x88\xe3\x83\x83\xe3\x83\x86\x00" as
                  *const u8 as *const libc::c_char) == 0 {
        /* 「待つ」 IPALのバグ ?? */
        return pp_kstr_to_code(b"\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\x83\xe3\x83\x86\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char)
    } else {
        if strcmp(cp, b"\xe3\x83\x8e\x00" as *const u8 as *const libc::c_char)
               == 0 {
            /* 格要素でなくなる場合 */
            return -(10 as libc::c_int)
        }
    }
    /* fprintf(stderr, "Invalid string (%s) in PP !\n", cp); */
    return -(10 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pp_hstr_to_code(mut cp: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !PP_str_to_code[i as usize].hstr.is_null() {
        if strcmp(PP_str_to_code[i as usize].hstr, cp) == 0 {
            return PP_str_to_code[i as usize].code
        }
        i += 1
    }
    return -(10 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pp_code_to_kstr(mut num: libc::c_int)
 -> *mut libc::c_char {
    return PP_str_to_code[num as usize].kstr;
}
#[no_mangle]
pub unsafe extern "C" fn pp_code_to_hstr(mut num: libc::c_int)
 -> *mut libc::c_char {
    return PP_str_to_code[num as usize].hstr;
}
#[no_mangle]
pub unsafe extern "C" fn pp_code_to_kstr_in_context(mut cpm_ptr:
                                                        *mut CF_PRED_MGR,
                                                    mut num: libc::c_int)
 -> *mut libc::c_char {
    if (*cpm_ptr).cf.type_flag != 0 &&
           MatchPP(num,
                   b"\xcf\x86\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char) != 0 ||
           (*cpm_ptr).cf.type_0 == 2 as libc::c_int {
        return b"\xe3\x83\x8e\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return pp_code_to_kstr(num);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn MatchPPn(mut n: libc::c_int,
                                  mut list: *mut libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if n < 0 as libc::c_int { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while *list.offset(i as isize) != -(10 as libc::c_int) {
        if n == *list.offset(i as isize) { return 1 as libc::c_int }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn MatchPP(mut n: libc::c_int,
                                 mut pp: *mut libc::c_char) -> libc::c_int 
 /*==================================================================*/
 {
    if n < 0 as libc::c_int { return 0 as libc::c_int }
    if strcmp(pp_code_to_kstr(n), pp) == 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn MatchPP2(mut n: *mut libc::c_int, mut pp: *const libc::c_char) -> libc::c_int
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    /* 格の配列の中に調べたい格があるかどうか */
    if n < 0 as *mut libc::c_int { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while *n.offset(i as isize) != -(10 as libc::c_int) {
        if strcmp(pp_code_to_kstr(*n.offset(i as isize)), pp) == 0 {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CF_MatchPP(mut c: libc::c_int,
                                    mut cf: *mut CASE_FRAME) -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf).element_num {
        j = 0 as libc::c_int;
        while (*cf).pp[i as usize][j as usize] != -(10 as libc::c_int) {
            if (*cf).pp[i as usize][j as usize] == c {
                return 1 as libc::c_int
            }
            j += 1
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckCfAdjacent(mut cf: *mut CASE_FRAME)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf).element_num {
        if (*cf).adjacent[i as usize] != 0 &&
               MatchPP((*cf).pp[i as usize][0 as libc::c_int as usize],
                       b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char) != 0 {
            return 0 as libc::c_int
        }
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckCfClosest(mut cmm: *mut CF_MATCH_MGR,
                                        mut closest: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    return (*(*cmm).cf_ptr).adjacent[(*cmm).result_lists_d[0 as libc::c_int as
                                                               usize].flag[closest
                                                                               as
                                                                               usize]
                                         as usize];
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn have_real_component(mut cf_ptr: *mut CASE_FRAME)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 入力側が修飾、無格以外をもっているかどうか */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf_ptr).element_num {
        if MatchPP((*cf_ptr).pp[i as usize][0 as libc::c_int as usize],
                   b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char) != 0 ||
               MatchPP((*cf_ptr).pp[i as usize][0 as libc::c_int as usize],
                       b"\xcf\x86\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char) != 0 {
        } else { return (0 as libc::c_int == 0) as libc::c_int }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_best_cf(mut sp: *mut SENTENCE_DATA,
                                      mut cpm_ptr: *mut CF_PRED_MGR,
                                      mut closest: libc::c_int,
                                      mut decide: libc::c_int,
                                      mut para_cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_double 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut frame_num: libc::c_int = 0 as libc::c_int;
    let mut pat_num: libc::c_int = 0;
    let mut cf_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    let mut b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    let mut tempcmm: CF_MATCH_MGR =
        CF_MATCH_MGR{cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
                     score: 0.,
                     pure_score: [0.; 10],
                     sufficiency: 0.,
                     result_num: 0,
                     result_lists_p:
                         [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],};
                             10],
                     result_lists_d:
                         [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],};
                             10],
                     cpm: 0 as *const cpm_def as *mut cpm_def,};
    /* 格要素なしの時の実験 */
    if (*cf_ptr).type_0 == 1 as libc::c_int &&
           ((*cf_ptr).element_num == 0 as libc::c_int ||
                have_real_component(cf_ptr) == 0 as libc::c_int) {
        /* この用言のすべての格フレームの OR、または
	   格フレームが 1 つのときはそれそのもの にする予定 */
        if (*b_ptr).cf_num > 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*b_ptr).cf_num {
                if (*(*b_ptr).cf_ptr.offset(i as isize)).etcflag &
                       1 as libc::c_int != 0 {
                    let fresh1 = frame_num;
                    frame_num = frame_num + 1;
                    let ref mut fresh2 =
                        (*Cf_match_mgr.offset(fresh1 as isize)).cf_ptr;
                    *fresh2 = (*b_ptr).cf_ptr.offset(i as isize);
                    break ;
                } else { i += 1 }
            }
            /* OR格フレームがないとき
	       「動,形,準」の指定がないことがあればこうなる */
            if frame_num == 0 as libc::c_int {
                let fresh3 = frame_num;
                frame_num = frame_num + 1;
                let ref mut fresh4 =
                    (*Cf_match_mgr.offset(fresh3 as isize)).cf_ptr;
                *fresh4 = (*b_ptr).cf_ptr
            }
        } else {
            let fresh5 = frame_num;
            frame_num = frame_num + 1;
            let ref mut fresh6 =
                (*Cf_match_mgr.offset(fresh5 as isize)).cf_ptr;
            *fresh6 = (*b_ptr).cf_ptr
        }
        case_frame_match(cpm_ptr, Cf_match_mgr, OptCFMode,
                         -(1 as libc::c_int), 0 as *mut CF_PRED_MGR);
        (*cpm_ptr).score = (*Cf_match_mgr).score;
        (*cpm_ptr).cmm[0 as libc::c_int as usize] = *Cf_match_mgr;
        (*cpm_ptr).result_num = 1 as libc::c_int
    } else {
        let mut hiragana_prefer_flag: libc::c_int = 0 as libc::c_int;
        /* 表記がひらがなの場合: 
	   格フレームの表記がひらがなの場合が多ければひらがなの格フレームのみを対象に、
	   ひらがな以外が多ければひらがな以外のみを対象にする */
        if OptCaseFlag & 32 as libc::c_int == 0 &&
               check_str_type((*(*b_ptr).head_ptr).Goi.as_mut_ptr() as
                                  *mut libc::c_uchar, 2 as libc::c_int,
                              0 as libc::c_int) != 0 {
            if !check_feature((*b_ptr).f,
                              b"\xe4\xbb\xa3\xe8\xa1\xa8\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                hiragana_prefer_flag = 1 as libc::c_int
            } else { hiragana_prefer_flag = -(1 as libc::c_int) }
        }
        /* 格フレーム設定 */
        i = 0 as libc::c_int;
        while i < (*b_ptr).cf_num {
            /* OR の格フレームを除く */
            if !((*(*b_ptr).cf_ptr.offset(i as isize)).etcflag &
                     1 as libc::c_int != 0) {
                if !(hiragana_prefer_flag > 0 as libc::c_int &&
                         check_str_type((*(*b_ptr).cf_ptr.offset(i as
                                                                     isize)).entry
                                            as *mut libc::c_uchar,
                                        2 as libc::c_int, 0 as libc::c_int) ==
                             0 ||
                         hiragana_prefer_flag < 0 as libc::c_int &&
                             check_str_type((*(*b_ptr).cf_ptr.offset(i as
                                                                         isize)).entry
                                                as *mut libc::c_uchar,
                                            2 as libc::c_int,
                                            0 as libc::c_int) != 0) {
                    let fresh7 = frame_num;
                    frame_num = frame_num + 1;
                    let ref mut fresh8 =
                        (*Cf_match_mgr.offset(fresh7 as isize)).cf_ptr;
                    *fresh8 = (*b_ptr).cf_ptr.offset(i as isize)
                }
            }
            i += 1
        }
        if frame_num == 0 as libc::c_int {
            /* 上の処理でひとつも残らないとき */
            i = 0 as libc::c_int;
            while i < (*b_ptr).cf_num {
                let fresh9 = frame_num;
                frame_num = frame_num + 1;
                let ref mut fresh10 =
                    (*Cf_match_mgr.offset(fresh9 as isize)).cf_ptr;
                *fresh10 = (*b_ptr).cf_ptr.offset(i as isize);
                i += 1
            }
        }
        (*cpm_ptr).result_num = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < frame_num {
            /* 選択可能
	       EXAMPLE
	       SEMANTIC_MARKER */
            /* closest があれば、直前格要素のみのスコアになる */
            case_frame_match(cpm_ptr, Cf_match_mgr.offset(i as isize),
                             OptCFMode, closest, para_cpm_ptr);
            /* 結果を格納 */
            (*cpm_ptr).cmm[(*cpm_ptr).result_num as usize] =
                *Cf_match_mgr.offset(i as isize);
            /* DEBUG出力用: 下の print_good_crrspnds() で使う Cf_match_mgr のスコアを正規化 */
            if OptDisplay == 3 as libc::c_int && closest > -(1 as libc::c_int)
                   && OptEllipsis == 0 {
                pat_num =
                    count_pat_element((*Cf_match_mgr.offset(i as
                                                                isize)).cf_ptr,
                                      &mut *(*Cf_match_mgr.offset(i as
                                                                      isize)).result_lists_p.as_mut_ptr().offset(0
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     isize));
                if !((*Cf_match_mgr.offset(i as isize)).score <
                         0 as libc::c_int as libc::c_double ||
                         pat_num == 0 as libc::c_int) {
                    (*Cf_match_mgr.offset(i as isize)).score =
                        if OptCaseFlag & 16 as libc::c_int != 0 {
                            (*Cf_match_mgr.offset(i as
                                                      isize)).pure_score[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                        } else {
                            ((*Cf_match_mgr.offset(i as
                                                       isize)).pure_score[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize])
                                / sqrt(pat_num as libc::c_double)
                        }
                }
            }
            /* スコア順にソート */
            j = (*cpm_ptr).result_num - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                if !((*cpm_ptr).cmm[j as usize].score <
                         (*cpm_ptr).cmm[(j + 1 as libc::c_int) as usize].score
                         ||
                         (OptCaseFlag & 16 as libc::c_int != 0 &&
                              (*cpm_ptr).cmm[j as usize].score !=
                                  -(1001 as libc::c_int) as libc::c_double ||
                              OptCaseFlag & 16 as libc::c_int == 0 &&
                                  (*cpm_ptr).cmm[j as usize].score !=
                                      -(2 as libc::c_int) as libc::c_double)
                             &&
                             (*cpm_ptr).cmm[j as usize].score ==
                                 (*cpm_ptr).cmm[(j + 1 as libc::c_int) as
                                                    usize].score &&
                             (closest > -(1 as libc::c_int) &&
                                  (CheckCfClosest(&mut *(*cpm_ptr).cmm.as_mut_ptr().offset((j
                                                                                                +
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                                                               as
                                                                                               isize),
                                                  closest) ==
                                       (0 as libc::c_int == 0) as libc::c_int
                                       &&
                                       CheckCfClosest(&mut *(*cpm_ptr).cmm.as_mut_ptr().offset(j
                                                                                                   as
                                                                                                   isize),
                                                      closest) ==
                                           0 as libc::c_int) ||
                                  closest < 0 as libc::c_int &&
                                      (*cpm_ptr).cmm[j as usize].sufficiency <
                                          (*cpm_ptr).cmm[(j +
                                                              1 as
                                                                  libc::c_int)
                                                             as
                                                             usize].sufficiency))
                   {
                    break ;
                }
                tempcmm = (*cpm_ptr).cmm[j as usize];
                (*cpm_ptr).cmm[j as usize] =
                    (*cpm_ptr).cmm[(j + 1 as libc::c_int) as usize];
                (*cpm_ptr).cmm[(j + 1 as libc::c_int) as usize] = tempcmm;
                j -= 1
            }
            if (*cpm_ptr).result_num < 5 as libc::c_int - 1 as libc::c_int {
                (*cpm_ptr).result_num += 1
            }
            i += 1
        }
        /* スコアが同点の格フレームの個数を設定 */
        if (*cpm_ptr).result_num > 0 as libc::c_int &&
               (OptCaseFlag & 16 as libc::c_int != 0 &&
                    (*cpm_ptr).cmm[0 as libc::c_int as usize].score !=
                        -(1001 as libc::c_int) as libc::c_double ||
                    OptCaseFlag & 16 as libc::c_int == 0 &&
                        (*cpm_ptr).cmm[0 as libc::c_int as usize].score !=
                            -(2 as libc::c_int) as libc::c_double) {
            let mut top: libc::c_double = 0.;
            let mut cflag: libc::c_int = 0 as libc::c_int;
            (*cpm_ptr).tie_num = 1 as libc::c_int;
            top = (*cpm_ptr).cmm[0 as libc::c_int as usize].score;
            if closest > -(1 as libc::c_int) &&
                   CheckCfClosest(&mut *(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                  closest) ==
                       (0 as libc::c_int == 0) as libc::c_int {
                cflag = 1 as libc::c_int
            }
            i = 1 as libc::c_int;
            while i < (*cpm_ptr).result_num {
                /* score が最高で、
		   直前格要素が格フレームの直前格にマッチしているものがあれば(0番目をチェック)
		   直前格要素が格フレームの直前格にマッチしていることが条件
		   ↓
		   score が最高であることだけにした
		*/
                if !((*cpm_ptr).cmm[i as usize].score == top) { break ; }
                /*		if (cpm_ptr->cmm[i].score == top && 
		    (cflag == 0 || CheckCfClosest(&(cpm_ptr->cmm[i]), closest) == TRUE)) { */
                (*cpm_ptr).tie_num += 1;
                i += 1
            }
        }
        /* とりあえず設定
	   closest > -1: decided 決定用 */
        (*cpm_ptr).score =
            (*cpm_ptr).cmm[0 as libc::c_int as usize].score as libc::c_int as
                libc::c_double
    }
    /* 文脈解析: 直前格要素のスコアが閾値以上なら格フレームを決定 */
    if decide != 0 {
        if OptEllipsis != 0 {
            if closest > -(1 as libc::c_int) &&
                   (*cpm_ptr).score > 7 as libc::c_int as libc::c_double {
                if (*cpm_ptr).tie_num > 1 as libc::c_int {
                    (*cpm_ptr).decided = 1 as libc::c_int
                } else {
                    (*cpm_ptr).decided = 2 as libc::c_int;
                    /* exact match して、最高点の格フレームがひとつなら、それだけを表示 */
                    if (*cpm_ptr).score == EX_match_exact as libc::c_double {
                        (*cpm_ptr).result_num = 1 as libc::c_int
                    }
                }
            } else if closest == -(1 as libc::c_int) &&
                          (*cpm_ptr).cf.element_num > 0 as libc::c_int &&
                          !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                         b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() {
                (*cpm_ptr).decided = 2 as libc::c_int
            }
        } else if closest > -(1 as libc::c_int) {
            (*cpm_ptr).decided = 2 as libc::c_int
        }
    }
    if (*cf_ptr).element_num != 0 as libc::c_int {
        /* 直前格があるときは直前格のスコアしか考慮されていないので、
	   すべての格のスコアを足して正規化したものにする */
        if closest > -(1 as libc::c_int) {
            let mut slot_i: libc::c_int = 0;
            let mut slot_j: libc::c_int = 0;
            let mut pos_i: libc::c_int = 0;
            let mut pos_j: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*cpm_ptr).result_num {
                /* 割り当て失敗のとき(score==-1)は、pure_score は定義されていない */
		/* 入力側に任意格しかなく割り当てがないとき(score==0)は、分子分母ともに0になる */
                pat_num =
                    count_pat_element((*cpm_ptr).cmm[i as usize].cf_ptr,
                                      &mut *(*(*cpm_ptr).cmm.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)).result_lists_p.as_mut_ptr().offset(0
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize));
                if (*cpm_ptr).cmm[i as usize].score <
                       0 as libc::c_int as libc::c_double ||
                       pat_num == 0 as libc::c_int {
                    break ;
                }
                (*cpm_ptr).cmm[i as usize].score =
                    if OptCaseFlag & 16 as libc::c_int != 0 {
                        (*cpm_ptr).cmm[i as
                                           usize].pure_score[0 as libc::c_int
                                                                 as usize]
                    } else {
                        ((*cpm_ptr).cmm[i as
                                            usize].pure_score[0 as libc::c_int
                                                                  as usize]) /
                            sqrt(pat_num as libc::c_double)
                    };
                i += 1
            }
            /* 直前格スコアが同点の格フレームを、すべてのスコアでsort */
            i = (*cpm_ptr).tie_num - 1 as libc::c_int;
            while i >= 1 as libc::c_int {
                j = i - 1 as libc::c_int;
                while j >= 0 as libc::c_int {
                    slot_i =
                        (*cpm_ptr).cmm[i as
                                           usize].result_lists_d[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].flag[closest
                                                                                     as
                                                                                     usize];
                    pos_i =
                        if slot_i >= 0 as libc::c_int &&
                               !(*(*cpm_ptr).cmm[i as
                                                     usize].cf_ptr).ex_freq[slot_i
                                                                                as
                                                                                usize].is_null()
                           {
                            (*cpm_ptr).cmm[i as
                                               usize].result_lists_p[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].pos[slot_i
                                                                                        as
                                                                                        usize]
                        } else { -(1 as libc::c_int) };
                    slot_j =
                        (*cpm_ptr).cmm[j as
                                           usize].result_lists_d[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].flag[closest
                                                                                     as
                                                                                     usize];
                    pos_j =
                        if slot_j >= 0 as libc::c_int &&
                               !(*(*cpm_ptr).cmm[j as
                                                     usize].cf_ptr).ex_freq[slot_j
                                                                                as
                                                                                usize].is_null()
                           {
                            (*cpm_ptr).cmm[j as
                                               usize].result_lists_p[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].pos[slot_j
                                                                                        as
                                                                                        usize]
                        } else { -(1 as libc::c_int) };
                    if (*cpm_ptr).cmm[i as usize].score >
                           (*cpm_ptr).cmm[j as usize].score ||
                           pos_i >= 0 as libc::c_int &&
                               pos_j >= 0 as libc::c_int &&
                               *(*(*cpm_ptr).cmm[i as
                                                     usize].cf_ptr).ex_freq[slot_i
                                                                                as
                                                                                usize].offset(pos_i
                                                                                                  as
                                                                                                  isize)
                                   >
                                   *(*(*cpm_ptr).cmm[j as
                                                         usize].cf_ptr).ex_freq[slot_j
                                                                                    as
                                                                                    usize].offset(pos_j
                                                                                                      as
                                                                                                      isize)
                       {
                        tempcmm = (*cpm_ptr).cmm[i as usize];
                        (*cpm_ptr).cmm[i as usize] =
                            (*cpm_ptr).cmm[j as usize];
                        (*cpm_ptr).cmm[j as usize] = tempcmm
                    }
                    j -= 1
                }
                i -= 1
            }
        }
        (*cpm_ptr).score = (*cpm_ptr).cmm[0 as libc::c_int as usize].score
    }
    if OptDisplay == 3 as libc::c_int && OptCKY == 0 as libc::c_int {
        print_data_cframe(cpm_ptr, Cf_match_mgr);
        /* print_good_crrspnds(cpm_ptr, Cf_match_mgr, frame_num); */
        i = 0 as libc::c_int;
        while i < (*cpm_ptr).result_num {
            print_crrspnd(cpm_ptr,
                          &mut *(*cpm_ptr).cmm.as_mut_ptr().offset(i as
                                                                       isize));
            i += 1
        }
    }
    return (*cpm_ptr).score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_closest_case_component(mut sp:
                                                        *mut SENTENCE_DATA,
                                                    mut cpm_ptr:
                                                        *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 用言より前にある格要素の中で
       もっとも用言に近いものを探す
       (内部文節は除く: num == -1) 
       対象格: ヲ格, ニ格 */
    let mut i: libc::c_int = 0;
    let mut min: libc::c_int = -(1 as libc::c_int);
    let mut elem_b_num: libc::c_int = 0;
    /* 直前格要素を走査 */
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if !(*cpm_ptr).elem_b_ptr[i as usize].is_null() {
            /* 複合名詞の一部: 直前としない */
            if (*(*cpm_ptr).elem_b_ptr[i as usize]).inum > 0 as libc::c_int {
                return -(1 as libc::c_int)
            } else {
                /* 「〜を〜に」 */
                if (*(*cpm_ptr).pred_b_ptr).num ==
                       (*(*cpm_ptr).elem_b_ptr[i as usize]).num {
                    return i
                } else {
                    /* 用言にもっとも近い格要素を探す 
	   <回数>:無格 以外 */
                    if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int)
                           &&
                           (*(*cpm_ptr).elem_b_ptr[i as usize]).num <=
                               (*(*cpm_ptr).pred_b_ptr).num &&
                           min < (*(*cpm_ptr).elem_b_ptr[i as usize]).num &&
                           !(MatchPP((*cpm_ptr).cf.pp[i as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize],
                                     b"\xcf\x86\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char) != 0 &&
                                 !check_feature((*(*cpm_ptr).elem_b_ptr[i as
                                                                            usize]).f,
                                                b"\xe5\x9b\x9e\xe6\x95\xb0\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char).is_null())
                       {
                        min = (*(*cpm_ptr).elem_b_ptr[i as usize]).num;
                        elem_b_num = i
                    }
                }
            }
        }
        i += 1
    }
    /* 1. ヲ格, ニ格であるとき
       2. <主体>にマッチしない 1, 2 以外の格 (MatchPP(cpm_ptr->cf.pp[elem_b_num][0], "ガ"))
       3. 用言の直前の未格 (副詞がはさまってもよい)
       ★形容詞, 判定詞は?
       check_feature してもよい
       条件廃止: cpm_ptr->cf.pp[elem_b_num][1] == END_M */
    if min != -(1 as libc::c_int) {
        /* 決定しない:
	   1. 最近格要素が指示詞の場合 ★格だけマッチさせる?
	   2. ガ格で意味素がないとき */
        if !check_feature((*(*sp).tag_data.offset(min as isize)).f,
                          b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() ||
               Thesaurus == 2 as libc::c_int &&
                   (*(*sp).tag_data.offset(min as
                                               isize)).SM_code[0 as
                                                                   libc::c_int
                                                                   as usize]
                       as libc::c_int == '\u{0}' as i32 &&
                   MatchPP((*cpm_ptr).cf.pp[elem_b_num as
                                                usize][0 as libc::c_int as
                                                           usize],
                           b"\xe3\x82\xac\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) != 0
           {
            return -(2 as libc::c_int)
        } else {
            if (*cpm_ptr).cf.pp[elem_b_num as
                                    usize][0 as libc::c_int as usize] ==
                   -(1 as libc::c_int) &&
                   ((*(*cpm_ptr).pred_b_ptr).num == min + 1 as libc::c_int ||
                        (*(*cpm_ptr).pred_b_ptr).num == min + 2 as libc::c_int
                            &&
                            (!check_feature((*(*sp).tag_data.offset(min as
                                                                        isize).offset(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize)).f,
                                            b"\xe5\x89\xaf\xe8\xa9\x9e\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char).is_null()
                                 ||
                                 !check_feature((*(*sp).tag_data.offset(min as
                                                                            isize).offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)).f,
                                                b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char).is_null()))
                   ||
                   MatchPP((*cpm_ptr).cf.pp[elem_b_num as
                                                usize][0 as libc::c_int as
                                                           usize],
                           b"\xe3\x83\xb2\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) != 0
                   ||
                   MatchPP((*cpm_ptr).cf.pp[elem_b_num as
                                                usize][0 as libc::c_int as
                                                           usize],
                           b"\xe3\x83\x8b\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) != 0
                   ||
                   ((*cpm_ptr).cf.pp[elem_b_num as
                                         usize][0 as libc::c_int as usize] >
                        0 as libc::c_int &&
                        (*cpm_ptr).cf.pp[elem_b_num as
                                             usize][0 as libc::c_int as usize]
                            < 9 as libc::c_int ||
                        MatchPP((*cpm_ptr).cf.pp[elem_b_num as
                                                     usize][0 as libc::c_int
                                                                as usize],
                                b"\xe3\x83\x9e\xe3\x83\x87\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char) != 0) &&
                       cf_match_element((*cpm_ptr).cf.sm[elem_b_num as usize],
                                        b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        0 as libc::c_int) == 0 {
                (*cpm_ptr).cf.adjacent[elem_b_num as usize] =
                    (0 as libc::c_int == 0) as
                        libc::c_int; /* 直前格のマーク */
                return elem_b_num
            }
        }
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
unsafe extern "C" fn number_compare(mut i: *const libc::c_void,
                                    mut j: *const libc::c_void)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* sort function */
    return *(i as *const libc::c_int) - *(j as *const libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn inputcc2num(mut cpm_ptr: *mut CF_PRED_MGR)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut numbers: [libc::c_int; 24] = [0; 24];
    let mut str: [libc::c_char; 70] = [0; 70];
    let mut token: [libc::c_char; 3] = [0; 3];
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        numbers[i as usize] = (*(*cpm_ptr).elem_b_ptr[i as usize]).num;
        i += 1
    }
    qsort(numbers.as_mut_ptr() as *mut libc::c_void,
          (*cpm_ptr).cf.element_num as size_t,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          Some(number_compare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    str[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if i != 0 {
            sprintf(token.as_mut_ptr(),
                    b" %d\x00" as *const u8 as *const libc::c_char,
                    numbers[i as usize]);
        } else {
            sprintf(token.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    numbers[i as usize]);
        }
        strcat(str.as_mut_ptr(), token.as_mut_ptr());
        i += 1
    }
    sprintf(token.as_mut_ptr(),
            b" %d\x00" as *const u8 as *const libc::c_char,
            (*(*cpm_ptr).pred_b_ptr).num);
    strcat(str.as_mut_ptr(), token.as_mut_ptr());
    key = strdup(str.as_mut_ptr());
    return key;
}
#[no_mangle]
pub static mut CPMcache: [*mut CPM_CACHE; 1024] =
    [0 as *const CPM_CACHE as *mut CPM_CACHE; 1024];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn InitCPMcache() 
 /*==================================================================*/
 {
    memset(CPMcache.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut CPM_CACHE>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int as
                                                libc::c_ulong));
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ClearCPMcache() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut ccp: *mut CPM_CACHE = 0 as *mut CPM_CACHE;
    let mut next: *mut CPM_CACHE = 0 as *mut CPM_CACHE;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !CPMcache[i as usize].is_null() {
            ccp = CPMcache[i as usize];
            while !ccp.is_null() {
                free((*ccp).key as *mut libc::c_void);
                clear_case_frame(&mut (*(*ccp).cpm).cf);
                free((*ccp).cpm as *mut libc::c_void);
                next = (*ccp).next;
                free(ccp as *mut libc::c_void);
                ccp = next
            }
            CPMcache[i as usize] = 0 as *mut CPM_CACHE
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn RegisterCPM(mut cpm_ptr: *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    let mut num: libc::c_int = 0;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccpp: *mut *mut CPM_CACHE = 0 as *mut *mut CPM_CACHE;
    key = inputcc2num(cpm_ptr);
    if key.is_null() { return }
    num = hash(key as *mut libc::c_uchar, strlen(key) as libc::c_int);
    ccpp =
        &mut *CPMcache.as_mut_ptr().offset(num as isize) as
            *mut *mut CPM_CACHE;
    while !(*ccpp).is_null() { ccpp = &mut (**ccpp).next }
    *ccpp =
        malloc_data(::std::mem::size_of::<CPM_CACHE>() as libc::c_ulong,
                    b"RegisterCPM\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut CPM_CACHE;
    (**ccpp).key = key;
    (**ccpp).cpm =
        malloc_data(::std::mem::size_of::<CF_PRED_MGR>() as libc::c_ulong,
                    b"RegisterCPM\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut CF_PRED_MGR;
    init_case_frame(&mut (*(**ccpp).cpm).cf);
    copy_cpm((**ccpp).cpm, cpm_ptr, 0 as libc::c_int);
    (**ccpp).next = 0 as *mut cpm_cache;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckCPM(mut cpm_ptr: *mut CF_PRED_MGR)
 -> *mut CF_PRED_MGR 
 /*==================================================================*/
 {
    let mut num: libc::c_int = 0;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccp: *mut CPM_CACHE = 0 as *mut CPM_CACHE;
    key = inputcc2num(cpm_ptr);
    if key.is_null() { return 0 as *mut CF_PRED_MGR }
    num = hash(key as *mut libc::c_uchar, strlen(key) as libc::c_int);
    ccp = CPMcache[num as usize];
    while !ccp.is_null() {
        if strcmp(key, (*ccp).key) == 0 {
            free(key as *mut libc::c_void);
            return (*ccp).cpm
        }
        ccp = (*ccp).next
    }
    free(key as *mut libc::c_void);
    return 0 as *mut CF_PRED_MGR;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn case_analysis(mut sp: *mut SENTENCE_DATA,
                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut t_ptr: *mut TAG_DATA)
 -> libc::c_double 
 /*==================================================================*/
 {
    /*
                                              戻値
      入力の格要素がない場合                    -3
      格フレームがない場合                      -2
      入力側に必須格が残る場合(解析不成功)      -1
      解析成功                               score (0以上)
    */
    let mut closest: libc::c_int = 0;
    let mut cache_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    /* 初期化 */
    (*cpm_ptr).pred_b_ptr = t_ptr;
    (*cpm_ptr).score = -(1 as libc::c_int) as libc::c_double;
    (*cpm_ptr).result_num = 0 as libc::c_int;
    (*cpm_ptr).tie_num = 0 as libc::c_int;
    (*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr = 0 as *mut CASE_FRAME;
    (*cpm_ptr).decided = 0 as libc::c_int;
    /* 入力文側の格要素設定 */
    case_data::set_data_cf_type(cpm_ptr);
    closest = make_data_cframe(sp, cpm_ptr);
    /* 格フレーム解析スキップ
    if (cpm_ptr->cf.element_num == 0) {
	    cpm_ptr->cmm[0].cf_ptr = NULL;
	    return -3;
    }
    */
    /* cache */
    if OptAnalysis == 1 as libc::c_int && OptCaseFlag & 16 as libc::c_int == 0
           && { cache_ptr = CheckCPM(cpm_ptr); !cache_ptr.is_null() } {
        copy_cpm(cpm_ptr, cache_ptr, 0 as libc::c_int);
        return (*cpm_ptr).score
    }
    /* もっともスコアのよい格フレームを決定する
       文脈解析: 直前格要素がなければ格フレームを決定しない */
    /* 直前格要素がある場合 (closest > -1) のときは格フレームを決定する */
    find_best_cf(sp, cpm_ptr, closest, 1 as libc::c_int,
                 0 as *mut CF_PRED_MGR);
    if OptAnalysis == 1 as libc::c_int && OptCaseFlag & 16 as libc::c_int == 0
       {
        RegisterCPM(cpm_ptr);
    }
    return (*cpm_ptr).score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn all_case_analysis(mut sp: *mut SENTENCE_DATA,
                                           mut t_ptr: *mut TAG_DATA,
                                           mut t_mgr: *mut TOTAL_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut i: libc::c_int = 0;
    let mut renyou_modifying_num: libc::c_int = 0;
    let mut adverb_modifying_num: libc::c_int = 0;
    let mut current_pred_num: libc::c_int = 0;
    let mut one_case_point: libc::c_double = 0.;
    /* 格フレームの有無をチェック: set_pred_caseframe()の条件に従う */
    if (*t_ptr).para_top_p as libc::c_int !=
           (0 as libc::c_int == 0) as libc::c_int &&
           (*t_ptr).cf_num > 0 as libc::c_int {
        /* 格フレーム辞書になくてもデフォルト格フレームがあるので1以上になる */
        if (*t_mgr).pred_num >= 64 as libc::c_int {
            fprintf(stderr,
                    b";; too many predicates in a sentence. (> %d)\n\x00" as
                        *const u8 as *const libc::c_char, 64 as libc::c_int);
            exit(1 as libc::c_int);
        }
        cpm_ptr =
            &mut *(*t_mgr).cpm.as_mut_ptr().offset((*t_mgr).pred_num as isize)
                as *mut CF_PRED_MGR;
        one_case_point = case_analysis(sp, cpm_ptr, t_ptr);
        /* 解析不成功(入力側に必須格が残る)場合にその依存構造の解析を
	   やめる場合
	if (one_case_point == -1) return FALSE;
	*/
        (*t_mgr).score += one_case_point;
        (*t_mgr).pred_num += 1
    }
    /* 文末はEOSからの生成 (どの構造も等しいので、今のところ考慮しない) *
    if (check_feature(t_ptr->f, "文末") && 
	t_ptr->para_top_p != TRUE && 
	t_ptr->cf_num > 0 && 
	check_feature(t_ptr->f, "用言")) {
	t_mgr->score += calc_vp_modifying_probability(NULL, NULL, t_ptr, cpm_ptr->cmm[0].cf_ptr);
    }
    */
    renyou_modifying_num = 0 as libc::c_int;
    adverb_modifying_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !(*t_ptr).child[i as usize].is_null() {
        current_pred_num = (*t_mgr).pred_num;
        if all_case_analysis(sp, (*t_ptr).child[i as usize], t_mgr) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        if OptCaseFlag & 16 as libc::c_int != 0 &&
               (*t_ptr).para_top_p as libc::c_int !=
                   (0 as libc::c_int == 0) as libc::c_int &&
               (*t_ptr).cf_num > 0 as libc::c_int &&
               !check_feature((*t_ptr).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            if !check_feature((*(*t_ptr).child[i as usize]).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*(*t_ptr).child[i as usize]).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                   check_feature((*(*t_ptr).child[i as usize]).f,
                                 b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
                (*t_mgr).score +=
                    calc_vp_modifying_probability(t_ptr,
                                                  (*cpm_ptr).cmm[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].cf_ptr,
                                                  (*t_ptr).child[i as usize],
                                                  (*t_mgr).cpm[current_pred_num
                                                                   as
                                                                   usize].cmm[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize].cf_ptr);
                renyou_modifying_num += 1
            }
            /* この用言に係る副詞または修飾をカウント */
            if !check_feature((*(*t_ptr).child[i as usize]).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*(*t_ptr).child[i as usize]).f,
                                 b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char).is_null() ||
                   !check_feature((*(*t_ptr).child[i as usize]).f,
                                  b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                (*t_mgr).score +=
                    calc_adv_modifying_probability(t_ptr,
                                                   (*cpm_ptr).cmm[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].cf_ptr,
                                                   (*t_ptr).child[i as
                                                                      usize]);
                adverb_modifying_num += 1
            }
        }
        i += 1
    }
    if OptCaseFlag & 16 as libc::c_int != 0 &&
           (*t_ptr).para_top_p as libc::c_int !=
               (0 as libc::c_int == 0) as libc::c_int &&
           (*t_ptr).cf_num > 0 as libc::c_int &&
           !check_feature((*t_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
        (*t_mgr).score +=
            calc_vp_modifying_num_probability(t_ptr,
                                              (*cpm_ptr).cmm[0 as libc::c_int
                                                                 as
                                                                 usize].cf_ptr,
                                              renyou_modifying_num);
        (*t_mgr).score +=
            calc_adv_modifying_num_probability(t_ptr,
                                               (*cpm_ptr).cmm[0 as libc::c_int
                                                                  as
                                                                  usize].cf_ptr,
                                               adverb_modifying_num)
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cf_with_alloc(mut dst: *mut CASE_FRAME,
                                            mut src: *mut CASE_FRAME) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*dst).type_0 = (*src).type_0;
    (*dst).element_num = (*src).element_num;
    i = 0 as libc::c_int;
    while i < (*src).element_num {
        (*dst).oblig[i as usize] = (*src).oblig[i as usize];
        (*dst).adjacent[i as usize] = (*src).adjacent[i as usize];
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            (*dst).pp[i as usize][j as usize] =
                (*src).pp[i as usize][j as usize];
            j += 1
        }
        if !(*src).pp_str[i as usize].is_null() {
            (*dst).pp_str[i as usize] = strdup((*src).pp_str[i as usize])
        } else { (*dst).pp_str[i as usize] = 0 as *mut libc::c_char }
        if !(*src).sm[i as usize].is_null() {
            (*dst).sm[i as usize] = strdup((*src).sm[i as usize])
        } else { (*dst).sm[i as usize] = 0 as *mut libc::c_char }
        if !(*src).ex[i as usize].is_null() {
            (*dst).ex[i as usize] = strdup((*src).ex[i as usize])
        } else { (*dst).ex[i as usize] = 0 as *mut libc::c_char }
        if !(*src).ex_list[i as usize].is_null() {
            (*dst).ex_list[i as usize] =
                malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((*src).ex_size[i
                                                                                as
                                                                                usize]
                                                                 as
                                                                 libc::c_ulong),
                            b"copy_cf_with_alloc\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut *mut libc::c_char;
            (*dst).ex_freq[i as usize] =
                malloc_data((::std::mem::size_of::<libc::c_int>() as
                                 libc::c_ulong).wrapping_mul((*src).ex_size[i
                                                                                as
                                                                                usize]
                                                                 as
                                                                 libc::c_ulong),
                            b"copy_cf_with_alloc\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_int;
            j = 0 as libc::c_int;
            while j < (*src).ex_num[i as usize] {
                let ref mut fresh11 =
                    *(*dst).ex_list[i as usize].offset(j as isize);
                *fresh11 =
                    strdup(*(*src).ex_list[i as usize].offset(j as isize));
                *(*dst).ex_freq[i as usize].offset(j as isize) =
                    *(*src).ex_freq[i as usize].offset(j as isize);
                j += 1
            }
        } else {
            (*dst).ex_list[i as usize] = 0 as *mut *mut libc::c_char;
            (*dst).ex_freq[i as usize] = 0 as *mut libc::c_int
        }
        (*dst).ex_size[i as usize] = (*src).ex_size[i as usize];
        (*dst).ex_num[i as usize] = (*src).ex_num[i as usize];
        if !(*src).semantics[i as usize].is_null() {
            (*dst).semantics[i as usize] =
                strdup((*src).semantics[i as usize])
        } else { (*dst).semantics[i as usize] = 0 as *mut libc::c_char }
        i += 1
    }
    (*dst).voice = (*src).voice;
    (*dst).cf_address = (*src).cf_address;
    (*dst).cf_size = (*src).cf_size;
    strcpy((*dst).cf_id.as_mut_ptr(), (*src).cf_id.as_mut_ptr());
    strcpy((*dst).pred_type.as_mut_ptr(), (*src).pred_type.as_mut_ptr());
    strcpy((*dst).imi.as_mut_ptr(), (*src).imi.as_mut_ptr());
    (*dst).etcflag = (*src).etcflag;
    if !(*src).feature.is_null() {
        (*dst).feature = strdup((*src).feature)
    } else { (*dst).feature = 0 as *mut libc::c_char }
    if !(*src).entry.is_null() {
        (*dst).entry = strdup((*src).entry)
    } else { (*dst).entry = 0 as *mut libc::c_char }
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        (*dst).samecase[i as usize][0 as libc::c_int as usize] =
            (*src).samecase[i as usize][0 as libc::c_int as usize];
        (*dst).samecase[i as usize][1 as libc::c_int as usize] =
            (*src).samecase[i as usize][1 as libc::c_int as usize];
        i += 1
    }
    (*dst).cf_similarity = (*src).cf_similarity;
    /* weight, pred_b_ptr は未設定 */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cf(mut dst: *mut CASE_FRAME,
                                 mut src: *mut CASE_FRAME) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*dst).type_0 = (*src).type_0;
    (*dst).type_flag = (*src).type_flag;
    (*dst).element_num = (*src).element_num;
    /*    for (i = 0; i < CF_ELEMENT_MAX; i++) { */
    i = 0 as libc::c_int; /* これを使う場合問題あり */
    while i < (*src).element_num {
        (*dst).oblig[i as usize] = (*src).oblig[i as usize];
        (*dst).adjacent[i as usize] = (*src).adjacent[i as usize];
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            (*dst).pp[i as usize][j as usize] =
                (*src).pp[i as usize][j as usize];
            j += 1
        }
        (*dst).pp_str[i as usize] = (*src).pp_str[i as usize];
        /* for (j = 0; j < SM_ELEMENT_MAX*SM_CODE_SIZE; j++) {
	    dst->sm[i][j] = src->sm[i][j];
	} */
        if !(*src).sm[i as usize].is_null() {
            strcpy((*dst).sm[i as usize], (*src).sm[i as usize]);
        }
        if !(*src).ex[i as usize].is_null() {
            strcpy((*dst).ex[i as usize], (*src).ex[i as usize]);
        }
        strcpy(*(*dst).ex_list[i as usize].offset(0 as libc::c_int as isize),
               *(*src).ex_list[i as usize].offset(0 as libc::c_int as isize));
        j = 0 as libc::c_int;
        while j < (*src).ex_num[i as usize] {
            *(*dst).ex_freq[i as usize].offset(j as isize) =
                *(*src).ex_freq[i as usize].offset(j as isize);
            j += 1
        }
        (*dst).ex_size[i as usize] = (*src).ex_size[i as usize];
        (*dst).ex_num[i as usize] = (*src).ex_num[i as usize];
        i += 1
    }
    (*dst).voice = (*src).voice;
    (*dst).cf_address = (*src).cf_address;
    (*dst).cf_size = (*src).cf_size;
    strcpy((*dst).cf_id.as_mut_ptr(), (*src).cf_id.as_mut_ptr());
    strcpy((*dst).pred_type.as_mut_ptr(), (*src).pred_type.as_mut_ptr());
    strcpy((*dst).imi.as_mut_ptr(), (*src).imi.as_mut_ptr());
    (*dst).etcflag = (*src).etcflag;
    (*dst).feature = (*src).feature;
    (*dst).entry = (*src).entry;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        (*dst).samecase[i as usize][0 as libc::c_int as usize] =
            (*src).samecase[i as usize][0 as libc::c_int as usize];
        (*dst).samecase[i as usize][1 as libc::c_int as usize] =
            (*src).samecase[i as usize][1 as libc::c_int as usize];
        i += 1
    }
    (*dst).pred_b_ptr = (*src).pred_b_ptr;
    (*dst).cf_similarity = (*src).cf_similarity;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cpm(mut dst: *mut CF_PRED_MGR,
                                  mut src: *mut CF_PRED_MGR,
                                  mut flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if flag != 0 {
        copy_cf_with_alloc(&mut (*dst).cf, &mut (*src).cf);
    } else { copy_cf(&mut (*dst).cf, &mut (*src).cf); }
    (*dst).pred_b_ptr = (*src).pred_b_ptr;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        (*dst).elem_b_ptr[i as usize] = (*src).elem_b_ptr[i as usize];
        (*dst).para_b_ptr[i as usize] = (*src).para_b_ptr[i as usize];
        (*dst).elem_b_num[i as usize] = (*src).elem_b_num[i as usize];
        (*dst).elem_s_ptr[i as usize] = (*src).elem_s_ptr[i as usize];
        i += 1
    }
    (*dst).score = (*src).score;
    (*dst).result_num = (*src).result_num;
    (*dst).tie_num = (*src).tie_num;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        (*dst).cmm[i as usize] = (*src).cmm[i as usize];
        i += 1
    }
    (*dst).decided = (*src).decided;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_mgr(mut dst: *mut TOTAL_MGR,
                                  mut src: *mut TOTAL_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    (*dst).dpnd = (*src).dpnd;
    (*dst).pssb = (*src).pssb;
    (*dst).dflt = (*src).dflt;
    (*dst).score = (*src).score;
    (*dst).pred_num = (*src).pred_num;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        copy_cpm(&mut *(*dst).cpm.as_mut_ptr().offset(i as isize),
                 &mut *(*src).cpm.as_mut_ptr().offset(i as isize),
                 0 as libc::c_int);
        i += 1
    }
    (*dst).ID = (*src).ID;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn call_case_analysis(mut sp: *mut SENTENCE_DATA,
                                            mut dpnd: DPND,
                                            mut eos_flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut one_topic_score: libc::c_int = 0;
    let mut topic_score: libc::c_int = 0;
    let mut topic_score_sum: libc::c_int = 0 as libc::c_int;
    let mut topic_slot: [libc::c_int; 2] = [0; 2];
    let mut distance_cost: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 格構造解析のメイン関数 */
    /* 依存構造木作成 */
    dpnd_info_to_bnst(sp, &mut dpnd);
    dpnd_info_to_tag(sp, &mut dpnd);
    if make_dpnd_tree(sp) == 0 as libc::c_int { return 0 as libc::c_int }
    bnst_to_tag_tree(sp);
    if OptDisplay == 3 as libc::c_int {
        print_kakari(sp, 1 as libc::c_int, 1 as libc::c_int);
    }
    /* 格解析作業領域の初期化 */
    Work_mgr.pssb = Possibility;
    Work_mgr.dpnd = dpnd;
    Work_mgr.score = 0 as libc::c_int as libc::c_double;
    Work_mgr.pred_num = 0 as libc::c_int;
    Work_mgr.dflt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        Work_mgr.dflt += dpnd.dflt[i as usize];
        i += 1
    }
    /* 格解析呼び出し */
    if all_case_analysis(sp,
                         (*sp).tag_data.offset((*sp).Tag_num as
                                                   isize).offset(-(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)),
                         &mut Work_mgr) ==
           (0 as libc::c_int == 0) as libc::c_int {
        Possibility += 1
    } else { return 0 as libc::c_int }
    /* ここで default との距離のずれ, 提題を処理 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num - 1 as libc::c_int {
        /* ガ格 -> レベル:A (ルールでこの係り受けを許した場合は、
	   ここでコストを与える) */
        if OptCaseFlag & 16 as libc::c_int == 0 &&
               !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
               !check_feature((*(*sp).bnst_data.offset(dpnd.head[i as usize]
                                                           as isize)).f,
                              b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:A\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            distance_cost += LEVELA_COST
        }
        if dpnd.dflt[i as usize] > 0 as libc::c_int {
            /* 提題 */
            if OptCaseFlag & 16 as libc::c_int == 0 &&
                   !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                  b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                distance_cost += dpnd.dflt[i as usize];
                /* 提題につられて遠くに係ってしまった文節の距離コスト */
                j = 0 as libc::c_int;
                while j < i - 1 as libc::c_int {
                    if dpnd.head[i as usize] == dpnd.head[j as usize] {
                        k = j + 1 as libc::c_int;
                        while k < i {
                            if (*Mask_matrix.as_mut_ptr().offset(j as
                                                                     isize))[k
                                                                                 as
                                                                                 usize]
                                   != 0 &&
                                   (*Quote_matrix.as_mut_ptr().offset(j as
                                                                          isize))[k
                                                                                      as
                                                                                      usize]
                                       != 0 &&
                                   (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                         isize))[k
                                                                                     as
                                                                                     usize]
                                       != 0 &&
                                   (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                         isize))[k
                                                                                     as
                                                                                     usize]
                                       != 'd' as i32 {
                                distance_cost +=
                                    dpnd.dflt[i as usize] * TEIDAI_STEP
                            }
                            k += 1
                        }
                    }
                    j += 1
                }
            } else {
                /* 提題以外 */
	    /* 係り側が連用でないとき */
                if OptCaseFlag & 16 as libc::c_int == 0 &&
                       check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                     b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_char).is_null() {
                    /* 自分に読点がなく、隣の強い用言 (連体以外) を越えているとき */
                    if check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                     b"\xe8\xaa\xad\xe7\x82\xb9\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                        if dpnd.head[i as usize] > i + 1 as libc::c_int &&
                               subordinate_level_check(b"B\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*(*sp).bnst_data.offset(i
                                                                                    as
                                                                                    isize).offset(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)).f)
                                   != 0 &&
                               {
                                   cp =
                                       check_feature((*(*sp).bnst_data.offset(i
                                                                                  as
                                                                                  isize).offset(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).f,
                                                     b"\xe4\xbf\x82\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char);
                                   !cp.is_null()
                               } {
                            if strcmp(cp.offset(3 as libc::c_int as isize),
                                      b"\xe9\x80\xa3\xe4\xbd\x93\x00" as
                                          *const u8 as *const libc::c_char) !=
                                   0 &&
                                   strcmp(cp.offset(3 as libc::c_int as
                                                        isize),
                                          b"\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                              *const u8 as
                                              *const libc::c_char) != 0 {
                                distance_cost += STRONG_V_COST
                            }
                        }
                    } else if dpnd.head[i as usize] == i + 1 as libc::c_int {
                        distance_cost += ADJACENT_TOUTEN_COST
                    }
                }
                /* 自分に読点があり*/
                /* 隣に係るとき */
                /* 確率的: 副詞などのコスト (tentative) */
                if OptCaseFlag & 16 as libc::c_int != 0 {
                    if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                      b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() &&
                           check_feature((*(*sp).bnst_data.offset(i as
                                                                      isize)).f,
                                         b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                        distance_cost += dpnd.dflt[i as usize] * DISTANCE_STEP
                    }
                } else if check_feature((*(*sp).bnst_data.offset(i as
                                                                     isize)).f,
                                        b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char).is_null() ||
                              !check_feature((*(*sp).bnst_data.offset(i as
                                                                          isize)).f,
                                             b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                 {
                    distance_cost += dpnd.dflt[i as usize] * DISTANCE_STEP
                } else {
                    distance_cost += dpnd.dflt[i as usize] * RENKAKU_STEP
                }
            }
        }
        i += 1
    }
    Work_mgr.score -= distance_cost as libc::c_double;
    if OptCaseFlag & 16 as libc::c_int == 0 {
        i = (*sp).Bnst_num - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            /* デフォルトとの差 x 2 を距離のコストとする
	       ただし、形容詞を除く連格の場合は x 1 */
            /* 文末から用言ごとに提題を処理する */
            cp =
                check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                /* topic_slot[0]	時間以外のハ格のスロット
		   topic_slot[1]	「<<時間>>は」のスロット
		   両方とも 1 以下しか許可しない
		*/
                topic_slot[0 as libc::c_int as usize] = 0 as libc::c_int;
                topic_slot[1 as libc::c_int as usize] = 0 as libc::c_int;
                one_topic_score = 0 as libc::c_int;
                /* 係り側を探す */
                j = i - 1 as libc::c_int;
                while j >= 0 as libc::c_int {
                    if !(dpnd.head[j as usize] != i) {
                        if !check_feature((*(*sp).bnst_data.offset(j as
                                                                       isize)).f,
                                          b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                           {
                            if !check_feature((*(*sp).bnst_data.offset(j as
                                                                           isize)).f,
                                              b"\xe6\x99\x82\xe9\x96\x93\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                               {
                                topic_slot[1 as libc::c_int as usize] += 1
                            } else {
                                topic_slot[0 as libc::c_int as usize] += 1
                            }
                            sscanf(cp,
                                   b"%*[^:]:%d\x00" as *const u8 as
                                       *const libc::c_char,
                                   &mut topic_score as *mut libc::c_int);
                            one_topic_score += topic_score
                        }
                    }
                    j -= 1
                }
                if topic_slot[0 as libc::c_int as usize] > 0 as libc::c_int ||
                       topic_slot[1 as libc::c_int as usize] >
                           0 as libc::c_int {
                    one_topic_score += 20 as libc::c_int
                }
                Work_mgr.score += one_topic_score as libc::c_double;
                if OptDisplay == 3 as libc::c_int {
                    topic_score_sum += one_topic_score
                }
            }
            i -= 1
        }
    }
    if OptDisplay == 3 as libc::c_int {
        if OptCaseFlag & 16 as libc::c_int != 0 {
            fprintf(stdout,
                    b"\xe2\x96\xa0 %.5f\xe7\x82\xb9 (\xe8\xb7\x9d\xe9\x9b\xa2\xe6\xb8\x9b\xe7\x82\xb9 %d\xe7\x82\xb9)\n\x00"
                        as *const u8 as *const libc::c_char, Work_mgr.score,
                    distance_cost);
        } else {
            fprintf(stdout,
                    b"\xe2\x96\xa0 %d\xe7\x82\xb9 (\xe8\xb7\x9d\xe9\x9b\xa2\xe6\xb8\x9b\xe7\x82\xb9 %d\xe7\x82\xb9 (%d\xe7\x82\xb9) \xe6\x8f\x90\xe9\xa1\x8c\xe3\x82\xb9\xe3\x82\xb3\xe3\x82\xa2 %d\xe7\x82\xb9)\n\x00"
                        as *const u8 as *const libc::c_char,
                    Work_mgr.score as libc::c_int, distance_cost,
                    Work_mgr.dflt * 2 as libc::c_int, topic_score_sum);
        }
    }
    /* -nbestのため出力 */
    if OptNbest == (0 as libc::c_int == 0) as libc::c_int {
        /* featureを仮付与 */
        assign_general_feature((*sp).bnst_data as *mut libc::c_void,
                               (*sp).Bnst_num, 12 as libc::c_int,
                               0 as libc::c_int,
                               (0 as libc::c_int == 0) as libc::c_int);
        assign_general_feature((*sp).tag_data as *mut libc::c_void,
                               (*sp).Tag_num, 13 as libc::c_int,
                               0 as libc::c_int,
                               (0 as libc::c_int == 0) as libc::c_int);
        /* 格解析の結果をfeatureとして仮付与 */
        i = 0 as libc::c_int;
        while i < Work_mgr.pred_num {
            record_case_analysis(sp,
                                 &mut *Work_mgr.cpm.as_mut_ptr().offset(i as
                                                                            isize),
                                 0 as *mut ELLIPSIS_MGR,
                                 (0 as libc::c_int == 0) as libc::c_int);
            i += 1
        }
        (*sp).score = Work_mgr.score;
        print_result(sp, 0 as libc::c_int, eos_flag);
        /* 仮付与したfeatureを削除 */
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            feature::delete_temp_feature(&mut (*(*sp).bnst_data.offset(i as isize)).f);
            i += 1
        }
        i = 0 as libc::c_int;
        while i < (*sp).Tag_num {
            feature::delete_temp_feature(&mut (*(*sp).tag_data.offset(i as isize)).f);
            i += 1
        }
    }
    /* 後処理 */
    if Work_mgr.score > (*(*sp).Best_mgr).score ||
           Work_mgr.score == (*(*sp).Best_mgr).score &&
               compare_dpnd(sp, &mut Work_mgr, (*sp).Best_mgr) ==
                   (0 as libc::c_int == 0) as libc::c_int {
        copy_mgr((*sp).Best_mgr, &mut Work_mgr);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn add_cf_slot(mut cpm_ptr: *mut CF_PRED_MGR,
                                     mut cstr: *mut libc::c_char,
                                     mut num: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    if (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num >=
           24 as libc::c_int {
        return 0 as libc::c_int
    }
    _make_ipal_cframe_pp((*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr,
                         cstr as *mut libc::c_uchar,
                         (*(*cpm_ptr).cmm[0 as libc::c_int as
                                              usize].cf_ptr).element_num,
                         1 as libc::c_int);
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_d[0 as libc::c_int as
                                                 usize].flag[num as usize] =
        (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_d[0 as libc::c_int as
                                                 usize].score[num as usize] =
        0 as libc::c_int as libc::c_double;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_p[0 as libc::c_int as
                                                 usize].flag[(*(*cpm_ptr).cmm[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize].cf_ptr).element_num
                                                                 as usize] =
        num;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_p[0 as libc::c_int as
                                                 usize].score[(*(*cpm_ptr).cmm[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].cf_ptr).element_num
                                                                  as usize] =
        0 as libc::c_int as libc::c_double;
    (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num += 1;
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_cf_slot(mut cpm_ptr: *mut CF_PRED_MGR,
                                        mut cnum: libc::c_int,
                                        mut num: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    /* 格フレームのその格にすでに対応付けがあれば */
    if (*cpm_ptr).cmm[0 as libc::c_int as
                          usize].result_lists_p[0 as libc::c_int as
                                                    usize].flag[cnum as usize]
           != -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_d[0 as libc::c_int as
                                                 usize].flag[num as usize] =
        cnum;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_d[0 as libc::c_int as
                                                 usize].score[num as usize] =
        0 as libc::c_int as libc::c_double;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_p[0 as libc::c_int as
                                                 usize].flag[cnum as usize] =
        num;
    (*cpm_ptr).cmm[0 as libc::c_int as
                       usize].result_lists_p[0 as libc::c_int as
                                                 usize].score[cnum as usize] =
        0 as libc::c_int as libc::c_double;
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_ga2_ok(mut cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num
          {
        /* 割り当てなし、<主体>アリのガ格, ヲ格, ニ格が存在するならば、ガ２不可 */
        if (*cpm_ptr).cmm[0 as libc::c_int as
                              usize].result_lists_p[0 as libc::c_int as
                                                        usize].flag[i as
                                                                        usize]
               == -(1 as libc::c_int) &&
               cf_match_element((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                     usize].cf_ptr).sm[i as
                                                                           usize],
                                b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int) != 0
               &&
               (MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                                             usize].cf_ptr).pp[i as
                                                                   usize][0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize],
                        b"\xe3\x82\xac\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) != 0 ||
                    MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                 usize].cf_ptr).pp[i as
                                                                       usize][0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize],
                            b"\xe3\x83\xb2\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) != 0
                    ||
                    MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                 usize].cf_ptr).pp[i as
                                                                       usize][0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize],
                            b"\xe3\x83\x8b\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) !=
                        0) {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn decide_voice(mut sp: *mut SENTENCE_DATA,
                                      mut cpm_ptr: *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    let mut check_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    if (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).voice ==
           1 as libc::c_int {
        (*(*cpm_ptr).pred_b_ptr).voice = 0 as libc::c_int
    } else { (*(*cpm_ptr).pred_b_ptr).voice = 2 as libc::c_int }
    /* なくならないように */
    check_b_ptr = (*cpm_ptr).pred_b_ptr;
    while !(*check_b_ptr).parent.is_null() &&
              (*(*check_b_ptr).parent).para_top_p as libc::c_int ==
                  (0 as libc::c_int == 0) as libc::c_int {
        (*(*check_b_ptr).parent).voice = (*(*cpm_ptr).pred_b_ptr).voice;
        check_b_ptr = (*check_b_ptr).parent
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_print_string(mut bp: *mut TAG_DATA,
                                           mut flag: libc::c_int)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = 0 as libc::c_int;
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
       flag == 1: 自立語列
       flag == 0: 最後の自立語
    */
    if flag != 0 {
        /* 先頭をみる */
        i = 0 as libc::c_int;
        while i < (*bp).mrph_num {
            /* 付属の特殊を除く */
            if strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                                usize][0 as libc::c_int as usize].id as
                          *const libc::c_char,
                      b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                          *const libc::c_char) != 0 ||
                   !check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"\xe8\x87\xaa\xe7\xab\x8b\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                start = i;
                break ;
            } else { i += 1 }
        }
        /* 末尾をみる */
        i = (*bp).mrph_num - 1 as libc::c_int;
        while i >= start {
            /* 特殊, 助詞, 助動詞, 判定詞を除く */
            if (strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                                 usize][0 as libc::c_int as usize].id as
                           *const libc::c_char,
                       b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                           *const libc::c_char) != 0 ||
                    !check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                   b"\xe8\x87\xaa\xe7\xab\x8b\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) &&
                   strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                                    usize][0 as libc::c_int as usize].id as
                              *const libc::c_char,
                          b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                              *const libc::c_char) != 0 &&
                   strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                                    usize][0 as libc::c_int as usize].id as
                              *const libc::c_char,
                          b"\xe5\x8a\xa9\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                              *const u8 as *const libc::c_char) != 0 &&
                   strcmp(Class[(*(*bp).mrph_ptr.offset(i as isize)).Hinshi as
                                    usize][0 as libc::c_int as usize].id as
                              *const libc::c_char,
                          b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as
                              *const u8 as *const libc::c_char) != 0 {
                end = i;
                break ;
            } else { i -= 1 }
        }
        if start > end {
            start =
                (*bp).jiritu_ptr.wrapping_offset_from((*bp).mrph_ptr) as
                    libc::c_long as libc::c_int;
            end = (*bp).settou_num + (*bp).jiritu_num - 1 as libc::c_int
        }
        i = start;
        while i <= end {
            length =
                (length as
                     libc::c_ulong).wrapping_add(strlen((*(*bp).mrph_ptr.offset(i
                                                                                    as
                                                                                    isize)).Goi2.as_mut_ptr()))
                    as libc::c_int as libc::c_int;
            i += 1
        }
        if length == 0 as libc::c_int { return 0 as *mut libc::c_char }
        ret =
            malloc_data((length + 1 as libc::c_int) as size_t,
                        b"make_print_string\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        *ret = '\u{0}' as i32 as libc::c_char;
        i = start;
        while i <= end {
            strcat(ret,
                   (*(*bp).mrph_ptr.offset(i as isize)).Goi2.as_mut_ptr());
            i += 1
        }
    } else { ret = strdup((*(*bp).head_ptr).Goi2.as_mut_ptr()) }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn record_match_ex(mut sp: *mut SENTENCE_DATA,
                                         mut cpm_ptr: *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                               usize].result_lists_d[0 as libc::c_int as
                                                         usize].flag[i as
                                                                         usize];
        if num != -(2 as libc::c_int) &&
               !(*cpm_ptr).elem_b_ptr[i as usize].is_null() {
            /* && cpm_ptr->elem_b_num[i] < 0) { * 省略, 〜は, 連体修飾に限定する場合 */
            pos =
                (*cpm_ptr).cmm[0 as libc::c_int as
                                   usize].result_lists_p[0 as libc::c_int as
                                                             usize].pos[num as
                                                                            usize];
            if pos == -(2 as libc::c_int) || pos == -(1 as libc::c_int) {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x81\xe7\x94\xa8\xe4\xbe\x8b;%s:%s-%s\x00"
                            as *const u8 as *const libc::c_char,
                        pp_code_to_kstr_in_context(cpm_ptr,
                                                   (*(*cpm_ptr).cmm[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize].cf_ptr).pp[num
                                                                                              as
                                                                                              usize][0
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         usize]),
                        (*(*(*cpm_ptr).elem_b_ptr[i as
                                                      usize]).head_ptr).Goi.as_mut_ptr(),
                        if pos == -(2 as libc::c_int) {
                            b"NONE\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"SUBJECT\x00" as *const u8 as *const libc::c_char
                        });
            } else {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x81\xe7\x94\xa8\xe4\xbe\x8b;%s:%s-%s:%.5f\x00"
                            as *const u8 as *const libc::c_char,
                        pp_code_to_kstr_in_context(cpm_ptr,
                                                   (*(*cpm_ptr).cmm[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize].cf_ptr).pp[num
                                                                                              as
                                                                                              usize][0
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         usize]),
                        (*(*(*cpm_ptr).elem_b_ptr[i as
                                                      usize]).head_ptr).Goi.as_mut_ptr(),
                        *(*(*cpm_ptr).cmm[0 as libc::c_int as
                                              usize].cf_ptr).ex_list[num as
                                                                         usize].offset(pos
                                                                                           as
                                                                                           isize),
                        (*cpm_ptr).cmm[0 as libc::c_int as
                                           usize].result_lists_p[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].score[num
                                                                                      as
                                                                                      usize]);
            }
            assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                            feature_buffer.as_mut_ptr(), 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn record_closest_cc_match(mut sp: *mut SENTENCE_DATA,
                                                 mut cpm_ptr:
                                                     *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    /* 用言について、直前格要素のマッチスコアをfeatureに */
    let mut num: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut closest: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    if check_feature((*(*cpm_ptr).pred_b_ptr).f,
                     b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char).is_null() {
        return
    }
    closest = get_closest_case_component(sp, cpm_ptr);
    if closest >= 0 as libc::c_int &&
           (*(*cpm_ptr).elem_b_ptr[closest as usize]).num + 1 as libc::c_int
               == (*(*cpm_ptr).pred_b_ptr).num {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                               usize].result_lists_d[0 as libc::c_int as
                                                         usize].flag[closest
                                                                         as
                                                                         usize];
        if num != -(2 as libc::c_int) {
            pos =
                (*cpm_ptr).cmm[0 as libc::c_int as
                                   usize].result_lists_p[0 as libc::c_int as
                                                             usize].pos[num as
                                                                            usize];
            if pos != -(1 as libc::c_int) {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"\xe7\x9b\xb4\xe5\x89\x8d\xe6\xa0\xbc\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x81:%d\x00"
                            as *const u8 as *const libc::c_char,
                        (*cpm_ptr).cmm[0 as libc::c_int as
                                           usize].result_lists_p[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].score[num
                                                                                      as
                                                                                      usize]
                            as libc::c_int);
                assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                feature_buffer.as_mut_ptr(),
                                0 as libc::c_int);
            }
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_nil_assigned_components(mut sp:
                                                            *mut SENTENCE_DATA,
                                                        mut cpm_ptr:
                                                            *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if (*cpm_ptr).score < 0 as libc::c_int as libc::c_double { return }
    /* 未対応の格要素の処理 */
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if (*cpm_ptr).cmm[0 as libc::c_int as
                              usize].result_lists_d[0 as libc::c_int as
                                                        usize].flag[i as
                                                                        usize]
               == -(2 as libc::c_int) {
            /* 未格, 連格 */
            if (*cpm_ptr).elem_b_num[i as usize] == -(1 as libc::c_int) {
                /* <時間> => 時間 */
                if !check_feature((*(*cpm_ptr).elem_b_ptr[i as usize]).f,
                                  b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    if check_cf_case((*cpm_ptr).cmm[0 as libc::c_int as
                                                        usize].cf_ptr,
                                     b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char) < 0 as libc::c_int
                       {
                        add_cf_slot(cpm_ptr,
                                    b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, i);
                    }
                } else if (*(*cpm_ptr).elem_b_ptr[i as usize]).num <
                              (*(*cpm_ptr).pred_b_ptr).num &&
                              !check_feature((*(*cpm_ptr).elem_b_ptr[i as
                                                                         usize]).f,
                                             b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                              &&
                              (*(*cpm_ptr).pred_b_ptr).num !=
                                  (*(*cpm_ptr).elem_b_ptr[i as usize]).num +
                                      1 as libc::c_int &&
                              check_ga2_ok(cpm_ptr) != 0 {
                    if check_cf_case((*cpm_ptr).cmm[0 as libc::c_int as
                                                        usize].cf_ptr,
                                     b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char) < 0 as libc::c_int
                       {
                        add_cf_slot(cpm_ptr,
                                    b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, i);
                    }
                } else if (*cpm_ptr).cf.type_0 != 2 as libc::c_int &&
                              !((*(*cpm_ptr).elem_b_ptr[i as usize]).inum >
                                    0 as libc::c_int &&
                                    (*(*cpm_ptr).elem_b_ptr[i as
                                                                usize]).parent
                                        == (*cpm_ptr).pred_b_ptr) &&
                              (*cpm_ptr).cf.pp[i as
                                                   usize][0 as libc::c_int as
                                                              usize] !=
                                  pp_kstr_to_code(b"\xe6\x9c\xaa\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char) &&
                              MatchPP2((*cpm_ptr).cf.pp[i as
                                                            usize].as_mut_ptr(),
                                       b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                           as *const u8 as *const libc::c_char
                                           as *mut libc::c_char) != 0 {
                    /* 二重主語構文の外のガ格 */
                    /* その他 => 外の関係
		   複合名詞の前側: 保留
		   用言直前のノ格: 保留 */
                    /* 「外の関係」の可能性あるもの */
                    c =
                        check_cf_case((*cpm_ptr).cmm[0 as libc::c_int as
                                                         usize].cf_ptr,
                                      b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char);
                    if c < 0 as libc::c_int {
                        add_cf_slot(cpm_ptr,
                                    b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, i);
                    } else { assign_cf_slot(cpm_ptr, c, i); }
                }
            } else if check_cf_case((*cpm_ptr).cmm[0 as libc::c_int as
                                                       usize].cf_ptr,
                                    pp_code_to_kstr((*cpm_ptr).cf.pp[i as
                                                                         usize][0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize]))
                          < 0 as libc::c_int {
                add_cf_slot(cpm_ptr,
                            pp_code_to_kstr((*cpm_ptr).cf.pp[i as
                                                                 usize][0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]),
                            i);
            }
        }
        i += 1
    };
}
/* 格は明示されているが、格フレーム側にその格がなかった場合 */
	    /* ★ とりうる格が複数あるとき: ヘ格 */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_cc_string(mut word: *mut libc::c_char,
                                        mut tag_n: libc::c_int,
                                        mut pp_str: *mut libc::c_char,
                                        mut cc_type: libc::c_int,
                                        mut dist: libc::c_int,
                                        mut sid: *mut libc::c_char)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word_allocated_flag: libc::c_int = 1 as libc::c_int;
    if word.is_null() {
        /* 文字列がないとき */
        word =
            b"(null)\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        word_allocated_flag = 0 as libc::c_int
    }
    buf =
        malloc_data((strlen(pp_str).wrapping_add(strlen(word)).wrapping_add(strlen(sid))
                         as libc::c_double +
                         (if dist != 0 {
                              log(dist as libc::c_double)
                          } else { 0 as libc::c_int as libc::c_double }) +
                         11 as libc::c_int as libc::c_double) as size_t,
                    b"make_cc_string\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) as *mut libc::c_char;
    if tag_n < 0 as libc::c_int {
        /* 後処理により併合された基本句 */
        if OptDisplay == 5 as libc::c_int {
            sprintf(buf, b"%s/-\x00" as *const u8 as *const libc::c_char,
                    pp_str);
        } else {
            sprintf(buf,
                    b"%s/U/-/-/-/-\x00" as *const u8 as *const libc::c_char,
                    pp_str);
        }
    } else if OptDisplay == 5 as libc::c_int {
        sprintf(buf, b"%s/%s\x00" as *const u8 as *const libc::c_char, pp_str,
                word);
    } else {
        sprintf(buf,
                b"%s/%c/%s/%d/%d/%s\x00" as *const u8 as *const libc::c_char,
                pp_str,
                if cc_type == -(2 as libc::c_int) {
                    'O' as i32
                } else if cc_type == -(3 as libc::c_int) {
                    'D' as i32
                } else if cc_type == -(1 as libc::c_int) {
                    'N' as i32
                } else { 'C' as i32 }, word, tag_n, dist, sid);
    }
    if word_allocated_flag != 0 { free(word as *mut libc::c_void); }
    return buf;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn append_cf_feature(mut fpp: *mut *mut FEATURE,
                                           mut cpm_ptr: *mut CF_PRED_MGR,
                                           mut cf_ptr: *mut CASE_FRAME,
                                           mut n: libc::c_int) 
 /*==================================================================*/
 {
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    /* 格フレームのガ格が<主体準>をもつかどうか */
    if (*cf_ptr).etcflag & 2 as libc::c_int != 0 &&
           MatchPP((*cf_ptr).pp[n as usize][0 as libc::c_int as usize],
                   b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char) != 0 {
        sprintf(feature_buffer.as_mut_ptr(),
                b"\xef\xbc\xb4\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0-%s-\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00"
                    as *const u8 as *const libc::c_char,
                pp_code_to_kstr_in_context(cpm_ptr,
                                           (*cf_ptr).pp[n as
                                                            usize][0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]));
        assign_cfeature(fpp, feature_buffer.as_mut_ptr(), 0 as libc::c_int);
    } else if cf_match_element((*cf_ptr).sm[n as usize],
                               b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               0 as libc::c_int) != 0 {
        sprintf(feature_buffer.as_mut_ptr(),
                b"\xef\xbc\xb4\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0-%s-\xe4\xb8\xbb\xe4\xbd\x93\x00"
                    as *const u8 as *const libc::c_char,
                pp_code_to_kstr_in_context(cpm_ptr,
                                           (*cf_ptr).pp[n as
                                                            usize][0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]));
        assign_cfeature(fpp, feature_buffer.as_mut_ptr(), 0 as libc::c_int);
    };
    /* 格フレームが<主体>をもつかどうか */
    /* 格フレームが<補文>をもつかどうか *
    if (cf_match_element(cf_ptr->sm[n], "補文", TRUE)) {
	sprintf(feature_buffer, "Ｔ格フレーム-%s-補文", pp_code_to_kstr_in_context(cpm_ptr, cf_ptr->pp[n][0]));
	assign_cfeature(fpp, feature_buffer, FALSE);
    }
    */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_case_component_feature(mut sp: *mut SENTENCE_DATA,
                                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                                       mut temp_assign_flag: libc::c_int)
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 用言の各スロットの割り当てを用言featureに */
    i = 0 as libc::c_int;
    while i < (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num
          {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                               usize].result_lists_p[0 as libc::c_int as
                                                         usize].flag[i as
                                                                         usize];
        /* 割り当てなし */
        if num == -(1 as libc::c_int) {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-%s:NIL\x00" as
                        *const u8 as *const libc::c_char,
                    pp_code_to_kstr_in_context(cpm_ptr,
                                               (*(*cpm_ptr).cmm[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].cf_ptr).pp[i
                                                                                          as
                                                                                          usize][0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     usize]));
        } else if (*cpm_ptr).elem_b_num[num as usize] > -(2 as libc::c_int) {
            word =
                make_print_string((*cpm_ptr).elem_b_ptr[num as usize],
                                  0 as libc::c_int);
            if !word.is_null() {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-%s:%s\x00" as
                            *const u8 as *const libc::c_char,
                        pp_code_to_kstr_in_context(cpm_ptr,
                                                   (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).pp[i as usize][0 as libc::c_int as usize]
                        ),
                        word
                );
                free(word as *mut libc::c_void);
            }
        }
        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                        feature_buffer.as_mut_ptr(), temp_assign_flag);
        i += 1
    };
}
/* 割り当てあり (省略以外の場合) */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cat_case_analysis_result_parallel_child(mut buffer:
                                                                     *mut libc::c_char,
                                                                 mut cpm_ptr:
                                                                     *mut CF_PRED_MGR,
                                                                 mut cf_i:
                                                                     libc::c_int,
                                                                 mut dist_n:
                                                                     libc::c_int,
                                                                 mut sid:
                                                                     *mut libc::c_char) 
 /*==================================================================*/
 {
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: libc::c_int =
        (*cpm_ptr).cmm[0 as libc::c_int as
                           usize].result_lists_p[0 as libc::c_int as
                                                     usize].flag[cf_i as
                                                                     usize];
    /* 並列の子供を取得して、bufferにcat */
    /* 省略の先行詞の場合: elem_b_ptrの親がpara_top_p */
    if ((*cpm_ptr).cf.type_0 == 0) as libc::c_int == 2 as libc::c_int &&
           (*(*cpm_ptr).elem_b_ptr[num as usize]).para_type as libc::c_int ==
               1 as libc::c_int &&
           !(*(*cpm_ptr).elem_b_ptr[num as usize]).parent.is_null() &&
           (*(*(*cpm_ptr).elem_b_ptr[num as usize]).parent).para_top_p as
               libc::c_int != 0 {
        j = 0 as libc::c_int;
        while !(*(*(*cpm_ptr).elem_b_ptr[num as
                                             usize]).parent).child[j as
                                                                       usize].is_null()
              {
            if !((*cpm_ptr).elem_b_ptr[num as usize] ==
                     (*(*(*cpm_ptr).elem_b_ptr[num as
                                                   usize]).parent).child[j as
                                                                             usize]
                     ||
                     (*(*(*(*cpm_ptr).elem_b_ptr[num as
                                                     usize]).parent).child[j
                                                                               as
                                                                               usize]).para_type
                         as libc::c_int != 1 as libc::c_int ||
                     (*(*cpm_ptr).pred_b_ptr).num <
                         (*(*cpm_ptr).elem_b_ptr[num as usize]).num &&
                         ((*(*(*(*cpm_ptr).elem_b_ptr[num as
                                                          usize]).parent).child[j
                                                                                    as
                                                                                    usize]).num
                              < (*(*cpm_ptr).pred_b_ptr).num ||
                              (*(*cpm_ptr).elem_b_ptr[num as usize]).num <
                                  (*(*(*(*cpm_ptr).elem_b_ptr[num as
                                                                  usize]).parent).child[j
                                                                                            as
                                                                                            usize]).num))
               {
                cp =
                    make_cc_string(make_print_string((*(*(*cpm_ptr).elem_b_ptr[num
                                                                                   as
                                                                                   usize]).parent).child[j
                                                                                                             as
                                                                                                             usize],
                                                     0 as libc::c_int),
                                   (*(*(*(*cpm_ptr).elem_b_ptr[num as
                                                                   usize]).parent).child[j
                                                                                             as
                                                                                             usize]).num,
                                   pp_code_to_kstr_in_context(cpm_ptr,
                                                              (*(*cpm_ptr).cmm[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].cf_ptr).pp[cf_i
                                                                                                         as
                                                                                                         usize][0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    usize]),
                                   (*cpm_ptr).elem_b_num[num as usize],
                                   dist_n,
                                   if !sid.is_null() {
                                       sid as *const libc::c_char
                                   } else {
                                       b"?\x00" as *const u8 as
                                           *const libc::c_char
                                   } as *mut libc::c_char);
                strcat(buffer, cp);
                strcat(buffer, b";\x00" as *const u8 as *const libc::c_char);
                free(cp as *mut libc::c_void);
            }
            /* 新たな並列の子が元の子より後はいけない */
            j += 1
        }
    }
    /* 直接の係り受けの場合: elem_b_ptrがpara_top_p */
    if (*(*cpm_ptr).elem_b_ptr[num as usize]).para_top_p != 0 {
        j = 1 as libc::c_int;
        while !(*(*cpm_ptr).elem_b_ptr[num as
                                           usize]).child[j as usize].is_null()
              {
            /* 0は自分と同じでチェックされている */
            if (*(*(*cpm_ptr).elem_b_ptr[num as
                                             usize]).child[j as
                                                               usize]).para_type
                   as libc::c_int == 1 as libc::c_int &&
                   ((*(*cpm_ptr).pred_b_ptr).num >
                        (*(*cpm_ptr).elem_b_ptr[num as usize]).num ||
                        (*(*(*cpm_ptr).elem_b_ptr[num as
                                                      usize]).child[j as
                                                                        usize]).num
                            > (*(*cpm_ptr).pred_b_ptr).num) {
                /* 連体修飾の場合は用言より後のみ */
                cp =
                    make_cc_string(make_print_string((*(*cpm_ptr).elem_b_ptr[num
                                                                                 as
                                                                                 usize]).child[j
                                                                                                   as
                                                                                                   usize],
                                                     0 as libc::c_int),
                                   (*(*(*cpm_ptr).elem_b_ptr[num as
                                                                 usize]).child[j
                                                                                   as
                                                                                   usize]).num,
                                   pp_code_to_kstr_in_context(cpm_ptr,
                                                              (*(*cpm_ptr).cmm[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].cf_ptr).pp[cf_i
                                                                                                         as
                                                                                                         usize][0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    usize]),
                                   (*cpm_ptr).elem_b_num[num as usize],
                                   dist_n,
                                   if !sid.is_null() {
                                       sid as *const libc::c_char
                                   } else {
                                       b"?\x00" as *const u8 as
                                           *const libc::c_char
                                   } as *mut libc::c_char);
                strcat(buffer, cp);
                strcat(buffer, b";\x00" as *const u8 as *const libc::c_char);
                free(cp as *mut libc::c_void);
            }
            j += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_feature_samecase(mut cpm_ptr:
                                                     *mut CF_PRED_MGR,
                                                 mut temp_assign_flag:
                                                     libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut case_str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut case_str2: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while (*(*cpm_ptr).cmm[0 as libc::c_int as
                               usize].cf_ptr).samecase[i as
                                                           usize][0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
              != -(10 as libc::c_int) {
        /* 格が存在し、「修飾」ではない */
        case_str1 =
            pp_code_to_kstr((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                 usize].cf_ptr).samecase[i as
                                                                             usize][0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        usize]);
        if !case_str1.is_null() &&
               strcmp(case_str1,
                      b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                          *const libc::c_char) != 0 {
            case_str2 =
                pp_code_to_kstr((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                     usize].cf_ptr).samecase[i
                                                                                 as
                                                                                 usize][1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            usize]);
            if !case_str2.is_null() &&
                   strcmp(case_str2,
                          b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                              *const libc::c_char) != 0 {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"\xe9\xa1\x9e\xe4\xbc\xbc\xe6\xa0\xbc;%s\xef\xbc\x9d%s\x00"
                            as *const u8 as *const libc::c_char, case_str1,
                        case_str2);
                assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                feature_buffer.as_mut_ptr(),
                                temp_assign_flag);
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_aligned_case(mut cf_align: *mut CF_ALIGNMENT,
                                           mut src_case: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (*cf_align).aligned_case[i as usize][0 as libc::c_int as usize] !=
              -(10 as libc::c_int) {
        if (*cf_align).aligned_case[i as usize][0 as libc::c_int as usize] ==
               src_case {
            return (*cf_align).aligned_case[i as
                                                usize][1 as libc::c_int as
                                                           usize]
        }
        i += 1
    }
    return src_case;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckEllipsisComponent(mut ccp:
                                                    *mut ELLIPSIS_COMPONENT,
                                                mut pp_str: *mut libc::c_char)
 -> *mut ELLIPSIS_COMPONENT 
 /*==================================================================*/
 {
    if pp_str.is_null() {
        return ccp
    } else {
        while !ccp.is_null() {
            if !(*ccp).pp_str.is_null() && strcmp((*ccp).pp_str, pp_str) == 0
               {
                return ccp
            }
            ccp = (*ccp).next
        }
    }
    return 0 as *mut ELLIPSIS_COMPONENT;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _retrieve_parent_case_component(mut sp:
                                                             *mut SENTENCE_DATA,
                                                         mut pred_ptr:
                                                             *mut TAG_DATA,
                                                         mut target_case_num:
                                                             libc::c_int,
                                                         mut case_str:
                                                             *mut libc::c_char)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut case_num: libc::c_int = 0;
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut parent_ptr: *mut TAG_DATA = (*pred_ptr).parent;
    let mut elem_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    while !parent_ptr.is_null() &&
              !check_feature((*parent_ptr).f,
                             b"\xe6\xa9\x9f\xe8\x83\xbd\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        if !(*parent_ptr).cpm_ptr.is_null() {
            cpm_ptr = (*parent_ptr).cpm_ptr;
            /* それぞれの格要素 */
            i = 0 as libc::c_int;
            while i <
                      (*(*cpm_ptr).cmm[0 as libc::c_int as
                                           usize].cf_ptr).element_num {
                num =
                    (*cpm_ptr).cmm[0 as libc::c_int as
                                       usize].result_lists_p[0 as libc::c_int
                                                                 as
                                                                 usize].flag[i
                                                                                 as
                                                                                 usize];
                if num != -(1 as libc::c_int) {
                    /* 割り当てあり */
                    case_num =
                        (*(*cpm_ptr).cmm[0 as libc::c_int as
                                             usize].cf_ptr).pp[i as
                                                                   usize][0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize];
                    if case_num == target_case_num {
                        /* 元の格と同じ格 */
                        elem_b_ptr = (*cpm_ptr).elem_b_ptr[num as usize];
                        if (*elem_b_ptr).num != (*pred_ptr).num {
                            /* 述語と同じならダメ */
                            return make_cc_string(make_print_string(elem_b_ptr,
                                                                    0 as
                                                                        libc::c_int),
                                                  (*elem_b_ptr).num, case_str,
                                                  (*cpm_ptr).elem_b_num[num as
                                                                            usize],
                                                  0 as libc::c_int,
                                                  if !(*sp).KNPSID.is_null() {
                                                      (*sp).KNPSID.offset(5 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                                                          as
                                                          *const libc::c_char
                                                  } else {
                                                      b"?\x00" as *const u8 as
                                                          *const libc::c_char
                                                  } as *mut libc::c_char)
                        }
                    }
                }
                i += 1
            }
        }
        parent_ptr = (*parent_ptr).parent
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn retrieve_parent_case_component(mut sp:
                                                            *mut SENTENCE_DATA,
                                                        mut pred_ptr:
                                                            *mut TAG_DATA,
                                                        mut target_case_num:
                                                            libc::c_int,
                                                        mut case_str:
                                                            *mut libc::c_char)
 -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 「ガ」のときはまず「ガ２」を探しにいく */
    return if MatchPP(target_case_num,
                      b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char) != 0 &&
        {
            cp =
                _retrieve_parent_case_component(sp, pred_ptr,
                                                pp_kstr_to_code(b"\xe3\x82\xac\xef\xbc\x92\x00"
                                                    as
                                                    *const u8
                                                    as
                                                    *const libc::c_char
                                                    as
                                                    *mut libc::c_char),
                                                case_str);
            !cp.is_null()
        } {
        cp
    } else {
        _retrieve_parent_case_component(sp, pred_ptr, target_case_num,
                                        case_str)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn record_case_analysis_result(mut sp:
                                                         *mut SENTENCE_DATA,
                                                     mut cpm_ptr:
                                                         *mut CF_PRED_MGR,
                                                     mut em_ptr:
                                                         *mut ELLIPSIS_MGR,
                                                     mut temp_assign_flag:
                                                         libc::c_int,
                                                     mut feature_head:
                                                         *mut libc::c_char,
                                                     mut cf_align:
                                                         *mut CF_ALIGNMENT) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut dist_n: libc::c_int = 0;
    let mut sent_n: libc::c_int = 0;
    let mut tag_n: libc::c_int = 0;
    let mut first_arg_flag: libc::c_int = 1 as libc::c_int;
    let mut case_num: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut buffer: [libc::c_char; 5120] = [0; 5120];
    let mut sid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut case_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccp: *mut ELLIPSIS_COMPONENT = 0 as *mut ELLIPSIS_COMPONENT;
    let mut elem_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut pred_b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    /* 格フレーム側からの記述
       => ★「格要素-ガ」などを集めるように修正する */
    /* 述語が後処理により併合された場合: 併合された先の基本句を探す */
    while (*pred_b_ptr).num < 0 as libc::c_int {
        pred_b_ptr = pred_b_ptr.offset(-1)
    }
    /* 格フレームID */
    sprintf(feature_buffer.as_mut_ptr(),
            b"%s\x00" as *const u8 as *const libc::c_char, feature_head);
    if OptDisplay != 5 as libc::c_int {
        strcat(feature_buffer.as_mut_ptr(),
               b":\x00" as *const u8 as *const libc::c_char);
        strcat(feature_buffer.as_mut_ptr(),
               if !cf_align.is_null() {
                   (*cf_align).cf_id
               } else {
                   (*(*cpm_ptr).cmm[0 as libc::c_int as
                                        usize].cf_ptr).cf_id.as_mut_ptr()
               });
    }
    /* それぞれの格要素 */
    i = 0 as libc::c_int;
    while i < (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num
          {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                               usize].result_lists_p[0 as libc::c_int as
                                                         usize].flag[i as
                                                                         usize];
        ccp =
            if !em_ptr.is_null() {
                CheckEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*(*(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)).cf_ptr).pp.as_mut_ptr().offset(i
                                                                                                                                                                       as
                                                                                                                                                                       isize)).as_mut_ptr().offset(0
                                                                                                                                                                                                       as
                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                       as
                                                                                                                                                                                                       isize)
                                                                                  as
                                                                                  isize),
                                       (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                            usize].cf_ptr).pp_str[i
                                                                                      as
                                                                                      usize])
            } else { 0 as *mut ELLIPSIS_COMPONENT };
        case_num =
            if !cf_align.is_null() {
                find_aligned_case(cf_align,
                                  (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                       usize].cf_ptr).pp[i as
                                                                             usize][0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        usize])
            } else {
                (*(*cpm_ptr).cmm[0 as libc::c_int as
                                     usize].cf_ptr).pp[i as
                                                           usize][0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
            };
        if !(case_num == -(10 as libc::c_int)) {
            case_str = pp_code_to_kstr_in_context(cpm_ptr, case_num);
            /* 割り当てなし */
            if num == -(1 as libc::c_int) {
                /* 正規化時は割り当てなしを表示しない, 通常は必須格のみ(-print-case-all-slot時はすべて) */
                if cf_align.is_null() &&
                       (OptCaseFlag & 8388608 as libc::c_int != 0 ||
                            (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                 usize].cf_ptr).oblig[i as
                                                                          usize]
                                == (0 as libc::c_int == 0) as libc::c_int &&
                                MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                             usize].cf_ptr).pp[i
                                                                                   as
                                                                                   usize][0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize],
                                        b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char) == 0 &&
                                MatchPP((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                             usize].cf_ptr).pp[i
                                                                                   as
                                                                                   usize][0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize],
                                        b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) == 0) {
                    if first_arg_flag != 0 {
                        /* 格フレームIDの後の":" */
                        strcat(feature_buffer.as_mut_ptr(),
                               b":\x00" as *const u8 as *const libc::c_char);
                    } else {
                        /* 2つ目以降の格要素なら区切り";"を出力 */
                        strcat(feature_buffer.as_mut_ptr(),
                               b";\x00" as *const u8 as *const libc::c_char);
                    }
                    first_arg_flag = 0 as libc::c_int;
                    if OptDisplay == 5 as libc::c_int {
                        sprintf(buffer.as_mut_ptr(),
                                b"%s/-\x00" as *const u8 as
                                    *const libc::c_char, case_str);
                    } else {
                        sprintf(buffer.as_mut_ptr(),
                                b"%s/U/-/-/-/-\x00" as *const u8 as
                                    *const libc::c_char, case_str);
                    }
                    /* 割り当てない場合に親(機能的基本句)から項を取得する */
                    if OptCaseFlag & 16777216 as libc::c_int != 0 &&
                           check_feature((*pred_b_ptr).f,
                                         b"\xe6\xa9\x9f\xe8\x83\xbd\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() &&
                           {
                               cp =
                                   retrieve_parent_case_component(sp,
                                                                  pred_b_ptr,
                                                                  case_num,
                                                                  case_str);
                               !cp.is_null()
                           } {
                        strcat(feature_buffer.as_mut_ptr(), cp);
                        free(cp as *mut libc::c_void);
                    } else {
                        strcat(feature_buffer.as_mut_ptr(),
                               buffer.as_mut_ptr());
                    }
                }
            } else {
                /* 割り当てあり */
                if first_arg_flag != 0 {
                    /* 格フレームIDの後の":" */
                    strcat(feature_buffer.as_mut_ptr(),
                           b":\x00" as *const u8 as *const libc::c_char);
                } else {
                    /* 2つ目以降の格要素なら区切り";"を出力 */
                    strcat(feature_buffer.as_mut_ptr(),
                           b";\x00" as *const u8 as *const libc::c_char);
                }
                first_arg_flag = 0 as libc::c_int;
                /* 例外タグ */
                if (*cpm_ptr).elem_b_num[num as usize] <= -(2 as libc::c_int)
                       && (*cpm_ptr).elem_s_ptr[num as usize].is_null() {
                    if OptDisplay == 5 as libc::c_int { /* 不特定-人 */
                        sprintf(buffer.as_mut_ptr(),
                                b"%s/%s\x00" as *const u8 as
                                    *const libc::c_char, case_str,
                                *ETAG_name.as_mut_ptr().offset(2 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)); /* 不特定-人 */
                    } else {
                        sprintf(buffer.as_mut_ptr(),
                                b"%s/E/%s/-/-/-\x00" as *const u8 as
                                    *const libc::c_char, case_str,
                                *ETAG_name.as_mut_ptr().offset(2 as
                                                                   libc::c_int
                                                                   as isize));
                    }
                    strcat(feature_buffer.as_mut_ptr(), buffer.as_mut_ptr());
                } else {
                    /* 省略の場合 (特殊タグ以外) */
                    if (*cpm_ptr).elem_b_num[num as usize] <=
                           -(2 as libc::c_int) {
                        sid =
                            if !(*(*cpm_ptr).elem_s_ptr[num as
                                                            usize]).KNPSID.is_null()
                               {
                                (*(*cpm_ptr).elem_s_ptr[num as
                                                            usize]).KNPSID.offset(5
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)
                            } else { 0 as *mut libc::c_char };
                        dist_n =
                            (*sp).Sen_num -
                                (*(*cpm_ptr).elem_s_ptr[num as
                                                            usize]).Sen_num;
                        sent_n =
                            (*(*cpm_ptr).elem_s_ptr[num as usize]).Sen_num
                    } else {
                        /* 同文内 */
                        sid =
                            if !(*sp).KNPSID.is_null() {
                                (*sp).KNPSID.offset(5 as libc::c_int as isize)
                            } else { 0 as *mut libc::c_char };
                        dist_n = 0 as libc::c_int;
                        sent_n = (*sp).Sen_num
                    }
                    /* 並列の子供 */
                    cat_case_analysis_result_parallel_child(feature_buffer.as_mut_ptr(),
                                                            cpm_ptr, i,
                                                            dist_n, sid);
                    if (*(*cpm_ptr).elem_b_ptr[num as usize]).num <
                           0 as libc::c_int {
                        /* 後処理により併合された基本句 */
                        elem_b_ptr = (*cpm_ptr).elem_b_ptr[num as usize];
                        while (*elem_b_ptr).num < 0 as libc::c_int {
                            elem_b_ptr = elem_b_ptr.offset(-1)
                        }
                        if (*elem_b_ptr).num >= (*pred_b_ptr).num {
                            /* 併合された連体修飾は用言自身になってしまうので非表示 */
                            tag_n = -(1 as libc::c_int)
                        } else { tag_n = (*elem_b_ptr).num }
                    } else {
                        elem_b_ptr = (*cpm_ptr).elem_b_ptr[num as usize];
                        tag_n = (*elem_b_ptr).num
                    }
                    cp =
                        make_cc_string(make_print_string(elem_b_ptr,
                                                         0 as libc::c_int),
                                       tag_n, case_str,
                                       (*cpm_ptr).elem_b_num[num as usize],
                                       dist_n,
                                       if !sid.is_null() {
                                           sid as *const libc::c_char
                                       } else {
                                           b"?\x00" as *const u8 as
                                               *const libc::c_char
                                       } as *mut libc::c_char);
                    strcat(feature_buffer.as_mut_ptr(), cp);
                    free(cp as *mut libc::c_void);
                    /* 格・省略関係の保存 (文脈解析用) */
                    if OptEllipsis != 0 {
                        context::RegisterTagTarget((*(*pred_b_ptr).head_ptr).Goi.as_mut_ptr(),
                                          (*pred_b_ptr).voice,
                                          (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).cf_address,
                                          (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).pp[i as usize][0 as libc::c_int  as usize],
                                          if (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).type_0
                                                 == 2 as libc::c_int {
                                              (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).pp_str[i as usize]
                                          } else { 0 as *mut libc::c_char },
                                          make_print_string(elem_b_ptr,
                                                            0 as libc::c_int),
                                          sent_n, tag_n, 1 as libc::c_int);
                    }
                }
            }
        }
        /* 正規化時に、対応先の格がない(NIL)場合 */
        i += 1
    }
    assign_cfeature(&mut (*pred_b_ptr).f, feature_buffer.as_mut_ptr(),
                    temp_assign_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn record_case_analysis(mut sp: *mut SENTENCE_DATA,
                                              mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut em_ptr: *mut ELLIPSIS_MGR,
                                              mut temp_assign_flag:
                                                  libc::c_int) 
 /*==================================================================*/
 {
    /* temp_assign_flag: TRUEのときfeatureを「仮付与」する */
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut relation: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut case_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut elem_b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut pred_b_ptr: *mut TAG_DATA = (*cpm_ptr).pred_b_ptr;
    /* 述語が後処理により併合された場合: 併合された先の基本句を探す */
    while (*pred_b_ptr).num < 0 as libc::c_int {
        pred_b_ptr = pred_b_ptr.offset(-1)
    }
    /* voice 決定 */
    if (*pred_b_ptr).voice == 32 as libc::c_int { decide_voice(sp, cpm_ptr); }
    /* 主節かどうかチェック
    check_feature(pred_b_ptr->f, "主節")
    */
    /* 「格フレーム変化」フラグがついている格フレームを使用した場合 */
    if (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).etcflag &
           4 as libc::c_int != 0 {
        assign_cfeature(&mut (*pred_b_ptr).f,
                        b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe5\xa4\x89\xe5\x8c\x96\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char, temp_assign_flag);
    }
    /* 類似格をfeatureに */
    assign_feature_samecase(cpm_ptr, temp_assign_flag);
    let mut current_block_39: u64;
    /* 入力側の各格要素の記述 */
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        /* 省略解析の結果は除く
	   指示詞の解析をする場合は、指示詞を除く */
        if !((*cpm_ptr).elem_b_num[i as usize] <= -(2 as libc::c_int)) {
            /* 後処理により併合された連体修飾詞を除く */
            if (*(*cpm_ptr).elem_b_ptr[i as usize]).num < 0 as libc::c_int {
                elem_b_ptr = (*cpm_ptr).elem_b_ptr[i as usize];
                while (*elem_b_ptr).num < 0 as libc::c_int {
                    elem_b_ptr = elem_b_ptr.offset(-1)
                }
                if (*elem_b_ptr).num >= (*pred_b_ptr).num {
                    current_block_39 = 7651349459974463963;
                } else { current_block_39 = 10652014663920648156; }
            } else {
                elem_b_ptr = (*cpm_ptr).elem_b_ptr[i as usize];
                current_block_39 = 10652014663920648156;
            }
            match current_block_39 {
                7651349459974463963 => { }
                _ => {
                    num =
                        (*cpm_ptr).cmm[0 as libc::c_int as
                                           usize].result_lists_d[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].flag[i
                                                                                     as
                                                                                     usize];
                    /* 割り当てなし */
                    if !(num == -(2 as libc::c_int)) {
                        /* 割り当てられている格 */
                        if num >= 0 as libc::c_int {
                            case_str =
                                pp_code_to_kstr_in_context(cpm_ptr,
                                                           (*(*cpm_ptr).cmm[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize].cf_ptr).pp[num
                                                                                                      as
                                                                                                      usize][0
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 usize]);
                            relation =
                                malloc_data(strlen(case_str).wrapping_add(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong),
                                            b"record_case_analysis\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) as
                                    *mut libc::c_char;
                            strcpy(relation, case_str);
                            if OptCaseFlag & 4194304 as libc::c_int != 0 &&
                                   (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                        usize].cf_ptr).oblig[num
                                                                                 as
                                                                                 usize]
                                       == 0 as libc::c_int {
                                /* 任意格情報を表示する場合 */
                                strcat(relation,
                                       b"*\x00" as *const u8 as
                                           *const libc::c_char);
                            }
                        }
                        /* else: UNASSIGNED はないはず */
                        /* featureを格要素文節に与える */
                        if (*elem_b_ptr).num < (*pred_b_ptr).num {
                            sprintf(feature_buffer.as_mut_ptr(),
                                    b"\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc:%s\x00"
                                        as *const u8 as *const libc::c_char,
                                    relation);
                        } else {
                            sprintf(feature_buffer.as_mut_ptr(),
                                    b"\xe8\xa7\xa3\xe6\x9e\x90\xe9\x80\xa3\xe6\xa0\xbc:%s\x00"
                                        as *const u8 as *const libc::c_char,
                                    relation);
                        }
                        assign_cfeature(&mut (*elem_b_ptr).f,
                                        feature_buffer.as_mut_ptr(),
                                        temp_assign_flag);
                        /* feature を用言文節に与える */
                        word =
                            make_print_string(elem_b_ptr, 0 as libc::c_int);
                        if !word.is_null() {
                            if OptCaseFlag & 2097152 as libc::c_int != 0 {
                                /* 格ごとのスコアを出す場合 */
                                sprintf(feature_buffer.as_mut_ptr(),
                                        b"\xe6\xa0\xbc\xe9\x96\xa2\xe4\xbf\x82%d:%s:%s:%.3f\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        if (*elem_b_ptr).num >=
                                               0 as libc::c_int {
                                            (*elem_b_ptr).num
                                        } else {
                                            (*(*elem_b_ptr).parent).num
                                        }, relation, word,
                                        (*cpm_ptr).cmm[0 as libc::c_int as
                                                           usize].result_lists_d[0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize].score[i
                                                                                                      as
                                                                                                      usize]);
                            } else {
                                sprintf(feature_buffer.as_mut_ptr(),
                                        b"\xe6\xa0\xbc\xe9\x96\xa2\xe4\xbf\x82%d:%s:%s\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        if (*elem_b_ptr).num >=
                                               0 as libc::c_int {
                                            (*elem_b_ptr).num
                                        } else {
                                            (*(*elem_b_ptr).parent).num
                                        }, relation, word);
                            }
                            assign_cfeature(&mut (*pred_b_ptr).f,
                                            feature_buffer.as_mut_ptr(),
                                            temp_assign_flag);
                            free(word as *mut libc::c_void);
                        }
                        free(relation as *mut libc::c_void);
                    }
                }
            }
        }
        /* 連体修飾 */
        i += 1
    }
    /* 格フレーム側からの格解析結果の記述 */
    record_case_analysis_result(sp, cpm_ptr, em_ptr, temp_assign_flag,
                                b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                0 as *mut CF_ALIGNMENT);
    /* 格解析の結果、決定した格フレームIDから標準用言代表表記を生成 */
    sprintf(feature_buffer.as_mut_ptr(),
            b"\xe6\xa8\x99\xe6\xba\x96\xe7\x94\xa8\xe8\xa8\x80\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:%s\x00"
                as *const u8 as *const libc::c_char,
            (*(*cpm_ptr).cmm[0 as libc::c_int as
                                 usize].cf_ptr).cf_id.as_mut_ptr());
    cp = strrchr(feature_buffer.as_mut_ptr(), ':' as i32);
    if !cp.is_null() {
        /* 格フレーム番号を削除 */
        *cp = '\u{0}' as i32 as libc::c_char
    }
    assign_cfeature(&mut (*pred_b_ptr).f, feature_buffer.as_mut_ptr(),
                    temp_assign_flag);
    /* 正規化格解析結果 */
    i = 0 as libc::c_int;
    while !(*(*cpm_ptr).cmm[0 as libc::c_int as
                                usize].cf_ptr).cf_align[i as
                                                            usize].cf_id.is_null()
          {
        sprintf(feature_buffer.as_mut_ptr(),
                b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c-%d\x00"
                    as *const u8 as *const libc::c_char, i);
        record_case_analysis_result(sp, cpm_ptr, em_ptr, temp_assign_flag,
                                    feature_buffer.as_mut_ptr(),
                                    &mut *(*(*(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).cf_ptr).cf_align.as_mut_ptr().offset(i
                                                                                                                                      as
                                                                                                                                      isize));
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn record_all_case_analisys(mut sp: *mut SENTENCE_DATA,
                                                  mut temp_assign_flag:
                                                      libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*sp).Best_mgr).pred_num {
        if !(*(*sp).Best_mgr).cpm[i as usize].pred_b_ptr.is_null() {
            if OptCaseFlag & 16 as libc::c_int != 0 &&
                   (*(*sp).Best_mgr).cpm[i as
                                             usize].cmm[0 as libc::c_int as
                                                            usize].score !=
                       -(1001 as libc::c_int) as libc::c_double ||
                   OptCaseFlag & 16 as libc::c_int == 0 &&
                       (*(*sp).Best_mgr).cpm[i as
                                                 usize].cmm[0 as libc::c_int
                                                                as
                                                                usize].score
                           != -(2 as libc::c_int) as libc::c_double {
                record_case_analysis(sp,
                                     &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                                                                         as
                                                                                         isize),
                                     0 as *mut ELLIPSIS_MGR,
                                     temp_assign_flag);
            }
        }
        /* 述語ではないと判断したものはスキップ */
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn decide_alt_mrph(mut m_ptr: *mut MRPH_DATA,
                                         mut alt_num: libc::c_int,
                                         mut f_str: *mut libc::c_char) 
 /*==================================================================*/
 {
    if alt_num == 0 as libc::c_int {
        assign_cfeature(&mut (*m_ptr).f, f_str, 0 as libc::c_int);
    } else {
        let mut alt_count: libc::c_int = 1 as libc::c_int;
        let mut fp: *mut FEATURE = (*m_ptr).f;
        let mut m: MRPH_DATA =
            MRPH_DATA{type_0: 0,
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
                      Type: [0; 9],};
        /* ALTをチェック */
        while !fp.is_null() {
            if strncmp((*fp).cp,
                       b"ALT-\x00" as *const u8 as *const libc::c_char,
                       4 as libc::c_int as libc::c_ulong) == 0 {
                if alt_count == alt_num {
                    /* target */
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
                    /* 現在の形態素をALTに保存 */
                    assign_feature_alt_mrph(&mut (*m_ptr).f, m_ptr);
                    /* このALTを最終結果の形態素にする */
                    delete_existing_features(m_ptr); /* 現在の形態素featureを削除 */
                    copy_mrph(m_ptr, &mut m,
                              (0 as libc::c_int == 0) as libc::c_int);
                    delete_cfeature(&mut (*m_ptr).f, (*fp).cp);
                    assign_cfeature(&mut (*m_ptr).f, f_str, 0 as libc::c_int);
                    break ;
                } else { alt_count += 1 }
            }
            fp = (*fp).next
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _noun_lexical_disambiguation_by_case_analysis(mut cpm_ptr:
                                                                           *mut CF_PRED_MGR,
                                                                       mut i:
                                                                           libc::c_int,
                                                                       mut exact_flag:
                                                                           libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 格解析結果から名詞の曖昧性解消を行う

    対象の形態素: cpm_ptr->elem_b_ptr[i]->head_ptr */
    let mut num: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut expand: libc::c_int = 0;
    let mut alt_num: libc::c_int = 0;
    let mut alt_count: libc::c_int = 0;
    let mut rep_length: libc::c_int = 0;
    let mut rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score: libc::c_float = 0.;
    let mut tmp_score: libc::c_float = 0.;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut m: MRPH_DATA =
        MRPH_DATA{type_0: 0,
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
                  Type: [0; 9],};
    num =
        (*cpm_ptr).cmm[0 as libc::c_int as
                           usize].result_lists_d[0 as libc::c_int as
                                                     usize].flag[i as usize];
    alt_num = -(1 as libc::c_int);
    score = 0 as libc::c_int as libc::c_float;
    alt_count = 0 as libc::c_int;
    if exact_flag == 0 {
        if !check_feature((*(*cpm_ptr).elem_b_ptr[i as usize]).f,
                          b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            expand = 1 as libc::c_int
        } else { expand = 2 as libc::c_int }
    }
    /* まず現在の形態素をチェック */
    if OptCaseFlag & 32 as libc::c_int != 0 {
        rep_strt =
            get_mrph_rep((*(*cpm_ptr).elem_b_ptr[i as
                                                     usize]).head_ptr); /* 代表表記 */
        rep_length = get_mrph_rep_length(rep_strt);
        if rep_length == 0 as libc::c_int {
            /* なければ作る */
            rep_strt =
                make_mrph_rn((*(*cpm_ptr).elem_b_ptr[i as usize]).head_ptr);
            rep_length = strlen(rep_strt) as libc::c_int;
            rep_malloc_flag = 1 as libc::c_int
        }
    } else {
        rep_strt =
            (*(*(*cpm_ptr).elem_b_ptr[i as usize]).head_ptr).Goi.as_mut_ptr();
        rep_length = strlen(rep_strt) as libc::c_int
    }
    if !rep_strt.is_null() && rep_length != 0 {
        if exact_flag != 0 {
            /* exact matchによるチェック */
            if cf_match_exactly(rep_strt, rep_length,
                                (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                     usize].cf_ptr).ex_list[num
                                                                                as
                                                                                usize],
                                (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                     usize].cf_ptr).ex_num[num
                                                                               as
                                                                               usize],
                                &mut pos) != 0 {
                score =
                    *(*(*cpm_ptr).cmm[0 as libc::c_int as
                                          usize].cf_ptr).ex_freq[num as
                                                                     usize].offset(pos
                                                                                       as
                                                                                       isize)
                        as libc::c_float;
                alt_num = alt_count
                /* 0 */
            }
        } else {
            /* 意味素によるチェック */
            exd = get_str_code_with_len(rep_strt, rep_length, Thesaurus);
            if !exd.is_null() {
                score =
                    _calc_similarity_sm_cf(exd, expand,
                                           (*(*(*cpm_ptr).elem_b_ptr[i as
                                                                         usize]).head_ptr).Goi2.as_mut_ptr(),
                                           (*cpm_ptr).cmm[0 as libc::c_int as
                                                              usize].cf_ptr,
                                           num, &mut pos);
                if score > 0 as libc::c_int as libc::c_float {
                    alt_num = alt_count
                    /* 0 */
                }
                free(exd as *mut libc::c_void);
            }
        }
    }
    if rep_malloc_flag != 0 { free(rep_strt as *mut libc::c_void); }
    /* ALTをチェック */
    alt_count += 1; /* 代表表記 */
    fp = (*(*(*cpm_ptr).elem_b_ptr[i as usize]).head_ptr).f;
    while !fp.is_null() {
        if strncmp((*fp).cp, b"ALT-\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 {
            rep_malloc_flag = 0 as libc::c_int;
            sscanf((*fp).cp.offset(4 as libc::c_int as isize),
                   b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00" as *const u8 as
                       *const libc::c_char, m.Goi2.as_mut_ptr(),
                   m.Yomi.as_mut_ptr(), m.Goi.as_mut_ptr(),
                   &mut m.Hinshi as *mut libc::c_int,
                   &mut m.Bunrui as *mut libc::c_int,
                   &mut m.Katuyou_Kata as *mut libc::c_int,
                   &mut m.Katuyou_Kei as *mut libc::c_int,
                   m.Imi.as_mut_ptr());
            if OptCaseFlag & 32 as libc::c_int != 0 {
                rep_strt = get_mrph_rep(&mut m);
                rep_length = get_mrph_rep_length(rep_strt);
                if rep_length == 0 as libc::c_int {
                    /* なければ作る */
                    rep_strt = make_mrph_rn(&mut m);
                    rep_length = strlen(rep_strt) as libc::c_int;
                    rep_malloc_flag = 1 as libc::c_int
                }
            } else {
                rep_strt = m.Goi.as_mut_ptr();
                rep_length = strlen(rep_strt) as libc::c_int
            }
            if !rep_strt.is_null() && rep_length != 0 {
                if exact_flag != 0 {
                    /* exact matchによるチェック */
                    if cf_match_exactly(rep_strt, rep_length,
                                        (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                             usize].cf_ptr).ex_list[num
                                                                                        as
                                                                                        usize],
                                        (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                             usize].cf_ptr).ex_num[num
                                                                                       as
                                                                                       usize],
                                        &mut pos) != 0 {
                        tmp_score =
                            *(*(*cpm_ptr).cmm[0 as libc::c_int as
                                                  usize].cf_ptr).ex_freq[num
                                                                             as
                                                                             usize].offset(pos
                                                                                               as
                                                                                               isize)
                                as libc::c_float;
                        if score < tmp_score {
                            score = tmp_score;
                            alt_num = alt_count
                        }
                    }
                } else {
                    /* 意味素によるチェック */
                    exd =
                        get_str_code_with_len(rep_strt, rep_length,
                                              Thesaurus);
                    if !exd.is_null() {
                        tmp_score =
                            _calc_similarity_sm_cf(exd, expand,
                                                   (*(*(*cpm_ptr).elem_b_ptr[i
                                                                                 as
                                                                                 usize]).head_ptr).Goi2.as_mut_ptr(),
                                                   (*cpm_ptr).cmm[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].cf_ptr,
                                                   num, &mut pos);
                        if score < tmp_score {
                            score = tmp_score;
                            alt_num = alt_count
                        }
                        free(exd as *mut libc::c_void);
                    }
                }
            }
            if rep_malloc_flag != 0 { free(rep_strt as *mut libc::c_void); }
            alt_count += 1
        }
        fp = (*fp).next
    }
    /* 決定 */
    if alt_num > -(1 as libc::c_int) {
        decide_alt_mrph((*(*cpm_ptr).elem_b_ptr[i as usize]).head_ptr,
                        alt_num,
                        b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char);
        return (0 as libc::c_int == 0) as libc::c_int
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn noun_lexical_disambiguation_by_case_analysis(mut cpm_ptr:
                                                                          *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if !((*cpm_ptr).elem_b_ptr[i as usize].is_null() ||
                 check_feature((*(*(*cpm_ptr).elem_b_ptr[i as
                                                             usize]).head_ptr).f,
                               b"\xe5\x93\x81\xe6\x9b\x96\x00" as *const u8 as
                                   *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                 !check_feature((*(*(*cpm_ptr).elem_b_ptr[i as
                                                              usize]).head_ptr).f,
                                b"\xe7\x94\xa8\xe8\xa8\x80\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char).is_null()) {
            /* 省略の格要素、格フレームとあまりマッチしないときなどは対象としない */
            if !((*cpm_ptr).elem_b_num[i as usize] < -(1 as libc::c_int) ||
                     (*cpm_ptr).cmm[0 as libc::c_int as
                                        usize].result_lists_d[0 as libc::c_int
                                                                  as
                                                                  usize].flag[i
                                                                                  as
                                                                                  usize]
                         < 0 as libc::c_int ||
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
                         == -(1 as libc::c_int) ||
                     OptCaseFlag & 16 as libc::c_int != 0 &&
                         (*cpm_ptr).cmm[0 as libc::c_int as
                                            usize].result_lists_d[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].score[i
                                                                                       as
                                                                                       usize]
                             < -13.815511f64 ||
                     OptCaseFlag & 16 as libc::c_int == 0 &&
                         (*cpm_ptr).cmm[0 as libc::c_int as
                                            usize].result_lists_d[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].score[i
                                                                                       as
                                                                                       usize]
                             <= 7 as libc::c_int as libc::c_double ||
                     !check_feature((*(*(*cpm_ptr).elem_b_ptr[i as
                                                                  usize]).head_ptr).f,
                                    b"\xe9\x9f\xb3\xe8\xa8\x93\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char).is_null()) {
                /* exactマッチをチェックして名詞の曖昧性解消 */
                if _noun_lexical_disambiguation_by_case_analysis(cpm_ptr, i,
                                                                 1 as
                                                                     libc::c_int)
                       == 0 {
                    /* マッチした意味素をもとに名詞の曖昧性解消 */
                    _noun_lexical_disambiguation_by_case_analysis(cpm_ptr, i,
                                                                  0 as
                                                                      libc::c_int);
                }
            }
        }
        /*音訓解消はされていない */
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn verb_lexical_disambiguation_by_case_analysis(mut cpm_ptr:
                                                                          *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    /* 格解析結果から用言の曖昧性解消を行う */
    let mut rep_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut m: MRPH_DATA =
        MRPH_DATA{type_0: 0,
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
                  Type: [0; 9],};
    /* 直前格が1つ以上割り当てられていることを条件とする */
    if count_assigned_adjacent_element((*cpm_ptr).cmm[0 as libc::c_int as
                                                          usize].cf_ptr,
                                       &mut *(*(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).result_lists_p.as_mut_ptr().offset(0
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     isize))
           != 0 &&
           (!check_feature((*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                           b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
                check_str_type((*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr()
                                   as *mut libc::c_uchar, 2 as libc::c_int,
                               0 as libc::c_int) != 0 &&
                    !check_feature((*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                                   b"\xe5\x93\x81\xe6\x9b\x96\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) &&
           check_feature((*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                         b"\xe9\x9f\xb3\xe8\xa8\x93\xe8\xa7\xa3\xe6\xb6\x88\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() {
        /*音訓解消はされていない */
        /* 現在の形態素でよいとき */
        rep_cp =
            get_mrph_rep((*(*cpm_ptr).pred_b_ptr).head_ptr); /* あれば削除 */
        if !rep_cp.is_null() &&
               strncmp(rep_cp,
                       (*(*cpm_ptr).cmm[0 as libc::c_int as
                                            usize].cf_ptr).entry,
                       strlen((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                   usize].cf_ptr).entry)) == 0
           {
            assign_cfeature(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                            b"\xe7\x94\xa8\xe8\xa8\x80\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            delete_cfeature(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                            b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char);
            return
        }
        /* 現在の形態素代表表記と格フレームの表記が異なる場合のみ形態素を変更 */
        fp = (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f;
        while !fp.is_null() {
            if strncmp((*fp).cp,
                       b"ALT-\x00" as *const u8 as *const libc::c_char,
                       4 as libc::c_int as libc::c_ulong) == 0 {
                sscanf((*fp).cp.offset(4 as libc::c_int as isize),
                       b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00" as
                           *const u8 as *const libc::c_char,
                       m.Goi2.as_mut_ptr(), m.Yomi.as_mut_ptr(),
                       m.Goi.as_mut_ptr(), &mut m.Hinshi as *mut libc::c_int,
                       &mut m.Bunrui as *mut libc::c_int,
                       &mut m.Katuyou_Kata as *mut libc::c_int,
                       &mut m.Katuyou_Kei as *mut libc::c_int,
                       m.Imi.as_mut_ptr());
                rep_cp = get_mrph_rep(&mut m);
                /* 選択した格フレームの表記と一致する代表表記をもつ形態素を選択 */
                if !rep_cp.is_null() &&
                       strncmp(rep_cp,
                               (*(*cpm_ptr).cmm[0 as libc::c_int as
                                                    usize].cf_ptr).entry,
                               strlen((*(*cpm_ptr).cmm[0 as libc::c_int as
                                                           usize].cf_ptr).entry))
                           == 0 {
                    /* 現在の形態素をALTに保存 */
                    assign_feature_alt_mrph(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                                            (*(*cpm_ptr).pred_b_ptr).head_ptr);
                    /* このALTを最終結果の形態素にする */
                    delete_existing_features((*(*cpm_ptr).pred_b_ptr).head_ptr); /* 現在の形態素featureを削除 */
                    copy_mrph((*(*cpm_ptr).pred_b_ptr).head_ptr, &mut m,
                              (0 as libc::c_int == 0) as
                                  libc::c_int); /* あれば削除 */
                    delete_cfeature(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                                    (*fp).cp);
                    assign_cfeature(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                                    b"\xe7\x94\xa8\xe8\xa8\x80\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                    delete_cfeature(&mut (*(*(*cpm_ptr).pred_b_ptr).head_ptr).f,
                                    b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x9b\x96\xe6\x98\xa7\xe6\x80\xa7\xe8\xa7\xa3\xe6\xb6\x88\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                    break ;
                }
            }
            fp = (*fp).next
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_dist_from_work_mgr(mut bp: *mut BNST_DATA,
                                                mut hp: *mut BNST_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0 as libc::c_int;
    /* 候補チェック */
    if Work_mgr.dpnd.check[(*bp).num as usize].num == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    i = 0 as libc::c_int;
    while i < Work_mgr.dpnd.check[(*bp).num as usize].num {
        if Work_mgr.dpnd.check[(*bp).num as usize].pos[i as usize] ==
               (*hp).num {
            i += 1;
            dist = i;
            break ;
        } else { i += 1 }
    }
    if dist == 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else { if dist > 1 as libc::c_int { dist = 2 as libc::c_int } }
    return dist;
}
/*====================================================================
                               END
====================================================================*/
