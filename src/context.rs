#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types, ptr_wrapping_offset_from, register_tool)]

use crate::{_FEATURE, BNST_DATA, case_analysis, corefer, cpm_def, FEATURE, MRPH_DATA, PARA_DATA, PARA_MANAGER, sentence_data, TAG_DATA, TOTAL_MGR};
use crate::bnst_compare::subordinate_level_check;
use crate::case_analysis::{copy_cf_with_alloc, make_print_string, MatchPP, noun_lexical_disambiguation_by_case_analysis, pp_code_to_kstr, pp_code_to_kstr_in_context, pp_kstr_to_code, record_case_analysis, verb_lexical_disambiguation_by_case_analysis};
use crate::case_data::{_make_data_cframe_ex, _make_data_cframe_sm, make_data_cframe};
use crate::case_ipal::{CFSimExist, clear_cf, clear_mgr_cf, get_cfs_similarity, init_mgr_cf};
use crate::case_match::{calc_similarity_word_cf, calc_similarity_word_cf_with_sm, case_frame_match, cf_match_element, cf_match_sm_thesaurus, count_pat_element, EX_match_exact, EX_match_score, EX_match_subject, sms_match};
use crate::case_print::{print_data_cframe, print_good_crrspnds};
use crate::consts::{VERBOSE2, VERBOSE3};
use crate::corefer::corefer_id;
use crate::ctools::{assign_cfeature, check_feature, malloc_data, Outfp, stderr, stdout};
use crate::feature::{append_feature, check_str_type, clear_feature, print_feature};
use crate::lib_dt::dt_classify;
use crate::lib_event::get_cf_event_value;
use crate::lib_print::print_result;
use crate::lib_sm::{assign_ga_subject, ClearSMList, sm2code, specify_sm_from_cf};
use crate::structs::{case_component, CDB_FILE, CF_ALIGNMENT, cf_list, CF_MATCH_MGR, ellipsis_component, entity_list, LIST, predicate_anaphora_list, sentence};
use crate::tools::{hash, OptAddSvmFeatureDiscourseDepth, OptAddSvmFeatureObjectRecognition, OptAddSvmFeatureReferedNum, OptAddSvmFeatureUtype, OptAnaphoraBaseline, OptArticle, OptCaseFlag, OptCFMode, OptDiscFlag, OptDiscNounMethod, OptDiscPredMethod, OptDisplay, OptEllipsis, OptLearn, OptMergeCFResult, OptNoCandidateBehind, realloc_data, strdup_with_check, VerboseLevel};
use crate::types::{CASE_COMPONENT, CASE_FRAME, CF_PRED_MGR, CFLIST, DBM_FILE, E_CANDIDATE, E_FEATURES, E_SVM_FEATURES, E_TWIN_CAND_SVM_FEATURES, ELLIPSIS_CMM, ELLIPSIS_COMPONENT, ELLIPSIS_MGR, ENTITY_LIST, FEATUREptr, PALIST, SENTENCE_DATA, size_t};

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
pub static mut LocationNames: [*mut libc::c_char; 22] =
    [b"PARENTV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PARENTV_MC\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"CHILDPV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"CHILDV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PARENTNPARENTV\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"PARENTNPARENTV_MC\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"PV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PV_MC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PARENTVPARENTV\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"PARENTVPARENTV_MC\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"MC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"SC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PRE_OTHERS\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"POST_OTHERS\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"S1_MC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S1_SC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S1_OTHERS\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"S2_MC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S2_SC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S2_OTHERS\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"OTHERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut LocationNums: [libc::c_int; 22] =
    [0x2 as libc::c_int, 0x3 as libc::c_int, 0x200 as libc::c_int,
        0x400 as libc::c_int, 0x4 as libc::c_int, 0x5 as libc::c_int,
        0x8 as libc::c_int, 0x9 as libc::c_int, 0x10 as libc::c_int,
        0x11 as libc::c_int, 0x2001 as libc::c_int, 0x4000 as libc::c_int,
        0x8000 as libc::c_int, 0x9000 as libc::c_int, 0x12001 as libc::c_int,
        0x14000 as libc::c_int, 0x10000 as libc::c_int, 0x22001 as libc::c_int,
        0x24000 as libc::c_int, 0x20000 as libc::c_int, 0 as libc::c_int,
        -(10 as libc::c_int)];
#[no_mangle]
pub static mut LocationOrder: [[libc::c_int; 21]; 4] =
    [[-(10 as libc::c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0],
        [0x200 as libc::c_int, 0x3 as libc::c_int, 0x2 as libc::c_int,
            0x9 as libc::c_int, 0x8 as libc::c_int, 0x5 as libc::c_int,
            0x10 as libc::c_int, 0x400 as libc::c_int, 0x4 as libc::c_int,
            0x8000 as libc::c_int, 0x4000 as libc::c_int, 0x11 as libc::c_int,
            0x2001 as libc::c_int, 0x12001 as libc::c_int, 0x10000 as libc::c_int,
            0x14000 as libc::c_int, 0x9000 as libc::c_int, 0x22001 as libc::c_int,
            0x20000 as libc::c_int, 0x24000 as libc::c_int, -(10 as libc::c_int)],
        [0x400 as libc::c_int, 0x8000 as libc::c_int, 0x200 as libc::c_int,
            0x8 as libc::c_int, 0x10000 as libc::c_int, 0x9 as libc::c_int,
            0x5 as libc::c_int, 0x2 as libc::c_int, 0x11 as libc::c_int,
            0x12001 as libc::c_int, 0x10 as libc::c_int, 0x4 as libc::c_int,
            0x20000 as libc::c_int, 0x3 as libc::c_int, 0x4000 as libc::c_int,
            0x22001 as libc::c_int, 0x9000 as libc::c_int, 0x2001 as libc::c_int,
            0x24000 as libc::c_int, 0x14000 as libc::c_int, -(10 as libc::c_int)],
        [0x200 as libc::c_int, 0x10 as libc::c_int, 0x8 as libc::c_int,
            0x8000 as libc::c_int, 0x400 as libc::c_int, 0x2 as libc::c_int,
            0x10000 as libc::c_int, 0x11 as libc::c_int, 0x12001 as libc::c_int,
            0x9 as libc::c_int, 0x14000 as libc::c_int, 0x4000 as libc::c_int,
            0x20000 as libc::c_int, 0x3 as libc::c_int, 0x4 as libc::c_int,
            0x2001 as libc::c_int, 0x5 as libc::c_int, 0x22001 as libc::c_int,
            0x9000 as libc::c_int, 0x24000 as libc::c_int, -(10 as libc::c_int)]];
#[no_mangle]
pub static mut maxscore: libc::c_float = 0.;
#[no_mangle]
pub static mut maxrawscore: libc::c_float = 0.;
#[no_mangle]
pub static mut maxs: *mut SENTENCE_DATA =
    0 as *const SENTENCE_DATA as *mut SENTENCE_DATA;
#[no_mangle]
pub static mut maxi: libc::c_int = 0;
#[no_mangle]
pub static mut maxpos: libc::c_int = 0;
#[no_mangle]
pub static mut maxtag: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut maxfeatures: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut Bcheck: *mut *mut libc::c_int =
    0 as *const *mut libc::c_int as *mut *mut libc::c_int;
#[no_mangle]
pub static mut LC: *mut *mut libc::c_int =
    0 as *const *mut libc::c_int as *mut *mut libc::c_int;
#[no_mangle]
pub static mut ExtraCheck: libc::c_int = 0;
#[no_mangle]
pub static mut ExtraLC: libc::c_int = 0;
#[no_mangle]
pub static mut PrintFeatures: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut PrintEx: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ExtraTags: [*mut libc::c_char; 4] =
    [b"\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut ETAG_name: [*mut libc::c_char; 7] =
    [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe4\xba\xba\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe7\x8a\xb6\xe6\xb3\x81\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x89\x8d\xe6\x96\x87\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"\xe5\xbe\x8c\xe6\x96\x87\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char];
#[no_mangle]
pub static mut AntecedentDecideThresholdPredGeneral: libc::c_float = 0.60f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForGa: libc::c_float = 0.60f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForNoun: libc::c_float = 1.00f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForNounBonus1: libc::c_float = 0.50f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForNounBonus2: libc::c_float = 0.70f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForNounSM: libc::c_float = 0.80f64 as libc::c_float;
#[no_mangle]
pub static mut AntecedentDecideThresholdForNi: libc::c_float = 0.90f64 as libc::c_float;
#[no_mangle]
pub static mut CFSimThreshold: libc::c_float = 0.80f64 as libc::c_float;
#[no_mangle]
pub static mut SVM_FREQ_SD: libc::c_float = 80.08846f64 as libc::c_float;
#[no_mangle]
pub static mut SVM_FREQ_SD_NO: libc::c_float = 504.70998f64 as libc::c_float;
#[no_mangle]
pub static mut SVM_R_NUM_S_SD: libc::c_float = 1 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut SVM_R_NUM_E_SD: libc::c_float = 1 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut BaseForExponentialFunction: libc::c_float =
    0.9f64 as libc::c_float;
#[no_mangle]
pub static mut palist: [PALIST; 1024] =
    [PALIST {
        key: 0 as *const libc::c_char as *mut libc::c_char,
        voice: 0,
        cf_addr: 0,
        cc: [0 as *const CASE_COMPONENT as *mut CASE_COMPONENT; 20],
        next:
        0 as *const predicate_anaphora_list as
            *mut predicate_anaphora_list,
    }; 1024];
#[no_mangle]
pub static mut cflist: [CFLIST; 1024] =
    [CFLIST {
        key: 0 as *const libc::c_char as *mut libc::c_char,
        cfid: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        cfid_num: 0,
        cfid_max: 0,
        next: 0 as *const cf_list as *mut cf_list,
    }; 1024];
#[no_mangle]
pub static mut elist: [ENTITY_LIST; 1024] =
    [ENTITY_LIST {
        key: 0 as *const libc::c_char as *mut libc::c_char,
        surface_num: 0.,
        ellipsis_num: 0.,
        next: 0 as *const entity_list as *mut entity_list,
    }; 1024];
#[no_mangle]
pub static mut ante_cands: *mut E_CANDIDATE =
    0 as *const E_CANDIDATE as *mut E_CANDIDATE;
#[no_mangle]
pub static mut cand_num: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cand_num_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut CaseOrder: [[*mut libc::c_char; 4]; 3] =
    [[b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char,
        b"\xe3\x83\xb2\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe3\x83\xb2\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
            b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
            b"\xe3\x83\xb2\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut DiscAddedCases: [libc::c_int; 44] =
    [-(10 as libc::c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0];
#[no_mangle]
pub static mut LocationLimit: [libc::c_int; 44] =
    [-(10 as libc::c_int), -(10 as libc::c_int), -(10 as libc::c_int),
        -(10 as libc::c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub static mut PrevSentenceLimit: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut AlreadyDecidedFlag: libc::c_int = 0;
#[no_mangle]
pub static mut OptUseSmfix: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn loc_code_to_str(mut loc: libc::c_int)
                                         -> *mut libc::c_char {
    if loc == 0x2 as libc::c_int {
        return b"PARENTV\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    } else {
        if loc == 0x3 as libc::c_int {
            return b"PARENTV_MC\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        } else {
            if loc == 0x200 as libc::c_int {
                return b"CHILDPV\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            } else {
                if loc == 0x400 as libc::c_int {
                    return b"CHILDV\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                } else {
                    if loc == 0x4 as libc::c_int {
                        return b"PARENTNPARENTV\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char;
                    } else {
                        if loc == 0x5 as libc::c_int {
                            return b"PARENTNPARENTV_MC\x00" as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char;
                        } else {
                            if loc == 0x8 as libc::c_int {
                                return b"PV\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char;
                            } else {
                                if loc == 0x9 as libc::c_int {
                                    return b"PV_MC\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                } else {
                                    if loc == 0x10 as libc::c_int {
                                        return b"PARENTVPARENTV\x00" as
                                            *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char;
                                    } else {
                                        if loc == 0x11 as libc::c_int {
                                            return b"PARENTVPARENTV_MC\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char;
                                        } else {
                                            if loc == 0x2001 as libc::c_int {
                                                return b"MC\x00" as *const u8
                                                    as
                                                    *const libc::c_char
                                                    as
                                                    *mut libc::c_char;
                                            } else {
                                                if loc ==
                                                    0x4000 as libc::c_int {
                                                    return b"SC\x00" as
                                                        *const u8 as
                                                        *const libc::c_char
                                                        as
                                                        *mut libc::c_char;
                                                } else {
                                                    if loc ==
                                                        0x8000 as
                                                            libc::c_int {
                                                        return b"PRE_OTHERS\x00"
                                                            as
                                                            *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char;
                                                    } else {
                                                        if loc ==
                                                            0x9000 as
                                                                libc::c_int
                                                        {
                                                            return b"POST_OTHERS\x00"
                                                                as
                                                                *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char;
                                                        } else {
                                                            if loc ==
                                                                0x12001 as
                                                                    libc::c_int
                                                            {
                                                                return b"S1_MC\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char;
                                                            } else {
                                                                if loc ==
                                                                    0x14000
                                                                        as
                                                                        libc::c_int
                                                                {
                                                                    return b"S1_SC\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char;
                                                                } else {
                                                                    if loc ==
                                                                        0x10000
                                                                            as
                                                                            libc::c_int
                                                                    {
                                                                        return b"S1_OTHERS\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char;
                                                                    } else {
                                                                        if loc
                                                                            ==
                                                                            0x22001
                                                                                as
                                                                                libc::c_int
                                                                        {
                                                                            return b"S2_MC\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char;
                                                                        } else {
                                                                            if loc
                                                                                ==
                                                                                0x24000
                                                                                    as
                                                                                    libc::c_int
                                                                            {
                                                                                return b"S2_SC\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                                    as
                                                                                    *mut libc::c_char;
                                                                            } else {
                                                                                if loc
                                                                                    ==
                                                                                    0x20000
                                                                                        as
                                                                                        libc::c_int
                                                                                {
                                                                                    return b"S2_OTHERS\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char;
                                                                                } else {
                                                                                    if loc
                                                                                        ==
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                    {
                                                                                        return b"OTHERS\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char
                                                                                            as
                                                                                            *mut libc::c_char;
                                                                                    } else {
                                                                                        if loc
                                                                                            ==
                                                                                            -(10
                                                                                                as
                                                                                                libc::c_int)
                                                                                        {
                                                                                            return b"NIL\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char
                                                                                                as
                                                                                                *mut libc::c_char;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn get_utype(mut bp: *mut TAG_DATA) -> libc::c_int {
    let mut bc: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut utype: *mut libc::c_char = 0 as *mut libc::c_char;
    bc = (*bp).b_ptr;
    while !bc.is_null() {
        utype =
            check_feature((*bc).f,
                          b"\xe7\x99\xba\xe8\xa9\xb1\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x97\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !utype.is_null() {
            utype =
                utype.offset(strlen(b"\xe7\x99\xba\xe8\xa9\xb1\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x97:\x00"
                    as *const u8 as *const libc::c_char)
                    as isize);
            return if strcmp(utype,
                             b"\xe4\xbd\x9c\xe6\xa5\xad:\xe5\xa4\xa7\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                0x1 as libc::c_int
            } else if strcmp(utype,
                             b"\xe4\xbd\x9c\xe6\xa5\xad:\xe4\xb8\xad\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                0x2 as libc::c_int
            } else if strcmp(utype,
                             b"\xe4\xbd\x9c\xe6\xa5\xad:\xe5\xb0\x8f\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                0x3 as libc::c_int
            } else if strcmp(utype,
                             b"\xe7\x95\x99\xe6\x84\x8f\xe4\xba\x8b\xe9\xa0\x85\x00"
                                 as *const u8 as *const libc::c_char) == 0 ||
                strcmp(utype,
                       b"\xe7\x95\x99\xe6\x84\x8f\xe4\xba\x8b\xe9\xa0\x85\xe3\x83\xbb\xe3\x82\xb3\xe3\x83\x84\x00"
                           as *const u8 as *const libc::c_char) == 0
                ||
                strcmp(utype,
                       b"\xe7\x95\x99\xe6\x84\x8f\xe4\xba\x8b\xe9\xa0\x85\xe3\x83\xbb\xe6\xb3\xa8\xe6\x84\x8f\x00"
                           as *const u8 as *const libc::c_char) == 0
            {
                0x4 as libc::c_int
            } else if strcmp(utype,
                             b"\xe9\xa3\x9f\xe5\x93\x81\xe3\x83\xbb\xe9\x81\x93\xe5\x85\xb7\xe6\x8f\x90\xe7\xa4\xba\x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                0x5 as libc::c_int
            } else if strcmp(utype,
                             b"\xe6\x96\x99\xe7\x90\x86\xe7\x8a\xb6\xe6\x85\x8b\x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                0x6 as libc::c_int
            } else if strcmp(utype,
                             b"\xe7\xa8\x8b\xe5\xba\xa6\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                0x7 as libc::c_int
            } else if strcmp(utype,
                             b"\xe5\x8a\xb9\xe6\x9e\x9c\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                0x8 as libc::c_int
            } else if strcmp(utype,
                             b"\xe8\xa3\x9c\xe8\xb6\xb3\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                0x9 as libc::c_int
            } else if strcmp(utype,
                             b"\xe4\xbb\xa3\xe6\x9b\xbf\xe5\x8f\xaf\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                0x10 as libc::c_int
            } else if strcmp(utype,
                             b"\xe7\xb5\x82\xe4\xba\x86\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                0x11 as libc::c_int
            } else { 0 as libc::c_int };
        } else { bc = (*bc).parent }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_discourse_depth(mut bp: *mut TAG_DATA)
                                             -> libc::c_int {
    let mut bc: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut depth_char: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut depth: libc::c_int = 0 as libc::c_int;
    bc = (*bp).b_ptr;
    while !bc.is_null() {
        depth_char =
            check_feature((*bc).f,
                          b"\xe8\xab\x87\xe8\xa9\xb1\xe6\xa7\x8b\xe9\x80\xa0\xe6\xb7\xb1\xe3\x81\x95\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !depth_char.is_null() {
            depth_char =
                depth_char.offset(strlen(b"\xe8\xab\x87\xe8\xa9\xb1\xe6\xa7\x8b\xe9\x80\xa0\xe6\xb7\xb1\xe3\x81\x95:\x00"
                    as *const u8 as
                    *const libc::c_char) as isize);
            depth = atoi(depth_char);
            return depth;
        }
        bc = (*bc).parent
    }
    return depth;
}

#[no_mangle]
pub unsafe extern "C" fn objectrecognition_match(mut bp: *mut TAG_DATA,
                                                 mut s: *mut SENTENCE_DATA)
                                                 -> libc::c_int {
    let mut objectrecognition: [libc::c_char; 32] = [0; 32];
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str =
        strstr((*s).Comment,
               b"\xe7\x89\xa9\xe4\xbd\x93\xe8\xaa\x8d\xe8\xad\x98\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                   as *const u8 as *const libc::c_char);
    if !str.is_null() {
        str =
            str.offset(strlen(b"\xe7\x89\xa9\xe4\xbd\x93\xe8\xaa\x8d\xe8\xad\x98\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        sscanf(str, b"%s\x00" as *const u8 as *const libc::c_char,
               objectrecognition.as_mut_ptr());
        return (strcmp((*(*bp).head_ptr).Goi.as_mut_ptr(),
                       objectrecognition.as_mut_ptr()) == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn loc_name_to_code(mut loc: *mut libc::c_char)
                                          -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *LocationNames[i as usize].offset(0 as libc::c_int as isize) != 0 {
        if strcmp(loc, LocationNames[i as usize]) == 0 {
            return LocationNums[i as usize];
        }
        i += 1
    }
    return -(10 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn ClearCCList(mut pap: *mut PALIST) {
    let mut j: libc::c_int = 0;
    let mut ccp: *mut CASE_COMPONENT = 0 as *mut CASE_COMPONENT;
    let mut next: *mut CASE_COMPONENT = 0 as *mut CASE_COMPONENT;
    j = 0 as libc::c_int;
    while j < 20 as libc::c_int {
        if !(*pap).cc[j as usize].is_null() {
            free((*(*pap).cc[j as usize]).word as *mut libc::c_void);
            if !(*(*pap).cc[j as usize]).pp_str.is_null() {
                free((*(*pap).cc[j as usize]).pp_str as *mut libc::c_void);
            }
            ccp = (*(*pap).cc[j as usize]).next;
            free((*pap).cc[j as usize] as *mut libc::c_void);
            while !ccp.is_null() {
                free((*ccp).word as *mut libc::c_void);
                if !(*ccp).pp_str.is_null() {
                    free((*ccp).pp_str as *mut libc::c_void);
                }
                next = (*ccp).next;
                free(ccp as *mut libc::c_void);
                ccp = next
            }
        }
        j += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClearAnaphoraList() {
    let mut i: libc::c_int = 0;
    let mut pap: *mut PALIST = 0 as *mut PALIST;
    let mut next: *mut PALIST = 0 as *mut PALIST;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !palist[i as usize].key.is_null() {
            free(palist[i as usize].key as *mut libc::c_void);
            palist[i as usize].key = 0 as *mut libc::c_char
        }
        ClearCCList(&mut *palist.as_mut_ptr().offset(i as isize));
        pap = palist[i as usize].next;
        while !pap.is_null() {
            free((*pap).key as *mut libc::c_void);
            ClearCCList(pap);
            next = (*pap).next;
            free(pap as *mut libc::c_void);
            pap = next
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClearEntityList() {
    let mut i: libc::c_int = 0;
    let mut ep: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    let mut next: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !elist[i as usize].key.is_null() {
            free(elist[i as usize].key as *mut libc::c_void);
            elist[i as usize].key = 0 as *mut libc::c_char
        }
        ep = elist[i as usize].next;
        while !ep.is_null() {
            free((*ep).key as *mut libc::c_void);
            next = (*ep).next;
            free(ep as *mut libc::c_void);
            ep = next
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClearCFList() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cfp: *mut CFLIST = 0 as *mut CFLIST;
    let mut next: *mut CFLIST = 0 as *mut CFLIST;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !cflist[i as usize].key.is_null() {
            free(cflist[i as usize].key as *mut libc::c_void);
        }
        j = 0 as libc::c_int;
        while j < cflist[i as usize].cfid_num {
            free(*cflist[i as usize].cfid.offset(j as isize) as
                *mut libc::c_void);
            j += 1
        }
        free(cflist[i as usize].cfid as *mut libc::c_void);
        cfp = cflist[i as usize].next;
        while !cfp.is_null() {
            free((*cfp).key as *mut libc::c_void);
            j = 0 as libc::c_int;
            while j < (*cfp).cfid_num {
                free(*(*cfp).cfid.offset(j as isize) as *mut libc::c_void);
                j += 1
            }
            next = (*cfp).next;
            free(cfp as *mut libc::c_void);
            cfp = next
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn InitContextHash() {
    memset(palist.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<PALIST>() as
               libc::c_ulong).wrapping_mul(1024 as libc::c_int as
               libc::c_ulong));
    memset(cflist.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<CFLIST>() as
               libc::c_ulong).wrapping_mul(1024 as libc::c_int as
               libc::c_ulong));
    memset(elist.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<ENTITY_LIST>() as
               libc::c_ulong).wrapping_mul(1024 as libc::c_int as
               libc::c_ulong));
}

#[no_mangle]
pub unsafe extern "C" fn InitEllipsisMGR(mut em: *mut ELLIPSIS_MGR) {
    memset(em as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<ELLIPSIS_MGR>() as libc::c_ulong);
}

#[no_mangle]
pub unsafe extern "C" fn ClearEllipsisComponent(mut ec:
                                                *mut ELLIPSIS_COMPONENT) {
    let mut emp: *mut ELLIPSIS_COMPONENT = 0 as *mut ELLIPSIS_COMPONENT;
    let mut next: *mut ELLIPSIS_COMPONENT = 0 as *mut ELLIPSIS_COMPONENT;
    if !(*ec).pp_str.is_null() { free((*ec).pp_str as *mut libc::c_void); }
    emp = (*ec).next;
    while !emp.is_null() {
        if !(*emp).pp_str.is_null() {
            free((*emp).pp_str as *mut libc::c_void);
        }
        next = (*emp).next;
        free(emp as *mut libc::c_void);
        emp = next
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClearEllipsisMGR(mut em: *mut ELLIPSIS_MGR) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        ClearEllipsisComponent(&mut *(*em).cc.as_mut_ptr().offset(i as
            isize));
        i += 1
    }
    clear_feature(&mut (*em).f);
    InitEllipsisMGR(em);
}

#[no_mangle]
pub unsafe extern "C" fn CopyEllipsisComponent(mut dst:
                                               *mut ELLIPSIS_COMPONENT,
                                               mut src:
                                               *mut ELLIPSIS_COMPONENT) {
    (*dst).s = (*src).s;
    if !(*src).pp_str.is_null() {
        (*dst).pp_str = strdup((*src).pp_str)
    } else { (*dst).pp_str = 0 as *mut libc::c_char }
    (*dst).bnst = (*src).bnst;
    (*dst).score = (*src).score;
    (*dst).dist = (*src).dist;
    if !(*src).next.is_null() {
        (*dst).next =
            malloc_data(::std::mem::size_of::<ELLIPSIS_COMPONENT>() as
                            libc::c_ulong,
                        b"CopyEllipsisComponent\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut ELLIPSIS_COMPONENT;
        CopyEllipsisComponent((*dst).next, (*src).next);
    } else { (*dst).next = 0 as *mut ellipsis_component };
}

#[no_mangle]
pub unsafe extern "C" fn CheckBasicPP(mut pp: libc::c_int) -> libc::c_int {
    if pp == 41 as libc::c_int { return 1 as libc::c_int; }
    if pp == -(10 as libc::c_int) || pp > 8 as libc::c_int ||
        pp < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn StoreCaseComponent(mut ccpp:
                                            *mut *mut CASE_COMPONENT,
                                            mut word: *mut libc::c_char,
                                            mut pp_str: *mut libc::c_char,
                                            mut sent_n: libc::c_int,
                                            mut tag_n: libc::c_int,
                                            mut flag: libc::c_int) {
    while !(*ccpp).is_null() {
        if !pp_str.is_null() && !(**ccpp).pp_str.is_null() &&
            strcmp((**ccpp).pp_str, pp_str) == 0 {
            free((**ccpp).word as *mut libc::c_void);
            (**ccpp).word = strdup(word);
            (**ccpp).sent_num = sent_n;
            (**ccpp).tag_num = tag_n;
            (**ccpp).count = 1 as libc::c_int;
            (**ccpp).flag = flag;
            return;
        } else {
            if pp_str.is_null() && (**ccpp).pp_str.is_null() &&
                strcmp((**ccpp).word, word) == 0 {
                if (**ccpp).flag == 2 as libc::c_int &&
                    flag == 1 as libc::c_int {
                    (**ccpp).flag = 1 as libc::c_int
                }
                (**ccpp).count += 1;
                return;
            }
        }
        ccpp = &mut (**ccpp).next
    }
    *ccpp =
        malloc_data(::std::mem::size_of::<CASE_COMPONENT>() as libc::c_ulong,
                    b"StoreCaseComponent\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut CASE_COMPONENT;
    (**ccpp).word = strdup(word);
    if !pp_str.is_null() {
        (**ccpp).pp_str = strdup(pp_str)
    } else { (**ccpp).pp_str = 0 as *mut libc::c_char }
    (**ccpp).sent_num = sent_n;
    (**ccpp).tag_num = tag_n;
    (**ccpp).count = 1 as libc::c_int;
    (**ccpp).flag = flag;
    (**ccpp).next = 0 as *mut case_component;
}

#[no_mangle]
pub unsafe extern "C" fn StoreEllipsisComponent(mut ccp:
                                                *mut ELLIPSIS_COMPONENT,
                                                mut pp_str: *mut libc::c_char,
                                                mut sp: *mut SENTENCE_DATA,
                                                mut tag_n: libc::c_int,
                                                mut score: libc::c_float,
                                                mut dist: libc::c_int) {
    if pp_str.is_null() {
        (*ccp).s = sp;
        (*ccp).pp_str = 0 as *mut libc::c_char;
        (*ccp).bnst = tag_n;
        (*ccp).score = score;
        (*ccp).dist = dist;
        (*ccp).next = 0 as *mut ellipsis_component;
        return;
    } else {
        let mut ccpp: *mut *mut ELLIPSIS_COMPONENT = &mut ccp;
        while !(*ccpp).is_null() && !(**ccpp).s.is_null() &&
            (**ccpp).bnst != 0 {
            if !(**ccpp).pp_str.is_null() &&
                strcmp((**ccpp).pp_str, pp_str) == 0 {
                (**ccpp).s = sp;
                (**ccpp).pp_str = strdup(pp_str);
                (**ccpp).bnst = tag_n;
                (**ccpp).score = score;
                (**ccpp).dist = dist;
                return;
            }
            ccpp = &mut (**ccpp).next
        }
        if (*ccpp).is_null() {
            *ccpp =
                malloc_data(::std::mem::size_of::<ELLIPSIS_COMPONENT>() as
                                libc::c_ulong,
                            b"StoreEllipsisComponent\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut ELLIPSIS_COMPONENT
        }
        (**ccpp).s = sp;
        (**ccpp).pp_str = strdup(pp_str);
        (**ccpp).bnst = tag_n;
        (**ccpp).score = score;
        (**ccpp).dist = dist;
        (**ccpp).next = 0 as *mut ellipsis_component
    };
}

#[no_mangle]
pub unsafe extern "C" fn RegisterEntity(mut key: *mut libc::c_char,
                                        mut flag: libc::c_int) {
    let mut ep: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    if key.is_null() { return; }
    ep =
        &mut *elist.as_mut_ptr().offset((hash as
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
            as isize) as *mut ENTITY_LIST;
    if !(*ep).key.is_null() {
        let mut epp: *mut *mut ENTITY_LIST = 0 as *mut *mut ENTITY_LIST;
        epp = &mut ep;
        loop {
            if strcmp((**epp).key, key) == 0 {
                if flag != 0 {
                    (**epp).surface_num += 1.
                } else { (**epp).ellipsis_num += 1. }
                return;
            }
            epp = &mut (**epp).next;
            if (*epp).is_null() { break; }
        }
        *epp =
            malloc_data(::std::mem::size_of::<ENTITY_LIST>() as libc::c_ulong,
                        b"RegisterEntity\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut ENTITY_LIST;
        (**epp).key = strdup(key);
        if flag != 0 {
            (**epp).surface_num = 1 as libc::c_int as libc::c_double;
            (**epp).ellipsis_num = 0 as libc::c_int as libc::c_double
        } else {
            (**epp).surface_num = 0 as libc::c_int as libc::c_double;
            (**epp).ellipsis_num = 1 as libc::c_int as libc::c_double
        }
        (**epp).next = 0 as *mut entity_list
    } else {
        (*ep).key = strdup(key);
        if flag != 0 {
            (*ep).surface_num = 1 as libc::c_int as libc::c_double;
            (*ep).ellipsis_num = 0 as libc::c_int as libc::c_double
        } else {
            (*ep).surface_num = 0 as libc::c_int as libc::c_double;
            (*ep).ellipsis_num = 1 as libc::c_int as libc::c_double
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn DecayEntityList() {
    let mut i: libc::c_int = 0;
    let mut ep: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    // let mut next: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !elist[i as usize].key.is_null() {
            elist[i as usize].surface_num *=
                BaseForExponentialFunction as libc::c_double;
            elist[i as usize].ellipsis_num *=
                BaseForExponentialFunction as libc::c_double
        }
        ep = elist[i as usize].next;
        while !ep.is_null() {
            (*ep).surface_num *= BaseForExponentialFunction as libc::c_double;
            (*ep).ellipsis_num *=
                BaseForExponentialFunction as libc::c_double;
            ep = (*ep).next
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn RegisterEllipsisEntity(mut sp: *mut SENTENCE_DATA,
                                                mut cpm_ptr: *mut CF_PRED_MGR,
                                                mut em_ptr:
                                                *mut ELLIPSIS_MGR) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).element_num
    {
        num =
            (*cpm_ptr).cmm[0 as libc::c_int as
                usize].result_lists_p[0 as libc::c_int as
                usize].flag[i as
                usize];
        if num != -(1 as libc::c_int) &&
            (*cpm_ptr).elem_b_num[num as usize] <= -(2 as libc::c_int) {
            if !(*cpm_ptr).elem_b_ptr[num as usize].is_null() {
                RegisterEntity((*(*(*cpm_ptr).elem_b_ptr[num as
                    usize]).head_ptr).Goi.as_mut_ptr(),
                               0 as libc::c_int);
            } else {
                RegisterEntity(ExtraTags[1 as libc::c_int as usize],
                               0 as libc::c_int);
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn RegisterAllSurfaceEntity(mut sp: *mut SENTENCE_DATA)
                                                  -> *mut ENTITY_LIST {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            RegisterEntity((*(*(*sp).tag_data.offset(i as
                isize)).head_ptr).Goi.as_mut_ptr(),
                           (0 as libc::c_int == 0) as libc::c_int);
        }
        i += 1
    }
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn CheckEntity(mut key: *mut libc::c_char)
                                     -> *mut ENTITY_LIST {
    let mut ep: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    ep =
        &mut *elist.as_mut_ptr().offset((hash as
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
            as isize) as *mut ENTITY_LIST;
    if (*ep).key.is_null() { return 0 as *mut ENTITY_LIST; }
    while !ep.is_null() {
        if strcmp((*ep).key, key) == 0 { return ep; }
        ep = (*ep).next
    }
    return 0 as *mut ENTITY_LIST;
}

#[no_mangle]
pub unsafe extern "C" fn RegisterTagTarget(mut key: *mut libc::c_char,
                                           mut voice: libc::c_int,
                                           mut cf_addr: libc::c_int,
                                           mut pp: libc::c_int,
                                           mut pp_str: *mut libc::c_char,
                                           mut word: *mut libc::c_char,
                                           mut sent_n: libc::c_int,
                                           mut tag_n: libc::c_int,
                                           mut flag: libc::c_int) {
    let mut pap: *mut PALIST = 0 as *mut PALIST;
    if word.is_null() { return; }
    if CheckBasicPP(pp) == 0 as libc::c_int {
        free(word as *mut libc::c_void);
        return;
    }
    pap =
        &mut *palist.as_mut_ptr().offset((hash as
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
            as isize) as *mut PALIST;
    if !(*pap).key.is_null() {
        let mut papp: *mut *mut PALIST = 0 as *mut *mut PALIST;
        papp = &mut pap;
        loop {
            if strcmp((**papp).key, key) == 0 && (**papp).voice == voice &&
                (**papp).cf_addr == (**papp).cf_addr {
                StoreCaseComponent(&mut *(**papp).cc.as_mut_ptr().offset(pp as
                    isize),
                                   word, pp_str, sent_n, tag_n, flag);
                free(word as *mut libc::c_void);
                return;
            }
            papp = &mut (**papp).next;
            if (*papp).is_null() { break; }
        }
        *papp =
            malloc_data(::std::mem::size_of::<PALIST>() as libc::c_ulong,
                        b"RegisterTagTarget\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut PALIST;
        (**papp).key = strdup(key);
        (**papp).voice = voice;
        (**papp).cf_addr = cf_addr;
        memset((**papp).cc.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int,
               (::std::mem::size_of::<*mut CASE_COMPONENT>() as
                   libc::c_ulong).wrapping_mul(20 as libc::c_int as
                   libc::c_ulong));
        StoreCaseComponent(&mut *(**papp).cc.as_mut_ptr().offset(pp as isize),
                           word, pp_str, sent_n, tag_n, flag);
        (**papp).next = 0 as *mut predicate_anaphora_list
    } else {
        (*pap).key = strdup(key);
        (*pap).voice = voice;
        (*pap).cf_addr = cf_addr;
        StoreCaseComponent(&mut *(*pap).cc.as_mut_ptr().offset(pp as isize),
                           word, pp_str, sent_n, tag_n, flag);
    }
    free(word as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn CheckTagTarget(mut key: *mut libc::c_char,
                                        mut voice: libc::c_int,
                                        mut cf_addr: libc::c_int,
                                        mut pp: libc::c_int,
                                        mut pp_str: *mut libc::c_char)
                                        -> *mut CASE_COMPONENT {
    let mut pap: *mut PALIST = 0 as *mut PALIST;
    let mut ccp: *mut CASE_COMPONENT = 0 as *mut CASE_COMPONENT;
    if CheckBasicPP(pp) == 0 as libc::c_int {
        return 0 as *mut CASE_COMPONENT;
    }
    pap =
        &mut *palist.as_mut_ptr().offset((hash as
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
            as isize) as *mut PALIST;
    if (*pap).key.is_null() { return 0 as *mut CASE_COMPONENT; }
    while !pap.is_null() {
        if strcmp((*pap).key, key) == 0 && (*pap).voice == voice &&
            (*pap).cf_addr == cf_addr {
            ccp = (*pap).cc[pp as usize];
            if !pp_str.is_null() {
                while !ccp.is_null() {
                    if (*ccp).pp_str.is_null() ||
                        strcmp((*ccp).pp_str, pp_str) == 0 {
                        return ccp;
                    }
                    ccp = (*ccp).next
                }
            } else if !ccp.is_null() {
                while !(*ccp).next.is_null() { ccp = (*ccp).next }
                return ccp;
            }
            return 0 as *mut CASE_COMPONENT;
        }
        pap = (*pap).next
    }
    return 0 as *mut CASE_COMPONENT;
}

#[no_mangle]
pub unsafe extern "C" fn get_pred_id(mut cfid: *mut libc::c_char)
                                     -> *mut libc::c_char {
    let mut verb: [libc::c_char; 128] = [0; 128];
    let mut type_0: [libc::c_char; 128] = [0; 128];
    let mut voice: [libc::c_char; 128] = [0; 128];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    if sscanf(cfid,
              b"%[^:]:%[^:]:%[^0-9]%d\x00" as *const u8 as
                  *const libc::c_char, verb.as_mut_ptr(), type_0.as_mut_ptr(),
              voice.as_mut_ptr(), &mut index as *mut libc::c_int) ==
        4 as libc::c_int {
        ret =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                libc::c_ulong).wrapping_mul(strlen(verb.as_mut_ptr()).wrapping_add(strlen(type_0.as_mut_ptr())).wrapping_add(strlen(voice.as_mut_ptr())).wrapping_add(3
                as
                libc::c_int
                as
                libc::c_ulong)),
                        b"get_pred_id\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut libc::c_char;
        sprintf(ret, b"%s:%s:%s\x00" as *const u8 as *const libc::c_char,
                verb.as_mut_ptr(), type_0.as_mut_ptr(), voice.as_mut_ptr());
    } else if sscanf(cfid,
                     b"%[^:]:%[^0-9]%d\x00" as *const u8 as
                         *const libc::c_char, verb.as_mut_ptr(),
                     type_0.as_mut_ptr(), &mut index as *mut libc::c_int) ==
        3 as libc::c_int {
        ret =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                libc::c_ulong).wrapping_mul(strlen(verb.as_mut_ptr()).wrapping_add(strlen(type_0.as_mut_ptr())).wrapping_add(2
                as
                libc::c_int
                as
                libc::c_ulong)),
                        b"get_pred_id\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut libc::c_char;
        sprintf(ret, b"%s:%s\x00" as *const u8 as *const libc::c_char,
                verb.as_mut_ptr(), type_0.as_mut_ptr());
    } else { ret = 0 as *mut libc::c_char }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn RegisterCF(mut cfid: *mut libc::c_char) {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cfp: *mut CFLIST = 0 as *mut CFLIST;
    if cfid.is_null() { return; }
    key = get_pred_id(cfid);
    if key.is_null() { return; }
    cfp =
        &mut *cflist.as_mut_ptr().offset((hash as
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
            as isize) as *mut CFLIST;
    if !(*cfp).key.is_null() {
        let mut cfpp: *mut *mut CFLIST = 0 as *mut *mut CFLIST;
        cfpp = &mut cfp;
        loop {
            if strcmp((**cfpp).key, key) == 0 {
                if (**cfpp).cfid_num >= (**cfpp).cfid_max {
                    (**cfpp).cfid_max <<= 1 as libc::c_int;
                    (**cfpp).cfid =
                        realloc_data((**cfpp).cfid as *mut libc::c_void,
                                     (::std::mem::size_of::<*mut libc::c_char>()
                                         as
                                         libc::c_ulong).wrapping_mul((**cfpp).cfid_max
                                         as
                                         libc::c_ulong),
                                     b"RegisterCF\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char) as
                            *mut *mut libc::c_char
                }
                let fresh0 = (**cfpp).cfid_num;
                (**cfpp).cfid_num = (**cfpp).cfid_num + 1;
                let ref mut fresh1 = *(**cfpp).cfid.offset(fresh0 as isize);
                *fresh1 = strdup(cfid);
                free(key as *mut libc::c_void);
                return;
            }
            cfpp = &mut (**cfpp).next;
            if (*cfpp).is_null() { break; }
        }
        *cfpp =
            malloc_data(::std::mem::size_of::<CFLIST>() as libc::c_ulong,
                        b"RegisterCF\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut CFLIST;
        cfp = *cfpp
    }
    (*cfp).key = strdup(key);
    (*cfp).cfid_num = 1 as libc::c_int;
    (*cfp).cfid_max = 2 as libc::c_int;
    (*cfp).cfid =
        malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
            libc::c_ulong).wrapping_mul((*cfp).cfid_max as
            libc::c_ulong),
                    b"RegisterCF\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut *mut libc::c_char;
    *(*cfp).cfid = strdup(cfid);
    (*cfp).next = 0 as *mut cf_list;
    free(key as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn CheckCF(mut key: *mut libc::c_char) -> *mut CFLIST {
    let mut cfp: *mut CFLIST = 0 as *mut CFLIST;
    cfp =
        &mut *cflist.as_mut_ptr().offset((hash as
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
            as isize) as *mut CFLIST;
    if (*cfp).key.is_null() { return 0 as *mut CFLIST; }
    while !cfp.is_null() {
        if strcmp((*cfp).key, key) == 0 { return cfp; }
        cfp = (*cfp).next
    }
    return 0 as *mut CFLIST;
}

#[no_mangle]
pub unsafe extern "C" fn ClearSentence(mut s: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s).Mrph_num {
        clear_feature(&mut (*(*s).mrph_data.offset(i as isize)).f);
        i += 1
    }
    free((*s).mrph_data as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*s).Bnst_num {
        clear_feature(&mut (*(*s).bnst_data.offset(i as isize)).f);
        i += 1
    }
    free((*s).bnst_data as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*s).Tag_num {
        clear_feature(&mut (*(*s).tag_data.offset(i as isize)).f);
        i += 1
    }
    free((*s).tag_data as *mut libc::c_void);
    free((*s).para_data as *mut libc::c_void);
    free((*s).para_manager as *mut libc::c_void);
    free((*s).Comment as *mut libc::c_void);
    if !(*s).cpm.is_null() { free((*s).cpm as *mut libc::c_void); }
    if !(*s).cf.is_null() { free((*s).cf as *mut libc::c_void); }
    if !(*s).KNPSID.is_null() { free((*s).KNPSID as *mut libc::c_void); }
    if !(*s).Best_mgr.is_null() {
        clear_mgr_cf(s);
        free((*s).Best_mgr as *mut libc::c_void);
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClearSentences(mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Sen_num - 1 as libc::c_int {
        if OptArticle != 0 {
            print_result(sentence_data.as_mut_ptr().offset(i as isize),
                         1 as libc::c_int, 1 as libc::c_int);
        }
        ClearSentence(sentence_data.as_mut_ptr().offset(i as isize));
        i += 1
    }
    (*sp).Sen_num = 1 as libc::c_int;
    corefer_id = 0 as libc::c_int;
    ClearAnaphoraList();
    ClearEntityList();
    ClearCFList();
    ClearSMList();
    InitContextHash();
}

#[no_mangle]
pub unsafe extern "C" fn InitSentence(mut s: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*s).mrph_data =
        malloc_data((::std::mem::size_of::<MRPH_DATA>() as
            libc::c_ulong).wrapping_mul(200 as libc::c_int as
            libc::c_ulong),
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut MRPH_DATA;
    (*s).bnst_data =
        malloc_data((::std::mem::size_of::<BNST_DATA>() as
            libc::c_ulong).wrapping_mul(200 as libc::c_int as
            libc::c_ulong),
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut BNST_DATA;
    (*s).tag_data =
        malloc_data((::std::mem::size_of::<TAG_DATA>() as
            libc::c_ulong).wrapping_mul(200 as libc::c_int as
            libc::c_ulong),
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut TAG_DATA;
    (*s).para_data =
        malloc_data((::std::mem::size_of::<PARA_DATA>() as
            libc::c_ulong).wrapping_mul(32 as libc::c_int as
            libc::c_ulong),
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut PARA_DATA;
    (*s).para_manager =
        malloc_data((::std::mem::size_of::<PARA_MANAGER>() as
            libc::c_ulong).wrapping_mul(32 as libc::c_int as
            libc::c_ulong),
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut PARA_MANAGER;
    (*s).Best_mgr =
        malloc_data(::std::mem::size_of::<TOTAL_MGR>() as libc::c_ulong,
                    b"InitSentence\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut TOTAL_MGR;
    (*s).Sen_num = 0 as libc::c_int;
    (*s).Mrph_num = 0 as libc::c_int;
    (*s).Bnst_num = 0 as libc::c_int;
    (*s).New_Bnst_num = 0 as libc::c_int;
    (*s).Tag_num = 0 as libc::c_int;
    (*s).New_Tag_num = 0 as libc::c_int;
    (*s).KNPSID = 0 as *mut libc::c_char;
    (*s).Comment = 0 as *mut libc::c_char;
    (*s).cpm = 0 as *mut CF_PRED_MGR;
    (*s).cf = 0 as *mut CASE_FRAME;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh2 = (*(*s).mrph_data.offset(i as isize)).f;
        *fresh2 = 0 as FEATUREptr;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh3 = (*(*s).bnst_data.offset(i as isize)).f;
        *fresh3 = 0 as FEATUREptr;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh4 = (*(*s).tag_data.offset(i as isize)).f;
        *fresh4 = 0 as FEATUREptr;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            let ref mut fresh5 =
                (*(*s).para_data.offset(i as isize)).f_pattern.fp[j as usize];
            *fresh5 = 0 as *mut FEATURE;
            j += 1
        }
        i += 1
    }
    init_mgr_cf((*s).Best_mgr);
}

#[no_mangle]
pub unsafe extern "C" fn PreserveSentence(mut sp: *mut SENTENCE_DATA)
                                          -> *mut SENTENCE_DATA {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sp_new: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    if (*sp).Sen_num > 512 as libc::c_int {
        fprintf(stderr,
                b"Sentence buffer overflowed!\n\x00" as *const u8 as
                    *const libc::c_char);
        ClearSentences(sp);
    }
    sp_new =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    (*sp_new).available = (*sp).available;
    (*sp_new).Sen_num = (*sp).Sen_num;
    if !(*sp).Comment.is_null() { (*sp_new).Comment = strdup((*sp).Comment) }
    (*sp_new).Mrph_num = (*sp).Mrph_num;
    (*sp_new).mrph_data =
        malloc_data((::std::mem::size_of::<MRPH_DATA>() as
            libc::c_ulong).wrapping_mul((*sp).Mrph_num as
            libc::c_ulong),
                    b"MRPH DATA\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut MRPH_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        *(*sp_new).mrph_data.offset(i as isize) =
            *(*sp).mrph_data.offset(i as isize);
        i += 1
    }
    (*sp_new).Bnst_num = (*sp).Bnst_num;
    (*sp_new).New_Bnst_num = (*sp).New_Bnst_num;
    (*sp_new).bnst_data =
        malloc_data((::std::mem::size_of::<BNST_DATA>() as
            libc::c_ulong).wrapping_mul(((*sp).Bnst_num +
            (*sp).New_Bnst_num)
            as libc::c_ulong),
                    b"BNST DATA\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut BNST_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num + (*sp).New_Bnst_num {
        *(*sp_new).bnst_data.offset(i as isize) =
            *(*sp).bnst_data.offset(i as isize);
        let ref mut fresh6 =
            (*(*sp_new).bnst_data.offset(i as isize)).mrph_ptr;
        *fresh6 =
            (*sp_new).mrph_data.offset((*(*sp).bnst_data.offset(i as
                isize)).mrph_ptr.wrapping_offset_from((*sp).mrph_data)
                as libc::c_long as isize);
        let ref mut fresh7 =
            (*(*sp_new).bnst_data.offset(i as isize)).head_ptr;
        *fresh7 =
            (*sp_new).mrph_data.offset((*(*sp).bnst_data.offset(i as
                isize)).head_ptr.wrapping_offset_from((*sp).mrph_data)
                as libc::c_long as isize);
        if !(*(*sp).bnst_data.offset(i as isize)).parent.is_null() {
            let ref mut fresh8 =
                (*(*sp_new).bnst_data.offset(i as isize)).parent;
            *fresh8 =
                (*sp_new).bnst_data.offset((*(*sp).bnst_data.offset(i as
                    isize)).parent.wrapping_offset_from((*sp).bnst_data)
                    as libc::c_long as isize)
        }
        j = 0 as libc::c_int;
        while !(*(*sp_new).bnst_data.offset(i as
            isize)).child[j as
            usize].is_null()
        {
            let ref mut fresh9 =
                (*(*sp_new).bnst_data.offset(i as isize)).child[j as usize];
            *fresh9 =
                (*sp_new).bnst_data.offset((*(*sp).bnst_data.offset(i as
                    isize)).child[j
                    as
                    usize].wrapping_offset_from((*sp).bnst_data)
                    as libc::c_long as isize);
            j += 1
        }
        if !(*(*sp).bnst_data.offset(i as isize)).pred_b_ptr.is_null() {
            let ref mut fresh10 =
                (*(*sp_new).bnst_data.offset(i as isize)).pred_b_ptr;
            *fresh10 =
                (*sp_new).bnst_data.offset((*(*sp).bnst_data.offset(i as
                    isize)).pred_b_ptr.wrapping_offset_from((*sp).bnst_data)
                    as libc::c_long as isize)
        }
        i += 1
    }
    (*sp_new).Tag_num = (*sp).Tag_num;
    (*sp_new).New_Tag_num = (*sp).New_Tag_num;
    (*sp_new).tag_data =
        malloc_data((::std::mem::size_of::<TAG_DATA>() as
            libc::c_ulong).wrapping_mul(((*sp).Tag_num +
            (*sp).New_Tag_num)
            as libc::c_ulong),
                    b"TAG DATA\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut TAG_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num + (*sp).New_Tag_num {
        *(*sp_new).tag_data.offset(i as isize) =
            *(*sp).tag_data.offset(i as isize);
        let ref mut fresh11 =
            (*(*sp_new).tag_data.offset(i as isize)).mrph_ptr;
        *fresh11 =
            (*sp_new).mrph_data.offset((*(*sp).tag_data.offset(i as
                isize)).mrph_ptr.wrapping_offset_from((*sp).mrph_data)
                as libc::c_long as isize);
        if !(*(*sp).tag_data.offset(i as isize)).settou_ptr.is_null() {
            let ref mut fresh12 =
                (*(*sp_new).tag_data.offset(i as isize)).settou_ptr;
            *fresh12 =
                (*sp_new).mrph_data.offset((*(*sp).tag_data.offset(i as
                    isize)).settou_ptr.wrapping_offset_from((*sp).mrph_data)
                    as libc::c_long as isize)
        }
        let ref mut fresh13 =
            (*(*sp_new).tag_data.offset(i as isize)).jiritu_ptr;
        *fresh13 =
            (*sp_new).mrph_data.offset((*(*sp).tag_data.offset(i as
                isize)).jiritu_ptr.wrapping_offset_from((*sp).mrph_data)
                as libc::c_long as isize);
        if !(*(*sp).tag_data.offset(i as isize)).fuzoku_ptr.is_null() {
            let ref mut fresh14 =
                (*(*sp_new).tag_data.offset(i as isize)).fuzoku_ptr;
            *fresh14 =
                (*sp_new).mrph_data.offset((*(*sp).tag_data.offset(i as
                    isize)).fuzoku_ptr.wrapping_offset_from((*sp).mrph_data)
                    as libc::c_long as isize)
        }
        let ref mut fresh15 =
            (*(*sp_new).tag_data.offset(i as isize)).head_ptr;
        *fresh15 =
            (*sp_new).mrph_data.offset((*(*sp).tag_data.offset(i as
                isize)).head_ptr.wrapping_offset_from((*sp).mrph_data)
                as libc::c_long as isize);
        if !(*(*sp).tag_data.offset(i as isize)).parent.is_null() {
            let ref mut fresh16 =
                (*(*sp_new).tag_data.offset(i as isize)).parent;
            *fresh16 =
                (*sp_new).tag_data.offset((*(*sp).tag_data.offset(i as
                    isize)).parent.wrapping_offset_from((*sp).tag_data)
                    as libc::c_long as isize)
        }
        j = 0 as libc::c_int;
        while !(*(*sp_new).tag_data.offset(i as
            isize)).child[j as
            usize].is_null()
        {
            let ref mut fresh17 =
                (*(*sp_new).tag_data.offset(i as isize)).child[j as usize];
            *fresh17 =
                (*sp_new).tag_data.offset((*(*sp).tag_data.offset(i as
                    isize)).child[j
                    as
                    usize].wrapping_offset_from((*sp).tag_data)
                    as libc::c_long as isize);
            j += 1
        }
        if !(*(*sp).tag_data.offset(i as isize)).pred_b_ptr.is_null() {
            let ref mut fresh18 =
                (*(*sp_new).tag_data.offset(i as isize)).pred_b_ptr;
            *fresh18 =
                (*sp_new).tag_data.offset((*(*sp).tag_data.offset(i as
                    isize)).pred_b_ptr.wrapping_offset_from((*sp).tag_data)
                    as libc::c_long as isize)
        }
        if !(*(*sp).tag_data.offset(i as isize)).next.is_null() {
            let ref mut fresh19 =
                (*(*sp_new).tag_data.offset(i as isize)).next;
            *fresh19 =
                (*sp_new).tag_data.offset((*(*sp).tag_data.offset(i as
                    isize)).next.wrapping_offset_from((*sp).tag_data)
                    as libc::c_long as isize)
        }
        let ref mut fresh20 = (*(*sp_new).tag_data.offset(i as isize)).b_ptr;
        *fresh20 =
            (*sp_new).bnst_data.offset((*(*sp).tag_data.offset(i as
                isize)).b_ptr.wrapping_offset_from((*sp).bnst_data)
                as libc::c_long as isize);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num + (*sp).New_Bnst_num {
        if !(*(*sp).bnst_data.offset(i as isize)).tag_ptr.is_null() {
            let ref mut fresh21 =
                (*(*sp_new).bnst_data.offset(i as isize)).tag_ptr;
            *fresh21 =
                (*sp_new).tag_data.offset((*(*sp).bnst_data.offset(i as
                    isize)).tag_ptr.wrapping_offset_from((*sp).tag_data)
                    as libc::c_long as isize)
        }
        i += 1
    }
    if !(*sp).KNPSID.is_null() {
        (*sp_new).KNPSID = strdup((*sp).KNPSID)
    } else { (*sp_new).KNPSID = 0 as *mut libc::c_char }
    (*sp_new).para_data =
        malloc_data((::std::mem::size_of::<PARA_DATA>() as
            libc::c_ulong).wrapping_mul((*sp).Para_num as
            libc::c_ulong),
                    b"PARA DATA\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut PARA_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        *(*sp_new).para_data.offset(i as isize) =
            *(*sp).para_data.offset(i as isize);
        let ref mut fresh22 =
            (*(*sp_new).para_data.offset(i as isize)).manager_ptr;
        *fresh22 =
            (*fresh22).offset((*sp_new).para_manager.wrapping_offset_from((*sp).para_manager)
                as libc::c_long as isize);
        i += 1
    }
    (*sp_new).para_manager =
        malloc_data((::std::mem::size_of::<PARA_MANAGER>() as
            libc::c_ulong).wrapping_mul((*sp).Para_M_num as
            libc::c_ulong),
                    b"PARA MANAGER\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut PARA_MANAGER;
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        *(*sp_new).para_manager.offset(i as isize) =
            *(*sp).para_manager.offset(i as isize);
        let ref mut fresh23 =
            (*(*sp_new).para_manager.offset(i as isize)).parent;
        *fresh23 =
            (*fresh23).offset((*sp_new).para_manager.wrapping_offset_from((*sp).para_manager)
                as libc::c_long as isize);
        j = 0 as libc::c_int;
        while j < (*(*sp_new).para_manager.offset(i as isize)).child_num {
            let ref mut fresh24 =
                (*(*sp_new).para_manager.offset(i as
                    isize)).child[j as usize];
            *fresh24 =
                (*fresh24).offset((*sp_new).para_manager.wrapping_offset_from((*sp).para_manager)
                    as libc::c_long as isize);
            j += 1
        }
        let ref mut fresh25 =
            (*(*sp_new).para_manager.offset(i as isize)).bnst_ptr;
        *fresh25 =
            (*fresh25).offset((*sp_new).bnst_data.wrapping_offset_from((*sp).bnst_data)
                as libc::c_long as isize);
        i += 1
    }
    (*sp_new).cpm = 0 as *mut CF_PRED_MGR;
    (*sp_new).cf = 0 as *mut CASE_FRAME;
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn PreserveCPM(mut sp_new: *mut SENTENCE_DATA,
                                     mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cfnum: libc::c_int = 0 as libc::c_int;
    (*sp_new).cpm =
        malloc_data((::std::mem::size_of::<CF_PRED_MGR>() as
            libc::c_ulong).wrapping_mul((*(*sp).Best_mgr).pred_num
            as libc::c_ulong),
                    b"CF PRED MGR\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut CF_PRED_MGR;
    i = 0 as libc::c_int;
    while i < (*(*sp).Best_mgr).pred_num {
        cfnum += (*(*sp).Best_mgr).cpm[i as usize].result_num;
        i += 1
    }
    (*sp_new).cf =
        malloc_data((::std::mem::size_of::<CASE_FRAME>() as
            libc::c_ulong).wrapping_mul(cfnum as libc::c_ulong),
                    b"CASE FRAME\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut CASE_FRAME;
    cfnum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*(*sp).Best_mgr).pred_num {
        *(*sp_new).cpm.offset(i as isize) = (*(*sp).Best_mgr).cpm[i as usize];
        num = (*(*(*sp).Best_mgr).cpm[i as usize].pred_b_ptr).num;
        let ref mut fresh26 =
            (*(*sp_new).tag_data.offset(num as isize)).cpm_ptr;
        *fresh26 = (*sp_new).cpm.offset(i as isize);
        let ref mut fresh27 = (*(*sp_new).cpm.offset(i as isize)).pred_b_ptr;
        *fresh27 = (*sp_new).tag_data.offset(num as isize);
        j = 0 as libc::c_int;
        while j < (*(*sp_new).cpm.offset(i as isize)).cf.element_num {
            if (*(*sp_new).cpm.offset(i as isize)).elem_b_num[j as usize] >
                -(2 as libc::c_int) {
                let ref mut fresh28 =
                    (*(*sp_new).cpm.offset(i as
                        isize)).elem_b_ptr[j as usize];
                *fresh28 =
                    (*sp_new).tag_data.offset((*(*sp_new).cpm.offset(i as
                        isize)).elem_b_ptr[j
                        as
                        usize].wrapping_offset_from((*sp).tag_data)
                        as libc::c_long as isize)
            }
            j += 1
        }
        let ref mut fresh29 =
            (*(*(*sp_new).cpm.offset(i as isize)).pred_b_ptr).cf_ptr;
        *fresh29 = (*sp_new).cf.offset(cfnum as isize);
        j = 0 as libc::c_int;
        while j < (*(*sp_new).cpm.offset(i as isize)).result_num {
            copy_cf_with_alloc((*sp_new).cf.offset(cfnum as isize),
                               (*(*sp_new).cpm.offset(i as
                                   isize)).cmm[j as
                                   usize].cf_ptr);
            let ref mut fresh30 =
                (*(*sp_new).cpm.offset(i as isize)).cmm[j as usize].cf_ptr;
            *fresh30 = (*sp_new).cf.offset(cfnum as isize);
            (*(*sp).Best_mgr).cpm[i as usize].cmm[j as usize].cf_ptr =
                (*sp_new).cf.offset(cfnum as isize);
            cfnum += 1;
            j += 1
        }
        i += 1
    }
    i = (*sp).Tag_num;
    while i < (*sp).Tag_num + (*sp).New_Tag_num {
        if !(*(*sp_new).tag_data.offset(i as isize)).cpm_ptr.is_null() {
            let ref mut fresh31 =
                (*(*sp_new).tag_data.offset(i as isize)).cpm_ptr;
            *fresh31 =
                (*(*sp_new).tag_data.offset((*(*(*(*sp_new).tag_data.offset(i
                    as
                    isize)).cpm_ptr).pred_b_ptr).num
                    as isize)).cpm_ptr
        }
        i += 1
    }
    (*sp_new).Best_mgr = 0 as *mut TOTAL_MGR;
}

#[no_mangle]
pub unsafe extern "C" fn CheckCaseComponent(mut cpm_ptr: *mut CF_PRED_MGR,
                                            mut tp: *mut TAG_DATA)
                                            -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int) {
            if (*(*cpm_ptr).elem_b_ptr[i as usize]).num == (*tp).num {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            j = 0 as libc::c_int;
            while !(*(*cpm_ptr).elem_b_ptr[i as
                usize]).child[j as
                usize].is_null()
            {
                if (*(*(*cpm_ptr).elem_b_ptr[i as
                    usize]).child[j as
                    usize]).bnum
                    == (*(*cpm_ptr).elem_b_ptr[i as usize]).bnum &&
                    (*(*(*cpm_ptr).elem_b_ptr[i as
                        usize]).child[j as
                        usize]).num
                        == (*tp).num {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
                j += 1
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn CheckHaveEllipsisComponentPara(mut tp: *mut TAG_DATA,
                                                        mut word:
                                                        *mut libc::c_char)
                                                        -> libc::c_int {
    let mut j: libc::c_int = 0;
    if (*tp).para_type as libc::c_int == 1 as libc::c_int &&
        !(*tp).parent.is_null() &&
        (*(*tp).parent).para_top_p as libc::c_int != 0 {
        j = 0 as libc::c_int;
        while !(*(*tp).parent).child[j as usize].is_null() {
            if tp != (*(*tp).parent).child[j as usize] &&
                (*(*(*tp).parent).child[j as usize]).para_type as
                    libc::c_int == 1 as libc::c_int &&
                strcmp((*(*(*(*tp).parent).child[j as
                    usize]).head_ptr).Goi.as_mut_ptr(),
                       word) == 0 {
                return 1 as libc::c_int;
            }
            j += 1
        }
    } else if (*tp).para_top_p != 0 {
        j = 1 as libc::c_int;
        while !(*tp).child[j as usize].is_null() {
            if (*(*tp).child[j as usize]).para_type as libc::c_int ==
                1 as libc::c_int &&
                strcmp((*(*(*tp).child[j as
                    usize]).head_ptr).Goi.as_mut_ptr(),
                       word) == 0 {
                return 1 as libc::c_int;
            }
            j += 1
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn CheckHaveEllipsisComponent(mut cpm_ptr:
                                                    *mut CF_PRED_MGR,
                                                    mut cmm_ptr:
                                                    *mut CF_MATCH_MGR,
                                                    mut l: libc::c_int,
                                                    mut word:
                                                    *mut libc::c_char)
                                                    -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*cmm_ptr).cf_ptr).element_num {
        num = (*cmm_ptr).result_lists_p[l as usize].flag[i as usize];
        if num >= 0 as libc::c_int {
            if !(*cpm_ptr).elem_b_ptr[num as usize].is_null() {
                if !word.is_null() &&
                    (strcmp((*(*(*cpm_ptr).elem_b_ptr[num as
                        usize]).head_ptr).Goi.as_mut_ptr(),
                            word) == 0 ||
                        (*cpm_ptr).cf.type_0 == 2 as libc::c_int &&
                            CheckHaveEllipsisComponentPara((*cpm_ptr).elem_b_ptr[num
                                as
                                usize],
                                                           word) != 0) {
                    return 1 as libc::c_int;
                }
            } else if word.is_null() { return 1 as libc::c_int; }
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn CheckObligatoryCase(mut cpm_ptr: *mut CF_PRED_MGR,
                                             mut cmm_ptr: *mut CF_MATCH_MGR,
                                             mut l: libc::c_int,
                                             mut bp: *mut TAG_DATA)
                                             -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if cpm_ptr.is_null() { return 0 as libc::c_int; }
    if (*cmm_ptr).score != -(2 as libc::c_int) as libc::c_double {
        i = 0 as libc::c_int;
        while i < (*(*cmm_ptr).cf_ptr).element_num {
            num = (*cmm_ptr).result_lists_p[l as usize].flag[i as usize];
            if num != -(1 as libc::c_int) &&
                (*cpm_ptr).elem_b_num[num as usize] > -(2 as libc::c_int)
                && (*(*cpm_ptr).elem_b_ptr[num as usize]).num == (*bp).num
            {
                if MatchPP((*(*cmm_ptr).cf_ptr).pp[i as
                    usize][0 as libc::c_int
                    as usize],
                           b"\xe3\x82\xac\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) != 0
                    ||
                    MatchPP((*(*cmm_ptr).cf_ptr).pp[i as
                        usize][0 as
                        libc::c_int
                        as
                        usize],
                            b"\xe3\x83\xb2\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 ||
                    MatchPP((*(*cmm_ptr).cf_ptr).pp[i as
                        usize][0 as
                        libc::c_int
                        as
                        usize],
                            b"\xe3\x83\x8b\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 ||
                    MatchPP((*(*cmm_ptr).cf_ptr).pp[i as
                        usize][0 as
                        libc::c_int
                        as
                        usize],
                            b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 {
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn GetCandCase(mut cpm_ptr: *mut CF_PRED_MGR,
                                     mut cmm_ptr: *mut CF_MATCH_MGR,
                                     mut bp: *mut TAG_DATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if !cpm_ptr.is_null() && (*cpm_ptr).result_num > 0 as libc::c_int &&
        (*cmm_ptr).score != -(2 as libc::c_int) as libc::c_double {
        i = 0 as libc::c_int;
        while i < (*(*cmm_ptr).cf_ptr).element_num {
            num =
                (*cmm_ptr).result_lists_p[0 as libc::c_int as
                    usize].flag[i as usize];
            if num != -(1 as libc::c_int) &&
                !(*cpm_ptr).elem_b_ptr[num as usize].is_null() &&
                (*(*cpm_ptr).elem_b_ptr[num as usize]).num == (*bp).num {
                return (*(*cmm_ptr).cf_ptr).pp[i as
                    usize][0 as libc::c_int as
                    usize];
            }
            i += 1
        }
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn CheckCaseCorrespond(mut cpm_ptr: *mut CF_PRED_MGR,
                                             mut cmm_ptr: *mut CF_MATCH_MGR,
                                             mut l: libc::c_int,
                                             mut bp: *mut TAG_DATA,
                                             mut cf_ptr: *mut CASE_FRAME,
                                             mut n: libc::c_int)
                                             -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if (*cpm_ptr).result_num > 0 as libc::c_int &&
        (*cmm_ptr).score != -(2 as libc::c_int) as libc::c_double {
        i = 0 as libc::c_int;
        while i < (*(*cmm_ptr).cf_ptr).element_num {
            num = (*cmm_ptr).result_lists_p[l as usize].flag[i as usize];
            if num != -(1 as libc::c_int) &&
                (*cpm_ptr).elem_b_num[num as usize] > -(2 as libc::c_int)
                && (*(*cpm_ptr).elem_b_ptr[num as usize]).num == (*bp).num
            {
                if (*cf_ptr).pp[n as usize][0 as libc::c_int as usize] ==
                    (*(*cmm_ptr).cf_ptr).pp[i as
                        usize][0 as libc::c_int as
                        usize] ||
                    MatchPP((*cf_ptr).pp[n as
                        usize][0 as libc::c_int as
                        usize],
                            b"\xe3\x82\xac\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 &&
                        MatchPP((*(*cmm_ptr).cf_ptr).pp[i as
                            usize][0 as
                            libc::c_int
                            as
                            usize],
                                b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char) != 0 {
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn GetRealParent(mut sp: *mut SENTENCE_DATA,
                                       mut bp: *mut TAG_DATA)
                                       -> *mut TAG_DATA {
    if (*bp).dpnd_head != -(1 as libc::c_int) {
        return (*sp).tag_data.offset((*bp).dpnd_head as isize);
    }
    return 0 as *mut TAG_DATA;
}

#[no_mangle]
pub unsafe extern "C" fn CountBnstDistance(mut cs: *mut SENTENCE_DATA,
                                           mut candn: libc::c_int,
                                           mut ps: *mut SENTENCE_DATA,
                                           mut pn: libc::c_int)
                                           -> libc::c_int {
    let mut sdiff: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0 as libc::c_int;
    sdiff = ps.wrapping_offset_from(cs) as libc::c_long as libc::c_int;
    if sdiff > 0 as libc::c_int {
        i = 1 as libc::c_int;
        while i < sdiff {
            diff += (*ps.offset(-(i as isize))).Tag_num;
            i += 1
        }
        diff += pn + (*cs).Tag_num - candn
    } else { diff = pn - candn }
    return diff;
}

#[no_mangle]
pub unsafe extern "C" fn CheckPredicateChild(mut pred_b_ptr: *mut TAG_DATA,
                                             mut child_ptr: *mut TAG_DATA)
                                             -> libc::c_int {
    if !(*child_ptr).parent.is_null() {
        if (*(*child_ptr).parent).num == (*pred_b_ptr).num {
            return 1 as libc::c_int;
        } else {
            if (*child_ptr).para_type as libc::c_int == 1 as libc::c_int &&
                (*(*child_ptr).parent).para_top_p as libc::c_int != 0 &&
                !(*(*child_ptr).parent).parent.is_null() &&
                (*(*(*child_ptr).parent).parent).num == (*pred_b_ptr).num {
                return 1 as libc::c_int;
            }
        }
    } else if !(*pred_b_ptr).parent.is_null() {
        if (*(*pred_b_ptr).parent).num == (*child_ptr).num {
            return 1 as libc::c_int;
        } else {
            if (*(*pred_b_ptr).parent).para_top_p as libc::c_int != 0 &&
                (*child_ptr).para_type as libc::c_int == 1 as libc::c_int
                &&
                (*(*child_ptr).parent).num == (*(*pred_b_ptr).parent).num {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisSvmFeatures2String(mut esf:
                                                    *mut E_SVM_FEATURES)
                                                    -> *mut libc::c_char {
    let mut max: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prenum: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    prenum = 5 as libc::c_int;
    max =
        (::std::mem::size_of::<E_SVM_FEATURES>() as
            libc::c_ulong).wrapping_sub((prenum as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
            as
            libc::c_ulong)).wrapping_div(::std::mem::size_of::<libc::c_int>()
            as
            libc::c_ulong).wrapping_add(prenum
            as
            libc::c_ulong)
            as libc::c_int;
    sbuf =
        malloc_data((::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
            libc::c_double *
            (10 as libc::c_int as libc::c_double +
                log(max as libc::c_double))) as size_t,
                    b"EllipsisSvmFeatures2String\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    buffer =
        malloc_data((::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
            libc::c_double *
            (10 as libc::c_int as libc::c_double +
                log(max as libc::c_double)) *
            max as libc::c_double +
            20 as libc::c_int as libc::c_double) as size_t,
                    b"EllipsisSvmFeatures2String\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    sprintf(buffer, b"1:%.5f\x00" as *const u8 as *const libc::c_char,
            (*esf).similarity as libc::c_double);
    prenum -= 3 as libc::c_int;
    if OptAddSvmFeatureDiscourseDepth == 0 { max -= 1 }
    if OptAddSvmFeatureReferedNum == 0 { max -= 2 as libc::c_int }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        sprintf(sbuf, b" %d:%d\x00" as *const u8 as *const libc::c_char,
                prenum, (*esf).frequency as libc::c_int);
    } else {
        sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                prenum, (*esf).frequency as libc::c_double);
    }
    strcat(buffer, sbuf);
    if OptAddSvmFeatureDiscourseDepth != 0 {
        prenum += 1;
        sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                prenum, (*esf).discourse_depth_inverse as libc::c_double);
        strcat(buffer, sbuf);
    }
    if OptAddSvmFeatureReferedNum != 0 {
        prenum += 1;
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
            sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                    prenum, (*esf).refered_num_surface as libc::c_double);
        } else {
            sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                    prenum, (*esf).refered_num_surface as libc::c_double);
        }
        strcat(buffer, sbuf);
        prenum += 1;
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
            sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                    prenum, (*esf).refered_num_ellipsis as libc::c_double);
        } else {
            sprintf(sbuf, b" %d:%.5f\x00" as *const u8 as *const libc::c_char,
                    prenum, (*esf).refered_num_ellipsis as libc::c_double);
        }
        strcat(buffer, sbuf);
    }
    i = prenum + 1 as libc::c_int;
    while i <= max {
        sprintf(sbuf, b" %d:%d\x00" as *const u8 as *const libc::c_char, i,
                *(*esf).c_pp.as_mut_ptr().offset(i as
                    isize).offset(-(prenum as
                    isize)).offset(-(1
                    as
                    libc::c_int
                    as
                    isize)));
        strcat(buffer, sbuf);
        i += 1
    }
    free(sbuf as *mut libc::c_void);
    return buffer;
}

#[no_mangle]
pub unsafe extern "C" fn TwinCandSvmFeatures2String(mut esf:
                                                    *mut E_TWIN_CAND_SVM_FEATURES)
                                                    -> *mut libc::c_char {
    let mut max: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prenum: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    prenum = 2 as libc::c_int;
    max =
        (::std::mem::size_of::<E_TWIN_CAND_SVM_FEATURES>() as
            libc::c_ulong).wrapping_sub((prenum as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
            as
            libc::c_ulong)).wrapping_div(::std::mem::size_of::<libc::c_int>()
            as
            libc::c_ulong).wrapping_add(prenum
            as
            libc::c_ulong)
            as libc::c_int;
    sbuf =
        malloc_data((::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
            libc::c_double *
            (10 as libc::c_int as libc::c_double +
                log(max as libc::c_double))) as size_t,
                    b"TwinCandSvmFeatures2String\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    buffer =
        malloc_data((::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
            libc::c_double *
            (10 as libc::c_int as libc::c_double +
                log(max as libc::c_double)) *
            max as libc::c_double +
            20 as libc::c_int as libc::c_double) as size_t,
                    b"TwinCandSvmFeatures2String\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    sprintf(buffer, b"1:%.5f 2:%.5f\x00" as *const u8 as *const libc::c_char,
            (*esf).c1_similarity as libc::c_double,
            (*esf).c2_similarity as libc::c_double);
    i = prenum + 1 as libc::c_int;
    while i <= max {
        sprintf(sbuf, b" %d:%d\x00" as *const u8 as *const libc::c_char, i,
                *(*esf).c1_pp.as_mut_ptr().offset(i as
                    isize).offset(-(prenum
                    as
                    isize)).offset(-(1
                    as
                    libc::c_int
                    as
                    isize)));
        strcat(buffer, sbuf);
        i += 1
    }
    free(sbuf as *mut libc::c_void);
    return buffer;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisSvmFeaturesString2Feature(mut em_ptr:
                                                           *mut ELLIPSIS_MGR,
                                                           mut cpm_ptr:
                                                           *mut CF_PRED_MGR,
                                                           mut ellipsis_class:
                                                           libc::c_int,
                                                           mut ecp:
                                                           *mut libc::c_char,
                                                           mut word:
                                                           *mut libc::c_char,
                                                           mut pp:
                                                           libc::c_int,
                                                           mut sid:
                                                           *mut libc::c_char,
                                                           mut num:
                                                           libc::c_int,
                                                           mut loc:
                                                           libc::c_int) {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if PrintFeatures == 0 { return; }
    if word.is_null() { return; }
    buffer =
        malloc_data(strlen(ecp).wrapping_add(128 as libc::c_int as
            libc::c_ulong).wrapping_add(strlen(word)),
                    b"EllipsisSvmFeaturesString2FeatureString\x00" as
                        *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
    sprintf(buffer,
            b"SVM\xe5\xad\xa6\xe7\xbf\x92FEATURE;%s;%s;%s;%s;%d:%d %s\x00" as
                *const u8 as *const libc::c_char, word,
            pp_code_to_kstr_in_context(cpm_ptr, pp),
            if loc >= 0 as libc::c_int {
                loc_code_to_str(loc) as *const libc::c_char
            } else { b"NONE\x00" as *const u8 as *const libc::c_char }, sid,
            num, ellipsis_class, ecp);
    assign_cfeature(&mut (*em_ptr).f, buffer, 0 as libc::c_int);
    free(buffer as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn TwinCandSvmFeaturesString2Feature(mut em_ptr:
                                                           *mut ELLIPSIS_MGR,
                                                           mut ecp:
                                                           *mut libc::c_char,
                                                           mut c1:
                                                           *mut E_CANDIDATE,
                                                           mut c2:
                                                           *mut E_CANDIDATE) {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sid1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sid2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    if !(*c1).tp.is_null() {
        if (*(*(*c1).tp).head_ptr).Goi.as_mut_ptr().is_null() {
            return;
        } else {
            w1 = (*(*(*c1).tp).head_ptr).Goi.as_mut_ptr();
            p1 = pp_code_to_kstr((*(*c1).ef).c_pp);
            sid1 =
                if !(*(*c1).s).KNPSID.is_null() {
                    (*(*c1).s).KNPSID.offset(5 as libc::c_int as isize) as
                        *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char } as
                    *mut libc::c_char;
            n1 = (*(*c1).tp).num
        }
    } else {
        w1 = (*c1).tag;
        sid1 =
            b"?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        n1 = -(1 as libc::c_int)
    }
    if !(*c2).tp.is_null() {
        if (*(*(*c2).tp).head_ptr).Goi.as_mut_ptr().is_null() {
            return;
        } else {
            w2 = (*(*(*c2).tp).head_ptr).Goi.as_mut_ptr();
            sid2 =
                if !(*(*c2).s).KNPSID.is_null() {
                    (*(*c2).s).KNPSID.offset(5 as libc::c_int as isize) as
                        *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char } as
                    *mut libc::c_char;
            n2 = (*(*c2).tp).num
        }
    } else {
        w2 = (*c2).tag;
        sid2 =
            b"?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        n2 = -(1 as libc::c_int)
    }
    buffer =
        malloc_data(strlen(ecp).wrapping_add(256 as libc::c_int as
            libc::c_ulong).wrapping_add(strlen(w1)).wrapping_add(strlen(w2)),
                    b"TwinCandSvmFeaturesString2FeatureString\x00" as
                        *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
    sprintf(buffer,
            b"SVM\xe5\xad\xa6\xe7\xbf\x92FEATURE;%s;%s;%s;%s;%d;%s;%s;%s;%d:%s\x00"
                as *const u8 as *const libc::c_char,
            pp_code_to_kstr((*(*c1).ef).p_pp), w1,
            if (*(*c1).ef).c_location >= 0 as libc::c_int {
                loc_code_to_str((*(*c1).ef).c_location) as *const libc::c_char
            } else { b"NONE\x00" as *const u8 as *const libc::c_char }, sid1,
            n1, w2,
            if (*(*c2).ef).c_location >= 0 as libc::c_int {
                loc_code_to_str((*(*c2).ef).c_location) as *const libc::c_char
            } else { b"NONE\x00" as *const u8 as *const libc::c_char }, sid2,
            n2, ecp);
    assign_cfeature(&mut (*em_ptr).f, buffer, 0 as libc::c_int);
    free(buffer as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisFeatures2EllipsisSvmFeatures(mut ef:
                                                              *mut E_FEATURES,
                                                              mut learn_flag:
                                                              libc::c_int)
                                                              -> *mut E_SVM_FEATURES {
    let mut f: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
    let mut i: libc::c_int = 0;
    f =
        malloc_data(::std::mem::size_of::<E_SVM_FEATURES>() as libc::c_ulong,
                    b"SetEllipsisFeatures\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut E_SVM_FEATURES;
    (*f).similarity = (*ef).similarity;
    if learn_flag == (0 as libc::c_int == 0) as libc::c_int {
        (*f).frequency = (*ef).frequency as libc::c_float
    } else if (*ef).p_pp ==
        pp_kstr_to_code(b"\xe3\x83\x8e\x00" as *const u8 as
            *const libc::c_char as
            *mut libc::c_char) {
        (*f).frequency = (*ef).frequency as libc::c_float / SVM_FREQ_SD_NO
    } else { (*f).frequency = (*ef).frequency as libc::c_float / SVM_FREQ_SD }
    if learn_flag == (0 as libc::c_int == 0) as libc::c_int {
        (*f).refered_num_surface = (*ef).refered_num_surface;
        (*f).refered_num_ellipsis = (*ef).refered_num_ellipsis
    } else {
        (*f).refered_num_surface = (*ef).refered_num_surface / SVM_R_NUM_S_SD;
        (*f).refered_num_ellipsis =
            (*ef).refered_num_ellipsis / SVM_R_NUM_E_SD
    }
    i = 0 as libc::c_int;
    while i < 44 as libc::c_int {
        (*f).c_pp[i as usize] =
            if (*ef).c_pp == i { 1 as libc::c_int } else { 0 as libc::c_int };
        i += 1
    }
    (*f).c_distance = (*ef).c_distance;
    (*f).c_location[0 as libc::c_int as usize] =
        if (*ef).c_location == 0x2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[1 as libc::c_int as usize] =
        if (*ef).c_location == 0x3 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[2 as libc::c_int as usize] =
        if (*ef).c_location == 0x200 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[3 as libc::c_int as usize] =
        if (*ef).c_location == 0x400 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[4 as libc::c_int as usize] =
        if (*ef).c_location == 0x4 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[5 as libc::c_int as usize] =
        if (*ef).c_location == 0x5 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[6 as libc::c_int as usize] =
        if (*ef).c_location == 0x8 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[7 as libc::c_int as usize] =
        if (*ef).c_location == 0x9 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[8 as libc::c_int as usize] =
        if (*ef).c_location == 0x10 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[9 as libc::c_int as usize] =
        if (*ef).c_location == 0x11 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[10 as libc::c_int as usize] =
        if (*ef).c_location == 0x2001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[11 as libc::c_int as usize] =
        if (*ef).c_location == 0x4000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[12 as libc::c_int as usize] =
        if (*ef).c_location == 0x8000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[13 as libc::c_int as usize] =
        if (*ef).c_location == 0x9000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[14 as libc::c_int as usize] =
        if (*ef).c_location == 0x12001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[15 as libc::c_int as usize] =
        if (*ef).c_location == 0x14000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[16 as libc::c_int as usize] =
        if (*ef).c_location == 0x10000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[17 as libc::c_int as usize] =
        if (*ef).c_location == 0x22001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[18 as libc::c_int as usize] =
        if (*ef).c_location == 0x24000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[19 as libc::c_int as usize] =
        if (*ef).c_location == 0x20000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_location[20 as libc::c_int as usize] =
        if (*ef).c_location == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_fs_flag = (*ef).c_fs_flag;
    (*f).c_topic_flag = (*ef).c_topic_flag;
    (*f).c_no_topic_flag = (*ef).c_no_topic_flag;
    (*f).c_in_cnoun_flag = (*ef).c_in_cnoun_flag;
    (*f).c_subject_flag = (*ef).c_subject_flag;
    (*f).c_dep_mc_flag = (*ef).c_dep_mc_flag;
    (*f).c_n_modify_flag = (*ef).c_n_modify_flag;
    (*f).c_dep_p_level[0 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"A-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_dep_p_level[1 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"A\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_dep_p_level[2 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"B-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_dep_p_level[3 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"B\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_dep_p_level[4 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"B+\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_dep_p_level[5 as libc::c_int as usize] =
        if strcmp((*ef).c_dep_p_level.as_mut_ptr(),
                  b"C\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_prev_p_flag = (*ef).c_prev_p_flag;
    (*f).c_get_over_p_flag = (*ef).c_get_over_p_flag;
    (*f).c_sm_none_flag = (*ef).c_sm_none_flag;
    (*f).c_extra_tag[0 as libc::c_int as usize] =
        if (*ef).c_extra_tag == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_extra_tag[1 as libc::c_int as usize] =
        if (*ef).c_extra_tag == 1 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_extra_tag[2 as libc::c_int as usize] =
        if (*ef).c_extra_tag == 2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*f).p_pp[i as usize] =
            if (*ef).p_pp == i + 1 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        i += 1
    }
    (*f).p_voice[0 as libc::c_int as usize] =
        if (*ef).p_voice & 1 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_voice[1 as libc::c_int as usize] =
        if (*ef).p_voice & 2 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_voice[2 as libc::c_int as usize] =
        if (*ef).p_voice & 8 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[0 as libc::c_int as usize] =
        if (*ef).p_type == 1 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[1 as libc::c_int as usize] =
        if (*ef).p_type == 2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[2 as libc::c_int as usize] =
        if (*ef).p_type == 3 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_sahen_flag = (*ef).p_sahen_flag;
    (*f).p_cf_subject_flag = (*ef).p_cf_subject_flag;
    (*f).p_cf_sentence_flag = (*ef).p_cf_sentence_flag;
    (*f).p_n_modify_flag = (*ef).p_n_modify_flag;
    (*f).match_case = (*ef).match_case;
    (*f).match_verb = (*ef).match_verb;
    if OptAddSvmFeatureUtype != 0 {
        (*f).utype[0 as libc::c_int as usize] =
            if (*ef).utype == 0x1 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[1 as libc::c_int as usize] =
            if (*ef).utype == 0x2 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[2 as libc::c_int as usize] =
            if (*ef).utype == 0x3 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[3 as libc::c_int as usize] =
            if (*ef).utype == 0x4 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[4 as libc::c_int as usize] =
            if (*ef).utype == 0x5 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[5 as libc::c_int as usize] =
            if (*ef).utype == 0x6 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[6 as libc::c_int as usize] =
            if (*ef).utype == 0x7 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[7 as libc::c_int as usize] =
            if (*ef).utype == 0x8 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[8 as libc::c_int as usize] =
            if (*ef).utype == 0x9 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[9 as libc::c_int as usize] =
            if (*ef).utype == 0x10 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[10 as libc::c_int as usize] =
            if (*ef).utype == 0x11 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).utype[11 as libc::c_int as usize] =
            if (*ef).utype == 0 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int }
    } else {
        memset(&mut *(*f).utype.as_mut_ptr().offset(0 as libc::c_int as isize)
                   as *mut libc::c_int as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(12 as libc::c_int as
                   libc::c_ulong));
    }
    if OptAddSvmFeatureDiscourseDepth != 0 {
        if (*ef).discourse_depth == 0 as libc::c_int {
            (*f).discourse_depth_inverse = 0.0f64 as libc::c_float
        } else {
            (*f).discourse_depth_inverse =
                1 as libc::c_int as libc::c_float /
                    (*ef).discourse_depth as libc::c_float
        }
    } else {
        (*f).discourse_depth_inverse = 0 as libc::c_int as libc::c_float
    }
    if OptAddSvmFeatureObjectRecognition != 0 {
        (*f).objectrecognition = (*ef).objectrecognition
    } else { (*f).objectrecognition = 0 as libc::c_int }
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn MakeTwinCandSvmFeatures(mut ef1: *mut E_FEATURES,
                                                 mut ef2: *mut E_FEATURES)
                                                 -> *mut E_TWIN_CAND_SVM_FEATURES {
    let mut f: *mut E_TWIN_CAND_SVM_FEATURES =
        0 as *mut E_TWIN_CAND_SVM_FEATURES;
    let mut i: libc::c_int = 0;
    f =
        malloc_data(::std::mem::size_of::<E_TWIN_CAND_SVM_FEATURES>() as
                        libc::c_ulong,
                    b"MakeTwinCandSvmFeatures\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut E_TWIN_CAND_SVM_FEATURES;
    (*f).c1_similarity = (*ef1).similarity;
    i = 0 as libc::c_int;
    while i < 44 as libc::c_int {
        (*f).c1_pp[i as usize] =
            if (*ef1).c_pp == i {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        i += 1
    }
    (*f).c1_location[0 as libc::c_int as usize] =
        if (*ef1).c_location == 0x2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[1 as libc::c_int as usize] =
        if (*ef1).c_location == 0x3 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[2 as libc::c_int as usize] =
        if (*ef1).c_location == 0x200 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[3 as libc::c_int as usize] =
        if (*ef1).c_location == 0x400 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[4 as libc::c_int as usize] =
        if (*ef1).c_location == 0x4 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[5 as libc::c_int as usize] =
        if (*ef1).c_location == 0x5 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[6 as libc::c_int as usize] =
        if (*ef1).c_location == 0x8 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[7 as libc::c_int as usize] =
        if (*ef1).c_location == 0x9 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[8 as libc::c_int as usize] =
        if (*ef1).c_location == 0x10 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[9 as libc::c_int as usize] =
        if (*ef1).c_location == 0x11 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[10 as libc::c_int as usize] =
        if (*ef1).c_location == 0x2001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[11 as libc::c_int as usize] =
        if (*ef1).c_location == 0x4000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[12 as libc::c_int as usize] =
        if (*ef1).c_location == 0x8000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[13 as libc::c_int as usize] =
        if (*ef1).c_location == 0x9000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[14 as libc::c_int as usize] =
        if (*ef1).c_location == 0x12001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[15 as libc::c_int as usize] =
        if (*ef1).c_location == 0x14000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[16 as libc::c_int as usize] =
        if (*ef1).c_location == 0x10000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[17 as libc::c_int as usize] =
        if (*ef1).c_location == 0x22001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[18 as libc::c_int as usize] =
        if (*ef1).c_location == 0x24000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[19 as libc::c_int as usize] =
        if (*ef1).c_location == 0x20000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_location[20 as libc::c_int as usize] =
        if (*ef1).c_location == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_fs_flag = (*ef1).c_fs_flag;
    (*f).c1_topic_flag = (*ef1).c_topic_flag;
    (*f).c1_no_topic_flag = (*ef1).c_no_topic_flag;
    (*f).c1_in_cnoun_flag = (*ef1).c_in_cnoun_flag;
    (*f).c1_subject_flag = (*ef1).c_subject_flag;
    (*f).c1_dep_mc_flag = (*ef1).c_dep_mc_flag;
    (*f).c1_n_modify_flag = (*ef1).c_n_modify_flag;
    (*f).c1_dep_p_level[0 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"A-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_dep_p_level[1 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"A\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_dep_p_level[2 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"B-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_dep_p_level[3 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"B\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_dep_p_level[4 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"B+\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_dep_p_level[5 as libc::c_int as usize] =
        if strcmp((*ef1).c_dep_p_level.as_mut_ptr(),
                  b"C\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_prev_p_flag = (*ef1).c_prev_p_flag;
    (*f).c1_get_over_p_flag = (*ef1).c_get_over_p_flag;
    (*f).c1_sm_none_flag = (*ef1).c_sm_none_flag;
    (*f).c1_extra_tag[0 as libc::c_int as usize] =
        if (*ef1).c_extra_tag == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_extra_tag[1 as libc::c_int as usize] =
        if (*ef1).c_extra_tag == 1 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c1_extra_tag[2 as libc::c_int as usize] =
        if (*ef1).c_extra_tag == 2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_similarity = (*ef2).similarity;
    i = 0 as libc::c_int;
    while i < 44 as libc::c_int {
        (*f).c2_pp[i as usize] =
            if (*ef2).c_pp == i {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        i += 1
    }
    (*f).c2_location[0 as libc::c_int as usize] =
        if (*ef2).c_location == 0x2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[1 as libc::c_int as usize] =
        if (*ef2).c_location == 0x3 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[2 as libc::c_int as usize] =
        if (*ef2).c_location == 0x200 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[3 as libc::c_int as usize] =
        if (*ef2).c_location == 0x400 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[4 as libc::c_int as usize] =
        if (*ef2).c_location == 0x4 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[5 as libc::c_int as usize] =
        if (*ef2).c_location == 0x5 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[6 as libc::c_int as usize] =
        if (*ef2).c_location == 0x8 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[7 as libc::c_int as usize] =
        if (*ef2).c_location == 0x9 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[8 as libc::c_int as usize] =
        if (*ef2).c_location == 0x10 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[9 as libc::c_int as usize] =
        if (*ef2).c_location == 0x11 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[10 as libc::c_int as usize] =
        if (*ef2).c_location == 0x2001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[11 as libc::c_int as usize] =
        if (*ef2).c_location == 0x4000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[12 as libc::c_int as usize] =
        if (*ef2).c_location == 0x8000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[13 as libc::c_int as usize] =
        if (*ef2).c_location == 0x9000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[14 as libc::c_int as usize] =
        if (*ef2).c_location == 0x12001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[15 as libc::c_int as usize] =
        if (*ef2).c_location == 0x14000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[16 as libc::c_int as usize] =
        if (*ef2).c_location == 0x10000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[17 as libc::c_int as usize] =
        if (*ef2).c_location == 0x22001 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[18 as libc::c_int as usize] =
        if (*ef2).c_location == 0x24000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[19 as libc::c_int as usize] =
        if (*ef2).c_location == 0x20000 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_location[20 as libc::c_int as usize] =
        if (*ef2).c_location == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_fs_flag = (*ef2).c_fs_flag;
    (*f).c2_topic_flag = (*ef2).c_topic_flag;
    (*f).c2_no_topic_flag = (*ef2).c_no_topic_flag;
    (*f).c2_in_cnoun_flag = (*ef2).c_in_cnoun_flag;
    (*f).c2_subject_flag = (*ef2).c_subject_flag;
    (*f).c2_dep_mc_flag = (*ef2).c_dep_mc_flag;
    (*f).c2_n_modify_flag = (*ef2).c_n_modify_flag;
    (*f).c2_dep_p_level[0 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"A-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_dep_p_level[1 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"A\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_dep_p_level[2 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"B-\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_dep_p_level[3 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"B\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_dep_p_level[4 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"B+\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_dep_p_level[5 as libc::c_int as usize] =
        if strcmp((*ef2).c_dep_p_level.as_mut_ptr(),
                  b"C\x00" as *const u8 as *const libc::c_char) == 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_prev_p_flag = (*ef2).c_prev_p_flag;
    (*f).c2_get_over_p_flag = (*ef2).c_get_over_p_flag;
    (*f).c2_sm_none_flag = (*ef2).c_sm_none_flag;
    (*f).c2_extra_tag[0 as libc::c_int as usize] =
        if (*ef2).c_extra_tag == 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_extra_tag[1 as libc::c_int as usize] =
        if (*ef2).c_extra_tag == 1 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c2_extra_tag[2 as libc::c_int as usize] =
        if (*ef2).c_extra_tag == 2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*f).p_pp[i as usize] =
            if (*ef1).p_pp == i + 1 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        i += 1
    }
    (*f).p_voice[0 as libc::c_int as usize] =
        if (*ef1).p_voice & 1 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_voice[1 as libc::c_int as usize] =
        if (*ef1).p_voice & 2 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_voice[2 as libc::c_int as usize] =
        if (*ef1).p_voice & 8 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[0 as libc::c_int as usize] =
        if (*ef1).p_type == 1 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[1 as libc::c_int as usize] =
        if (*ef1).p_type == 2 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_type[2 as libc::c_int as usize] =
        if (*ef1).p_type == 3 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_sahen_flag = (*ef1).p_sahen_flag;
    (*f).p_cf_subject_flag = (*ef1).p_cf_subject_flag;
    (*f).p_cf_sentence_flag = (*ef1).p_cf_sentence_flag;
    (*f).p_n_modify_flag = (*ef1).p_n_modify_flag;
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn SetEllipsisFeaturesForPred(mut f: *mut E_FEATURES,
                                                    mut cpm_ptr:
                                                    *mut CF_PRED_MGR,
                                                    mut cf_ptr:
                                                    *mut CASE_FRAME,
                                                    mut n: libc::c_int) {
    let mut level: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
        (*f).p_pp = (*cf_ptr).pp[n as usize][0 as libc::c_int as usize];
        (*f).p_voice = (*(*cpm_ptr).pred_b_ptr).voice;
        if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            (*f).p_type = 1 as libc::c_int
        } else if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                 b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            (*f).p_type = 2 as libc::c_int
        } else if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                 b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            (*f).p_type = 3 as libc::c_int
        } else { (*f).p_type = 0 as libc::c_int }
    } else {
        (*f).p_pp = -(1 as libc::c_int);
        (*f).p_voice = -(1 as libc::c_int);
        (*f).p_type = -(1 as libc::c_int)
    }
    if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                      b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
        &&
        !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                       b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        (*f).p_sahen_flag = 1 as libc::c_int
    } else { (*f).p_sahen_flag = 0 as libc::c_int }
    (*f).p_cf_subject_flag =
        if cf_match_element((*cf_ptr).sm[n as usize],
                            b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as libc::c_int) != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_cf_sentence_flag =
        if cf_match_element((*cf_ptr).sm[n as usize],
                            b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            (0 as libc::c_int == 0) as libc::c_int) != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).p_n_modify_flag =
        if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                          b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    level =
        check_feature((*(*cpm_ptr).pred_b_ptr).f,
                      b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char);
    if !level.is_null() {
        strcpy((*f).p_dep_p_level.as_mut_ptr(),
               level.offset(7 as libc::c_int as isize));
    } else {
        (*f).p_dep_p_level[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_example_class(mut c_cpm_ptr: *mut CF_PRED_MGR,
                                           mut s: *mut SENTENCE_DATA,
                                           mut bp: *mut TAG_DATA,
                                           mut cf_ptr: *mut CASE_FRAME,
                                           mut n: libc::c_int)
                                           -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*c_cpm_ptr).cf.element_num {
        if (*cf_ptr).type_0 == 1 as libc::c_int &&
            (*c_cpm_ptr).cf.pp[i as usize][0 as libc::c_int as usize] ==
                (*cf_ptr).pp[n as usize][0 as libc::c_int as usize] ||
            (*cf_ptr).type_0 == 2 as libc::c_int &&
                (*c_cpm_ptr).cf.pp[i as usize][0 as libc::c_int as usize]
                    ==
                    pp_kstr_to_code(b"\xe3\x83\x8e\x00" as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char) {
            if !bp.is_null() {
                if !(*c_cpm_ptr).elem_b_ptr[i as usize].is_null() &&
                    ((*bp).num ==
                        (*(*c_cpm_ptr).elem_b_ptr[i as usize]).num &&
                        (*s).Sen_num ==
                            (*(*c_cpm_ptr).elem_s_ptr[i as usize]).Sen_num
                        ||
                        strcmp((*(*bp).head_ptr).Goi.as_mut_ptr(),
                               (*(*(*c_cpm_ptr).elem_b_ptr[i as
                                   usize]).head_ptr).Goi.as_mut_ptr())
                            == 0) {
                    return 1 as libc::c_int;
                }
            } else if (*c_cpm_ptr).elem_b_ptr[i as usize].is_null() {
                return 1 as libc::c_int;
            }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn SetEllipsisFeatures(mut s: *mut SENTENCE_DATA,
                                             mut cs: *mut SENTENCE_DATA,
                                             mut cpm_ptr: *mut CF_PRED_MGR,
                                             mut cmm_ptr: *mut CF_MATCH_MGR,
                                             mut bp: *mut TAG_DATA,
                                             mut cf_ptr: *mut CASE_FRAME,
                                             mut n: libc::c_int,
                                             mut loc: libc::c_int,
                                             mut vs: *mut SENTENCE_DATA,
                                             mut vp: *mut TAG_DATA)
                                             -> *mut E_FEATURES {
    let mut f: *mut E_FEATURES = 0 as *mut E_FEATURES;
    let mut level: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut en: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    f =
        malloc_data(::std::mem::size_of::<E_FEATURES>() as libc::c_ulong,
                    b"SetEllipsisFeatures\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut E_FEATURES;
    if !(*(*cpm_ptr).pred_b_ptr).c_cpm_ptr.is_null() {
        (*f).ellipsis_class =
            get_example_class((*(*cpm_ptr).pred_b_ptr).c_cpm_ptr, s, bp,
                              cf_ptr, n)
    } else { (*f).ellipsis_class = 0 as libc::c_int }
    (*f).pos = -(2 as libc::c_int);
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
        (*f).similarity =
            calc_similarity_word_cf_with_sm(bp, cf_ptr, n, &mut (*f).pos)
    } else {
        (*f).similarity =
            calc_similarity_word_cf(bp, cf_ptr, n, &mut (*f).pos);
        (*f).match_sm_flag = cf_match_sm_thesaurus(bp, cf_ptr, n)
    }
    (*f).frequency =
        if (*f).similarity as libc::c_double > 1.0f64 {
            *(*cf_ptr).ex_freq[n as usize].offset((*f).pos as isize)
        } else { 0 as libc::c_int };
    en = CheckEntity((*(*bp).head_ptr).Goi.as_mut_ptr());
    if !en.is_null() {
        (*f).refered_num_surface = (*en).surface_num as libc::c_float;
        (*f).refered_num_ellipsis = (*en).ellipsis_num as libc::c_float
    } else {
        (*f).refered_num_surface = 0 as libc::c_int as libc::c_float;
        (*f).refered_num_ellipsis = 0 as libc::c_int as libc::c_float
    }
    if !vp.is_null() {
        (*f).event1 =
            get_cf_event_value((*(*vp).cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr,
                               (*cmm_ptr).cf_ptr);
        (*f).event2 =
            get_cf_event_value((*cmm_ptr).cf_ptr,
                               (*(*vp).cpm_ptr).cmm[0 as libc::c_int as
                                   usize].cf_ptr);
        (*f).c_pp =
            GetCandCase((*vp).cpm_ptr,
                        &mut *(*(*vp).cpm_ptr).cmm.as_mut_ptr().offset(0 as
                            libc::c_int
                            as
                            isize),
                        bp);
        level =
            check_feature((*vp).f,
                          b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !level.is_null() {
            strcpy((*f).c_dep_p_level.as_mut_ptr(),
                   level.offset(7 as libc::c_int as isize));
        } else {
            (*f).c_dep_p_level[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        }
        (*f).c_dep_mc_flag =
            if !check_feature((*vp).f,
                              b"\xe4\xb8\xbb\xe7\xaf\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        (*f).c_n_modify_flag =
            if !check_feature((*vp).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                1 as libc::c_int
            } else { 0 as libc::c_int }
    } else {
        (*f).event1 = -(1 as libc::c_int) as libc::c_float;
        (*f).event2 = -(1 as libc::c_int) as libc::c_float;
        (*f).c_pp = -(1 as libc::c_int);
        (*f).c_dep_p_level[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        (*f).c_dep_mc_flag = 0 as libc::c_int;
        (*f).c_n_modify_flag = 0 as libc::c_int
    }
    (*f).c_distance =
        cs.wrapping_offset_from(vs) as libc::c_long as libc::c_int;
    if s == vs {
        (*f).c_dist_bnst =
            CountBnstDistance(s, (*bp).num, cs, (*(*cpm_ptr).pred_b_ptr).num);
        (*f).c_fs_flag =
            if (*s).Sen_num == 1 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        if (*f).c_distance > 0 as libc::c_int ||
            (*f).c_distance == 0 as libc::c_int &&
                (*bp).num < (*(*cpm_ptr).pred_b_ptr).num {
            (*f).c_prev_p_flag = 1 as libc::c_int
        } else { (*f).c_prev_p_flag = 0 as libc::c_int }
        if (*f).c_distance == 0 as libc::c_int &&
            (*bp).num < (*(*cpm_ptr).pred_b_ptr).num &&
            (*bp).dpnd_head > (*(*cpm_ptr).pred_b_ptr).num {
            (*f).c_get_over_p_flag = 1 as libc::c_int
        } else { (*f).c_get_over_p_flag = 0 as libc::c_int }
    } else {
        (*f).c_dist_bnst =
            CountBnstDistance(vs, (*vp).num, cs,
                              (*(*cpm_ptr).pred_b_ptr).num);
        (*f).c_fs_flag =
            if (*vs).Sen_num == 1 as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        if (*f).c_distance > 0 as libc::c_int ||
            (*f).c_distance == 0 as libc::c_int &&
                (*vp).num < (*(*cpm_ptr).pred_b_ptr).num {
            (*f).c_prev_p_flag = 1 as libc::c_int
        } else { (*f).c_prev_p_flag = 0 as libc::c_int }
        if (*f).c_distance == 0 as libc::c_int &&
            (*vp).num < (*(*cpm_ptr).pred_b_ptr).num &&
            (*vp).dpnd_head > (*(*cpm_ptr).pred_b_ptr).num {
            (*f).c_get_over_p_flag = 1 as libc::c_int
        } else { (*f).c_get_over_p_flag = 0 as libc::c_int }
    }
    (*f).c_location = loc;
    (*f).c_topic_flag =
        if !check_feature((*bp).f,
                          b"\xe4\xb8\xbb\xe9\xa1\x8c\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_no_topic_flag =
        if !check_feature((*bp).f,
                          b"\xe6\xba\x96\xe4\xb8\xbb\xe9\xa1\x8c\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_in_cnoun_flag =
        if (*bp).inum != 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_subject_flag =
        if sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char),
                     (*bp).SM_code.as_mut_ptr(),
                     if !check_feature((*bp).f,
                                       b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                                           as *const u8 as *const libc::c_char
                                           as *mut libc::c_char).is_null() {
                         1 as libc::c_int
                     } else { 2 as libc::c_int }) != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_sm_none_flag =
        if (*f).similarity < 0 as libc::c_int as libc::c_float {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*f).c_extra_tag = -(1 as libc::c_int);
    if !vp.is_null() &&
        strcmp((*(*(*cpm_ptr).pred_b_ptr).jiritu_ptr).Goi.as_mut_ptr(),
               (*(*vp).jiritu_ptr).Goi.as_mut_ptr()) == 0 {
        (*f).match_verb = 1 as libc::c_int
    } else { (*f).match_verb = 0 as libc::c_int }
    if OptAddSvmFeatureUtype != 0 {
        (*f).utype = get_utype(bp)
    } else { (*f).utype = 0 as libc::c_int }
    if OptAddSvmFeatureDiscourseDepth != 0 {
        (*f).discourse_depth = get_discourse_depth(bp)
    } else { (*f).discourse_depth = 0 as libc::c_int }
    if OptAddSvmFeatureObjectRecognition != 0 {
        (*f).objectrecognition = objectrecognition_match(bp, s)
    } else { (*f).objectrecognition = 0 as libc::c_int }
    SetEllipsisFeaturesForPred(f, cpm_ptr, cf_ptr, n);
    if (*f).c_pp == (*f).p_pp {
        (*f).match_case = 1 as libc::c_int
    } else { (*f).match_case = 0 as libc::c_int }
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn SetEllipsisFeaturesExtraTags(mut tag: libc::c_int,
                                                      mut cpm_ptr:
                                                      *mut CF_PRED_MGR,
                                                      mut cf_ptr:
                                                      *mut CASE_FRAME,
                                                      mut n: libc::c_int,
                                                      mut loc: libc::c_int)
                                                      -> *mut E_FEATURES {
    let mut f: *mut E_FEATURES = 0 as *mut E_FEATURES;
    let mut en: *mut ENTITY_LIST = 0 as *mut ENTITY_LIST;
    f =
        malloc_data(::std::mem::size_of::<E_FEATURES>() as libc::c_ulong,
                    b"SetEllipsisFeaturesExtraTags\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut E_FEATURES;
    memset(f as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<E_FEATURES>() as libc::c_ulong);
    if !(*(*cpm_ptr).pred_b_ptr).c_cpm_ptr.is_null() {
        (*f).ellipsis_class =
            get_example_class((*(*cpm_ptr).pred_b_ptr).c_cpm_ptr,
                              0 as *mut SENTENCE_DATA, 0 as *mut TAG_DATA,
                              cf_ptr, n)
    } else { (*f).ellipsis_class = 0 as libc::c_int }
    if strcmp(ExtraTags[tag as usize],
              b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\x00" as
                  *const u8 as *const libc::c_char) == 0 &&
        cf_match_element((*cf_ptr).sm[n as usize],
                         b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         0 as libc::c_int) != 0 {
        (*f).similarity =
            EX_match_subject as libc::c_float /
                11 as libc::c_int as libc::c_float;
        (*f).pos = -(1 as libc::c_int)
    } else {
        (*f).similarity = -(1 as libc::c_int) as libc::c_float;
        (*f).pos = -(2 as libc::c_int)
    }
    en = CheckEntity(ExtraTags[tag as usize]);
    if !en.is_null() {
        (*f).refered_num_surface = (*en).surface_num as libc::c_float;
        (*f).refered_num_ellipsis = (*en).ellipsis_num as libc::c_float
    } else {
        (*f).refered_num_surface = 0 as libc::c_int as libc::c_float;
        (*f).refered_num_ellipsis = 0 as libc::c_int as libc::c_float
    }
    (*f).c_pp = -(1 as libc::c_int);
    (*f).c_distance = 0 as libc::c_int;
    (*f).c_dist_bnst = 0 as libc::c_int;
    (*f).c_location = loc;
    (*f).c_extra_tag = tag;
    if OptAddSvmFeatureUtype != 0 {
        (*f).utype = 0 as libc::c_int
    } else { (*f).utype = 0 as libc::c_int }
    if OptAddSvmFeatureDiscourseDepth != 0 {
        (*f).discourse_depth = 0 as libc::c_int
    } else { (*f).discourse_depth = 0 as libc::c_int }
    (*f).objectrecognition = 0 as libc::c_int;
    SetEllipsisFeaturesForPred(f, cpm_ptr, cf_ptr, n);
    if (*f).c_pp == (*f).p_pp {
        (*f).match_case = 1 as libc::c_int
    } else { (*f).match_case = 0 as libc::c_int }
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn classify_by_learning(mut ecp: *mut libc::c_char,
                                              mut pp: libc::c_int,
                                              mut method: libc::c_int)
                                              -> libc::c_float {
    if !(method == 2 as libc::c_int) {
        if method == 3 as libc::c_int { return dt_classify(ecp, pp); }
    }
    return -(1 as libc::c_int) as libc::c_float;
}

#[no_mangle]
pub unsafe extern "C" fn ScoreCheckCore(mut cf_ptr: *mut CASE_FRAME,
                                        mut n: libc::c_int,
                                        mut score: libc::c_float,
                                        mut pos: libc::c_int) -> libc::c_int {
    if MatchPP((*cf_ptr).pp[n as usize][0 as libc::c_int as usize],
               b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char) != 0 {
        if score > AntecedentDecideThresholdForNi {
            return 1 as libc::c_int;
        } else { if pos == -(1 as libc::c_int) { return 1 as libc::c_int; } }
    } else if MatchPP((*cf_ptr).pp[n as usize][0 as libc::c_int as usize],
                      b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char) != 0 {
        if score > AntecedentDecideThresholdForGa { return 1 as libc::c_int; }
    } else if (*cf_ptr).type_0 == 1 as libc::c_int &&
        score > AntecedentDecideThresholdPredGeneral ||
        (*cf_ptr).type_0 == 2 as libc::c_int &&
            score >= AntecedentDecideThresholdForNounBonus1 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ScoreCheck(mut cf_ptr: *mut CASE_FRAME,
                                    mut n: libc::c_int) -> libc::c_int {
    let mut value: libc::c_int = 0 as libc::c_int;
    if (*cf_ptr).type_0 == 1 as libc::c_int &&
        OptDiscFlag & 16 as libc::c_int != 0 {
        return 0 as libc::c_int;
    } else {
        if OptDiscFlag & 4 as libc::c_int != 0 {
            if !maxs.is_null() { value = 1 as libc::c_int }
        } else { value = ScoreCheckCore(cf_ptr, n, maxscore, maxpos) }
    }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        if value != 0 && AlreadyDecidedFlag == 0 {
            AlreadyDecidedFlag = 1 as libc::c_int
        }
        return 0 as libc::c_int;
    }
    return value;
}

#[no_mangle]
pub unsafe extern "C" fn push_cand(mut ef: *mut E_FEATURES,
                                   mut s: *mut SENTENCE_DATA,
                                   mut tp: *mut TAG_DATA,
                                   mut tag: *mut libc::c_char,
                                   mut cf_ptr: *mut CASE_FRAME,
                                   mut n: libc::c_int) {
    if ScoreCheckCore(cf_ptr, n, (*ef).similarity, 0 as libc::c_int) == 0 {
        return;
    }
    while cand_num >= cand_num_max {
        if cand_num_max == 0 as libc::c_int {
            cand_num_max = 1 as libc::c_int;
            ante_cands =
                malloc_data((::std::mem::size_of::<E_CANDIDATE>() as
                    libc::c_ulong).wrapping_mul(cand_num_max as
                    libc::c_ulong),
                            b"push_cand\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut E_CANDIDATE
        } else {
            cand_num_max <<= 1 as libc::c_int;
            ante_cands =
                realloc_data(ante_cands as *mut libc::c_void,
                             (::std::mem::size_of::<E_CANDIDATE>() as
                                 libc::c_ulong).wrapping_mul(cand_num_max as
                                 libc::c_ulong),
                             b"push_cand\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) as
                    *mut E_CANDIDATE
        }
    }
    let ref mut fresh32 = (*ante_cands.offset(cand_num as isize)).ef;
    *fresh32 = ef;
    let ref mut fresh33 = (*ante_cands.offset(cand_num as isize)).s;
    *fresh33 = s;
    let ref mut fresh34 = (*ante_cands.offset(cand_num as isize)).tp;
    *fresh34 = tp;
    let ref mut fresh35 = (*ante_cands.offset(cand_num as isize)).tag;
    *fresh35 = tag;
    cand_num += 1;
}

#[no_mangle]
pub unsafe extern "C" fn print_svm_feature(mut ante_cands_0: *mut E_CANDIDATE,
                                           mut i: libc::c_int,
                                           mut em_ptr: *mut ELLIPSIS_MGR,
                                           mut cpm_ptr: *mut CF_PRED_MGR) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ecf: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
    ecf =
        EllipsisFeatures2EllipsisSvmFeatures((*ante_cands_0.offset(i as
            isize)).ef,
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int);
    cp = EllipsisSvmFeatures2String(ecf);
    if PrintEx != 0 || OptDisplay == 3 as libc::c_int {
        fprintf(stderr,
                b";; \xe2\x98\x85 SVM\xe5\xad\xa6\xe7\xbf\x92Feature(for %s %s) %s %d: \xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6=%f, \xe9\xa0\xbb\xe5\xba\xa6=%d, \xe4\xbd\x8d\xe7\xbd\xaeC=%s, \xe6\xb7\xb1\xe3\x81\x95=%d, \xe7\x99\xba\xe8\xa9\xb1\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x97=%d, \xe5\x87\xba\xe7\x8f\xbe\xe5\x9b\x9e\xe6\x95\xb0=%.3f, \xe7\x9c\x81\xe7\x95\xa5\xe5\x8f\x82\xe7\x85\xa7\xe5\x9b\x9e\xe6\x95\xb0=%.3f, \xe5\x85\x88\xe8\xa1\x8c\xe8\xa9\x9e\xe6\xa0\xbc=%s, \xe5\x85\x88\xe8\xa1\x8c\xe8\xa9\x9e\xe7\xaf\x80=%s, \xe4\xb8\xbb\xe7\xaf\x80=%d, \xe9\x80\xa3\xe6\xa0\xbc=%d, \xe4\xb8\xbb\xe9\xa1\x8c=%d, \xe6\xba\x96\xe4\xb8\xbb\xe9\xa1\x8c=%d, \xe8\xa4\x87\xe5\x90\x88\xe5\x90\x8d\xe8\xa9\x9e=%d, \xe4\xbe\x8b\xe5\xa4\x96=%d, \xe7\x94\xa8\xe8\xa8\x80\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x97=%d, \xe7\x94\xa8\xe8\xa8\x80\xe6\x85\x8b=%d, \xe7\x94\xa8\xe8\xa8\x80\xe7\xaf\x80=%s, \xe7\x94\xa8\xe8\xa8\x80\xe4\xb8\xbb\xe4\xbd\x93=%d, \xe7\x94\xa8\xe8\xa8\x80\xe8\xa3\x9c\xe6\x96\x87=%d, \xe7\x94\xa8\xe8\xa8\x80\xe9\x80\xa3\xe6\xa0\xbc=%d, \xe6\xa0\xbc\xe4\xb8\x80\xe8\x87\xb4=%d, \xe7\x94\xa8\xe8\xa8\x80\xe4\xb8\x80\xe8\x87\xb4=%d\n\x00"
                    as *const u8 as *const libc::c_char,
                pp_code_to_kstr_in_context(cpm_ptr,
                                           (*(*ante_cands_0.offset(i as
                                               isize)).ef).p_pp),
                (*(*(*cpm_ptr).pred_b_ptr).jiritu_ptr).Goi.as_mut_ptr(),
                if !(*ante_cands_0.offset(i as isize)).tp.is_null() {
                    (*(*(*ante_cands_0.offset(i as
                        isize)).tp).head_ptr).Goi.as_mut_ptr()
                } else { (*ante_cands_0.offset(i as isize)).tag },
                (*(*ante_cands_0.offset(i as isize)).ef).ellipsis_class,
                (*(*ante_cands_0.offset(i as isize)).ef).similarity as
                    libc::c_double,
                (*(*ante_cands_0.offset(i as isize)).ef).frequency,
                loc_code_to_str((*(*ante_cands_0.offset(i as
                    isize)).ef).c_location),
                (*(*ante_cands_0.offset(i as isize)).ef).discourse_depth,
                (*(*ante_cands_0.offset(i as isize)).ef).utype,
                (*(*ante_cands_0.offset(i as isize)).ef).refered_num_surface
                    as libc::c_double,
                (*(*ante_cands_0.offset(i as isize)).ef).refered_num_ellipsis
                    as libc::c_double,
                if (*(*ante_cands_0.offset(i as isize)).ef).c_pp >
                    0 as libc::c_int {
                    pp_code_to_kstr((*(*ante_cands_0.offset(i as
                        isize)).ef).c_pp)
                        as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*(*ante_cands_0.offset(i as
                    isize)).ef).c_dep_p_level.as_mut_ptr(),
                (*(*ante_cands_0.offset(i as isize)).ef).c_dep_mc_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).c_n_modify_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).c_topic_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).c_no_topic_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).c_in_cnoun_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).c_extra_tag,
                (*(*ante_cands_0.offset(i as isize)).ef).p_type,
                (*(*ante_cands_0.offset(i as isize)).ef).p_voice,
                (*(*ante_cands_0.offset(i as
                    isize)).ef).p_dep_p_level.as_mut_ptr(),
                (*(*ante_cands_0.offset(i as isize)).ef).p_cf_subject_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).p_cf_sentence_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).p_n_modify_flag,
                (*(*ante_cands_0.offset(i as isize)).ef).match_case,
                (*(*ante_cands_0.offset(i as isize)).ef).match_verb);
    }
    EllipsisSvmFeaturesString2Feature(em_ptr, cpm_ptr,
                                      (*(*ante_cands_0.offset(i as
                                          isize)).ef).ellipsis_class,
                                      cp,
                                      if !(*ante_cands_0.offset(i as
                                          isize)).tp.is_null()
                                      {
                                          (*(*(*ante_cands_0.offset(i as
                                              isize)).tp).head_ptr).Goi.as_mut_ptr()
                                      } else {
                                          (*ante_cands_0.offset(i as
                                              isize)).tag
                                      },
                                      (*(*ante_cands_0.offset(i as
                                          isize)).ef).p_pp,
                                      if !(*ante_cands_0.offset(i as
                                          isize)).s.is_null()
                                      {
                                          if !(*(*ante_cands_0.offset(i as
                                              isize)).s).KNPSID.is_null()
                                          {
                                              (*(*ante_cands_0.offset(i as
                                                  isize)).s).KNPSID.offset(5
                                                  as
                                                  libc::c_int
                                                  as
                                                  isize)
                                                  as *const libc::c_char
                                          } else {
                                              b"?\x00" as *const u8 as
                                                  *const libc::c_char
                                          }
                                      } else {
                                          b"-1\x00" as *const u8 as
                                              *const libc::c_char
                                      } as *mut libc::c_char,
                                      if !(*ante_cands_0.offset(i as
                                          isize)).tp.is_null()
                                      {
                                          (*(*ante_cands_0.offset(i as
                                              isize)).tp).num
                                      } else { -(1 as libc::c_int) },
                                      (*(*ante_cands_0.offset(i as
                                          isize)).ef).c_location);
    free(ecf as *mut libc::c_void);
    free(cp as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn clear_cands() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < cand_num {
        free((*ante_cands.offset(i as isize)).ef as *mut libc::c_void);
        i += 1
    }
    cand_num = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn classify_twin_candidate(mut sp: *mut SENTENCE_DATA,
                                                 mut em_ptr:
                                                 *mut ELLIPSIS_MGR,
                                                 mut cpm_ptr:
                                                 *mut CF_PRED_MGR)
                                                 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut max_num: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut score: libc::c_float = 0.;
    let mut max: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut closest_i: libc::c_int = 0;
    let mut closest_s: libc::c_int = 0;
    let mut closest_tnum: libc::c_int = 0;
    if cand_num == 0 as libc::c_int {
        return 0 as libc::c_int;
    } else {
        if cand_num > 1 as libc::c_int {
            if OptDiscFlag & 48 as libc::c_int != 0 {
                if OptAnaphoraBaseline != 0 {
                    closest_i = -(1 as libc::c_int);
                    closest_s = -(1 as libc::c_int);
                    closest_tnum = -(1 as libc::c_int);
                    i = 0 as libc::c_int;
                    while i < cand_num {
                        if OptLearn == (0 as libc::c_int == 0) as libc::c_int
                        {
                            print_svm_feature(ante_cands, i, em_ptr, cpm_ptr);
                        }
                        score = 0 as libc::c_int as libc::c_float;
                        sprintf(feature_buffer.as_mut_ptr(),
                                b"C\xe7\x94\xa8;%s;%s;%s;%d;%d;%.3f|%.3f\x00"
                                    as *const u8 as *const libc::c_char,
                                if !(*ante_cands.offset(i as
                                    isize)).tp.is_null()
                                {
                                    (*(*(*ante_cands.offset(i as
                                        isize)).tp).head_ptr).Goi.as_mut_ptr()
                                } else {
                                    (*ante_cands.offset(i as isize)).tag
                                },
                                pp_code_to_kstr_in_context(cpm_ptr,
                                                           (*(*ante_cands.offset(i
                                                               as
                                                               isize)).ef).p_pp),
                                loc_code_to_str((*(*ante_cands.offset(i as
                                    isize)).ef).c_location),
                                (*(*ante_cands.offset(i as
                                    isize)).ef).c_distance,
                                if !(*ante_cands.offset(i as
                                    isize)).tp.is_null()
                                {
                                    (*(*ante_cands.offset(i as isize)).tp).num
                                } else { -(1 as libc::c_int) },
                                (*(*ante_cands.offset(i as
                                    isize)).ef).similarity
                                    as libc::c_double,
                                score as libc::c_double);
                        assign_cfeature(&mut (*em_ptr).f,
                                        feature_buffer.as_mut_ptr(),
                                        0 as libc::c_int);
                        if !(*ante_cands.offset(i as isize)).s.is_null() {
                            if (*(*ante_cands.offset(i as isize)).s).Sen_num >
                                closest_s {
                                closest_s =
                                    (*(*ante_cands.offset(i as
                                        isize)).s).Sen_num;
                                closest_tnum =
                                    (*(*ante_cands.offset(i as
                                        isize)).tp).num;
                                closest_i = i
                            } else if (*(*ante_cands.offset(i as
                                isize)).s).Sen_num
                                == closest_s {
                                if (*(*ante_cands.offset(i as isize)).tp).num
                                    >= closest_tnum {
                                    closest_s =
                                        (*(*ante_cands.offset(i as
                                            isize)).s).Sen_num;
                                    closest_tnum =
                                        (*(*ante_cands.offset(i as
                                            isize)).tp).num;
                                    closest_i = i
                                }
                            }
                        } else if OptAnaphoraBaseline == 2 as libc::c_int {
                            closest_i = i;
                            break;
                        }
                        i += 1
                    }
                    if closest_i >= 0 as libc::c_int { max_num = closest_i }
                } else {
                    let mut ecf: *mut E_SVM_FEATURES =
                        0 as *mut E_SVM_FEATURES;
                    max = -(100000 as libc::c_int) as libc::c_float;
                    i = 0 as libc::c_int;
                    while i < cand_num {
                        if OptLearn == (0 as libc::c_int == 0) as libc::c_int
                            || OptDisplay == 3 as libc::c_int {
                            print_svm_feature(ante_cands, i, em_ptr, cpm_ptr);
                        }
                        ecf =
                            EllipsisFeatures2EllipsisSvmFeatures((*ante_cands.offset(i
                                as
                                isize)).ef,
                                                                 0 as
                                                                     libc::c_int);
                        cp = EllipsisSvmFeatures2String(ecf);
                        score =
                            classify_by_learning(cp,
                                                 if (*cpm_ptr).cf.type_0 ==
                                                     1 as libc::c_int {
                                                     (*(*ante_cands.offset(i
                                                         as
                                                         isize)).ef).p_pp
                                                 } else {
                                                     pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                         as
                                                         *const u8
                                                         as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char)
                                                 },
                                                 if (*cpm_ptr).cf.type_0 ==
                                                     1 as libc::c_int {
                                                     OptDiscPredMethod
                                                 } else {
                                                     OptDiscNounMethod
                                                 });
                        if max < score {
                            max = score;
                            max_num = i
                        }
                        sprintf(feature_buffer.as_mut_ptr(),
                                b"C\xe7\x94\xa8;%s;%s;%s;%d;%d;%.3f|%.3f\x00"
                                    as *const u8 as *const libc::c_char,
                                if !(*ante_cands.offset(i as
                                    isize)).tp.is_null()
                                {
                                    (*(*(*ante_cands.offset(i as
                                        isize)).tp).head_ptr).Goi.as_mut_ptr()
                                } else {
                                    (*ante_cands.offset(i as isize)).tag
                                },
                                pp_code_to_kstr_in_context(cpm_ptr,
                                                           (*(*ante_cands.offset(i
                                                               as
                                                               isize)).ef).p_pp),
                                loc_code_to_str((*(*ante_cands.offset(i as
                                    isize)).ef).c_location),
                                (*(*ante_cands.offset(i as
                                    isize)).ef).c_distance,
                                if !(*ante_cands.offset(i as
                                    isize)).tp.is_null()
                                {
                                    (*(*ante_cands.offset(i as isize)).tp).num
                                } else { -(1 as libc::c_int) },
                                (*(*ante_cands.offset(i as
                                    isize)).ef).similarity
                                    as libc::c_double,
                                score as libc::c_double);
                        assign_cfeature(&mut (*em_ptr).f,
                                        feature_buffer.as_mut_ptr(),
                                        0 as libc::c_int);
                        free(ecf as *mut libc::c_void);
                        free(cp as *mut libc::c_void);
                        i += 1
                    }
                }
            } else {
                let mut f: *mut E_TWIN_CAND_SVM_FEATURES =
                    0 as *mut E_TWIN_CAND_SVM_FEATURES;
                let mut vote: *mut libc::c_int = 0 as *mut libc::c_int;
                vote =
                    malloc_data((::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(cand_num as
                        libc::c_ulong),
                                b"classify_twin_candidate\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_int;
                i = 0 as libc::c_int;
                while i < cand_num {
                    *vote.offset(i as isize) = 0 as libc::c_int;
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < cand_num - 1 as libc::c_int {
                    j = i + 1 as libc::c_int;
                    while j < cand_num {
                        f =
                            MakeTwinCandSvmFeatures((*ante_cands.offset(i as
                                isize)).ef,
                                                    (*ante_cands.offset(j as
                                                        isize)).ef);
                        cp = TwinCandSvmFeatures2String(f);
                        if OptLearn == (0 as libc::c_int == 0) as libc::c_int
                        {
                            TwinCandSvmFeaturesString2Feature(em_ptr, cp,
                                                              ante_cands.offset(i
                                                                  as
                                                                  isize),
                                                              ante_cands.offset(j
                                                                  as
                                                                  isize));
                        }
                        score =
                            classify_by_learning(cp,
                                                 if (*cpm_ptr).cf.type_0 ==
                                                     1 as libc::c_int {
                                                     (*(*ante_cands.offset(i
                                                         as
                                                         isize)).ef).p_pp
                                                 } else {
                                                     pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                         as
                                                         *const u8
                                                         as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char)
                                                 },
                                                 if (*cpm_ptr).cf.type_0 ==
                                                     1 as libc::c_int {
                                                     OptDiscPredMethod
                                                 } else {
                                                     OptDiscNounMethod
                                                 });
                        if score > 0 as libc::c_int as libc::c_float {
                            let ref mut fresh36 = *vote.offset(i as isize);
                            *fresh36 += 1
                        } else {
                            let ref mut fresh37 = *vote.offset(j as isize);
                            *fresh37 += 1
                        }
                        free(f as *mut libc::c_void);
                        free(cp as *mut libc::c_void);
                        j += 1
                    }
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < cand_num {
                    if max < *vote.offset(i as isize) as libc::c_float {
                        max = *vote.offset(i as isize) as libc::c_float;
                        max_num = i
                    }
                    sprintf(feature_buffer.as_mut_ptr(),
                            b"C\xe7\x94\xa8;%s;%s;%s;%d;%d;%.3f|%.3f\x00" as
                                *const u8 as *const libc::c_char,
                            if !(*ante_cands.offset(i as isize)).tp.is_null()
                            {
                                (*(*(*ante_cands.offset(i as
                                    isize)).tp).head_ptr).Goi.as_mut_ptr()
                            } else { (*ante_cands.offset(i as isize)).tag },
                            pp_code_to_kstr_in_context(cpm_ptr,
                                                       (*(*ante_cands.offset(i
                                                           as
                                                           isize)).ef).p_pp),
                            loc_code_to_str((*(*ante_cands.offset(i as
                                isize)).ef).c_location),
                            (*(*ante_cands.offset(i as isize)).ef).c_distance,
                            if !(*ante_cands.offset(i as isize)).tp.is_null()
                            {
                                (*(*ante_cands.offset(i as isize)).tp).num
                            } else { -(1 as libc::c_int) },
                            (*(*ante_cands.offset(i as isize)).ef).similarity
                                as libc::c_double,
                            (*vote.offset(i as isize) as libc::c_float /
                                cand_num as libc::c_float) as
                                libc::c_double);
                    assign_cfeature(&mut (*em_ptr).f,
                                    feature_buffer.as_mut_ptr(),
                                    0 as libc::c_int);
                    i += 1
                }
                free(vote as *mut libc::c_void);
            }
        } else { max = 1 as libc::c_int as libc::c_float }
    }
    maxrawscore = (*(*ante_cands.offset(max_num as isize)).ef).similarity;
    maxscore = maxrawscore;
    maxs = (*ante_cands.offset(max_num as isize)).s;
    maxpos = (*(*ante_cands.offset(max_num as isize)).ef).pos;
    maxi =
        if !(*ante_cands.offset(max_num as isize)).tp.is_null() {
            (*(*ante_cands.offset(max_num as isize)).tp).num
        } else { -(1 as libc::c_int) };
    maxtag = (*ante_cands.offset(max_num as isize)).tag;
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectSubcontractExtraTagsWithLearning(mut cs:
                                                                        *mut SENTENCE_DATA,
                                                                        mut em_ptr:
                                                                        *mut ELLIPSIS_MGR,
                                                                        mut cpm_ptr:
                                                                        *mut CF_PRED_MGR,
                                                                        mut cmm_ptr:
                                                                        *mut CF_MATCH_MGR,
                                                                        mut l:
                                                                        libc::c_int,
                                                                        mut tag:
                                                                        libc::c_int,
                                                                        mut cf_ptr:
                                                                        *mut CASE_FRAME,
                                                                        mut n:
                                                                        libc::c_int,
                                                                        mut loc:
                                                                        libc::c_int) {
    let mut ef: *mut E_FEATURES = 0 as *mut E_FEATURES;
    let mut esf: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
    let mut score: libc::c_float = 0.;
    let mut ecp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    ef = SetEllipsisFeaturesExtraTags(tag, cpm_ptr, cf_ptr, n, loc);
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
        OptDiscFlag & 16 as libc::c_int != 0 {
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int ||
            CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                       0 as *mut libc::c_char) == 0 &&
                !(strcmp((*cf_ptr).pred_type.as_mut_ptr(),
                         b"\xe5\x88\xa4\x00" as *const u8 as
                             *const libc::c_char) == 0 &&
                    sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                        *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char),
                              (*(*cpm_ptr).pred_b_ptr).SM_code.as_mut_ptr(),
                              1 as libc::c_int) == 0 &&
                    MatchPP((*cf_ptr).pp[n as
                        usize][0 as libc::c_int as
                        usize],
                            b"\xe3\x82\xac\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 &&
                    (*cf_ptr).etcflag & 2 as libc::c_int != 0) {
            push_cand(ef, 0 as *mut SENTENCE_DATA, 0 as *mut TAG_DATA,
                      ExtraTags[tag as usize], cf_ptr, n);
        }
        return;
    }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        esf =
            EllipsisFeatures2EllipsisSvmFeatures(ef,
                                                 (0 as libc::c_int == 0) as
                                                     libc::c_int);
        ecp = EllipsisSvmFeatures2String(esf);
        EllipsisSvmFeaturesString2Feature(em_ptr, cpm_ptr,
                                          (*ef).ellipsis_class, ecp,
                                          ExtraTags[tag as usize],
                                          (*cf_ptr).pp[n as
                                              usize][0 as
                                              libc::c_int
                                              as
                                              usize],
                                          b"?\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char,
                                          -(1 as libc::c_int),
                                          -(1 as libc::c_int));
    }
    esf = EllipsisFeatures2EllipsisSvmFeatures(ef, 0 as libc::c_int);
    ecp = EllipsisSvmFeatures2String(esf);
    score =
        classify_by_learning(ecp,
                             if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
                                 (*cf_ptr).pp[n as
                                     usize][0 as libc::c_int as
                                     usize]
                             } else {
                                 pp_kstr_to_code(b"\xe3\x83\x8e\x00" as
                                     *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char)
                             },
                             if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
                                 OptDiscPredMethod
                             } else { OptDiscNounMethod });
    if score > maxscore {
        maxscore = score;
        maxrawscore = 1.0f64 as libc::c_float;
        maxtag = ExtraTags[tag as usize]
    }
    free(ef as *mut libc::c_void);
    free(esf as *mut libc::c_void);
    free(ecp as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn _EllipsisDetectSubcontractWithLearning(mut s:
                                                                *mut SENTENCE_DATA,
                                                                mut cs:
                                                                *mut SENTENCE_DATA,
                                                                mut em_ptr:
                                                                *mut ELLIPSIS_MGR,
                                                                mut cpm_ptr:
                                                                *mut CF_PRED_MGR,
                                                                mut cmm_ptr:
                                                                *mut CF_MATCH_MGR,
                                                                mut l:
                                                                libc::c_int,
                                                                mut bp:
                                                                *mut TAG_DATA,
                                                                mut cf_ptr:
                                                                *mut CASE_FRAME,
                                                                mut n:
                                                                libc::c_int,
                                                                mut loc:
                                                                libc::c_int,
                                                                mut vs:
                                                                *mut SENTENCE_DATA,
                                                                mut vp:
                                                                *mut TAG_DATA) {
    let mut ef: *mut E_FEATURES = 0 as *mut E_FEATURES;
    let mut esf: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
    let mut ecp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut score: libc::c_float = 0.;
    let mut similarity: libc::c_float = 0.;
    ef =
        SetEllipsisFeatures(s, cs, cpm_ptr, cmm_ptr, bp, cf_ptr, n, loc, vs,
                            vp);
    if OptDiscFlag & 16 as libc::c_int != 0 {
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int ||
            CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                       (*(*bp).head_ptr).Goi.as_mut_ptr())
                == 0 {
            push_cand(ef, s, bp, 0 as *mut libc::c_char, cf_ptr, n);
        }
        return;
    }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        esf =
            EllipsisFeatures2EllipsisSvmFeatures(ef,
                                                 (0 as libc::c_int == 0) as
                                                     libc::c_int);
        ecp = EllipsisSvmFeatures2String(esf);
        EllipsisSvmFeaturesString2Feature(em_ptr, cpm_ptr,
                                          (*ef).ellipsis_class, ecp,
                                          (*(*bp).head_ptr).Goi.as_mut_ptr(),
                                          (*cf_ptr).pp[n as
                                              usize][0 as
                                              libc::c_int
                                              as
                                              usize],
                                          if !(*s).KNPSID.is_null() {
                                              (*s).KNPSID.offset(5 as
                                                  libc::c_int
                                                  as isize)
                                                  as *const libc::c_char
                                          } else {
                                              b"?\x00" as *const u8 as
                                                  *const libc::c_char
                                          } as *mut libc::c_char, (*bp).num,
                                          loc);
        free(esf as *mut libc::c_void);
        free(ecp as *mut libc::c_void);
    }
    esf = EllipsisFeatures2EllipsisSvmFeatures(ef, 0 as libc::c_int);
    ecp = EllipsisSvmFeatures2String(esf);
    if CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                  (*(*bp).head_ptr).Goi.as_mut_ptr()) != 0 {
        free(ef as *mut libc::c_void);
        free(esf as *mut libc::c_void);
        free(ecp as *mut libc::c_void);
        return;
    }
    if (*cpm_ptr).cf.type_0 == 2 as libc::c_int {
        if (*ef).similarity >= AntecedentDecideThresholdForNoun {
            score =
                classify_by_learning(ecp,
                                     pp_kstr_to_code(b"\xe3\x83\x8e\x00" as
                                         *const u8 as
                                         *const libc::c_char
                                         as
                                         *mut libc::c_char),
                                     OptDiscNounMethod);
            similarity = (*ef).similarity
        } else if (*ef).match_sm_flag != 0 &&
            (*ef).similarity >= AntecedentDecideThresholdForNounSM {
            score =
                classify_by_learning(ecp,
                                     pp_kstr_to_code(b"\xe3\x83\x8e\x00" as
                                         *const u8 as
                                         *const libc::c_char
                                         as
                                         *mut libc::c_char),
                                     OptDiscNounMethod);
            similarity =
                EX_match_subject as libc::c_float /
                    11 as libc::c_int as libc::c_float;
            (*ef).pos = -(1 as libc::c_int)
        } else {
            score = -(1 as libc::c_int) as libc::c_float;
            similarity = -(1 as libc::c_int) as libc::c_float
        }
    } else {
        score =
            classify_by_learning(ecp,
                                 (*cf_ptr).pp[n as
                                     usize][0 as libc::c_int as
                                     usize],
                                 OptDiscPredMethod);
        similarity = (*ef).similarity
    }
    sprintf(feature_buffer.as_mut_ptr(),
            b"C\xe7\x94\xa8;%s;%s;%s;%d;%d;%.3f|%.3f\x00" as *const u8 as
                *const libc::c_char, (*(*bp).head_ptr).Goi.as_mut_ptr(),
            pp_code_to_kstr_in_context(cpm_ptr,
                                       (*cf_ptr).pp[n as
                                           usize][0 as
                                           libc::c_int
                                           as usize]),
            loc_code_to_str(loc), (*ef).c_distance, (*bp).num,
            (*ef).similarity as libc::c_double, score as libc::c_double);
    assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                    0 as libc::c_int);
    if score > 0 as libc::c_int as libc::c_float {
        if OptDiscFlag & 4 as libc::c_int == 0 { score = similarity }
        if AlreadyDecidedFlag == 0 && score > maxscore {
            maxscore = score;
            maxrawscore = (*ef).similarity;
            maxs = s;
            maxpos = (*ef).pos;
            maxi = (*bp).num;
            maxtag = 0 as *mut libc::c_char
        }
    }
    free(ef as *mut libc::c_void);
    free(esf as *mut libc::c_void);
    free(ecp as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectSubcontractExtraTags(mut cs:
                                                            *mut SENTENCE_DATA,
                                                            mut em_ptr:
                                                            *mut ELLIPSIS_MGR,
                                                            mut cpm_ptr:
                                                            *mut CF_PRED_MGR,
                                                            mut cmm_ptr:
                                                            *mut CF_MATCH_MGR,
                                                            mut l:
                                                            libc::c_int,
                                                            mut tag:
                                                            libc::c_int,
                                                            mut cf_ptr:
                                                            *mut CASE_FRAME,
                                                            mut n:
                                                            libc::c_int,
                                                            mut loc:
                                                            libc::c_int)
                                                            -> libc::c_int {
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
        (OptDiscPredMethod == 2 as libc::c_int ||
            OptDiscPredMethod == 3 as libc::c_int) {
        EllipsisDetectSubcontractExtraTagsWithLearning(cs, em_ptr, cpm_ptr,
                                                       cmm_ptr, l, tag,
                                                       cf_ptr, n, loc);
    } else {
        let mut ef: *mut E_FEATURES = 0 as *mut E_FEATURES;
        let mut esf: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
        let mut ecp: *mut libc::c_char = 0 as *mut libc::c_char;
        ef = SetEllipsisFeaturesExtraTags(tag, cpm_ptr, cf_ptr, n, loc);
        if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
            OptDiscFlag & 16 as libc::c_int != 0 {
            if OptLearn == (0 as libc::c_int == 0) as libc::c_int ||
                CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                           0 as *mut libc::c_char) == 0 &&
                    !(strcmp((*cf_ptr).pred_type.as_mut_ptr(),
                             b"\xe5\x88\xa4\x00" as *const u8 as
                                 *const libc::c_char) == 0 &&
                        sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                            as *const u8 as
                            *const libc::c_char as
                            *mut libc::c_char),
                                  (*(*cpm_ptr).pred_b_ptr).SM_code.as_mut_ptr(),
                                  1 as libc::c_int) == 0 &&
                        MatchPP((*cf_ptr).pp[n as
                            usize][0 as libc::c_int
                            as usize],
                                b"\xe3\x82\xac\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char) != 0 &&
                        (*cf_ptr).etcflag & 2 as libc::c_int != 0) {
                push_cand(ef, 0 as *mut SENTENCE_DATA, 0 as *mut TAG_DATA,
                          ExtraTags[tag as usize], cf_ptr, n);
            }
            return 0 as libc::c_int;
        }
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
            esf =
                EllipsisFeatures2EllipsisSvmFeatures(ef,
                                                     (0 as libc::c_int == 0)
                                                         as libc::c_int);
            ecp = EllipsisSvmFeatures2String(esf);
            EllipsisSvmFeaturesString2Feature(em_ptr, cpm_ptr,
                                              (*ef).ellipsis_class, ecp,
                                              ExtraTags[tag as usize],
                                              (*cf_ptr).pp[n as
                                                  usize][0 as
                                                  libc::c_int
                                                  as
                                                  usize],
                                              b"?\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              -(1 as libc::c_int),
                                              -(1 as libc::c_int));
            free(esf as *mut libc::c_void);
            free(ecp as *mut libc::c_void);
        }
        free(ef as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn _EllipsisDetectSubcontract(mut s: *mut SENTENCE_DATA,
                                                    mut cs:
                                                    *mut SENTENCE_DATA,
                                                    mut em_ptr:
                                                    *mut ELLIPSIS_MGR,
                                                    mut cpm_ptr:
                                                    *mut CF_PRED_MGR,
                                                    mut cmm_ptr:
                                                    *mut CF_MATCH_MGR,
                                                    mut l: libc::c_int,
                                                    mut bp: *mut TAG_DATA,
                                                    mut cf_ptr:
                                                    *mut CASE_FRAME,
                                                    mut n: libc::c_int,
                                                    mut loc: libc::c_int,
                                                    mut vs:
                                                    *mut SENTENCE_DATA,
                                                    mut vp: *mut TAG_DATA) {
    let mut ef: *mut E_FEATURES = 0 as *mut E_FEATURES;
    let mut esf: *mut E_SVM_FEATURES = 0 as *mut E_SVM_FEATURES;
    // let mut tmp_bp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut ecp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut score: libc::c_float = 0.;
    ef =
        SetEllipsisFeatures(s, cs, cpm_ptr, cmm_ptr, bp, cf_ptr, n, loc, vs,
                            vp);
    if OptDiscFlag & 16 as libc::c_int != 0 {
        if OptLearn == (0 as libc::c_int == 0) as libc::c_int ||
            CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                       (*(*bp).head_ptr).Goi.as_mut_ptr())
                == 0 {
            push_cand(ef, s, bp, 0 as *mut libc::c_char, cf_ptr, n);
        }
        return;
    }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        esf =
            EllipsisFeatures2EllipsisSvmFeatures(ef,
                                                 (0 as libc::c_int == 0) as
                                                     libc::c_int);
        ecp = EllipsisSvmFeatures2String(esf);
        EllipsisSvmFeaturesString2Feature(em_ptr, cpm_ptr,
                                          (*ef).ellipsis_class, ecp,
                                          (*(*bp).head_ptr).Goi.as_mut_ptr(),
                                          (*cf_ptr).pp[n as
                                              usize][0 as
                                              libc::c_int
                                              as
                                              usize],
                                          if !(*s).KNPSID.is_null() {
                                              (*s).KNPSID.offset(5 as
                                                  libc::c_int
                                                  as isize)
                                                  as *const libc::c_char
                                          } else {
                                              b"?\x00" as *const u8 as
                                                  *const libc::c_char
                                          } as *mut libc::c_char, (*bp).num,
                                          loc);
        free(esf as *mut libc::c_void);
        free(ecp as *mut libc::c_void);
    }
    if CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l,
                                  (*(*bp).head_ptr).Goi.as_mut_ptr()) != 0 {
        free(ef as *mut libc::c_void);
        return;
    }
    if (*cpm_ptr).cf.type_0 == 2 as libc::c_int {
        if (*(*cpm_ptr).pred_b_ptr).num == (*bp).dpnd_head &&
            (*ef).similarity >= AntecedentDecideThresholdForNounBonus1 {
            score =
                (*ef).similarity + AntecedentDecideThresholdForNoun -
                    AntecedentDecideThresholdForNounBonus1
        }
        if (*ef).c_dist_bnst == 1 as libc::c_int &&
            !check_feature((*bp).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            (*ef).similarity >= AntecedentDecideThresholdForNounBonus1 {
            score =
                (*ef).similarity + AntecedentDecideThresholdForNoun -
                    AntecedentDecideThresholdForNounBonus1
        } else if (*ef).c_dist_bnst == 1 as libc::c_int &&
            (!check_feature((*bp).f,
                            b"\xe3\x83\x8f\x00" as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*bp).f,
                               b"\xe3\x83\x87\x00" as *const u8 as
                                   *const libc::c_char as
                                   *mut libc::c_char).is_null()) &&
            check_feature((*(*cpm_ptr).pred_b_ptr).f,
                          b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (*ef).similarity >=
                AntecedentDecideThresholdForNounBonus2 {
            score =
                (*ef).similarity + AntecedentDecideThresholdForNoun -
                    AntecedentDecideThresholdForNounBonus2
        } else if (*ef).similarity >= AntecedentDecideThresholdForNoun {
            score =
                ((*ef).similarity as libc::c_double +
                    (if (*ef).similarity > 1 as libc::c_int as libc::c_float
                    {
                        (0.05f64 * (*ef).frequency as libc::c_double) /
                            ((*ef).frequency + 100 as libc::c_int) as
                                libc::c_double
                    } else { 0 as libc::c_int as libc::c_double }) +
                    vs.wrapping_offset_from(cs) as libc::c_long as
                        libc::c_double * 0.01f64 +
                    (if !check_feature((*bp).f,
                                       b"\xe3\x83\x8f\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null() {
                        0.05f64
                    } else { 0 as libc::c_int as libc::c_double }) +
                    0.02f64 * (*ef).c_no_topic_flag as libc::c_double) as
                    libc::c_float;
            if (*ef).c_dist_bnst < 0 as libc::c_int &&
                (*ef).c_dist_bnst > -(5 as libc::c_int) {
                score = (score as libc::c_double + 0.12f64) as libc::c_float
            }
            if !(*(*cpm_ptr).pred_b_ptr).child.as_mut_ptr().is_null() &&
                !(*(*cpm_ptr).pred_b_ptr).child[0 as libc::c_int as
                    usize].is_null() &&
                !check_feature((*(*(*cpm_ptr).pred_b_ptr).child[0 as
                    libc::c_int
                    as
                    usize]).f,
                               b"\xe9\x80\xa3\xe4\xbd\x93\xe8\xa9\x9e\xe5\xbd\xa2\xe6\x85\x8b\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                (*ef).c_dist_bnst < 0 as libc::c_int {
                score = (score as libc::c_double - 0.1f64) as libc::c_float
            }
        } else if (*ef).match_sm_flag != 0 &&
            (*ef).similarity >= AntecedentDecideThresholdForNounSM {
            score =
                EX_match_subject as libc::c_float /
                    11 as libc::c_int as libc::c_float;
            (*ef).pos = -(1 as libc::c_int)
        } else { score = -(1 as libc::c_int) as libc::c_float }
    } else { score = (*ef).similarity }
    sprintf(feature_buffer.as_mut_ptr(),
            b"C\xe7\x94\xa8;%s;%s;%s;%d;%d;%.3f|%.3f\x00" as *const u8 as
                *const libc::c_char, (*(*bp).head_ptr).Goi.as_mut_ptr(),
            pp_code_to_kstr_in_context(cpm_ptr,
                                       (*cf_ptr).pp[n as
                                           usize][0 as
                                           libc::c_int
                                           as usize]),
            loc_code_to_str(loc), (*ef).c_distance, (*bp).num,
            (*ef).similarity as libc::c_double,
            (*ef).similarity as libc::c_double);
    assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                    0 as libc::c_int);
    if AlreadyDecidedFlag == 0 && score > maxscore {
        maxscore = score;
        maxrawscore = score;
        maxs = s;
        maxpos = (*ef).pos;
        maxi = (*bp).num;
        maxtag = 0 as *mut libc::c_char
    }
    free(ef as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectSubcontract(mut s: *mut SENTENCE_DATA,
                                                   mut cs: *mut SENTENCE_DATA,
                                                   mut em_ptr:
                                                   *mut ELLIPSIS_MGR,
                                                   mut cpm_ptr:
                                                   *mut CF_PRED_MGR,
                                                   mut cmm_ptr:
                                                   *mut CF_MATCH_MGR,
                                                   mut l: libc::c_int,
                                                   mut bp: *mut TAG_DATA,
                                                   mut cf_ptr:
                                                   *mut CASE_FRAME,
                                                   mut n: libc::c_int,
                                                   mut loc: libc::c_int,
                                                   mut vs: *mut SENTENCE_DATA,
                                                   mut vp: *mut TAG_DATA)
                                                   -> libc::c_int {
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
        OptDiscPredMethod != 1 as libc::c_int ||
        (*cpm_ptr).cf.type_0 == 2 as libc::c_int &&
            OptDiscNounMethod != 1 as libc::c_int {
        _EllipsisDetectSubcontractWithLearning(s, cs, em_ptr, cpm_ptr,
                                               cmm_ptr, l, bp, cf_ptr, n, loc,
                                               vs, vp);
    } else {
        _EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, bp,
                                   cf_ptr, n, loc, vs, vp);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn AppendToCF(mut cpm_ptr: *mut CF_PRED_MGR,
                                    mut cmm_ptr: *mut CF_MATCH_MGR,
                                    mut l: libc::c_int,
                                    mut b_ptr: *mut TAG_DATA,
                                    mut cf_ptr: *mut CASE_FRAME,
                                    mut n: libc::c_int,
                                    mut maxscore_0: libc::c_float,
                                    mut maxpos_0: libc::c_int,
                                    mut maxs_0: *mut SENTENCE_DATA)
                                    -> libc::c_int {
    let mut c_ptr: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    let mut d: libc::c_int = 0;
    let mut demonstrative: libc::c_int = 0;
    let mut old_score: libc::c_int = 0;
    if (*c_ptr).element_num >= 24 as libc::c_int { return 0 as libc::c_int; }
    if (*cmm_ptr).result_lists_p[l as usize].flag[n as usize] !=
        -(1 as libc::c_int) {
        d = (*cmm_ptr).result_lists_p[l as usize].flag[n as usize];
        old_score =
            (*cmm_ptr).result_lists_p[l as usize].score[n as usize] as
                libc::c_int;
        demonstrative = 1 as libc::c_int
    } else {
        d = (*c_ptr).element_num;
        demonstrative = 0 as libc::c_int
    }
    (*cmm_ptr).result_lists_p[l as usize].flag[n as usize] = d;
    (*cmm_ptr).result_lists_d[l as usize].flag[d as usize] = n;
    (*cmm_ptr).result_lists_p[l as usize].pos[n as usize] = maxpos_0;
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
        (OptDiscPredMethod == 2 as libc::c_int ||
            OptDiscPredMethod == 3 as libc::c_int) {
        if maxscore_0 < 0 as libc::c_int as libc::c_float {
            (*cmm_ptr).result_lists_p[l as usize].score[n as usize] =
                0 as libc::c_int as libc::c_double
        } else {
            (*cmm_ptr).result_lists_p[l as usize].score[n as usize] =
                if maxscore_0 > 1 as libc::c_int as libc::c_float {
                    EX_match_exact as libc::c_float
                } else if maxpos_0 == -(1 as libc::c_int) {
                    EX_match_subject as libc::c_float
                } else { (maxscore_0) * 11 as libc::c_int as libc::c_float }
                    as libc::c_double
        }
    } else {
        (*cmm_ptr).result_lists_p[l as usize].score[n as usize] =
            if maxscore_0 > 1 as libc::c_int as libc::c_float {
                EX_match_exact
            } else if maxpos_0 == -(1 as libc::c_int) {
                EX_match_subject
            } else {
                *EX_match_score.as_mut_ptr().offset((maxscore_0 *
                    7 as libc::c_int as
                        libc::c_float) as
                    libc::c_int as isize)
            } as libc::c_double
    }
    (*c_ptr).pp[d as usize][0 as libc::c_int as usize] =
        (*cf_ptr).pp[n as usize][0 as libc::c_int as usize];
    (*c_ptr).pp[d as usize][1 as libc::c_int as usize] = -(10 as libc::c_int);
    (*c_ptr).oblig[d as usize] = (0 as libc::c_int == 0) as libc::c_int;
    (*cpm_ptr).elem_b_ptr[d as usize] = b_ptr;
    (*cpm_ptr).elem_s_ptr[d as usize] = maxs_0;
    (*c_ptr).weight[d as usize] = 0 as libc::c_int;
    (*c_ptr).adjacent[d as usize] = 0 as libc::c_int;
    if demonstrative == 0 {
        (*cpm_ptr).elem_b_num[d as usize] = -(2 as libc::c_int);
        if !b_ptr.is_null() {
            _make_data_cframe_sm(cpm_ptr, b_ptr);
            _make_data_cframe_ex(cpm_ptr, b_ptr);
        }
        (*c_ptr).element_num += 1
    } else {
        (*cpm_ptr).elem_b_num[d as usize] = -(3 as libc::c_int);
        (*cmm_ptr).pure_score[l as usize] -= old_score as libc::c_double
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn DeleteFromCF(mut em_ptr: *mut ELLIPSIS_MGR,
                                      mut cpm_ptr: *mut CF_PRED_MGR,
                                      mut cmm_ptr: *mut CF_MATCH_MGR,
                                      mut l: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if (*cpm_ptr).elem_b_num[i as usize] <= -(2 as libc::c_int) {
            (*cmm_ptr).result_lists_p[l as
                usize].flag[(*cmm_ptr).result_lists_d[l
                as
                usize].flag[i
                as
                usize]
                as usize] =
                -(1 as libc::c_int);
            (*cmm_ptr).result_lists_d[l as usize].flag[i as usize] =
                -(1 as libc::c_int);
            count += 1
        }
        i += 1
    }
    (*cpm_ptr).cf.element_num -= count;
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn CheckAppropriateCandidate(mut s: *mut SENTENCE_DATA,
                                                   mut cs: *mut SENTENCE_DATA,
                                                   mut cpm_ptr:
                                                   *mut CF_PRED_MGR,
                                                   mut bp: *mut TAG_DATA,
                                                   mut pp: libc::c_int,
                                                   mut cf_ptr:
                                                   *mut CASE_FRAME,
                                                   mut n: libc::c_int,
                                                   mut loc: libc::c_int,
                                                   mut flag: libc::c_int)
                                                   -> libc::c_int {
    if OptNoCandidateBehind != 0 { flag = 0 as libc::c_int }
    if *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
        isize)).offset((*bp).num as isize) != 0 ||
        check_feature((*bp).f,
                      b"\xe5\x85\x88\xe8\xa1\x8c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
            (*cpm_ptr).cf.type_0 == 1 as libc::c_int ||
        check_feature((*bp).f,
                      b"\xe5\x90\x8d\xe8\xa9\x9e\xe9\xa0\x85\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
            check_feature((*bp).f,
                          b"\xe5\x85\x88\xe8\xa1\x8c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (*cpm_ptr).cf.type_0 == 2 as libc::c_int ||
        s == cs && (*bp).num == (*(*cpm_ptr).pred_b_ptr).num {
        return 0 as libc::c_int;
    }
    if OptLearn == (0 as libc::c_int == 0) as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if strcmp((*(*bp).head_ptr).Goi.as_mut_ptr(),
              (*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    if s == cs &&
        ((*bp).num >= (*(*cpm_ptr).pred_b_ptr).num &&
            ((*cpm_ptr).cf.type_0 == 1 as libc::c_int ||
                (*bp).num == (*(*cpm_ptr).pred_b_ptr).num ||
                flag == 0 &&
                    (*bp).dpnd_head !=
                        (*(*cpm_ptr).pred_b_ptr).dpnd_head) ||
            flag == 0 &&
                check_feature((*bp).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                (*bp).dpnd_head == (*(*cpm_ptr).pred_b_ptr).num ||
            (*(*cpm_ptr).pred_b_ptr).dpnd_head == (*bp).num ||
            flag == 0 && CheckCaseComponent(cpm_ptr, bp) != 0) {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectRecursive(mut s: *mut SENTENCE_DATA,
                                                 mut cs: *mut SENTENCE_DATA,
                                                 mut em_ptr:
                                                 *mut ELLIPSIS_MGR,
                                                 mut cpm_ptr:
                                                 *mut CF_PRED_MGR,
                                                 mut cmm_ptr:
                                                 *mut CF_MATCH_MGR,
                                                 mut l: libc::c_int,
                                                 mut tp: *mut TAG_DATA,
                                                 mut cf_ptr: *mut CASE_FRAME,
                                                 mut n: libc::c_int,
                                                 mut loc: libc::c_int)
                                                 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*tp).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int ||
        CheckAppropriateCandidate(s, cs, cpm_ptr, tp, -(1 as libc::c_int),
                                  cf_ptr, n, 0 as libc::c_int,
                                  0 as libc::c_int) == 0 {
        if *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
            isize)).offset((*tp).num as isize) == 0 {
            *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
                isize)).offset((*tp).num as isize) =
                1 as libc::c_int
        }
    } else if OptDiscFlag & 2 as libc::c_int != 0 ||
        (*cpm_ptr).cf.type_0 == 2 as libc::c_int ||
        loc == 0 as libc::c_int ||
        (loc == 0x10000 as libc::c_int ||
            loc == 0x20000 as libc::c_int) && s != cs ||
        loc == 0x8000 as libc::c_int && s == cs &&
            (*tp).num < (*(*cpm_ptr).pred_b_ptr).num ||
        loc == 0x9000 as libc::c_int && s == cs &&
            (*tp).num > (*(*cpm_ptr).pred_b_ptr).num {
        EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp,
                                  cf_ptr, n, loc, s, (*tp).pred_b_ptr);
        *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
            isize)).offset((*tp).num as isize) =
            1 as libc::c_int;
        if OptDiscFlag & 2 as libc::c_int == 0 && ScoreCheck(cf_ptr, n) != 0 {
            return 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while !(*tp).child[i as usize].is_null() {
        if EllipsisDetectRecursive(s, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                   (*tp).child[i as usize], cf_ptr, n, loc) ==
            1 as libc::c_int {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn CheckLocation(mut s: *mut SENTENCE_DATA,
                                       mut cs: *mut SENTENCE_DATA,
                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut tp: *mut TAG_DATA,
                                       mut loc: libc::c_int) -> libc::c_int {
    if loc == 0x10000 as libc::c_int || loc == 0x20000 as libc::c_int {
        return if s != cs {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    } else {
        if loc == 0x8000 as libc::c_int {
            return if s == cs && (*tp).num < (*(*cpm_ptr).pred_b_ptr).num {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        } else {
            if loc == 0x9000 as libc::c_int {
                return if s == cs && (*tp).num > (*(*cpm_ptr).pred_b_ptr).num {
                    1 as libc::c_int
                } else { 0 as libc::c_int };
            }
        }
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectRecursive2(mut s: *mut SENTENCE_DATA,
                                                  mut cs: *mut SENTENCE_DATA,
                                                  mut em_ptr:
                                                  *mut ELLIPSIS_MGR,
                                                  mut cpm_ptr:
                                                  *mut CF_PRED_MGR,
                                                  mut cmm_ptr:
                                                  *mut CF_MATCH_MGR,
                                                  mut l: libc::c_int,
                                                  mut tp: *mut TAG_DATA,
                                                  mut cf_ptr: *mut CASE_FRAME,
                                                  mut n: libc::c_int,
                                                  mut loc: libc::c_int,
                                                  mut rec_flag: libc::c_int)
                                                  -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    if (*tp).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int ||
        ((*cpm_ptr).cf.type_0 == 0) as libc::c_int == 2 as libc::c_int &&
            CheckAppropriateCandidate(s, cs, cpm_ptr, tp,
                                      -(1 as libc::c_int), cf_ptr, n,
                                      0 as libc::c_int, 0 as libc::c_int)
                == 0 ||
        (*cpm_ptr).cf.type_0 == 2 as libc::c_int &&
            CheckAppropriateCandidate(s, cs, cpm_ptr, tp,
                                      -(1 as libc::c_int), cf_ptr, n,
                                      0 as libc::c_int,
                                      (0 as libc::c_int == 0) as
                                          libc::c_int) == 0 {
        if *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
            isize)).offset((*tp).num as isize) == 0 {
            *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
                isize)).offset((*tp).num as isize) =
                1 as libc::c_int
        }
    } else if OptDiscFlag & 2 as libc::c_int != 0 ||
        (*cpm_ptr).cf.type_0 == 2 as libc::c_int ||
        loc == 0 as libc::c_int ||
        (loc == 0x10000 as libc::c_int ||
            loc == 0x20000 as libc::c_int) && s != cs ||
        loc == 0x8000 as libc::c_int && s == cs &&
            (*tp).num < (*(*cpm_ptr).pred_b_ptr).num ||
        loc == 0x9000 as libc::c_int && s == cs &&
            (*tp).num > (*(*cpm_ptr).pred_b_ptr).num {
        EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp,
                                  cf_ptr, n, loc, s, (*tp).pred_b_ptr);
        *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
            isize)).offset((*tp).num as isize) =
            1 as libc::c_int;
        if OptDiscFlag & 2 as libc::c_int == 0 && ScoreCheck(cf_ptr, n) != 0 {
            return 1 as libc::c_int;
        }
    }
    tp2 = tp;
    while (*tp2).para_top_p != 0 {
        tp2 = (*tp2).child[0 as libc::c_int as usize]
    }
    SearchCaseComponent(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp2, cf_ptr, n,
                        loc);
    if OptDiscFlag & 2 as libc::c_int == 0 && ScoreCheck(cf_ptr, n) != 0 {
        return 1 as libc::c_int;
    }
    if rec_flag == (0 as libc::c_int == 0) as libc::c_int {
        i = 0 as libc::c_int;
        while !(*tp).child[i as usize].is_null() {
            if EllipsisDetectRecursive2(s, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                        (*tp).child[i as usize], cf_ptr, n,
                                        loc,
                                        (0 as libc::c_int == 0) as
                                            libc::c_int) == 1 as libc::c_int {
                return 1 as libc::c_int;
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectOne(mut s: *mut SENTENCE_DATA,
                                           mut cs: *mut SENTENCE_DATA,
                                           mut em_ptr: *mut ELLIPSIS_MGR,
                                           mut cpm_ptr: *mut CF_PRED_MGR,
                                           mut cmm_ptr: *mut CF_MATCH_MGR,
                                           mut l: libc::c_int,
                                           mut tp: *mut TAG_DATA,
                                           mut cf_ptr: *mut CASE_FRAME,
                                           mut n: libc::c_int)
                                           -> libc::c_int {
    // let mut i: libc::c_int = 0;
    while (*tp).para_top_p != 0 {
        tp = (*tp).child[0 as libc::c_int as usize]
    }
    if CheckAppropriateCandidate(s, cs, cpm_ptr, tp, -(1 as libc::c_int),
                                 cf_ptr, n, 0 as libc::c_int,
                                 0 as libc::c_int) != 0 {
        EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp,
                                  cf_ptr, n, 0 as libc::c_int, s,
                                  (*tp).pred_b_ptr);
        if ScoreCheck(cf_ptr, n) != 0 { return 1 as libc::c_int; }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn SearchCompoundChild(mut s: *mut SENTENCE_DATA,
                                             mut cs: *mut SENTENCE_DATA,
                                             mut em_ptr: *mut ELLIPSIS_MGR,
                                             mut cpm_ptr: *mut CF_PRED_MGR,
                                             mut cmm_ptr: *mut CF_MATCH_MGR,
                                             mut l: libc::c_int,
                                             mut tp: *mut TAG_DATA,
                                             mut cf_ptr: *mut CASE_FRAME,
                                             mut n: libc::c_int,
                                             mut loc: libc::c_int,
                                             mut eflag: libc::c_int)
                                             -> libc::c_int {
    let mut i: libc::c_int = 0;
    while (*tp).para_top_p != 0 {
        tp = (*tp).child[0 as libc::c_int as usize]
    }
    i = 0 as libc::c_int;
    while !(*tp).child[i as usize].is_null() {
        if (!check_feature((*(*tp).child[i as usize]).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*(*tp).child[i as usize]).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe4\xbd\x93\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*(*tp).child[i as usize]).f,
                           b"\xe4\xbf\x82:\xe9\x9a\xa3\x00" as *const u8
                               as *const libc::c_char as
                               *mut libc::c_char).is_null()) &&
            CheckAppropriateCandidate(s, cs, cpm_ptr,
                                      (*tp).child[i as usize],
                                      -(2 as libc::c_int), cf_ptr, n, loc,
                                      0 as libc::c_int) != 0 {
            EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                      (*tp).child[i as usize], cf_ptr, n, loc,
                                      s,
                                      (*(*tp).child[i as usize]).pred_b_ptr);
            if eflag == 0 {
                *(*Bcheck.offset(cs.wrapping_offset_from(s) as libc::c_long as
                    isize)).offset((*(*tp).child[i as
                    usize]).num
                    as isize) =
                    1 as libc::c_int
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn _SearchCompoundChild(mut tp: *mut TAG_DATA,
                                              mut lc: *mut libc::c_int,
                                              mut lc_num: libc::c_int)
                                              -> libc::c_int {
    let mut i: libc::c_int = 0;
    while (*tp).para_top_p != 0 {
        tp = (*tp).child[0 as libc::c_int as usize]
    }
    i = 0 as libc::c_int;
    while !(*tp).child[i as usize].is_null() {
        if !check_feature((*(*tp).child[i as usize]).f,
                          b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            !check_feature((*(*tp).child[i as usize]).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe4\xbd\x93\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*(*tp).child[i as usize]).f,
                           b"\xe4\xbf\x82:\xe9\x9a\xa3\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() {
            if *lc.offset((*(*tp).child[i as usize]).num as isize) == 0 {
                *lc.offset((*(*tp).child[i as usize]).num as isize) = lc_num
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ListPredChildren(mut tp: *mut TAG_DATA)
                                          -> *mut *mut TAG_DATA {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 32 as libc::c_int;
    let mut ret: *mut *mut TAG_DATA = 0 as *mut *mut TAG_DATA;
    ret =
        malloc_data((::std::mem::size_of::<*mut TAG_DATA>() as
            libc::c_ulong).wrapping_mul(size as libc::c_ulong),
                    b"ListPredChildren\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut *mut TAG_DATA;
    i = 0 as libc::c_int;
    while !(*tp).child[i as usize].is_null() {
        if !check_feature((*(*tp).child[i as usize]).f,
                          b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            let fresh38 = count;
            count = count + 1;
            let ref mut fresh39 = *ret.offset(fresh38 as isize);
            *fresh39 = (*tp).child[i as usize]
        }
        i += 1
    }
    if (*tp).para_type as libc::c_int == 1 as libc::c_int {
        let mut tmp: *mut TAG_DATA = (*tp).parent;
        while !tmp.is_null() && (*tmp).para_top_p as libc::c_int != 0 {
            i = 0 as libc::c_int;
            while !(*tmp).child[i as usize].is_null() {
                if (*(*tmp).child[i as usize]).para_type as libc::c_int !=
                    1 as libc::c_int {
                    if count >= size - 1 as libc::c_int {
                        size <<= 1 as libc::c_int;
                        ret =
                            realloc_data(ret as *mut libc::c_void,
                                         (::std::mem::size_of::<*mut TAG_DATA>()
                                             as
                                             libc::c_ulong).wrapping_mul(size
                                             as
                                             libc::c_ulong),
                                         b"ListPredChildren\x00" as *const u8
                                             as *const libc::c_char as
                                             *mut libc::c_char) as
                                *mut *mut TAG_DATA
                    }
                    if !check_feature((*(*tmp).child[i as usize]).f,
                                      b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                        let fresh40 = count;
                        count = count + 1;
                        let ref mut fresh41 = *ret.offset(fresh40 as isize);
                        *fresh41 = (*tmp).child[i as usize]
                    }
                }
                i += 1
            }
            tmp = (*tmp).parent
        }
    }
    let ref mut fresh42 = *ret.offset(count as isize);
    *fresh42 = 0 as *mut TAG_DATA;
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchCaseComponent(mut cs: *mut SENTENCE_DATA,
                                              mut bp: *mut TAG_DATA,
                                              mut lc: *mut *mut libc::c_int,
                                              mut lc_num: libc::c_int,
                                              mut dist: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    /* cpm_ptr: 省略格要素をもつ用言
       bp:      格要素の探索対象となっている用言文節
    */
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut sent: libc::c_int = 0;
    let mut children: *mut *mut TAG_DATA = 0 as *mut *mut TAG_DATA;
    /* 用言の格要素をチェック */
    if !(*bp).cpm_ptr.is_null() {
        if (*(*bp).cpm_ptr).cmm[0 as libc::c_int as usize].score !=
            -(2 as libc::c_int) as libc::c_double {
            i = 0 as libc::c_int;
            while i <
                (*(*(*bp).cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).element_num {
                num =
                    (*(*bp).cpm_ptr).cmm[0 as libc::c_int as
                        usize].result_lists_p[0 as
                        libc::c_int
                        as
                        usize].flag[i
                        as
                        usize];
                if num != -(1 as libc::c_int) {
                    if (*(*bp).cpm_ptr).elem_b_ptr[num as usize].is_null() {
                        /* 不特定 */
                        if ExtraLC == 0 { ExtraLC = lc_num }
                    } else {
                        if (*(*bp).cpm_ptr).elem_b_num[num as usize] >
                            -(2 as libc::c_int) {
                            sent = dist
                        } else {
                            /* 省略 */
                            sent =
                                (dist as libc::c_long +
                                    cs.wrapping_offset_from((*(*bp).cpm_ptr).elem_s_ptr[num
                                        as
                                        usize])
                                        as libc::c_long) as libc::c_int
                        }
                        if *(*lc.offset(sent as
                            isize)).offset((*(*(*bp).cpm_ptr).elem_b_ptr[num
                            as
                            usize]).num
                            as isize) == 0
                        {
                            *(*lc.offset(sent as
                                isize)).offset((*(*(*bp).cpm_ptr).elem_b_ptr[num
                                as
                                usize]).num
                                as isize) =
                                lc_num
                        }
                        /* ノ格の子供をチェック */
                        _SearchCompoundChild((*(*bp).cpm_ptr).elem_b_ptr[num
                            as
                            usize],
                                             *lc.offset(sent as isize),
                                             lc_num);
                    }
                }
                i += 1
            }
        }
        /* 格要素になっていない子供もチェック */
        children = ListPredChildren((*(*bp).cpm_ptr).pred_b_ptr);
        i = 0 as libc::c_int;
        while !(*children.offset(i as isize)).is_null() {
            if *(*lc.offset(dist as
                isize)).offset((**children.offset(i as
                isize)).num
                as isize) == 0 {
                *(*lc.offset(dist as
                    isize)).offset((**children.offset(i as
                    isize)).num
                    as isize) = lc_num
            }
            /* ノ格の子供をチェック */
            _SearchCompoundChild(*children.offset(i as isize),
                                 *lc.offset(dist as isize), lc_num);
            i += 1
        }
        free(children as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchRelatedComponent(mut s: *mut SENTENCE_DATA,
                                                mut em_ptr: *mut ELLIPSIS_MGR,
                                                mut cpm_ptr: *mut CF_PRED_MGR,
                                                mut cmm_ptr:
                                                *mut CF_MATCH_MGR,
                                                mut l: libc::c_int,
                                                mut bp: *mut TAG_DATA,
                                                mut cf_ptr: *mut CASE_FRAME,
                                                mut n: libc::c_int,
                                                mut loc: libc::c_int)
                                                -> libc::c_int
/*==================================================================*/
{
    /* cpm_ptr: 省略格要素をもつ用言
       bp:      要素の探索対象となっている体言文節
    */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* <PARA> */
    if !((*bp).para_top_p != 0) {
        /* bpに係る要素をチェック */
        i = 0 as libc::c_int;
        while !(*bp).child[i as usize].is_null() {
            if !((*bp).child[i as usize] == (*cpm_ptr).pred_b_ptr) {
                if (*(*bp).child[i as usize]).para_top_p != 0 {
                    j = 0 as libc::c_int;
                    while !(*(*bp).child[i as
                        usize]).child[j as
                        usize].is_null()
                    {
                        if (*(*(*bp).child[i as
                            usize]).child[j as
                            usize]).para_type
                            as libc::c_int == 1 as libc::c_int &&
                            *(*Bcheck.offset(0 as libc::c_int as
                                isize)).offset((*(*(*bp).child[i
                                as
                                usize]).child[j
                                as
                                usize]).num
                                as
                                isize)
                                == 0 &&
                            CheckAppropriateCandidate(s, s, cpm_ptr,
                                                      (*(*bp).child[i as
                                                          usize]).child[j
                                                          as
                                                          usize],
                                                      -(1 as libc::c_int),
                                                      cf_ptr, n, loc,
                                                      0 as libc::c_int) !=
                                0 {
                            EllipsisDetectSubcontract(s, s, em_ptr, cpm_ptr,
                                                      cmm_ptr, l,
                                                      (*(*bp).child[i as
                                                          usize]).child[j
                                                          as
                                                          usize],
                                                      cf_ptr, n, loc, s,
                                                      (*(*(*bp).child[i as
                                                          usize]).child[j
                                                          as
                                                          usize]).pred_b_ptr);
                            *(*Bcheck.offset(0 as libc::c_int as
                                isize)).offset((*(*(*bp).child[i
                                as
                                usize]).child[j
                                as
                                usize]).num
                                as isize)
                                = 1 as libc::c_int
                            /* return 1; */
                        }
                        j += 1
                    }
                } else if *(*Bcheck.offset(0 as libc::c_int as
                    isize)).offset((*(*bp).child[i
                    as
                    usize]).num
                    as isize) ==
                    0 &&
                    CheckAppropriateCandidate(s, s, cpm_ptr,
                                              (*bp).child[i as
                                                  usize],
                                              -(1 as libc::c_int),
                                              cf_ptr, n, loc,
                                              0 as libc::c_int) != 0
                {
                    EllipsisDetectSubcontract(s, s, em_ptr, cpm_ptr, cmm_ptr,
                                              l, (*bp).child[i as usize],
                                              cf_ptr, n, loc, s,
                                              (*(*bp).child[i as
                                                  usize]).pred_b_ptr);
                    *(*Bcheck.offset(0 as libc::c_int as
                        isize)).offset((*(*bp).child[i as
                        usize]).num
                        as isize) =
                        1 as libc::c_int
                }
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_mc(mut tp: *mut TAG_DATA) -> libc::c_int
/*==================================================================*/
{
    if !check_feature((*tp).f,
                      b"\xe4\xb8\xbb\xe7\xaf\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        /* check_feature(tp->f, "文末")) { */
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchMC(mut s: *mut SENTENCE_DATA,
                                  mut cs: *mut SENTENCE_DATA,
                                  mut em_ptr: *mut ELLIPSIS_MGR,
                                  mut cpm_ptr: *mut CF_PRED_MGR,
                                  mut cmm_ptr: *mut CF_MATCH_MGR,
                                  mut l: libc::c_int,
                                  mut cf_ptr: *mut CASE_FRAME,
                                  mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut dist: libc::c_int = 0;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    dist = cs.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    i = (*s).Tag_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tp = (*s).tag_data.offset(i as isize);
        while (*tp).para_top_p != 0 {
            tp = (*tp).child[0 as libc::c_int as usize]
        }
        if check_mc(tp) != 0 {
            flag = 1 as libc::c_int;
            break;
        } else { i -= 1 }
    }
    if flag == 0 as libc::c_int {
        tp =
            (*s).tag_data.offset((*s).Tag_num as
                isize).offset(-(1 as libc::c_int as
                isize));
        while (*tp).para_top_p != 0 {
            tp = (*tp).child[0 as libc::c_int as usize]
        }
    }
    SearchCaseComponent(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp, cf_ptr, n,
                        if dist == 2 as libc::c_int {
                            0x22001 as libc::c_int
                        } else if dist == 1 as libc::c_int {
                            0x12001 as libc::c_int
                        } else { 0x2001 as libc::c_int });
    /* 文末にある体言(先行詞候補)は OK */
    if CheckAppropriateCandidate(s, cs, cpm_ptr, tp, -(2 as libc::c_int),
                                 cf_ptr, n, 0x2001 as libc::c_int,
                                 0 as libc::c_int) != 0 {
        EllipsisDetectSubcontract(s, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp,
                                  cf_ptr, n,
                                  if dist == 2 as libc::c_int {
                                      0x22001 as libc::c_int
                                  } else if dist == 1 as libc::c_int {
                                      0x12001 as libc::c_int
                                  } else { 0x2001 as libc::c_int }, s,
                                  (*tp).pred_b_ptr);
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mark_all_children(mut cs: *mut SENTENCE_DATA,
                                           mut tp: *mut TAG_DATA,
                                           mut lc: *mut *mut libc::c_int,
                                           mut lc_num: libc::c_int,
                                           mut sent: libc::c_int)
                                           -> libc::c_int
/*==================================================================*/
{
    // let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    if *(*lc.offset(sent as isize)).offset((*tp).num as isize) == 0 {
        *(*lc.offset(sent as isize)).offset((*tp).num as isize) = lc_num
    }
    _SearchCaseComponent(cs, tp, lc, lc_num, sent);
    panic!("Reached end of non-void function without returning");
    /*
    for (i = 0; tp->child[i]; i++) {
	if (tp->child[i]->para_top_p) {
	    for (j = 0; tp->child[i]->child[j]; j++) {
		if (tp->child[i]->child[j]->para_type == PARA_NORMAL) {
		    if (check_feature(tp->child[i]->child[j]->f, "格要素")) {
			if (!lc[tp->child[i]->child[j]->num]) {
			    lc[tp->child[i]->child[j]->num] = lc_num;
			}
		    }
		}
		* ★ <PARA> のとき ★ *
	    }
	}
	else if (check_feature(tp->child[i]->f, "格要素")) {
	    if (!lc[tp->child[i]->num]) {
		lc[tp->child[i]->num] = lc_num;
	    }
	}
    }
    */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchMC(mut s: *mut SENTENCE_DATA,
                                   mut ctp: *mut TAG_DATA,
                                   mut lc: *mut *mut libc::c_int,
                                   mut dist: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = (*s).Tag_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tp = (*s).tag_data.offset(i as isize);
        while (*tp).para_top_p != 0 {
            tp = (*tp).child[0 as libc::c_int as usize]
        }
        if check_mc(tp) != 0 {
            flag = 1 as libc::c_int;
            break;
        } else { i -= 1 }
    }
    if flag == 0 as libc::c_int {
        tp =
            (*s).tag_data.offset((*s).Tag_num as
                isize).offset(-(1 as libc::c_int as
                isize));
        while (*tp).para_top_p != 0 {
            tp = (*tp).child[0 as libc::c_int as usize]
        }
    }
    /* 対象ではない */
    if ctp.is_null() || (*tp).num != (*ctp).num {
        mark_all_children(s, tp, lc,
                          if dist == 2 as libc::c_int {
                              0x22001 as libc::c_int
                          } else if dist == 1 as libc::c_int {
                              0x12001 as libc::c_int
                          } else { 0x2001 as libc::c_int }, dist);
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchSC(mut s: *mut SENTENCE_DATA,
                                  mut cs: *mut SENTENCE_DATA,
                                  mut em_ptr: *mut ELLIPSIS_MGR,
                                  mut cpm_ptr: *mut CF_PRED_MGR,
                                  mut cmm_ptr: *mut CF_MATCH_MGR,
                                  mut l: libc::c_int,
                                  mut cf_ptr: *mut CASE_FRAME,
                                  mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    dist = cs.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    i = (*s).Tag_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tp = (*s).tag_data.offset(i as isize);
        if check_mc(tp) != 0 {
            if (*tp).para_top_p != 0 {
                /* 主節をチェックしないように */
                start = 1 as libc::c_int
            } else { start = 0 as libc::c_int }
            j = start;
            while !(*tp).child[j as usize].is_null() {
                tp2 = (*tp).child[j as usize];
                while (*tp2).para_top_p != 0 {
                    tp2 = (*tp2).child[0 as libc::c_int as usize]
                }
                /* レベルがBより強い従属節 */
                if !check_feature((*tp2).f,
                                  b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                    subordinate_level_check(b"B\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char,
                                            (*(tp2 as *mut BNST_DATA)).f)
                        != 0 {
                    SearchCaseComponent(s, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                        tp2, cf_ptr, n,
                                        if dist == 2 as libc::c_int {
                                            0x24000 as libc::c_int
                                        } else if dist == 1 as libc::c_int {
                                            0x14000 as libc::c_int
                                        } else { 0x4000 as libc::c_int });
                }
                j += 1
            }
            break;
        } else { i -= 1 }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchSC(mut s: *mut SENTENCE_DATA,
                                   mut ctp: *mut TAG_DATA,
                                   mut lc: *mut *mut libc::c_int,
                                   mut dist: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = (*s).Tag_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tp = (*s).tag_data.offset(i as isize);
        if check_mc(tp) != 0 {
            if (*tp).para_top_p != 0 {
                /* 主節をチェックしないように */
                start = 1 as libc::c_int
            } else { start = 0 as libc::c_int }
            j = start;
            while !(*tp).child[j as usize].is_null() {
                tp2 = (*tp).child[j as usize];
                while (*tp2).para_top_p != 0 {
                    tp2 = (*tp2).child[0 as libc::c_int as usize]
                }
                /* レベルがBより強い従属節 */
                if (ctp.is_null() || (*tp2).num != (*ctp).num) &&
                    !check_feature((*tp2).f,
                                   b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null() &&
                    subordinate_level_check(b"B\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char,
                                            (*(tp2 as *mut BNST_DATA)).f)
                        != 0 {
                    mark_all_children(s, tp2, lc,
                                      if dist == 2 as libc::c_int {
                                          0x24000 as libc::c_int
                                      } else if dist == 1 as libc::c_int {
                                          0x14000 as libc::c_int
                                      } else { 0x4000 as libc::c_int }, dist);
                }
                j += 1
            }
            break;
        } else { i -= 1 }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckMatchedLC(mut s: *mut SENTENCE_DATA,
                                        mut cs: *mut SENTENCE_DATA,
                                        mut em_ptr: *mut ELLIPSIS_MGR,
                                        mut cpm_ptr: *mut CF_PRED_MGR,
                                        mut cmm_ptr: *mut CF_MATCH_MGR,
                                        mut l: libc::c_int,
                                        mut tp: *mut TAG_DATA,
                                        mut cf_ptr: *mut CASE_FRAME,
                                        mut n: libc::c_int,
                                        mut loc: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut sent: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut ts: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    /* 不特定 */
    if OptDiscFlag & 16 as libc::c_int != 0 && ExtraLC == loc &&
        ExtraCheck == 0 {
        EllipsisDetectSubcontractExtraTags(cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                           1 as libc::c_int, cf_ptr, n,
                                           loc); /* "1"は不特定-人 */
        ExtraCheck = 1 as libc::c_int
    }
    sent = 0 as libc::c_int;
    while (sent as libc::c_long) <
        (*cs).Sen_num as libc::c_long -
            cs.wrapping_offset_from(s) as libc::c_long {
        ts = s.offset(-(sent as isize));
        dist =
            (cs.wrapping_offset_from(s) as libc::c_long +
                sent as libc::c_long) as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*ts).Tag_num {
            if *(*LC.offset(dist as isize)).offset(i as isize) == loc {
                if CheckAppropriateCandidate(ts, cs, cpm_ptr,
                                             (*ts).tag_data.offset(i as
                                                 isize),
                                             -(2 as libc::c_int), cf_ptr, n,
                                             loc, 0 as libc::c_int) != 0 {
                    EllipsisDetectSubcontract(ts, cs, em_ptr, cpm_ptr,
                                              cmm_ptr, l,
                                              (*ts).tag_data.offset(i as
                                                  isize),
                                              cf_ptr, n, loc, ts,
                                              (*(*ts).tag_data.offset(i as
                                                  isize)).pred_b_ptr);
                    *(*Bcheck.offset(dist as isize)).offset(i as isize) =
                        1 as libc::c_int
                }
            }
            i += 1
        }
        sent += 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchParentV(mut cs: *mut SENTENCE_DATA,
                                       mut em_ptr: *mut ELLIPSIS_MGR,
                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut cmm_ptr: *mut CF_MATCH_MGR,
                                       mut l: libc::c_int,
                                       mut tp: *mut TAG_DATA,
                                       mut cf_ptr: *mut CASE_FRAME,
                                       mut n: libc::c_int,
                                       mut mccheck: libc::c_int)
                                       -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() &&
        !check_feature((*(*tp).parent).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        let mut i: libc::c_int = 0;
        let mut mcflag: libc::c_int = 0;
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        mcflag = check_mc((*tp).parent);
        if !(mccheck == 0 as libc::c_int ||
            mccheck > 0 as libc::c_int && mcflag != 0 ||
            mccheck < 0 as libc::c_int && mcflag == 0) {
            return 0 as libc::c_int;
        }
        /* 親が<PARA>なら<P>の子供を全部チェック */
        if (*(*tp).parent).para_top_p != 0 {
            i = 0 as libc::c_int;
            while !(*(*tp).parent).child[i as usize].is_null() {
                if (*(*(*tp).parent).child[i as usize]).para_type as
                    libc::c_int == 1 as libc::c_int &&
                    (*(*(*tp).parent).child[i as usize]).num > (*tp).num {
                    /* <PARA>なら child[0]をみていく必要がある */
                    tp2 = (*(*tp).parent).child[i as usize];
                    while (*tp2).para_top_p != 0 {
                        tp2 = (*tp2).child[0 as libc::c_int as usize]
                    }
                    SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                        tp2, cf_ptr, n,
                                        if mcflag != 0 {
                                            0x3 as libc::c_int
                                        } else { 0x2 as libc::c_int });
                }
                i += 1
            }
        } else {
            SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                (*tp).parent, cf_ptr, n,
                                if mcflag != 0 {
                                    0x3 as libc::c_int
                                } else { 0x2 as libc::c_int });
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchParentV(mut s: *mut SENTENCE_DATA,
                                        mut tp: *mut TAG_DATA,
                                        mut lc: *mut *mut libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() &&
        !check_feature((*(*tp).parent).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        let mut i: libc::c_int = 0;
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        /* 親が<PARA>なら<P>の子供を全部チェック */
        if (*(*tp).parent).para_top_p != 0 {
            i = 0 as libc::c_int;
            while !(*(*tp).parent).child[i as usize].is_null() {
                if (*(*(*tp).parent).child[i as usize]).para_type as
                    libc::c_int == 1 as libc::c_int &&
                    (*(*(*tp).parent).child[i as usize]).num > (*tp).num {
                    /* <PARA>なら child[0]をみていく必要がある */
                    tp2 = (*(*tp).parent).child[i as usize];
                    while (*tp2).para_top_p != 0 {
                        tp2 = (*tp2).child[0 as libc::c_int as usize]
                    }
                    mark_all_children(s, tp2, lc,
                                      if check_mc(tp2) != 0 {
                                          0x3 as libc::c_int
                                      } else { 0x2 as libc::c_int },
                                      0 as libc::c_int);
                }
                i += 1
            }
        } else {
            mark_all_children(s, (*tp).parent, lc,
                              if check_mc((*tp).parent) != 0 {
                                  0x3 as libc::c_int
                              } else { 0x2 as libc::c_int },
                              0 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn GoUpParaChild(mut cs: *mut SENTENCE_DATA,
                                       mut em_ptr: *mut ELLIPSIS_MGR,
                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut cmm_ptr: *mut CF_MATCH_MGR,
                                       mut l: libc::c_int,
                                       mut tp: *mut TAG_DATA,
                                       mut orig_tp: *mut TAG_DATA,
                                       mut cf_ptr: *mut CASE_FRAME,
                                       mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* tp : <PARA> */
    if !tp.is_null() && (*tp).para_top_p as libc::c_int != 0 {
        /* ★この子供の順番は? */
        i = 0 as libc::c_int;
        while !(*tp).child[i as usize].is_null() {
            if (*(*tp).child[i as usize]).num < (*orig_tp).num &&
                (*(*tp).child[i as usize]).para_type as libc::c_int ==
                    1 as libc::c_int {
                /* <PARA>でなくなるまでさかのぼる必要がある */
                tp2 = (*tp).child[i as usize];
                while (*tp2).para_top_p != 0 {
                    tp2 = (*tp2).child[0 as libc::c_int as usize]
                }
                SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l, tp2,
                                    cf_ptr, n, 0x200 as libc::c_int);
                /* 並列の子は全部<PARA>に係るので再帰に呼ぶのは間違い
		   GoUpParaChild(cs, em_ptr, cpm_ptr, cmm_ptr,
		   tp->child[i], tp->child[i]->child[0], cf_ptr, n); */
            }
            i += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _GoUpParaChild(mut s: *mut SENTENCE_DATA,
                                        mut tp: *mut TAG_DATA,
                                        mut orig_tp: *mut TAG_DATA,
                                        mut lc: *mut *mut libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* tp : <PARA> */
    if !tp.is_null() && (*tp).para_top_p as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while !(*tp).child[i as usize].is_null() {
            if (*(*tp).child[i as usize]).num < (*orig_tp).num &&
                (*(*tp).child[i as usize]).para_type as libc::c_int ==
                    1 as libc::c_int {
                /* <PARA>でなくなるまでさかのぼる必要がある */
                tp2 = (*tp).child[i as usize];
                while (*tp2).para_top_p != 0 {
                    tp2 = (*tp2).child[0 as libc::c_int as usize]
                }
                mark_all_children(s, tp2, lc, 0x200 as libc::c_int,
                                  0 as libc::c_int);
            }
            i += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchChildPV(mut cs: *mut SENTENCE_DATA,
                                       mut em_ptr: *mut ELLIPSIS_MGR,
                                       mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut cmm_ptr: *mut CF_MATCH_MGR,
                                       mut l: libc::c_int,
                                       mut tp: *mut TAG_DATA,
                                       mut cf_ptr: *mut CASE_FRAME,
                                       mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    if (*tp).para_type as libc::c_int == 1 as libc::c_int &&
        !(*tp).parent.is_null() &&
        (*(*tp).parent).para_top_p as libc::c_int != 0 {
        GoUpParaChild(cs, em_ptr, cpm_ptr, cmm_ptr, l, (*tp).parent, tp,
                      cf_ptr, n);
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchChildPV(mut s: *mut SENTENCE_DATA,
                                        mut tp: *mut TAG_DATA,
                                        mut lc: *mut *mut libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    if (*tp).para_type as libc::c_int == 1 as libc::c_int &&
        !(*tp).parent.is_null() &&
        (*(*tp).parent).para_top_p as libc::c_int != 0 {
        _GoUpParaChild(s, (*tp).parent, tp, lc);
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchChildV(mut cs: *mut SENTENCE_DATA,
                                      mut em_ptr: *mut ELLIPSIS_MGR,
                                      mut cpm_ptr: *mut CF_PRED_MGR,
                                      mut cmm_ptr: *mut CF_MATCH_MGR,
                                      mut l: libc::c_int,
                                      mut tp: *mut TAG_DATA,
                                      mut cf_ptr: *mut CASE_FRAME,
                                      mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* 自分は<PARA>でないので並列ではない */
    if (*tp).para_type as libc::c_int == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*tp).child[i as usize].is_null() {
            if !check_feature((*(*tp).child[i as usize]).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                    (*tp).child[i as usize], cf_ptr, n,
                                    0x400 as libc::c_int);
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchChildV(mut s: *mut SENTENCE_DATA,
                                       mut tp: *mut TAG_DATA,
                                       mut lc: *mut *mut libc::c_int)
                                       -> libc::c_int
/*==================================================================*/
{
    /* 自分は<PARA>でないので並列ではない */
    if (*tp).para_type as libc::c_int == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*tp).child[i as usize].is_null() {
            if !check_feature((*(*tp).child[i as usize]).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                mark_all_children(s, (*tp).child[i as usize], lc,
                                  0x400 as libc::c_int, 0 as libc::c_int);
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchParentNParentV(mut cs: *mut SENTENCE_DATA,
                                              mut em_ptr: *mut ELLIPSIS_MGR,
                                              mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut cmm_ptr: *mut CF_MATCH_MGR,
                                              mut l: libc::c_int,
                                              mut tp: *mut TAG_DATA,
                                              mut cf_ptr: *mut CASE_FRAME,
                                              mut n: libc::c_int,
                                              mut mccheck: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() && (*tp).para_type == 0 &&
        !(*(*tp).parent).parent.is_null() &&
        check_feature((*(*tp).parent).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() {
        let mut mcflag: libc::c_int = 0;
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        mcflag = check_mc((*(*tp).parent).parent);
        if !(mccheck == 0 as libc::c_int ||
            mccheck > 0 as libc::c_int && mcflag != 0 ||
            mccheck < 0 as libc::c_int && mcflag == 0) {
            return 0 as libc::c_int;
        }
        if !check_feature((*(*(*tp).parent).parent).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (*(*(*tp).parent).parent).para_top_p != 0 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !(*(*(*tp).parent).parent).child[i as usize].is_null() {
                    if (*(*(*(*tp).parent).parent).child[i as
                        usize]).para_type
                        as libc::c_int == 1 as libc::c_int &&
                        (*(*(*(*tp).parent).parent).child[i as usize]).num
                            > (*(*tp).parent).num {
                        /* <PARA>なら child[0]をみていく必要がある */
                        tp2 = (*(*(*tp).parent).parent).child[i as usize];
                        while (*tp2).para_top_p != 0 {
                            tp2 = (*tp2).child[0 as libc::c_int as usize]
                        }
                        SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr,
                                            l, tp2, cf_ptr, n,
                                            if mcflag != 0 {
                                                0x5 as libc::c_int
                                            } else { 0x4 as libc::c_int });
                    }
                    i += 1
                }
            } else {
                SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                    (*(*tp).parent).parent, cf_ptr, n,
                                    if mcflag != 0 {
                                        0x5 as libc::c_int
                                    } else { 0x4 as libc::c_int });
            }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchParentNParentV(mut s: *mut SENTENCE_DATA,
                                               mut tp: *mut TAG_DATA,
                                               mut lc: *mut *mut libc::c_int)
                                               -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() && (*tp).para_type == 0 &&
        !(*(*tp).parent).parent.is_null() &&
        check_feature((*(*tp).parent).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() {
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        if !check_feature((*(*(*tp).parent).parent).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (*(*(*tp).parent).parent).para_top_p != 0 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !(*(*(*tp).parent).parent).child[i as usize].is_null() {
                    if (*(*(*(*tp).parent).parent).child[i as
                        usize]).para_type
                        as libc::c_int == 1 as libc::c_int &&
                        (*(*(*(*tp).parent).parent).child[i as usize]).num
                            > (*(*tp).parent).num {
                        /* <PARA>なら child[0]をみていく必要がある */
                        tp2 = (*(*(*tp).parent).parent).child[i as usize];
                        while (*tp2).para_top_p != 0 {
                            tp2 = (*tp2).child[0 as libc::c_int as usize]
                        }
                        mark_all_children(s, tp2, lc,
                                          if check_mc(tp2) != 0 {
                                              0x5 as libc::c_int
                                          } else { 0x4 as libc::c_int },
                                          0 as libc::c_int);
                    }
                    i += 1
                }
            } else {
                mark_all_children(s, (*(*tp).parent).parent, lc,
                                  if check_mc((*(*tp).parent).parent) != 0 {
                                      0x5 as libc::c_int
                                  } else { 0x4 as libc::c_int },
                                  0 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn SearchParentVParentV(mut cs: *mut SENTENCE_DATA,
                                              mut em_ptr: *mut ELLIPSIS_MGR,
                                              mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut cmm_ptr: *mut CF_MATCH_MGR,
                                              mut l: libc::c_int,
                                              mut tp: *mut TAG_DATA,
                                              mut cf_ptr: *mut CASE_FRAME,
                                              mut n: libc::c_int,
                                              mut mccheck: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() && (*tp).para_type == 0 &&
        !(*(*tp).parent).parent.is_null() &&
        !check_feature((*(*tp).parent).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        let mut mcflag: libc::c_int = 0;
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        mcflag = check_mc((*(*tp).parent).parent);
        if !(mccheck == 0 as libc::c_int ||
            mccheck > 0 as libc::c_int && mcflag != 0 ||
            mccheck < 0 as libc::c_int && mcflag == 0) {
            return 0 as libc::c_int;
        }
        if !check_feature((*(*(*tp).parent).parent).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (*(*(*tp).parent).parent).para_top_p != 0 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !(*(*(*tp).parent).parent).child[i as usize].is_null() {
                    if (*(*(*(*tp).parent).parent).child[i as
                        usize]).para_type
                        as libc::c_int == 1 as libc::c_int &&
                        (*(*(*(*tp).parent).parent).child[i as usize]).num
                            > (*(*tp).parent).num {
                        /* <PARA>なら child[0]をみていく必要がある */
                        tp2 = (*(*(*tp).parent).parent).child[i as usize];
                        while (*tp2).para_top_p != 0 {
                            tp2 = (*tp2).child[0 as libc::c_int as usize]
                        }
                        SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr,
                                            l, tp2, cf_ptr, n,
                                            if mcflag != 0 {
                                                0x11 as libc::c_int
                                            } else { 0x10 as libc::c_int });
                    }
                    i += 1
                }
            } else {
                SearchCaseComponent(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                    (*(*tp).parent).parent, cf_ptr, n,
                                    if mcflag != 0 {
                                        0x11 as libc::c_int
                                    } else { 0x10 as libc::c_int });
            }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchParentVParentV(mut s: *mut SENTENCE_DATA,
                                               mut tp: *mut TAG_DATA,
                                               mut lc: *mut *mut libc::c_int)
                                               -> libc::c_int
/*==================================================================*/
{
    if !(*tp).parent.is_null() && (*tp).para_type == 0 &&
        !(*(*tp).parent).parent.is_null() &&
        !check_feature((*(*tp).parent).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
        if !check_feature((*(*(*tp).parent).parent).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (*(*(*tp).parent).parent).para_top_p != 0 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !(*(*(*tp).parent).parent).child[i as usize].is_null() {
                    if (*(*(*(*tp).parent).parent).child[i as
                        usize]).para_type
                        as libc::c_int == 1 as libc::c_int &&
                        (*(*(*(*tp).parent).parent).child[i as usize]).num
                            > (*(*tp).parent).num {
                        /* <PARA>なら child[0]をみていく必要がある */
                        tp2 = (*(*(*tp).parent).parent).child[i as usize];
                        while (*tp2).para_top_p != 0 {
                            tp2 = (*tp2).child[0 as libc::c_int as usize]
                        }
                        mark_all_children(s, tp2, lc,
                                          if check_mc(tp2) != 0 {
                                              0x11 as libc::c_int
                                          } else { 0x10 as libc::c_int },
                                          0 as libc::c_int);
                    }
                    i += 1
                }
            } else {
                mark_all_children(s, (*(*tp).parent).parent, lc,
                                  if check_mc((*(*tp).parent).parent) != 0 {
                                      0x11 as libc::c_int
                                  } else { 0x10 as libc::c_int },
                                  0 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _SearchPV(mut s: *mut SENTENCE_DATA,
                                   mut tp: *mut TAG_DATA,
                                   mut lc: *mut *mut libc::c_int)
                                   -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if (*tp).para_type as libc::c_int == 1 as libc::c_int {
        /* <PARA>に係る要素 */
        /* mark_all_children(tp->parent, lc, check_mc(tp->parent) ? LOC_PV_MC : LOC_PV); */
        i = 0 as libc::c_int;
        while !(*(*tp).parent).child[i as usize].is_null() {
            if (*(*(*tp).parent).child[i as usize]).num > (*tp).num &&
                (*(*(*tp).parent).child[i as usize]).para_type as
                    libc::c_int == 1 as libc::c_int {
                mark_all_children(s, (*(*tp).parent).child[i as usize], lc,
                                  if check_mc((*(*tp).parent).child[i as
                                      usize])
                                      != 0 {
                                      0x9 as libc::c_int
                                  } else { 0x8 as libc::c_int },
                                  0 as libc::c_int);
            }
            i += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectForVerb(mut sp: *mut SENTENCE_DATA,
                                               mut em_ptr: *mut ELLIPSIS_MGR,
                                               mut cpm_ptr: *mut CF_PRED_MGR,
                                               mut cmm_ptr: *mut CF_MATCH_MGR,
                                               mut l: libc::c_int,
                                               mut cf_ptr: *mut CASE_FRAME,
                                               mut n: libc::c_int)
                                               -> libc::c_int
/*==================================================================*/
{
    let mut current_block: u64;
    /* 用言とその省略格が与えられる */
    /* cf_ptr = cpm_ptr->cmm[0].cf_ptr である */
    /* 用言 cpm_ptr の cf_ptr->pp[n][0] 格が省略されている
       cf_ptr->ex[n] に似ている文節を探す */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut mc: libc::c_int = 0 as libc::c_int;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut etc_buffer: [libc::c_char; 5120] = [0; 5120];
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut s: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut cs: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    // let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut ptp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    maxscore = 0 as libc::c_int as libc::c_float;
    maxtag = 0 as *mut libc::c_char;
    maxs = 0 as *mut SENTENCE_DATA;
    maxpos = -(2 as libc::c_int);
    AlreadyDecidedFlag = 0 as libc::c_int;
    cs =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    i = 0 as libc::c_int;
    while i < (*sp).Sen_num {
        memset(*Bcheck.offset(i as isize) as *mut libc::c_void,
               0 as libc::c_int,
               (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(200 as libc::c_int as
                   libc::c_ulong));
        i += 1
    }
    ExtraCheck = 0 as libc::c_int;
    /* best解を探す場合 */
    if OptDiscFlag & 2 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i <= PrevSentenceLimit {
            if (cs.wrapping_offset_from(sentence_data.as_mut_ptr()) as
                libc::c_long) < i as libc::c_long {
                break;
            }
            EllipsisDetectRecursive(cs.offset(-(i as isize)), cs, em_ptr,
                                    cpm_ptr, cmm_ptr, l,
                                    (*cs.offset(-(i as
                                        isize))).tag_data.offset((*cs.offset(-(i
                                        as
                                        isize))).Tag_num
                                        as
                                        isize).offset(-(1
                                        as
                                        libc::c_int
                                        as
                                        isize)),
                                    cf_ptr, n, 0 as libc::c_int);
            i += 1
        }
        /* 閾値を越えるものが見つからなかった */
        if ScoreCheck(cf_ptr, n) == 0 {
            /* 閾値を越えるものがなく、格フレームに<主体>があるとき */
            if cf_match_element((*cf_ptr).sm[n as usize],
                                b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int) != 0
            {
                maxtag = ExtraTags[1 as libc::c_int as usize]
                /* 不特定-人 */
            } else { return 0 as libc::c_int; }
        }
    } else {
        /* flat */
        if OptDiscFlag & 8 as libc::c_int != 0 {
            let mut max_n: libc::c_int = 0;
            let mut post_n: libc::c_int =
                (*cs).Tag_num - (*(*cpm_ptr).pred_b_ptr).num -
                    1 as libc::c_int;
            let mut pre_n: libc::c_int = (*(*cpm_ptr).pred_b_ptr).num;
            max_n = if post_n > pre_n { post_n } else { pre_n };
            /* 対象文 */
            i = 1 as libc::c_int;
            loop {
                if !(i < max_n) {
                    current_block = 14072441030219150333;
                    break;
                }
                if (*(*cpm_ptr).pred_b_ptr).num - i >= 0 as libc::c_int {
                    EllipsisDetectOne(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                      (*cs).tag_data.offset((*(*cpm_ptr).pred_b_ptr).num
                                          as
                                          isize).offset(-(i
                                          as
                                          isize)),
                                      cf_ptr, n);
                }
                if (*(*cpm_ptr).pred_b_ptr).num + i < (*cs).Tag_num {
                    EllipsisDetectOne(cs, cs, em_ptr, cpm_ptr, cmm_ptr, l,
                                      (*cs).tag_data.offset((*(*cpm_ptr).pred_b_ptr).num
                                          as
                                          isize).offset(i
                                          as
                                          isize),
                                      cf_ptr, n);
                }
                if ScoreCheck(cf_ptr, n) != 0 {
                    current_block = 4548502464050555819;
                    break;
                }
                i += 1
            }
            match current_block {
                4548502464050555819 => {}
                _ =>
                /* 前文以前 */
                    {
                        j = 1 as libc::c_int;
                        's_197:
                        loop {
                            if !(j <= PrevSentenceLimit) {
                                current_block = 3689906465960840878;
                                break;
                            }
                            if (cs.wrapping_offset_from(sentence_data.as_mut_ptr())
                                as libc::c_long) < j as libc::c_long {
                                current_block = 3689906465960840878;
                                break;
                            }
                            i =
                                (*cs.offset(-(j as isize))).Tag_num -
                                    1 as libc::c_int;
                            while i >= 0 as libc::c_int {
                                EllipsisDetectOne(cs.offset(-(j as isize)),
                                                  cs, em_ptr, cpm_ptr,
                                                  cmm_ptr, l,
                                                  (*cs.offset(-(j as
                                                      isize))).tag_data.offset(i
                                                      as
                                                      isize),
                                                  cf_ptr, n);
                                if ScoreCheck(cf_ptr, n) != 0 {
                                    current_block = 4548502464050555819;
                                    break 's_197;
                                }
                                i -= 1
                            }
                            j += 1
                        }
                    }
            }
        } else { current_block = 3689906465960840878; }
        match current_block {
            4548502464050555819 => {}
            _ => {
                /* 位置カテゴリの順番で探す */
                /* (用言の)並列を吸収 */
                tp = (*cpm_ptr).pred_b_ptr;
                while (*tp).para_type as libc::c_int == 1 as libc::c_int &&
                    !(*tp).parent.is_null() &&
                    (*(*tp).parent).para_top_p as libc::c_int != 0 {
                    tp = (*tp).parent
                }
                /* 並列なら tp は <PARA> になる */
                ptp = tp;
                if !(MatchPP((*cf_ptr).pp[n as
                    usize][0 as libc::c_int as
                    usize],
                             b"\xe3\x82\xac\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) !=
                    0 ||
                    MatchPP((*cf_ptr).pp[n as
                        usize][0 as libc::c_int as
                        usize],
                            b"\xe3\x83\xb2\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0 ||
                    MatchPP((*cf_ptr).pp[n as
                        usize][0 as libc::c_int as
                        usize],
                            b"\xe3\x83\x8b\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char)
                        != 0) {
                    fprintf(stderr,
                            b";; Cannot handle <%s> of zero pronoun\n\x00" as
                                *const u8 as *const libc::c_char,
                            pp_code_to_kstr((*cf_ptr).pp[n as
                                usize][0 as
                                libc::c_int
                                as
                                usize]));
                    return 0 as libc::c_int;
                }
                j = 0 as libc::c_int;
                loop {
                    if !(LocationOrder[(*cf_ptr).pp[n as
                        usize][0 as
                        libc::c_int
                        as usize]
                        as usize][j as usize] !=
                        -(10 as libc::c_int) &&
                        (OptLearn ==
                            (0 as libc::c_int == 0) as libc::c_int ||
                            OptDiscFlag & 16 as libc::c_int != 0 ||
                            LocationLimit[(*cf_ptr).pp[n as
                                usize][0 as
                                libc::c_int
                                as
                                usize]
                                as usize] ==
                                -(10 as libc::c_int) ||
                            j <
                                LocationLimit[(*cf_ptr).pp[n as
                                    usize][0
                                    as
                                    libc::c_int
                                    as
                                    usize]
                                    as usize])) {
                        current_block = 5873035170358615968;
                        break;
                    }
                    match LocationOrder[(*cf_ptr).pp[n as
                        usize][0 as
                        libc::c_int
                        as usize]
                        as usize][j as usize] {
                        73729 | 81920 | 65536 => {
                            if cs.wrapping_offset_from(sentence_data.as_mut_ptr())
                                as libc::c_long >
                                0 as libc::c_int as libc::c_long {
                                CheckMatchedLC(cs.offset(-(1 as libc::c_int as
                                    isize)), cs,
                                               em_ptr, cpm_ptr, cmm_ptr, l,
                                               ptp, cf_ptr, n,
                                               LocationOrder[(*cf_ptr).pp[n as
                                                   usize][0
                                                   as
                                                   libc::c_int
                                                   as
                                                   usize]
                                                   as
                                                   usize][j as
                                                   usize]);
                            }
                        }
                        139265 | 147456 | 131072 => {
                            if cs.wrapping_offset_from(sentence_data.as_mut_ptr())
                                as libc::c_long >
                                1 as libc::c_int as libc::c_long {
                                CheckMatchedLC(cs.offset(-(2 as libc::c_int as
                                    isize)), cs,
                                               em_ptr, cpm_ptr, cmm_ptr, l,
                                               ptp, cf_ptr, n,
                                               LocationOrder[(*cf_ptr).pp[n as
                                                   usize][0
                                                   as
                                                   libc::c_int
                                                   as
                                                   usize]
                                                   as
                                                   usize][j as
                                                   usize]);
                            }
                        }
                        _ => {
                            CheckMatchedLC(cs, cs, em_ptr, cpm_ptr, cmm_ptr,
                                           l, ptp, cf_ptr, n,
                                           LocationOrder[(*cf_ptr).pp[n as
                                               usize][0
                                               as
                                               libc::c_int
                                               as
                                               usize]
                                               as
                                               usize][j as
                                               usize]);
                        }
                    }
                    if ScoreCheck(cf_ptr, n) != 0 {
                        current_block = 4548502464050555819;
                        break;
                    }
                    j += 1
                }
                match current_block {
                    4548502464050555819 => {}
                    _ =>
                    /* 2文より以前 */
                        {
                            i = 3 as libc::c_int;
                            loop {
                                if !(i <= PrevSentenceLimit) {
                                    current_block = 2472048668343472511;
                                    break;
                                }
                                if (cs.wrapping_offset_from(sentence_data.as_mut_ptr())
                                    as libc::c_long) < i as libc::c_long {
                                    current_block = 2472048668343472511;
                                    break;
                                }
                                CheckMatchedLC(cs.offset(-(i as isize)), cs,
                                               em_ptr, cpm_ptr, cmm_ptr, l, ptp,
                                               cf_ptr, n, 0 as libc::c_int);
                                if ScoreCheck(cf_ptr, n) != 0 {
                                    current_block = 4548502464050555819;
                                    break;
                                }
                                i += 1
                            }
                            match current_block {
                                4548502464050555819 => {}
                                _ => {
                                    if !(OptLearn ==
                                        (0 as libc::c_int == 0) as
                                            libc::c_int &&
                                        AlreadyDecidedFlag != 0) {
                                        if OptDiscFlag & 16 as libc::c_int != 0 {
                                            /* 例外タグ */
                                            /* for (i = 0; ExtraTags[i][0]; i++) */
                                            i =
                                                1 as
                                                    libc::c_int; /* とりあえず 不特定-人 */
                                            if ExtraCheck == 0 {
                                                EllipsisDetectSubcontractExtraTags(cs,
                                                                                   em_ptr,
                                                                                   cpm_ptr,
                                                                                   cmm_ptr,
                                                                                   l,
                                                                                   i,
                                                                                   cf_ptr,
                                                                                   n,
                                                                                   0
                                                                                       as
                                                                                       libc::c_int);
                                            }
                                            if classify_twin_candidate(cs, em_ptr,
                                                                       cpm_ptr) !=
                                                0 {
                                                if ScoreCheckCore(cf_ptr, n,
                                                                  maxscore,
                                                                  maxpos) != 0 {
                                                    clear_cands();
                                                    current_block =
                                                        4548502464050555819;
                                                } else {
                                                    current_block =
                                                        10369920510435091891;
                                                }
                                            } else {
                                                current_block =
                                                    10369920510435091891;
                                            }
                                            match current_block {
                                                4548502464050555819 => {}
                                                _ => { clear_cands(); }
                                            }
                                        } else if ScoreCheck(cf_ptr, n) == 0 {
                                            /* 閾値を越えるものが見つからなかった */
                                            /* 閾値を越えるものがなく、ガ格または格フレームに<主体>があるとき */
                                            if MatchPP((*cf_ptr).pp[n as
                                                usize][0
                                                as
                                                libc::c_int
                                                as
                                                usize],
                                                       b"\xe3\x82\xac\x00" as
                                                           *const u8 as
                                                           *const libc::c_char as
                                                           *mut libc::c_char) != 0
                                                ||
                                                cf_match_element((*cf_ptr).sm[n
                                                    as
                                                    usize],
                                                                 b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                                     as
                                                                     *mut libc::c_char,
                                                                 0 as
                                                                     libc::c_int)
                                                    != 0 {
                                                maxtag =
                                                    ExtraTags[1 as libc::c_int as
                                                        usize]; /* 不特定-人 */
                                                maxpos = -(1 as libc::c_int);
                                                maxscore =
                                                    EX_match_subject as
                                                        libc::c_float /
                                                        11 as libc::c_int as
                                                            libc::c_float
                                            } else { return 0 as libc::c_int; }
                                        }
                                    }
                                }
                            }
                        }
                }
            }
        }
    }
    if !maxtag.is_null() {
        if strcmp(maxtag,
                  b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\x00" as
                      *const u8 as *const libc::c_char) == 0 {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\xe3\x80\x91;%s;-1;-1;1\x00"
                        as *const u8 as *const libc::c_char,
                    pp_code_to_kstr_in_context(cpm_ptr,
                                               (*cf_ptr).pp[n as
                                                   usize][0 as
                                                   libc::c_int
                                                   as
                                                   usize]));
            assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                            0 as libc::c_int);
            StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                as
                isize)).as_mut_ptr().offset(0
                as
                libc::c_int
                as
                isize)
                as
                isize),
                                   0 as *mut libc::c_char,
                                   0 as *mut SENTENCE_DATA,
                                   -(2 as libc::c_int),
                                   0 as libc::c_int as libc::c_float,
                                   0 as libc::c_int);
            /* 指示対象を格フレームに保存 */
            AppendToCF(cpm_ptr, cmm_ptr, l, 0 as *mut TAG_DATA, cf_ptr, n,
                       maxscore, maxpos, 0 as *mut SENTENCE_DATA);
            return 1 as libc::c_int;
        } else {
            if strcmp(maxtag,
                      b"\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\x00" as *const u8
                          as *const libc::c_char) == 0 {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\xe3\x80\x91;%s;-1;-1;1\x00"
                            as *const u8 as *const libc::c_char,
                        pp_code_to_kstr_in_context(cpm_ptr,
                                                   (*cf_ptr).pp[n as
                                                       usize][0
                                                       as
                                                       libc::c_int
                                                       as
                                                       usize]));
                assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                                0 as libc::c_int);
                StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                    as
                    isize)).as_mut_ptr().offset(0
                    as
                    libc::c_int
                    as
                    isize)
                    as
                    isize),
                                       0 as *mut libc::c_char,
                                       0 as *mut SENTENCE_DATA,
                                       -(3 as libc::c_int),
                                       0 as libc::c_int as libc::c_float,
                                       0 as libc::c_int);
                return 1 as libc::c_int;
            } else {
                if strcmp(maxtag,
                          b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\x00"
                              as *const u8 as *const libc::c_char) == 0 {
                    sprintf(feature_buffer.as_mut_ptr(),
                            b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\xe3\x80\x91;%s;-1;-1;1\x00"
                                as *const u8 as *const libc::c_char,
                            pp_code_to_kstr_in_context(cpm_ptr,
                                                       (*cf_ptr).pp[n as
                                                           usize][0
                                                           as
                                                           libc::c_int
                                                           as
                                                           usize]));
                    assign_cfeature(&mut (*em_ptr).f,
                                    feature_buffer.as_mut_ptr(),
                                    0 as libc::c_int);
                    StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                        as
                        isize)).as_mut_ptr().offset(0
                        as
                        libc::c_int
                        as
                        isize)
                        as
                        isize),
                                           0 as *mut libc::c_char,
                                           0 as *mut SENTENCE_DATA,
                                           -(4 as libc::c_int),
                                           0 as libc::c_int as libc::c_float,
                                           0 as libc::c_int);
                    return 1 as libc::c_int;
                }
            }
        }
    } else if !maxs.is_null() {
        let mut distance: libc::c_int = 0;
        let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
        word =
            make_print_string((*maxs).tag_data.offset(maxi as isize),
                              0 as libc::c_int);
        distance =
            cs.wrapping_offset_from(maxs) as libc::c_long as libc::c_int;
        if distance == 0 as libc::c_int {
            strcpy(etc_buffer.as_mut_ptr(),
                   b"\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87\x00" as *const u8 as
                       *const libc::c_char);
        } else if distance > 0 as libc::c_int {
            sprintf(etc_buffer.as_mut_ptr(),
                    b"%d\xe6\x96\x87\xe5\x89\x8d\x00" as *const u8 as
                        *const libc::c_char, distance);
        }
        /* 決定した照応関係 */
        if (*cmm_ptr).result_lists_p[l as usize].flag[n as usize] !=
            -(1 as libc::c_int) {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"\xe7\x85\xa7\xe5\xbf\x9c\xe4\xbb\xae\xe6\xb1\xba\xe5\xae\x9a;%d;C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;%s;%d;%d;%.3f:%s(%s):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                        as *const u8 as *const libc::c_char,
                    (*(*cpm_ptr).elem_b_ptr[(*cmm_ptr).result_lists_p[l as
                        usize].flag[n
                        as
                        usize]
                        as usize]).num,
                    if !word.is_null() {
                        word as *const libc::c_char
                    } else { b"?\x00" as *const u8 as *const libc::c_char },
                    pp_code_to_kstr_in_context(cpm_ptr,
                                               (*cf_ptr).pp[n as
                                                   usize][0 as
                                                   libc::c_int
                                                   as
                                                   usize]),
                    distance, maxi, maxscore as libc::c_double,
                    if !(*maxs).KNPSID.is_null() {
                        (*maxs).KNPSID.offset(5 as libc::c_int as isize) as
                            *const libc::c_char
                    } else { b"?\x00" as *const u8 as *const libc::c_char },
                    etc_buffer.as_mut_ptr(), maxi);
        } else {
            /* 決定した省略関係 */
            sprintf(feature_buffer.as_mut_ptr(),
                    b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;%s;%d;%d;%.3f:%s(%s):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                        as *const u8 as *const libc::c_char,
                    if !word.is_null() {
                        word as *const libc::c_char
                    } else { b"?\x00" as *const u8 as *const libc::c_char },
                    pp_code_to_kstr_in_context(cpm_ptr,
                                               (*cf_ptr).pp[n as
                                                   usize][0 as
                                                   libc::c_int
                                                   as
                                                   usize]),
                    distance, maxi, maxscore as libc::c_double,
                    if !(*maxs).KNPSID.is_null() {
                        (*maxs).KNPSID.offset(5 as libc::c_int as isize) as
                            *const libc::c_char
                    } else { b"?\x00" as *const u8 as *const libc::c_char },
                    etc_buffer.as_mut_ptr(), maxi);
        }
        free(word as *mut libc::c_void);
        assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                        0 as libc::c_int);
        StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
            as
            isize)).as_mut_ptr().offset(0
            as
            libc::c_int
            as
            isize)
            as
            isize),
                               0 as *mut libc::c_char, maxs, maxi, maxscore,
                               distance);
        /* 指示対象を格フレームに保存 */
        AppendToCF(cpm_ptr, cmm_ptr, l,
                   (*maxs).tag_data.offset(maxi as isize), cf_ptr, n,
                   maxscore, maxpos, maxs);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectForNoun(mut sp: *mut SENTENCE_DATA,
                                               mut em_ptr: *mut ELLIPSIS_MGR,
                                               mut cpm_ptr: *mut CF_PRED_MGR,
                                               mut cmm_ptr: *mut CF_MATCH_MGR,
                                               mut l: libc::c_int,
                                               mut cf_ptr: *mut CASE_FRAME,
                                               mut n: libc::c_int)
                                               -> libc::c_int
/*==================================================================*/
{
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut cs: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut etc_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut ccp: *mut CASE_COMPONENT = 0 as *mut CASE_COMPONENT;
    maxscore = 0 as libc::c_int as libc::c_float;
    maxtag = 0 as *mut libc::c_char;
    maxs = 0 as *mut SENTENCE_DATA;
    maxpos = -(2 as libc::c_int);
    AlreadyDecidedFlag = 0 as libc::c_int;
    cs =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    i = 0 as libc::c_int;
    while i < (*sp).Sen_num {
        memset(*Bcheck.offset(i as isize) as *mut libc::c_void,
               0 as libc::c_int,
               (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(200 as libc::c_int as
                   libc::c_ulong));
        i += 1
    }
    ExtraCheck = 0 as libc::c_int;
    /* 共参照リンクを辿ってタグつけ
       ただし名詞に限定し(接尾辞除く)ここでは同じ表記の語を辿るだけ
       格解析の結果適当な格が埋まらなかった場合と
       共参照解析結果がない場合のみ実行される */
    if 0 as libc::c_int != 0 &&
        (*(*(*cpm_ptr).pred_b_ptr).head_ptr).Hinshi == 6 as libc::c_int &&
        {
            ccp =
                CheckTagTarget((*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr(),
                               (*(*cpm_ptr).pred_b_ptr).voice,
                               (*(*cmm_ptr).cf_ptr).cf_address as
                                   libc::c_int,
                               (*cf_ptr).pp[n as
                                   usize][0 as libc::c_int as
                                   usize],
                               (*cf_ptr).pp_str[n as usize]);
            !ccp.is_null()
        } && (*sp).Sen_num - (*ccp).sent_num < 5 as libc::c_int {
        if CheckHaveEllipsisComponent(cpm_ptr, cmm_ptr, l, (*ccp).word) == 0 {
            maxs =
                sentence_data.as_mut_ptr().offset((*ccp).sent_num as
                    isize).offset(-(1 as
                    libc::c_int
                    as
                    isize));
            maxi = (*ccp).tag_num;
            maxscore = 1.0f64 as libc::c_float;
            current_block = 16196696234010193729;
        } else { current_block = 17833034027772472439; }
    } else { current_block = 17833034027772472439; }
    match current_block {
        17833034027772472439 =>
        /* best解を探す場合 */
            {
                if OptDiscFlag & 2 as libc::c_int != 0 {
                    i = 0 as libc::c_int;
                    while i <= PrevSentenceLimit {
                        if (cs.wrapping_offset_from(sentence_data.as_mut_ptr()) as
                            libc::c_long) < i as libc::c_long {
                            break;
                        }
                        EllipsisDetectRecursive2(cs.offset(-(i as isize)), cs,
                                                 em_ptr, cpm_ptr, cmm_ptr, l,
                                                 (*cs.offset(-(i as
                                                     isize))).tag_data.offset((*cs.offset(-(i
                                                     as
                                                     isize))).Tag_num
                                                     as
                                                     isize).offset(-(1
                                                     as
                                                     libc::c_int
                                                     as
                                                     isize)),
                                                 cf_ptr, n, 0 as libc::c_int,
                                                 (0 as libc::c_int == 0) as
                                                     libc::c_int);
                        i += 1
                    }
                    /* 閾値を越えるものが見つからなかった */
                    if ScoreCheck(cf_ptr, n) == 0 { return 0 as libc::c_int; }
                } else if !(!(*(*cpm_ptr).pred_b_ptr).parent.is_null() &&
                    EllipsisDetectRecursive2(cs, cs, em_ptr, cpm_ptr,
                                             cmm_ptr, l,
                                             (*(*cpm_ptr).pred_b_ptr).parent,
                                             cf_ptr, n,
                                             0 as libc::c_int,
                                             0 as libc::c_int) != 0) {
                    /* 親 */
                    /* 主節 */
                    if !(EllipsisDetectRecursive2(cs, cs, em_ptr, cpm_ptr,
                                                  cmm_ptr, l,
                                                  (*cs).tag_data.offset((*cs).Tag_num
                                                      as
                                                      isize).offset(-(1
                                                      as
                                                      libc::c_int
                                                      as
                                                      isize)),
                                                  cf_ptr, n, 0 as libc::c_int,
                                                  (0 as libc::c_int == 0) as
                                                      libc::c_int) != 0) {
                        /* 前文より前 */
                        i = 1 as libc::c_int;
                        loop {
                            if !(i <= PrevSentenceLimit) {
                                current_block = 5529461102203738653;
                                break;
                            }
                            if (cs.wrapping_offset_from(sentence_data.as_mut_ptr())
                                as libc::c_long) < i as libc::c_long {
                                current_block = 5529461102203738653;
                                break;
                            }
                            if EllipsisDetectRecursive2(cs.offset(-(i as isize)),
                                                        cs, em_ptr, cpm_ptr,
                                                        cmm_ptr, l,
                                                        (*cs.offset(-(i as
                                                            isize))).tag_data.offset((*cs.offset(-(i
                                                            as
                                                            isize))).Tag_num
                                                            as
                                                            isize).offset(-(1
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize)),
                                                        cf_ptr, n,
                                                        0 as libc::c_int,
                                                        (0 as libc::c_int == 0) as
                                                            libc::c_int) != 0 {
                                current_block = 16196696234010193729;
                                break;
                            }
                            i += 1
                        }
                        match current_block {
                            16196696234010193729 => {}
                            _ => {
                                if !(OptLearn ==
                                    (0 as libc::c_int == 0) as libc::c_int &&
                                    AlreadyDecidedFlag != 0) {
                                    if OptDiscFlag & 16 as libc::c_int != 0 {
                                        if classify_twin_candidate(cs, em_ptr,
                                                                   cpm_ptr) != 0 {
                                            if ScoreCheckCore(cf_ptr, n, maxscore,
                                                              maxpos) != 0 {
                                                clear_cands();
                                                current_block =
                                                    16196696234010193729;
                                            } else {
                                                current_block =
                                                    14447253356787937536;
                                            }
                                        } else {
                                            current_block = 14447253356787937536;
                                        }
                                        match current_block {
                                            16196696234010193729 => {}
                                            _ => { clear_cands(); }
                                        }
                                    } else {
                                        /* 閾値を越えるものがないとき */
                                        return 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        _ => {}
    }
    /* 閾値を越えるものが見つかった */
    if !maxtag.is_null() {
        if strcmp(maxtag,
                  b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\x00" as
                      *const u8 as *const libc::c_char) == 0 {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\xe3\x80\x91;%s;-1;-1;1\x00"
                        as *const u8 as *const libc::c_char,
                    pp_code_to_kstr_in_context(cpm_ptr,
                                               (*cf_ptr).pp[n as
                                                   usize][0 as
                                                   libc::c_int
                                                   as
                                                   usize]));
            assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                            0 as libc::c_int);
            StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                as
                isize)).as_mut_ptr().offset(0
                as
                libc::c_int
                as
                isize)
                as
                isize),
                                   (*cf_ptr).pp_str[n as usize],
                                   0 as *mut SENTENCE_DATA,
                                   -(2 as libc::c_int),
                                   0 as libc::c_int as libc::c_float,
                                   0 as libc::c_int);
            /* 指示対象を格フレームに保存 */
            AppendToCF(cpm_ptr, cmm_ptr, l, 0 as *mut TAG_DATA, cf_ptr, n,
                       maxscore, maxpos, 0 as *mut SENTENCE_DATA);
            return 1 as libc::c_int;
        } else {
            if strcmp(maxtag,
                      b"\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\x00" as *const u8
                          as *const libc::c_char) == 0 {
                sprintf(feature_buffer.as_mut_ptr(),
                        b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\xe3\x80\x91;%s;-1;-1;1\x00"
                            as *const u8 as *const libc::c_char,
                        pp_code_to_kstr_in_context(cpm_ptr,
                                                   (*cf_ptr).pp[n as
                                                       usize][0
                                                       as
                                                       libc::c_int
                                                       as
                                                       usize]));
                assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                                0 as libc::c_int);
                StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                    as
                    isize)).as_mut_ptr().offset(0
                    as
                    libc::c_int
                    as
                    isize)
                    as
                    isize),
                                       (*cf_ptr).pp_str[n as usize],
                                       0 as *mut SENTENCE_DATA,
                                       -(3 as libc::c_int),
                                       0 as libc::c_int as libc::c_float,
                                       0 as libc::c_int);
                return 1 as libc::c_int;
            } else {
                if strcmp(maxtag,
                          b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\x00"
                              as *const u8 as *const libc::c_char) == 0 {
                    sprintf(feature_buffer.as_mut_ptr(),
                            b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\xe3\x80\x91;%s;-1;-1;1\x00"
                                as *const u8 as *const libc::c_char,
                            pp_code_to_kstr_in_context(cpm_ptr,
                                                       (*cf_ptr).pp[n as
                                                           usize][0
                                                           as
                                                           libc::c_int
                                                           as
                                                           usize]));
                    assign_cfeature(&mut (*em_ptr).f,
                                    feature_buffer.as_mut_ptr(),
                                    0 as libc::c_int);
                    StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
                        as
                        isize)).as_mut_ptr().offset(0
                        as
                        libc::c_int
                        as
                        isize)
                        as
                        isize),
                                           (*cf_ptr).pp_str[n as usize],
                                           0 as *mut SENTENCE_DATA,
                                           -(4 as libc::c_int),
                                           0 as libc::c_int as libc::c_float,
                                           0 as libc::c_int);
                    return 1 as libc::c_int;
                }
            }
        }
    } else if !maxs.is_null() {
        let mut distance: libc::c_int = 0;
        let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
        word =
            make_print_string((*maxs).tag_data.offset(maxi as isize),
                              0 as libc::c_int);
        distance =
            cs.wrapping_offset_from(maxs) as libc::c_long as libc::c_int;
        if distance == 0 as libc::c_int {
            strcpy(etc_buffer.as_mut_ptr(),
                   b"\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87\x00" as *const u8 as
                       *const libc::c_char);
        } else if distance > 0 as libc::c_int {
            sprintf(etc_buffer.as_mut_ptr(),
                    b"%d\xe6\x96\x87\xe5\x89\x8d\x00" as *const u8 as
                        *const libc::c_char, distance);
        }
        /* 決定した省略関係 */
        sprintf(feature_buffer.as_mut_ptr(),
                b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;%s;%d;%d;%.3f:%s(%s):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                    as *const u8 as *const libc::c_char,
                if !word.is_null() {
                    word as *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char },
                pp_code_to_kstr_in_context(cpm_ptr,
                                           (*cf_ptr).pp[n as
                                               usize][0 as
                                               libc::c_int
                                               as
                                               usize]),
                distance, maxi, maxscore as libc::c_double,
                if !(*maxs).KNPSID.is_null() {
                    (*maxs).KNPSID.offset(5 as libc::c_int as isize) as
                        *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char },
                etc_buffer.as_mut_ptr(), maxi);
        assign_cfeature(&mut (*em_ptr).f, feature_buffer.as_mut_ptr(),
                        0 as libc::c_int);
        free(word as *mut libc::c_void);
        StoreEllipsisComponent(&mut *(*em_ptr).cc.as_mut_ptr().offset(*(*(*cf_ptr).pp.as_mut_ptr().offset(n
            as
            isize)).as_mut_ptr().offset(0
            as
            libc::c_int
            as
            isize)
            as
            isize),
                               (*cf_ptr).pp_str[n as usize], maxs, maxi,
                               maxscore, distance);
        /* 指示対象を格フレームに保存 */
        AppendToCF(cpm_ptr, cmm_ptr, l,
                   (*maxs).tag_data.offset(maxi as isize), cf_ptr, n,
                   maxscore, maxpos, maxs);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn GetElementID(mut cfp: *mut CASE_FRAME,
                                      mut c: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* 格の番号から、格フレームの要素番号に変換する */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if (*cfp).pp[i as usize][0 as libc::c_int as usize] == c { return i; }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn RuleRecognition(mut cpm_ptr: *mut CF_PRED_MGR,
                                         mut cf_ptr: *mut CASE_FRAME,
                                         mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    /* <不特定:状況> をガ格としてとる判定詞 */
    if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                      b"\xef\xbc\xb4\xe6\x99\x82\xe9\x96\x93\xe3\x82\xac\xe7\x9c\x81\xe7\x95\xa5\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        MatchPP((*cf_ptr).pp[n as usize][0 as libc::c_int as usize],
                b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char) != 0 {
        sprintf(feature_buffer.as_mut_ptr(),
                b"C\xe7\x94\xa8;\xe3\x80\x90\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe7\x8a\xb6\xe6\xb3\x81\xe3\x80\x91;%s;-1;-1;1\x00"
                    as *const u8 as *const libc::c_char,
                pp_code_to_kstr_in_context(cpm_ptr,
                                           (*cf_ptr).pp[n as
                                               usize][0 as
                                               libc::c_int
                                               as
                                               usize]));
        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                        feature_buffer.as_mut_ptr(), 0 as libc::c_int);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckToCase(mut cpm_ptr: *mut CF_PRED_MGR,
                                     mut cmm_ptr: *mut CF_MATCH_MGR,
                                     mut l: libc::c_int,
                                     mut cf_ptr: *mut CASE_FRAME)
                                     -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    /* 格フレームに補文ト格に割り当てがあるかどうか調べる */
    i = 0 as libc::c_int;
    while i < (*cf_ptr).element_num {
        num = (*cmm_ptr).result_lists_p[l as usize].flag[i as usize];
        if num != -(1 as libc::c_int) &&
            MatchPP((*cf_ptr).pp[i as usize][0 as libc::c_int as usize],
                    b"\xe3\x83\x88\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) != 0 {
            /* check_feature(cpm_ptr->elem_b_ptr[num]->f, "補文")) { */
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    /* 入力文に補文ト格があるか調べる */
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if (*cpm_ptr).elem_b_num[i as usize] > -(2 as libc::c_int) &&
            MatchPP((*cpm_ptr).cf.pp[i as
                usize][0 as libc::c_int as usize],
                    b"\xe3\x83\x88\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) != 0 {
            /* check_feature(cpm_ptr->elem_b_ptr[i]->f, "補文")) { */
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectForVerbMain(mut sp: *mut SENTENCE_DATA,
                                                   mut em_ptr:
                                                   *mut ELLIPSIS_MGR,
                                                   mut cpm_ptr:
                                                   *mut CF_PRED_MGR,
                                                   mut cmm_ptr:
                                                   *mut CF_MATCH_MGR,
                                                   mut l: libc::c_int,
                                                   mut cf_ptr:
                                                   *mut CASE_FRAME,
                                                   mut order:
                                                   *mut *mut libc::c_char)
                                                   -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut num: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut demoflag: libc::c_int = 0;
    let mut toflag: libc::c_int = 0;
    let mut cases: [libc::c_int; 44] = [0; 44];
    let mut count: libc::c_int = 0 as libc::c_int;
    if OptDiscFlag & 64 as libc::c_int != 0 {
        toflag = 0 as libc::c_int
    } else {
        /* ト格<補文>があればヲ格を省略解析しない場合 */
        toflag = CheckToCase(cpm_ptr, cmm_ptr, l, cf_ptr)
    }
    j = 0 as libc::c_int;
    while **order.offset(j as isize) != 0 {
        let fresh43 = count;
        count = count + 1;
        cases[fresh43 as usize] = pp_kstr_to_code(*order.offset(j as isize));
        j += 1
    }
    j = 0 as libc::c_int;
    while DiscAddedCases[j as usize] != -(10 as libc::c_int) {
        let fresh44 = count;
        count = count + 1;
        cases[fresh44 as usize] = DiscAddedCases[j as usize];
        j += 1
    }
    cases[count as usize] = -(10 as libc::c_int);
    /* 格を与えられた順番に */
    j = 0 as libc::c_int;
    while cases[j as usize] != -(10 as libc::c_int) {
        i = 0 as libc::c_int;
        while i < (*cf_ptr).element_num {
            /* 指示詞の解析 (割り当てあり) */
            if OptEllipsis & 2 as libc::c_int != 0 &&
                (*cmm_ptr).result_lists_p[l as usize].flag[i as usize] !=
                    -(1 as libc::c_int) &&
                (*cf_ptr).pp[i as usize][0 as libc::c_int as usize] ==
                    cases[j as usize] &&
                !(*cpm_ptr).elem_b_ptr[(*cmm_ptr).result_lists_p[l as
                    usize].flag[i
                    as
                    usize]
                    as usize].is_null() &&
                !check_feature((*(*cpm_ptr).elem_b_ptr[(*cmm_ptr).result_lists_p[l
                    as
                    usize].flag[i
                    as
                    usize]
                    as usize]).f,
                               b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe5\xaf\xbe\xe8\xb1\xa1\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                demoflag = 1 as libc::c_int
            } else { demoflag = 0 as libc::c_int }
            if demoflag == 1 as libc::c_int ||
                OptEllipsis & 1 as libc::c_int != 0 &&
                    (*cf_ptr).pp[i as usize][0 as libc::c_int as usize] ==
                        cases[j as usize] &&
                    (*cmm_ptr).result_lists_p[l as usize].flag[i as usize]
                        == -(1 as libc::c_int) &&
                    !(toflag != 0 &&
                        MatchPP((*cf_ptr).pp[i as
                            usize][0 as libc::c_int
                            as usize],
                                b"\xe3\x83\xb2\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char) != 0) {
                result =
                    EllipsisDetectForVerb(sp, em_ptr, cpm_ptr, cmm_ptr, l,
                                          cf_ptr, i);
                /* append_cf_feature(&(em_ptr->f), cpm_ptr, cf_ptr, i); */
                if result != 0 {
                    (*em_ptr).cc[(*cf_ptr).pp[i as
                        usize][0 as libc::c_int as
                        usize] as
                        usize].score = maxscore;
                    if OptDiscPredMethod == 2 as libc::c_int ||
                        OptDiscPredMethod == 3 as libc::c_int {
                        (*em_ptr).score +=
                            if maxscore as libc::c_double > 1.0f64 {
                                EX_match_exact as libc::c_float
                            } else if maxscore <
                                0 as libc::c_int as libc::c_float {
                                0 as libc::c_int as libc::c_float
                            } else {
                                (11 as libc::c_int as libc::c_float) *
                                    maxscore
                            }
                    } else if maxpos == -(1 as libc::c_int) {
                        (*em_ptr).score += EX_match_subject as libc::c_float
                    } else {
                        (*em_ptr).score +=
                            if maxscore as libc::c_double > 1.0f64 {
                                EX_match_exact
                            } else {
                                *EX_match_score.as_mut_ptr().offset((maxscore
                                    *
                                    7 as
                                        libc::c_int
                                        as
                                        libc::c_float)
                                    as
                                    libc::c_int
                                    as
                                    isize)
                            } as libc::c_float
                    }
                } else { (demoflag) == 1 as libc::c_int; }
            }
            i += 1
        }
        j += 1
    }
    return (*em_ptr).score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn EllipsisDetectForNounMain(mut sp: *mut SENTENCE_DATA,
                                                   mut em_ptr:
                                                   *mut ELLIPSIS_MGR,
                                                   mut cpm_ptr:
                                                   *mut CF_PRED_MGR,
                                                   mut cmm_ptr:
                                                   *mut CF_MATCH_MGR,
                                                   mut l: libc::c_int,
                                                   mut cf_ptr:
                                                   *mut CASE_FRAME)
                                                   -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    // let mut num: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    // let mut demoflag: libc::c_int = 0;
    // let mut toflag: libc::c_int = 0;
    // let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*cf_ptr).element_num {
        /* 割り当てなし => 省略 */
        if OptEllipsis & 4 as libc::c_int != 0 &&
            (*cmm_ptr).result_lists_p[l as usize].flag[i as usize] ==
                -(1 as libc::c_int) {
            result =
                EllipsisDetectForNoun(sp, em_ptr, cpm_ptr, cmm_ptr, l, cf_ptr,
                                      i);
            /* append_cf_feature(&(em_ptr->f), cpm_ptr, cf_ptr, i); */
            if result != 0 {
                (*em_ptr).cc[(*cf_ptr).pp[i as
                    usize][0 as libc::c_int as
                    usize] as
                    usize].score = maxscore;
                /* 現在は、rule base */
                if 0 as libc::c_int != 0 &&
                    (OptDiscNounMethod == 2 as libc::c_int ||
                        OptDiscNounMethod == 3 as libc::c_int) {
                    (*em_ptr).score +=
                        if maxscore as libc::c_double > 1.0f64 {
                            EX_match_exact as libc::c_float
                        } else if maxscore < 0 as libc::c_int as libc::c_float
                        {
                            0 as libc::c_int as libc::c_float
                        } else {
                            (11 as libc::c_int as libc::c_float) * maxscore
                        }
                } else if maxpos == -(1 as libc::c_int) {
                    (*em_ptr).score += EX_match_subject as libc::c_float
                } else {
                    (*em_ptr).score +=
                        maxscore * 11 as libc::c_int as libc::c_float
                    /* maxscore > 1.0 ? EX_match_exact : *(EX_match_score+(int)(maxscore * 7)); */
                }
            }
        }
        i += 1
    }
    return (*em_ptr).score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CompareCPM(mut a: *mut CF_PRED_MGR,
                                    mut b: *mut CF_PRED_MGR) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    /* 異なる場合は 1 を返す */
    if (*a).cf.element_num != (*b).cf.element_num { return 1 as libc::c_int; }
    /* 順番が違うことがあるので単純比較ではだめ */
    i = 0 as libc::c_int;
    while i < (*a).cf.element_num {
        flag = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*b).cf.element_num {
            if (*a).elem_b_ptr[i as usize] == (*b).elem_b_ptr[j as usize] {
                flag = 1 as libc::c_int;
                break;
            } else { j += 1 }
        }
        if flag == 0 { return 1 as libc::c_int; }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CompareCMM(mut ap: *mut CF_PRED_MGR,
                                    mut a: *mut CF_MATCH_MGR,
                                    mut bp: *mut CF_PRED_MGR,
                                    mut b: *mut CF_MATCH_MGR,
                                    mut l: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* 異なる場合は 1 を返す */
    i = 0 as libc::c_int;
    while i < (*(*a).cf_ptr).element_num {
        if (*a).result_lists_p[0 as libc::c_int as usize].flag[i as usize] !=
            -(1 as libc::c_int) &&
            (*ap).elem_b_ptr[(*a).result_lists_p[0 as libc::c_int as
                usize].flag[i as
                usize]
                as usize] !=
                (*bp).elem_b_ptr[(*b).result_lists_p[l as
                    usize].flag[i as
                    usize]
                    as usize] {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CompareAssignList(mut maxem: *mut ELLIPSIS_MGR,
                                           mut cpm: *mut CF_PRED_MGR,
                                           mut cmm: *mut CF_MATCH_MGR,
                                           mut l: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*maxem).result_num {
        /* 同じものがすでにある場合 */
        if (*maxem).ecmm[i as usize].cmm.cf_ptr == (*cmm).cf_ptr &&
            (*maxem).ecmm[i as usize].element_num == (*cpm).cf.element_num
            &&
            CompareCPM(&mut (*(*maxem).ecmm.as_mut_ptr().offset(i as
                isize)).cpm,
                       cpm) == 0 &&
            CompareCMM(&mut (*(*maxem).ecmm.as_mut_ptr().offset(i as
                isize)).cpm,
                       &mut (*(*maxem).ecmm.as_mut_ptr().offset(i as
                           isize)).cmm,
                       cpm, cmm, l) == 0 {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CompareClosestScore(mut a: *mut CF_MATCH_MGR,
                                             mut b: *mut CF_MATCH_MGR,
                                             mut l: libc::c_int)
                                             -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut acount: libc::c_int = 0 as libc::c_int;
    let mut bcount: libc::c_int = 0 as libc::c_int;
    let mut ascore: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut bscore: libc::c_float = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < (*(*a).cf_ptr).element_num {
        if (*a).result_lists_p[0 as libc::c_int as usize].flag[i as usize] !=
            -(1 as libc::c_int) &&
            (*(*a).cf_ptr).adjacent[i as usize] ==
                (0 as libc::c_int == 0) as libc::c_int {
            acount += 1;
            ascore =
                (*a).result_lists_p[0 as libc::c_int as
                    usize].score[i as usize] as
                    libc::c_float
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*(*b).cf_ptr).element_num {
        if (*b).result_lists_p[l as usize].flag[i as usize] !=
            -(1 as libc::c_int) &&
            (*(*b).cf_ptr).adjacent[i as usize] ==
                (0 as libc::c_int == 0) as libc::c_int {
            bcount += 1;
            bscore =
                (*b).result_lists_p[l as usize].score[i as usize] as
                    libc::c_float
        }
        i += 1
    }
    if acount < bcount || acount == bcount && ascore < bscore {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CompareClosestExFrequency(mut a: *mut CF_MATCH_MGR,
                                                   mut b: *mut CF_MATCH_MGR)
                                                   -> libc::c_int
/*==================================================================*/
{
    /* マッチした直前格の用例の頻度を比較する関数 */
    let mut i: libc::c_int = 0;
    let mut afreq: libc::c_int = 0 as libc::c_int;
    let mut bfreq: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*(*a).cf_ptr).element_num {
        if (*a).result_lists_p[0 as libc::c_int as usize].flag[i as usize] !=
            -(1 as libc::c_int) &&
            (*(*a).cf_ptr).adjacent[i as usize] ==
                (0 as libc::c_int == 0) as libc::c_int &&
            (*a).result_lists_p[0 as libc::c_int as usize].pos[i as usize]
                >= 0 as libc::c_int {
            afreq +=
                *(*(*a).cf_ptr).ex_freq[i as
                    usize].offset((*a).result_lists_p[0
                    as
                    libc::c_int
                    as
                    usize].pos[i
                    as
                    usize]
                    as isize)
            /* 直前格が複数あれば足す */
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*(*b).cf_ptr).element_num {
        if (*b).result_lists_p[0 as libc::c_int as usize].flag[i as usize] !=
            -(1 as libc::c_int) &&
            (*(*b).cf_ptr).adjacent[i as usize] ==
                (0 as libc::c_int == 0) as libc::c_int &&
            (*b).result_lists_p[0 as libc::c_int as usize].pos[i as usize]
                >= 0 as libc::c_int {
            bfreq +=
                *(*(*b).cf_ptr).ex_freq[i as
                    usize].offset((*b).result_lists_p[0
                    as
                    libc::c_int
                    as
                    usize].pos[i
                    as
                    usize]
                    as isize)
        }
        i += 1
    }
    if afreq < bfreq { return 1 as libc::c_int; }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn CheckClosestAssigned(mut cmm: *mut CF_MATCH_MGR,
                                              mut l: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*(*cmm).cf_ptr).element_num {
        if (*(*cmm).cf_ptr).adjacent[i as usize] ==
            (0 as libc::c_int == 0) as libc::c_int {
            if (*cmm).result_lists_p[0 as libc::c_int as
                usize].flag[i as usize] !=
                -(1 as libc::c_int) {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            flag = 1 as libc::c_int
        }
        i += 1
    }
    if flag != 0 { return 0 as libc::c_int; }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn FindBestCFforContext(mut sp: *mut SENTENCE_DATA,
                                              mut maxem: *mut ELLIPSIS_MGR,
                                              mut cpm_ptr: *mut CF_PRED_MGR,
                                              mut order:
                                              *mut *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    // let mut type_0: libc::c_int = 0;
    let mut frame_num: libc::c_int = 0;
    let mut cf_array: *mut *mut CASE_FRAME = 0 as *mut *mut CASE_FRAME;
    let mut cmm: CF_MATCH_MGR =
        CF_MATCH_MGR {
            cf_ptr: 0 as *mut CASE_FRAME,
            score: 0.,
            pure_score: [0.; 10],
            sufficiency: 0.,
            result_num: 0,
            result_lists_p:
            [LIST { flag: [0; 24], score: [0.; 24], pos: [0; 24] };
                10],
            result_lists_d:
            [LIST { flag: [0; 24], score: [0.; 24], pos: [0; 24] };
                10],
            cpm: 0 as *mut cpm_def,
        };
    let mut tempecmm: ELLIPSIS_CMM =
        ELLIPSIS_CMM {
            cmm:
            CF_MATCH_MGR {
                cf_ptr: 0 as *mut CASE_FRAME,
                score: 0.,
                pure_score: [0.; 10],
                sufficiency: 0.,
                result_num: 0,
                result_lists_p:
                [LIST {
                    flag: [0; 24],
                    score: [0.; 24],
                    pos: [0; 24],
                }; 10],
                result_lists_d:
                [LIST {
                    flag: [0; 24],
                    score: [0.; 24],
                    pos: [0; 24],
                }; 10],
                cpm: 0 as *mut cpm_def,
            },
            cpm:
            CF_PRED_MGR {
                cf:
                CASE_FRAME {
                    type_0: 0,
                    type_flag: 0,
                    element_num: 0,
                    oblig: [0; 24],
                    adjacent: [0; 24],
                    pp: [[0; 10]; 24],
                    sp: [0; 24],
                    pp_str:
                    [0 as
                        *mut libc::c_char;
                        24],
                    sm:
                    [0 as
                        *mut libc::c_char;
                        24],
                    sm_delete:
                    [0 as
                        *mut libc::c_char;
                        24],
                    sm_delete_size: [0; 24],
                    sm_delete_num: [0; 24],
                    sm_specify:
                    [0 as
                        *mut libc::c_char;
                        24],
                    sm_specify_size: [0; 24],
                    sm_specify_num: [0; 24],
                    ex:
                    [0 as
                        *mut libc::c_char;
                        24],
                    ex_list:
                    [0 as
                        *mut *mut libc::c_char;
                        24],
                    ex_freq:
                    [0 as
                        *mut libc::c_int;
                        24],
                    ex_size: [0; 24],
                    ex_num: [0; 24],
                    freq: [0; 24],
                    semantics:
                    [0 as
                        *mut libc::c_char;
                        24],
                    gex_list:
                    [0 as
                        *mut *mut libc::c_char;
                        24],
                    gex_freq:
                    [0 as
                        *mut libc::c_double;
                        24],
                    gex_size: [0; 24],
                    gex_num: [0; 24],
                    voice: 0,
                    cf_address: 0,
                    cf_size: 0,
                    cf_id: [0; 280],
                    pred_type: [0; 4],
                    entry:
                    0 as
                        *mut libc::c_char,
                    imi: [0; 128],
                    etcflag: 0,
                    feature:
                    0 as
                        *mut libc::c_char,
                    weight: [0; 24],
                    samecase: [[0; 2]; 24],
                    cf_align:
                    [CF_ALIGNMENT {
                        cf_id:
                        0 as
                            *mut libc::c_char,
                        aligned_case:
                        [[0;
                            2];
                            24],
                    };
                        5],
                    pred_b_ptr:
                    0 as *mut TAG_DATA,
                    cf_similarity: 0.,
                },
                pred_b_ptr: 0 as *mut TAG_DATA,
                elem_b_ptr: [0 as *mut TAG_DATA; 24],
                para_b_ptr: [0 as *mut TAG_DATA; 24],
                elem_s_ptr: [0 as *mut sentence; 24],
                elem_b_num: [0; 24],
                score: 0.,
                result_num: 0,
                tie_num: 0,
                cmm:
                [CF_MATCH_MGR {
                    cf_ptr:
                    0 as
                        *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.; 24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    result_lists_d:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.; 24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    cpm:
                    0 as
                        *mut cpm_def,
                };
                    5],
                decided: 0,
            },
            element_num: 0,
        };
    let mut workem: ELLIPSIS_MGR =
        ELLIPSIS_MGR {
            score: 0.,
            pure_score: 0.,
            cc:
            [ELLIPSIS_COMPONENT {
                s: 0 as *mut SENTENCE_DATA,
                pp_str: 0 as *mut libc::c_char,
                bnst: 0,
                score: 0.,
                dist: 0,
                next:
                0 as
                    *mut ellipsis_component,
            };
                50],
            f: 0 as *mut _FEATURE,
            result_num: 0,
            ecmm:
            [ELLIPSIS_CMM {
                cmm:
                CF_MATCH_MGR {
                    cf_ptr:
                    0 as
                        *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    result_lists_d:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    cpm:
                    0 as
                        *mut cpm_def,
                },
                cpm:
                CF_PRED_MGR {
                    cf:
                    CASE_FRAME {
                        type_0:
                        0,
                        type_flag:
                        0,
                        element_num:
                        0,
                        oblig:
                        [0;
                            24],
                        adjacent:
                        [0;
                            24],
                        pp:
                        [[0;
                            10];
                            24],
                        sp:
                        [0;
                            24],
                        pp_str:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete_size:
                        [0;
                            24],
                        sm_delete_num:
                        [0;
                            24],
                        sm_specify:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_specify_size:
                        [0;
                            24],
                        sm_specify_num:
                        [0;
                            24],
                        ex:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        ex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        ex_freq:
                        [0
                            as
                            *mut libc::c_int;
                            24],
                        ex_size:
                        [0;
                            24],
                        ex_num:
                        [0;
                            24],
                        freq:
                        [0;
                            24],
                        semantics:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        gex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        gex_freq:
                        [0
                            as
                            *mut libc::c_double;
                            24],
                        gex_size:
                        [0;
                            24],
                        gex_num:
                        [0;
                            24],
                        voice:
                        0,
                        cf_address:
                        0,
                        cf_size:
                        0,
                        cf_id:
                        [0;
                            280],
                        pred_type:
                        [0;
                            4],
                        entry:
                        0 as
                            *mut libc::c_char,
                        imi:
                        [0;
                            128],
                        etcflag:
                        0,
                        feature:
                        0 as
                            *mut libc::c_char,
                        weight:
                        [0;
                            24],
                        samecase:
                        [[0;
                            2];
                            24],
                        cf_align:
                        [CF_ALIGNMENT {
                            cf_id:
                            0
                                as
                                *mut libc::c_char,
                            aligned_case:
                            [[0;
                                2];
                                24],
                        };
                            5],
                        pred_b_ptr:
                        0 as
                            *mut TAG_DATA,
                        cf_similarity:
                        0.,
                    },
                    pred_b_ptr:
                    0 as *mut TAG_DATA,
                    elem_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    para_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    elem_s_ptr:
                    [0 as
                        *mut sentence;
                        24],
                    elem_b_num: [0; 24],
                    score: 0.,
                    result_num: 0,
                    tie_num: 0,
                    cmm:
                    [CF_MATCH_MGR {
                        cf_ptr:
                        0
                            as
                            *mut CASE_FRAME,
                        score:
                        0.,
                        pure_score:
                        [0.;
                            10],
                        sufficiency:
                        0.,
                        result_num:
                        0,
                        result_lists_p:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        result_lists_d:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        cpm:
                        0
                            as
                            *mut cpm_def,
                    };
                        5],
                    decided: 0,
                },
                element_num: 0,
            }; 5],
        };
    InitEllipsisMGR(&mut workem);
    if OptDiscFlag & 1 as libc::c_int != 0 {
        frame_num = 0 as libc::c_int;
        cf_array =
            malloc_data(::std::mem::size_of::<*mut CASE_FRAME>() as
                            libc::c_ulong,
                        b"FindBestCFforContext\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut CASE_FRAME;
        l = 0 as libc::c_int;
        while l < (*(*cpm_ptr).pred_b_ptr).cf_num {
            if (*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l as isize)).etcflag &
                1 as libc::c_int != 0 ||
                (*(*cpm_ptr).pred_b_ptr).cf_num == 1 as libc::c_int {
                *cf_array =
                    (*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l as isize);
                frame_num = 1 as libc::c_int;
                break;
            } else { l += 1 }
        }
    } else if (*cpm_ptr).decided == 1 as libc::c_int {
        frame_num = (*cpm_ptr).tie_num;
        cf_array =
            malloc_data((::std::mem::size_of::<*mut CASE_FRAME>() as
                libc::c_ulong).wrapping_mul(frame_num as
                libc::c_ulong),
                        b"FindBestCFforContext\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut CASE_FRAME;
        l = 0 as libc::c_int;
        while l < frame_num {
            let ref mut fresh45 = *cf_array.offset(l as isize);
            *fresh45 = (*cpm_ptr).cmm[l as usize].cf_ptr;
            l += 1
        }
    } else {
        frame_num = 0 as libc::c_int;
        cf_array =
            malloc_data((::std::mem::size_of::<*mut CASE_FRAME>() as
                libc::c_ulong).wrapping_mul((*(*cpm_ptr).pred_b_ptr).cf_num
                as
                libc::c_ulong),
                        b"FindBestCFforContext\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut CASE_FRAME;
        /* 格フレームcache */
        if OptUseSmfix == (0 as libc::c_int == 0) as libc::c_int &&
            CFSimExist == (0 as libc::c_int == 0) as libc::c_int {
            let mut cfp: *mut CFLIST = 0 as *mut CFLIST;
            let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
            key =
                get_pred_id((*(*(*cpm_ptr).pred_b_ptr).cf_ptr).cf_id.as_mut_ptr());
            if !key.is_null() {
                cfp = CheckCF(key);
                free(key as *mut libc::c_void);
                if !cfp.is_null() {
                    l = 0 as libc::c_int;
                    while l < (*(*cpm_ptr).pred_b_ptr).cf_num {
                        i = 0 as libc::c_int;
                        while i < (*cfp).cfid_num {
                            if (*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l as
                                isize)).type_0
                                == (*cpm_ptr).cf.type_0 &&
                                {
                                    let ref mut fresh46 =
                                        (*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l
                                            as
                                            isize)).cf_similarity;
                                    *fresh46 =
                                        get_cfs_similarity((*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l
                                            as
                                            isize)).cf_id.as_mut_ptr(),
                                                           *(*cfp).cfid.offset(i
                                                               as
                                                               isize));
                                    (*fresh46) > CFSimThreshold
                                } {
                                let fresh47 = frame_num;
                                frame_num = frame_num + 1;
                                let ref mut fresh48 =
                                    *cf_array.offset(fresh47 as isize);
                                *fresh48 =
                                    (*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l
                                        as
                                        isize);
                                break;
                            } else { i += 1 }
                        }
                        l += 1
                    }
                    (*(*cpm_ptr).pred_b_ptr).e_cf_num = frame_num;
                    if VerboseLevel as libc::c_uint >=
                        VERBOSE2 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b";; \xe2\x98\x85 %s [%s] CF -> %d/%d\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*sp).KNPSID,
                                (*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr(),
                                frame_num, (*(*cpm_ptr).pred_b_ptr).cf_num);
                    }
                }
            }
        }
        if frame_num == 0 as libc::c_int {
            let mut hiragana_prefer_flag: libc::c_int = 0 as libc::c_int;
            /* 表記がひらがなの場合:
		   格フレームの表記がひらがなの場合が多ければひらがなの格フレームのみを対象に、
		   ひらがな以外が多ければひらがな以外のみを対象にする */
            if OptCaseFlag & 32 as libc::c_int == 0 &&
                check_str_type((*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr()
                                   as *mut libc::c_uchar, 2 as libc::c_int,
                               0 as libc::c_int) != 0 {
                if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                  b"\xe4\xbb\xa3\xe8\xa1\xa8\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    hiragana_prefer_flag = 1 as libc::c_int
                } else { hiragana_prefer_flag = -(1 as libc::c_int) }
            }
            l = 0 as libc::c_int;
            while l < (*(*cpm_ptr).pred_b_ptr).cf_num {
                if (*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l as
                    isize)).type_0
                    == (*cpm_ptr).cf.type_0 &&
                    (hiragana_prefer_flag == 0 as libc::c_int ||
                        hiragana_prefer_flag > 0 as libc::c_int &&
                            check_str_type((*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l
                                as
                                isize)).entry
                                               as *mut libc::c_uchar,
                                           2 as libc::c_int,
                                           0 as libc::c_int) != 0 ||
                        hiragana_prefer_flag < 0 as libc::c_int &&
                            check_str_type((*(*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l
                                as
                                isize)).entry
                                               as *mut libc::c_uchar,
                                           2 as libc::c_int,
                                           0 as libc::c_int) == 0) {
                    let fresh49 = frame_num;
                    frame_num = frame_num + 1;
                    let ref mut fresh50 = *cf_array.offset(fresh49 as isize);
                    *fresh50 =
                        (*(*cpm_ptr).pred_b_ptr).cf_ptr.offset(l as isize)
                }
                l += 1
            }
        }
    }
    /* 候補の格フレームについて省略解析を実行 */
    l = 0 as libc::c_int;
    while l < frame_num {
        /* OR の格フレームを除く */
        if !((**cf_array.offset(l as isize)).etcflag & 1 as libc::c_int != 0
            && frame_num != 1 as libc::c_int) {
            /* 格フレームを仮指定 */
            cmm.cf_ptr = *cf_array.offset(l as isize);
            (*cpm_ptr).result_num = 1 as libc::c_int;
            /* 入力側格要素を設定
	   照応解析時はすでにある格要素を上書きしてしまうのでここで再設定
	   それ以外のときは下の DeleteFromCF() で省略要素をクリア */
            if OptEllipsis & 2 as libc::c_int != 0 ||
                (*cpm_ptr).cf.type_flag != 0 {
                make_data_cframe(sp, cpm_ptr);
            }
            /* 今ある格要素を対応づけ */
            case_frame_match(cpm_ptr, &mut cmm, OptCFMode,
                             -(1 as libc::c_int), 0 as *mut CF_PRED_MGR);
            (*cpm_ptr).score = cmm.score;
            /* for (i = 0; i < cmm.result_num; i++) */
            i = 0 as libc::c_int;
            ClearEllipsisMGR(&mut workem);
            if (*cpm_ptr).cf.type_0 == 2 as libc::c_int {
                EllipsisDetectForNounMain(sp, &mut workem, cpm_ptr, &mut cmm,
                                          i, *cf_array.offset(l as isize));
            } else {
                EllipsisDetectForVerbMain(sp, &mut workem, cpm_ptr, &mut cmm,
                                          i, *cf_array.offset(l as isize),
                                          order);
            }
            if 0 as libc::c_int != 0 && CheckClosestAssigned(&mut cmm, i) == 0
            {
                workem.score = -(1 as libc::c_int) as libc::c_float
            } else if cmm.score >= 0 as libc::c_int as libc::c_double {
                /* 直接の格要素の正規化していないスコアを足す */
                workem.score =
                    (workem.score as libc::c_double +
                        cmm.pure_score[i as usize]) as libc::c_float;
                workem.pure_score = workem.score;
                /* 正規化 */
                if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
                    workem.score =
                        (workem.score as libc::c_double /
                            sqrt(count_pat_element(cmm.cf_ptr,
                                                   &mut *cmm.result_lists_p.as_mut_ptr().offset(i
                                                       as
                                                       isize))
                                as libc::c_double)) as libc::c_float
                }
                cmm.score = workem.score as libc::c_double
            } else {
                /* 格解析失敗のとき -- 解析をひとつだけ結果に入れるために
	       最大スコアのデフォルトを -2 にしている */
                workem.score = cmm.score as libc::c_float
            }
            /* DEBUG 表示 */
            if VerboseLevel as libc::c_uint >=
                VERBOSE3 as libc::c_int as libc::c_uint {
                fprintf(stdout,
                        b"\xe2\x98\x85 \xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0 %d\n\x00"
                            as *const u8 as *const libc::c_char, l);
                print_data_cframe(cpm_ptr, &mut cmm);
                print_good_crrspnds(cpm_ptr, &mut cmm, 1 as libc::c_int);
                fprintf(stdout,
                        b"   FEATURES: \x00" as *const u8 as
                            *const libc::c_char);
                print_feature(workem.f, Outfp);
                fputc('\n' as i32, Outfp);
            }
            if workem.score > (*maxem).score ||
                workem.score == (*maxem).score &&
                    (CompareClosestScore(&mut (*(*maxem).ecmm.as_mut_ptr().offset(0
                        as
                        libc::c_int
                        as
                        isize)).cmm,
                                         &mut cmm, i) != 0 ||
                        CompareClosestExFrequency(&mut (*(*maxem).ecmm.as_mut_ptr().offset(0
                            as
                            libc::c_int
                            as
                            isize)).cmm,
                                                  &mut cmm) != 0) {
                /* 頻度が高いとき */
                k = 0 as libc::c_int;
                while k < 50 as libc::c_int {
                    ClearEllipsisComponent(&mut *(*maxem).cc.as_mut_ptr().offset(k
                        as
                        isize));
                    CopyEllipsisComponent(&mut *(*maxem).cc.as_mut_ptr().offset(k
                        as
                        isize),
                                          &mut *workem.cc.as_mut_ptr().offset(k
                                              as
                                              isize));
                    k += 1
                }
                (*maxem).score = workem.score;
                (*maxem).pure_score = workem.pure_score;
                (*maxem).f = workem.f;
                workem.f = 0 as FEATUREptr;
                /* ひとつずつずらす */
                k =
                    if (*maxem).result_num >=
                        5 as libc::c_int - 1 as libc::c_int {
                        ((*maxem).result_num) - 1 as libc::c_int
                    } else { (*maxem).result_num };
                while k >= 0 as libc::c_int {
                    (*maxem).ecmm[(k + 1 as libc::c_int) as usize] =
                        (*maxem).ecmm[k as usize];
                    k -= 1
                }
                /* 今回が最大マッチ */
                (*maxem).ecmm[0 as libc::c_int as usize].cmm = cmm;
                (*maxem).ecmm[0 as libc::c_int as usize].cpm = *cpm_ptr;
                (*maxem).ecmm[0 as libc::c_int as usize].element_num =
                    (*cpm_ptr).cf.element_num;
                (*maxem).ecmm[0 as libc::c_int as usize].cmm.result_num =
                    1 as libc::c_int;
                (*maxem).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_p[0 as libc::c_int
                    as usize] =
                    cmm.result_lists_p[i as usize];
                (*maxem).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_d[0 as libc::c_int
                    as usize] =
                    cmm.result_lists_d[i as usize];
                (*maxem).ecmm[0 as libc::c_int as
                    usize].cmm.pure_score[0 as libc::c_int as
                    usize] =
                    workem.pure_score as libc::c_double;
                if (*maxem).result_num < 5 as libc::c_int - 1 as libc::c_int {
                    (*maxem).result_num += 1
                }
            } else if CompareAssignList(maxem, cpm_ptr, &mut cmm, i) != 0 {
                (*maxem).ecmm[(*maxem).result_num as usize].cmm = cmm;
                (*maxem).ecmm[(*maxem).result_num as usize].cpm = *cpm_ptr;
                (*maxem).ecmm[(*maxem).result_num as usize].element_num =
                    (*cpm_ptr).cf.element_num;
                (*maxem).ecmm[(*maxem).result_num as usize].cmm.result_num =
                    1 as libc::c_int;
                (*maxem).ecmm[(*maxem).result_num as
                    usize].cmm.result_lists_p[0 as libc::c_int
                    as usize] =
                    cmm.result_lists_p[i as usize];
                (*maxem).ecmm[(*maxem).result_num as
                    usize].cmm.result_lists_d[0 as libc::c_int
                    as usize] =
                    cmm.result_lists_d[i as usize];
                (*maxem).ecmm[(*maxem).result_num as
                    usize].cmm.pure_score[0 as libc::c_int as
                    usize] =
                    workem.pure_score as libc::c_double;
                k = (*maxem).result_num - 1 as libc::c_int;
                while k >= 0 as libc::c_int {
                    if !((*maxem).ecmm[k as usize].cmm.score <
                        (*maxem).ecmm[(k + 1 as libc::c_int) as
                            usize].cmm.score) {
                        break;
                    }
                    tempecmm = (*maxem).ecmm[k as usize];
                    (*maxem).ecmm[k as usize] =
                        (*maxem).ecmm[(k + 1 as libc::c_int) as usize];
                    (*maxem).ecmm[(k + 1 as libc::c_int) as usize] = tempecmm;
                    k -= 1
                }
                if (*maxem).result_num < 5 as libc::c_int - 1 as libc::c_int {
                    (*maxem).result_num += 1
                }
            }
            /* 新しい種類の格フレーム(割り当て)を保存 */
            /* 格フレームの追加エントリの削除 */
            if OptEllipsis & 2 as libc::c_int == 0 {
                DeleteFromCF(&mut workem, cpm_ptr, &mut cmm, i);
            }
        }
        l += 1
    }
    free(cf_array as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn AssignFeaturesByProgram(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    /* 撲滅予定 */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        /* 隣以外の AのB はルールで与えられていない */
        if check_feature((*(*sp).tag_data.offset(i as isize)).f,
                         b"\xe6\xba\x96\xe4\xb8\xbb\xe9\xa1\x8c\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            !(*(*sp).tag_data.offset(i as isize)).parent.is_null() &&
            !check_feature((*(*(*sp).tag_data.offset(i as
                isize)).parent).f,
                           b"\xe4\xb8\xbb\xe9\xa1\x8c\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            assign_cfeature(&mut (*(*sp).tag_data.offset(i as isize)).f,
                            b"\xe6\xba\x96\xe4\xb8\xbb\xe9\xa1\x8c\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mark_location_classes(mut sp: *mut SENTENCE_DATA,
                                               mut tp: *mut TAG_DATA)
                                               -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0; /* 自分は対象外 */
    let mut j: libc::c_int = 0; /* 自分の子供は対象外 */
    let mut cs: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    cs =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    LC =
        malloc_data((::std::mem::size_of::<*mut libc::c_int>() as
            libc::c_ulong).wrapping_mul((*sp).Sen_num as
            libc::c_ulong),
                    b"mark_location_classes\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Sen_num {
        let ref mut fresh51 = *LC.offset(i as isize);
        *fresh51 =
            malloc_data((::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul(200 as libc::c_int as
                libc::c_ulong),
                        b"mark_location_classes\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_int;
        memset(*LC.offset(i as isize) as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(200 as libc::c_int as
                   libc::c_ulong));
        i += 1
    }
    ExtraLC = 0 as libc::c_int;
    *(*LC.offset(0 as libc::c_int as isize)).offset((*tp).num as isize) =
        -(10 as libc::c_int);
    _SearchCaseComponent(cs, tp, LC, -(10 as libc::c_int), 0 as libc::c_int);
    _SearchPV(cs, tp, LC);
    _SearchParentV(cs, tp, LC);
    _SearchParentNParentV(cs, tp, LC);
    _SearchParentVParentV(cs, tp, LC);
    _SearchChildPV(cs, tp, LC);
    _SearchChildV(cs, tp, LC);
    _SearchMC(cs, tp, LC, 0 as libc::c_int);
    _SearchSC(cs, tp, LC, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*cs).Tag_num {
        if !(*(*LC.offset(0 as libc::c_int as isize)).offset(i as isize) !=
            0 as libc::c_int) {
            if i < (*tp).num {
                *(*LC.offset(0 as libc::c_int as isize)).offset(i as isize) =
                    0x8000 as libc::c_int;
                mark_all_children(cs, (*cs).tag_data.offset(i as isize), LC,
                                  0x8000 as libc::c_int, 0 as libc::c_int);
            } else {
                *(*LC.offset(0 as libc::c_int as isize)).offset(i as isize) =
                    0x9000 as libc::c_int;
                mark_all_children(cs, (*cs).tag_data.offset(i as isize), LC,
                                  0x9000 as libc::c_int, 0 as libc::c_int);
            }
        }
        i += 1
    }
    if cs.wrapping_offset_from(sentence_data.as_mut_ptr()) as libc::c_long >
        0 as libc::c_int as libc::c_long {
        _SearchMC(cs.offset(-(1 as libc::c_int as isize)), 0 as *mut TAG_DATA,
                  LC, 1 as libc::c_int);
        _SearchSC(cs.offset(-(1 as libc::c_int as isize)), 0 as *mut TAG_DATA,
                  LC, 1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < (*cs.offset(-(1 as libc::c_int as isize))).Tag_num {
            if !(*(*LC.offset(1 as libc::c_int as isize)).offset(i as isize)
                != 0 as libc::c_int) {
                *(*LC.offset(1 as libc::c_int as isize)).offset(i as isize) =
                    0x10000 as libc::c_int;
                mark_all_children(cs.offset(-(1 as libc::c_int as isize)),
                                  (*cs.offset(-(1 as libc::c_int as
                                      isize))).tag_data.offset(i
                                      as
                                      isize),
                                  LC, 0x10000 as libc::c_int,
                                  1 as libc::c_int);
            }
            i += 1
        }
    }
    if cs.wrapping_offset_from(sentence_data.as_mut_ptr()) as libc::c_long >
        1 as libc::c_int as libc::c_long {
        _SearchMC(cs.offset(-(2 as libc::c_int as isize)), 0 as *mut TAG_DATA,
                  LC, 2 as libc::c_int);
        _SearchSC(cs.offset(-(2 as libc::c_int as isize)), 0 as *mut TAG_DATA,
                  LC, 2 as libc::c_int);
        i = 0 as libc::c_int;
        while i < (*cs.offset(-(2 as libc::c_int as isize))).Tag_num {
            if !(*(*LC.offset(2 as libc::c_int as isize)).offset(i as isize)
                != 0 as libc::c_int) {
                *(*LC.offset(2 as libc::c_int as isize)).offset(i as isize) =
                    0x20000 as libc::c_int;
                mark_all_children(cs.offset(-(2 as libc::c_int as isize)),
                                  (*cs.offset(-(2 as libc::c_int as
                                      isize))).tag_data.offset(i
                                      as
                                      isize),
                                  LC, 0x20000 as libc::c_int,
                                  2 as libc::c_int);
            }
            i += 1
        }
    }
    /* 2文以前 */
    j = 3 as libc::c_int;
    while j <= PrevSentenceLimit {
        if (cs.wrapping_offset_from(sentence_data.as_mut_ptr()) as
            libc::c_long) < j as libc::c_long {
            break;
        }
        i = 0 as libc::c_int;
        while i < (*cs.offset(-(j as isize))).Tag_num {
            *(*LC.offset(j as isize)).offset(i as isize) = 0 as libc::c_int;
            i += 1
        }
        j += 1
    }
    if VerboseLevel as libc::c_uint >= VERBOSE2 as libc::c_int as libc::c_uint
    {
        let mut j_0: libc::c_int = 0;
        fprintf(stderr,
                b";;; %s for %s(%d):\x00" as *const u8 as *const libc::c_char,
                if !(*cs).KNPSID.is_null() {
                    (*cs).KNPSID as *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char },
                (*(*tp).head_ptr).Goi.as_mut_ptr(), (*tp).num);
        i = 0 as libc::c_int;
        while i < (*sp).Sen_num {
            j_0 = 0 as libc::c_int;
            while j_0 < (*cs.offset(-(i as isize))).Tag_num {
                fprintf(stderr,
                        b" %s(%d):%s\x00" as *const u8 as *const libc::c_char,
                        (*(*(*cs.offset(-(i as
                            isize))).tag_data.offset(j_0 as
                            isize)).head_ptr).Goi.as_mut_ptr(),
                        j_0,
                        loc_code_to_str(*(*LC.offset(i as
                            isize)).offset(j_0 as
                            isize)));
                j_0 += 1
            }
            i += 1
        }
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn merge_cf_ptr(mut cf_ptr1: *mut CASE_FRAME,
                                      mut cf_ptr2: *mut CASE_FRAME)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf_ptr2).element_num {
        j = (*cf_ptr1).element_num + i;
        if j >= 24 as libc::c_int { break; }
        /* semantics */
        (*cf_ptr1).oblig[j as usize] =
            (*cf_ptr2).oblig[i as usize]; /* oblig */
        (*cf_ptr1).adjacent[j as usize] =
            (*cf_ptr2).adjacent[i as usize]; /* adjacent */
        k = 0 as libc::c_int;
        while k < 10 as libc::c_int {
            (*cf_ptr1).pp[j as usize][k as usize] =
                (*cf_ptr2).pp[i as usize][k as usize];
            k += 1
            /* pp */
        } /* sp */
        (*cf_ptr1).sp[j as usize] = (*cf_ptr2).sp[i as usize]; /* pp_str */
        (*cf_ptr1).pp_str[j as usize] =
            strdup_with_check((*cf_ptr2).pp_str[i as usize]); /* sm */
        (*cf_ptr1).sm[j as usize] =
            strdup_with_check((*cf_ptr2).sm[i as usize]); /* sm_delete */
        (*cf_ptr1).sm_delete[j as usize] =
            strdup_with_check((*cf_ptr2).sm_delete[i as
                usize]); /* sm_delete_size */
        if !(*cf_ptr2).sm_delete[i as usize].is_null() {
            (*cf_ptr1).sm_delete_size[j as usize] =
                (*cf_ptr2).sm_delete_size[i as usize];
            (*cf_ptr1).sm_delete_num[j as usize] =
                (*cf_ptr2).sm_delete_num[i as usize]
            /* sm_delete_num */
        } /* sm_specify */
        (*cf_ptr1).sm_specify[j as usize] =
            strdup_with_check((*cf_ptr2).sm_specify[i as
                usize]); /* sm_specify_size */
        if !(*cf_ptr1).sm_specify[i as usize].is_null() {
            (*cf_ptr1).sm_specify_size[j as usize] =
                (*cf_ptr2).sm_specify_size[i as usize];
            (*cf_ptr1).sm_specify_num[j as usize] =
                (*cf_ptr2).sm_specify_num[i as usize]
            /* sm_specify_num */
        } /* ex */
        (*cf_ptr1).ex[j as usize] =
            strdup_with_check((*cf_ptr2).ex[i as usize]);
        if (*cf_ptr2).ex_num[i as usize] != 0 {
            (*cf_ptr1).ex_list[j as usize] =
                malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                    libc::c_ulong).wrapping_mul((*cf_ptr2).ex_num[i
                    as
                    usize]
                    as
                    libc::c_ulong),
                            b"merge_cf_ptr\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut *mut libc::c_char;
            k = 0 as libc::c_int;
            while k < (*cf_ptr2).ex_num[i as usize] {
                let ref mut fresh52 =
                    *(*cf_ptr1).ex_list[j as usize].offset(k as isize);
                *fresh52 =
                    strdup_with_check(*(*cf_ptr2).ex_list[i as
                        usize].offset(k
                        as
                        isize));
                k += 1
                /* ex_num */
                /* ex_list */
            }
            (*cf_ptr1).ex_freq[j as usize] =
                malloc_data((::std::mem::size_of::<libc::c_int>() as
                    libc::c_ulong).wrapping_mul((*cf_ptr2).ex_num[i
                    as
                    usize]
                    as
                    libc::c_ulong),
                            b"merge_cf_ptr\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_int;
            k = 0 as libc::c_int;
            while k < (*cf_ptr2).ex_num[i as usize] {
                *(*cf_ptr1).ex_freq[j as usize].offset(k as isize) =
                    *(*cf_ptr2).ex_freq[i as usize].offset(k as isize);
                k += 1
            }
            (*cf_ptr1).ex_size[j as usize] = (*cf_ptr2).ex_size[i as usize];
            (*cf_ptr1).ex_num[j as usize] = (*cf_ptr2).ex_num[i as usize]
        }
        (*cf_ptr1).semantics[j as usize] =
            strdup_with_check((*cf_ptr2).semantics[i as usize]);
        i += 1
    }
    (*cf_ptr1).element_num += i;
    /* ex_freq */
    /* ex_size */
    /* element_num */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn merge_em(mut em1: *mut ELLIPSIS_MGR,
                                  mut em2: *mut ELLIPSIS_MGR)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* em2->cc[41].scoreに値が入っている場合はマージしない(本来、空であるはず、要検討) */
    if (*em2).cc[41 as libc::c_int as usize].score >
        0 as libc::c_int as libc::c_float {
        return;
    }
    /* CF_ELEMENT_MAXを上回ってしまう場合は一切マージを行わない */
    if (*em1).ecmm[0 as libc::c_int as usize].element_num +
        (*em2).ecmm[0 as libc::c_int as usize].element_num >=
        24 as libc::c_int ||
        (*(*em1).ecmm[0 as libc::c_int as usize].cmm.cf_ptr).element_num +
            (*(*em2).ecmm[0 as libc::c_int as
                usize].cmm.cf_ptr).element_num >=
            24 as libc::c_int {
        return;
    }
    /* 一番良い結果(ecmm[0])のみをマージする */
    (*em1).score += (*em2).score;
    (*em1).pure_score += (*em2).pure_score;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !(*em2).cc[i as usize].s.is_null() ||
            (*em2).cc[i as usize].bnst != 0 {
            (*em1).cc[i as usize] = (*em2).cc[i as usize];
            (*em2).cc[i as usize].s = 0 as *mut SENTENCE_DATA;
            (*em2).cc[i as usize].pp_str = 0 as *mut libc::c_char;
            (*em2).cc[i as usize].bnst = 0 as libc::c_int;
            (*em2).cc[i as usize].score = 0 as libc::c_int as libc::c_float;
            (*em2).cc[i as usize].dist = 0 as libc::c_int;
            (*em2).cc[i as usize].next = 0 as *mut ellipsis_component
        }
        i += 1
    }
    (*em1).ecmm[0 as libc::c_int as usize].cpm.score +=
        (*em2).ecmm[0 as libc::c_int as usize].cpm.score;
    /* 名詞解析の入力格要素 */
    i = 0 as libc::c_int;
    while i < (*em2).ecmm[0 as libc::c_int as usize].element_num {
        j = (*em1).ecmm[0 as libc::c_int as usize].element_num + i;
        if j >= 24 as libc::c_int { break; }
        /* CF_PRED */
        k = 0 as libc::c_int;
        while k < 10 as libc::c_int {
            (*em1).ecmm[0 as libc::c_int as
                usize].cpm.cf.pp[j as usize][k as usize] =
                (*em2).ecmm[0 as libc::c_int as
                    usize].cpm.cf.pp[i as usize][k as usize];
            k += 1
        }
        strcpy((*em1).ecmm[0 as libc::c_int as usize].cpm.cf.sm[j as usize],
               (*em2).ecmm[0 as libc::c_int as usize].cpm.cf.sm[i as usize]);
        (*em1).ecmm[0 as libc::c_int as usize].cpm.elem_b_ptr[j as usize] =
            (*em2).ecmm[0 as libc::c_int as usize].cpm.elem_b_ptr[i as usize];
        (*em1).ecmm[0 as libc::c_int as usize].cpm.elem_s_ptr[j as usize] =
            (*em2).ecmm[0 as libc::c_int as usize].cpm.elem_s_ptr[i as usize];
        (*em1).ecmm[0 as libc::c_int as usize].cpm.elem_b_num[j as usize] =
            (*em2).ecmm[0 as libc::c_int as usize].cpm.elem_b_num[i as usize];
        /* CF_MATCH (結果はベストだけなので0だけ見る) */
        if (*em2).ecmm[0 as libc::c_int as
            usize].cmm.result_lists_d[0 as libc::c_int as
            usize].flag[i as
            usize]
            == -(2 as libc::c_int) {
            (*em1).ecmm[0 as libc::c_int as
                usize].cmm.result_lists_d[0 as libc::c_int as
                usize].flag[j as
                usize]
                = -(2 as libc::c_int)
        } else {
            (*em1).ecmm[0 as libc::c_int as
                usize].cmm.result_lists_d[0 as libc::c_int as
                usize].flag[j as
                usize]
                =
                (*em2).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_d[0 as libc::c_int as
                    usize].flag[i as
                    usize]
                    +
                    (*(*em1).ecmm[0 as libc::c_int as
                        usize].cmm.cf_ptr).element_num;
            (*em1).ecmm[0 as libc::c_int as
                usize].cmm.result_lists_d[0 as libc::c_int as
                usize].score[j as
                usize]
                =
                (*em2).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_d[0 as libc::c_int as
                    usize].score[i
                    as
                    usize];
            (*em1).ecmm[0 as libc::c_int as
                usize].cmm.result_lists_d[0 as libc::c_int as
                usize].pos[j as
                usize]
                =
                (*em2).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_d[0 as libc::c_int as
                    usize].pos[i as
                    usize]
        }
        i += 1
    }
    /* 名詞格フレーム */
    if !(*em2).ecmm[0 as libc::c_int as usize].cmm.cf_ptr.is_null() {
        i = 0 as libc::c_int;
        while i <
            (*(*em2).ecmm[0 as libc::c_int as
                usize].cmm.cf_ptr).element_num {
            j =
                (*(*em1).ecmm[0 as libc::c_int as
                    usize].cmm.cf_ptr).element_num + i;
            if (*em2).ecmm[0 as libc::c_int as
                usize].cmm.result_lists_p[0 as libc::c_int as
                usize].flag[i as
                usize]
                == -(1 as libc::c_int) {
                (*em1).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_p[0 as libc::c_int as
                    usize].flag[j as
                    usize]
                    = -(1 as libc::c_int)
            } else {
                (*em1).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_p[0 as libc::c_int as
                    usize].flag[j as
                    usize]
                    =
                    (*em2).ecmm[0 as libc::c_int as
                        usize].cmm.result_lists_p[0 as libc::c_int
                        as
                        usize].flag[i
                        as
                        usize]
                        + (*em1).ecmm[0 as libc::c_int as usize].element_num;
                (*em1).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_p[0 as libc::c_int as
                    usize].score[j
                    as
                    usize]
                    =
                    (*em2).ecmm[0 as libc::c_int as
                        usize].cmm.result_lists_p[0 as libc::c_int
                        as
                        usize].score[i
                        as
                        usize];
                (*em1).ecmm[0 as libc::c_int as
                    usize].cmm.result_lists_p[0 as libc::c_int as
                    usize].pos[j as
                    usize]
                    =
                    (*em2).ecmm[0 as libc::c_int as
                        usize].cmm.result_lists_p[0 as libc::c_int
                        as
                        usize].pos[i
                        as
                        usize]
            }
            i += 1
        }
        merge_cf_ptr((*em1).ecmm[0 as libc::c_int as usize].cmm.cf_ptr,
                     (*em2).ecmm[0 as libc::c_int as usize].cmm.cf_ptr);
    }
    (*em1).ecmm[0 as libc::c_int as usize].element_num +=
        (*em2).ecmm[0 as libc::c_int as usize].element_num;
    append_feature(&mut (*em1).f, (*em2).f);
    (*em2).f = 0 as FEATUREptr;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn demonstrative2coreference(mut sp: *mut SENTENCE_DATA,
                                                   mut cpm_ptr:
                                                   *mut CF_PRED_MGR)
/*==================================================================*/
{
    /* 用言についている<照応仮決定...>を指示詞のタグに移動する */
    let mut num: libc::c_int = 0;
    let mut feature_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut target: [libc::c_char; 5120] = [0; 5120];
    let mut rel: [libc::c_char; 5120] = [0; 5120];
    let mut rest_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut fpp: *mut *mut FEATURE = &mut (*(*cpm_ptr).pred_b_ptr).f;
    let mut pre_fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut next: *mut FEATURE = 0 as *mut FEATURE;
    while !(*fpp).is_null() {
        if strncmp((**fpp).cp,
                   b"\xe7\x85\xa7\xe5\xbf\x9c\xe4\xbb\xae\xe6\xb1\xba\xe5\xae\x9a\x00"
                       as *const u8 as *const libc::c_char,
                   strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe4\xbb\xae\xe6\xb1\xba\xe5\xae\x9a\x00"
                       as *const u8 as *const libc::c_char)) == 0 {
            sscanf((**fpp).cp.offset(11 as libc::c_int as isize),
                   b"%d;C\xe7\x94\xa8;%[^;];%[^;];%s\x00" as *const u8 as
                       *const libc::c_char, &mut num as *mut libc::c_int,
                   target.as_mut_ptr(), rel.as_mut_ptr(),
                   rest_buffer.as_mut_ptr());
            sprintf(feature_buffer.as_mut_ptr(),
                    b"C\xe7\x94\xa8;%s;=;%s\x00" as *const u8 as
                        *const libc::c_char, target.as_mut_ptr(),
                    rest_buffer.as_mut_ptr());
            assign_cfeature(&mut (*(*sp).tag_data.offset(num as isize)).f,
                            feature_buffer.as_mut_ptr(), 0 as libc::c_int);
            free((**fpp).cp as *mut libc::c_void);
            if pre_fp.is_null() {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                *fpp = next
            } else {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                (*pre_fp).next = next;
                fpp = &mut next
            }
        } else {
            pre_fp = *fpp;
            fpp = &mut (**fpp).next
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn DiscourseAnalysis(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int =
        0; /* 表層に出現している要素を参照回数DBに登録 */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut score: libc::c_float = 0.;
    let mut workem: ELLIPSIS_MGR =
        ELLIPSIS_MGR {
            score: 0.,
            pure_score: 0.,
            cc:
            [ELLIPSIS_COMPONENT {
                s: 0 as *mut SENTENCE_DATA,
                pp_str: 0 as *mut libc::c_char,
                bnst: 0,
                score: 0.,
                dist: 0,
                next:
                0 as
                    *mut ellipsis_component,
            };
                50],
            f: 0 as *mut _FEATURE,
            result_num: 0,
            ecmm:
            [ELLIPSIS_CMM {
                cmm:
                CF_MATCH_MGR {
                    cf_ptr:
                    0 as
                        *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    result_lists_d:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    cpm:
                    0 as
                        *mut cpm_def,
                },
                cpm:
                CF_PRED_MGR {
                    cf:
                    CASE_FRAME {
                        type_0:
                        0,
                        type_flag:
                        0,
                        element_num:
                        0,
                        oblig:
                        [0;
                            24],
                        adjacent:
                        [0;
                            24],
                        pp:
                        [[0;
                            10];
                            24],
                        sp:
                        [0;
                            24],
                        pp_str:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete_size:
                        [0;
                            24],
                        sm_delete_num:
                        [0;
                            24],
                        sm_specify:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_specify_size:
                        [0;
                            24],
                        sm_specify_num:
                        [0;
                            24],
                        ex:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        ex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        ex_freq:
                        [0
                            as
                            *mut libc::c_int;
                            24],
                        ex_size:
                        [0;
                            24],
                        ex_num:
                        [0;
                            24],
                        freq:
                        [0;
                            24],
                        semantics:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        gex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        gex_freq:
                        [0
                            as
                            *mut libc::c_double;
                            24],
                        gex_size:
                        [0;
                            24],
                        gex_num:
                        [0;
                            24],
                        voice:
                        0,
                        cf_address:
                        0,
                        cf_size:
                        0,
                        cf_id:
                        [0;
                            280],
                        pred_type:
                        [0;
                            4],
                        entry:
                        0 as
                            *mut libc::c_char,
                        imi:
                        [0;
                            128],
                        etcflag:
                        0,
                        feature:
                        0 as
                            *mut libc::c_char,
                        weight:
                        [0;
                            24],
                        samecase:
                        [[0;
                            2];
                            24],
                        cf_align:
                        [CF_ALIGNMENT {
                            cf_id:
                            0
                                as
                                *mut libc::c_char,
                            aligned_case:
                            [[0;
                                2];
                                24],
                        };
                            5],
                        pred_b_ptr:
                        0 as
                            *mut TAG_DATA,
                        cf_similarity:
                        0.,
                    },
                    pred_b_ptr:
                    0 as *mut TAG_DATA,
                    elem_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    para_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    elem_s_ptr:
                    [0 as
                        *mut sentence;
                        24],
                    elem_b_num: [0; 24],
                    score: 0.,
                    result_num: 0,
                    tie_num: 0,
                    cmm:
                    [CF_MATCH_MGR {
                        cf_ptr:
                        0
                            as
                            *mut CASE_FRAME,
                        score:
                        0.,
                        pure_score:
                        [0.;
                            10],
                        sufficiency:
                        0.,
                        result_num:
                        0,
                        result_lists_p:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        result_lists_d:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        cpm:
                        0
                            as
                            *mut cpm_def,
                    };
                        5],
                    decided: 0,
                },
                element_num: 0,
            }; 5],
        };
    let mut maxem: ELLIPSIS_MGR =
        ELLIPSIS_MGR {
            score: 0.,
            pure_score: 0.,
            cc:
            [ELLIPSIS_COMPONENT {
                s: 0 as *mut SENTENCE_DATA,
                pp_str: 0 as *mut libc::c_char,
                bnst: 0,
                score: 0.,
                dist: 0,
                next:
                0 as
                    *mut ellipsis_component,
            };
                50],
            f: 0 as *mut _FEATURE,
            result_num: 0,
            ecmm:
            [ELLIPSIS_CMM {
                cmm:
                CF_MATCH_MGR {
                    cf_ptr:
                    0 as
                        *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    result_lists_d:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    cpm:
                    0 as
                        *mut cpm_def,
                },
                cpm:
                CF_PRED_MGR {
                    cf:
                    CASE_FRAME {
                        type_0:
                        0,
                        type_flag:
                        0,
                        element_num:
                        0,
                        oblig:
                        [0;
                            24],
                        adjacent:
                        [0;
                            24],
                        pp:
                        [[0;
                            10];
                            24],
                        sp:
                        [0;
                            24],
                        pp_str:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete_size:
                        [0;
                            24],
                        sm_delete_num:
                        [0;
                            24],
                        sm_specify:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_specify_size:
                        [0;
                            24],
                        sm_specify_num:
                        [0;
                            24],
                        ex:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        ex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        ex_freq:
                        [0
                            as
                            *mut libc::c_int;
                            24],
                        ex_size:
                        [0;
                            24],
                        ex_num:
                        [0;
                            24],
                        freq:
                        [0;
                            24],
                        semantics:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        gex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        gex_freq:
                        [0
                            as
                            *mut libc::c_double;
                            24],
                        gex_size:
                        [0;
                            24],
                        gex_num:
                        [0;
                            24],
                        voice:
                        0,
                        cf_address:
                        0,
                        cf_size:
                        0,
                        cf_id:
                        [0;
                            280],
                        pred_type:
                        [0;
                            4],
                        entry:
                        0 as
                            *mut libc::c_char,
                        imi:
                        [0;
                            128],
                        etcflag:
                        0,
                        feature:
                        0 as
                            *mut libc::c_char,
                        weight:
                        [0;
                            24],
                        samecase:
                        [[0;
                            2];
                            24],
                        cf_align:
                        [CF_ALIGNMENT {
                            cf_id:
                            0
                                as
                                *mut libc::c_char,
                            aligned_case:
                            [[0;
                                2];
                                24],
                        };
                            5],
                        pred_b_ptr:
                        0 as
                            *mut TAG_DATA,
                        cf_similarity:
                        0.,
                    },
                    pred_b_ptr:
                    0 as *mut TAG_DATA,
                    elem_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    para_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    elem_s_ptr:
                    [0 as
                        *mut sentence;
                        24],
                    elem_b_num: [0; 24],
                    score: 0.,
                    result_num: 0,
                    tie_num: 0,
                    cmm:
                    [CF_MATCH_MGR {
                        cf_ptr:
                        0
                            as
                            *mut CASE_FRAME,
                        score:
                        0.,
                        pure_score:
                        [0.;
                            10],
                        sufficiency:
                        0.,
                        result_num:
                        0,
                        result_lists_p:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        result_lists_d:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        cpm:
                        0
                            as
                            *mut cpm_def,
                    };
                        5],
                    decided: 0,
                },
                element_num: 0,
            }; 5],
        };
    let mut maxem_noun: ELLIPSIS_MGR =
        ELLIPSIS_MGR {
            score: 0.,
            pure_score: 0.,
            cc:
            [ELLIPSIS_COMPONENT {
                s: 0 as *mut SENTENCE_DATA,
                pp_str: 0 as *mut libc::c_char,
                bnst: 0,
                score: 0.,
                dist: 0,
                next:
                0 as
                    *mut ellipsis_component,
            };
                50],
            f: 0 as *mut _FEATURE,
            result_num: 0,
            ecmm:
            [ELLIPSIS_CMM {
                cmm:
                CF_MATCH_MGR {
                    cf_ptr:
                    0 as
                        *mut CASE_FRAME,
                    score: 0.,
                    pure_score: [0.; 10],
                    sufficiency: 0.,
                    result_num: 0,
                    result_lists_p:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    result_lists_d:
                    [LIST {
                        flag:
                        [0; 24],
                        score:
                        [0.;
                            24],
                        pos:
                        [0;
                            24],
                    };
                        10],
                    cpm:
                    0 as
                        *mut cpm_def,
                },
                cpm:
                CF_PRED_MGR {
                    cf:
                    CASE_FRAME {
                        type_0:
                        0,
                        type_flag:
                        0,
                        element_num:
                        0,
                        oblig:
                        [0;
                            24],
                        adjacent:
                        [0;
                            24],
                        pp:
                        [[0;
                            10];
                            24],
                        sp:
                        [0;
                            24],
                        pp_str:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_delete_size:
                        [0;
                            24],
                        sm_delete_num:
                        [0;
                            24],
                        sm_specify:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        sm_specify_size:
                        [0;
                            24],
                        sm_specify_num:
                        [0;
                            24],
                        ex:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        ex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        ex_freq:
                        [0
                            as
                            *mut libc::c_int;
                            24],
                        ex_size:
                        [0;
                            24],
                        ex_num:
                        [0;
                            24],
                        freq:
                        [0;
                            24],
                        semantics:
                        [0
                            as
                            *mut libc::c_char;
                            24],
                        gex_list:
                        [0
                            as
                            *mut *mut libc::c_char;
                            24],
                        gex_freq:
                        [0
                            as
                            *mut libc::c_double;
                            24],
                        gex_size:
                        [0;
                            24],
                        gex_num:
                        [0;
                            24],
                        voice:
                        0,
                        cf_address:
                        0,
                        cf_size:
                        0,
                        cf_id:
                        [0;
                            280],
                        pred_type:
                        [0;
                            4],
                        entry:
                        0 as
                            *mut libc::c_char,
                        imi:
                        [0;
                            128],
                        etcflag:
                        0,
                        feature:
                        0 as
                            *mut libc::c_char,
                        weight:
                        [0;
                            24],
                        samecase:
                        [[0;
                            2];
                            24],
                        cf_align:
                        [CF_ALIGNMENT {
                            cf_id:
                            0
                                as
                                *mut libc::c_char,
                            aligned_case:
                            [[0;
                                2];
                                24],
                        };
                            5],
                        pred_b_ptr:
                        0 as
                            *mut TAG_DATA,
                        cf_similarity:
                        0.,
                    },
                    pred_b_ptr:
                    0 as *mut TAG_DATA,
                    elem_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    para_b_ptr:
                    [0 as
                        *mut TAG_DATA;
                        24],
                    elem_s_ptr:
                    [0 as
                        *mut sentence;
                        24],
                    elem_b_num: [0; 24],
                    score: 0.,
                    result_num: 0,
                    tie_num: 0,
                    cmm:
                    [CF_MATCH_MGR {
                        cf_ptr:
                        0
                            as
                            *mut CASE_FRAME,
                        score:
                        0.,
                        pure_score:
                        [0.;
                            10],
                        sufficiency:
                        0.,
                        result_num:
                        0,
                        result_lists_p:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        result_lists_d:
                        [LIST {
                            flag:
                            [0;
                                24],
                            score:
                            [0.;
                                24],
                            pos:
                            [0;
                                24],
                        };
                            10],
                        cpm:
                        0
                            as
                            *mut cpm_def,
                    };
                        5],
                    decided: 0,
                },
                element_num: 0,
            }; 5],
        };
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut cmm_ptr: *mut CF_MATCH_MGR = 0 as *mut CF_MATCH_MGR;
    let mut cf_ptr: *mut CASE_FRAME = 0 as *mut CASE_FRAME;
    let mut sp_new: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    sp_new =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    InitEllipsisMGR(&mut workem);
    InitEllipsisMGR(&mut maxem);
    InitEllipsisMGR(&mut maxem_noun);
    AssignFeaturesByProgram(sp);
    RegisterAllSurfaceEntity(sp);
    if (*sp).available != 0 {
        Bcheck =
            malloc_data((::std::mem::size_of::<*mut libc::c_int>() as
                libc::c_ulong).wrapping_mul((*sp).Sen_num as
                libc::c_ulong),
                        b"DiscourseAnalysis\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < (*sp).Sen_num {
            let ref mut fresh53 = *Bcheck.offset(i as isize);
            *fresh53 =
                malloc_data((::std::mem::size_of::<libc::c_int>() as
                    libc::c_ulong).wrapping_mul(200 as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"DiscourseAnalysis\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_int;
            i += 1
        }
        /* 各用言をチェック (文頭から) */
        j = (*(*sp).Best_mgr).pred_num - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            cpm_ptr =
                &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(j as isize) as
                    *mut CF_PRED_MGR;
            /* 格フレームがない場合 (ガ格ぐらい探してもいいかもしれない)
	       格解析が失敗した場合 */
            if !((*cpm_ptr).result_num == 0 as libc::c_int ||
                (*(*cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).cf_address ==
                    -(1 as libc::c_int) as libc::c_ulonglong ||
                (*cpm_ptr).cmm[0 as libc::c_int as usize].score <
                    0 as libc::c_int as libc::c_double) {
                /* 省略解析しない用言 */
                if check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                 b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
                    /* 固有名詞は省略解析しない (用言に対して) */
                    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
                        (!check_feature((*(*(*cpm_ptr).pred_b_ptr).b_ptr).f,
                                        b"\xe4\xba\xba\xe5\x90\x8d\x00" as
                                            *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char).is_null() ||
                            !check_feature((*(*(*cpm_ptr).pred_b_ptr).b_ptr).f,
                                           b"\xe5\x9c\xb0\xe5\x90\x8d\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null()
                            ||
                            !check_feature((*(*(*cpm_ptr).pred_b_ptr).b_ptr).f,
                                           b"\xe7\xb5\x84\xe7\xb9\x94\xe5\x90\x8d\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null())
                    {
                        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                    } else if (*cpm_ptr).cf.type_0 == 1 as libc::c_int &&
                        (!check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                        b"NE\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char).is_null()
                            ||
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           b"NE\xe5\x86\x85\x00" as
                                               *const u8 as
                                               *const libc::c_char
                                               as
                                               *mut libc::c_char).is_null())
                    {
                        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                    } else if (*cpm_ptr).cf.type_0 == 2 as libc::c_int &&
                        (!check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                        b"NE\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char).is_null()
                            ||
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           b"NE\xe5\x86\x85\x00" as
                                               *const u8 as
                                               *const libc::c_char
                                               as
                                               *mut libc::c_char).is_null())
                    {
                        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                    } else if (*cpm_ptr).cf.type_0 == 2 as libc::c_int &&
                        (!check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                        b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char).is_null()
                            ||
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\xe5\x86\x85\x00"
                                               as *const u8 as
                                               *const libc::c_char
                                               as
                                               *mut libc::c_char).is_null()
                            ||
                            check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                          b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                              as *const u8 as
                                              *const libc::c_char
                                              as
                                              *mut libc::c_char).is_null()
                            ||
                            !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                                           b"\xe4\xb8\x80\xe4\xba\xba\xe7\xa7\xb0\x00"
                                               as *const u8 as
                                               *const libc::c_char
                                               as
                                               *mut libc::c_char).is_null())
                    {
                        assign_cfeature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                    } else {
                        mark_location_classes(sp, (*cpm_ptr).pred_b_ptr);
                        cmm_ptr =
                            &mut *(*cpm_ptr).cmm.as_mut_ptr().offset(0 as
                                libc::c_int
                                as
                                isize)
                                as *mut CF_MATCH_MGR;
                        cf_ptr = (*cmm_ptr).cf_ptr;
                        /* 固有名詞は間接照応解析しない */
                        /* 共参照解析結果のある語、照応詞候補でない語は解析しない */
                        /* もっともスコアがよくなる順番で省略の指示対象を決定する */
                        maxem.score = -(2 as libc::c_int) as libc::c_float;
                        if (*cpm_ptr).cf.type_0 == 2 as libc::c_int {
                            FindBestCFforContext(sp, &mut maxem, cpm_ptr,
                                                 0 as *mut *mut libc::c_char);
                        } else {
                            i = 0 as libc::c_int;
                            while i < 3 as libc::c_int {
                                if (*cpm_ptr).decided == 2 as libc::c_int {
                                    /* 入力側格要素を設定
			   照応解析時はすでにある格要素を上書きしてしまうのでここで再設定
			   それ以外のときは下の DeleteFromCF() で省略要素をクリア */
                                    if OptEllipsis & 2 as libc::c_int != 0 {
                                        make_data_cframe(sp, cpm_ptr);
                                    }
                                    ClearEllipsisMGR(&mut workem);
                                    score =
                                        EllipsisDetectForVerbMain(sp,
                                                                  &mut workem,
                                                                  cpm_ptr,
                                                                  &mut *(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                                                  0 as
                                                                      libc::c_int,
                                                                  (*cpm_ptr).cmm[0
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      usize].cf_ptr,
                                                                  CaseOrder[i
                                                                      as
                                                                      usize].as_mut_ptr());
                                    /* 直接の格要素の正規化していないスコアを足す */
                                    workem.score =
                                        (workem.score as libc::c_double +
                                            (*cpm_ptr).cmm[0 as libc::c_int
                                                as
                                                usize].pure_score[0
                                                as
                                                libc::c_int
                                                as
                                                usize])
                                            as libc::c_float;
                                    workem.pure_score += workem.score;
                                    workem.score =
                                        (workem.score as libc::c_double /
                                            sqrt(count_pat_element((*cpm_ptr).cmm[0
                                                as
                                                libc::c_int
                                                as
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
                                                as libc::c_double)) as
                                            libc::c_float;
                                    if workem.score > maxem.score {
                                        maxem = workem;
                                        maxem.result_num =
                                            (*cpm_ptr).result_num;
                                        k = 0 as libc::c_int;
                                        while k < maxem.result_num {
                                            maxem.ecmm[k as usize].cmm =
                                                (*cpm_ptr).cmm[k as usize];
                                            maxem.ecmm[k as usize].cpm =
                                                *cpm_ptr;
                                            maxem.ecmm[k as usize].element_num
                                                = (*cpm_ptr).cf.element_num;
                                            k += 1
                                        }
                                        workem.f = 0 as FEATUREptr
                                    }
                                    /* 格フレームの追加エントリの削除 */
                                    if OptEllipsis & 2 as libc::c_int == 0 {
                                        DeleteFromCF(&mut workem, cpm_ptr,
                                                     &mut *(*cpm_ptr).cmm.as_mut_ptr().offset(0
                                                         as
                                                         libc::c_int
                                                         as
                                                         isize),
                                                     0 as libc::c_int);
                                    }
                                } else {
                                    /* 格フレーム未決定のとき */
                                    FindBestCFforContext(sp, &mut maxem,
                                                         cpm_ptr,
                                                         CaseOrder[i as
                                                             usize].as_mut_ptr());
                                }
                                i += 1
                            }
                            if OptMergeCFResult != 0 &&
                                (*cpm_ptr).cf.type_flag != 0 &&
                                OptEllipsis & 4 as libc::c_int != 0 {
                                (*cpm_ptr).cf.type_0 = 2 as libc::c_int;
                                maxem_noun.score =
                                    -(2 as libc::c_int) as libc::c_float;
                                FindBestCFforContext(sp, &mut maxem_noun,
                                                     cpm_ptr,
                                                     0 as
                                                         *mut *mut libc::c_char);
                                if maxem_noun.score >
                                    -(2 as libc::c_int) as libc::c_float {
                                    merge_em(&mut maxem, &mut maxem_noun);
                                }
                                (*cpm_ptr).cf.type_0 = 1 as libc::c_int
                            }
                        }
                        /* もっとも score のよかった組み合わせを登録 */
                        if maxem.score > -(2 as libc::c_int) as libc::c_float
                        {
                            (*cpm_ptr).score = maxem.score as libc::c_double;
                            maxem.ecmm[0 as libc::c_int as usize].cmm.score =
                                maxem.score as libc::c_double;
                            maxem.ecmm[0 as libc::c_int as
                                usize].cmm.pure_score[0 as
                                libc::c_int
                                as usize]
                                = maxem.pure_score as libc::c_double;
                            /* cmm を復元 */
                            (*cpm_ptr).result_num = maxem.result_num;
                            k = 0 as libc::c_int;
                            while k < (*cpm_ptr).result_num {
                                (*cpm_ptr).cmm[k as usize] =
                                    maxem.ecmm[k as usize].cmm;
                                (*cpm_ptr).cmm[k as usize].cpm =
                                    malloc_data(::std::mem::size_of::<CF_PRED_MGR>()
                                                    as libc::c_ulong,
                                                b"DiscourseAnalysis\x00" as
                                                    *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char) as
                                        *mut CF_PRED_MGR;
                                *(*cpm_ptr).cmm[k as usize].cpm =
                                    maxem.ecmm[k as usize].cpm;
                                k += 1
                            }
                            (*cpm_ptr).cf.element_num =
                                maxem.ecmm[0 as libc::c_int as
                                    usize].element_num;
                            k = 0 as libc::c_int;
                            while k <
                                maxem.ecmm[0 as libc::c_int as
                                    usize].element_num {
                                (*cpm_ptr).elem_b_ptr[k as usize] =
                                    maxem.ecmm[0 as libc::c_int as
                                        usize].cpm.elem_b_ptr[k as
                                        usize];
                                (*cpm_ptr).elem_b_num[k as usize] =
                                    maxem.ecmm[0 as libc::c_int as
                                        usize].cpm.elem_b_num[k as
                                        usize];
                                (*cpm_ptr).elem_s_ptr[k as usize] =
                                    maxem.ecmm[0 as libc::c_int as
                                        usize].cpm.elem_s_ptr[k as
                                        usize];
                                l = 0 as libc::c_int;
                                while l < 10 as libc::c_int {
                                    (*cpm_ptr).cf.pp[k as usize][l as usize] =
                                        maxem.ecmm[0 as libc::c_int as
                                            usize].cpm.cf.pp[k as
                                            usize][l
                                            as
                                            usize];
                                    l += 1
                                }
                                strcpy((*cpm_ptr).cf.sm[k as usize],
                                       maxem.ecmm[0 as libc::c_int as
                                           usize].cpm.cf.sm[k as
                                           usize]);
                                k += 1
                            }
                            /* feature の伝搬 */
                            append_feature(&mut (*(*cpm_ptr).pred_b_ptr).f,
                                           maxem.f);
                            maxem.f = 0 as FEATUREptr;
                            /* 文脈解析において格フレームを決定した場合 */
                            if (*cpm_ptr).decided != 2 as libc::c_int {
                                case_analysis::assign_nil_assigned_components(sp, cpm_ptr);
                                if OptCaseFlag & 2 as libc::c_int != 0 {
                                    assign_ga_subject(sp_new, cpm_ptr);
                                    /* CF_CAND_DECIDED の場合は行っているが */
                                }
                                if OptUseSmfix ==
                                    (0 as libc::c_int == 0) as libc::c_int
                                {
                                    specify_sm_from_cf(sp_new, cpm_ptr);
                                }
                            }
                            /* マッチした用例をfeatureに出力 *
		record_match_ex(sp, cpm_ptr); */
                            /* 直前格のマッチスコアをfeatureに出力 *
		   record_closest_cc_match(sp, cpm_ptr); */
                            /* 省略解析の結果を参照回数DBに登録 */
                            RegisterEllipsisEntity(sp, cpm_ptr, &mut maxem);
                            /* 指示詞の解析結果を=のタグに変換する */
                            if OptEllipsis & 2 as libc::c_int != 0 &&
                                OptEllipsis & 8 as libc::c_int != 0 {
                                demonstrative2coreference(sp, cpm_ptr);
                            }
                            /* 格・省略解析の結果をfeatureへ */
                            record_case_analysis(sp, cpm_ptr, &mut maxem,
                                                 0 as libc::c_int);
                            /* 格・省略解析の結果を用いて形態素曖昧性を解消 */
                            verb_lexical_disambiguation_by_case_analysis(cpm_ptr);
                            noun_lexical_disambiguation_by_case_analysis(cpm_ptr);
                        }
                        ClearEllipsisMGR(&mut maxem);
                        ClearEllipsisMGR(&mut maxem_noun);
                        /* 格フレームの保存 */
                        if (*cpm_ptr).cmm[0 as libc::c_int as usize].score >
                            0 as libc::c_int as libc::c_double {
                            RegisterCF((*(*cpm_ptr).cmm[0 as libc::c_int as
                                usize].cf_ptr).cf_id.as_mut_ptr());
                        }
                        i = 0 as libc::c_int;
                        while i < (*sp).Sen_num {
                            free(*LC.offset(i as isize) as *mut libc::c_void);
                            i += 1
                        }
                        free(LC as *mut libc::c_void);
                    }
                }
            }
            j -= 1
        }
        PreserveCPM(sp_new, sp);
        /* 出現回数、参照回数を減衰させる */
        DecayEntityList();
        i = 0 as libc::c_int;
        while i < (*sp).Sen_num {
            free(*Bcheck.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(Bcheck as *mut libc::c_void);
    }
    /* 共参照解析(橋渡し指示解析の結果を使用) */
    if OptEllipsis & 8 as libc::c_int != 0 { corefer::corefer_analysis_after_br(sp); }
    clear_cf(0 as libc::c_int);
}
