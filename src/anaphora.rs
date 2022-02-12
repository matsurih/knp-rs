#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{_FEATURE, atoi, BNST_DATA, Class, entity, FEATURE, fprintf, free, memset, mention, MENTION_MGR, MRPH_DATA, printf, sprintf, sscanf, strcat, strchr, strcmp, strcpy, strlen, strncmp, strstr, tnode_b};
use crate::case_analysis::{clear_case_frame, init_case_frame, make_print_string, MatchPP, pp_code_to_kstr, pp_kstr_to_code};
use crate::case_data::make_data_cframe;
use crate::case_ipal::{_get_ex_probability_internal, CFSimExist, get_case_function_probability_for_pred, get_case_probability, get_cf_probability_for_pred, get_cfs_similarity, get_class_probability, get_ex_ne_probability, get_ex_probability, get_ex_probability_with_para, get_general_probability, get_key_probability, make_pred_string, make_pred_string_from_mrph};
use crate::consts::{CF_ELEMENT_MAX, CF_NOUN, CF_PRED, DATA_LEN, ELLIPSIS_CASE_NUM, ENTITY_MAX, MENTION_MAX, MENTIONED_MAX, O_FEATURE_NUM, OPT_ALL_CASE, OPT_CASE_ANALYSIS, OPT_COREFER_AUTO, OPT_ELLIPSIS, OPT_GS, OPT_TRAIN, OPT_UNNAMED_ENTITY, PARA_NORMAL, REPNAME_LEN_MAX, SMALL_DATA_LEN};
use crate::context::{CFSimThreshold, CheckCF, ClearSentence, get_pred_id, OptUseSmfix};
use crate::corefer::corefer_id;
use crate::ctools::{assign_cfeature, check_feature, entity_manager, log, malloc, malloc_data, memcpy, pow, sentence_data, stderr, strncat, strtok};
use crate::db::db_get;
use crate::feature::{check_str_type, delete_cfeature};
use crate::read_data::get_bnst_head_canonical_rep;
use crate::structs::CF_ALIGNMENT;
use crate::tools::{analysis_flags, author_score, author_sen, author_tag, base_entity_num, base_sentence_num, event_db, learned_all_arguments_weight, learned_case_feature_weight, learned_overt_arguments_weight, loc_category, OptAnaphora, OptCaseFlag, OptDisplay, OptEllipsis, OptExpress, OptGeneralCF, OptReadFeature, reader_score, reader_sen, reader_tag};
use crate::types::{CASE_FRAME, CF_PRED_MGR, CF_TAG_MGR, CFLIST, ENTITY, MENTION, SENTENCE_DATA, TAG_CASE_FRAME, TAG_DATA};

/* 省略解析に関するパラメータ */
//const CASE_CANDIDATE_MAX  10 /* 照応解析用格解析結果を保持する数 */
const CASE_CANDIDATE_MAX: libc::c_int = 5; /* 照応解析用格解析結果を保持する数 */

const CASE_CAND_DIF_MAX: libc::c_double = 4.6 as libc::c_double; /* 格解析の候補として考慮するスコアの差の最大値(log(20)) */
//const ELLIPSIS_RESULT_MAX 100  /* 省略解析結果を保持する */
const ELLIPSIS_RESULT_MAX: libc::c_int = 10;  /* 省略解析結果を保持する */
const ELLIPSIS_CORRECT_MAX: libc::c_int = 3;  /* 省略解析結果のうち正解のものを保持する */
const SALIENCE_DECAY_RATE: libc::c_double = 0.5 as libc::c_double; /* salience_scoreの減衰率 */
const SALIENCE_THRESHOLD: libc::c_int = 0; /* 解析対象とするsalience_scoreの閾値(=は含まない) */
const INITIAL_SCORE: libc::c_int = -10000;

/* 文の出現要素に与えるsalience_score */
const SALIENCE_THEMA: libc::c_double = 2.0 as libc::c_double; /* 重要な要素(未格,文末)に与える */
const SALIENCE_CANDIDATE: libc::c_double = 1.0 as libc::c_double; /* 先行詞候補とする要素(ガ格,ヲ格など)に与える */
const SALIENCE_NORMAL: libc::c_double = 0.4 as libc::c_double; /* 上記以外の要素に与える */
const SALIENCE_ZERO: libc::c_double = 1.0 as libc::c_double; /* ゼロ代名詞に与える */
const SALIENCE_ASSO: libc::c_double = 0.0 as libc::c_double; /* 連想照応の先行詞に与える */


const MODALITY_NUM: libc::c_int = 11;
const VOICE_NUM: libc::c_int = 5;
const KEIGO_NUM: libc::c_int = 3;

/* 位置カテゴリ(主節や用言であるか等は無視)    */
const LOC_SELF: libc::c_int = 0; /* 自分自身     */
const LOC_PARENT: libc::c_int = 1; /* 親           */
const LOC_CHILD: libc::c_int = 2; /* 子供         */
const LOC_PARA_PARENT: libc::c_int = 3; /* 並列(親側)   */
const LOC_PARA_CHILD: libc::c_int = 4; /* 並列(子側)   */
const LOC_PARENT_N_PARENT: libc::c_int = 5; /* 親体言の親   */
const LOC_PARENT_V_PARENT: libc::c_int = 6; /* 親用言の親   */
const LOC_OTHERS_BEFORE: libc::c_int = 7; /* その他(前)   */
const LOC_OTHERS_AFTER: libc::c_int = 8; /* その他(後)   */
const LOC_OTHERS_THEME: libc::c_int = 9; /* その他(主題) */

const AUTHOR_REP_NUM: libc::c_int = 7;
const READER_REP_NUM: libc::c_int = 5;

const UNNAMED_ENTITY_NUM: libc::c_int = 5;
const UNNAMED_ENTITY_NAME_NUM: libc::c_int = 2;
const UNNAMED_ENTITY_CATEGORY_NUM: libc::c_int = 21;
const UNNAMED_ENTITY_NE_NUM: libc::c_int = 7;
const UNNAMED_ENTITY_REP_NUM: libc::c_int = 6;
const CATEGORY_NUM: libc::c_int = 21;

#[no_mangle]
pub static mut case_candidate_ctm: [CF_TAG_MGR; 5] =
    [CF_TAG_MGR {
        score: 0.,
        score_def: 0.,
        case_analysis_score: 0.,
        cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
        filled_element: [0; 24],
        filled_entity: [0; 4096],
        non_match_element: [0; 24],
        result_num: 0,
        case_result_num: 0,
        annotated_result_num: 0,
        cf_element_num: [0; 24],
        tcf_element_num: [0; 24],
        tcf_element_num_functional: [0; 24],
        elem_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
        entity_num: [0; 24],
        type_0: [0; 24],
        ga_entity: 0,
        case_analysis_ga_entity: 0,
        overt_arguments_score: 0.,
        all_arguments_score: 0.,
        omit_feature: [[0.; 9152]; 4],
    }; 5];
#[no_mangle]
pub static mut ellipsis_result_ctm: [CF_TAG_MGR; 10] =
    [CF_TAG_MGR {
        score: 0.,
        score_def: 0.,
        case_analysis_score: 0.,
        cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
        filled_element: [0; 24],
        filled_entity: [0; 4096],
        non_match_element: [0; 24],
        result_num: 0,
        case_result_num: 0,
        annotated_result_num: 0,
        cf_element_num: [0; 24],
        tcf_element_num: [0; 24],
        tcf_element_num_functional: [0; 24],
        elem_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
        entity_num: [0; 24],
        type_0: [0; 24],
        ga_entity: 0,
        case_analysis_ga_entity: 0,
        overt_arguments_score: 0.,
        all_arguments_score: 0.,
        omit_feature: [[0.; 9152]; 4],
    }; 10];
#[no_mangle]
pub static mut ellipsis_correct_ctm: [CF_TAG_MGR; 3] =
    [CF_TAG_MGR {
        score: 0.,
        score_def: 0.,
        case_analysis_score: 0.,
        cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
        filled_element: [0; 24],
        filled_entity: [0; 4096],
        non_match_element: [0; 24],
        result_num: 0,
        case_result_num: 0,
        annotated_result_num: 0,
        cf_element_num: [0; 24],
        tcf_element_num: [0; 24],
        tcf_element_num_functional: [0; 24],
        elem_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
        entity_num: [0; 24],
        type_0: [0; 24],
        ga_entity: 0,
        case_analysis_ga_entity: 0,
        overt_arguments_score: 0.,
        all_arguments_score: 0.,
        omit_feature: [[0.; 9152]; 4],
    }; 3];
#[no_mangle]
pub static mut ELLIPSIS_CASE_LIST_VERB: [*mut libc::c_char; 5] =
    [b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x83\xb2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut ELLIPSIS_CASE_LIST_NOUN: [*mut libc::c_char; 4] =
    [b"\xe3\x83\x8e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x83\x8e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x83\x8e\xef\xbc\x9f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut ELLIPSIS_CASE_LIST: *mut *mut libc::c_char = unsafe { ELLIPSIS_CASE_LIST_VERB.as_ptr() as *mut _ };
#[no_mangle]
pub static mut overt_arguments_weight: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut all_arguments_weight: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut case_feature_weight: [[libc::c_double; 9152]; 4] = [[0.; 9152]; 4];
#[no_mangle]
pub static mut def_overt_arguments_weight: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut def_all_arguments_weight: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut def_case_feature_weight: [[libc::c_double; 9152]; 4] = [[0.; 9152]; 4];
#[no_mangle]
pub static mut ModifyWeight: [libc::c_double; 4] = [1.2f64, 0.8f64, 0.8f64, 0.0f64];
#[no_mangle]
pub static mut OptZeroPronoun: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut unnamed_entity: [*mut libc::c_char; 5] =
    [b"\xe8\x91\x97\xe8\x80\x85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xaa\xad\xe8\x80\x85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut unnamed_entity_name: [[*mut libc::c_char; 2]; 5] =
    [[b"\xe8\x91\x97\xe8\x80\x85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe8\xaa\xad\xe8\x80\x85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe7\x8a\xb6\xe6\xb3\x81\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char, 0 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut unnamed_entity_category: [[*mut libc::c_char; 21]; 5] =
    [[b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe7\xb5\x84\xe7\xb9\x94\xe3\x83\xbb\xe5\x9b\xa3\xe4\xbd\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\xe5\x8b\x95\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\x8b\x95\xe7\x89\xa9-\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe6\xa4\x8d\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe6\xa4\x8d\xe7\x89\xa9-\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe4\xb9\x97\xe3\x82\x8a\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe8\xa1\xa3\xe9\xa1\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe9\xa3\x9f\xe3\x81\xb9\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\xa9\x9f\xe8\x83\xbd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xa0\xb4\xe6\x89\x80-\xe8\x87\xaa\xe7\x84\xb6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\x96\xbd\xe8\xa8\xad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\x96\xbd\xe8\xa8\xad\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xa0\xb4\xe6\x89\x80-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe8\x87\xaa\xe7\x84\xb6\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe6\x8a\xbd\xe8\xb1\xa1\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe8\x89\xb2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut unnamed_entity_ne: [[*mut libc::c_char; 7]; 5] =
    [[b"PERSON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ORGANIZATION\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char],
        [b"PERSON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"PERSON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"ORGANIZATION\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"ARTIFACT\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
            b"DATE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"TIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"PERCENT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"MONEY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"LOCATION\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"TIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"DATE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut unnamed_entity_rep: [[*mut libc::c_char; 6]; 5] =
    [[b"\xe7\xa7\x81/\xe3\x82\x8f\xe3\x81\x9f\xe3\x81\x97\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\x88\x91\xe3\x80\x85/\xe3\x82\x8f\xe3\x82\x8c\xe3\x82\x8f\xe3\x82\x8c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xbf\xba/\xe3\x81\x8a\xe3\x82\x8c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x83\x95/\xe3\x81\xbc\xe3\x81\x8f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe3\x81\x82\xe3\x81\xaa\xe3\x81\x9f/\xe3\x81\x82\xe3\x81\xaa\xe3\x81\x9f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xae\xa2/\xe3\x81\x8d\xe3\x82\x83\xe3\x81\x8f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\x90\x9b/\xe3\x81\x8d\xe3\x81\xbf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe7\x9a\x86\xe6\xa7\x98/\xe3\x81\xbf\xe3\x81\xaa\xe3\x81\x95\xe3\x81\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xba\xba/\xe3\x81\xb2\xe3\x81\xa8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"\xe7\x8a\xb6\xe6\xb3\x81/\xe3\x81\x98\xe3\x82\x87\xe3\x81\x86\xe3\x81\x8d\xe3\x82\x87\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe3\x82\x82\xe3\x81\xae/\xe3\x82\x82\xe3\x81\xae\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char],
        [b"<\xe8\xa3\x9c\xe6\x96\x87>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"<\xe6\x99\x82\xe9\x96\x93>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"<\xe6\x95\xb0\xe9\x87\x8f>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut category_list: [*mut libc::c_char; 21] =
    [b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe7\xb5\x84\xe7\xb9\x94\xe3\x83\xbb\xe5\x9b\xa3\xe4\xbd\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x8b\x95\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x8b\x95\xe7\x89\xa9-\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\xa4\x8d\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\xa4\x8d\xe7\x89\xa9-\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe4\xb9\x97\xe3\x82\x8a\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe9\x87\x91\xe9\x8a\xad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe8\xa1\xa3\xe9\xa1\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe9\xa3\x9f\xe3\x81\xb9\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\xa9\x9f\xe8\x83\xbd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xa0\xb4\xe6\x89\x80-\xe8\x87\xaa\xe7\x84\xb6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\x96\xbd\xe8\xa8\xad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\x96\xbd\xe8\xa8\xad\xe9\x83\xa8\xe4\xbd\x8d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xa0\xb4\xe6\x89\x80-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\x87\xaa\xe7\x84\xb6\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\x8a\xbd\xe8\xb1\xa1\xe7\x89\xa9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\x89\xb2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut alternate_category: [[*mut libc::c_char; 21]; 21] =
    [[b"\xe7\xb5\x84\xe7\xb9\x94\xe3\x83\xbb\xe5\x9b\xa3\xe4\xbd\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char],
        [b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char],
        [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char]];
#[no_mangle]
pub static mut modality: [*mut libc::c_char; 11] =
    [b"\xe6\x84\x8f\xe5\xbf\x97\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x8b\xa7\xe8\xaa\x98\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x91\xbd\xe4\xbb\xa4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe7\xa6\x81\xe6\xad\xa2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xa9\x95\xe4\xbe\xa1:\xe5\xbc\xb1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xa9\x95\xe4\xbe\xa1:\xe5\xbc\xb7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xaa\x8d\xe8\xad\x98-\xe6\x8e\xa8\xe9\x87\x8f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xaa\x8d\xe8\xad\x98-\xe8\x93\x8b\xe7\x84\xb6\xe6\x80\xa7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xaa\x8d\xe8\xad\x98-\xe8\xa8\xbc\xe6\x8b\xa0\xe6\x80\xa7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xbe\x9d\xe9\xa0\xbc\xef\xbc\xa1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xbe\x9d\xe9\xa0\xbc\xef\xbc\xa2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut modality_count: [libc::c_int; 11] = [0; 11];
#[no_mangle]
pub static mut keigo: [*mut libc::c_char; 3] =
    [b"\xe5\xb0\x8a\xe6\x95\xac\xe8\xa1\xa8\xe7\x8f\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xac\x99\xe8\xad\xb2\xe8\xa1\xa8\xe7\x8f\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe4\xb8\x81\xe5\xaf\xa7\xe8\xa1\xa8\xe7\x8f\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut keigo_count: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut yobikake_count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut voice: [*mut libc::c_char; 5] =
    [b"\xe4\xbd\xbf\xe5\xbd\xb9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x8f\x97\xe5\x8b\x95\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x8f\xaf\xe8\x83\xbd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x82\x82\xe3\x82\x89\xe3\x81\x86\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe3\x81\xbb\xe3\x81\x97\xe3\x81\x84\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut tense: [*mut libc::c_char; 4] =
    [b"\xe6\x9c\xaa\xe6\x9d\xa5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe7\x8f\xbe\xe5\x9c\xa8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe9\x81\x8e\xe5\x8e\xbb\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe7\x84\xa1\xe6\x99\x82\xe5\x88\xb6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut ne: [*mut libc::c_char; 8] =
    [b"PERSON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ORIGANIZATION\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ARTIFACT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"DATE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"LOCATION\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"TIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"MONY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PERCENT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut context_feature: [[libc::c_double; 10]; 4096] = [[0.; 10]; 4096];
#[no_mangle]
pub static mut svm_feaature_opt: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut candidate_entities: [libc::c_int; 4096] = [0; 4096];
#[no_mangle]
pub static mut max_reliabirity: libc::c_double = 0.;
#[no_mangle]
pub static mut max_reliabirity_tag_ptr: *mut TAG_DATA = 0 as *const TAG_DATA as *mut TAG_DATA;
#[no_mangle]
pub static mut analysis_flag: libc::c_int = 0;
#[no_mangle]
pub static mut ite_count: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn reset_hypo_information() {
    let mut entity_id: libc::c_int = 0;
    entity_id = 0 as libc::c_int;
    while entity_id < entity_manager.num {
        let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
        entity_ptr = entity_manager.entity.as_mut_ptr().offset(entity_id as isize);
        if OptAnaphora & 2048 as libc::c_int != 0 && (*entity_ptr).hypothetical_entity == 0 as libc::c_int || OptAnaphora & 4096 as libc::c_int != 0 && (*entity_ptr).hypothetical_entity == 1 as libc::c_int {
            if (*entity_ptr).hypothetical_flag != 1 as libc::c_int {
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                (*entity_manager.entity.as_mut_ptr().offset((*entity_ptr).hypothetical_entity as isize)).real_entity = -(1 as libc::c_int);
                (*entity_manager.entity.as_mut_ptr().offset((*entity_ptr).hypothetical_entity as isize)).skip_flag = 0 as libc::c_int;
                (*entity_ptr).hypothetical_entity = -(1 as libc::c_int);
                strcat((*entity_ptr).hypothetical_name.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char);
                cp = strchr((*entity_ptr).name.as_mut_ptr(), '|' as i32);
                if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
            }
        }
        entity_id += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_eid(mut tag_num: libc::c_int, mut sent_num: libc::c_int) -> libc::c_int {
    let mut entity_id: libc::c_int = 0;
    let mut mention_id: libc::c_int = 0;
    entity_id = 0 as libc::c_int;
    while entity_id < entity_manager.num {
        let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
        entity_ptr = entity_manager.entity.as_mut_ptr().offset(entity_id as isize);
        mention_id = 0 as libc::c_int;
        while mention_id < (*entity_ptr).mentioned_num {
            let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
            mention_ptr = (*entity_ptr).mention[mention_id as usize];
            if (*mention_ptr).type_0 as libc::c_int == 'S' as i32 || (*mention_ptr).type_0 as libc::c_int == '=' as i32 {
                if (*mention_ptr).sent_num == sent_num && (*mention_ptr).tag_num == tag_num {
                    return (*entity_ptr).num;
                }
            }
            mention_id += 1
        }
        entity_id += 1
    }
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn convert_case_result(mut buf: *mut libc::c_char, mut feature: *mut libc::c_char, mut sen_num: libc::c_int) {
    let mut pred_name: [libc::c_char; 280] = [0; 280];
    let mut id: [libc::c_char; 280] = [0; 280];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut temp: [libc::c_char; 128] = [0; 128];
    let mut type_0: libc::c_char = 0;
    let mut rel: [libc::c_char; 128] = [0; 128];
    let mut entity_name: [libc::c_char; 256] = [0; 256];
    let mut tag_num_str: [libc::c_char; 128] = [0; 128];
    let mut sent_num_str: [libc::c_char; 128] = [0; 128];
    // let mut tag_num: libc::c_int = 0;
    // let mut sent_num: libc::c_int = 0;
    sscanf(feature,
           b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:%[^:]:%[^:]:\x00" as *const u8 as *const libc::c_char, pred_name.as_mut_ptr(),
           id.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        feature = strchr(feature, ':' as i32).offset(1 as libc::c_int as isize);
        i += 1
    }
    feature = feature.offset(-1);
    sprintf(buf,
            b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0:%s:%s\x00" as *const u8 as *const libc::c_char, pred_name.as_mut_ptr(),
            id.as_mut_ptr());
    if !feature.is_null() {
        cp = feature;
        while *cp != 0 {
            if *cp as libc::c_int == ';' as i32 || *cp as libc::c_int == ':' as i32 {
                if sscanf(cp.offset(1 as libc::c_int as isize),
                          b"%[^/]/%c/%[^/]/%[^/]/%[^/]\x00" as *const u8 as *const libc::c_char, rel.as_mut_ptr(),
                          &mut type_0 as *mut libc::c_char,
                          entity_name.as_mut_ptr(), tag_num_str.as_mut_ptr(),
                          sent_num_str.as_mut_ptr()) != 0 {
                    if strcmp(tag_num_str.as_mut_ptr(), b"-\x00" as *const u8 as *const libc::c_char) != 0 && strcmp(tag_num_str.as_mut_ptr(), b"-\x00" as *const u8 as *const libc::c_char) != 0 {
                        let mut eid: libc::c_int = 0;
                        eid = get_eid(atoi(tag_num_str.as_mut_ptr()), sen_num - atoi(sent_num_str.as_mut_ptr()));
                        sprintf(temp.as_mut_ptr(),
                                b"%c%s/%c/%s/%s/%s/%d\x00" as *const u8 as *const libc::c_char, *cp as libc::c_int,
                                rel.as_mut_ptr(),
                                type_0 as libc::c_int,
                                entity_name.as_mut_ptr(),
                                sent_num_str.as_mut_ptr(),
                                tag_num_str.as_mut_ptr(),
                                eid);
                        strcat(buf, temp.as_mut_ptr());
                    } else {
                        sprintf(temp.as_mut_ptr(),
                                b"%c%s/%c/%s/%s/%s/%c\x00" as *const u8 as *const libc::c_char,
                                *cp as libc::c_int,
                                rel.as_mut_ptr(),
                                type_0 as libc::c_int,
                                entity_name.as_mut_ptr(),
                                sent_num_str.as_mut_ptr(),
                                tag_num_str.as_mut_ptr(),
                                '-' as i32);
                        strcat(buf, temp.as_mut_ptr());
                    }
                }
            }
            cp = cp.offset(1)
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn abbreviate_NE(mut cp: *mut libc::c_char) {
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut temp_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(buf.as_mut_ptr(), cp);
    strcpy(cp, b"\x00" as *const u8 as *const libc::c_char);
    if !strstr(buf.as_mut_ptr(), b"PERCENT\x00" as *const u8 as *const libc::c_char).is_null() {
        strcpy(cp, b"NE:%\x00" as *const u8 as *const libc::c_char);
    } else {
        strcpy(cp, buf.as_mut_ptr());
        *cp.offset(strlen(b"NE:\x00" as *const u8 as *const libc::c_char).wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = '\u{0}' as i32 as libc::c_char
    }
    temp_cp = buf.as_mut_ptr().offset(strlen(b"NE:\x00" as *const u8 as *const libc::c_char) as isize);
    while *temp_cp as libc::c_int != ':' as i32 {
        temp_cp = temp_cp.offset(1)
    }
    strcat(cp, temp_cp);
}

#[no_mangle]
pub unsafe extern "C" fn check_feature_entity(mut entity_ptr: *mut ENTITY, mut feature: *mut libc::c_char) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*entity_ptr).mentioned_num {
        if (*(*entity_ptr).mention[j as usize]).type_0 as libc::c_int == 'S' as i32 || (*(*entity_ptr).mention[j as usize]).type_0 as libc::c_int == '=' as i32 {
            cp = check_feature((*(*(*entity_ptr).mention[j as usize]).tag_ptr).f, feature);
            if !cp.is_null() {
                if !cp.is_null() {
                    return cp;
                }
            }
        }
        j += 1
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn set_mention_from_coreference(mut tag_ptr: *mut TAG_DATA, mut mention_ptr: *mut MENTION) {
    let mut hypo_name: [libc::c_char; 256] = [0; 256];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 5120] = [0; 5120];
    let mut hypo_flag: libc::c_int = 0;
    let mut name_change_flag: libc::c_int = 0;
    let mut parent_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    (*mention_ptr).explicit_mention = 0 as *mut mention;
    (*mention_ptr).salience_score = (*(*mention_ptr).entity).salience_score;
    (*mention_ptr).static_salience_score = calc_static_salience_score(tag_ptr);
    strcpy((*mention_ptr).cpp_string.as_mut_ptr(), b"\xef\xbc\x8a\x00" as *const u8 as *const libc::c_char);
    (*(*mention_ptr).entity).salience_score += (*mention_ptr).static_salience_score;
    parent_ptr = (*tag_ptr).parent;
    while !parent_ptr.is_null() && (*parent_ptr).para_top_p as libc::c_int != 0 {
        parent_ptr = (*parent_ptr).parent
    }
    if !check_feature((*tag_ptr).f, b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() || !check_feature((*tag_ptr).f, b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
        (*(*mention_ptr).entity).tmp_salience_flag = 1 as libc::c_int
    }
    cp = check_feature((*tag_ptr).f, b"\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        strcpy((*mention_ptr).spp_string.as_mut_ptr(), cp.offset(strlen(b"\xe4\xbf\x82:\x00" as *const u8 as *const libc::c_char) as isize));
    } else if !check_feature((*tag_ptr).f, b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
        strcpy((*mention_ptr).spp_string.as_mut_ptr(), b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as *const libc::c_char);
    } else {
        strcpy((*mention_ptr).spp_string.as_mut_ptr(), b"\xef\xbc\x8a\x00" as *const u8 as *const libc::c_char);
    }
    (*mention_ptr).type_0 = '=' as i32 as libc::c_char;
    cp = strchr((*(*mention_ptr).entity).name.as_mut_ptr(), '|' as i32);
    hypo_flag = 0 as libc::c_int;
    name_change_flag = 0 as libc::c_int;
    if !cp.is_null() {
        strcpy(hypo_name.as_mut_ptr(), cp);
        hypo_flag = 1 as libc::c_int
    }
    cp = check_feature((*tag_ptr).f, b"NE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        strcpy((*(*mention_ptr).entity).named_entity.as_mut_ptr(), cp.offset(strlen(b"NE:\x00" as *const u8 as *const libc::c_char) as isize));
    }
    if strcmp((*(*mention_ptr).entity).name.as_mut_ptr(), b"\xe3\x81\xae\x00" as *const u8 as *const libc::c_char) == 0 || (*mention_ptr).salience_score == 0 as libc::c_int as libc::c_double && (*(*mention_ptr).entity).salience_score > 0 as libc::c_int as libc::c_double {
        cp = check_feature((*tag_ptr).f, b"NE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            strcpy(temp.as_mut_ptr(), cp);
            abbreviate_NE(temp.as_mut_ptr());
            strcpy((*(*mention_ptr).entity).name.as_mut_ptr(), temp.as_mut_ptr().offset(strlen(b"NE:\x00" as *const u8 as *const libc::c_char) as isize));
        } else {
            cp = check_feature((*tag_ptr).f, b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
            if !cp.is_null() {
                strcpy((*(*mention_ptr).entity).name.as_mut_ptr(), cp.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:\x00" as *const u8 as *const libc::c_char) as isize));
            } else {
                strcpy((*(*mention_ptr).entity).name.as_mut_ptr(), (*(*tag_ptr).head_ptr).Goi2.as_mut_ptr());
            }
        }
        name_change_flag = 1 as libc::c_int
    }
    if strchr((*(*mention_ptr).entity).name.as_mut_ptr(), ':' as i32).is_null() && {
        cp = check_feature((*tag_ptr).f, b"NE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        !cp.is_null()
    } {
        strcpy(temp.as_mut_ptr(), cp);
        abbreviate_NE(temp.as_mut_ptr());
        strcpy((*(*mention_ptr).entity).name.as_mut_ptr(), temp.as_mut_ptr().offset(strlen(b"NE:\x00" as *const u8 as *const libc::c_char) as isize));
        name_change_flag = 1 as libc::c_int
    } else if strchr((*(*mention_ptr).entity).name.as_mut_ptr(), ':' as i32).is_null() && !check_feature((*tag_ptr).f, b"\xe5\x90\x8c\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
        cp = check_feature((*tag_ptr).f, b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char);
        if !cp.is_null() {
            strcpy((*(*mention_ptr).entity).name.as_mut_ptr(),
                   cp.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:\x00" as *const u8 as *const libc::c_char) as isize));
        } else {
            strcpy((*(*mention_ptr).entity).name.as_mut_ptr(), (*(*tag_ptr).head_ptr).Goi2.as_mut_ptr());
        }
        name_change_flag = 1 as libc::c_int
    }
    if name_change_flag == 1 as libc::c_int {
        (*(*mention_ptr).entity).rep_sen_num = (*mention_ptr).sent_num;
        (*(*mention_ptr).entity).rep_tag_num = (*tag_ptr).num
    }
    if name_change_flag == 1 as libc::c_int && hypo_flag == 1 as libc::c_int {
        strcat((*(*mention_ptr).entity).name.as_mut_ptr(), hypo_name.as_mut_ptr());
    };
}

#[no_mangle]
pub unsafe extern "C" fn calc_static_salience_score(mut tag_ptr: *mut TAG_DATA) -> libc::c_double {
    return if (!check_feature((*tag_ptr).f,
                              b"\xe3\x83\x8f\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe3\x83\xa2\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null()) &&
        !check_feature((*tag_ptr).f,
                       b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        check_feature((*tag_ptr).f,
                      b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe5\x90\x8c\xe6\xa0\xbc\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        2.0f64
    } else if !check_feature((*tag_ptr).f,
                             b"\xe8\xaa\xad\xe7\x82\xb9\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
        (*tag_ptr).para_type as libc::c_int !=
            1 as libc::c_int ||
        !check_feature((*tag_ptr).f,
                       b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                           as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                           as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        1.0f64
    } else { 0.4f64 };
}

#[no_mangle]
pub unsafe extern "C" fn link_entity_from_corefer_id(mut sp:
                                                     *mut SENTENCE_DATA,
                                                     mut tag_ptr:
                                                     *mut TAG_DATA,
                                                     mut cp:
                                                     *mut libc::c_char) {
    let mut corefer_id_0: libc::c_int = 0;
    let mut mention_mgr: *mut MENTION_MGR = &mut (*tag_ptr).mention_mgr;
    let mut entity_num: libc::c_int = 0;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    // let mut parent_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    // let mut hypo_flag: libc::c_int = 0;
    // let mut name_change_flag: libc::c_int = 0;
    // let mut hypo_name: [libc::c_char; 256] = [0; 256];
    sscanf(cp, b"COREFER_ID:%d\x00" as *const u8 as *const libc::c_char,
           &mut corefer_id_0 as *mut libc::c_int);
    mention_ptr = (*mention_mgr).mention.as_mut_ptr();
    entity_num = 0 as libc::c_int;
    while entity_num < entity_manager.num {
        if entity_manager.entity[entity_num as usize].corefer_id ==
            corefer_id_0 {
            (*mention_ptr).entity =
                &mut *entity_manager.entity.as_mut_ptr().offset(entity_num as
                    isize) as
                    *mut ENTITY;
            set_mention_from_coreference(tag_ptr, mention_ptr);
            (*(*mention_ptr).entity).mention[(*(*mention_ptr).entity).mentioned_num
                as usize] = mention_ptr;
            if (*(*mention_ptr).entity).mentioned_num >=
                256 as libc::c_int - 1 as libc::c_int {
                fprintf(stderr,
                        b"Entity \"%s\" mentiond too many times!\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*(*mention_ptr).entity).name.as_mut_ptr());
            } else { (*(*mention_ptr).entity).mentioned_num += 1 }
        }
        entity_num += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_candidate_entities(mut sent_num: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut entity_num: libc::c_int = 0;
    let mut mention_num: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        if OptReadFeature & 64 as libc::c_int != 0 ||
            OptAnaphora & 262144 as libc::c_int != 0 {
            candidate_entities[i as usize] = 0 as libc::c_int
        } else { candidate_entities[i as usize] = 1 as libc::c_int }
        i += 1
    }
    entity_num = 0 as libc::c_int;
    while entity_num < entity_manager.num {
        if entity_num < 5 as libc::c_int {
            candidate_entities[entity_num as usize] = 1 as libc::c_int
        } else {
            if strcmp(entity_manager.entity[entity_num as
                usize].named_entity.as_mut_ptr(),
                      b"\x00" as *const u8 as *const libc::c_char) != 0 {
                mention_num = 0 as libc::c_int;
                while mention_num <
                    entity_manager.entity[entity_num as
                        usize].mentioned_num {
                    if sent_num -
                        (*entity_manager.entity[entity_num as
                            usize].mention[mention_num
                            as
                            usize]).sent_num
                        <= 3 as libc::c_int &&
                        sent_num -
                            (*entity_manager.entity[entity_num as
                                usize].mention[mention_num
                                as
                                usize]).sent_num
                            >= -(3 as libc::c_int) {
                        candidate_entities[entity_num as usize] =
                            1 as libc::c_int
                    }
                    mention_num += 1
                }
            }
            mention_num = 0 as libc::c_int;
            while mention_num <
                entity_manager.entity[entity_num as usize].mentioned_num
            {
                if sent_num -
                    (*entity_manager.entity[entity_num as
                        usize].mention[mention_num
                        as
                        usize]).sent_num
                    <= 1 as libc::c_int &&
                    sent_num -
                        (*entity_manager.entity[entity_num as
                            usize].mention[mention_num
                            as
                            usize]).sent_num
                        >= 0 as libc::c_int {
                    candidate_entities[entity_num as usize] = 1 as libc::c_int
                }
                mention_num += 1
            }
        }
        entity_num += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn calc_score_of_case_frame_assingemnt(mut ctm_ptr:
                                                             *mut CF_TAG_MGR,
                                                             mut tcf_ptr:
                                                             *mut TAG_CASE_FRAME)
                                                             -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    let mut debug: libc::c_int = 1 as libc::c_int;
    let mut score: libc::c_double = 0.;
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    // let mut tmp_score: libc::c_double = 0.;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    score =
        get_cf_probability_for_pred(&mut (*tcf_ptr).cf, (*ctm_ptr).cf_ptr);
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        let mut tmp_score_0: libc::c_double = -13.815511f64;
        let mut prob: libc::c_double = 0.;
        e_num = (*ctm_ptr).cf_element_num[i as usize];
        entity_ptr =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        if OptAnaphora & 32 as libc::c_int != 0 &&
            (*entity_ptr).num < 5 as libc::c_int {
            let mut entity_num: libc::c_int = (*entity_ptr).num;
            if OptGeneralCF & 2 as libc::c_int != 0 {
                k = 0 as libc::c_int;
                while k < 21 as libc::c_int {
                    if strcmp(unnamed_entity_category[entity_num as
                        usize][k as usize],
                              b"\x00" as *const u8 as *const libc::c_char) ==
                        0 {
                        break;
                    }
                    sprintf(key.as_mut_ptr(),
                            b"CT:%s:\x00" as *const u8 as *const libc::c_char,
                            unnamed_entity_category[entity_num as
                                usize][k as usize]);
                    prob =
                        get_ex_ne_probability(key.as_mut_ptr(), e_num,
                                              (*ctm_ptr).cf_ptr,
                                              (0 as libc::c_int == 0) as
                                                  libc::c_int);
                    if prob != 0. && tmp_score_0 < log(prob) {
                        tmp_score_0 = log(prob)
                    }
                    k += 1
                }
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if strcmp(unnamed_entity_rep[entity_num as usize][k as usize],
                          b"\x00" as *const u8 as *const libc::c_char) == 0 {
                    break;
                }
                sprintf(key.as_mut_ptr(),
                        unnamed_entity_rep[entity_num as usize][k as usize]);
                prob =
                    _get_ex_probability_internal(key.as_mut_ptr(), e_num,
                                                 (*ctm_ptr).cf_ptr);
                if prob != 0. && tmp_score_0 < log(prob) {
                    tmp_score_0 = log(prob)
                }
                k += 1
            }
        } else {
            j = 0 as libc::c_int;
            while j < (*entity_ptr).mentioned_num {
                if !((*(*entity_ptr).mention[j as usize]).type_0 as
                    libc::c_int != 'S' as i32 &&
                    (*(*entity_ptr).mention[j as usize]).type_0 as
                        libc::c_int != '=' as i32) {
                    if OptGeneralCF & 2 as libc::c_int != 0 &&
                        {
                            cp =
                                check_feature((*(*(*(*entity_ptr).mention[j
                                    as
                                    usize]).tag_ptr).head_ptr).f,
                                              b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            !cp.is_null()
                        } {
                        while !strchr(cp, ':' as i32).is_null() &&
                            {
                                cp = strchr(cp, ':' as i32);
                                !cp.is_null()
                            } ||
                            {
                                cp = strchr(cp, ';' as i32);
                                !cp.is_null()
                            } {
                            cp = cp.offset(1);
                            sprintf(key.as_mut_ptr(),
                                    b"CT:%s:\x00" as *const u8 as
                                        *const libc::c_char, cp);
                            if !strchr(key.as_mut_ptr().offset(3 as
                                libc::c_int
                                as isize),
                                       ';' as i32).is_null() {
                                *strchr(key.as_mut_ptr().offset(3 as
                                    libc::c_int
                                    as isize),
                                        ';' as i32) =
                                    ':' as i32 as libc::c_char
                            }
                            prob =
                                get_ex_ne_probability(key.as_mut_ptr(), e_num,
                                                      (*ctm_ptr).cf_ptr,
                                                      (0 as libc::c_int == 0)
                                                          as libc::c_int);
                            if prob != 0. && tmp_score_0 < log(prob) {
                                tmp_score_0 = log(prob)
                            }
                        }
                    }
                    if OptGeneralCF & 1 as libc::c_int != 0 &&
                        {
                            cp =
                                check_feature((*(*(*entity_ptr).mention[j
                                    as
                                    usize]).tag_ptr).f,
                                              b"NE\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            !cp.is_null()
                        } {
                        prob =
                            get_ex_ne_probability(cp, e_num,
                                                  (*ctm_ptr).cf_ptr,
                                                  (0 as libc::c_int == 0) as
                                                      libc::c_int);
                        if prob != 0. && tmp_score_0 < log(prob) {
                            tmp_score_0 = log(prob)
                        }
                    }
                    prob =
                        get_ex_probability((*ctm_ptr).tcf_element_num_functional[i
                            as
                            usize],
                                           &mut (*tcf_ptr).cf,
                                           (*(*entity_ptr).mention[j as
                                               usize]).tag_ptr,
                                           e_num, (*ctm_ptr).cf_ptr,
                                           0 as libc::c_int);
                    if prob != 0. && tmp_score_0 < prob { tmp_score_0 = prob }
                }
                j += 1
            }
        }
        score += tmp_score_0;
        if 0 as libc::c_int != 0 && OptDisplay == 3 as libc::c_int &&
            debug != 0 {
            if i < (*ctm_ptr).case_result_num {
                printf(b";;\xe5\xaf\xbe\xe5\xbf\x9c\xe3\x81\x82\xe3\x82\x8a:%s-%s:%f:%f \x00"
                           as *const u8 as *const libc::c_char,
                       (*(*(*ctm_ptr).elem_b_ptr[i as
                           usize]).head_ptr).Goi2.as_mut_ptr(),
                       pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num as
                           usize][0 as
                           libc::c_int
                           as
                           usize]),
                       get_ex_probability_with_para((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                    &mut (*tcf_ptr).cf, e_num,
                                                    (*ctm_ptr).cf_ptr),
                       get_case_function_probability_for_pred((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                              &mut (*tcf_ptr).cf,
                                                              e_num,
                                                              (*ctm_ptr).cf_ptr,
                                                              (0 as
                                                                  libc::c_int
                                                                  == 0) as
                                                                  libc::c_int));
            } else if (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as
                isize)).hypothetical_flag
                != 1 as libc::c_int {
                printf(b";;\xe5\xaf\xbe\xe5\xbf\x9c\xe3\x81\x82\xe3\x82\x8a:%s-%s:%f:%f \x00"
                           as *const u8 as *const libc::c_char,
                       (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                           as
                           usize]
                           as
                           isize)).name.as_mut_ptr(),
                       pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num as
                           usize][0 as
                           libc::c_int
                           as
                           usize]),
                       get_ex_probability_with_para((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                    &mut (*tcf_ptr).cf, e_num,
                                                    (*ctm_ptr).cf_ptr),
                       get_case_function_probability_for_pred((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                              &mut (*tcf_ptr).cf,
                                                              e_num,
                                                              (*ctm_ptr).cf_ptr,
                                                              (0 as
                                                                  libc::c_int
                                                                  == 0) as
                                                                  libc::c_int));
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*tcf_ptr).cf.element_num - (*ctm_ptr).case_result_num {
        if 0 as libc::c_int != 0 && OptDisplay == 3 as libc::c_int &&
            debug != 0 {
            if i < (*ctm_ptr).case_result_num {
                printf(b";;\xe5\xaf\xbe\xe5\xbf\x9c\xe3\x81\xaa\xe3\x81\x97:%s:%f \x00"
                           as *const u8 as *const libc::c_char,
                       (*(*(*tcf_ptr).elem_b_ptr[(*ctm_ptr).non_match_element[i
                           as
                           usize]
                           as
                           usize]).head_ptr).Goi2.as_mut_ptr(),
                       score);
            }
        }
        score += -13.815511f64 + -11.512925f64;
        i += 1
    }
    if 0 as libc::c_int != 0 && OptDisplay == 3 as libc::c_int && debug != 0 {
        printf(b";; %f \x00" as *const u8 as *const libc::c_char, score);
    }
    e_num = 0 as libc::c_int;
    while e_num < (*(*ctm_ptr).cf_ptr).element_num {
        if !((*tcf_ptr).cf.type_0 == 2 as libc::c_int) {
            score +=
                get_case_probability(e_num, (*ctm_ptr).cf_ptr,
                                     (*ctm_ptr).filled_element[e_num as
                                         usize],
                                     0 as *mut CF_PRED_MGR)
        }
        e_num += 1
    }
    if 0 as libc::c_int != 0 && OptDisplay == 3 as libc::c_int && debug != 0 {
        printf(b";; %f\n\x00" as *const u8 as *const libc::c_char, score);
    }
    return score;
}

#[no_mangle]
pub unsafe extern "C" fn relax_compare_result(mut gresult: *mut libc::c_char,
                                              mut aresult: *mut libc::c_char)
                                              -> libc::c_int {
    let mut gresult_entity_list: [[libc::c_int; 4096]; 4] = [[0; 4096]; 4];
    let mut aresult_entity_list: [[libc::c_int; 4096]; 4] = [[0; 4096]; 4];
    let mut gresult_entity_num: [libc::c_int; 4] = [0; 4];
    let mut aresult_entity_num: [libc::c_int; 4] = [0; 4];
    let mut check_case: [libc::c_int; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cp: [libc::c_char; 256] = [0; 256];
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        gresult_entity_num[i as usize] = 0 as libc::c_int;
        aresult_entity_num[i as usize] = 0 as libc::c_int;
        i += 1
    }
    strcpy(cp.as_mut_ptr(), aresult);
    if *cp.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
        tp =
            strtok(cp.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
        while !tp.is_null() {
            if !tp.is_null() {
                let mut case_num: libc::c_int = 0;
                case_num = 0 as libc::c_int;
                while case_num < 4 as libc::c_int {
                    let mut case_ptr: *mut libc::c_char =
                        0 as *mut libc::c_char;
                    case_ptr =
                        strstr(tp,
                               ELLIPSIS_CASE_LIST_VERB[case_num as usize]);
                    if !case_ptr.is_null() {
                        let mut e_num: libc::c_int = 0;
                        case_ptr =
                            case_ptr.offset(strlen(ELLIPSIS_CASE_LIST_VERB[case_num
                                as
                                usize])
                                as isize);
                        sscanf(case_ptr,
                               b":%d\x00" as *const u8 as *const libc::c_char,
                               &mut e_num as *mut libc::c_int);
                        aresult_entity_list[case_num as
                            usize][aresult_entity_num[case_num
                            as
                            usize]
                            as usize] = e_num;
                        aresult_entity_num[case_num as usize] += 1
                    }
                    case_num += 1
                }
                tp =
                    strtok(0 as *mut libc::c_char,
                           b" \x00" as *const u8 as *const libc::c_char)
            }
        }
    }
    strcpy(cp.as_mut_ptr(), gresult);
    if *cp.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
        tp =
            strtok(cp.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
        while !tp.is_null() {
            if !tp.is_null() {
                let mut case_num_0: libc::c_int = 0;
                case_num_0 = 0 as libc::c_int;
                while case_num_0 < 4 as libc::c_int {
                    let mut case_ptr_0: *mut libc::c_char =
                        0 as *mut libc::c_char;
                    case_ptr_0 =
                        strstr(tp,
                               ELLIPSIS_CASE_LIST_VERB[case_num_0 as usize]);
                    if !case_ptr_0.is_null() {
                        let mut e_num_0: libc::c_int = 0;
                        case_ptr_0 =
                            case_ptr_0.offset(strlen(ELLIPSIS_CASE_LIST_VERB[case_num_0
                                as
                                usize])
                                as isize);
                        sscanf(case_ptr_0,
                               b":%d\x00" as *const u8 as *const libc::c_char,
                               &mut e_num_0 as *mut libc::c_int);
                        gresult_entity_list[case_num_0 as
                            usize][gresult_entity_num[case_num_0
                            as
                            usize]
                            as usize] =
                            e_num_0;
                        gresult_entity_num[case_num_0 as usize] += 1
                    }
                    case_num_0 += 1
                }
                tp =
                    strtok(0 as *mut libc::c_char,
                           b" \x00" as *const u8 as *const libc::c_char)
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if aresult_entity_num[i as usize] == 0 as libc::c_int &&
            gresult_entity_num[i as usize] == 0 as libc::c_int {
            check_case[i as usize] = 1 as libc::c_int
        } else if aresult_entity_num[i as usize] == 0 as libc::c_int ||
            gresult_entity_num[i as usize] == 0 as libc::c_int {
            check_case[i as usize] = -(1 as libc::c_int)
        } else {
            check_case[i as usize] = -(1 as libc::c_int);
            j = 0 as libc::c_int;
            while j < aresult_entity_num[i as usize] {
                k = 0 as libc::c_int;
                while k < gresult_entity_num[i as usize] {
                    if aresult_entity_list[i as usize][j as usize] ==
                        gresult_entity_list[i as usize][k as usize] {
                        check_case[i as usize] = 1 as libc::c_int
                    }
                    k += 1
                }
                j += 1
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if check_case[i as usize] == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn author_detect() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp_author_sen: libc::c_int = 100 as libc::c_int;
    let mut temp_author_tag: libc::c_int = 100 as libc::c_int;
    let mut temp_reader_sen: libc::c_int = 100 as libc::c_int;
    let mut temp_reader_tag: libc::c_int = 100 as libc::c_int;
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut author_entity: *mut ENTITY = 0 as *mut ENTITY;
    let mut reader_entity: *mut ENTITY = 0 as *mut ENTITY;
    let mut author_rep: [*mut libc::c_char; 7] =
        [b"\xe7\xa7\x81/\xe3\x82\x8f\xe3\x81\x9f\xe3\x81\x97\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
            b"\xe5\x83\x95/\xe3\x81\xbc\xe3\x81\x8f\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            b"\xe6\x88\x91\xe3\x80\x85/\xe3\x82\x8f\xe3\x82\x8c\xe3\x82\x8f\xe3\x82\x8c\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe4\xbf\xba/\xe3\x81\x8a\xe3\x82\x8c\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            b"\xe5\xbd\x93\xe7\xa4\xbe/\xe3\x81\xa8\xe3\x81\x86\xe3\x81\x97\xe3\x82\x83\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xbc\x8a\xe7\xa4\xbe/\xe3\x81\xb8\xe3\x81\x84\xe3\x81\x97\xe3\x82\x83\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xbd\x93\xe5\xba\x97/\xe3\x81\xa8\xe3\x81\x86\xe3\x81\xa6\xe3\x82\x93\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char];
    let mut reader_rep: [*mut libc::c_char; 5] =
        [b"\xe3\x81\x82\xe3\x81\xaa\xe3\x81\x9f/\xe3\x81\x82\xe3\x81\xaa\xe3\x81\x9f\x00"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\x90\x9b/\xe3\x81\x8d\xe3\x81\xbf\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            b"\xe7\x9a\x86\xe3\x81\x95\xe3\x82\x93/\xe3\x81\xbf\xe3\x81\xaa\xe3\x81\x95\xe3\x82\x93\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\xe5\xae\xa2/\xe3\x81\x8d\xe3\x82\x83\xe3\x81\x8f\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"\xe7\x9a\x86\xe6\xa7\x98/\xe3\x81\xbf\xe3\x81\xaa\xe3\x81\x95\xe3\x81\xbe\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char];
    if entity_manager.entity[0 as libc::c_int as usize].real_entity ==
        -(1 as libc::c_int) && OptAnaphora & 128 as libc::c_int == 0 ||
        (entity_manager.entity[1 as libc::c_int as usize].real_entity ==
            -(1 as libc::c_int) || OptAnaphora & 256 as libc::c_int == 0)
    {
        i = 0 as libc::c_int;
        while i < entity_manager.num {
            entity_ptr =
                &mut *entity_manager.entity.as_mut_ptr().offset(i as isize) as
                    *mut ENTITY;
            j = 0 as libc::c_int;
            while j < (*entity_ptr).mentioned_num {
                let mut author_rep_flag: libc::c_int = 0 as libc::c_int;
                let mut reader_rep_flag: libc::c_int = 0 as libc::c_int;
                if entity_manager.entity[0 as libc::c_int as
                    usize].real_entity ==
                    -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < 7 as libc::c_int {
                        let mut temp_rep: [libc::c_char; 266] = [0; 266];
                        strcpy(temp_rep.as_mut_ptr(),
                               b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                   as *const u8 as *const libc::c_char);
                        strcat(temp_rep.as_mut_ptr(), author_rep[k as usize]);
                        if !check_feature((*(*(*entity_ptr).mention[j as
                            usize]).tag_ptr).f,
                                          temp_rep.as_mut_ptr()).is_null() {
                            author_rep_flag = 1 as libc::c_int
                        }
                        k += 1
                    }
                }
                if entity_manager.entity[1 as libc::c_int as
                    usize].real_entity ==
                    -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < 5 as libc::c_int {
                        let mut temp_rep_0: [libc::c_char; 266] = [0; 266];
                        strcpy(temp_rep_0.as_mut_ptr(),
                               b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                   as *const u8 as *const libc::c_char);
                        strcat(temp_rep_0.as_mut_ptr(),
                               reader_rep[k as usize]);
                        if !check_feature((*(*(*entity_ptr).mention[j as
                            usize]).tag_ptr).f,
                                          temp_rep_0.as_mut_ptr()).is_null() {
                            reader_rep_flag = 1 as libc::c_int
                        }
                        k += 1
                    }
                }
                if temp_author_sen >=
                    (*(*entity_ptr).mention[j as usize]).sent_num &&
                    temp_author_tag >=
                        (*(*entity_ptr).mention[j as usize]).tag_num {
                    if author_rep_flag == 1 as libc::c_int {
                        author_entity = entity_ptr;
                        temp_author_sen =
                            (*(*entity_ptr).mention[j as usize]).sent_num;
                        temp_author_tag =
                            (*(*entity_ptr).mention[j as usize]).tag_num
                    }
                }
                if temp_reader_sen >=
                    (*(*entity_ptr).mention[j as usize]).sent_num &&
                    temp_reader_tag >=
                        (*(*entity_ptr).mention[j as usize]).tag_num {
                    if reader_rep_flag == 1 as libc::c_int {
                        reader_entity = entity_ptr;
                        temp_reader_sen =
                            (*(*entity_ptr).mention[j as usize]).sent_num;
                        temp_reader_tag =
                            (*(*entity_ptr).mention[j as usize]).tag_num
                    }
                }
                j += 1
            }
            i += 1
        }
        if OptAnaphora & 128 as libc::c_int == 0 && !author_entity.is_null() {
            let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
            let mut mention_id: libc::c_int = 0;
            (*author_entity).hypothetical_entity = 0 as libc::c_int;
            entity_manager.entity[0 as libc::c_int as usize].real_entity =
                (*author_entity).num;
            entity_manager.entity[0 as libc::c_int as usize].skip_flag =
                1 as libc::c_int;
            strcat((*author_entity).name.as_mut_ptr(),
                   b"|\xe8\x91\x97\xe8\x80\x85\x00" as *const u8 as
                       *const libc::c_char);
            mention_id = 0 as libc::c_int;
            while mention_id < (*author_entity).mentioned_num {
                mention_ptr = (*author_entity).mention[mention_id as usize];
                if (*mention_ptr).type_0 as libc::c_int == 'S' as i32 ||
                    (*mention_ptr).type_0 as libc::c_int == '=' as i32 {
                    assign_cfeature(&mut (*(*mention_ptr).tag_ptr).f,
                                    b"\xe8\x91\x97\xe8\x80\x85\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                mention_id += 1
            }
        }
        if OptAnaphora & 256 as libc::c_int == 0 && !reader_entity.is_null() {
            let mut mention_ptr_0: *mut MENTION = 0 as *mut MENTION;
            let mut mention_id_0: libc::c_int = 0;
            (*reader_entity).hypothetical_entity = 1 as libc::c_int;
            entity_manager.entity[1 as libc::c_int as usize].real_entity =
                (*reader_entity).num;
            entity_manager.entity[1 as libc::c_int as usize].skip_flag =
                1 as libc::c_int;
            strcat((*reader_entity).name.as_mut_ptr(),
                   b"|\xe8\xaa\xad\xe8\x80\x85\x00" as *const u8 as
                       *const libc::c_char);
            mention_id_0 = 0 as libc::c_int;
            while mention_id_0 < (*reader_entity).mentioned_num {
                mention_ptr_0 =
                    (*reader_entity).mention[mention_id_0 as usize];
                if (*mention_ptr_0).type_0 as libc::c_int == 'S' as i32 ||
                    (*mention_ptr_0).type_0 as libc::c_int == '=' as i32 {
                    assign_cfeature(&mut (*(*mention_ptr_0).tag_ptr).f,
                                    b"\xe8\xaa\xad\xe8\x80\x85\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                mention_id_0 += 1
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn merge_two_entity(mut target_entity_ptr: *mut ENTITY,
                                          mut source_entity_ptr:
                                          *mut ENTITY) {
    // let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*source_entity_ptr).mentioned_num {
        (*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num + j)
            as usize] =
            (*source_entity_ptr).mention[j as usize];
        if (*(*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num
            + j) as usize]).type_0 as
            libc::c_int == 'E' as i32 {
            (*(*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num
                + j) as usize]).type_0 =
                'O' as i32 as libc::c_char
        }
        if (*(*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num
            + j) as usize]).type_0 as
            libc::c_int == 'S' as i32 ||
            (*(*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num
                + j) as usize]).type_0 as
                libc::c_int == '=' as i32 {
            let ref mut fresh0 =
                (*(*(*(*target_entity_ptr).mention[((*target_entity_ptr).mentioned_num
                    + j) as
                    usize]).tag_ptr).mention_mgr.mention.as_mut_ptr()).entity;
            *fresh0 = target_entity_ptr
        }
        j += 1
    }
    (*target_entity_ptr).mentioned_num += (*source_entity_ptr).mentioned_num;
    (*source_entity_ptr).mentioned_num = 0 as libc::c_int;
    (*source_entity_ptr).skip_flag = 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn merge_hypo_real_entity_auto() {
    let mut real_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut entity_idx: libc::c_int = 0;
    let mut mention_idx: libc::c_int = 0;
    entity_idx = 0 as libc::c_int;
    while entity_idx < entity_manager.num {
        let mut hypo_num: libc::c_int = -(1 as libc::c_int);
        real_entity_ptr =
            &mut *entity_manager.entity.as_mut_ptr().offset(entity_idx as
                isize) as
                *mut ENTITY;
        mention_idx = 0 as libc::c_int;
        while mention_idx < (*real_entity_ptr).mentioned_num {
            mention_ptr =
                entity_manager.entity[entity_idx as
                    usize].mention[mention_idx as
                    usize];
            if !((*mention_ptr).type_0 as libc::c_int != '=' as i32 &&
                (*mention_ptr).type_0 as libc::c_int != 'S' as i32) {
                if (*mention_ptr).sent_num == author_sen &&
                    (*mention_ptr).tag_num == author_tag {
                    hypo_num = 0 as libc::c_int
                } else if (*mention_ptr).sent_num == reader_sen &&
                    (*mention_ptr).tag_num == reader_tag {
                    hypo_num = 1 as libc::c_int
                }
                if hypo_num != -(1 as libc::c_int) {
                    entity_manager.entity[hypo_num as usize].real_entity =
                        entity_idx;
                    (*real_entity_ptr).hypothetical_entity = hypo_num;
                    merge_two_entity(real_entity_ptr,
                                     &mut *entity_manager.entity.as_mut_ptr().offset(hypo_num
                                         as
                                         isize));
                    strcat((*real_entity_ptr).name.as_mut_ptr(),
                           b"|\x00" as *const u8 as *const libc::c_char);
                    strcat((*real_entity_ptr).name.as_mut_ptr(),
                           entity_manager.entity[hypo_num as
                               usize].name.as_mut_ptr());
                    break;
                }
            }
            mention_idx += 1
        }
        entity_idx += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn merge_hypo_real_entity() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    let mut hypo_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut real_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    i = 0 as libc::c_int;
    while i < entity_manager.num {
        if !(entity_manager.entity[i as usize].hypothetical_flag !=
            1 as libc::c_int) {
            hypo_entity_ptr =
                &mut *entity_manager.entity.as_mut_ptr().offset(i as isize) as
                    *mut ENTITY;
            if i < 5 as libc::c_int {
                if (*hypo_entity_ptr).real_entity != -(1 as libc::c_int) {
                    real_entity_ptr =
                        &mut *entity_manager.entity.as_mut_ptr().offset((*hypo_entity_ptr).real_entity
                            as
                            isize)
                            as *mut ENTITY;
                    merge_two_entity(real_entity_ptr, hypo_entity_ptr);
                }
            } else {
                j = 0 as libc::c_int;
                while j < entity_manager.num {
                    if strcmp((*hypo_entity_ptr).name.as_mut_ptr(),
                              entity_manager.entity[j as
                                  usize].hypothetical_name.as_mut_ptr())
                        == 0 {
                        real_entity_ptr =
                            &mut *entity_manager.entity.as_mut_ptr().offset(j
                                as
                                isize)
                                as *mut ENTITY;
                        merge_two_entity(real_entity_ptr, hypo_entity_ptr);
                    }
                    j += 1
                }
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn link_hypothetical_entity(mut token:
                                                  *mut libc::c_char,
                                                  mut entity_ptr: *mut ENTITY)
                                                  -> libc::c_int {
    let mut type_0: libc::c_char = 0;
    let mut rel: [libc::c_char; 128] = [0; 128];
    let mut entity_name: [libc::c_char; 256] = [0; 256];
    // let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tag_num: libc::c_int = 0;
    let mut sent_num: libc::c_int = 0;
    let mut hypo_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut entity_num: libc::c_int = -(1 as libc::c_int);
    token =
        strchr(token.offset(strlen(b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:\x00"
            as *const u8 as *const libc::c_char) as
            isize),
               ':' as i32).offset(1 as libc::c_int as isize);
    while *token != 0 {
        if *token as libc::c_int == ':' as i32 ||
            *token as libc::c_int == ';' as i32 {
            token = token.offset(1);
            if !(sscanf(token,
                        b"%[^/]/%c/%[^/]/%d/%d/\x00" as *const u8 as
                            *const libc::c_char, rel.as_mut_ptr(),
                        &mut type_0 as *mut libc::c_char,
                        entity_name.as_mut_ptr(),
                        &mut tag_num as *mut libc::c_int,
                        &mut sent_num as *mut libc::c_int) == 0) {
                if (strcmp(rel.as_mut_ptr(),
                           b"=\x00" as *const u8 as *const libc::c_char) == 0
                    ||
                    strcmp(rel.as_mut_ptr(),
                           b"=\xe6\xa7\x8b\x00" as *const u8 as
                               *const libc::c_char) == 0 ||
                    strcmp(rel.as_mut_ptr(),
                           b"=\xe5\xbd\xb9\x00" as *const u8 as
                               *const libc::c_char) == 0) &&
                    tag_num == -(1 as libc::c_int) {
                    i = 0 as libc::c_int;
                    while i < 5 as libc::c_int {
                        j = 0 as libc::c_int;
                        while j < 2 as libc::c_int {
                            if strcmp(unnamed_entity_name[i as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                break;
                            }
                            if strcmp(unnamed_entity_name[i as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) != 0 &&
                                strcmp(unnamed_entity_name[i as
                                    usize][j as
                                    usize],
                                       entity_name.as_mut_ptr()) == 0 {
                                (*entity_ptr).hypothetical_entity = i;
                                strcpy((*entity_ptr).hypothetical_name.as_mut_ptr(),
                                       entity_name.as_mut_ptr());
                                entity_manager.entity[i as usize].real_entity
                                    = (*entity_ptr).num;
                                strcat((*entity_ptr).name.as_mut_ptr(),
                                       b"|\x00" as *const u8 as
                                           *const libc::c_char);
                                strcat((*entity_ptr).name.as_mut_ptr(),
                                       entity_name.as_mut_ptr());
                                return (0 as libc::c_int == 0) as libc::c_int;
                            }
                            j += 1
                        }
                        i += 1
                    }
                    i = 0 as libc::c_int;
                    while i < entity_manager.num {
                        if strcmp(entity_manager.entity[i as
                            usize].hypothetical_name.as_mut_ptr(),
                                  entity_name.as_mut_ptr()) == 0 {
                            merge_two_entity(&mut *entity_manager.entity.as_mut_ptr().offset(i
                                as
                                isize),
                                             entity_ptr);
                            return (0 as libc::c_int == 0) as libc::c_int;
                        }
                        i += 1
                    }
                    i = 0 as libc::c_int;
                    while i < 5 as libc::c_int {
                        j = 0 as libc::c_int;
                        while j < 2 as libc::c_int {
                            if strcmp(unnamed_entity_name[i as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                break;
                            }
                            if strcmp(unnamed_entity_name[i as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) != 0 &&
                                !strstr(entity_name.as_mut_ptr(),
                                        unnamed_entity_name[i as
                                            usize][j as
                                            usize]).is_null()
                            {
                                entity_num = i
                            }
                            j += 1
                        }
                        i += 1
                    }
                    hypo_entity_ptr =
                        make_each_unnamed_entity(entity_name.as_mut_ptr(),
                                                 entity_num);
                    (*hypo_entity_ptr).skip_flag = 1 as libc::c_int;
                    (*entity_ptr).hypothetical_entity =
                        (*hypo_entity_ptr).num;
                    strcpy((*entity_ptr).hypothetical_name.as_mut_ptr(),
                           entity_name.as_mut_ptr());
                    (*hypo_entity_ptr).real_entity = (*entity_ptr).num;
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
            }
        }
        token = token.offset(1)
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn make_tag_pa_string(mut tag_pa_string:
                                            *mut libc::c_char,
                                            mut ga_elem: *const libc::c_char,
                                            mut wo_elem: *const libc::c_char,
                                            mut ni_elem: *const libc::c_char,
                                            mut pred: *const libc::c_char) {
    strcpy(tag_pa_string, ga_elem);
    strcat(tag_pa_string,
           b":\xe3\x82\xac;\x00" as *const u8 as *const libc::c_char);
    strcat(tag_pa_string, wo_elem);
    strcat(tag_pa_string,
           b":\xe3\x83\xb2;\x00" as *const u8 as *const libc::c_char);
    strcat(tag_pa_string, ni_elem);
    strcat(tag_pa_string,
           b":\xe3\x83\x8b;\x00" as *const u8 as *const libc::c_char);
    strcat(tag_pa_string, pred);
}

#[no_mangle]
pub unsafe extern "C" fn bnst_to_pseudo_pp(mut child: *mut BNST_DATA)
                                           -> libc::c_int {
    return if !check_feature((*child).f,
                             b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        1 as libc::c_int
    } else if !check_feature((*child).f,
                             b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        2 as libc::c_int
    } else if !check_feature((*child).f,
                             b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        3 as libc::c_int
    } else { 0 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn get_swappable_lift_value(mut analysing_tag_pa_string:
                                                  *mut libc::c_char,
                                                  mut target_tag_pa_string:
                                                  *mut libc::c_char,
                                                  mut temp_match_string:
                                                  *mut libc::c_char,
                                                  mut sequence:
                                                  *mut libc::c_int)
                                                  -> libc::c_double {
    let mut score1: libc::c_double = 0.;
    let mut score2: libc::c_double = 0.;
    score1 =
        get_lift_value(analysing_tag_pa_string, target_tag_pa_string,
                       temp_match_string);
    score2 =
        get_lift_value(target_tag_pa_string, analysing_tag_pa_string,
                       temp_match_string);
    return if score1 > score2 {
        *sequence = -(1 as libc::c_int);
        score1
    } else {
        *sequence = 1 as libc::c_int;
        score2
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_lift_value(mut analysing_tag_pa_string:
                                        *mut libc::c_char,
                                        mut target_tag_pa_string:
                                        *mut libc::c_char,
                                        mut temp_match_string:
                                        *mut libc::c_char)
                                        -> libc::c_double {
    let mut pa_pair_string: [libc::c_char; 795] = [0; 795];
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lift_value: libc::c_double = 0.;
    strcpy(pa_pair_string.as_mut_ptr(), analysing_tag_pa_string);
    strcat(pa_pair_string.as_mut_ptr(),
           b"=>\x00" as *const u8 as *const libc::c_char);
    strcat(pa_pair_string.as_mut_ptr(), target_tag_pa_string);
    value = db_get(event_db, pa_pair_string.as_mut_ptr());
    if !value.is_null() {
        sscanf(value, b"%lf\x00" as *const u8 as *const libc::c_char,
               &mut lift_value as *mut libc::c_double);
        strcpy(temp_match_string, pa_pair_string.as_mut_ptr());
        return lift_value;
    }
    return 0 as libc::c_int as libc::c_double;
}

#[no_mangle]
pub unsafe extern "C" fn check_pred_pair(mut pred1: *mut libc::c_char,
                                         mut pred2: *mut libc::c_char)
                                         -> libc::c_int {
    let mut pred_pair_string: [libc::c_char; 267] = [0; 267];
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_int = 0;
    strcpy(pred_pair_string.as_mut_ptr(), pred1);
    strcat(pred_pair_string.as_mut_ptr(),
           b"=>\x00" as *const u8 as *const libc::c_char);
    strcat(pred_pair_string.as_mut_ptr(), pred2);
    value = db_get(event_db, pred_pair_string.as_mut_ptr());
    if !value.is_null() {
        sscanf(value, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut flag as *mut libc::c_int);
        if flag == 1 as libc::c_int { return 1 as libc::c_int; }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn check_swappable_pred_pair(mut pred1:
                                                   *mut libc::c_char,
                                                   mut pred2:
                                                   *mut libc::c_char)
                                                   -> libc::c_int {
    let mut flag: libc::c_int = -(1 as libc::c_int);
    flag = check_pred_pair(pred1, pred2);
    if flag == 1 as libc::c_int { return 1 as libc::c_int; }
    flag = check_pred_pair(pred2, pred1);
    if flag == 1 as libc::c_int { return 1 as libc::c_int; }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn check_relational_event(mut analysing_tag:
                                                *mut TAG_DATA,
                                                mut ctm_ptr: *mut CF_TAG_MGR,
                                                mut focus_pp: libc::c_int,
                                                mut target_mention_ptr:
                                                *mut MENTION,
                                                mut sent_num: libc::c_int,
                                                mut tag_num: libc::c_int,
                                                mut loc_name:
                                                *mut libc::c_char)
                                                -> libc::c_double {
    let mut current_block: u64;
    let mut analysing_cf_entry: [libc::c_char; 258] =
        *::std::mem::transmute::<&[u8; 258],
            &mut [libc::c_char; 258]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut target_cf_entry: [libc::c_char; 258] =
        *::std::mem::transmute::<&[u8; 258],
            &mut [libc::c_char; 258]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut mention_mgr_ptr: *mut MENTION_MGR = 0 as *mut MENTION_MGR;
    // let mut analysing_tag_pa_num: libc::c_int = 0 as libc::c_int;
    let mut analysing_tag_arg_num: [libc::c_int; 4] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
            0 as libc::c_int];
    // let mut target_tag_pa_num: libc::c_int = 0 as libc::c_int;
    let mut target_tag_arg_num: [libc::c_int; 4] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
            0 as libc::c_int];
    let mut target_pp: libc::c_int =
        pp_kstr_to_code((*target_mention_ptr).cpp_string.as_mut_ptr());
    let mut pred_tag_num: libc::c_int = -(1 as libc::c_int);
    let mut analysing_ga_mention_idx: libc::c_int = 0;
    let mut analysing_wo_mention_idx: libc::c_int = 0;
    let mut analysing_ni_mention_idx: libc::c_int = 0;
    let mut target_ga_mention_idx: libc::c_int = 0;
    let mut target_wo_mention_idx: libc::c_int = 0;
    let mut target_ni_mention_idx: libc::c_int = 0;
    let mut backword_flag: libc::c_int = 0 as libc::c_int;
    let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut match_string: [libc::c_char; 795] = [0; 795];
    let mut temp_match_string: [libc::c_char; 795] = [0; 795];
    let mut sequence: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut key2: [libc::c_char; 128] = [0; 128];
    let mut un_lex_match: libc::c_int = 0 as libc::c_int;
    let mut analysing_tag_arg_string: *mut *mut *mut libc::c_char =
        0 as *mut *mut *mut libc::c_char;
    let mut target_tag_arg_string: *mut *mut *mut libc::c_char =
        0 as *mut *mut *mut libc::c_char;
    analysing_tag_arg_string =
        malloc((::std::mem::size_of::<*mut *mut libc::c_char>() as
            libc::c_ulong).wrapping_mul(4 as libc::c_int as
            libc::c_ulong)) as
            *mut *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh1 = *analysing_tag_arg_string.offset(i as isize);
        *fresh1 =
            malloc((::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul((256 as libc::c_int +
                2 as libc::c_int) as
                libc::c_ulong)) as
                *mut *mut libc::c_char;
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int + 2 as libc::c_int {
            let ref mut fresh2 =
                *(*analysing_tag_arg_string.offset(i as
                    isize)).offset(j as
                    isize);
            *fresh2 =
                malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((256 as libc::c_int +
                    4 as libc::c_int)
                    as libc::c_ulong))
                    as *mut libc::c_char;
            j += 1
        }
        i += 1
    }
    target_tag_arg_string =
        malloc((::std::mem::size_of::<*mut *mut libc::c_char>() as
            libc::c_ulong).wrapping_mul(4 as libc::c_int as
            libc::c_ulong)) as
            *mut *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh3 = *target_tag_arg_string.offset(i as isize);
        *fresh3 =
            malloc((::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul((256 as libc::c_int +
                2 as libc::c_int) as
                libc::c_ulong)) as
                *mut *mut libc::c_char;
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int + 2 as libc::c_int {
            let ref mut fresh4 =
                *(*target_tag_arg_string.offset(i as
                    isize)).offset(j as
                    isize);
            *fresh4 =
                malloc((::std::mem::size_of::<libc::c_char>() as
                    libc::c_ulong).wrapping_mul((256 as libc::c_int +
                    4 as libc::c_int)
                    as libc::c_ulong))
                    as *mut libc::c_char;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if i == focus_pp - 1 as libc::c_int {
            if un_lex_match == 1 as libc::c_int {
                strcpy(*(*analysing_tag_arg_string.offset(i as
                    isize)).offset(0
                    as
                    libc::c_int
                    as
                    isize),
                       b"X\x00" as *const u8 as *const libc::c_char);
                analysing_tag_arg_num[i as usize] += 1
            }
        } else {
            strcpy(*(*analysing_tag_arg_string.offset(i as
                isize)).offset(0 as
                libc::c_int
                as
                isize),
                   b"*\x00" as *const u8 as *const libc::c_char);
            analysing_tag_arg_num[i as usize] += 1;
            if un_lex_match == 1 as libc::c_int {
                strcpy(*(*analysing_tag_arg_string.offset(i as
                    isize)).offset(1
                    as
                    libc::c_int
                    as
                    isize),
                       b"Y\x00" as *const u8 as *const libc::c_char);
                analysing_tag_arg_num[i as usize] += 1
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        let mut pp: libc::c_int = 0;
        let mut e_num: libc::c_int = 0;
        e_num = (*ctm_ptr).cf_element_num[i as usize];
        entity_ptr =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        pp =
            (*(*ctm_ptr).cf_ptr).pp[e_num as
                usize][0 as libc::c_int as usize];
        if !(pp > 3 as libc::c_int || pp < 1 as libc::c_int) {
            if !(un_lex_match == 1 as libc::c_int && pp == focus_pp) {
                j = 0 as libc::c_int;
                while j < (*entity_ptr).mentioned_num {
                    if !((*(*entity_ptr).mention[j as usize]).type_0 as
                        libc::c_int != 'S' as i32 &&
                        (*(*entity_ptr).mention[j as usize]).type_0 as
                            libc::c_int != '=' as i32) {
                        cp =
                            get_bnst_head_canonical_rep((*(*(*entity_ptr).mention[j
                                as
                                usize]).tag_ptr).b_ptr,
                                                        OptCaseFlag &
                                                            512 as
                                                                libc::c_int);
                        if !cp.is_null() {
                            strcpy(*(*analysing_tag_arg_string.offset((pp -
                                1
                                    as
                                    libc::c_int)
                                as
                                isize)).offset(analysing_tag_arg_num[(pp
                                -
                                1
                                    as
                                    libc::c_int)
                                as
                                usize]
                                as
                                isize),
                                   cp);
                            analysing_tag_arg_num[(pp - 1 as libc::c_int) as
                                usize] += 1
                        }
                        if un_lex_match == 0 as libc::c_int {
                            cp =
                                check_feature((*(*(*(*entity_ptr).mention[j as
                                    usize]).tag_ptr).head_ptr).f,
                                              b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !cp.is_null() {
                                while !strchr(cp, ':' as i32).is_null() &&
                                    {
                                        cp = strchr(cp, ':' as i32);
                                        !cp.is_null()
                                    } ||
                                    {
                                        cp = strchr(cp, ';' as i32);
                                        !cp.is_null()
                                    } {
                                    cp = cp.offset(1);
                                    sprintf(key.as_mut_ptr(),
                                            b"CT:%s:\x00" as *const u8 as
                                                *const libc::c_char, cp);
                                    if !strchr(key.as_mut_ptr().offset(3 as
                                        libc::c_int
                                        as
                                        isize),
                                               ';' as i32).is_null() {
                                        *strchr(key.as_mut_ptr().offset(3 as
                                            libc::c_int
                                            as
                                            isize),
                                                ';' as i32) =
                                            ':' as i32 as libc::c_char
                                    }
                                    *strchr(key.as_mut_ptr().offset(3 as
                                        libc::c_int
                                        as
                                        isize),
                                            ':' as i32) =
                                        '\u{0}' as i32 as libc::c_char;
                                    sprintf(key2.as_mut_ptr(),
                                            b"<%s>\x00" as *const u8 as
                                                *const libc::c_char,
                                            key.as_mut_ptr());
                                    strcpy(*(*analysing_tag_arg_string.offset((pp
                                        -
                                        1
                                            as
                                            libc::c_int)
                                        as
                                        isize)).offset(analysing_tag_arg_num[(pp
                                        -
                                        1
                                            as
                                            libc::c_int)
                                        as
                                        usize]
                                        as
                                        isize),
                                           key2.as_mut_ptr());
                                    analysing_tag_arg_num[(pp -
                                        1 as
                                            libc::c_int)
                                        as usize] += 1
                                }
                            }
                            cp =
                                check_feature((*(*(*entity_ptr).mention[j as
                                    usize]).tag_ptr).f,
                                              b"NE\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !cp.is_null() {
                                strcpy(key.as_mut_ptr(), cp);
                                *strchr(key.as_mut_ptr().offset(3 as
                                    libc::c_int
                                    as isize),
                                        ':' as i32) =
                                    '\u{0}' as i32 as libc::c_char;
                                sprintf(key2.as_mut_ptr(),
                                        b"<%s>\x00" as *const u8 as
                                            *const libc::c_char,
                                        key.as_mut_ptr());
                                strcpy(*(*analysing_tag_arg_string.offset((pp
                                    -
                                    1
                                        as
                                        libc::c_int)
                                    as
                                    isize)).offset(analysing_tag_arg_num[(pp
                                    -
                                    1
                                        as
                                        libc::c_int)
                                    as
                                    usize]
                                    as
                                    isize),
                                       key2.as_mut_ptr());
                                analysing_tag_arg_num[(pp - 1 as libc::c_int)
                                    as usize] += 1
                            }
                        }
                    }
                    j += 1
                }
            }
        }
        i += 1
    }
    cp =
        check_feature((*(*analysing_tag).b_ptr).f,
                      b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() {
        strcpy(analysing_cf_entry.as_mut_ptr(),
               cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                   as *const u8 as *const libc::c_char) as
                   isize));
    }
    if (*analysing_tag).voice & 1 as libc::c_int != 0 {
        strcat(analysing_cf_entry.as_mut_ptr(),
               b":C\x00" as *const u8 as *const libc::c_char);
    } else if (*analysing_tag).voice & 2 as libc::c_int != 0 ||
        (*analysing_tag).voice & 32 as libc::c_int != 0 {
        strcat(analysing_cf_entry.as_mut_ptr(),
               b":P\x00" as *const u8 as *const libc::c_char);
    } else if (*analysing_tag).voice & 4 as libc::c_int != 0 {
        strcat(analysing_cf_entry.as_mut_ptr(),
               b":PC\x00" as *const u8 as *const libc::c_char);
    }
    mention_mgr_ptr = &mut (*(*target_mention_ptr).tag_ptr).mention_mgr;
    if !(*(*(*target_mention_ptr).tag_ptr).b_ptr).parent.is_null() &&
        analysis_flags[(*target_mention_ptr).sent_num as
            usize][(*(*(*target_mention_ptr).tag_ptr).parent).num
            as usize] == 0 as libc::c_int {
        let mut pred_bnst_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
        if (*(*(*target_mention_ptr).tag_ptr).b_ptr).parent.is_null() ||
            check_feature((*(*(*(*target_mention_ptr).tag_ptr).b_ptr).parent).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            score = 0 as libc::c_int as libc::c_double;
            current_block = 1955628783100920040;
        } else {
            target_pp =
                bnst_to_pseudo_pp((*(*target_mention_ptr).tag_ptr).b_ptr);
            if target_pp < 0 as libc::c_int {
                score = 0 as libc::c_int as libc::c_double;
                current_block = 1955628783100920040;
            } else {
                pred_bnst_ptr =
                    (*(*(*target_mention_ptr).tag_ptr).b_ptr).parent;
                cp =
                    check_feature((*pred_bnst_ptr).f,
                                  b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char);
                if !cp.is_null() {
                    strcpy(target_cf_entry.as_mut_ptr(),
                           cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                               as *const u8 as
                               *const libc::c_char) as
                               isize));
                }
                pred_tag_num = (*(*pred_bnst_ptr).tag_ptr).num;
                if (*pred_bnst_ptr).voice & 1 as libc::c_int != 0 {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":C\x00" as *const u8 as *const libc::c_char);
                } else if (*pred_bnst_ptr).voice & 2 as libc::c_int != 0 ||
                    (*pred_bnst_ptr).voice & 32 as libc::c_int != 0
                {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":P\x00" as *const u8 as *const libc::c_char);
                } else if (*pred_bnst_ptr).voice & 4 as libc::c_int != 0 {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":PC\x00" as *const u8 as *const libc::c_char);
                }
                if target_cf_entry.as_mut_ptr().is_null() ||
                    strcmp(b"\x00" as *const u8 as *const libc::c_char,
                           target_cf_entry.as_mut_ptr()) == 0 {
                    score = 0 as libc::c_int as libc::c_double;
                    current_block = 1955628783100920040;
                } else if check_swappable_pred_pair(analysing_cf_entry.as_mut_ptr(),
                                                    target_cf_entry.as_mut_ptr())
                    == 0 as libc::c_int {
                    score = 0 as libc::c_int as libc::c_double;
                    current_block = 1955628783100920040;
                } else {
                    i = 0 as libc::c_int;
                    while i < 3 as libc::c_int {
                        if i == target_pp - 1 as libc::c_int {
                            if un_lex_match == 1 as libc::c_int {
                                strcpy(*(*target_tag_arg_string.offset(i as
                                    isize)).offset(0
                                    as
                                    libc::c_int
                                    as
                                    isize),
                                       b"X\x00" as *const u8 as
                                           *const libc::c_char);
                                target_tag_arg_num[i as usize] += 1
                            }
                        } else {
                            strcpy(*(*target_tag_arg_string.offset(i as
                                isize)).offset(0
                                as
                                libc::c_int
                                as
                                isize),
                                   b"*\x00" as *const u8 as
                                       *const libc::c_char);
                            target_tag_arg_num[i as usize] += 1;
                            if un_lex_match == 1 as libc::c_int {
                                strcpy(*(*target_tag_arg_string.offset(i as
                                    isize)).offset(1
                                    as
                                    libc::c_int
                                    as
                                    isize),
                                       b"Y\x00" as *const u8 as
                                           *const libc::c_char);
                                target_tag_arg_num[i as usize] += 1
                            }
                        }
                        i += 1
                    }
                    backword_flag = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while !(*pred_bnst_ptr).child[i as usize].is_null() {
                        let mut pp_0: libc::c_int = 0 as libc::c_int;
                        if !check_feature((*(*pred_bnst_ptr).child[i as
                            usize]).f,
                                          b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            pp_0 = 1 as libc::c_int
                        } else if !check_feature((*(*pred_bnst_ptr).child[i as
                            usize]).f,
                                                 b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char).is_null()
                        {
                            pp_0 = 2 as libc::c_int
                        } else if !check_feature((*(*pred_bnst_ptr).child[i as
                            usize]).f,
                                                 b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char).is_null()
                        {
                            pp_0 = 3 as libc::c_int
                        }
                        if !(un_lex_match == 1 as libc::c_int &&
                            target_pp == pp_0) {
                            if !(pp_0 == 0 as libc::c_int) {
                                j = 0 as libc::c_int;
                                while j <
                                    (*(*pred_bnst_ptr).child[i as
                                        usize]).tag_num
                                {
                                    if !(*(*(*(*pred_bnst_ptr).child[i as
                                        usize]).tag_ptr.offset(j
                                        as
                                        isize)).mention_mgr.mention.as_mut_ptr()).entity.is_null()
                                    {
                                        entity_ptr =
                                            (*(*(*(*pred_bnst_ptr).child[i as
                                                usize]).tag_ptr.offset(j
                                                as
                                                isize)).mention_mgr.mention.as_mut_ptr()).entity
                                    }
                                    j += 1
                                }
                                j = 0 as libc::c_int;
                                while j < (*entity_ptr).mentioned_num {
                                    if !((*(*entity_ptr).mention[j as
                                        usize]).type_0
                                        as libc::c_int != 'S' as i32 &&
                                        (*(*entity_ptr).mention[j as
                                            usize]).type_0
                                            as libc::c_int != '=' as i32)
                                    {
                                        cp =
                                            get_bnst_head_canonical_rep((*(*(*entity_ptr).mention[j
                                                as
                                                usize]).tag_ptr).b_ptr,
                                                                        OptCaseFlag
                                                                            &
                                                                            512
                                                                                as
                                                                                libc::c_int);
                                        if !cp.is_null() {
                                            strcpy(*(*target_tag_arg_string.offset((pp_0
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                isize)).offset(target_tag_arg_num[(pp_0
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                usize]
                                                as
                                                isize),
                                                   cp);
                                            target_tag_arg_num[(pp_0 -
                                                1 as
                                                    libc::c_int)
                                                as usize]
                                                += 1
                                        }
                                        if un_lex_match == 0 as libc::c_int {
                                            cp =
                                                check_feature((*(*(*(*entity_ptr).mention[j
                                                    as
                                                    usize]).tag_ptr).head_ptr).f,
                                                              b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char);
                                            if !cp.is_null() {
                                                while !strchr(cp,
                                                              ':' as
                                                                  i32).is_null()
                                                    &&
                                                    {
                                                        cp =
                                                            strchr(cp,
                                                                   ':'
                                                                       as
                                                                       i32);
                                                        !cp.is_null()
                                                    } ||
                                                    {
                                                        cp =
                                                            strchr(cp,
                                                                   ';'
                                                                       as
                                                                       i32);
                                                        !cp.is_null()
                                                    } {
                                                    cp = cp.offset(1);
                                                    sprintf(key.as_mut_ptr(),
                                                            b"CT:%s:\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            cp);
                                                    if !strchr(key.as_mut_ptr().offset(3
                                                        as
                                                        libc::c_int
                                                        as
                                                        isize),
                                                               ';' as
                                                                   i32).is_null()
                                                    {
                                                        *strchr(key.as_mut_ptr().offset(3
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize),
                                                                ';' as i32) =
                                                            ':' as i32 as
                                                                libc::c_char
                                                    }
                                                    *strchr(key.as_mut_ptr().offset(3
                                                        as
                                                        libc::c_int
                                                        as
                                                        isize),
                                                            ':' as i32) =
                                                        '\u{0}' as i32 as
                                                            libc::c_char;
                                                    sprintf(key2.as_mut_ptr(),
                                                            b"<%s>\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            key.as_mut_ptr());
                                                    strcpy(*(*target_tag_arg_string.offset((pp_0
                                                        -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        isize)).offset(target_tag_arg_num[(pp_0
                                                        -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        usize]
                                                        as
                                                        isize),
                                                           key2.as_mut_ptr());
                                                    target_tag_arg_num[(pp_0 -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        usize]
                                                        += 1
                                                }
                                            }
                                            cp =
                                                check_feature((*(*(*entity_ptr).mention[j
                                                    as
                                                    usize]).tag_ptr).f,
                                                              b"NE\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char);
                                            if !cp.is_null() {
                                                strcpy(key.as_mut_ptr(), cp);
                                                *strchr(key.as_mut_ptr().offset(3
                                                    as
                                                    libc::c_int
                                                    as
                                                    isize),
                                                        ':' as i32) =
                                                    '\u{0}' as i32 as
                                                        libc::c_char;
                                                sprintf(key2.as_mut_ptr(),
                                                        b"<%s>\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        key.as_mut_ptr());
                                                strcpy(*(*target_tag_arg_string.offset((pp_0
                                                    -
                                                    1
                                                        as
                                                        libc::c_int)
                                                    as
                                                    isize)).offset(target_tag_arg_num[(pp_0
                                                    -
                                                    1
                                                        as
                                                        libc::c_int)
                                                    as
                                                    usize]
                                                    as
                                                    isize),
                                                       key2.as_mut_ptr());
                                                target_tag_arg_num[(pp_0 -
                                                    1 as
                                                        libc::c_int)
                                                    as
                                                    usize]
                                                    += 1
                                            }
                                        }
                                    }
                                    j += 1
                                }
                            }
                        }
                        i += 1
                    }
                    current_block = 7348614267943210136;
                }
            }
        }
    } else if (*(*target_mention_ptr).tag_ptr).b_ptr.is_null() {
        score = 0 as libc::c_int as libc::c_double;
        current_block = 1955628783100920040;
    } else {
        let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        if target_pp < 0 as libc::c_int {
            score = 0 as libc::c_int as libc::c_double;
            current_block = 1955628783100920040;
        } else if (*target_mention_ptr).type_0 as libc::c_int == 'S' as i32 ||
            (*target_mention_ptr).type_0 as libc::c_int ==
                '=' as i32 {
            score = 0 as libc::c_int as libc::c_double;
            current_block = 1955628783100920040;
        } else {
            cp_0 =
                check_feature((*(*(*target_mention_ptr).tag_ptr).b_ptr).f,
                              b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp_0.is_null() {
                strcpy(target_cf_entry.as_mut_ptr(),
                       cp_0.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                           as *const u8 as
                           *const libc::c_char) as isize));
            }
            if target_cf_entry.as_mut_ptr().is_null() ||
                strcmp(b"\x00" as *const u8 as *const libc::c_char,
                       target_cf_entry.as_mut_ptr()) == 0 {
                score = 0 as libc::c_int as libc::c_double;
                current_block = 1955628783100920040;
            } else {
                if (*(*target_mention_ptr).tag_ptr).voice & 1 as libc::c_int
                    != 0 {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":C\x00" as *const u8 as *const libc::c_char);
                } else if (*(*target_mention_ptr).tag_ptr).voice &
                    2 as libc::c_int != 0 ||
                    (*(*target_mention_ptr).tag_ptr).voice &
                        32 as libc::c_int != 0 {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":P\x00" as *const u8 as *const libc::c_char);
                } else if (*(*target_mention_ptr).tag_ptr).voice &
                    4 as libc::c_int != 0 {
                    strcat(target_cf_entry.as_mut_ptr(),
                           b":PC\x00" as *const u8 as *const libc::c_char);
                }
                if check_swappable_pred_pair(analysing_cf_entry.as_mut_ptr(),
                                             target_cf_entry.as_mut_ptr()) ==
                    0 as libc::c_int {
                    score = 0 as libc::c_int as libc::c_double;
                    current_block = 1955628783100920040;
                } else {
                    i = 0 as libc::c_int;
                    while i < 3 as libc::c_int {
                        if i == target_pp - 1 as libc::c_int {
                            if un_lex_match == 1 as libc::c_int {
                                strcpy(*(*target_tag_arg_string.offset(i as
                                    isize)).offset(0
                                    as
                                    libc::c_int
                                    as
                                    isize),
                                       b"X\x00" as *const u8 as
                                           *const libc::c_char);
                                target_tag_arg_num[i as usize] += 1
                            }
                        } else {
                            strcpy(*(*target_tag_arg_string.offset(i as
                                isize)).offset(0
                                as
                                libc::c_int
                                as
                                isize),
                                   b"*\x00" as *const u8 as
                                       *const libc::c_char);
                            target_tag_arg_num[i as usize] += 1;
                            if un_lex_match == 1 as libc::c_int {
                                strcpy(*(*target_tag_arg_string.offset(i as
                                    isize)).offset(1
                                    as
                                    libc::c_int
                                    as
                                    isize),
                                       b"Y\x00" as *const u8 as
                                           *const libc::c_char);
                                target_tag_arg_num[i as usize] += 1
                            }
                        }
                        i += 1
                    }
                    i = 1 as libc::c_int;
                    while i < (*mention_mgr_ptr).num {
                        let mut pp_1: libc::c_int = 0;
                        pp_1 =
                            pp_kstr_to_code((*mention_mgr_ptr).mention[i as
                                usize].cpp_string.as_mut_ptr());
                        if !(pp_1 > 3 as libc::c_int ||
                            pp_1 < 1 as libc::c_int) {
                            if !(un_lex_match == 1 as libc::c_int &&
                                target_pp == pp_1) {
                                entity_ptr =
                                    (*mention_mgr_ptr).mention[i as
                                        usize].entity;
                                j = 0 as libc::c_int;
                                while j < (*entity_ptr).mentioned_num {
                                    if !((*(*entity_ptr).mention[j as
                                        usize]).type_0
                                        as libc::c_int != 'S' as i32 &&
                                        (*(*entity_ptr).mention[j as
                                            usize]).type_0
                                            as libc::c_int != '=' as i32)
                                    {
                                        cp_0 =
                                            get_bnst_head_canonical_rep((*(*(*entity_ptr).mention[j
                                                as
                                                usize]).tag_ptr).b_ptr,
                                                                        OptCaseFlag
                                                                            &
                                                                            512
                                                                                as
                                                                                libc::c_int);
                                        if !cp_0.is_null() {
                                            strcpy(*(*target_tag_arg_string.offset((pp_1
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                isize)).offset(target_tag_arg_num[(pp_1
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                usize]
                                                as
                                                isize),
                                                   cp_0);
                                            target_tag_arg_num[(pp_1 -
                                                1 as
                                                    libc::c_int)
                                                as usize]
                                                += 1
                                        }
                                        if un_lex_match == 0 as libc::c_int {
                                            cp_0 =
                                                check_feature((*(*(*(*entity_ptr).mention[j
                                                    as
                                                    usize]).tag_ptr).head_ptr).f,
                                                              b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char);
                                            if !cp_0.is_null() {
                                                while !strchr(cp_0,
                                                              ':' as
                                                                  i32).is_null()
                                                    &&
                                                    {
                                                        cp_0 =
                                                            strchr(cp_0,
                                                                   ':'
                                                                       as
                                                                       i32);
                                                        !cp_0.is_null()
                                                    } ||
                                                    {
                                                        cp_0 =
                                                            strchr(cp_0,
                                                                   ';'
                                                                       as
                                                                       i32);
                                                        !cp_0.is_null()
                                                    } {
                                                    cp_0 = cp_0.offset(1);
                                                    sprintf(key.as_mut_ptr(),
                                                            b"CT:%s:\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            cp_0);
                                                    if !strchr(key.as_mut_ptr().offset(3
                                                        as
                                                        libc::c_int
                                                        as
                                                        isize),
                                                               ';' as
                                                                   i32).is_null()
                                                    {
                                                        *strchr(key.as_mut_ptr().offset(3
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize),
                                                                ';' as i32) =
                                                            ':' as i32 as
                                                                libc::c_char
                                                    }
                                                    *strchr(key.as_mut_ptr().offset(3
                                                        as
                                                        libc::c_int
                                                        as
                                                        isize),
                                                            ':' as i32) =
                                                        '\u{0}' as i32 as
                                                            libc::c_char;
                                                    sprintf(key2.as_mut_ptr(),
                                                            b"<%s>\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            key.as_mut_ptr());
                                                    strcpy(*(*target_tag_arg_string.offset((pp_1
                                                        -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        isize)).offset(target_tag_arg_num[(pp_1
                                                        -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        usize]
                                                        as
                                                        isize),
                                                           key2.as_mut_ptr());
                                                    target_tag_arg_num[(pp_1 -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        usize]
                                                        += 1
                                                }
                                            }
                                            cp_0 =
                                                check_feature((*(*(*entity_ptr).mention[j
                                                    as
                                                    usize]).tag_ptr).f,
                                                              b"NE\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char);
                                            if !cp_0.is_null() {
                                                strcpy(key.as_mut_ptr(),
                                                       cp_0);
                                                *strchr(key.as_mut_ptr().offset(3
                                                    as
                                                    libc::c_int
                                                    as
                                                    isize),
                                                        ':' as i32) =
                                                    '\u{0}' as i32 as
                                                        libc::c_char;
                                                sprintf(key2.as_mut_ptr(),
                                                        b"<%s>\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        key.as_mut_ptr());
                                                strcpy(*(*target_tag_arg_string.offset((pp_1
                                                    -
                                                    1
                                                        as
                                                        libc::c_int)
                                                    as
                                                    isize)).offset(target_tag_arg_num[(pp_1
                                                    -
                                                    1
                                                        as
                                                        libc::c_int)
                                                    as
                                                    usize]
                                                    as
                                                    isize),
                                                       key2.as_mut_ptr());
                                                target_tag_arg_num[(pp_1 -
                                                    1 as
                                                        libc::c_int)
                                                    as
                                                    usize]
                                                    += 1
                                            }
                                        }
                                    }
                                    j += 1
                                }
                            }
                        }
                        i += 1
                    }
                    current_block = 7348614267943210136;
                }
            }
        }
    }
    match current_block {
        7348614267943210136 => {
            analysing_ga_mention_idx = 0 as libc::c_int;
            while analysing_ga_mention_idx <
                analysing_tag_arg_num[0 as libc::c_int as usize] {
                analysing_wo_mention_idx = 0 as libc::c_int;
                while analysing_wo_mention_idx <
                    analysing_tag_arg_num[1 as libc::c_int as usize] {
                    analysing_ni_mention_idx = 0 as libc::c_int;
                    while analysing_ni_mention_idx <
                        analysing_tag_arg_num[2 as libc::c_int as usize]
                    {
                        target_ga_mention_idx = 0 as libc::c_int;
                        while target_ga_mention_idx <
                            target_tag_arg_num[0 as libc::c_int as
                                usize] {
                            target_wo_mention_idx = 0 as libc::c_int;
                            while target_wo_mention_idx <
                                target_tag_arg_num[1 as libc::c_int as
                                    usize] {
                                target_ni_mention_idx = 0 as libc::c_int;
                                while target_ni_mention_idx <
                                    target_tag_arg_num[2 as libc::c_int
                                        as usize] {
                                    // let mut pa_pair_string: [libc::c_char; 1563] = [0; 1563];
                                    let mut lift_value: libc::c_double = 0.;
                                    let mut analysing_tag_pa_string:
                                        [libc::c_char; 780] = [0; 780];
                                    let mut target_tag_pa_string:
                                        [libc::c_char; 780] = [0; 780];
                                    let mut temp_sequence: libc::c_int =
                                        0 as libc::c_int;
                                    make_tag_pa_string(analysing_tag_pa_string.as_mut_ptr(),
                                                       *(*analysing_tag_arg_string.offset(0
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(analysing_ga_mention_idx
                                                           as
                                                           isize),
                                                       *(*analysing_tag_arg_string.offset(1
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(analysing_wo_mention_idx
                                                           as
                                                           isize),
                                                       *(*analysing_tag_arg_string.offset(2
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(analysing_ni_mention_idx
                                                           as
                                                           isize),
                                                       analysing_cf_entry.as_mut_ptr());
                                    make_tag_pa_string(target_tag_pa_string.as_mut_ptr(),
                                                       *(*target_tag_arg_string.offset(0
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(target_ga_mention_idx
                                                           as
                                                           isize),
                                                       *(*target_tag_arg_string.offset(1
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(target_wo_mention_idx
                                                           as
                                                           isize),
                                                       *(*target_tag_arg_string.offset(2
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)).offset(target_ni_mention_idx
                                                           as
                                                           isize),
                                                       target_cf_entry.as_mut_ptr());
                                    lift_value =
                                        get_swappable_lift_value(analysing_tag_pa_string.as_mut_ptr(),
                                                                 target_tag_pa_string.as_mut_ptr(),
                                                                 temp_match_string.as_mut_ptr(),
                                                                 &mut temp_sequence);
                                    if lift_value > score {
                                        strcpy(match_string.as_mut_ptr(),
                                               temp_match_string.as_mut_ptr());
                                        sequence = temp_sequence;
                                        score = lift_value
                                    }
                                    if un_lex_match == 1 as libc::c_int {
                                        if strcmp(*(*analysing_tag_arg_string.offset(0
                                            as
                                            libc::c_int
                                            as
                                            isize)).offset(analysing_ga_mention_idx
                                            as
                                            isize),
                                                  b"*\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                            0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(0
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_ga_mention_idx
                                                as
                                                isize),
                                                   b"X\x00" as *const u8 as
                                                       *const libc::c_char)
                                                != 0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(0
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_ga_mention_idx
                                                as
                                                isize),
                                                   *(*target_tag_arg_string.offset(0
                                                       as
                                                       libc::c_int
                                                       as
                                                       isize)).offset(target_ga_mention_idx
                                                       as
                                                       isize))
                                                == 0 {
                                            make_tag_pa_string(analysing_tag_pa_string.as_mut_ptr(),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               *(*analysing_tag_arg_string.offset(1
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_wo_mention_idx
                                                                   as
                                                                   isize),
                                                               *(*analysing_tag_arg_string.offset(2
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_ni_mention_idx
                                                                   as
                                                                   isize),
                                                               analysing_cf_entry.as_mut_ptr());
                                            make_tag_pa_string(target_tag_pa_string.as_mut_ptr(),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               *(*target_tag_arg_string.offset(1
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_wo_mention_idx
                                                                   as
                                                                   isize),
                                                               *(*target_tag_arg_string.offset(2
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_ni_mention_idx
                                                                   as
                                                                   isize),
                                                               target_cf_entry.as_mut_ptr());
                                            lift_value =
                                                get_swappable_lift_value(analysing_tag_pa_string.as_mut_ptr(),
                                                                         target_tag_pa_string.as_mut_ptr(),
                                                                         temp_match_string.as_mut_ptr(),
                                                                         &mut temp_sequence);
                                            if lift_value > score {
                                                strcpy(match_string.as_mut_ptr(),
                                                       temp_match_string.as_mut_ptr());
                                                score = lift_value;
                                                sequence = temp_sequence
                                            }
                                        }
                                        if strcmp(*(*analysing_tag_arg_string.offset(1
                                            as
                                            libc::c_int
                                            as
                                            isize)).offset(analysing_wo_mention_idx
                                            as
                                            isize),
                                                  b"*\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                            0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(1
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_wo_mention_idx
                                                as
                                                isize),
                                                   b"X\x00" as *const u8 as
                                                       *const libc::c_char)
                                                != 0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(1
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_wo_mention_idx
                                                as
                                                isize),
                                                   *(*target_tag_arg_string.offset(1
                                                       as
                                                       libc::c_int
                                                       as
                                                       isize)).offset(target_wo_mention_idx
                                                       as
                                                       isize))
                                                == 0 {
                                            make_tag_pa_string(analysing_tag_pa_string.as_mut_ptr(),
                                                               *(*analysing_tag_arg_string.offset(0
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_ga_mention_idx
                                                                   as
                                                                   isize),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               *(*analysing_tag_arg_string.offset(2
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_ni_mention_idx
                                                                   as
                                                                   isize),
                                                               analysing_cf_entry.as_mut_ptr());
                                            make_tag_pa_string(target_tag_pa_string.as_mut_ptr(),
                                                               *(*target_tag_arg_string.offset(0
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_ga_mention_idx
                                                                   as
                                                                   isize),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               *(*target_tag_arg_string.offset(2
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_ni_mention_idx
                                                                   as
                                                                   isize),
                                                               target_cf_entry.as_mut_ptr());
                                            lift_value =
                                                get_swappable_lift_value(analysing_tag_pa_string.as_mut_ptr(),
                                                                         target_tag_pa_string.as_mut_ptr(),
                                                                         temp_match_string.as_mut_ptr(),
                                                                         &mut temp_sequence);
                                            if lift_value > score {
                                                strcpy(match_string.as_mut_ptr(),
                                                       temp_match_string.as_mut_ptr());
                                                score = lift_value;
                                                sequence = temp_sequence
                                            }
                                        }
                                        if strcmp(*(*analysing_tag_arg_string.offset(2
                                            as
                                            libc::c_int
                                            as
                                            isize)).offset(analysing_ni_mention_idx
                                            as
                                            isize),
                                                  b"*\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                            0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(2
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_ni_mention_idx
                                                as
                                                isize),
                                                   b"X\x00" as *const u8 as
                                                       *const libc::c_char)
                                                != 0 &&
                                            strcmp(*(*analysing_tag_arg_string.offset(2
                                                as
                                                libc::c_int
                                                as
                                                isize)).offset(analysing_ni_mention_idx
                                                as
                                                isize),
                                                   *(*target_tag_arg_string.offset(2
                                                       as
                                                       libc::c_int
                                                       as
                                                       isize)).offset(target_ni_mention_idx
                                                       as
                                                       isize))
                                                == 0 {
                                            make_tag_pa_string(analysing_tag_pa_string.as_mut_ptr(),
                                                               *(*analysing_tag_arg_string.offset(0
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_ga_mention_idx
                                                                   as
                                                                   isize),
                                                               *(*analysing_tag_arg_string.offset(1
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(analysing_wo_mention_idx
                                                                   as
                                                                   isize),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               analysing_cf_entry.as_mut_ptr());
                                            make_tag_pa_string(target_tag_pa_string.as_mut_ptr(),
                                                               *(*target_tag_arg_string.offset(0
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_ga_mention_idx
                                                                   as
                                                                   isize),
                                                               *(*target_tag_arg_string.offset(1
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(target_wo_mention_idx
                                                                   as
                                                                   isize),
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               target_cf_entry.as_mut_ptr());
                                            lift_value =
                                                get_swappable_lift_value(analysing_tag_pa_string.as_mut_ptr(),
                                                                         target_tag_pa_string.as_mut_ptr(),
                                                                         temp_match_string.as_mut_ptr(),
                                                                         &mut temp_sequence);
                                            if lift_value > score {
                                                strcpy(match_string.as_mut_ptr(),
                                                       temp_match_string.as_mut_ptr());
                                                score = lift_value;
                                                sequence = temp_sequence
                                            }
                                        }
                                    }
                                    target_ni_mention_idx += 1
                                }
                                target_wo_mention_idx += 1
                            }
                            target_ga_mention_idx += 1
                        }
                        analysing_ni_mention_idx += 1
                    }
                    analysing_wo_mention_idx += 1
                }
                analysing_ga_mention_idx += 1
            }
        }
        _ => {}
    }
    if score > 0 as libc::c_int as libc::c_double {
        let mut seq: libc::c_int = 0;
        if pred_tag_num == -(1 as libc::c_int) {
            pred_tag_num = (*(*target_mention_ptr).tag_ptr).num
        }
        if sent_num > (*target_mention_ptr).sent_num {
            seq = sequence
        } else if sent_num < (*target_mention_ptr).sent_num {
            seq = sequence * -(1 as libc::c_int)
        } else if tag_num > pred_tag_num {
            seq = sequence
        } else { seq = sequence * -(1 as libc::c_int) }
        printf(b"lift%d %s %s %f sen1:%d tag1:%d sen2:%d tag2:%d\n\x00" as
                   *const u8 as *const libc::c_char, seq, loc_name,
               match_string.as_mut_ptr(), score, sent_num, tag_num,
               (*target_mention_ptr).sent_num, pred_tag_num);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int + 2 as libc::c_int {
            free(*(*analysing_tag_arg_string.offset(i as
                isize)).offset(j as
                isize)
                as *mut libc::c_void);
            j += 1
        }
        free(*analysing_tag_arg_string.offset(i as isize) as
            *mut libc::c_void);
        i += 1
    }
    free(analysing_tag_arg_string as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int + 2 as libc::c_int {
            free(*(*target_tag_arg_string.offset(i as
                isize)).offset(j as
                isize)
                as *mut libc::c_void);
            j += 1
        }
        free(*target_tag_arg_string.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(target_tag_arg_string as *mut libc::c_void);
    return score;
}

#[no_mangle]
pub unsafe extern "C" fn set_bnst_cueue(mut analysis_bnst_cueue:
                                        *mut libc::c_int,
                                        mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bnst_num: libc::c_int = 0;
    // let mut tag_num: libc::c_int = 0;
    let mut check_bnst: [libc::c_int; 200] = [0; 200];
    // let mut check_tag: [libc::c_int; 200] = [0; 200];
    let mut set_idx: libc::c_int = 0 as libc::c_int;
    let mut bnst_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        *analysis_bnst_cueue.offset(i as isize) = -(1 as libc::c_int);
        i += 1
    }
    bnst_num = (*sp).Bnst_num - 1 as libc::c_int;
    while bnst_num >= 0 as libc::c_int {
        check_bnst[bnst_num as usize] = 0 as libc::c_int;
        bnst_num -= 1
    }
    bnst_num = (*sp).Bnst_num - 1 as libc::c_int;
    while bnst_num >= 0 as libc::c_int {
        let mut check_flag: libc::c_int = 0 as libc::c_int;
        bnst_ptr = (*sp).bnst_data.offset(bnst_num as isize);
        i = 0 as libc::c_int;
        while !(*bnst_ptr).child[i as usize].is_null() {
            if (*(*bnst_ptr).child[i as usize]).para_top_p != 0 {
                j = 0 as libc::c_int;
                while !(*(*bnst_ptr).child[i as
                    usize]).child[j as
                    usize].is_null()
                {
                    if (*(*(*bnst_ptr).child[i as
                        usize]).child[j as
                        usize]).para_top_p
                        == 0 {
                        if !check_feature((*(*(*bnst_ptr).child[i as
                            usize]).child[j
                            as
                            usize]).f,
                                          b"\xe3\x83\x8f\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            check_flag = 1 as libc::c_int
                        }
                    }
                    j += 1
                }
            } else if !check_feature((*(*bnst_ptr).child[i as usize]).f,
                                     b"\xe3\x83\x8f\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                check_flag = 1 as libc::c_int
            }
            i += 1
        }
        if check_flag == 1 as libc::c_int {
            check_bnst[(*bnst_ptr).num as usize] = 1 as libc::c_int;
            *analysis_bnst_cueue.offset(set_idx as isize) = (*bnst_ptr).num;
            set_idx += 1
        }
        bnst_num -= 1
    }
    bnst_num = (*sp).Bnst_num - 1 as libc::c_int;
    while bnst_num >= 0 as libc::c_int {
        bnst_ptr = (*sp).bnst_data.offset(bnst_num as isize);
        if !(check_bnst[(*bnst_ptr).num as usize] == 1 as libc::c_int) {
            check_bnst[(*bnst_ptr).num as usize] = 1 as libc::c_int;
            *analysis_bnst_cueue.offset(set_idx as isize) = (*bnst_ptr).num;
            set_idx += 1
        }
        bnst_num -= 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_cf_entry_type(mut cf_entry_type:
                                            *mut libc::c_char,
                                            mut cf_ptr: *mut CASE_FRAME) {
    let mut cf_type: libc::c_char = 0;
    strcpy(cf_entry_type, (*cf_ptr).entry);
    cf_type =
        *(*cf_ptr).cf_id.as_mut_ptr().offset(strlen((*cf_ptr).entry) as
            isize).offset(1 as
            libc::c_int
            as
            isize).offset(strlen((*cf_ptr).pred_type.as_mut_ptr())
            as
            isize).offset(1
            as
            libc::c_int
            as
            isize);
    if cf_type as libc::c_int == 'C' as i32 ||
        cf_type as libc::c_int == 'P' as i32 {
        strncat(cf_entry_type, b":\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong);
        strncat(cf_entry_type, &mut cf_type,
                1 as libc::c_int as libc::c_ulong);
    };
}

#[no_mangle]
pub unsafe extern "C" fn merge_ellipsis_result_and_correct() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp_result_ctm: *mut CF_TAG_MGR = 0 as *mut CF_TAG_MGR;
    let mut correct_cf_aresult_strings: [[libc::c_char; 256]; 3] =
        [[0; 256]; 3];
    let mut correct_flag: libc::c_int = 1 as libc::c_int;
    // let mut initital_ctm_flag: libc::c_int = 0 as libc::c_int;
    temp_result_ctm =
        malloc_data((::std::mem::size_of::<CF_TAG_MGR>() as
            libc::c_ulong).wrapping_mul(10 as libc::c_int as
            libc::c_ulong),
                    b"reordering_ellipsis_result\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut CF_TAG_MGR;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        copy_ctm(&mut *ellipsis_result_ctm.as_mut_ptr().offset(i as isize),
                 &mut *temp_result_ctm.offset(i as isize));
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        strcpy(correct_cf_aresult_strings[i as usize].as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        let mut temp_cf_aresult_string: [libc::c_char; 256] = [0; 256];
        let mut aresult: [libc::c_char; 256] = [0; 256];
        if correct_flag == 1 as libc::c_int {
            copy_ctm(&mut *ellipsis_correct_ctm.as_mut_ptr().offset(j as
                isize),
                     &mut *ellipsis_result_ctm.as_mut_ptr().offset(i as
                         isize));
            make_aresult_string(&mut *ellipsis_result_ctm.as_mut_ptr().offset(i
                as
                isize),
                                aresult.as_mut_ptr());
            strcpy(temp_cf_aresult_string.as_mut_ptr(),
                   (*ellipsis_result_ctm[i as
                       usize].cf_ptr).cf_id.as_mut_ptr());
            strcat(temp_cf_aresult_string.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
            strcat(temp_cf_aresult_string.as_mut_ptr(), aresult.as_mut_ptr());
            strcpy(correct_cf_aresult_strings[j as usize].as_mut_ptr(),
                   temp_cf_aresult_string.as_mut_ptr());
            j += 1;
            if j >= 3 as libc::c_int ||
                ellipsis_correct_ctm[j as usize].score ==
                    -(10000 as libc::c_int) as libc::c_double {
                correct_flag = 0 as libc::c_int;
                j = 0 as libc::c_int
            }
        } else if correct_flag == 0 as libc::c_int {
            let mut copy_flag: libc::c_int = 1 as libc::c_int;
            if (*temp_result_ctm.offset(j as isize)).score !=
                -(10000 as libc::c_int) as libc::c_double {
                make_aresult_string(&mut *temp_result_ctm.offset(j as isize),
                                    aresult.as_mut_ptr());
                strcpy(temp_cf_aresult_string.as_mut_ptr(),
                       (*(*temp_result_ctm.offset(j as
                           isize)).cf_ptr).cf_id.as_mut_ptr());
                strcat(temp_cf_aresult_string.as_mut_ptr(),
                       b" \x00" as *const u8 as *const libc::c_char);
                strcat(temp_cf_aresult_string.as_mut_ptr(),
                       aresult.as_mut_ptr());
                k = 0 as libc::c_int;
                while k < 3 as libc::c_int {
                    if strcmp(temp_cf_aresult_string.as_mut_ptr(),
                              correct_cf_aresult_strings[k as
                                  usize].as_mut_ptr())
                        == 0 {
                        copy_flag = 0 as libc::c_int
                    }
                    k += 1
                }
            }
            if copy_flag == 1 as libc::c_int {
                copy_ctm(&mut *temp_result_ctm.offset(j as isize),
                         &mut *ellipsis_result_ctm.as_mut_ptr().offset(i as
                             isize));
                if j == 10 as libc::c_int - 1 as libc::c_int { break; }
            } else { i -= 1 }
            j += 1
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_aresult_string(mut ctm_ptr: *mut CF_TAG_MGR,
                                             mut aresult: *mut libc::c_char) {
    let mut pp_code: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut entity_num: [libc::c_int; 4096] = [0; 4096];
    let mut mention_num: libc::c_int = 0;
    let mut cp: [libc::c_char; 256] = [0; 256];
    *aresult.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    pp_code = 0 as libc::c_int;
    while pp_code < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            entity_num[i as usize] = 4096 as libc::c_int;
            i += 1
        }
        mention_num = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*ctm_ptr).result_num {
            if strcmp(ELLIPSIS_CASE_LIST_VERB[pp_code as usize],
                      pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[(*ctm_ptr).cf_element_num[j
                          as
                          usize]
                          as
                          usize][0 as
                          libc::c_int
                          as
                          usize]))
                == 0 {
                if mention_num == 0 as libc::c_int {
                    entity_num[0 as libc::c_int as usize] =
                        (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                            as
                            usize]
                            as
                            isize)).num;
                    mention_num += 1
                } else {
                    i = 0 as libc::c_int;
                    while i < mention_num {
                        if (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                            as
                            usize]
                            as
                            isize)).num
                            < entity_num[i as usize] {
                            k = mention_num;
                            while k > i {
                                entity_num[k as usize] =
                                    entity_num[(k - 1 as libc::c_int) as
                                        usize];
                                k -= 1
                            }
                            entity_num[i as usize] =
                                (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                                    as
                                    usize]
                                    as
                                    isize)).num;
                            mention_num += 1;
                            break;
                        } else { i += 1 }
                    }
                }
            }
            j += 1
        }
        let mut current_block_27: u64;
        i = 0 as libc::c_int;
        while i < mention_num {
            if entity_num[i as usize] == 4096 as libc::c_int { break; }
            if OptAnaphora & 32 as libc::c_int == 0 &&
                (OptAnaphora & 16 as libc::c_int != 0 ||
                    OptAnaphora & 64 as libc::c_int != 0) {
                if (*entity_manager.entity.as_mut_ptr().offset(entity_num[i as
                    usize]
                    as
                    isize)).hypothetical_flag
                    == 1 as libc::c_int {
                    current_block_27 = 11636175345244025579;
                } else { current_block_27 = 6417057564578538666; }
            } else { current_block_27 = 6417057564578538666; }
            match current_block_27 {
                6417057564578538666 => {
                    sprintf(cp.as_mut_ptr(),
                            b" %s:%d\x00" as *const u8 as *const libc::c_char,
                            ELLIPSIS_CASE_LIST_VERB[pp_code as usize],
                            entity_num[i as usize]);
                    strcat(aresult, cp.as_mut_ptr());
                }
                _ => {}
            }
            i += 1
        }
        pp_code += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_case_assingment_string(mut ctm_ptr:
                                                     *mut CF_TAG_MGR,
                                                     mut aresult:
                                                     *mut libc::c_char) {
    let mut pp_code: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut entity_num: [libc::c_int; 4096] = [0; 4096];
    let mut mention_num: libc::c_int = 0;
    let mut cp: [libc::c_char; 256] = [0; 256];
    *aresult.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    pp_code = 0 as libc::c_int;
    while pp_code < 44 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            entity_num[i as usize] = 4096 as libc::c_int;
            i += 1
        }
        mention_num = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*ctm_ptr).result_num {
            if pp_code ==
                (*(*ctm_ptr).cf_ptr).pp[(*ctm_ptr).cf_element_num[j as
                    usize]
                    as
                    usize][0 as libc::c_int as
                    usize] {
                if mention_num == 0 as libc::c_int {
                    entity_num[0 as libc::c_int as usize] =
                        (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                            as
                            usize]
                            as
                            isize)).num;
                    mention_num += 1
                } else {
                    i = 0 as libc::c_int;
                    while i < mention_num {
                        if (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                            as
                            usize]
                            as
                            isize)).num
                            < entity_num[i as usize] {
                            k = mention_num;
                            while k > i {
                                entity_num[k as usize] =
                                    entity_num[(k - 1 as libc::c_int) as
                                        usize];
                                k -= 1
                            }
                            entity_num[i as usize] =
                                (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[j
                                    as
                                    usize]
                                    as
                                    isize)).num;
                            mention_num += 1;
                            break;
                        } else { i += 1 }
                    }
                }
            }
            j += 1
        }
        let mut current_block_27: u64;
        i = 0 as libc::c_int;
        while i < mention_num {
            if entity_num[i as usize] == 4096 as libc::c_int { break; }
            if OptAnaphora & 32 as libc::c_int == 0 &&
                (OptAnaphora & 16 as libc::c_int != 0 ||
                    OptAnaphora & 64 as libc::c_int != 0) {
                if entity_num[i as usize] < 5 as libc::c_int {
                    current_block_27 = 11636175345244025579;
                } else { current_block_27 = 6417057564578538666; }
            } else { current_block_27 = 6417057564578538666; }
            match current_block_27 {
                6417057564578538666 => {
                    sprintf(cp.as_mut_ptr(),
                            b" %s:%d\x00" as *const u8 as *const libc::c_char,
                            pp_code_to_kstr(pp_code), entity_num[i as usize]);
                    strcat(aresult, cp.as_mut_ptr());
                }
                _ => {}
            }
            i += 1
        }
        pp_code += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_gresult_strings(mut tag_ptr: *mut TAG_DATA,
                                              mut gresult:
                                              *mut libc::c_char) {
    let mut pp_code: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut entity_num: [libc::c_int; 4096] = [0; 4096];
    let mut mention_num: libc::c_int = 0;
    let mut cp: [libc::c_char; 256] = [0; 256];
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    *gresult.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    pp_code = 0 as libc::c_int;
    while pp_code < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            entity_num[i as usize] = 4096 as libc::c_int;
            i += 1
        }
        mention_num = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j < (*tag_ptr).mention_mgr.num {
            mention_ptr =
                (*tag_ptr).mention_mgr.mention.as_mut_ptr().offset(j as
                    isize);
            if (*mention_ptr).type_0 as libc::c_int == 'O' as i32 ||
                (*mention_ptr).type_0 as libc::c_int == 'E' as i32 ||
                (*mention_ptr).type_0 as libc::c_int == 'C' as i32 {
                if strcmp(ELLIPSIS_CASE_LIST_VERB[pp_code as usize],
                          (*mention_ptr).cpp_string.as_mut_ptr()) == 0 {
                    if mention_num == 0 as libc::c_int {
                        entity_num[0 as libc::c_int as usize] =
                            (*(*mention_ptr).entity).num;
                        mention_num += 1
                    } else {
                        i = 0 as libc::c_int;
                        while i <= mention_num {
                            if (*(*mention_ptr).entity).num <
                                entity_num[i as usize] {
                                k = mention_num;
                                while k > i {
                                    entity_num[k as usize] =
                                        entity_num[(k - 1 as libc::c_int) as
                                            usize];
                                    k -= 1
                                }
                                entity_num[i as usize] =
                                    (*(*mention_ptr).entity).num;
                                mention_num += 1;
                                break;
                            } else { i += 1 }
                        }
                    }
                }
            }
            j += 1
        }
        let mut current_block_30: u64;
        i = 0 as libc::c_int;
        while i < mention_num {
            if entity_num[i as usize] == 4096 as libc::c_int { break; }
            if OptAnaphora & 32 as libc::c_int == 0 &&
                (OptAnaphora & 16 as libc::c_int != 0 ||
                    OptAnaphora & 64 as libc::c_int != 0) {
                if entity_num[i as usize] < 5 as libc::c_int {
                    current_block_30 = 9853141518545631134;
                } else { current_block_30 = 3934796541983872331; }
            } else { current_block_30 = 3934796541983872331; }
            match current_block_30 {
                3934796541983872331 => {
                    sprintf(cp.as_mut_ptr(),
                            b" %s:%d\x00" as *const u8 as *const libc::c_char,
                            ELLIPSIS_CASE_LIST_VERB[pp_code as usize],
                            entity_num[i as usize]);
                    strcat(gresult, cp.as_mut_ptr());
                }
                _ => {}
            }
            i += 1
        }
        pp_code += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_param() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    def_overt_arguments_weight = 1 as libc::c_int as libc::c_double;
    def_all_arguments_weight = 1 as libc::c_int as libc::c_double;
    if OptAnaphora & 16 as libc::c_int != 0 {
        overt_arguments_weight = def_overt_arguments_weight;
        all_arguments_weight = def_all_arguments_weight
    } else {
        overt_arguments_weight = learned_overt_arguments_weight;
        all_arguments_weight = learned_all_arguments_weight
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j <
            2 as libc::c_int *
                ((4 as libc::c_int +
                    (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int +
                    5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int +
                    4 as libc::c_int + 10 as libc::c_int) *
                    (3 as libc::c_int + 5 as libc::c_int)) {
            if j %
                (4 as libc::c_int + (33 as libc::c_int + 5 as libc::c_int)
                    + 3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int + 5 as libc::c_int
                    + 11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int + 4 as libc::c_int
                    + 10 as libc::c_int) == 0 as libc::c_int {
                def_case_feature_weight[i as usize][j as usize] =
                    1 as libc::c_int as libc::c_double
            } else if j %
                (4 as libc::c_int +
                    (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int +
                    5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int +
                    4 as libc::c_int + 10 as libc::c_int) <=
                4 as libc::c_int {
                def_case_feature_weight[i as usize][j as usize] = 0.5f64
            } else if j %
                (4 as libc::c_int +
                    (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int +
                    5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int +
                    4 as libc::c_int + 10 as libc::c_int) ==
                9 as libc::c_int {
                if i == 0 as libc::c_int {
                    def_case_feature_weight[i as usize][j as usize] = -1.8f64
                } else {
                    def_case_feature_weight[i as usize][j as usize] = -1.4f64
                }
            } else {
                def_case_feature_weight[i as usize][j as usize] =
                    0 as libc::c_int as libc::c_double
            }
            if OptAnaphora & 16 as libc::c_int != 0 {
                case_feature_weight[i as usize][j as usize] =
                    def_case_feature_weight[i as usize][j as usize]
            } else {
                case_feature_weight[i as usize][j as usize] =
                    learned_case_feature_weight[i as usize][j as usize]
            }
            j += 1
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn link_hypo_enity_after_analysis(mut sp:
                                                        *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut mention_mgr: *mut MENTION_MGR = 0 as *mut MENTION_MGR;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        tag_ptr = substance_tag_ptr((*sp).tag_data.offset(i as isize));
        mention_mgr = &mut (*tag_ptr).mention_mgr;
        if OptAnaphora & 32 as libc::c_int != 0 &&
            (OptReadFeature & 32 as libc::c_int != 0 ||
                OptReadFeature & 128 as libc::c_int != 0) ||
            OptAnaphora & 16 as libc::c_int != 0 ||
            OptAnaphora & 64 as libc::c_int != 0 {
            cp =
                check_feature((*tag_ptr).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                link_hypothetical_entity(cp,
                                         (*(*mention_mgr).mention.as_mut_ptr()).entity);
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_entity_from_coreference(mut sp:
                                                      *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut mention_mgr: *mut MENTION_MGR = 0 as *mut MENTION_MGR;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        tag_ptr = substance_tag_ptr((*sp).tag_data.offset(i as isize));
        mention_mgr = &mut (*tag_ptr).mention_mgr;
        (*(*mention_mgr).mention.as_mut_ptr()).tag_num = i;
        (*(*mention_mgr).mention.as_mut_ptr()).sent_num = (*sp).Sen_num;
        let ref mut fresh5 = (*(*mention_mgr).mention.as_mut_ptr()).tag_ptr;
        *fresh5 = tag_ptr;
        let ref mut fresh6 = (*(*mention_mgr).mention.as_mut_ptr()).entity;
        *fresh6 = 0 as *mut entity;
        let ref mut fresh7 =
            (*(*mention_mgr).mention.as_mut_ptr()).explicit_mention;
        *fresh7 = 0 as *mut mention;
        (*(*mention_mgr).mention.as_mut_ptr()).salience_score =
            0 as libc::c_int as libc::c_double;
        (*mention_mgr).num = 1 as libc::c_int;
        if OptReadFeature & 8 as libc::c_int != 0 {
            cp =
                check_feature((*tag_ptr).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                cp =
                    strchr(cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                        as *const u8 as
                        *const libc::c_char) as
                        isize),
                           ':' as i32).offset(1 as libc::c_int as isize);
                while *cp != 0 {
                    if *cp as libc::c_int == ':' as i32 ||
                        *cp as libc::c_int == ';' as i32 {
                        if read_one_annotation(sp, tag_ptr,
                                               cp.offset(1 as libc::c_int as
                                                   isize),
                                               (0 as libc::c_int == 0) as
                                                   libc::c_int) != 0 {
                            assign_cfeature(&mut (*tag_ptr).f,
                                            b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                                as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char,
                                            0 as libc::c_int);
                        }
                        (OptAnaphora & 16 as libc::c_int) == 0;
                    }
                    cp = cp.offset(1)
                }
            }
        } else if OptReadFeature & 64 as libc::c_int != 0 {
            cp =
                check_feature((*tag_ptr).f,
                              b"COREFER_ID\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
            if !cp.is_null() { link_entity_from_corefer_id(sp, tag_ptr, cp); }
        } else {
            cp =
                check_feature((*tag_ptr).f,
                              b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                read_one_annotation(sp, tag_ptr,
                                    cp.offset(strlen(b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7:\x00"
                                        as *const u8 as
                                        *const libc::c_char)
                                        as isize),
                                    (0 as libc::c_int == 0) as libc::c_int);
            }
        }
        if (*(*mention_mgr).mention.as_mut_ptr()).entity.is_null() {
            make_new_entity(tag_ptr, mention_mgr);
        }
        if OptAnaphora & 32 as libc::c_int != 0 &&
            OptReadFeature & 8 as libc::c_int != 0 &&
            OptAnaphora & 8192 as libc::c_int == 0 ||
            OptAnaphora & 16 as libc::c_int != 0 ||
            OptAnaphora & 64 as libc::c_int != 0 {
            cp =
                check_feature((*tag_ptr).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                link_hypothetical_entity(cp,
                                         (*(*mention_mgr).mention.as_mut_ptr()).entity);
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn clear_context(mut sp: *mut SENTENCE_DATA,
                                       mut init_flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    if OptAnaphora & 2 as libc::c_int != 0 {
        printf(b";;\n;;CONTEXT INITIALIZED\n\x00" as *const u8 as
            *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < (*sp).Sen_num - 1 as libc::c_int {
        ClearSentence(sentence_data.as_mut_ptr().offset(i as isize));
        i += 1
    }
    if init_flag != 0 {
        base_entity_num = 0 as libc::c_int;
        base_sentence_num = base_entity_num;
        corefer_id = 0 as libc::c_int
    } else {
        base_sentence_num += (*sp).Sen_num - 1 as libc::c_int;
        base_entity_num += entity_manager.num
    }
    (*sp).Sen_num = 1 as libc::c_int;
    entity_manager.num = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn match_ellipsis_case(mut key: *mut libc::c_char,
                                             mut list: *mut *mut libc::c_char)
                                             -> libc::c_int {
    let mut i: libc::c_int = 0;
    if list.is_null() { list = ELLIPSIS_CASE_LIST }
    i = 0 as libc::c_int;
    while **list.offset(i as isize) != 0 {
        if strcmp(key, *list.offset(i as isize)) == 0 {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_ellipsis_case_num(mut key: *mut libc::c_char,
                                               mut list:
                                               *mut *mut libc::c_char)
                                               -> libc::c_int {
    let mut i: libc::c_int = 0;
    if list.is_null() { list = ELLIPSIS_CASE_LIST }
    i = 0 as libc::c_int;
    while **list.offset(i as isize) != 0 {
        if strcmp(key, *list.offset(i as isize)) == 0 { return i; }
        i += 1
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn assign_mrph_num(mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        count =
            (count as
                libc::c_ulong).wrapping_add(strlen((*(*sp).mrph_data.offset(i
                as
                isize)).Goi2.as_mut_ptr()).wrapping_div(2
                as
                libc::c_int
                as
                libc::c_ulong))
                as libc::c_int as libc::c_int;
        (*(*sp).mrph_data.offset(i as isize)).Num = count;
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn substance_tag_ptr(mut tag_ptr: *mut TAG_DATA)
                                           -> *mut TAG_DATA {
    while !tag_ptr.is_null() && (*tag_ptr).para_top_p as libc::c_int != 0 {
        tag_ptr = (*tag_ptr).child[0 as libc::c_int as usize]
    }
    return tag_ptr;
}

#[no_mangle]
pub unsafe extern "C" fn get_location(mut loc_name: *mut libc::c_char,
                                      mut sent_num: libc::c_int,
                                      mut kstr: *mut libc::c_char,
                                      mut mention: *mut MENTION,
                                      mut old_flag: libc::c_int)
                                      -> libc::c_int {
    let mut cpp: [libc::c_char; 16] = [0; 16];
    let mut pos_name: [libc::c_char; 128] = [0; 128];
    if (*mention).sent_num == sent_num {
        sprintf(pos_name.as_mut_ptr(),
                b"C%d\x00" as *const u8 as *const libc::c_char,
                if !(*(*(*mention).tag_ptr).b_ptr).parent.is_null() {
                    loc_category[(*(*(*(*mention).tag_ptr).b_ptr).parent).num
                        as usize]
                } else {
                    loc_category[(*(*(*mention).tag_ptr).b_ptr).num as usize]
                });
    } else if (*mention).sent_num > sent_num {
        sprintf(pos_name.as_mut_ptr(),
                b"A%d\x00" as *const u8 as *const libc::c_char,
                if (*mention).sent_num - sent_num <= 3 as libc::c_int {
                    ((*mention).sent_num) - sent_num
                } else { 0 as libc::c_int });
    } else {
        sprintf(pos_name.as_mut_ptr(),
                b"B%d\x00" as *const u8 as *const libc::c_char,
                if sent_num - (*mention).sent_num <= 3 as libc::c_int {
                    (sent_num) - (*mention).sent_num
                } else { 0 as libc::c_int });
    }
    if ((*mention).type_0 as libc::c_int == '=' as i32 ||
        (*mention).type_0 as libc::c_int == 'S' as i32) &&
        !(*(*mention).tag_ptr).parent.is_null() &&
        analysis_flags[(*mention).sent_num as
            usize][(*(*(*mention).tag_ptr).parent).num as
            usize] == 0 as libc::c_int {
        if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                          b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            sprintf(cpp.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                      b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                        b"\xe3\x82\xac\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\xb2\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\x8b\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"\xe3\x83\x8e\x00" as *const u8 as
                            *const libc::c_char
                    });
            sprintf(loc_name,
                    b"%s-C%s-%s\x00" as *const u8 as *const libc::c_char,
                    kstr, cpp.as_mut_ptr(), pos_name.as_mut_ptr());
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    } else {
        sprintf(loc_name,
                b"%s-%c%s-%s\x00" as *const u8 as *const libc::c_char, kstr,
                if (*mention).type_0 as libc::c_int == '=' as i32 {
                    'S' as i32
                } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                    'C' as i32
                } else { (*mention).type_0 as libc::c_int },
                if old_flag != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    (*mention).cpp_string.as_mut_ptr() as *const libc::c_char
                }, pos_name.as_mut_ptr());
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return if (*mention).sent_num == sent_num {
        if old_flag == 0 &&
            ((*mention).type_0 as libc::c_int == '=' as i32 ||
                (*mention).type_0 as libc::c_int == 'S' as i32) &&
            !(*(*(*mention).tag_ptr).b_ptr).parent.is_null() &&
            analysis_flags[(*(*(*(*mention).tag_ptr).b_ptr).parent).num as
                usize].as_mut_ptr().is_null() &&
            (!check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                            b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
            sprintf(loc_name,
                    b"%s-C%s-C%d\x00" as *const u8 as *const libc::c_char,
                    kstr,
                    if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                      b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                        b"\xe3\x82\xac\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\xb2\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\x8b\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"\xe3\x83\x8e\x00" as *const u8 as
                            *const libc::c_char
                    },
                    loc_category[(*(*(*(*mention).tag_ptr).b_ptr).parent).num
                        as usize]);
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            sprintf(loc_name,
                    b"%s-%c%s-C%d\x00" as *const u8 as *const libc::c_char,
                    kstr,
                    if (*mention).type_0 as libc::c_int == '=' as i32 {
                        'S' as i32
                    } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                        'C' as i32
                    } else { (*mention).type_0 as libc::c_int },
                    if old_flag != 0 {
                        b"\x00" as *const u8 as *const libc::c_char
                    } else {
                        (*mention).cpp_string.as_mut_ptr() as
                            *const libc::c_char
                    },
                    loc_category[(*(*(*mention).tag_ptr).b_ptr).num as
                        usize]);
            (0 as libc::c_int == 0) as libc::c_int
        }
    } else if sent_num - (*mention).sent_num == 1 as libc::c_int &&
        (!check_feature((*(*mention).tag_ptr).f,
                        b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8
                            as *const libc::c_char as
                            *mut libc::c_char).is_null() ||
            !check_feature((*(*mention).tag_ptr).f,
                           b"\xe8\xaa\xad\xe7\x82\xb9\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null()) &&
        !check_feature((*(*mention).tag_ptr).f,
                       b"\xe3\x83\x8f\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        sprintf(loc_name,
                b"%s-%c%s-B1B\x00" as *const u8 as *const libc::c_char, kstr,
                if (*mention).type_0 as libc::c_int == '=' as i32 {
                    'S' as i32
                } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                    'C' as i32
                } else { (*mention).type_0 as libc::c_int },
                if old_flag != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    (*mention).cpp_string.as_mut_ptr() as *const libc::c_char
                });
        (0 as libc::c_int == 0) as libc::c_int
    } else if sent_num - (*mention).sent_num == 1 as libc::c_int &&
        !check_feature((*(*mention).tag_ptr).f,
                       b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        !check_feature((*(*mention).tag_ptr).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        sprintf(loc_name,
                b"%s-%c%s-B1E\x00" as *const u8 as *const libc::c_char, kstr,
                if (*mention).type_0 as libc::c_int == '=' as i32 {
                    'S' as i32
                } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                    'C' as i32
                } else { (*mention).type_0 as libc::c_int },
                if old_flag != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    (*mention).cpp_string.as_mut_ptr() as *const libc::c_char
                });
        (0 as libc::c_int == 0) as libc::c_int
    } else if sent_num - (*mention).sent_num > 0 as libc::c_int {
        sprintf(loc_name,
                b"%s-%c%s-B%d\x00" as *const u8 as *const libc::c_char, kstr,
                if (*mention).type_0 as libc::c_int == '=' as i32 {
                    'S' as i32
                } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                    'C' as i32
                } else { (*mention).type_0 as libc::c_int },
                if old_flag != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    (*mention).cpp_string.as_mut_ptr() as *const libc::c_char
                },
                if sent_num - (*mention).sent_num <= 3 as libc::c_int {
                    (sent_num) - (*mention).sent_num
                } else { 0 as libc::c_int });
        (0 as libc::c_int == 0) as libc::c_int
    } else if sent_num - (*mention).sent_num < 0 as libc::c_int {
        if old_flag == 0 &&
            ((*mention).type_0 as libc::c_int == '=' as i32 ||
                (*mention).type_0 as libc::c_int == 'S' as i32) &&
            (!check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                            b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
            sprintf(loc_name,
                    b"%s-C%s-A%d\x00" as *const u8 as *const libc::c_char,
                    kstr,
                    if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                      b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                        b"\xe3\x82\xac\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\xb2\x00" as *const u8 as
                            *const libc::c_char
                    } else if !check_feature((*(*(*mention).tag_ptr).b_ptr).f,
                                             b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        b"\xe3\x83\x8b\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"\xe3\x83\x8e\x00" as *const u8 as
                            *const libc::c_char
                    },
                    if (*mention).sent_num - sent_num <= 3 as libc::c_int {
                        ((*mention).sent_num) - sent_num
                    } else { 0 as libc::c_int });
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            sprintf(loc_name,
                    b"%s-%c%s-A%d\x00" as *const u8 as *const libc::c_char,
                    kstr,
                    if (*mention).type_0 as libc::c_int == '=' as i32 {
                        'S' as i32
                    } else if (*mention).type_0 as libc::c_int == 'N' as i32 {
                        'C' as i32
                    } else { (*mention).type_0 as libc::c_int },
                    if old_flag != 0 {
                        b"\x00" as *const u8 as *const libc::c_char
                    } else {
                        (*mention).cpp_string.as_mut_ptr() as
                            *const libc::c_char
                    },
                    if (*mention).sent_num - sent_num <= 3 as libc::c_int {
                        ((*mention).sent_num) - sent_num
                    } else { 0 as libc::c_int });
            (0 as libc::c_int == 0) as libc::c_int
        }
    } else { 0 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn mark_loc_category(mut sp: *mut SENTENCE_DATA,
                                           mut tag_ptr: *mut TAG_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bnst_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut parent_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut pparent_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    bnst_ptr =
        substance_tag_ptr((*tag_ptr).b_ptr as *mut TAG_DATA) as
            *mut BNST_DATA;
    i = 0 as libc::c_int;
    while i < (*bnst_ptr).num {
        loc_category[i as usize] = 7 as libc::c_int;
        i += 1
    }
    i = (*bnst_ptr).num + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        loc_category[i as usize] = 8 as libc::c_int;
        i += 1
    }
    loc_category[(*bnst_ptr).num as usize] = 0 as libc::c_int;
    if (*bnst_ptr).para_type as libc::c_int == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while !(*(*bnst_ptr).parent).child[i as usize].is_null() {
            if (*(*(*bnst_ptr).parent).child[i as usize]).para_type as
                libc::c_int == 1 as libc::c_int &&
                (*(*(*bnst_ptr).parent).child[i as usize]).para_top_p == 0
            {
                if (*(*(*bnst_ptr).parent).child[i as usize]).num >
                    (*bnst_ptr).num {
                    loc_category[(*(*(*bnst_ptr).parent).child[i as
                        usize]).num
                        as usize] = 3 as libc::c_int
                } else if (*(*(*bnst_ptr).parent).child[i as usize]).num <
                    (*bnst_ptr).num {
                    loc_category[(*(*(*bnst_ptr).parent).child[i as
                        usize]).num
                        as usize] = 4 as libc::c_int
                }
            }
            i += 1
        }
        parent_ptr = (*bnst_ptr).parent;
        while (*parent_ptr).para_top_p as libc::c_int != 0 &&
            !(*parent_ptr).parent.is_null() {
            parent_ptr = (*parent_ptr).parent
        }
        if (*parent_ptr).para_top_p != 0 { parent_ptr = 0 as *mut BNST_DATA }
    } else if !(*bnst_ptr).parent.is_null() {
        parent_ptr = (*bnst_ptr).parent
    }
    if !parent_ptr.is_null() {
        loc_category[(*parent_ptr).num as usize] = 1 as libc::c_int;
        if !(*parent_ptr).parent.is_null() {
            pparent_ptr = (*parent_ptr).parent;
            while (*pparent_ptr).para_top_p as libc::c_int != 0 &&
                !(*pparent_ptr).parent.is_null() {
                pparent_ptr = (*pparent_ptr).parent
            }
            if (*pparent_ptr).para_top_p != 0 {
                pparent_ptr = 0 as *mut BNST_DATA
            }
        }
        if !pparent_ptr.is_null() {
            if !check_feature((*pparent_ptr).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                loc_category[(*pparent_ptr).num as usize] = 6 as libc::c_int
            } else {
                loc_category[(*pparent_ptr).num as usize] = 5 as libc::c_int
            }
        }
    }
    i = 0 as libc::c_int;
    while !(*bnst_ptr).child[i as usize].is_null() {
        if (*(*bnst_ptr).child[i as usize]).para_top_p != 0 {
            j = 0 as libc::c_int;
            while !(*(*bnst_ptr).child[i as
                usize]).child[j as usize].is_null()
            {
                if (*(*(*bnst_ptr).child[i as
                    usize]).child[j as
                    usize]).para_top_p
                    == 0 {
                    loc_category[(*(*(*bnst_ptr).child[i as
                        usize]).child[j as
                        usize]).num
                        as usize] = 2 as libc::c_int
                }
                j += 1
            }
        } else {
            loc_category[(*(*bnst_ptr).child[i as usize]).num as usize] =
                2 as libc::c_int
        }
        i += 1
    }
    if (*bnst_ptr).para_type as libc::c_int == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while !(*(*bnst_ptr).parent).child[i as usize].is_null() {
            if (*(*(*bnst_ptr).parent).child[i as usize]).para_type as
                libc::c_int == 0 as libc::c_int {
                loc_category[(*(*(*bnst_ptr).parent).child[i as usize]).num as
                    usize] = 2 as libc::c_int
            }
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < (*bnst_ptr).num {
        if (*(*(*sp).bnst_data.offset(i as isize)).parent).num != 0 &&
            (*(*(*sp).bnst_data.offset(i as isize)).parent).num >
                (*bnst_ptr).num &&
            !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                           b"\xe3\x83\x8f\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() {
            loc_category[i as usize] = 9 as libc::c_int
        }
        i += 1
    }
    if OptDisplay == 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            printf(b";;LOC %d-%s target_bnst:%d-%d\n\x00" as *const u8 as
                       *const libc::c_char, (*bnst_ptr).num,
                   (*bnst_ptr).Jiritu_Go.as_mut_ptr(), i,
                   loc_category[i as usize]);
            i += 1
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn check_analyze_tag(mut tag_ptr: *mut TAG_DATA,
                                           mut demo_flag: libc::c_int)
                                           -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bnst_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    if !check_feature((*tag_ptr).f,
                      b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"NE\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"NE\xe5\x86\x85\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe5\x90\x8c\xe6\xa0\xbc\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*tag_ptr).f,
                       b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\xe5\x86\x85\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        return 0 as libc::c_int;
    }
    if demo_flag != 0 &&
        (OptEllipsis & 4 as libc::c_int == 0 ||
            check_feature((*tag_ptr).f,
                          b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null()) {
        return 0 as libc::c_int;
    }
    if OptEllipsis & 4 as libc::c_int != 0 &&
        !check_feature((*tag_ptr).f,
                       b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        check_feature((*tag_ptr).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\xe4\xb8\x80\xe9\x83\xa8\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        !(OptEllipsis & 1 as libc::c_int != 0 &&
            !check_feature((*tag_ptr).f,
                           b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8
                               as *const libc::c_char as
                               *mut libc::c_char).is_null()) {
        if !check_feature((*tag_ptr).f,
                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            return 0 as libc::c_int;
        }
        if !check_feature((*tag_ptr).f,
                          b"\xe5\xbd\xa2\xe5\x89\xaf\xe5\x90\x8d\xe8\xa9\x9e\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            return 0 as libc::c_int;
        }
        bnst_ptr =
            substance_tag_ptr((*tag_ptr).b_ptr as *mut TAG_DATA) as
                *mut BNST_DATA;
        if !check_feature((*bnst_ptr).f,
                          b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            check_feature((*bnst_ptr).f,
                          b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            !(*bnst_ptr).child[0 as libc::c_int as usize].is_null() &&
            !check_feature((*(*bnst_ptr).child[0 as libc::c_int as
                usize]).f,
                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            return 0 as libc::c_int;
        }
        if demo_flag == 0 { return 2 as libc::c_int; }
        if !(*bnst_ptr).child[0 as libc::c_int as usize].is_null() &&
            strcmp((*(*(*bnst_ptr).child[0 as libc::c_int as
                usize]).head_ptr).Goi2.as_mut_ptr(),
                   b"\xe3\x81\x9d\xe3\x81\xae\x00" as *const u8 as
                       *const libc::c_char) != 0 &&
            ((*bnst_ptr).child[1 as libc::c_int as usize].is_null() ||
                strcmp((*(*(*bnst_ptr).child[1 as libc::c_int as
                    usize]).head_ptr).Goi2.as_mut_ptr(),
                       b"\xe3\x81\x9d\xe3\x81\xae\x00" as *const u8 as
                           *const libc::c_char) != 0) {
            return 0 as libc::c_int;
        }
        if demo_flag != 0 &&
            !(*bnst_ptr).child[0 as libc::c_int as usize].is_null() {
            return 2 as libc::c_int;
        }
        if (*bnst_ptr).para_type as libc::c_int == 1 as libc::c_int {
            i = 0 as libc::c_int;
            while !(*(*bnst_ptr).parent).child[i as usize].is_null() {
                if (*(*(*bnst_ptr).parent).child[i as usize]).para_type as
                    libc::c_int == 0 as libc::c_int &&
                    strcmp((*(*(*(*bnst_ptr).parent).child[i as
                        usize]).head_ptr).Goi2.as_mut_ptr(),
                           b"\xe3\x81\x9d\xe3\x81\xae\x00" as *const u8 as
                               *const libc::c_char) != 0 {
                    return 0 as libc::c_int;
                }
                if demo_flag != 0 &&
                    strcmp((*(*(*(*bnst_ptr).parent).child[i as
                        usize]).head_ptr).Goi2.as_mut_ptr(),
                           b"\xe3\x81\x9d\xe3\x81\xae\x00" as *const u8 as
                               *const libc::c_char) == 0 {
                    return 1 as libc::c_int;
                }
                i += 1
            }
        }
        if demo_flag != 0 { return 0 as libc::c_int; }
        return 2 as libc::c_int;
    }
    if OptEllipsis & 1 as libc::c_int != 0 &&
        !check_feature((*tag_ptr).f,
                       b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        if !(*tag_ptr).parent.is_null() &&
            !check_feature((*(*tag_ptr).parent).f,
                           b"\xe6\xa9\x9f\xe8\x83\xbd\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xef\xbc\xa1\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            assign_cfeature(&mut (*tag_ptr).f,
                            b"\xe6\x9c\xac\xe5\x8b\x95\xe8\xa9\x9e\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            assign_cfeature(&mut (*tag_ptr).f,
                            b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\xe8\xa9\x9e\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            return 1 as libc::c_int;
        }
    }
    if OptEllipsis & 1 as libc::c_int != 0 &&
        !check_feature((*tag_ptr).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() {
        if !check_feature((*tag_ptr).f,
                          b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            !check_feature((*tag_ptr).f,
                           b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            return 0 as libc::c_int;
        }
        if OptAnaphora & 4 as libc::c_int == 0 &&
            !check_feature((*tag_ptr).f,
                           b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn search_hypo_entity(mut sp: *mut SENTENCE_DATA, mut tag_ptr: *mut TAG_DATA, mut token: *mut libc::c_char) -> libc::c_int {
    let mut type_0: libc::c_char = 0;
    let mut rel: [libc::c_char; 128] = [0; 128];
    let mut entity_name: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tag_num: libc::c_int = 0;
    let mut sent_num: libc::c_int = 0;
    let mut mention_mgr: *mut MENTION_MGR = &mut (*tag_ptr).mention_mgr;

    if sscanf(token,
              b"%[^/]/%c/%[^/]/%d/%d/\x00" as *const u8 as *const libc::c_char,
              rel.as_mut_ptr(),
              &mut type_0 as *mut libc::c_char,
              entity_name.as_mut_ptr(),
              &mut tag_num as *mut libc::c_int,
              &mut sent_num as *mut libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    if OptAnaphora & 32 as libc::c_int != 0 &&
        type_0 as libc::c_int == 'E' as i32 {
        // let mut make_mention_flag: libc::c_int = 0 as libc::c_int;
        if (*mention_mgr).num >= 8 as libc::c_int - 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                if strcmp(b"\x00" as *const u8 as *const libc::c_char,
                          unnamed_entity_name[i as usize][j as usize]) == 0 {
                    break;
                }
                if !strstr(entity_name.as_mut_ptr(),
                           unnamed_entity_name[i as
                               usize][j as
                               usize]).is_null()
                {
                    let mut hypo_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
                    hypo_entity_ptr =
                        &mut *entity_manager.entity.as_mut_ptr().offset(i as
                            isize)
                            as *mut ENTITY;
                    (*hypo_entity_ptr).skip_flag = 0 as libc::c_int
                }
                j += 1
            }
            i += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}

pub unsafe extern "C" fn read_one_annotation(mut sp: *mut SENTENCE_DATA, mut tag_ptr: *mut TAG_DATA, mut token: *mut libc::c_char, mut co_flag: libc::c_int) -> libc::c_int {
    /* 解析結果からMENTION、ENTITYを作成する */
    /* co_flagがある場合は"="のみを処理、ない場合は"="以外を処理 */
    let mut i = 0;
    let mut j = 0;
    let mut tag_num = 0 as libc::c_int;
    let mut sent_num = 0;
    let mut mention_mgr: MENTION_MGR = tag_ptr.mention_mgr as MENTION_MGR;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut entity_ptr: ENTITY;
    let mut type_0: libc::c_char = "" as libc::c_char;
    let rel: [libc::c_char; SMALL_DATA_LEN as usize] = [];
    let mut cp: libc::c_char = "" as libc::c_char;
    let loc_name: [libc::c_char; SMALL_DATA_LEN as usize] = [];
    let entity_name: [libc::c_char; REPNAME_LEN_MAX as usize] = [];

    if !sscanf(token, "%[^/]/%c/%[^/]/%d/%d/" as *const libc::c_char, rel, &type_0, entity_name, &tag_num, &sent_num) {
        return 0;
    }
    /* 共参照関係の読み込み */
    if co_flag &&
        (!strcmp(rel as *mut libc::c_char, "=" as *const libc::c_char) || !strcmp(rel as *mut libc::c_char, "=構" as *const libc::c_char) || !strcmp(rel as *mut libc::c_char, "=役" as *const libc::c_char) != 0) as libc::c_int {
        if tag_num == -1 {
            return 0;
        } else {
            /* 複数の共参照情報が付与されている場合 */
            if mention_mgr.mention.entity {
                merge_two_entity(mention_mgr.mention.entity, substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity);
                substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity.link_entity = mention_mgr.mention.entity.num;
                return 1;
            }

            mention_ptr = mention_mgr.mention as *mut MENTION;
            if mention_ptr.entity.is_null() {
                if sp.Sen_num - sent_num > 0 && (sp - sent_num).Tag_num > tag_num {
                    mention_ptr.entity = substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity;
                } else {
                    return 0;
                }
            }

            /*共参照タグがずれている場合 暫定的な処置*/
            if mention_ptr.entity.is_null() {
                return 0;
            }

            set_mention_from_coreference(tag_ptr, mention_ptr);
        }
    } else if (!co_flag && (type_0 == 'N' as libc::c_char || type_0 == 'C' as libc::c_char || type_0 == 'O' as libc::c_char || type_0 == 'D' as libc::c_char) as libc::c_int) && (
        (OptReadFeature  & OPT_ALL_CASE) ||
            (check_analyze_tag(tag_ptr, 0) == CF_PRED &&
                (1 || (OptReadFeature & OPT_CASE_ANALYSIS) ||
                    match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST) != 0) ||
                check_analyze_tag(tag_ptr, 0) == CF_NOUN &&
                    (match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST) != 0 ||
                        match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST_VERB as *mut *mut libc::c_char) &&
                            strcpy(rel as *mut libc::c_char, "ノ？" as *mut libc::c_char) as libc::c_int)) as libc::c_int) {
        if tag_num == -1 {
            return 0;
        }
        if type_0 == 'O' as libc::c_char && (OptReadFeature & OPT_ELLIPSIS) == 0 {
            return 0;
        }
        if sp.Sen_num - sent_num <= 0 {
            return 0;
        }
        if mention_mgr.num >= MENTION_MAX - 1 {
            return 0;
        }
        if (OptReadFeature & OPT_ALL_CASE) ||
            (type_0 != 'O' as libc::c_char &&
                (check_feature(((sp - sent_num).tag_data + tag_num).f, "体言" as *mut libc::c_char) ||
                    check_feature(((sp - sent_num).tag_data + tag_num).f, "形副名詞" as *mut libc::c_char))) as libc::c_int||
            (type_0 == 'O' as libc::c_char &&
                check_feature_entity(substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity, "先行詞候補" as *mut libc::c_char) as bool) {
            mention_ptr = mention_mgr.mention + mention_mgr.num;
            if substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity.link_entity == -1 {
                mention_ptr.entity = substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity;
            } else {
                mention_ptr.entity = entity_manager.entity + (substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention.entity.link_entity);
            }
            if OptAnaphora & OPT_TRAIN {
                candidate_entities[mention_ptr.entity.num] = 1;
                //正解に出てくる照応先は候補に入れる
            }

            mention_ptr.explicit_mention = if type_0 == 'C' as libc::c_char { substance_tag_ptr((sp - sent_num).tag_data + tag_num).mention_mgr.mention as *mut mention } else { None };
            mention_ptr.salience_score = mention_ptr.entity.salience_score;
            mention_ptr.tag_num = mention_mgr.mention.tag_num;
            mention_ptr.sent_num = mention_mgr.mention.sent_num;
            mention_ptr.tag_ptr = (sentence_data + mention_ptr.sent_num - 1).tag_data + mention_ptr.tag_num;
            mention_ptr.type_0 = type_0;
            strcpy(mention_ptr.cpp_string as *mut libc::c_char, rel as *mut libc::c_char);
            if type_0 == 'C' as libc::c_char && check_feature(((sp - sent_num).tag_data + tag_num).f, "係" as *mut libc::c_char) as bool {
                strcpy(mention_ptr.spp_string as *mut libc::c_char, cp + strlen("係:" as *const libc::c_char));
            } else if type_0 == 'C' as libc::c_char && check_feature(((sp - sent_num).tag_data + tag_num).f, "文末" as *mut libc::c_char) as bool {
                strcpy(mention_ptr.spp_string as *mut libc::c_char, "文末" as *const libc::c_char);
            } else {
                strcpy(mention_ptr.spp_string as *mut libc::c_char, "＊" as *const libc::c_char);
            }
            mention_mgr.num += 1;

            /* 共参照タグを辿ると連体修飾先である場合はtype_0を'C'に変更 */
            if type_0 == 'O' as libc::c_char && check_feature(tag_ptr.f, "連体修飾" as *mut libc::c_char) as bool && tag_ptr.parent.mention_mgr.mention.entity == mention_ptr.entity {
                type_0 = 'C' as libc::c_char;
                mention_ptr.type_0 = type_0;
            }

            if type_0 == 'O' as libc::c_char {
                if check_analyze_tag(tag_ptr, 0) == CF_PRED {
                    mention_ptr.static_salience_score = SALIENCE_ZERO;
                } else {
                    mention_ptr.static_salience_score = SALIENCE_ASSO;
                }
            }
        } else {
            mention_ptr = mention_mgr.mention + mention_mgr.num;
            mention_ptr.entity = entity_manager.entity + 4;
            mention_ptr.explicit_mention = 0 as *mut mention;
            mention_ptr.tag_num = mention_mgr.mention.tag_num;
            mention_ptr.sent_num = mention_mgr.mention.sent_num;
            mention_ptr.tag_ptr = (sentence_data + mention_ptr.sent_num - 1).tag_data + mention_ptr.tag_num;
            mention_ptr.type_0 = 'E' as libc::c_char;
            strcpy(mention_ptr.cpp_string as *mut libc::c_char, rel as *mut libc::c_char);
            strcpy(mention_ptr.spp_string as *mut libc::c_char, "＊" as *const libc::c_char);
            mention_mgr.num += 1;
        }
    } else if ((OptAnaphora & OPT_UNNAMED_ENTITY) || (OptAnaphora & OPT_GS) || (OptAnaphora & OPT_TRAIN) != 0) && (!co_flag && (type_0 == ('E' as libc::c_char)) as libc::c_int) {
        let mut make_mention_flag = 0;
        if !(OptReadFeature & OPT_ELLIPSIS) {
            return 0;
        }
        if (OptReadFeature & OPT_ALL_CASE) != 0
            || check_analyze_tag(tag_ptr, 0) == CF_PRED as libc::c_int
            && ((OptReadFeature & OPT_CASE_ANALYSIS) != 0
            || match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST) != 0)
            || check_analyze_tag(tag_ptr, 0) == CF_NOUN
            && (
            match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST)
                || (match_ellipsis_case(rel as *mut libc::c_char, ELLIPSIS_CASE_LIST_VERB as *mut *mut libc::c_char)
                && strcpy(rel as *mut libc::c_char, "ノ？" as *const libc::c_char) as libc::c_int) as libc::c_int
        ) {} else {
            return 0;
        }
        if mention_mgr.num >= MENTION_MAX - 1 {
            return 0;
        }

        i = 0;
        while i < entity_manager.num {
            let mut name_match_flag = 0;
            if make_mention_flag == 1 {
                break;
            }
            if entity_manager.entity[i].hypothetical_flag != 1 {
                continue;
            }
            if i < UNNAMED_ENTITY_NUM {
                j = 0;
                while j < UNNAMED_ENTITY_NAME_NUM {
                    if !strcmp("" as *const libc::c_char, unnamed_entity_name[i][j]) {
                        break;
                    }
                    if !strcmp(entity_name as *const libc::c_char, unnamed_entity_name[i][j]) {
                        name_match_flag = 1;
                    }
                    j += 1;
                }
            } else {
                if !strcmp(entity_manager.entity[i].hypothetical_name, entity_name as *const libc::c_char) {
                    name_match_flag = 1;
                }
            }

            if name_match_flag == 1 {
                let mut hypo_entity_ptr: ENTITY = entity_manager.entity[i];
                make_mention_flag = 1;
                if hypo_entity_ptr.real_entity == -1 {
                    mention_ptr = mention_mgr.mention + mention_mgr.num;
                    mention_ptr.entity = entity_manager.entity + i;
                    mention_ptr.explicit_mention = 0 as *mut mention;
                    mention_ptr.tag_num = mention_mgr.mention.tag_num;
                    mention_ptr.sent_num = mention_mgr.mention.sent_num;
                    mention_ptr.tag_ptr = (sentence_data + mention_ptr.sent_num - 1).tag_data + mention_ptr.tag_num;
                    mention_ptr.type_0 = type_0;
                    strcpy(mention_ptr.cpp_string as *mut libc::c_char, rel as *mut libc::c_char);
                    strcpy(mention_ptr.spp_string as *mut libc::c_char, "＊" as *const libc::c_char);
                    mention_mgr.num += 1;
                } else {
                    mention_ptr = mention_mgr.mention + mention_mgr.num;
                    mention_ptr.entity = entity_manager.entity + (hypo_entity_ptr.real_entity);
                    mention_ptr.explicit_mention = 0 as *mut mention;
                    mention_ptr.salience_score = mention_ptr.entity.salience_score;
                    mention_ptr.tag_num = mention_mgr.mention.tag_num;
                    mention_ptr.sent_num = mention_mgr.mention.sent_num;
                    mention_ptr.tag_ptr = (sentence_data + mention_ptr.sent_num - 1).tag_data + mention_ptr.tag_num;
                    mention_ptr.type_0 = 'O' as libc::c_char;
                    strcpy(mention_ptr.cpp_string as *mut libc::c_char, rel as *mut libc::c_char);
                    strcpy(mention_ptr.spp_string as *mut libc::c_char, "＊" as *const libc::c_char);
                    mention_mgr.num += 1;
                }
            }
            i += 1;
        }
        if make_mention_flag == 0 {
            /*不特定-人nを作る*/
            let mut entity_num = -1;
            i = 0;
            while i < UNNAMED_ENTITY_NUM {
                j = 0;
                while j < UNNAMED_ENTITY_NAME_NUM {
                    if !strcmp(unnamed_entity_name[i][j], "" as *const libc::c_char) {
                        break;
                    }
                    if strstr(entity_name as *const libc::c_char, unnamed_entity_name[i][j]) {
                        entity_num = i;
                        break;
                    }
                    j += 1;
                }
                i += 1;
            }

            entity_ptr = *make_each_unnamed_entity(entity_name as *mut libc::c_char, entity_num);
            entity_ptr.salience_score = 0 as libc::c_double;
            entity_ptr.real_entity = -1;
            mention_ptr = mention_mgr.mention + mention_mgr.num;
            mention_ptr.entity = entity_ptr as *mut entity;
            mention_ptr.salience_score = 1 as libc::c_double;
            mention_ptr.tag_num = mention_mgr.mention.tag_num;
            mention_ptr.sent_num = mention_mgr.mention.sent_num;
            mention_ptr.tag_ptr = (sentence_data + mention_ptr.sent_num - 1).tag_data + mention_ptr.tag_num;
            mention_ptr.type_0 = 'E' as libc::c_char;
            strcpy(mention_ptr.cpp_string as *mut libc::c_char, rel as *mut libc::c_char);
            strcpy(mention_ptr.spp_string as *mut libc::c_char, "＊" as *const libc::c_char);
            mention_mgr.num += 1;
        }
    }
    if !mention_ptr {
        return 0;
    }
    mention_ptr.entity.mention[mention_ptr.entity.mentioned_num] = mention_ptr;
    if mention_ptr.entity.mentioned_num >= MENTIONED_MAX - 1 {
        fprintf(stderr, "Entity \"%s\" mentiond too many times!\n" as *const libc::c_char, mention_ptr.entity.name);
    } else {
        mention_ptr.entity.mentioned_num += 1;
    }
    /* 学習用情報の出力 */
    if (OptAnaphora & OPT_TRAIN) as libc::c_int && (type_0 == 'O' as libc::c_char) as libc::c_int && strcmp(rel as *mut libc::c_char, "=" as *const libc::c_char) != 0 {
        /* 位置カテゴリの出力 */
        mark_loc_category(sp, tag_ptr);
        entity_ptr = mention_ptr.entity as ENTITY;
        /* 何文以内にmentionを持っているかどうかのチェック */
        let mut diff_sen = 4;
        i = 0;
        while i < entity_ptr.mentioned_num {
            if mention_ptr.sent_num == entity_ptr.mention[i].sent_num && loc_category[(entity_ptr.mention[i].tag_ptr).b_ptr.num] == LOC_SELF {
                continue;
            }
            if mention_ptr.sent_num - entity_ptr.mention[i].sent_num < diff_sen {
                diff_sen = mention_ptr.sent_num - entity_ptr.mention[i].sent_num;
            }
            entity_ptr.mentioned_num += 1;
        }

        i = 0;
        while i < entity_ptr.mentioned_num {
            /* もっとも近くの文に出現したmentionのみ出力 */
            if mention_ptr.sent_num - entity_ptr.mention[i].sent_num > diff_sen {
                continue;
            }
            if entity_ptr.mention[i].sent_num == mention_ptr.sent_num && loc_category[(entity_ptr.mention[i].tag_ptr).b_ptr.num] == LOC_SELF {
                continue;
            }
            if get_location(loc_name as *mut libc::c_char, mention_ptr.sent_num, rel as *mut libc::c_char, entity_ptr.mention[i], 0) {
                printf(";;LOCATION-ANT: %s\n" as *const libc::c_char, loc_name);
            }
            i += 1;
        }
    }
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn expand_result_to_parallel_entity(mut tag_ptr:
                                                          *mut TAG_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut result_num: libc::c_int = 0;
    let mut ctm_ptr: *mut CF_TAG_MGR = (*tag_ptr).ctm_ptr;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut para_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut epnd_entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    // let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    result_num = (*ctm_ptr).result_num;
    i = 0 as libc::c_int;
    while i < result_num {
        entity_ptr =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        j = (*entity_ptr).mentioned_num - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if (*(*entity_ptr).mention[j as usize]).type_0 as libc::c_int ==
                'S' as i32 ||
                (*(*entity_ptr).mention[j as usize]).type_0 as libc::c_int
                    == '=' as i32 {
                break;
            }
            j -= 1
        }
        if !(OptAnaphora & 32 as libc::c_int != 0 &&
            (*entity_ptr).hypothetical_flag == 1 as libc::c_int ||
            (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num <
                (*(*entity_ptr).mention[j as usize]).sent_num) {
            t_ptr = (*(*entity_ptr).mention[j as usize]).tag_ptr;
            if (*t_ptr).para_type as libc::c_int == 1 as libc::c_int &&
                !(*t_ptr).parent.is_null() &&
                (*(*t_ptr).parent).para_top_p as libc::c_int != 0 {
                j = 0 as libc::c_int;
                while !(*(*t_ptr).parent).child[j as usize].is_null() {
                    para_ptr =
                        substance_tag_ptr((*(*t_ptr).parent).child[j as
                            usize]);
                    if para_ptr != t_ptr &&
                        !check_feature((*para_ptr).f,
                                       b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                           *const u8 as *const libc::c_char
                                           as *mut libc::c_char).is_null()
                        &&
                        (*para_ptr).para_type as libc::c_int ==
                            1 as libc::c_int &&
                        !((*ctm_ptr).type_0[i as usize] as libc::c_int ==
                            'O' as i32 &&
                            check_analyze_tag(tag_ptr, 0 as libc::c_int)
                                == 2 as libc::c_int) &&
                        !((*ctm_ptr).type_0[i as usize] as libc::c_int ==
                            'O' as i32 && (*tag_ptr).parent == para_ptr)
                    {
                        if !((*(*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity).num
                            == (*entity_ptr).num ||
                            (*(*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity).output_num
                                == (*entity_ptr).output_num) {
                            epnd_entity_ptr =
                                (*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity;
                            (*ctm_ptr).filled_entity[(*epnd_entity_ptr).num as
                                usize] =
                                (0 as libc::c_int == 0) as libc::c_int;
                            (*ctm_ptr).entity_num[(*ctm_ptr).result_num as
                                usize] =
                                (*epnd_entity_ptr).num;
                            (*ctm_ptr).type_0[(*ctm_ptr).result_num as usize]
                                = (*ctm_ptr).type_0[i as usize];
                            (*ctm_ptr).cf_element_num[(*ctm_ptr).result_num as
                                usize] =
                                (*ctm_ptr).cf_element_num[i as usize];
                            (*ctm_ptr).result_num += 1;
                            if OptDisplay == 3 as libc::c_int {
                                printf(b";;EXPANDED %s : %s -> %s\n\x00" as
                                           *const u8 as *const libc::c_char,
                                       (*(*tag_ptr).head_ptr).Goi2.as_mut_ptr(),
                                       (*entity_ptr).name.as_mut_ptr(),
                                       (*epnd_entity_ptr).name.as_mut_ptr());
                            }
                            if (*ctm_ptr).result_num == 24 as libc::c_int {
                                return;
                            }
                        }
                    }
                    j += 1
                }
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn anaphora_result_to_entity(mut tag_ptr:
                                                   *mut TAG_DATA) {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mention_mgr: *mut MENTION_MGR = &mut (*tag_ptr).mention_mgr;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut ctm_ptr: *mut CF_TAG_MGR = (*tag_ptr).ctm_ptr;
    if ctm_ptr.is_null() { return; }
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        if (*mention_mgr).num >= 8 as libc::c_int - 1 as libc::c_int {
            return;
        }
        mention_ptr =
            (*mention_mgr).mention.as_mut_ptr().offset((*mention_mgr).num as
                isize);
        (*mention_ptr).entity =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        (*mention_ptr).tag_num =
            (*(*mention_mgr).mention.as_mut_ptr()).tag_num;
        (*mention_ptr).sent_num =
            (*(*mention_mgr).mention.as_mut_ptr()).sent_num;
        (*mention_ptr).type_0 = (*ctm_ptr).type_0[i as usize];
        (*mention_ptr).tag_ptr =
            (*sentence_data.as_mut_ptr().offset((*mention_ptr).sent_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).tag_data.offset((*mention_ptr).tag_num
                as
                isize);
        strcpy((*mention_ptr).cpp_string.as_mut_ptr(),
               pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[(*ctm_ptr).cf_element_num[i
                   as
                   usize]
                   as
                   usize][0 as
                   libc::c_int
                   as
                   usize]));
        (*mention_ptr).salience_score =
            (*(*mention_ptr).entity).salience_score;
        if i < (*ctm_ptr).case_result_num {
            (*mention_ptr).explicit_mention =
                (*(*ctm_ptr).elem_b_ptr[i as
                    usize]).mention_mgr.mention.as_mut_ptr();
            if (*(*tag_ptr).tcf_ptr).cf.pp[(*ctm_ptr).tcf_element_num[i as
                usize]
                as
                usize][0 as libc::c_int as
                usize] >=
                9 as libc::c_int &&
                (*(*tag_ptr).tcf_ptr).cf.pp[(*ctm_ptr).tcf_element_num[i as
                    usize]
                    as
                    usize][0 as libc::c_int as
                    usize] <=
                    37 as libc::c_int {
                strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                       pp_code_to_kstr((*(*tag_ptr).tcf_ptr).cf.pp[(*ctm_ptr).tcf_element_num[i
                           as
                           usize]
                           as
                           usize][0
                           as
                           libc::c_int
                           as
                           usize]));
            } else {
                cp =
                    check_feature((*(*ctm_ptr).elem_b_ptr[i as usize]).f,
                                  b"\xe4\xbf\x82\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                if !cp.is_null() {
                    strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                           cp.offset(strlen(b"\xe4\xbf\x82:\x00" as *const u8
                               as *const libc::c_char) as
                               isize));
                } else if !check_feature((*(*ctm_ptr).elem_b_ptr[i as
                    usize]).f,
                                         b"\xe6\x96\x87\xe6\x9c\xab\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                    strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                           b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                           b"\xef\xbc\x8a\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
        } else {
            (*mention_ptr).explicit_mention = 0 as *mut mention;
            if (*ctm_ptr).type_0[i as usize] as libc::c_int != 'O' as i32 &&
                (*ctm_ptr).type_0[i as usize] as libc::c_int != 'E' as i32
            {
                strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                       b"\xef\xbc\xb0\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                strcpy((*mention_ptr).spp_string.as_mut_ptr(),
                       b"\xef\xbc\xaf\x00" as *const u8 as
                           *const libc::c_char);
                if check_analyze_tag(tag_ptr, 0 as libc::c_int) ==
                    1 as libc::c_int {
                    (*mention_ptr).static_salience_score = 1.0f64
                } else { (*mention_ptr).static_salience_score = 0.0f64 }
            }
        }
        (*mention_mgr).num += 1;
        (*(*mention_ptr).entity).mention[(*(*mention_ptr).entity).mentioned_num
            as usize] = mention_ptr;
        if (*(*mention_ptr).entity).mentioned_num >=
            256 as libc::c_int - 1 as libc::c_int {
            fprintf(stderr,
                    b"Entity \"%s\" mentiond too many times!\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*(*mention_ptr).entity).name.as_mut_ptr());
        } else { (*(*mention_ptr).entity).mentioned_num += 1 }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_tag_case_frame(mut sp: *mut SENTENCE_DATA,
                                            mut tag_ptr: *mut TAG_DATA,
                                            mut cpm_ptr: *mut CF_PRED_MGR)
                                            -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tcf_ptr: *mut TAG_CASE_FRAME = (*tag_ptr).tcf_ptr;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tcf_element_num: libc::c_int = 0 as libc::c_int;
    let mut cf_num: libc::c_int = 0 as libc::c_int;
    if check_analyze_tag(tag_ptr, 0 as libc::c_int) == 1 as libc::c_int {
        vtype =
            check_feature((*tag_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        vtype =
            vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as *const u8
                as *const libc::c_char) as isize);
        strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(), vtype);
        (*cpm_ptr).cf.type_0 = 1 as libc::c_int
    } else {
        strcpy((*cpm_ptr).cf.pred_type.as_mut_ptr(),
               b"\xe5\x90\x8d\x00" as *const u8 as *const libc::c_char);
        (*cpm_ptr).cf.type_0 = 2 as libc::c_int
    }
    (*cpm_ptr).cf.type_flag = 0 as libc::c_int;
    (*cpm_ptr).cf.voice = (*tag_ptr).voice;
    make_data_cframe(sp, cpm_ptr);
    (*tcf_ptr).cf = (*cpm_ptr).cf;
    (*tcf_ptr).pred_b_ptr = tag_ptr;
    (*tcf_ptr).cf_with_functional_tag[cf_num as usize] = (*cpm_ptr).cf;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        (*tcf_ptr).elem_b_ptr[i as usize] =
            substance_tag_ptr((*cpm_ptr).elem_b_ptr[i as usize]);
        (*tcf_ptr).elem_b_num[i as usize] = (*cpm_ptr).elem_b_num[i as usize];
        (*tcf_ptr).map_tcf_elem_to_cf[tcf_element_num as usize] = cf_num;
        (*tcf_ptr).map_tcf_elem_to_cf_elem[tcf_element_num as usize] = i;
        tcf_element_num += 1;
        i += 1
    }
    cf_num += 1;
    if !check_feature((*tag_ptr).f,
                      b"\xe6\x9c\xac\xe5\x8b\x95\xe8\xa9\x9e\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        let mut parent_ptr: *mut TAG_DATA = (*tag_ptr).parent;
        while !parent_ptr.is_null() &&
            !check_feature((*parent_ptr).f,
                           b"\xe6\xa9\x9f\xe8\x83\xbd\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            let mut function_cpm_ptr: *mut CF_PRED_MGR =
                0 as *mut CF_PRED_MGR;
            function_cpm_ptr =
                malloc_data(::std::mem::size_of::<CF_PRED_MGR>() as
                                libc::c_ulong,
                            b"make_context_structure: cpm_ptr\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char) as *mut CF_PRED_MGR;
            init_case_frame(&mut (*function_cpm_ptr).cf);
            (*function_cpm_ptr).pred_b_ptr = parent_ptr;
            if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
                strcpy((*function_cpm_ptr).cf.pred_type.as_mut_ptr(), vtype);
                (*function_cpm_ptr).cf.type_0 = 1 as libc::c_int
            } else {
                strcpy((*function_cpm_ptr).cf.pred_type.as_mut_ptr(),
                       b"\xe5\x90\x8d\x00" as *const u8 as
                           *const libc::c_char);
                (*function_cpm_ptr).cf.type_0 = 2 as libc::c_int
            }
            (*function_cpm_ptr).cf.type_flag = 0 as libc::c_int;
            (*function_cpm_ptr).cf.voice = (*parent_ptr).voice;
            make_data_cframe(sp, function_cpm_ptr);
            i = 0 as libc::c_int;
            while i < (*function_cpm_ptr).cf.element_num {
                (*tcf_ptr).elem_b_ptr[tcf_element_num as usize] =
                    substance_tag_ptr((*function_cpm_ptr).elem_b_ptr[i as
                        usize]);
                (*tcf_ptr).elem_b_num[tcf_element_num as usize] =
                    (*function_cpm_ptr).elem_b_num[i as usize];
                (*tcf_ptr).map_tcf_elem_to_cf[tcf_element_num as usize] =
                    cf_num;
                (*tcf_ptr).map_tcf_elem_to_cf_elem[tcf_element_num as usize] =
                    i;
                tcf_element_num += 1;
                i += 1
            }
            (*tcf_ptr).cf_with_functional_tag[cf_num as usize] =
                (*function_cpm_ptr).cf;
            free(function_cpm_ptr as *mut libc::c_void);
            cf_num += 1;
            parent_ptr = (*parent_ptr).parent
        }
    }
    (*tcf_ptr).cf_num = cf_num;
    (*tcf_ptr).cf.element_num = tcf_element_num;
    return (0 as libc::c_int == 0) as libc::c_int;
}

/// start番目からnum個のellipsis_result_ctmのr_numとスコアと比較し上位ならば保存する
/// num個のellipsis_result_ctmのスコアは降順にソートされていることを仮定している
/// 保存された場合は1、されなかった場合は0を返す
pub unsafe extern "C" fn preserve_ellipsis_gs_ctm(mut ctm_ptr: *mut CF_TAG_MGR,
                                                  mut start: libc::c_int,
                                                  mut num: libc::c_int,
                                                  mut result_ctm: *mut CF_TAG_MGR) -> libc::c_int {
    let mut i = start;
    let mut j = start + num - 1;
    while i < start + num {
        if ctm_ptr.annotated_result_num > result_ctm[i].annotated_result_num || (ctm_ptr.result_num == result_ctm[i].result_num && ctm_ptr.score > result_ctm[i].score) {
            j = start + num - 1;
            while j > 1 {
                if result_ctm[j - 1].score > INITIAL_SCORE {
                    copy_ctm(&result_ctm[j - 1], result_ctm[j]);
                }
                j -= 1;
            }
            copy_ctm(ctm_ptr, result_ctm[i]);
            return 1;
        }
        i += 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn set_cf_candidate(mut tag_ptr: *mut TAG_DATA,
                                          mut cf_array: *mut *mut CASE_FRAME)
                                          -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut frame_num: libc::c_int = 0 as libc::c_int;
    let mut hiragana_prefer_type: libc::c_int = 0 as libc::c_int;
    let mut cfp: *mut CFLIST = 0 as *mut CFLIST;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptUseSmfix == (0 as libc::c_int == 0) as libc::c_int &&
        CFSimExist == (0 as libc::c_int == 0) as libc::c_int {
        key = get_pred_id((*(*tag_ptr).cf_ptr).cf_id.as_mut_ptr());
        if !key.is_null() {
            cfp = CheckCF(key);
            free(key as *mut libc::c_void);
            if !cfp.is_null() {
                l = 0 as libc::c_int;
                while l < (*tag_ptr).cf_num {
                    i = 0 as libc::c_int;
                    while i < (*cfp).cfid_num {
                        if (*(*tag_ptr).cf_ptr.offset(l as isize)).type_0 ==
                            (*(*tag_ptr).tcf_ptr).cf.type_0 &&
                            {
                                let ref mut fresh8 =
                                    (*(*tag_ptr).cf_ptr.offset(l as
                                        isize)).cf_similarity;
                                *fresh8 =
                                    get_cfs_similarity((*(*tag_ptr).cf_ptr.offset(l
                                        as
                                        isize)).cf_id.as_mut_ptr(),
                                                       *(*cfp).cfid.offset(i
                                                           as
                                                           isize));
                                (*fresh8) > CFSimThreshold
                            } {
                            let fresh9 = frame_num;
                            frame_num = frame_num + 1;
                            let ref mut fresh10 =
                                *cf_array.offset(fresh9 as isize);
                            *fresh10 = (*tag_ptr).cf_ptr.offset(l as isize);
                            break;
                        } else { i += 1 }
                    }
                    l += 1
                }
                (*tag_ptr).e_cf_num = frame_num
            }
        }
    }
    if frame_num == 0 as libc::c_int {
        if OptCaseFlag & 32 as libc::c_int == 0 &&
            check_str_type((*(*tag_ptr).head_ptr).Goi.as_mut_ptr() as
                               *mut libc::c_uchar, 2 as libc::c_int,
                           0 as libc::c_int) != 0 {
            if !check_feature((*tag_ptr).f,
                              b"\xe4\xbb\xa3\xe8\xa1\xa8\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                hiragana_prefer_type = 1 as libc::c_int
            } else { hiragana_prefer_type = -(1 as libc::c_int) }
        }
        l = 0 as libc::c_int;
        while l < (*tag_ptr).cf_num {
            if (*(*tag_ptr).cf_ptr.offset(l as isize)).type_0 ==
                (*(*tag_ptr).tcf_ptr).cf.type_0 &&
                (hiragana_prefer_type == 0 as libc::c_int ||
                    hiragana_prefer_type > 0 as libc::c_int &&
                        check_str_type((*(*tag_ptr).cf_ptr.offset(l as
                            isize)).entry
                                           as *mut libc::c_uchar,
                                       2 as libc::c_int, 0 as libc::c_int)
                            != 0 ||
                    hiragana_prefer_type < 0 as libc::c_int &&
                        check_str_type((*(*tag_ptr).cf_ptr.offset(l as
                            isize)).entry
                                           as *mut libc::c_uchar,
                                       2 as libc::c_int, 0 as libc::c_int)
                            == 0) {
                let fresh11 = frame_num;
                frame_num = frame_num + 1;
                let ref mut fresh12 = *cf_array.offset(fresh11 as isize);
                *fresh12 = (*tag_ptr).cf_ptr.offset(l as isize)
            }
            l += 1
        }
    }
    return frame_num;
}

#[no_mangle]
pub unsafe extern "C" fn calc_score_of_ctm(mut ctm_ptr: *mut CF_TAG_MGR,
                                           mut tcf_ptr: *mut TAG_CASE_FRAME)
                                           -> libc::c_double {
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    let mut debug: libc::c_int = 1 as libc::c_int;
    let mut score: libc::c_double = 0.;
    // let mut key: [libc::c_char; 128] = [0; 128];
    (*ctm_ptr).case_analysis_ga_entity = -(1 as libc::c_int);
    score =
        get_cf_probability_for_pred(&mut (*tcf_ptr).cf, (*ctm_ptr).cf_ptr);
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).case_result_num {
        if !((*ctm_ptr).type_0[i as usize] as libc::c_int == 'O' as i32) {
            e_num = (*ctm_ptr).cf_element_num[i as usize];
            if (*(*ctm_ptr).cf_ptr).pp[e_num as
                usize][0 as libc::c_int as usize]
                ==
                pp_kstr_to_code(b"\xe3\x82\xac\x00" as *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) {
                (*ctm_ptr).case_analysis_ga_entity =
                    (*ctm_ptr).entity_num[i as usize]
            }
            score +=
                get_ex_probability_with_para((*ctm_ptr).tcf_element_num_functional[i
                    as
                    usize],
                                             &mut (*tcf_ptr).cf, e_num,
                                             (*ctm_ptr).cf_ptr) +
                    get_case_function_probability_for_pred((*ctm_ptr).tcf_element_num_functional[i
                        as
                        usize],
                                                           &mut (*tcf_ptr).cf,
                                                           e_num,
                                                           (*ctm_ptr).cf_ptr,
                                                           (0 as libc::c_int
                                                               == 0) as
                                                               libc::c_int);
            if OptDisplay == 3 as libc::c_int && debug != 0 {
                printf(b";;\xe5\xaf\xbe\xe5\xbf\x9c\xe3\x81\x82\xe3\x82\x8a:%s-%s:%f:%f \x00"
                           as *const u8 as *const libc::c_char,
                       (*(*(*ctm_ptr).elem_b_ptr[i as
                           usize]).head_ptr).Goi2.as_mut_ptr(),
                       pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num as
                           usize][0 as
                           libc::c_int
                           as
                           usize]),
                       get_ex_probability_with_para((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                    &mut (*tcf_ptr).cf, e_num,
                                                    (*ctm_ptr).cf_ptr),
                       get_case_function_probability_for_pred((*ctm_ptr).tcf_element_num_functional[i
                           as
                           usize],
                                                              &mut (*tcf_ptr).cf,
                                                              e_num,
                                                              (*ctm_ptr).cf_ptr,
                                                              (0 as
                                                                  libc::c_int
                                                                  == 0) as
                                                                  libc::c_int));
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*tcf_ptr).cf.element_num - (*ctm_ptr).case_result_num {
        if OptDisplay == 3 as libc::c_int && debug != 0 {
            printf(b";;\xe5\xaf\xbe\xe5\xbf\x9c\xe3\x81\xaa\xe3\x81\x97:%s:%f \x00"
                       as *const u8 as *const libc::c_char,
                   (*(*(*tcf_ptr).elem_b_ptr[(*ctm_ptr).non_match_element[i as
                       usize]
                       as
                       usize]).head_ptr).Goi2.as_mut_ptr(),
                   score);
        }
        score += -13.815511f64 + -11.512925f64;
        i += 1
    }
    if OptDisplay == 3 as libc::c_int && debug != 0 {
        printf(b";; %f \x00" as *const u8 as *const libc::c_char, score);
    }
    e_num = 0 as libc::c_int;
    while e_num < (*(*ctm_ptr).cf_ptr).element_num {
        if !((*tcf_ptr).cf.type_0 == 2 as libc::c_int) {
            score +=
                get_case_probability(e_num, (*ctm_ptr).cf_ptr,
                                     (*ctm_ptr).filled_element[e_num as
                                         usize],
                                     0 as *mut CF_PRED_MGR)
        }
        e_num += 1
    }
    if OptDisplay == 3 as libc::c_int && debug != 0 {
        printf(b";; %f\n\x00" as *const u8 as *const libc::c_char, score);
    }
    return score;
}

#[no_mangle]
pub unsafe extern "C" fn convert_locname_id(mut loc_name: *mut libc::c_char,
                                            mut loc_num_ptr: *mut libc::c_int,
                                            mut simple_loc_num_ptr:
                                            *mut libc::c_int)
                                            -> libc::c_int {
    // let mut id: libc::c_int = 0 as libc::c_int;
    let mut loc_id: libc::c_int = 0 as libc::c_int;
    let mut case_id: libc::c_int = 0 as libc::c_int;
    let mut c_type: libc::c_int = 9 as libc::c_int;
    let mut b_type: libc::c_int = 3 as libc::c_int;
    let mut a_type: libc::c_int = 3 as libc::c_int;
    let mut num: libc::c_int = 0 as libc::c_int;
    if strlen(loc_name) !=
        (3 as libc::c_int * 2 as libc::c_int + 5 as libc::c_int) as
            libc::c_ulong {
        *loc_num_ptr = -(1 as libc::c_int);
        *simple_loc_num_ptr = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if *loc_name.offset((3 as libc::c_int + 1 as libc::c_int) as isize) as
        libc::c_int == 'C' as i32 {
        case_id = 1 as libc::c_int
    } else if *loc_name.offset((3 as libc::c_int + 1 as libc::c_int) as isize)
        as libc::c_int == 'O' as i32 ||
        *loc_name.offset((3 as libc::c_int + 1 as libc::c_int) as
            isize) as libc::c_int == 'E' as i32 {
        case_id = 2 as libc::c_int
    } else if *loc_name.offset((3 as libc::c_int + 1 as libc::c_int) as isize)
        as libc::c_int == 'S' as i32 {
        case_id = 0 as libc::c_int
    } else {
        *loc_num_ptr = -(1 as libc::c_int);
        *simple_loc_num_ptr = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if *loc_name.offset((3 as libc::c_int + 1 as libc::c_int) as isize) as
        libc::c_int != 'S' as i32 {
        let mut case_type: libc::c_int = 4 as libc::c_int;
        if strncmp(loc_name.offset(3 as libc::c_int as
            isize).offset(2 as libc::c_int as
            isize),
                   b"\xe3\x83\xb2\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            case_id =
                (case_id - 1 as libc::c_int) * case_type + 1 as libc::c_int +
                    1 as libc::c_int
        } else if strncmp(loc_name.offset(3 as libc::c_int as
            isize).offset(2 as libc::c_int
            as isize),
                          b"\xe3\x83\x8b\x00" as *const u8 as
                              *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) == 0 {
            case_id =
                (case_id - 1 as libc::c_int) * case_type + 2 as libc::c_int +
                    1 as libc::c_int
        } else if strncmp(loc_name.offset(3 as libc::c_int as
            isize).offset(2 as libc::c_int
            as isize),
                          b"\xe3\x82\xac\x00" as *const u8 as
                              *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) == 0 {
            case_id =
                (case_id - 1 as libc::c_int) * case_type + 1 as libc::c_int
        } else if strncmp(loc_name.offset(3 as libc::c_int as
            isize).offset(2 as libc::c_int
            as isize),
                          b"\xe3\x83\x8e\x00" as *const u8 as
                              *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) == 0 {
            *loc_num_ptr = -(1 as libc::c_int);
            *simple_loc_num_ptr = -(1 as libc::c_int);
            return -(1 as libc::c_int);
        } else {
            *loc_num_ptr = -(1 as libc::c_int);
            *simple_loc_num_ptr = -(1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    }
    if *loc_name.offset((3 as libc::c_int * 2 as libc::c_int +
        3 as libc::c_int) as isize) as libc::c_int ==
        'B' as i32 {
        loc_id = c_type
    } else if *loc_name.offset((3 as libc::c_int * 2 as libc::c_int +
        3 as libc::c_int) as isize) as libc::c_int
        == 'A' as i32 {
        loc_id = b_type + c_type
    } else if *loc_name.offset((3 as libc::c_int * 2 as libc::c_int +
        3 as libc::c_int) as isize) as libc::c_int
        == 'C' as i32 {
        loc_id = 0 as libc::c_int
    } else {
        *loc_num_ptr = -(1 as libc::c_int);
        *simple_loc_num_ptr = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    num =
        atoi(loc_name.offset((3 as libc::c_int * 2 as libc::c_int) as
            isize).offset(4 as libc::c_int as isize));
    if num > 0 as libc::c_int {
        loc_id += num - 1 as libc::c_int
    } else {
        *loc_num_ptr = -(1 as libc::c_int);
        *simple_loc_num_ptr = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *loc_num_ptr = loc_id + case_id * (c_type + b_type + a_type);
    *simple_loc_num_ptr = loc_id;
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn make_ctm_from_corpus(mut tag_ptr: *mut TAG_DATA,
                                              mut ctm_ptr: *mut CF_TAG_MGR,
                                              mut handling_idx: libc::c_int,
                                              mut r_num: libc::c_int,
                                              mut annotated_r_num:
                                              libc::c_int,
                                              mut case_analysis_flag:
                                              libc::c_int)
                                              -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    // let mut l: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    // let mut exist_flag: libc::c_int = 0;
    let mut pre_filled_element: [libc::c_int; 24] = [0; 24];
    // let mut pre_filled_entity: [libc::c_int; 4096] = [0; 4096];
    memset((*ctm_ptr).filled_element.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    j = 0 as libc::c_int;
    while j < r_num {
        (*ctm_ptr).filled_element[(*ctm_ptr).cf_element_num[j as usize] as
            usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        j += 1
    }
    if case_analysis_flag == 0 as libc::c_int {
        let mut mention_idx: libc::c_int = handling_idx;
        if mention_idx < (*tag_ptr).mention_mgr.num {
            let mut assinged_flag: libc::c_int = 0 as libc::c_int;
            // let mut skip_flag: libc::c_int = 0 as libc::c_int;
            if case_analysis_flag == 0 as libc::c_int &&
                (*tag_ptr).mention_mgr.mention[mention_idx as usize].type_0
                    as libc::c_int == 'E' as i32 ||
                (*tag_ptr).mention_mgr.mention[mention_idx as usize].type_0
                    as libc::c_int == 'O' as i32 {
                e_num = 0 as libc::c_int;
                while e_num < (*(*ctm_ptr).cf_ptr).element_num {
                    if (*(*ctm_ptr).cf_ptr).pp[e_num as
                        usize][0 as libc::c_int as
                        usize] ==
                        pp_kstr_to_code((*tag_ptr).mention_mgr.mention[mention_idx
                            as
                            usize].cpp_string.as_mut_ptr())
                    {
                        if !((*ctm_ptr).filled_element[e_num as usize] ==
                            (0 as libc::c_int == 0) as libc::c_int) {
                            assinged_flag = 1 as libc::c_int;
                            (*ctm_ptr).cf_element_num[r_num as usize] = e_num;
                            (*ctm_ptr).entity_num[r_num as usize] =
                                (*(*tag_ptr).mention_mgr.mention[mention_idx
                                    as
                                    usize].entity).num;
                            make_ctm_from_corpus(tag_ptr, ctm_ptr,
                                                 mention_idx +
                                                     1 as libc::c_int,
                                                 r_num + 1 as libc::c_int,
                                                 annotated_r_num +
                                                     1 as libc::c_int,
                                                 case_analysis_flag);
                        }
                    }
                    e_num += 1
                }
            }
            make_ctm_from_corpus(tag_ptr, ctm_ptr,
                                 mention_idx + 1 as libc::c_int, r_num,
                                 annotated_r_num, case_analysis_flag);
        } else {
            let mut aresult: [libc::c_char; 256] = [0; 256];
            let mut score: libc::c_double = 0.;
            (*ctm_ptr).result_num = r_num;
            (*ctm_ptr).annotated_result_num = annotated_r_num;
            make_aresult_string(ctm_ptr, aresult.as_mut_ptr());
            score =
                calc_ellipsis_score_of_ctm(ctm_ptr, (*tag_ptr).tcf_ptr,
                                           tag_ptr);
            if score != -(10000 as libc::c_int) as libc::c_double {
                (*ctm_ptr).all_arguments_score =
                    0 as libc::c_int as libc::c_double;
                (*ctm_ptr).case_analysis_score =
                    calc_score_of_case_frame_assingemnt(ctm_ptr,
                                                        (*tag_ptr).tcf_ptr);
                (*ctm_ptr).score =
                    (*ctm_ptr).overt_arguments_score * overt_arguments_weight;
                (*ctm_ptr).score +=
                    (*ctm_ptr).all_arguments_score * all_arguments_weight;
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    k = 0 as libc::c_int;
                    while k <
                        2 as libc::c_int *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int)
                                + 3 as libc::c_int + 2 as libc::c_int
                                + 8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int
                                + 15 as libc::c_int * 3 as libc::c_int
                                + 5 as libc::c_int * 4 as libc::c_int
                                + 5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int)
                                *
                                (3 as libc::c_int + 5 as libc::c_int))
                    {
                        (*ctm_ptr).score +=
                            if (*ctm_ptr).omit_feature[j as usize][k as usize]
                                ==
                                -(10000 as libc::c_int) as libc::c_double {
                                0 as libc::c_int as libc::c_double
                            } else {
                                ((*ctm_ptr).omit_feature[j as
                                    usize][k as
                                    usize])
                                    *
                                    case_feature_weight[j as
                                        usize][k as usize]
                            };
                        k += 1
                    }
                    j += 1
                }
                preserve_ellipsis_gs_ctm(ctm_ptr, 0 as libc::c_int,
                                         3 as libc::c_int,
                                         ellipsis_correct_ctm.as_mut_ptr());
            }
        }
    } else {
        let mut cf_element_idx: libc::c_int = handling_idx;
        if cf_element_idx < (*(*tag_ptr).tcf_ptr).cf.element_num {
            let mut annotated_tag: [libc::c_char; 16] =
                *::std::mem::transmute::<&[u8; 16],
                    &mut [libc::c_char; 16]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
            let mut mention_idx_0: libc::c_int = 0;
            let mut annotated_flag: libc::c_int = 0 as libc::c_int;
            mention_idx_0 = 1 as libc::c_int;
            while mention_idx_0 < (*tag_ptr).mention_mgr.num {
                if (*(*(*(*(*tag_ptr).tcf_ptr).elem_b_ptr[cf_element_idx as
                    usize]).mention_mgr.mention.as_mut_ptr()).entity).num
                    ==
                    (*(*tag_ptr).mention_mgr.mention[mention_idx_0 as
                        usize].entity).num
                {
                    strcpy(annotated_tag.as_mut_ptr(),
                           (*tag_ptr).mention_mgr.mention[mention_idx_0 as
                               usize].cpp_string.as_mut_ptr());
                    annotated_flag = 1 as libc::c_int
                }
                mention_idx_0 += 1
            }
            e_num = 0 as libc::c_int;
            while e_num < (*(*ctm_ptr).cf_ptr).element_num {
                if annotated_flag == 1 as libc::c_int &&
                    (*(*ctm_ptr).cf_ptr).pp[e_num as
                        usize][0 as libc::c_int as
                        usize] ==
                        pp_kstr_to_code(annotated_tag.as_mut_ptr()) ||
                    annotated_flag == 0 as libc::c_int &&
                        ((*(*ctm_ptr).cf_ptr).pp[e_num as
                            usize][0 as
                            libc::c_int
                            as usize]
                            ==
                            pp_kstr_to_code(b"\xe4\xbf\xae\xe9\xa3\xbe\x00"
                                as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char) ||
                            (*(*ctm_ptr).cf_ptr).pp[e_num as
                                usize][0 as
                                libc::c_int
                                as
                                usize]
                                ==
                                pp_kstr_to_code(b"\xe6\x99\x82\xe9\x96\x93\x00"
                                    as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char) ||
                            (*(*ctm_ptr).cf_ptr).pp[e_num as
                                usize][0 as
                                libc::c_int
                                as
                                usize]
                                ==
                                pp_kstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                    as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char)) {
                    (*ctm_ptr).cf_element_num[r_num as usize] = e_num;
                    (*ctm_ptr).elem_b_ptr[r_num as usize] =
                        (*(*tag_ptr).tcf_ptr).elem_b_ptr[cf_element_idx as
                            usize];
                    (*ctm_ptr).tcf_element_num[r_num as usize] =
                        cf_element_idx;
                    (*ctm_ptr).tcf_element_num_functional[r_num as usize] =
                        cf_element_idx;
                    (*ctm_ptr).type_0[r_num as usize] =
                        if (*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf[cf_element_idx
                            as
                            usize]
                            > 0 as libc::c_int {
                            'O' as i32
                        } else if (*(*tag_ptr).tcf_ptr).elem_b_num[cf_element_idx
                            as
                            usize]
                            == -(1 as libc::c_int) {
                            'N' as i32
                        } else { 'C' as i32 } as libc::c_char;
                    (*ctm_ptr).entity_num[r_num as usize] =
                        (*(*(*(*(*tag_ptr).tcf_ptr).elem_b_ptr[cf_element_idx
                            as
                            usize]).mention_mgr.mention.as_mut_ptr()).entity).num;
                    make_ctm_from_corpus(tag_ptr, ctm_ptr,
                                         cf_element_idx + 1 as libc::c_int,
                                         r_num + 1 as libc::c_int,
                                         annotated_r_num + annotated_flag,
                                         case_analysis_flag);
                }
                e_num += 1
            }
            make_ctm_from_corpus(tag_ptr, ctm_ptr,
                                 cf_element_idx + 1 as libc::c_int, r_num,
                                 annotated_r_num, case_analysis_flag);
        } else {
            (*ctm_ptr).case_result_num = r_num;
            (*ctm_ptr).overt_arguments_score =
                calc_score_of_ctm(ctm_ptr, (*tag_ptr).tcf_ptr);
            make_ctm_from_corpus(tag_ptr, ctm_ptr, 0 as libc::c_int, r_num,
                                 annotated_r_num, 0 as libc::c_int);
        }
    }
    memcpy((*ctm_ptr).filled_element.as_mut_ptr() as *mut libc::c_void,
           pre_filled_element.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn calc_ellipsis_score_of_ctm(mut ctm_ptr:
                                                    *mut CF_TAG_MGR,
                                                    mut tcf_ptr:
                                                    *mut TAG_CASE_FRAME,
                                                    mut tag_ptr:
                                                    *mut TAG_DATA)
                                                    -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut loc_num: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    let mut sent_num: libc::c_int = 0;
    let mut tag_num: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut max_score: libc::c_double = 0.;
    let mut tmp_ne_ct_score: libc::c_double = 0.;
    let mut tmp_score: libc::c_double = 0.;
    let mut prob: libc::c_double = 0.;
    // let mut penalty: libc::c_double = 0.;
    let mut of_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    // let mut scase_prob_cs: libc::c_double = 0.;
    // let mut scase_prob: libc::c_double = 0.;
    let mut location_prob: libc::c_double = 0.;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut loc_name: [libc::c_char; 128] = [0; 128];
    let mut modality_feature: [libc::c_int; 11] = [0; 11];
    let mut voice_feature: [libc::c_int; 5] = [0; 5];
    let mut keigo_feature: [libc::c_int; 3] = [0; 3];
    let mut tense_feature: [libc::c_int; 4] = [0; 4];
    let mut pred_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    // let mut aresult: [libc::c_char; 256] = [0; 256];
    // let mut sen_cat_base: libc::c_int = 0;
    let mut adj_flag: libc::c_int = 0;
    let mut pred_dpnd_type: libc::c_int = 0 as libc::c_int;
    let mut verb_situation: libc::c_int = 0 as libc::c_int;
    let mut ga_filled_flag: libc::c_int = 0 as libc::c_int;
    // let mut adjacent_flag: libc::c_int = 0 as libc::c_int;
    let mut oblig_flag: libc::c_int = 1 as libc::c_int;
    let mut ex_case_prob: libc::c_double = 0.;
    let mut ex_prob: libc::c_double = 0.;
    let mut cf_ex_sum: libc::c_int = 0 as libc::c_int;
    let mut cf_ga_filled_ratio: libc::c_double = -13.815511f64;
    let mut hypo_entity_fill_case: [libc::c_int; 4] = [0; 4];
    let mut ability_feature: libc::c_int = 0 as libc::c_int;
    let mut functional_flag: libc::c_int = 0 as libc::c_int;
    sent_num =
        (*(*(*tcf_ptr).pred_b_ptr).mention_mgr.mention.as_mut_ptr()).sent_num;
    tag_num =
        (*(*(*tcf_ptr).pred_b_ptr).mention_mgr.mention.as_mut_ptr()).tag_num;
    if !check_feature((*(*tcf_ptr).pred_b_ptr).f,
                      b"\xe6\x9c\xac\xe5\x8b\x95\xe8\xa9\x9e\xe7\x9a\x84\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        pred_ptr = (*(*tcf_ptr).pred_b_ptr).parent;
        functional_flag = 1 as libc::c_int
    } else { pred_ptr = (*tcf_ptr).pred_b_ptr }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j <
            2 as libc::c_int *
                ((4 as libc::c_int +
                    (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int +
                    5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int +
                    4 as libc::c_int + 10 as libc::c_int) *
                    (3 as libc::c_int + 5 as libc::c_int)) {
            (*ctm_ptr).omit_feature[i as usize][j as usize] =
                -(10000 as libc::c_int) as libc::c_double;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        hypo_entity_fill_case[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    (*ctm_ptr).ga_entity = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        let mut feature_case_num: libc::c_int = 0;
        e_num = (*ctm_ptr).cf_element_num[i as usize];
        feature_case_num =
            get_ellipsis_case_num(pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num
                as
                usize][0
                as
                libc::c_int
                as
                usize]),
                                  0 as *mut *mut libc::c_char);
        if feature_case_num == 0 as libc::c_int {
            ga_filled_flag = 1 as libc::c_int
        }
        i += 1
    }
    e_num = 0 as libc::c_int;
    while e_num < (*(*ctm_ptr).cf_ptr).element_num {
        cf_ex_sum += (*(*ctm_ptr).cf_ptr).freq[e_num as usize];
        e_num += 1
    }
    e_num = 0 as libc::c_int;
    while e_num < (*(*ctm_ptr).cf_ptr).element_num {
        if cf_ex_sum != 0 {
            if (*(*ctm_ptr).cf_ptr).pp[e_num as
                usize][0 as libc::c_int as usize]
                ==
                pp_kstr_to_code(b"\xe3\x82\xac\x00" as *const u8 as
                    *const libc::c_char as
                    *mut libc::c_char) &&
                (*(*ctm_ptr).cf_ptr).freq[e_num as usize] != 0 {
                cf_ga_filled_ratio =
                    log((*(*ctm_ptr).cf_ptr).freq[e_num as usize] as
                        libc::c_double / cf_ex_sum as libc::c_double)
            }
            ((*ctm_ptr).filled_element[e_num as usize] == 0 &&
                (*(*ctm_ptr).cf_ptr).pp[e_num as
                    usize][0 as libc::c_int as usize]
                    !=
                    pp_kstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                        as *const u8 as *const libc::c_char
                        as *mut libc::c_char)) &&
                (*(*ctm_ptr).cf_ptr).freq[e_num as usize] as libc::c_double /
                    cf_ex_sum as libc::c_double > 0.5f64;
        }
        e_num += 1
    }
    if OptAnaphora & 32 as libc::c_int != 0 &&
        (OptAnaphora & 16 as libc::c_int == 0 &&
            ga_filled_flag == 0 as libc::c_int ||
            oblig_flag == 0 as libc::c_int) {
        return -(10000 as libc::c_int) as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        let mut mod_0: [libc::c_char; 5120] =
            *::std::mem::transmute::<&[u8; 5120],
                &mut [libc::c_char; 5120]>(b"\xe3\x83\xa2\xe3\x83\x80\xe3\x83\xaa\xe3\x83\x86\xe3\x82\xa3-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        strcat(mod_0.as_mut_ptr(), modality[i as usize]);
        if !check_feature((*(*pred_ptr).b_ptr).f,
                          mod_0.as_mut_ptr()).is_null() {
            modality_feature[i as usize] = 1 as libc::c_int
        } else { modality_feature[i as usize] = 0 as libc::c_int }
        i += 1
    }
    if functional_flag == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            let mut voice_str: [libc::c_char; 5120] =
                *::std::mem::transmute::<&[u8; 5120],
                    &mut [libc::c_char; 5120]>(b"\xe6\x85\x8b:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
            strcat(voice_str.as_mut_ptr(), voice[i as usize]);
            if !check_feature((*(*pred_ptr).b_ptr).f,
                              voice_str.as_mut_ptr()).is_null() {
                voice_feature[i as usize] = 1 as libc::c_int
            } else { voice_feature[i as usize] = 0 as libc::c_int }
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut kei: [libc::c_char; 5120] =
            *::std::mem::transmute::<&[u8; 5120],
                &mut [libc::c_char; 5120]>(b"\xe6\x95\xac\xe8\xaa\x9e:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        strcat(kei.as_mut_ptr(), keigo[i as usize]);
        if !check_feature((*(*pred_ptr).b_ptr).f, kei.as_mut_ptr()).is_null()
        {
            keigo_feature[i as usize] = 1 as libc::c_int
        } else { keigo_feature[i as usize] = 0 as libc::c_int }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut ten: [libc::c_char; 5120] =
            *::std::mem::transmute::<&[u8; 5120],
                &mut [libc::c_char; 5120]>(b"\xe6\x99\x82\xe5\x88\xb6-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        strcat(ten.as_mut_ptr(), tense[i as usize]);
        if !check_feature((*pred_ptr).f, ten.as_mut_ptr()).is_null() {
            tense_feature[i as usize] = 1 as libc::c_int
        } else { tense_feature[i as usize] = 0 as libc::c_int }
        i += 1
    }
    if !check_feature((*(*pred_ptr).b_ptr).f,
                      b"\xe5\x8f\xaf\xe8\x83\xbd\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        ability_feature = 1 as libc::c_int
    }
    if strcmp((*(*ctm_ptr).cf_ptr).pred_type.as_mut_ptr(),
              b"\xe5\xbd\xa2\x00" as *const u8 as *const libc::c_char) == 0 {
        adj_flag = 1 as libc::c_int
    } else { adj_flag = 0 as libc::c_int }
    if !check_feature((*pred_ptr).f,
                      b"\xe4\xbf\x82:\xe6\x96\x87\xe6\x9c\xab\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        pred_dpnd_type = 0 as libc::c_int
    } else if !check_feature((*pred_ptr).f,
                             b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        pred_dpnd_type = 1 as libc::c_int
    } else if !check_feature((*pred_ptr).f,
                             b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        pred_dpnd_type = 2 as libc::c_int
    }
    if !check_feature((*pred_ptr).f,
                      b"\xe5\x8b\x95\xe6\x85\x8b\xe8\xbf\xb0\xe8\xaa\x9e\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        verb_situation = 0 as libc::c_int
    } else if !check_feature((*pred_ptr).f,
                             b"\xe7\x8a\xb6\xe6\x85\x8b\xe8\xbf\xb0\xe8\xaa\x9e\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        verb_situation = 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
        let mut entity_num: libc::c_int = -(1 as libc::c_int);
        let mut hypothetical_num: libc::c_int = 0;
        entity_ptr =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        hypothetical_num = (*entity_ptr).hypothetical_entity;
        if OptAnaphora & 32 as libc::c_int != 0 &&
            ((*entity_ptr).num < 5 as libc::c_int ||
                hypothetical_num != -(1 as libc::c_int) &&
                    hypothetical_num < 5 as libc::c_int ||
                hypothetical_num != -(1 as libc::c_int) &&
                    (*entity_ptr).hypothetical_flag == 1 as libc::c_int) {
            if (*entity_ptr).num < 5 as libc::c_int {
                entity_num = (*entity_ptr).num
            } else if hypothetical_num < 5 as libc::c_int {
                entity_num = hypothetical_num
            } else {
                j = 0 as libc::c_int;
                while j < 5 as libc::c_int {
                    k = 0 as libc::c_int;
                    while k < 2 as libc::c_int {
                        if strcmp(unnamed_entity_name[j as usize][k as usize],
                                  b"\x00" as *const u8 as *const libc::c_char)
                            == 0 {
                            break;
                        }
                        if !strstr((*entity_ptr).hypothetical_name.as_mut_ptr(),
                                   unnamed_entity_name[j as
                                       usize][k as
                                       usize]).is_null()
                        {
                            entity_num = j;
                            break;
                        } else { k += 1 }
                    }
                    j += 1
                }
            }
        }
        if entity_num != -(1 as libc::c_int) {
            let mut feature_case_num_0: libc::c_int =
                get_ellipsis_case_num(pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num
                    as
                    usize][0
                    as
                    libc::c_int
                    as
                    usize]),
                                      0 as *mut *mut libc::c_char);
            if feature_case_num_0 != -(1 as libc::c_int) &&
                entity_num < 5 as libc::c_int {
                hypo_entity_fill_case[feature_case_num_0 as usize] =
                    entity_num
            }
        }
        i += 1
    }
    let mut current_block_647: u64;
    i = 0 as libc::c_int;
    while i < (*ctm_ptr).result_num {
        let mut feature_base: libc::c_int = 0;
        let mut feature_base_2: libc::c_int = -(1 as libc::c_int);
        let mut hypothetical_num_0: libc::c_int = 0;
        let mut filled_flag: libc::c_int = 0 as libc::c_int;
        let mut feature_case_num_1: libc::c_int = 0;
        let mut case_frame_exist_flag: libc::c_int = 0 as libc::c_int;
        let mut entity_ptr_0: *mut ENTITY = 0 as *mut ENTITY;
        e_num = (*ctm_ptr).cf_element_num[i as usize];
        entity_ptr_0 =
            entity_manager.entity.as_mut_ptr().offset((*ctm_ptr).entity_num[i
                as
                usize]
                as isize);
        hypothetical_num_0 = (*entity_ptr_0).hypothetical_entity;
        pp =
            (*(*ctm_ptr).cf_ptr).pp[e_num as
                usize][0 as libc::c_int as usize];
        feature_case_num_1 =
            get_ellipsis_case_num(pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num
                as
                usize][0
                as
                libc::c_int
                as
                usize]),
                                  0 as *mut *mut libc::c_char);
        if i < (*ctm_ptr).case_result_num {
            filled_flag = 1 as libc::c_int;
            case_frame_exist_flag = 1 as libc::c_int
        } else { feature_base = 0 as libc::c_int }
        if pp == 1 as libc::c_int {
            (*ctm_ptr).ga_entity = (*entity_ptr_0).num
        }
        if feature_case_num_1 == -(1 as libc::c_int) {
            if filled_flag == 1 as libc::c_int {
                current_block_647 = 14184516523743666873;
            } else {
                of_ptr =
                    (*ctm_ptr).omit_feature[0 as libc::c_int as
                        usize].as_mut_ptr();
                current_block_647 = 4127803603908737533;
            }
        } else {
            of_ptr =
                (*ctm_ptr).omit_feature[feature_case_num_1 as
                    usize].as_mut_ptr();
            current_block_647 = 4127803603908737533;
        }
        match current_block_647 {
            4127803603908737533 => {
                max_score = -(10000 as libc::c_int) as libc::c_double;
                if OptAnaphora & 32 as libc::c_int != 0 &&
                    ((*entity_ptr_0).num < 5 as libc::c_int ||
                        hypothetical_num_0 != -(1 as libc::c_int) &&
                            hypothetical_num_0 < 5 as libc::c_int ||
                        hypothetical_num_0 != -(1 as libc::c_int) &&
                            (*entity_ptr_0).hypothetical_flag ==
                                1 as libc::c_int) {
                    let mut entity_num_0: libc::c_int = 0;
                    let mut max_pmi: libc::c_double =
                        -(10000 as libc::c_int) as libc::c_double;
                    case_frame_exist_flag = 1 as libc::c_int;
                    if (*entity_ptr_0).num < 5 as libc::c_int {
                        entity_num_0 = (*entity_ptr_0).num
                    } else if hypothetical_num_0 < 5 as libc::c_int {
                        entity_num_0 = hypothetical_num_0
                    } else {
                        j = 0 as libc::c_int;
                        while j < 5 as libc::c_int {
                            k = 0 as libc::c_int;
                            while k < 2 as libc::c_int {
                                if strcmp(unnamed_entity_name[j as
                                    usize][k as
                                    usize],
                                          b"\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    break;
                                }
                                if !strstr((*entity_ptr_0).hypothetical_name.as_mut_ptr(),
                                           unnamed_entity_name[j as
                                               usize][k as
                                               usize]).is_null()
                                {
                                    entity_num_0 = j;
                                    break;
                                } else { k += 1 }
                            }
                            j += 1
                        }
                    }
                    feature_base =
                        (4 as libc::c_int +
                            (33 as libc::c_int + 5 as libc::c_int) +
                            3 as libc::c_int + 2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int * 3 as libc::c_int +
                            15 as libc::c_int * 3 as libc::c_int +
                            5 as libc::c_int * 4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int * 2 as libc::c_int +
                            3 as libc::c_int * 2 as libc::c_int +
                            4 as libc::c_int + 10 as libc::c_int) *
                            3 as libc::c_int +
                            entity_num_0 *
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int) +
                                    3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int);
                    feature_base +=
                        adj_flag *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                (3 as libc::c_int + 5 as libc::c_int));
                    feature_base_2 = feature_base;
                    *of_ptr.offset((feature_base + 9 as libc::c_int) as isize)
                        = 1 as libc::c_int as libc::c_double;
                    if OptAnaphora & 512 as libc::c_int != 0 &&
                        OptAnaphora & 32 as libc::c_int != 0 {
                        *of_ptr.offset((feature_base + 15 as libc::c_int) as
                            isize) = author_score
                    }
                    if OptAnaphora & 1024 as libc::c_int != 0 &&
                        OptAnaphora & 32 as libc::c_int != 0 {
                        *of_ptr.offset((feature_base + 16 as libc::c_int) as
                            isize) = reader_score
                    }
                    if (*(*ctm_ptr).cf_ptr).freq[e_num as usize] >
                        0 as libc::c_int {
                        *of_ptr.offset((feature_base + 29 as libc::c_int) as
                            isize) =
                            log((*(*ctm_ptr).cf_ptr).freq[e_num as usize] as
                                libc::c_double /
                                cf_ex_sum as libc::c_double)
                    } else {
                        *of_ptr.offset((feature_base + 29 as libc::c_int) as
                            isize) = -13.815511f64
                    }
                    *of_ptr.offset((feature_base + 14 as libc::c_int) as
                        isize) =
                        get_case_probability(e_num, (*ctm_ptr).cf_ptr,
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int,
                                             0 as *mut CF_PRED_MGR);
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int)) +
                        pred_dpnd_type) as isize) =
                        1 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            3 as libc::c_int) +
                        verb_situation) as isize) =
                        1 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            3 as libc::c_int +
                            2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int *
                                3 as libc::c_int +
                            15 as libc::c_int *
                                3 as libc::c_int +
                            5 as libc::c_int *
                                4 as libc::c_int)) as isize)
                        = yobikake_count as libc::c_double;
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int + 1 as libc::c_int {
                        k = 0 as libc::c_int;
                        while k < 5 as libc::c_int {
                            if hypo_entity_fill_case[j as usize] == k {
                                *of_ptr.offset((feature_base +
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as
                                                libc::c_int)
                                        + 3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int
                                        +
                                        15 as libc::c_int *
                                            3 as libc::c_int)
                                    + j * 5 as libc::c_int +
                                    k) as isize) =
                                    1 as libc::c_int as libc::c_double
                            }
                            k += 1
                        }
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 11 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int) + j) as
                            isize) =
                            modality_count[j as usize] as libc::c_double;
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int) +
                            11 as libc::c_int + j) as isize) =
                            modality_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 5 as libc::c_int {
                        *of_ptr.offset((feature_base + 33 as libc::c_int + j)
                            as isize) =
                            voice_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int) + j) as
                            isize) =
                            keigo_count[j as usize] as libc::c_double;
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int) +
                            3 as libc::c_int + j) as isize) =
                            keigo_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int +
                                3 as libc::c_int *
                                    2 as libc::c_int) + j) as
                            isize) =
                            tense_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    if ability_feature == 1 as libc::c_int {
                        *of_ptr.offset((feature_base + 31 as libc::c_int) as
                            isize) =
                            1 as libc::c_int as libc::c_double
                    }
                    if OptGeneralCF & 2 as libc::c_int != 0 {
                        j = 0 as libc::c_int;
                        while j < 21 as libc::c_int {
                            if strcmp(unnamed_entity_category[entity_num_0 as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                break;
                            }
                            sprintf(key.as_mut_ptr(),
                                    b"CT:%s:\x00" as *const u8 as
                                        *const libc::c_char,
                                    unnamed_entity_category[entity_num_0 as
                                        usize][j as
                                        usize]);
                            prob =
                                get_ex_ne_probability(key.as_mut_ptr(), e_num,
                                                      (*ctm_ptr).cf_ptr,
                                                      (0 as libc::c_int == 0)
                                                          as libc::c_int);
                            *strchr(key.as_mut_ptr().offset(3 as libc::c_int
                                as isize),
                                    ':' as i32) =
                                '\u{0}' as i32 as libc::c_char;
                            ex_prob =
                                get_general_probability(key.as_mut_ptr(),
                                                        b"KEY\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
                            if ex_prob != -13.815511f64 {
                                if prob == 0 as libc::c_int as libc::c_double
                                {
                                    tmp_score = -13.815511f64
                                } else { tmp_score = log(prob) }
                                ex_case_prob = tmp_score;
                                tmp_score -= ex_prob;
                                if tmp_score >
                                    *of_ptr.offset((feature_base +
                                        2 as libc::c_int)
                                        as isize) {
                                    if tmp_score > max_pmi {
                                        max_pmi = tmp_score
                                    }
                                    *of_ptr.offset((feature_base +
                                        2 as libc::c_int) as
                                        isize) = tmp_score;
                                    *of_ptr.offset((feature_base +
                                        19 as libc::c_int) as
                                        isize) = ex_case_prob;
                                    *of_ptr.offset((feature_base +
                                        20 as libc::c_int) as
                                        isize) = ex_prob
                                }
                            }
                            j += 1
                        }
                    }
                    if 0 as libc::c_int != 0 &&
                        OptGeneralCF & 1 as libc::c_int != 0 {
                        j = 0 as libc::c_int;
                        while j < 7 as libc::c_int {
                            if strcmp(unnamed_entity_ne[entity_num_0 as
                                usize][j as
                                usize],
                                      b"\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                break;
                            }
                            sprintf(key.as_mut_ptr(),
                                    b"NE:%s:\x00" as *const u8 as
                                        *const libc::c_char,
                                    unnamed_entity_ne[entity_num_0 as
                                        usize][j as usize]);
                            prob =
                                get_ex_ne_probability(key.as_mut_ptr(), e_num,
                                                      (*ctm_ptr).cf_ptr,
                                                      (0 as libc::c_int == 0)
                                                          as libc::c_int);
                            *strchr(key.as_mut_ptr().offset(3 as libc::c_int
                                as isize),
                                    ':' as i32) =
                                '\u{0}' as i32 as libc::c_char;
                            ex_prob =
                                get_general_probability(key.as_mut_ptr(),
                                                        b"KEY\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
                            if ex_prob != -13.815511f64 {
                                if prob == 0 as libc::c_int as libc::c_double
                                {
                                    tmp_score = -13.815511f64
                                } else { tmp_score = log(prob) }
                                ex_case_prob = tmp_score;
                                tmp_score -= ex_prob;
                                if tmp_score >
                                    *of_ptr.offset((feature_base +
                                        3 as libc::c_int)
                                        as isize) {
                                    if tmp_score > max_pmi {
                                        max_pmi = tmp_score
                                    }
                                    *of_ptr.offset((feature_base +
                                        3 as libc::c_int) as
                                        isize) = tmp_score;
                                    *of_ptr.offset((feature_base +
                                        21 as libc::c_int) as
                                        isize) = ex_case_prob;
                                    *of_ptr.offset((feature_base +
                                        22 as libc::c_int) as
                                        isize) = ex_prob
                                }
                                if tmp_score > tmp_ne_ct_score {
                                    tmp_ne_ct_score = tmp_score
                                }
                            }
                            j += 1
                        }
                    }
                    prob =
                        _get_ex_probability_internal(unnamed_entity[entity_num_0
                            as
                            usize],
                                                     e_num,
                                                     (*ctm_ptr).cf_ptr);
                    ex_prob =
                        get_general_probability(unnamed_entity[entity_num_0 as
                            usize],
                                                b"KEY\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char);
                    if prob != 0 as libc::c_int as libc::c_double {
                        if log(prob) >
                            *of_ptr.offset((feature_base +
                                32 as libc::c_int) as
                                isize) {
                            *of_ptr.offset((feature_base + 32 as libc::c_int)
                                as isize) = log(prob)
                        }
                    } else if *of_ptr.offset((feature_base +
                        32 as libc::c_int) as isize)
                        == -(10000 as libc::c_int) as libc::c_double
                    {
                        *of_ptr.offset((feature_base + 32 as libc::c_int) as
                            isize) = -13.815511f64
                    }
                    j = 0 as libc::c_int;
                    while j < 6 as libc::c_int {
                        if strcmp(unnamed_entity_rep[entity_num_0 as
                            usize][j as usize],
                                  b"\x00" as *const u8 as *const libc::c_char)
                            == 0 {
                            break;
                        }
                        sprintf(key.as_mut_ptr(),
                                unnamed_entity_rep[entity_num_0 as
                                    usize][j as usize]);
                        prob =
                            _get_ex_probability_internal(key.as_mut_ptr(),
                                                         e_num,
                                                         (*ctm_ptr).cf_ptr);
                        ex_prob =
                            get_general_probability(key.as_mut_ptr(),
                                                    b"KEY\x00" as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char);
                        if ex_prob != -13.815511f64 {
                            if prob == 0 as libc::c_int as libc::c_double {
                                tmp_score = -13.815511f64
                            } else { tmp_score = log(prob) }
                            ex_case_prob = tmp_score;
                            tmp_score -= ex_prob;
                            if tmp_score >
                                *of_ptr.offset((feature_base +
                                    1 as libc::c_int) as
                                    isize) {
                                if tmp_score > max_pmi { max_pmi = tmp_score }
                                *of_ptr.offset((feature_base +
                                    1 as libc::c_int) as
                                    isize) = tmp_score;
                                *of_ptr.offset((feature_base +
                                    17 as libc::c_int) as
                                    isize) = ex_case_prob;
                                *of_ptr.offset((feature_base +
                                    18 as libc::c_int) as
                                    isize) = ex_prob
                            }
                        }
                        j += 1
                    }
                    *of_ptr.offset((feature_base + 6 as libc::c_int) as isize)
                        = max_pmi
                }
                if (*entity_ptr_0).hypothetical_flag == 0 as libc::c_int {
                    let mut max_pmi_0: libc::c_double =
                        -(10000 as libc::c_int) as libc::c_double;
                    if filled_flag == 0 as libc::c_int {
                        feature_base =
                            4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int
                    } else {
                        feature_base =
                            (4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                2 as libc::c_int
                    }
                    feature_base +=
                        adj_flag *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                (3 as libc::c_int + 5 as libc::c_int));
                    *of_ptr.offset((feature_base + 9 as libc::c_int) as isize)
                        = 1 as libc::c_int as libc::c_double;
                    if OptAnaphora & 512 as libc::c_int != 0 &&
                        OptAnaphora & 32 as libc::c_int != 0 {
                        *of_ptr.offset((feature_base + 15 as libc::c_int) as
                            isize) = author_score
                    }
                    if OptAnaphora & 1024 as libc::c_int != 0 &&
                        OptAnaphora & 32 as libc::c_int != 0 {
                        *of_ptr.offset((feature_base + 16 as libc::c_int) as
                            isize) = reader_score
                    }
                    if (*(*ctm_ptr).cf_ptr).freq[e_num as usize] >
                        0 as libc::c_int {
                        *of_ptr.offset((feature_base + 29 as libc::c_int) as
                            isize) =
                            log((*(*ctm_ptr).cf_ptr).freq[e_num as usize] as
                                libc::c_double /
                                cf_ex_sum as libc::c_double)
                    } else {
                        *of_ptr.offset((feature_base + 29 as libc::c_int) as
                            isize) = -13.815511f64
                    }
                    *of_ptr.offset((feature_base + 14 as libc::c_int) as
                        isize) =
                        get_case_probability(e_num, (*ctm_ptr).cf_ptr,
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int,
                                             0 as *mut CF_PRED_MGR);
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int)) +
                        pred_dpnd_type) as isize) =
                        1 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            3 as libc::c_int) +
                        verb_situation) as isize) =
                        1 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base +
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            3 as libc::c_int +
                            2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int *
                                3 as libc::c_int +
                            15 as libc::c_int *
                                3 as libc::c_int +
                            5 as libc::c_int *
                                4 as libc::c_int)) as isize)
                        = yobikake_count as libc::c_double;
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int + 1 as libc::c_int {
                        k = 0 as libc::c_int;
                        while k < 5 as libc::c_int {
                            if hypo_entity_fill_case[j as usize] == k {
                                *of_ptr.offset((feature_base +
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as
                                                libc::c_int)
                                        + 3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int
                                        +
                                        15 as libc::c_int *
                                            3 as libc::c_int)
                                    + j * 5 as libc::c_int +
                                    k) as isize) =
                                    1 as libc::c_int as libc::c_double
                            }
                            k += 1
                        }
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 11 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int) + j) as
                            isize) =
                            modality_count[j as usize] as libc::c_double;
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int) +
                            11 as libc::c_int + j) as isize) =
                            modality_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 5 as libc::c_int {
                        *of_ptr.offset((feature_base + 33 as libc::c_int + j)
                            as isize) =
                            voice_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int) + j) as
                            isize) =
                            keigo_count[j as usize] as libc::c_double;
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int) +
                            3 as libc::c_int + j) as isize) =
                            keigo_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int +
                                3 as libc::c_int *
                                    2 as libc::c_int) + j) as
                            isize) =
                            tense_feature[j as usize] as libc::c_double;
                        j += 1
                    }
                    if ability_feature == 1 as libc::c_int {
                        *of_ptr.offset((feature_base + 31 as libc::c_int) as
                            isize) =
                            1 as libc::c_int as libc::c_double
                    }
                    j = 0 as libc::c_int;
                    while j < (*entity_ptr_0).mentioned_num {
                        if !((*(*entity_ptr_0).mention[j as usize]).type_0 as
                            libc::c_int != 'S' as i32 &&
                            (*(*entity_ptr_0).mention[j as usize]).type_0
                                as libc::c_int != '=' as i32) {
                            tmp_ne_ct_score = -13.815511f64;
                            if OptGeneralCF & 4 as libc::c_int != 0 &&
                                (*tcf_ptr).cf.type_0 == 1 as libc::c_int {
                                cp =
                                    get_bnst_head_canonical_rep((*(*(*entity_ptr_0).mention[j
                                        as
                                        usize]).tag_ptr).b_ptr,
                                                                OptCaseFlag &
                                                                    512 as
                                                                        libc::c_int);
                                if !cp.is_null() &&
                                    strlen(cp) <
                                        (128 as libc::c_int -
                                            4 as libc::c_int) as
                                            libc::c_ulong {
                                    sprintf(key.as_mut_ptr(),
                                            b"%s:CL\x00" as *const u8 as
                                                *const libc::c_char, cp);
                                    prob =
                                        get_class_probability(key.as_mut_ptr(),
                                                              e_num,
                                                              (*ctm_ptr).cf_ptr);
                                    if prob != 0. &&
                                        log(prob) >
                                            *of_ptr.offset((feature_base +
                                                4 as
                                                    libc::c_int)
                                                as isize) {
                                        *of_ptr.offset((feature_base +
                                            4 as libc::c_int)
                                            as isize) =
                                            log(prob)
                                    }
                                }
                            }
                            if OptGeneralCF & 2 as libc::c_int != 0 &&
                                {
                                    cp =
                                        check_feature((*(*(*(*entity_ptr_0).mention[j
                                            as
                                            usize]).tag_ptr).head_ptr).f,
                                                      b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    !cp.is_null()
                                } {
                                let mut category_ptr: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                strcpy(buf.as_mut_ptr(),
                                       cp.offset(strlen(b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa:\x00"
                                           as *const u8 as
                                           *const libc::c_char)
                                           as isize));
                                category_ptr =
                                    strtok(buf.as_mut_ptr(),
                                           b";\x00" as *const u8 as
                                               *const libc::c_char);
                                while !category_ptr.is_null() {
                                    sprintf(key.as_mut_ptr(),
                                            b"CT:%s:\x00" as *const u8 as
                                                *const libc::c_char,
                                            category_ptr);
                                    prob =
                                        get_ex_ne_probability(key.as_mut_ptr(),
                                                              e_num,
                                                              (*ctm_ptr).cf_ptr,
                                                              (0 as
                                                                  libc::c_int
                                                                  == 0) as
                                                                  libc::c_int);
                                    *strchr(key.as_mut_ptr().offset(3 as
                                        libc::c_int
                                        as
                                        isize),
                                            ':' as i32) =
                                        '\u{0}' as i32 as libc::c_char;
                                    ex_prob =
                                        get_general_probability(key.as_mut_ptr(),
                                                                b"KEY\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
                                    if ex_prob != -13.815511f64 {
                                        if prob ==
                                            0 as libc::c_int as
                                                libc::c_double {
                                            tmp_score = -13.815511f64
                                        } else { tmp_score = log(prob) }
                                        ex_case_prob = tmp_score;
                                        if ex_case_prob != -13.815511f64 ||
                                            ex_prob == -13.815511f64 {
                                            case_frame_exist_flag =
                                                1 as libc::c_int
                                        }
                                        tmp_score -= ex_prob;
                                        if tmp_score >
                                            *of_ptr.offset((feature_base +
                                                2 as
                                                    libc::c_int)
                                                as isize) {
                                            if tmp_score > max_pmi_0 {
                                                max_pmi_0 = tmp_score
                                            }
                                            *of_ptr.offset((feature_base +
                                                2 as
                                                    libc::c_int)
                                                as isize) =
                                                tmp_score;
                                            *of_ptr.offset((feature_base +
                                                19 as
                                                    libc::c_int)
                                                as isize) =
                                                ex_case_prob;
                                            *of_ptr.offset((feature_base +
                                                20 as
                                                    libc::c_int)
                                                as isize) =
                                                ex_prob
                                        }
                                        if tmp_score > tmp_ne_ct_score {
                                            tmp_ne_ct_score = tmp_score
                                        }
                                    }
                                    k = 0 as libc::c_int;
                                    while k < 21 as libc::c_int {
                                        if strcmp(category_list[k as usize],
                                                  category_ptr) == 0 {
                                            l = 0 as libc::c_int;
                                            while l < 21 as libc::c_int {
                                                if strcmp(alternate_category[k
                                                    as
                                                    usize][l
                                                    as
                                                    usize],
                                                          b"\x00" as *const u8
                                                              as
                                                              *const libc::c_char)
                                                    == 0 {
                                                    break;
                                                }
                                                sprintf(key.as_mut_ptr(),
                                                        b"CT:%s:\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        alternate_category[k
                                                            as
                                                            usize][l
                                                            as
                                                            usize]);
                                                prob =
                                                    get_ex_ne_probability(key.as_mut_ptr(),
                                                                          e_num,
                                                                          (*ctm_ptr).cf_ptr,
                                                                          (0
                                                                              as
                                                                              libc::c_int
                                                                              ==
                                                                              0)
                                                                              as
                                                                              libc::c_int);
                                                *strchr(key.as_mut_ptr().offset(3
                                                    as
                                                    libc::c_int
                                                    as
                                                    isize),
                                                        ':' as i32) =
                                                    '\u{0}' as i32 as
                                                        libc::c_char;
                                                ex_prob =
                                                    get_general_probability(key.as_mut_ptr(),
                                                                            b"KEY\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                                                if ex_prob != -13.815511f64 {
                                                    if prob ==
                                                        0 as libc::c_int as
                                                            libc::c_double
                                                    {
                                                        tmp_score =
                                                            -13.815511f64
                                                    } else {
                                                        tmp_score = log(prob)
                                                    }
                                                    ex_case_prob = tmp_score;
                                                    if ex_case_prob !=
                                                        -13.815511f64 ||
                                                        ex_prob ==
                                                            -13.815511f64 {
                                                        case_frame_exist_flag
                                                            = 1 as libc::c_int
                                                    }
                                                    tmp_score -= ex_prob;
                                                    if tmp_score >
                                                        *of_ptr.offset((feature_base
                                                            +
                                                            25
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize)
                                                    {
                                                        *of_ptr.offset((feature_base
                                                            +
                                                            25
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize)
                                                            = tmp_score;
                                                        *of_ptr.offset((feature_base
                                                            +
                                                            26
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize)
                                                            = ex_case_prob;
                                                        *of_ptr.offset((feature_base
                                                            +
                                                            27
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize)
                                                            = ex_prob
                                                    }
                                                }
                                                l += 1
                                            }
                                        }
                                        k += 1
                                    }
                                    (strcmp(b"\xe4\xba\xba\x00" as *const u8
                                                as *const libc::c_char,
                                            category_ptr) == 0) ||
                                        strcmp(b"\xe7\xb5\x84\xe7\xb9\x94\xe3\x83\xbb\xe5\x9b\xa3\xe4\xbd\x93\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               category_ptr) == 0;
                                    category_ptr =
                                        strtok(0 as *mut libc::c_char,
                                               b";\x00" as *const u8 as
                                                   *const libc::c_char)
                                }
                            }
                            if OptGeneralCF & 1 as libc::c_int != 0 &&
                                {
                                    cp =
                                        check_feature((*(*(*entity_ptr_0).mention[j
                                            as
                                            usize]).tag_ptr).f,
                                                      b"NE\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    !cp.is_null()
                                } {
                                prob =
                                    get_ex_ne_probability(cp, e_num,
                                                          (*ctm_ptr).cf_ptr,
                                                          (0 as libc::c_int ==
                                                              0) as
                                                              libc::c_int);
                                strcpy(key.as_mut_ptr(), cp);
                                *strchr(key.as_mut_ptr().offset(3 as
                                    libc::c_int
                                    as isize),
                                        ':' as i32) =
                                    '\u{0}' as i32 as libc::c_char;
                                ex_prob =
                                    get_general_probability(key.as_mut_ptr(),
                                                            b"KEY\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
                                if ex_prob != -13.815511f64 {
                                    if prob ==
                                        0 as libc::c_int as libc::c_double
                                    {
                                        tmp_score = -13.815511f64
                                    } else { tmp_score = log(prob) }
                                    ex_case_prob = tmp_score;
                                    if ex_case_prob != -13.815511f64 ||
                                        ex_prob == -13.815511f64 {
                                        case_frame_exist_flag =
                                            1 as libc::c_int
                                    }
                                    tmp_score -= ex_prob;
                                    if tmp_score >
                                        *of_ptr.offset((feature_base +
                                            3 as
                                                libc::c_int)
                                            as isize) {
                                        if tmp_score > max_pmi_0 {
                                            max_pmi_0 = tmp_score
                                        }
                                        *of_ptr.offset((feature_base +
                                            3 as libc::c_int)
                                            as isize) =
                                            tmp_score;
                                        *of_ptr.offset((feature_base +
                                            21 as libc::c_int)
                                            as isize) =
                                            ex_case_prob;
                                        *of_ptr.offset((feature_base +
                                            22 as libc::c_int)
                                            as isize) = ex_prob
                                    }
                                    if tmp_score > tmp_ne_ct_score {
                                        tmp_ne_ct_score = tmp_score
                                    }
                                }
                            }
                            ex_case_prob =
                                get_ex_probability((*ctm_ptr).tcf_element_num_functional[i
                                    as
                                    usize],
                                                   &mut (*tcf_ptr).cf,
                                                   (*(*entity_ptr_0).mention[j
                                                       as
                                                       usize]).tag_ptr,
                                                   e_num, (*ctm_ptr).cf_ptr,
                                                   0 as libc::c_int);
                            tmp_score = ex_case_prob;
                            ex_prob =
                                get_key_probability((*(*entity_ptr_0).mention[j
                                    as
                                    usize]).tag_ptr);
                            tmp_score -= ex_prob;
                            if ex_case_prob != -13.815511f64 ||
                                ex_prob == -13.815511f64 {
                                case_frame_exist_flag = 1 as libc::c_int
                            }
                            if ex_prob != -13.815511f64 ||
                                ex_case_prob != -13.815511f64 {
                                if tmp_score >
                                    *of_ptr.offset((feature_base +
                                        1 as libc::c_int)
                                        as isize) {
                                    if tmp_score > max_pmi_0 {
                                        max_pmi_0 = tmp_score
                                    }
                                    *of_ptr.offset((feature_base +
                                        1 as libc::c_int) as
                                        isize) = tmp_score;
                                    *of_ptr.offset((feature_base +
                                        17 as libc::c_int) as
                                        isize) = ex_case_prob;
                                    *of_ptr.offset((feature_base +
                                        18 as libc::c_int) as
                                        isize) = ex_prob
                                }
                            }
                            *of_ptr.offset((feature_base + 6 as libc::c_int)
                                as isize) = max_pmi_0;
                            if OptGeneralCF & 1 as libc::c_int != 0 &&
                                !check_feature((*(*(*entity_ptr_0).mention[j
                                    as
                                    usize]).tag_ptr).f,
                                               b"NE:PERSON\x00" as
                                                   *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null()
                                &&
                                *of_ptr.offset((feature_base +
                                    1 as libc::c_int) as
                                    isize) <
                                    0 as libc::c_int as libc::c_double &&
                                *of_ptr.offset((feature_base +
                                    1 as libc::c_int) as
                                    isize) <
                                    *of_ptr.offset((feature_base +
                                        3 as libc::c_int)
                                        as isize) {
                                *of_ptr.offset((feature_base +
                                    1 as libc::c_int) as
                                    isize) =
                                    0 as libc::c_int as libc::c_double
                            }
                            if ex_case_prob > -13.815511f64 &&
                                tmp_ne_ct_score > -13.815511f64 {
                                tmp_score =
                                    (tmp_score + tmp_ne_ct_score) /
                                        2 as libc::c_int as libc::c_double
                            } else if tmp_ne_ct_score > -13.815511f64 {
                                tmp_score = tmp_ne_ct_score
                            }
                            if tmp_score > max_score { max_score = tmp_score }
                        }
                        j += 1
                    }
                }
                if case_frame_exist_flag == 0 as libc::c_int {
                    return -(10000 as libc::c_int) as libc::c_double;
                }
                j =
                    adj_flag *
                        ((4 as libc::c_int +
                            (33 as libc::c_int + 5 as libc::c_int) +
                            3 as libc::c_int + 2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int * 3 as libc::c_int +
                            15 as libc::c_int * 3 as libc::c_int +
                            5 as libc::c_int * 4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int * 2 as libc::c_int +
                            3 as libc::c_int * 2 as libc::c_int +
                            4 as libc::c_int + 10 as libc::c_int) *
                            (3 as libc::c_int + 5 as libc::c_int)) +
                        (4 as libc::c_int +
                            (33 as libc::c_int + 5 as libc::c_int) +
                            3 as libc::c_int + 2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int * 3 as libc::c_int +
                            15 as libc::c_int * 3 as libc::c_int +
                            5 as libc::c_int * 4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int * 2 as libc::c_int +
                            3 as libc::c_int * 2 as libc::c_int +
                            4 as libc::c_int + 10 as libc::c_int);
                while j <
                    adj_flag *
                        ((4 as libc::c_int +
                            (33 as libc::c_int + 5 as libc::c_int) +
                            3 as libc::c_int + 2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int * 3 as libc::c_int +
                            15 as libc::c_int * 3 as libc::c_int +
                            5 as libc::c_int * 4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int * 2 as libc::c_int +
                            3 as libc::c_int * 2 as libc::c_int +
                            4 as libc::c_int + 10 as libc::c_int) *
                            (3 as libc::c_int + 5 as libc::c_int)) +
                        (4 as libc::c_int +
                            (33 as libc::c_int + 5 as libc::c_int) +
                            3 as libc::c_int + 2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int * 3 as libc::c_int +
                            15 as libc::c_int * 3 as libc::c_int +
                            5 as libc::c_int * 4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int * 2 as libc::c_int +
                            3 as libc::c_int * 2 as libc::c_int +
                            4 as libc::c_int + 10 as libc::c_int) *
                            (3 as libc::c_int + 5 as libc::c_int) {
                    if *of_ptr.offset((j %
                        (4 as libc::c_int +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            3 as libc::c_int +
                            2 as libc::c_int +
                            8 as libc::c_int +
                            135 as libc::c_int *
                                3 as libc::c_int +
                            15 as libc::c_int *
                                3 as libc::c_int +
                            5 as libc::c_int *
                                4 as libc::c_int +
                            5 as libc::c_int +
                            11 as libc::c_int *
                                2 as libc::c_int +
                            3 as libc::c_int *
                                2 as libc::c_int +
                            4 as libc::c_int +
                            10 as libc::c_int) +
                        adj_flag *
                            ((4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int +
                                3 as libc::c_int *
                                    2 as libc::c_int +
                                4 as libc::c_int +
                                10 as libc::c_int) *
                                (3 as libc::c_int +
                                    5 as libc::c_int)))
                        as isize) <
                        *of_ptr.offset(j as isize) {
                        *of_ptr.offset((j %
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int +
                                3 as libc::c_int *
                                    2 as libc::c_int +
                                4 as libc::c_int +
                                10 as libc::c_int) +
                            adj_flag *
                                ((4 as libc::c_int +
                                    (33 as libc::c_int +
                                        5 as libc::c_int) +
                                    3 as libc::c_int +
                                    2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int *
                                        3 as libc::c_int +
                                    15 as libc::c_int *
                                        3 as libc::c_int +
                                    5 as libc::c_int *
                                        4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int *
                                        2 as libc::c_int +
                                    3 as libc::c_int *
                                        2 as libc::c_int +
                                    4 as libc::c_int +
                                    10 as libc::c_int) *
                                    (3 as libc::c_int +
                                        5 as libc::c_int)))
                            as isize) =
                            *of_ptr.offset(j as isize)
                    }
                    j += 1
                }
                score += max_score;
                if filled_flag == 0 as libc::c_int {
                    feature_base =
                        adj_flag *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                (3 as libc::c_int + 5 as libc::c_int))
                } else {
                    feature_base =
                        adj_flag *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                (3 as libc::c_int + 5 as libc::c_int)) +
                            (4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int +
                                15 as libc::c_int * 3 as libc::c_int +
                                5 as libc::c_int * 4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int) *
                                2 as libc::c_int
                }
                if filled_flag == 0 as libc::c_int {
                    let mut closest_appearance: libc::c_int =
                        100 as libc::c_int;
                    let mut closest_wa_appearance: libc::c_int =
                        100 as libc::c_int;
                    let mut old_wa_flag: libc::c_int = 0 as libc::c_int;
                    *of_ptr.offset((feature_base + 13 as libc::c_int) as
                        isize) =
                        0 as libc::c_int as libc::c_double;
                    if feature_base_2 != -(1 as libc::c_int) {
                        *of_ptr.offset((feature_base_2 + 13 as libc::c_int) as
                            isize) =
                            0 as libc::c_int as libc::c_double
                    }
                    j = 0 as libc::c_int;
                    while j < (*entity_ptr_0).mentioned_num {
                        if (*(*entity_ptr_0).mention[j as usize]).type_0 as
                            libc::c_int == 'C' as i32 {
                            if pp_kstr_to_code((*(*entity_ptr_0).mention[j as
                                usize]).cpp_string.as_mut_ptr())
                                == pp &&
                                !(*(*(*entity_ptr_0).mention[j as
                                    usize]).tag_ptr).cf_ptr.is_null()
                                &&
                                !(*(*(*(*entity_ptr_0).mention[j as
                                    usize]).tag_ptr).cf_ptr).entry.is_null()
                                && !(*(*ctm_ptr).cf_ptr).entry.is_null() &&
                                strcmp((*(*(*(*entity_ptr_0).mention[j as
                                    usize]).tag_ptr).cf_ptr).entry,
                                       (*(*ctm_ptr).cf_ptr).entry) == 0 {
                                let ref mut fresh13 =
                                    *of_ptr.offset((feature_base +
                                        13 as libc::c_int) as
                                        isize);
                                *fresh13 += 1.;
                                if feature_base_2 != -(1 as libc::c_int) {
                                    let ref mut fresh14 =
                                        *of_ptr.offset((feature_base_2 +
                                            13 as libc::c_int)
                                            as isize);
                                    *fresh14 += 1.
                                }
                            }
                        }
                        j += 1
                    }
                    *of_ptr.offset((feature_base + 10 as libc::c_int) as
                        isize) =
                        0 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base + 11 as libc::c_int) as
                        isize) =
                        0 as libc::c_int as libc::c_double;
                    *of_ptr.offset((feature_base + 12 as libc::c_int) as
                        isize) =
                        0 as libc::c_int as libc::c_double;
                    if feature_base_2 != -(1 as libc::c_int) {
                        *of_ptr.offset((feature_base_2 + 10 as libc::c_int) as
                            isize) =
                            0 as libc::c_int as libc::c_double;
                        *of_ptr.offset((feature_base_2 + 11 as libc::c_int) as
                            isize) =
                            0 as libc::c_int as libc::c_double;
                        *of_ptr.offset((feature_base_2 + 12 as libc::c_int) as
                            isize) =
                            0 as libc::c_int as libc::c_double
                    }
                    j = 0 as libc::c_int;
                    while j < (*entity_ptr_0).mentioned_num {
                        if (*(*entity_ptr_0).mention[j as usize]).type_0 as
                            libc::c_int == 'S' as i32 ||
                            (*(*entity_ptr_0).mention[j as usize]).type_0
                                as libc::c_int == '=' as i32 {
                            let ref mut fresh15 =
                                *of_ptr.offset((feature_base +
                                    10 as libc::c_int) as
                                    isize);
                            *fresh15 += 1.;
                            if (*(*entity_ptr_0).mention[j as usize]).sent_num
                                < sent_num {
                                let ref mut fresh16 =
                                    *of_ptr.offset((feature_base +
                                        11 as libc::c_int) as
                                        isize);
                                *fresh16 += 1.
                            } else if (*(*entity_ptr_0).mention[j as
                                usize]).sent_num
                                > sent_num {
                                let ref mut fresh17 =
                                    *of_ptr.offset((feature_base +
                                        12 as libc::c_int) as
                                        isize);
                                *fresh17 += 1.
                            }
                            if (*(*entity_ptr_0).mention[j as usize]).sent_num
                                <= sent_num {
                                closest_appearance =
                                    sent_num -
                                        (*(*entity_ptr_0).mention[j as
                                            usize]).sent_num
                            }
                            if (*(*entity_ptr_0).mention[j as usize]).sent_num
                                < sent_num {
                                if !check_feature((*(*(*entity_ptr_0).mention[j
                                    as
                                    usize]).tag_ptr).f,
                                                  b"\xe3\x83\x8f\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char).is_null()
                                {
                                    closest_wa_appearance =
                                        (*(*entity_ptr_0).mention[j as
                                            usize]).sent_num
                                }
                            }
                            if feature_base_2 != -(1 as libc::c_int) {
                                let ref mut fresh18 =
                                    *of_ptr.offset((feature_base_2 +
                                        10 as libc::c_int) as
                                        isize);
                                *fresh18 += 1.;
                                if (*(*entity_ptr_0).mention[j as
                                    usize]).sent_num
                                    < sent_num {
                                    let ref mut fresh19 =
                                        *of_ptr.offset((feature_base_2 +
                                            11 as libc::c_int)
                                            as isize);
                                    *fresh19 += 1.
                                } else if (*(*entity_ptr_0).mention[j as
                                    usize]).sent_num
                                    > sent_num {
                                    let ref mut fresh20 =
                                        *of_ptr.offset((feature_base_2 +
                                            12 as libc::c_int)
                                            as isize);
                                    *fresh20 += 1.
                                }
                            }
                        }
                        j += 1
                    }
                    if closest_appearance >
                        4 as libc::c_int - 1 as libc::c_int {
                        closest_appearance =
                            4 as libc::c_int - 1 as libc::c_int
                    }
                    *of_ptr.offset((feature_base +
                        (33 as libc::c_int + 5 as libc::c_int)
                        + closest_appearance) as isize) =
                        1 as libc::c_int as libc::c_double;
                    if feature_base_2 != -(1 as libc::c_int) {
                        *of_ptr.offset((feature_base_2 +
                            (33 as libc::c_int +
                                5 as libc::c_int) +
                            closest_appearance) as isize) =
                            1 as libc::c_int as libc::c_double
                    }
                    if closest_wa_appearance != 100 as libc::c_int {
                        j = 0 as libc::c_int;
                        while j < entity_manager.num {
                            if !(j == (*entity_ptr_0).num) {
                                k = 0 as libc::c_int;
                                while k <
                                    entity_manager.entity[j as
                                        usize].mentioned_num
                                {
                                    if (*entity_manager.entity[j as
                                        usize].mention[k
                                        as
                                        usize]).type_0
                                        as libc::c_int == 'S' as i32 ||
                                        (*entity_manager.entity[j as
                                            usize].mention[k
                                            as
                                            usize]).type_0
                                            as libc::c_int == '=' as i32 {
                                        if !check_feature((*(*entity_manager.entity[j
                                            as
                                            usize].mention[k
                                            as
                                            usize]).tag_ptr).f,
                                                          b"\xe3\x83\x8f\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char).is_null()
                                        {
                                            if (*entity_manager.entity[j as
                                                usize].mention[k
                                                as
                                                usize]).sent_num
                                                <= sent_num {
                                                if closest_wa_appearance <
                                                    (*entity_manager.entity[j
                                                        as
                                                        usize].mention[k
                                                        as
                                                        usize]).sent_num
                                                {
                                                    old_wa_flag =
                                                        1 as libc::c_int
                                                }
                                            }
                                        }
                                    }
                                    k += 1
                                }
                            }
                            j += 1
                        }
                    }
                    *of_ptr.offset((feature_base + 30 as libc::c_int) as
                        isize) = old_wa_flag as libc::c_double;
                    if feature_base_2 != -(1 as libc::c_int) {
                        *of_ptr.offset((feature_base_2 + 30 as libc::c_int) as
                            isize) =
                            old_wa_flag as libc::c_double
                    }
                    max_score = -13.815511f64;
                    j = 0 as libc::c_int;
                    while j < 10 as libc::c_int {
                        *of_ptr.offset((feature_base +
                            (4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int +
                                2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int *
                                    3 as libc::c_int +
                                15 as libc::c_int *
                                    3 as libc::c_int +
                                5 as libc::c_int *
                                    4 as libc::c_int +
                                5 as libc::c_int +
                                11 as libc::c_int *
                                    2 as libc::c_int +
                                3 as libc::c_int *
                                    2 as libc::c_int +
                                4 as libc::c_int) + j) as
                            isize) =
                            context_feature[(*entity_ptr_0).num as
                                usize][j as usize];
                        if feature_base_2 != -(1 as libc::c_int) {
                            *of_ptr.offset((feature_base_2 +
                                (4 as libc::c_int +
                                    (33 as libc::c_int +
                                        5 as libc::c_int) +
                                    3 as libc::c_int +
                                    2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int *
                                        3 as libc::c_int +
                                    15 as libc::c_int *
                                        3 as libc::c_int +
                                    5 as libc::c_int *
                                        4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int *
                                        2 as libc::c_int +
                                    3 as libc::c_int *
                                        2 as libc::c_int +
                                    4 as libc::c_int) + j) as
                                isize) =
                                context_feature[(*entity_ptr_0).num as
                                    usize][j as usize]
                        }
                        j += 1
                    }
                    j = 0 as libc::c_int;
                    while j < (*entity_ptr_0).mentioned_num {
                        tmp_score = 0 as libc::c_int as libc::c_double;
                        if (*(*entity_ptr_0).mention[j as usize]).sent_num ==
                            sent_num &&
                            !check_feature((*(*(*entity_ptr_0).mention[j as
                                usize]).tag_ptr).f,
                                           b"\xe3\x83\x8f\x00" as *const u8
                                               as *const libc::c_char as
                                               *mut libc::c_char).is_null()
                        {
                            *of_ptr.offset((feature_base + 5 as libc::c_int)
                                as isize) =
                                1 as libc::c_int as libc::c_double;
                            if feature_base_2 != -(1 as libc::c_int) {
                                *of_ptr.offset((feature_base_2 +
                                    5 as libc::c_int) as
                                    isize) =
                                    1 as libc::c_int as libc::c_double
                            }
                        }
                        if !check_feature((*(*(*entity_ptr_0).mention[j as
                            usize]).tag_ptr).f,
                                          b"NE:PERSON\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            *of_ptr.offset((feature_base + 7 as libc::c_int)
                                as isize) =
                                1 as libc::c_int as libc::c_double;
                            if feature_base_2 != -(1 as libc::c_int) {
                                *of_ptr.offset((feature_base_2 +
                                    7 as libc::c_int) as
                                    isize) =
                                    1 as libc::c_int as libc::c_double
                            }
                        }
                        cp =
                            check_feature((*(*(*entity_ptr_0).mention[j as
                                usize]).tag_ptr).f,
                                          b"NE\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                        if !cp.is_null() {
                            k = 0 as libc::c_int;
                            while k < 8 as libc::c_int {
                                if !strstr(cp.offset(strlen(b"NE:\x00" as
                                    *const u8 as
                                    *const libc::c_char)
                                    as isize),
                                           ne[k as usize]).is_null() {
                                    *of_ptr.offset((feature_base +
                                        (4 as libc::c_int +
                                            (33 as
                                                libc::c_int
                                                +
                                                5 as
                                                    libc::c_int)
                                            +
                                            3 as libc::c_int
                                            +
                                            2 as libc::c_int)
                                        + k) as isize) =
                                        1 as libc::c_int as libc::c_double;
                                    if feature_base_2 != -(1 as libc::c_int) {
                                        *of_ptr.offset((feature_base_2 +
                                            (4 as libc::c_int
                                                +
                                                (33 as
                                                    libc::c_int
                                                    +
                                                    5 as
                                                        libc::c_int)
                                                +
                                                3 as
                                                    libc::c_int
                                                +
                                                2 as
                                                    libc::c_int)
                                            + k) as isize) =
                                            1 as libc::c_int as libc::c_double
                                    }
                                }
                                k += 1
                            }
                        }
                        !check_feature((*(*(*entity_ptr_0).mention[j as
                            usize]).tag_ptr).f,
                                       b"NE\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null();
                        if !((*(*entity_ptr_0).mention[j as usize]).sent_num
                            == sent_num &&
                            loc_category[(*(*(*(*entity_ptr_0).mention[j
                                as
                                usize]).tag_ptr).b_ptr).num
                                as usize] == 0) {
                            if (*tcf_ptr).cf.type_0 == 1 as libc::c_int {
                                let mut simple_loc_num: libc::c_int = 0;
                                get_location(loc_name.as_mut_ptr(), sent_num,
                                             pp_code_to_kstr(pp),
                                             (*entity_ptr_0).mention[j as
                                                 usize],
                                             0 as libc::c_int);
                                location_prob =
                                    get_general_probability(b"PMI\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            loc_name.as_mut_ptr());
                                convert_locname_id(loc_name.as_mut_ptr(),
                                                   &mut loc_num,
                                                   &mut simple_loc_num);
                                if loc_num != -(1 as libc::c_int) &&
                                    !(*(*(*entity_ptr_0).mention[j as
                                        usize]).tag_ptr).cf_ptr.is_null()
                                {
                                    let mut cf_entry_type:
                                        [libc::c_char; 256] =
                                        *::std::mem::transmute::<&[u8; 256],
                                            &mut [libc::c_char; 256]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                                    let mut mention_cf_entry_type:
                                        [libc::c_char; 256] =
                                        *::std::mem::transmute::<&[u8; 256],
                                            &mut [libc::c_char; 256]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                                    // let mut event_lift: libc::c_double = 0.;
                                    make_cf_entry_type(cf_entry_type.as_mut_ptr(),
                                                       (*ctm_ptr).cf_ptr);
                                    strncat(cf_entry_type.as_mut_ptr(),
                                            b":\x00" as *const u8 as
                                                *const libc::c_char,
                                            1 as libc::c_int as
                                                libc::c_ulong);
                                    strcat(cf_entry_type.as_mut_ptr(),
                                           pp_code_to_kstr(pp));
                                    make_cf_entry_type(mention_cf_entry_type.as_mut_ptr(),
                                                       (*(*(*entity_ptr_0).mention[j
                                                           as
                                                           usize]).tag_ptr).cf_ptr);
                                    strncat(mention_cf_entry_type.as_mut_ptr(),
                                            b":\x00" as *const u8 as
                                                *const libc::c_char,
                                            1 as libc::c_int as
                                                libc::c_ulong);
                                    strcat(mention_cf_entry_type.as_mut_ptr(),
                                           (*(*entity_ptr_0).mention[j as
                                               usize]).cpp_string.as_mut_ptr());
                                }
                                if loc_num != 1 as libc::c_int {
                                    *of_ptr.offset((feature_base +
                                        (4 as libc::c_int +
                                            (33 as
                                                libc::c_int
                                                +
                                                5 as
                                                    libc::c_int)
                                            +
                                            3 as libc::c_int
                                            +
                                            2 as libc::c_int
                                            +
                                            8 as libc::c_int)
                                        +
                                        loc_num *
                                            3 as libc::c_int)
                                        as isize) =
                                        1 as libc::c_int as libc::c_double;
                                    if feature_base_2 != -(1 as libc::c_int) {
                                        *of_ptr.offset((feature_base_2 +
                                            (4 as libc::c_int
                                                +
                                                (33 as
                                                    libc::c_int
                                                    +
                                                    5 as
                                                        libc::c_int)
                                                +
                                                3 as
                                                    libc::c_int
                                                +
                                                2 as
                                                    libc::c_int
                                                +
                                                8 as
                                                    libc::c_int)
                                            +
                                            loc_num *
                                                3 as
                                                    libc::c_int)
                                            as isize) =
                                            1 as libc::c_int as libc::c_double
                                    }
                                }
                            } else {
                                get_location(loc_name.as_mut_ptr(), sent_num,
                                             pp_code_to_kstr(pp),
                                             (*entity_ptr_0).mention[j as
                                                 usize],
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int);
                                location_prob =
                                    get_general_probability(b"T\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            loc_name.as_mut_ptr())
                            }
                            tmp_score += location_prob;
                            if tmp_score > max_score {
                                max_score = tmp_score;
                                (*ctm_ptr).elem_b_ptr[i as usize] =
                                    (*(*entity_ptr_0).mention[j as
                                        usize]).tag_ptr
                            }
                        }
                        j += 1
                    }
                }
                score += max_score
            }
            _ => {}
        }
        i += 1
    }
    e_num = 0 as libc::c_int;
    while e_num < (*(*ctm_ptr).cf_ptr).element_num {
        let mut feature_base_0: libc::c_int =
            adj_flag *
                ((4 as libc::c_int + (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int + 5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int + 4 as libc::c_int +
                    10 as libc::c_int) *
                    (3 as libc::c_int + 5 as libc::c_int));
        if (*ctm_ptr).filled_element[e_num as usize] == 0 &&
            match_ellipsis_case(pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num
                as
                usize][0
                as
                libc::c_int
                as
                usize]),
                                0 as *mut *mut libc::c_char) != 0 {
            let mut feature_case_num_2: libc::c_int = 0;
            feature_case_num_2 =
                get_ellipsis_case_num(pp_code_to_kstr((*(*ctm_ptr).cf_ptr).pp[e_num
                    as
                    usize][0
                    as
                    libc::c_int
                    as
                    usize]),
                                      0 as *mut *mut libc::c_char);
            if !(feature_case_num_2 == -(1 as libc::c_int) ||
                feature_case_num_2 == 0 as libc::c_int &&
                    ga_filled_flag == 1 as libc::c_int) {
                of_ptr =
                    (*ctm_ptr).omit_feature[feature_case_num_2 as
                        usize].as_mut_ptr();
                *of_ptr.offset((feature_base_0 + 0 as libc::c_int) as isize) =
                    get_case_probability(e_num, (*ctm_ptr).cf_ptr,
                                         0 as libc::c_int,
                                         0 as *mut CF_PRED_MGR);
                score +=
                    *of_ptr.offset((feature_base_0 + 0 as libc::c_int) as
                        isize)
            }
        }
        e_num += 1
    }
    return score;
}

#[no_mangle]
pub unsafe extern "C" fn preserve_case_candidate_ctm(mut ctm_ptr:
                                                     *mut CF_TAG_MGR,
                                                     mut start: libc::c_int,
                                                     mut num: libc::c_int)
                                                     -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut same_case_flame_num: libc::c_int = 0 as libc::c_int;
    i = start;
    while i < start + num {
        if case_candidate_ctm[i as usize].score !=
            -(10000 as libc::c_int) as libc::c_double &&
            strcmp((*(*ctm_ptr).cf_ptr).cf_id.as_mut_ptr(),
                   (*case_candidate_ctm[i as
                       usize].cf_ptr).cf_id.as_mut_ptr())
                == 0 {
            same_case_flame_num += 1;
            if same_case_flame_num >= 5 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        if (*ctm_ptr).score > case_candidate_ctm[i as usize].score {
            j = start + num - 1 as libc::c_int;
            while j > i {
                if case_candidate_ctm[(j - 1 as libc::c_int) as usize].score >
                    -(10000 as libc::c_int) as libc::c_double {
                    copy_ctm(&mut *case_candidate_ctm.as_mut_ptr().offset((j -
                        1
                            as
                            libc::c_int)
                        as
                        isize),
                             &mut *case_candidate_ctm.as_mut_ptr().offset(j as
                                 isize));
                }
                j -= 1
            }
            copy_ctm(ctm_ptr,
                     &mut *case_candidate_ctm.as_mut_ptr().offset(i as
                         isize));
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn reordering_ellipsis_result(mut result_ctm:
                                                    *mut CF_TAG_MGR)
                                                    -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cp_check: [libc::c_char; 10] = [0; 10];
    let mut copy_count: libc::c_int = 0 as libc::c_int;
    let mut max_idx: libc::c_int = 0;
    let mut temp_ellipsis_result_ctm: *mut CF_TAG_MGR = 0 as *mut CF_TAG_MGR;
    temp_ellipsis_result_ctm =
        malloc_data((::std::mem::size_of::<CF_TAG_MGR>() as
            libc::c_ulong).wrapping_mul(10 as libc::c_int as
            libc::c_ulong),
                    b"reordering_ellipsis_result\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut CF_TAG_MGR;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        cp_check[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1
    }
    while copy_count < 10 as libc::c_int {
        let mut max_score: libc::c_double =
            (-(10000 as libc::c_int) - 100 as libc::c_int) as libc::c_double;
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int {
            if cp_check[i as usize] as libc::c_int == 0 as libc::c_int &&
                (*result_ctm.offset(i as isize)).score > max_score {
                max_idx = i;
                max_score = (*result_ctm.offset(i as isize)).score
            }
            i += 1
        }
        if max_score < -(10000 as libc::c_int) as libc::c_double { break; }
        copy_ctm(&mut *result_ctm.offset(max_idx as isize),
                 &mut *temp_ellipsis_result_ctm.offset(copy_count as isize));
        copy_count += 1;
        cp_check[max_idx as usize] = 1 as libc::c_int as libc::c_char
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        copy_ctm(&mut *temp_ellipsis_result_ctm.offset(i as isize),
                 &mut *result_ctm.offset(i as isize));
        i += 1
    }
    free(temp_ellipsis_result_ctm as *mut libc::c_void);
    panic!("Reached end of non-void function without returning");
}


pub unsafe extern "C" fn copy_ctm(source_ctm: *const CF_TAG_MGR, target_ctm: *mut CF_TAG_MGR)
{
    let mut i = 0;
    (*target_ctm).score = (*source_ctm).score;
    (*target_ctm).score_def = (*source_ctm).score_def;
    (*target_ctm).case_analysis_score = (*source_ctm).case_analysis_score;
    (*target_ctm).cf_ptr = (*source_ctm).cf_ptr;
    (*target_ctm).result_num = (*source_ctm).result_num;
    (*target_ctm).annotated_result_num = (*source_ctm).annotated_result_num;
    (*target_ctm).ga_entity = (*source_ctm).ga_entity;
    (*target_ctm).case_analysis_ga_entity = (*source_ctm).case_analysis_ga_entity;
    (*target_ctm).case_result_num = (*source_ctm).case_result_num;
    while i < CF_ELEMENT_MAX {
        (*target_ctm).filled_element[i] = (*source_ctm).filled_element[i];
        (*target_ctm).non_match_element[i] = (*source_ctm).non_match_element[i];
        (*target_ctm).cf_element_num[i] = (*source_ctm).cf_element_num[i];
        (*target_ctm).tcf_element_num[i] = (*source_ctm).tcf_element_num[i];
        (*target_ctm).tcf_element_num_functional[i] = (*source_ctm).tcf_element_num_functional[i];
        (*target_ctm).entity_num[i] = (*source_ctm).entity_num[i];
        (*target_ctm).elem_b_ptr[i] = (*source_ctm).elem_b_ptr[i];
        (*target_ctm).type_0[i] = (*source_ctm).type_0[i];
        i += 1;
    }
    i = 0;
    while i < ENTITY_MAX {
        (*target_ctm).filled_entity[i] = (*source_ctm).filled_entity[i];
        i += 1;
    }
    i = 0;
    let mut j = 0;
    (*target_ctm).overt_arguments_score = (*source_ctm).overt_arguments_score;
    (*target_ctm).all_arguments_score = (*source_ctm).all_arguments_score;
    while i < ELLIPSIS_CASE_NUM {
        while j < O_FEATURE_NUM {
            (*target_ctm).omit_feature[i][j] = (*source_ctm).omit_feature[i][j];
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn preserve_ellipsis_result_ctm(mut ctm_ptr:
                                                      *mut CF_TAG_MGR,
                                                      mut start: libc::c_int,
                                                      mut num: libc::c_int,
                                                      mut result_ctm:
                                                      *mut CF_TAG_MGR)
                                                      -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = start;
    while i < start + num {
        if (*ctm_ptr).score > (*result_ctm.offset(i as isize)).score {
            j = start + num - 1 as libc::c_int;
            while j > i {
                if (*result_ctm.offset((j - 1 as libc::c_int) as isize)).score
                    > -(10000 as libc::c_int) as libc::c_double {
                    copy_ctm(&mut *result_ctm.offset((j - 1 as libc::c_int) as
                        isize),
                             &mut *result_ctm.offset(j as isize));
                }
                j -= 1
            }
            copy_ctm(ctm_ptr, &mut *result_ctm.offset(i as isize));
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn case_analysis_for_anaphora(mut tag_ptr:
                                                    *mut TAG_DATA,
                                                    mut ctm_ptr:
                                                    *mut CF_TAG_MGR,
                                                    mut i: libc::c_int,
                                                    mut r_num: libc::c_int)
                                                    -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    memset((*ctm_ptr).filled_element.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    j = 0 as libc::c_int;
    while j < r_num {
        (*ctm_ptr).filled_element[(*ctm_ptr).cf_element_num[j as usize] as
            usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        k = 0 as libc::c_int;
        while (*(*ctm_ptr).cf_ptr).samecase[k as
            usize][0 as libc::c_int as
            usize] !=
            -(10 as libc::c_int) {
            if (*(*ctm_ptr).cf_ptr).samecase[k as
                usize][0 as libc::c_int as
                usize] ==
                (*ctm_ptr).cf_element_num[j as usize] {
                (*ctm_ptr).filled_element[(*(*ctm_ptr).cf_ptr).samecase[k as
                    usize][1
                    as
                    libc::c_int
                    as
                    usize]
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            } else if (*(*ctm_ptr).cf_ptr).samecase[k as
                usize][1 as
                libc::c_int
                as usize]
                == (*ctm_ptr).cf_element_num[j as usize] {
                (*ctm_ptr).filled_element[(*(*ctm_ptr).cf_ptr).samecase[k as
                    usize][0
                    as
                    libc::c_int
                    as
                    usize]
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            }
            k += 1
        }
        j += 1
    }
    if i < (*(*tag_ptr).tcf_ptr).cf.element_num {
        let mut assinged_flag: libc::c_int = 0 as libc::c_int;
        let mut skip_flag: libc::c_int = 0 as libc::c_int;
        let mut functional_tag_cf: CASE_FRAME =
            CASE_FRAME {
                type_0: 0,
                type_flag: 0,
                element_num: 0,
                oblig: [0; 24],
                adjacent: [0; 24],
                pp: [[0; 10]; 24],
                sp: [0; 24],
                pp_str: [0 as *mut libc::c_char; 24],
                sm: [0 as *mut libc::c_char; 24],
                sm_delete: [0 as *mut libc::c_char; 24],
                sm_delete_size: [0; 24],
                sm_delete_num: [0; 24],
                sm_specify: [0 as *mut libc::c_char; 24],
                sm_specify_size: [0; 24],
                sm_specify_num: [0; 24],
                ex: [0 as *mut libc::c_char; 24],
                ex_list: [0 as *mut *mut libc::c_char; 24],
                ex_freq: [0 as *mut libc::c_int; 24],
                ex_size: [0; 24],
                ex_num: [0; 24],
                freq: [0; 24],
                semantics: [0 as *mut libc::c_char; 24],
                gex_list: [0 as *mut *mut libc::c_char; 24],
                gex_freq: [0 as *mut libc::c_double; 24],
                gex_size: [0; 24],
                gex_num: [0; 24],
                voice: 0,
                cf_address: 0,
                cf_size: 0,
                cf_id: [0; 280],
                pred_type: [0; 4],
                entry: 0 as *mut libc::c_char,
                imi: [0; 128],
                etcflag: 0,
                feature: 0 as *mut libc::c_char,
                weight: [0; 24],
                samecase: [[0; 2]; 24],
                cf_align:
                [CF_ALIGNMENT {
                    cf_id: 0 as *mut libc::c_char,
                    aligned_case: [[0; 2]; 24],
                }; 5],
                pred_b_ptr: 0 as *mut TAG_DATA,
                cf_similarity: 0.,
            };
        let mut functional_tag_elem_num: libc::c_int = 0;
        let mut assigned_case: [libc::c_int; 44] = [0; 44];
        j = 0 as libc::c_int;
        while j < 44 as libc::c_int {
            assigned_case[j as usize] = 0 as libc::c_int;
            j += 1
        }
        if (*(*tag_ptr).tcf_ptr).elem_b_ptr[i as usize] == tag_ptr {
            if skip_flag == 0 as libc::c_int {
                assinged_flag = 1 as libc::c_int;
                skip_flag = 1 as libc::c_int;
                (*ctm_ptr).non_match_element[(i - r_num) as usize] = i;
                case_analysis_for_anaphora(tag_ptr, ctm_ptr,
                                           i + 1 as libc::c_int, r_num);
            }
        } else {
            functional_tag_cf =
                (*(*tag_ptr).tcf_ptr).cf_with_functional_tag[(*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf[i
                    as
                    usize]
                    as usize];
            functional_tag_elem_num =
                (*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf_elem[i as usize];
            j = 0 as libc::c_int;
            while functional_tag_cf.pp[functional_tag_elem_num as
                usize][j as usize] !=
                -(10 as libc::c_int) {
                if functional_tag_cf.pp[functional_tag_elem_num as
                    usize][j as usize] ==
                    pp_kstr_to_code(b"\xef\xbc\x8a\x00" as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char) ||
                    (*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf[i as usize] >
                        0 as libc::c_int {
                    if skip_flag == 0 as libc::c_int {
                        assinged_flag = 1 as libc::c_int;
                        skip_flag = 1 as libc::c_int;
                        (*ctm_ptr).non_match_element[(i - r_num) as usize] =
                            i;
                        case_analysis_for_anaphora(tag_ptr, ctm_ptr,
                                                   i + 1 as libc::c_int,
                                                   r_num);
                    }
                }
                k = i + 1 as libc::c_int;
                while k < (*(*tag_ptr).tcf_ptr).cf.element_num {
                    l = 0 as libc::c_int;
                    while (*(*tag_ptr).tcf_ptr).cf.pp[k as usize][l as usize]
                        != -(10 as libc::c_int) {
                        if functional_tag_cf.pp[functional_tag_elem_num as
                            usize][j as usize] ==
                            (*(*tag_ptr).tcf_ptr).cf.pp[k as
                                usize][l as
                                usize]
                        {
                            if skip_flag == 0 as libc::c_int {
                                assinged_flag = 1 as libc::c_int;
                                skip_flag = 1 as libc::c_int;
                                (*ctm_ptr).non_match_element[(i - r_num) as
                                    usize] = i;
                                case_analysis_for_anaphora(tag_ptr, ctm_ptr,
                                                           i +
                                                               1 as
                                                                   libc::c_int,
                                                           r_num);
                            }
                        }
                        l += 1
                    }
                    k += 1
                }
                e_num = 0 as libc::c_int;
                while e_num < (*(*ctm_ptr).cf_ptr).element_num {
                    let mut analysed_case_flag: libc::c_int =
                        0 as libc::c_int;
                    k = 0 as libc::c_int;
                    while k < (*tag_ptr).mention_mgr.num {
                        if (*(*(*(*(*tag_ptr).tcf_ptr).elem_b_ptr[i as
                            usize]).mention_mgr.mention.as_mut_ptr()).entity).num
                            ==
                            (*(*tag_ptr).mention_mgr.mention[k as
                                usize].entity).num
                            &&
                            functional_tag_cf.pp[functional_tag_elem_num as
                                usize][j as usize] ==
                                pp_kstr_to_code((*tag_ptr).mention_mgr.mention[k
                                    as
                                    usize].cpp_string.as_mut_ptr())
                        {
                            analysed_case_flag = 1 as libc::c_int
                        }
                        k += 1
                    }
                    if !(OptReadFeature & 16 as libc::c_int != 0 &&
                        analysed_case_flag == 0 as libc::c_int) {
                        if (*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf[i as
                            usize]
                            != 0 &&
                            (MatchPP(functional_tag_cf.pp[functional_tag_elem_num
                                as
                                usize][j as
                                usize],
                                     b"\xe3\x82\xac\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char) != 0 ||
                                MatchPP(functional_tag_cf.pp[functional_tag_elem_num
                                    as
                                    usize][j
                                    as
                                    usize],
                                        b"\xe3\x83\xb2\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char) != 0 ||
                                MatchPP(functional_tag_cf.pp[functional_tag_elem_num
                                    as
                                    usize][j
                                    as
                                    usize],
                                        b"\xe3\x83\x8b\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char) != 0 ||
                                MatchPP(functional_tag_cf.pp[functional_tag_elem_num
                                    as
                                    usize][j
                                    as
                                    usize],
                                        b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                            *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) != 0) &&
                            (MatchPP((*(*ctm_ptr).cf_ptr).pp[e_num as
                                usize][0
                                as
                                libc::c_int
                                as
                                usize],
                                     b"\xe3\x82\xac\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char) != 0 ||
                                MatchPP((*(*ctm_ptr).cf_ptr).pp[e_num as
                                    usize][0
                                    as
                                    libc::c_int
                                    as
                                    usize],
                                        b"\xe3\x83\xb2\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char) != 0 ||
                                MatchPP((*(*ctm_ptr).cf_ptr).pp[e_num as
                                    usize][0
                                    as
                                    libc::c_int
                                    as
                                    usize],
                                        b"\xe3\x83\x8b\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char) != 0 ||
                                MatchPP((*(*ctm_ptr).cf_ptr).pp[e_num as
                                    usize][0
                                    as
                                    libc::c_int
                                    as
                                    usize],
                                        b"\xe3\x82\xac\xef\xbc\x92\x00" as
                                            *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) != 0) ||
                            functional_tag_cf.pp[functional_tag_elem_num as
                                usize][j as usize] ==
                                (*(*ctm_ptr).cf_ptr).pp[e_num as
                                    usize][0 as
                                    libc::c_int
                                    as
                                    usize]
                                &&
                                ((*(*tag_ptr).tcf_ptr).cf.type_0 !=
                                    2 as libc::c_int ||
                                    !check_feature((*(*(*tag_ptr).tcf_ptr).elem_b_ptr[i
                                        as
                                        usize]).f,
                                                   b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                                       as *const u8 as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null())
                        {
                            if (*ctm_ptr).filled_element[e_num as usize] ==
                                (0 as libc::c_int == 0) as libc::c_int {
                                if skip_flag == 0 as libc::c_int {
                                    assinged_flag = 1 as libc::c_int;
                                    skip_flag = 1 as libc::c_int;
                                    (*ctm_ptr).non_match_element[(i - r_num)
                                        as usize]
                                        = i;
                                    case_analysis_for_anaphora(tag_ptr,
                                                               ctm_ptr,
                                                               i +
                                                                   1 as
                                                                       libc::c_int,
                                                               r_num);
                                }
                            } else if !(assigned_case[(*(*ctm_ptr).cf_ptr).pp[e_num
                                as
                                usize][0
                                as
                                libc::c_int
                                as
                                usize]
                                as usize] ==
                                1 as libc::c_int) {
                                if !check_feature((*(*(*tag_ptr).tcf_ptr).elem_b_ptr[i
                                    as
                                    usize]).f,
                                                  b"\xe9\x9d\x9e\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char).is_null()
                                {
                                    if skip_flag == 0 as libc::c_int {
                                        assinged_flag = 1 as libc::c_int;
                                        skip_flag = 1 as libc::c_int;
                                        (*ctm_ptr).non_match_element[(i -
                                            r_num)
                                            as
                                            usize]
                                            = i;
                                        case_analysis_for_anaphora(tag_ptr,
                                                                   ctm_ptr,
                                                                   i +
                                                                       1 as
                                                                           libc::c_int,
                                                                   r_num);
                                    }
                                } else if !(0 as libc::c_int != 0 &&
                                    (*(*tag_ptr).tcf_ptr).cf.type_0
                                        != 2 as libc::c_int &&
                                    !check_feature((*(*(*tag_ptr).tcf_ptr).elem_b_ptr[i
                                        as
                                        usize]).f,
                                                   b"\xe5\x8a\xa9\xe8\xa9\x9e\x00"
                                                       as
                                                       *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                    &&
                                    (*(*ctm_ptr).cf_ptr).pp[e_num
                                        as
                                        usize][0
                                        as
                                        libc::c_int
                                        as
                                        usize]
                                        ==
                                        pp_kstr_to_code(b"\xe3\x83\xb2\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char
                                            as
                                            *mut libc::c_char)
                                    &&
                                    (*(*tag_ptr).tcf_ptr).cf.adjacent[i
                                        as
                                        usize]
                                        != 0 &&
                                    (*(*ctm_ptr).cf_ptr).adjacent[e_num
                                        as
                                        usize]
                                        == 0) {
                                    assinged_flag = 1 as libc::c_int;
                                    if (*(*tag_ptr).tcf_ptr).cf.type_0 ==
                                        2 as libc::c_int {
                                        (*(*ctm_ptr).cf_ptr).pp[e_num as
                                            usize][0
                                            as
                                            libc::c_int
                                            as
                                            usize]
                                            =
                                            pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                as *const u8
                                                as
                                                *const libc::c_char
                                                as
                                                *mut libc::c_char)
                                    }
                                    assigned_case[(*(*ctm_ptr).cf_ptr).pp[e_num
                                        as
                                        usize][0
                                        as
                                        libc::c_int
                                        as
                                        usize]
                                        as usize] =
                                        1 as libc::c_int;
                                    (*ctm_ptr).elem_b_ptr[r_num as usize] =
                                        (*(*tag_ptr).tcf_ptr).elem_b_ptr[i as
                                            usize];
                                    (*ctm_ptr).cf_element_num[r_num as usize]
                                        = e_num;
                                    (*ctm_ptr).tcf_element_num_functional[r_num
                                        as
                                        usize]
                                        = functional_tag_elem_num;
                                    (*ctm_ptr).tcf_element_num[r_num as usize]
                                        = i;
                                    (*ctm_ptr).type_0[r_num as usize] =
                                        if (*(*tag_ptr).tcf_ptr).map_tcf_elem_to_cf[i
                                            as
                                            usize]
                                            > 0 as libc::c_int {
                                            'O' as i32
                                        } else if (*(*tag_ptr).tcf_ptr).elem_b_num[i
                                            as
                                            usize]
                                            == -(1 as libc::c_int) {
                                            'N' as i32
                                        } else { 'C' as i32 } as libc::c_char;
                                    (*ctm_ptr).entity_num[r_num as usize] =
                                        (*(*(*(*ctm_ptr).elem_b_ptr[r_num as
                                            usize]).mention_mgr.mention.as_mut_ptr()).entity).num;
                                    case_analysis_for_anaphora(tag_ptr,
                                                               ctm_ptr,
                                                               i +
                                                                   1 as
                                                                       libc::c_int,
                                                               r_num +
                                                                   1 as
                                                                       libc::c_int);
                                }
                            }
                        }
                    }
                    e_num += 1
                }
                j += 1
            }
            if assinged_flag == 0 as libc::c_int &&
                skip_flag == 0 as libc::c_int {
                assinged_flag = 1 as libc::c_int;
                skip_flag = 1 as libc::c_int;
                (*ctm_ptr).non_match_element[(i - r_num) as usize] = i;
                case_analysis_for_anaphora(tag_ptr, ctm_ptr,
                                           i + 1 as libc::c_int, r_num);
            }
        }
    } else {
        (*ctm_ptr).case_result_num = r_num;
        (*ctm_ptr).result_num = (*ctm_ptr).case_result_num;
        (*ctm_ptr).overt_arguments_score =
            calc_score_of_ctm(ctm_ptr, (*tag_ptr).tcf_ptr);
        (*ctm_ptr).score = (*ctm_ptr).overt_arguments_score;
        preserve_case_candidate_ctm(ctm_ptr, 0 as libc::c_int,
                                    5 as libc::c_int);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ellipsis_analysis(mut tag_ptr: *mut TAG_DATA,
                                           mut ctm_ptr: *mut CF_TAG_MGR,
                                           mut i: libc::c_int,
                                           mut r_num: libc::c_int,
                                           mut gresult: *mut libc::c_char)
                                           -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut e_num: libc::c_int = 0;
    let mut exist_flag: libc::c_int = 0;
    let mut para_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut pre_filled_element: [libc::c_int; 24] = [0; 24];
    let mut pre_filled_entity: [libc::c_int; 4096] = [0; 4096];
    memcpy(pre_filled_element.as_mut_ptr() as *mut libc::c_void,
           (*ctm_ptr).filled_element.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    memcpy(pre_filled_entity.as_mut_ptr() as *mut libc::c_void,
           (*ctm_ptr).filled_entity.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(4096 as libc::c_int as
               libc::c_ulong));
    memset((*ctm_ptr).filled_element.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    memset((*ctm_ptr).filled_entity.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(4096 as libc::c_int as
               libc::c_ulong));
    j = 0 as libc::c_int;
    while j < r_num {
        (*ctm_ptr).filled_element[(*ctm_ptr).cf_element_num[j as usize] as
            usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        k = 0 as libc::c_int;
        while (*(*ctm_ptr).cf_ptr).samecase[k as
            usize][0 as libc::c_int as
            usize] !=
            -(10 as libc::c_int) {
            if (*(*ctm_ptr).cf_ptr).samecase[k as
                usize][0 as libc::c_int as
                usize] ==
                (*ctm_ptr).cf_element_num[j as usize] {
                (*ctm_ptr).filled_element[(*(*ctm_ptr).cf_ptr).samecase[k as
                    usize][1
                    as
                    libc::c_int
                    as
                    usize]
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            } else if (*(*ctm_ptr).cf_ptr).samecase[k as
                usize][1 as
                libc::c_int
                as usize]
                == (*ctm_ptr).cf_element_num[j as usize] {
                (*ctm_ptr).filled_element[(*(*ctm_ptr).cf_ptr).samecase[k as
                    usize][0
                    as
                    libc::c_int
                    as
                    usize]
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            }
            k += 1
        }
        (*ctm_ptr).filled_entity[(*ctm_ptr).entity_num[j as usize] as usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        if j < (*ctm_ptr).case_result_num &&
            !check_feature((*(*ctm_ptr).elem_b_ptr[j as usize]).f,
                           b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() &&
            (*substance_tag_ptr((*ctm_ptr).elem_b_ptr[j as
                usize])).para_type
                as libc::c_int == 1 as libc::c_int {
            k = 0 as libc::c_int;
            while !(*(*substance_tag_ptr((*ctm_ptr).elem_b_ptr[j as
                usize])).parent).child[k
                as
                usize].is_null()
            {
                para_ptr =
                    substance_tag_ptr((*(*substance_tag_ptr((*ctm_ptr).elem_b_ptr[j
                        as
                        usize])).parent).child[k
                        as
                        usize]);
                (*ctm_ptr).filled_entity[(*(*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity).num
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int;
                k += 1
            }
        } else if OptAnaphora & 32 as libc::c_int != 0 &&
            entity_manager.entity[(*ctm_ptr).entity_num[j as usize]
                as usize].hypothetical_flag ==
                0 as libc::c_int &&
            (*entity_manager.entity[(*ctm_ptr).entity_num[j as
                usize]
                as
                usize].mention[0 as
                libc::c_int
                as
                usize]).sent_num
                ==
                (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num
            &&
            (*(*entity_manager.entity[(*ctm_ptr).entity_num[j as
                usize]
                as
                usize].mention[0 as
                libc::c_int
                as
                usize]).tag_ptr).para_type
                as libc::c_int == 1 as libc::c_int {
            k = 0 as libc::c_int;
            while !(*(*(*entity_manager.entity[(*ctm_ptr).entity_num[j as
                usize]
                as
                usize].mention[0 as
                libc::c_int
                as
                usize]).tag_ptr).parent).child[k
                as
                usize].is_null()
            {
                para_ptr =
                    substance_tag_ptr((*(*(*entity_manager.entity[(*ctm_ptr).entity_num[j
                        as
                        usize]
                        as
                        usize].mention[0
                        as
                        libc::c_int
                        as
                        usize]).tag_ptr).parent).child[k
                        as
                        usize]);
                (*ctm_ptr).filled_entity[(*(*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity).num
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int;
                k += 1
            }
        }
        j += 1
    }
    (*ctm_ptr).filled_entity[(*(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).entity).num
        as usize] =
        (0 as libc::c_int == 0) as libc::c_int;
    if !(*tag_ptr).parent.is_null() &&
        check_feature((*(*tag_ptr).parent).f,
                      b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() {
        (*ctm_ptr).filled_entity[(*(*(*substance_tag_ptr((*tag_ptr).parent)).mention_mgr.mention.as_mut_ptr()).entity).num
            as usize] =
            (0 as libc::c_int == 0) as libc::c_int
    }
    if check_analyze_tag(tag_ptr, 0 as libc::c_int) == 2 as libc::c_int &&
        (*tag_ptr).para_type as libc::c_int == 1 as libc::c_int &&
        !(*tag_ptr).parent.is_null() &&
        (*(*tag_ptr).parent).para_top_p as libc::c_int != 0 {
        j = 0 as libc::c_int;
        while !(*(*tag_ptr).parent).child[j as usize].is_null() {
            para_ptr =
                substance_tag_ptr((*(*tag_ptr).parent).child[j as usize]);
            if para_ptr != tag_ptr &&
                !check_feature((*para_ptr).f,
                               b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                (*para_ptr).para_type as libc::c_int == 1 as libc::c_int &&
                (*para_ptr).dpnd_type as libc::c_int == 'P' as i32 {
                (*ctm_ptr).filled_entity[(*(*(*para_ptr).mention_mgr.mention.as_mut_ptr()).entity).num
                    as usize] =
                    (0 as libc::c_int == 0) as libc::c_int
            }
            j += 1
        }
    }
    if **ELLIPSIS_CASE_LIST.offset(i as isize) != 0 {
        exist_flag = 0 as libc::c_int;
        e_num = 0 as libc::c_int;
        while e_num < (*(*ctm_ptr).cf_ptr).element_num {
            if (*(*tag_ptr).tcf_ptr).cf.type_0 == 2 as libc::c_int {
                (*(*ctm_ptr).cf_ptr).pp[e_num as
                    usize][0 as libc::c_int as usize]
                    =
                    pp_kstr_to_code(b"\xe3\x83\x8e\x00" as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char);
                if e_num == i &&
                    strcmp(b"\xe3\x83\x8e\x00" as *const u8 as
                               *const libc::c_char,
                           *ELLIPSIS_CASE_LIST.offset(i as isize)) == 0 {
                    break;
                }
            } else if (*(*ctm_ptr).cf_ptr).pp[e_num as
                usize][0 as libc::c_int as
                usize] ==
                pp_kstr_to_code(*ELLIPSIS_CASE_LIST.offset(i as
                    isize))
            {
                break;
            }
            e_num += 1
        }
        if e_num == (*(*ctm_ptr).cf_ptr).element_num {
            ellipsis_analysis(tag_ptr, ctm_ptr, i + 1 as libc::c_int, r_num,
                              gresult);
        } else if (*ctm_ptr).filled_element[e_num as usize] ==
            (0 as libc::c_int == 0) as libc::c_int {
            ellipsis_analysis(tag_ptr, ctm_ptr, i + 1 as libc::c_int, r_num,
                              gresult);
        } else {
            let mut current_block_53: u64;
            k = 0 as libc::c_int;
            while k < entity_manager.num {
                if entity_manager.entity[k as usize].hypothetical_flag ==
                    0 as libc::c_int {
                    if entity_manager.entity[k as usize].salience_score == 0.
                        &&
                        !(!(*tag_ptr).parent.is_null() &&
                            (*(*(*substance_tag_ptr((*tag_ptr).parent)).mention_mgr.mention.as_mut_ptr()).entity).num
                                == entity_manager.entity[k as usize].num)
                        &&
                        !((*(*tag_ptr).tcf_ptr).cf.type_0 ==
                            2 as libc::c_int &&
                            entity_manager.entity[k as
                                usize].tmp_salience_flag
                                != 0) {
                        current_block_53 = 6072622540298447352;
                    } else if !check_feature((*(*entity_manager.entity[k as
                        usize].mention[0
                        as
                        libc::c_int
                        as
                        usize]).tag_ptr).f,
                                             b"\xe7\x96\x91\xe5\x95\x8f\xe8\xa9\x9e\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                    {
                        current_block_53 = 6072622540298447352;
                    } else { current_block_53 = 12930649117290160518; }
                } else { current_block_53 = 12930649117290160518; }
                match current_block_53 {
                    12930649117290160518 => {
                        if !(OptAnaphora & 32 as libc::c_int == 0 &&
                            entity_manager.entity[k as
                                usize].hypothetical_flag
                                == 1 as libc::c_int) {
                            if !(entity_manager.entity[k as usize].skip_flag
                                == 1 as libc::c_int) {
                                if !((*ctm_ptr).filled_entity[k as usize] !=
                                    0) {
                                    if !(candidate_entities[k as usize] ==
                                        0 as libc::c_int) {
                                        (*ctm_ptr).cf_element_num[r_num as
                                            usize] =
                                            e_num;
                                        (*ctm_ptr).entity_num[r_num as usize]
                                            = k;
                                        ellipsis_analysis(tag_ptr, ctm_ptr,
                                                          i +
                                                              1 as
                                                                  libc::c_int,
                                                          r_num +
                                                              1 as
                                                                  libc::c_int,
                                                          gresult);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                k += 1
            }
            ellipsis_analysis(tag_ptr, ctm_ptr, i + 1 as libc::c_int, r_num,
                              gresult);
        }
    } else {
        let mut score: libc::c_double = 0.;
        (*ctm_ptr).result_num = r_num;
        j = (*ctm_ptr).case_result_num;
        while j < r_num {
            if OptAnaphora & 32 as libc::c_int != 0 &&
                entity_manager.entity[(*ctm_ptr).entity_num[j as usize] as
                    usize].hypothetical_flag ==
                    1 as libc::c_int {
                (*ctm_ptr).type_0[j as usize] = 'E' as i32 as libc::c_char
            } else {
                (*ctm_ptr).type_0[j as usize] = 'O' as i32 as libc::c_char
            }
            j += 1
        }
        if OptAnaphora & 8 as libc::c_int != 0 ||
            (*(*tag_ptr).tcf_ptr).cf.type_0 == 2 as libc::c_int {
            (*ctm_ptr).score =
                calc_ellipsis_score_of_ctm(ctm_ptr, (*tag_ptr).tcf_ptr,
                                           tag_ptr) +
                    (*ctm_ptr).overt_arguments_score;
            (*ctm_ptr).score_def = (*ctm_ptr).score
        } else {
            let mut calc_flag: libc::c_int = 1 as libc::c_int;
            let mut aresult: [libc::c_char; 256] = [0; 256];
            make_aresult_string(ctm_ptr, aresult.as_mut_ptr());
            if calc_flag == 1 as libc::c_int {
                score =
                    calc_ellipsis_score_of_ctm(ctm_ptr, (*tag_ptr).tcf_ptr,
                                               tag_ptr);
                if score != -(10000 as libc::c_int) as libc::c_double {
                    (*ctm_ptr).all_arguments_score =
                        0 as libc::c_int as libc::c_double;
                    (*ctm_ptr).case_analysis_score =
                        calc_score_of_case_frame_assingemnt(ctm_ptr,
                                                            (*tag_ptr).tcf_ptr);
                    if OptAnaphora & 64 as libc::c_int != 0 {
                        (*ctm_ptr).score = (*ctm_ptr).case_analysis_score
                    } else {
                        (*ctm_ptr).score =
                            (*ctm_ptr).overt_arguments_score *
                                overt_arguments_weight;
                        (*ctm_ptr).score +=
                            (*ctm_ptr).all_arguments_score *
                                all_arguments_weight;
                        j = 0 as libc::c_int;
                        while j < 4 as libc::c_int {
                            k = 0 as libc::c_int;
                            while k <
                                2 as libc::c_int *
                                    ((4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int) *
                                        (3 as libc::c_int +
                                            5 as libc::c_int)) {
                                (*ctm_ptr).score +=
                                    if (*ctm_ptr).omit_feature[j as
                                        usize][k as
                                        usize]
                                        ==
                                        -(10000 as libc::c_int) as
                                            libc::c_double {
                                        0 as libc::c_int as libc::c_double
                                    } else {
                                        ((*ctm_ptr).omit_feature[j as
                                            usize][k
                                            as
                                            usize])
                                            *
                                            case_feature_weight[j as
                                                usize][k
                                                as
                                                usize]
                                    };
                                k += 1
                            }
                            j += 1
                        }
                    }
                    (*ctm_ptr).score_def =
                        (*ctm_ptr).overt_arguments_score *
                            def_overt_arguments_weight;
                    (*ctm_ptr).score_def +=
                        (*ctm_ptr).all_arguments_score *
                            def_all_arguments_weight;
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int {
                        k = 0 as libc::c_int;
                        while k <
                            2 as libc::c_int *
                                ((4 as libc::c_int +
                                    (33 as libc::c_int +
                                        5 as libc::c_int) +
                                    3 as libc::c_int +
                                    2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int *
                                        3 as libc::c_int +
                                    15 as libc::c_int *
                                        3 as libc::c_int +
                                    5 as libc::c_int *
                                        4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int *
                                        2 as libc::c_int +
                                    3 as libc::c_int *
                                        2 as libc::c_int +
                                    4 as libc::c_int +
                                    10 as libc::c_int) *
                                    (3 as libc::c_int +
                                        5 as libc::c_int)) {
                            (*ctm_ptr).score_def +=
                                if (*ctm_ptr).omit_feature[j as
                                    usize][k as
                                    usize]
                                    ==
                                    -(10000 as libc::c_int) as
                                        libc::c_double {
                                    0 as libc::c_int as libc::c_double
                                } else {
                                    ((*ctm_ptr).omit_feature[j as
                                        usize][k as
                                        usize])
                                        *
                                        def_case_feature_weight[j as
                                            usize][k
                                            as
                                            usize]
                                };
                            k += 1
                        }
                        j += 1
                    }
                } else {
                    (*ctm_ptr).score =
                        -(10000 as libc::c_int) as libc::c_double
                }
            }
        }
        if (*(*tag_ptr).tcf_ptr).cf.type_0 == 2 as libc::c_int &&
            check_analyze_tag(tag_ptr,
                              (0 as libc::c_int == 0) as libc::c_int) != 0
            && r_num == 0 as libc::c_int {
            (*ctm_ptr).score += -1.3863f64
        }
        if (*ctm_ptr).score != -(10000 as libc::c_int) as libc::c_double {
            preserve_ellipsis_result_ctm(ctm_ptr, 0 as libc::c_int,
                                         10 as libc::c_int,
                                         ellipsis_result_ctm.as_mut_ptr());
        }
    }
    memcpy((*ctm_ptr).filled_element.as_mut_ptr() as *mut libc::c_void,
           pre_filled_element.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(24 as libc::c_int as
               libc::c_ulong));
    memcpy((*ctm_ptr).filled_entity.as_mut_ptr() as *mut libc::c_void,
           pre_filled_entity.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
               libc::c_ulong).wrapping_mul(4096 as libc::c_int as
               libc::c_ulong));
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ellipsis_analysis_main(mut tag_ptr: *mut TAG_DATA)
                                                -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut frame_num: libc::c_int = 0 as libc::c_int;
    let mut rnum_check_flag: libc::c_int = 0;
    // let mut cp: [libc::c_char; 256] = [0; 256];
    let mut aresult: [libc::c_char; 256] = [0; 256];
    let mut gresult: [libc::c_char; 256] = [0; 256];
    let mut cf_aresult: [libc::c_char; 256] = [0; 256];
    let mut cf_array: *mut *mut CASE_FRAME = 0 as *mut *mut CASE_FRAME;
    // let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut work: CF_TAG_MGR =
        CF_TAG_MGR {
            score: 0.,
            score_def: 0.,
            case_analysis_score: 0.,
            cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
            filled_element: [0; 24],
            filled_entity: [0; 4096],
            non_match_element: [0; 24],
            result_num: 0,
            case_result_num: 0,
            annotated_result_num: 0,
            cf_element_num: [0; 24],
            tcf_element_num: [0; 24],
            tcf_element_num_functional: [0; 24],
            elem_b_ptr: [0 as *const TAG_DATA as *mut TAG_DATA; 24],
            entity_num: [0; 24],
            type_0: [0; 24],
            ga_entity: 0,
            case_analysis_ga_entity: 0,
            overt_arguments_score: 0.,
            all_arguments_score: 0.,
            omit_feature: [[0.; 9152]; 4],
        };
    let mut ctm_ptr: *mut CF_TAG_MGR = 0 as *mut CF_TAG_MGR;
    let mut gs_ctm_ptr: *mut CF_TAG_MGR = 0 as *mut CF_TAG_MGR;
    ctm_ptr = &mut work;
    cf_array =
        malloc_data((::std::mem::size_of::<*mut CASE_FRAME>() as
            libc::c_ulong).wrapping_mul((*tag_ptr).cf_num as
            libc::c_ulong),
                    b"ellipsis_analysis_main\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut *mut CASE_FRAME;
    frame_num = set_cf_candidate(tag_ptr, cf_array);
    if OptDisplay == 3 as libc::c_int {
        printf(b";;CASE FRAME NUM: %d\n\x00" as *const u8 as
                   *const libc::c_char, frame_num);
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        case_candidate_ctm[i as usize].score =
            -(10000 as libc::c_int) as libc::c_double;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        ellipsis_result_ctm[i as usize].score =
            -(10000 as libc::c_int) as libc::c_double;
        ellipsis_result_ctm[i as usize].score_def =
            -(10000 as libc::c_int) as libc::c_double;
        ellipsis_result_ctm[i as usize].case_analysis_score =
            -(10000 as libc::c_int) as libc::c_double;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        ellipsis_correct_ctm[i as usize].score =
            -(10000 as libc::c_int) as libc::c_double;
        ellipsis_correct_ctm[i as usize].score_def =
            -(10000 as libc::c_int) as libc::c_double;
        ellipsis_correct_ctm[i as usize].case_analysis_score =
            -(10000 as libc::c_int) as libc::c_double;
        ellipsis_correct_ctm[i as usize].result_num = -(1 as libc::c_int);
        ellipsis_correct_ctm[i as usize].annotated_result_num =
            -(1 as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < frame_num {
        if !((**cf_array.offset(i as isize)).etcflag & 1 as libc::c_int != 0
            && frame_num != 1 as libc::c_int) {
            (*ctm_ptr).score = -(10000 as libc::c_int) as libc::c_double;
            (*ctm_ptr).score_def = -(10000 as libc::c_int) as libc::c_double;
            (*ctm_ptr).case_analysis_score =
                -(10000 as libc::c_int) as libc::c_double;
            (*ctm_ptr).cf_ptr = *cf_array.offset(i as isize);
            if OptAnaphora & 16 as libc::c_int != 0 {
                let mut temp_ctm: CF_TAG_MGR =
                    CF_TAG_MGR {
                        score: 0.,
                        score_def: 0.,
                        case_analysis_score: 0.,
                        cf_ptr:
                        0 as *const CASE_FRAME as *mut CASE_FRAME,
                        filled_element: [0; 24],
                        filled_entity: [0; 4096],
                        non_match_element: [0; 24],
                        result_num: 0,
                        case_result_num: 0,
                        annotated_result_num: 0,
                        cf_element_num: [0; 24],
                        tcf_element_num: [0; 24],
                        tcf_element_num_functional: [0; 24],
                        elem_b_ptr:
                        [0 as *const TAG_DATA as *mut TAG_DATA;
                            24],
                        entity_num: [0; 24],
                        type_0: [0; 24],
                        ga_entity: 0,
                        case_analysis_ga_entity: 0,
                        overt_arguments_score: 0.,
                        all_arguments_score: 0.,
                        omit_feature: [[0.; 9152]; 4],
                    };
                copy_ctm(ctm_ptr, &mut temp_ctm);
            }
            case_analysis_for_anaphora(tag_ptr, ctm_ptr, 0 as libc::c_int,
                                       0 as libc::c_int);
        }
        i += 1
    }
    if OptAnaphora & 16 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            let mut temp_ctm_0: CF_TAG_MGR =
                CF_TAG_MGR {
                    score: 0.,
                    score_def: 0.,
                    case_analysis_score: 0.,
                    cf_ptr: 0 as *const CASE_FRAME as *mut CASE_FRAME,
                    filled_element: [0; 24],
                    filled_entity: [0; 4096],
                    non_match_element: [0; 24],
                    result_num: 0,
                    case_result_num: 0,
                    annotated_result_num: 0,
                    cf_element_num: [0; 24],
                    tcf_element_num: [0; 24],
                    tcf_element_num_functional: [0; 24],
                    elem_b_ptr:
                    [0 as *const TAG_DATA as *mut TAG_DATA; 24],
                    entity_num: [0; 24],
                    type_0: [0; 24],
                    ga_entity: 0,
                    case_analysis_ga_entity: 0,
                    overt_arguments_score: 0.,
                    all_arguments_score: 0.,
                    omit_feature: [[0.; 9152]; 4],
                };
            let mut ctm_ptr_0: *mut CF_TAG_MGR = &mut temp_ctm_0;
            if ellipsis_correct_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double {
                printf(b";;\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x82\xab\xe3\x83\x90\xe3\x83\xac\xe3\x83\x83\xe3\x82\xb8\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf%d-%d:%2d %.3f\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num,
                       (*tag_ptr).num, i + 1 as libc::c_int,
                       -(10000 as libc::c_int));
            } else {
                copy_ctm(&mut *ellipsis_correct_ctm.as_mut_ptr().offset(i as isize), ctm_ptr_0);
                make_case_assingment_string(ctm_ptr_0, aresult.as_mut_ptr());
                printf(b";;\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x82\xab\xe3\x83\x90\xe3\x83\xac\xe3\x83\x83\xe3\x82\xb8\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf%d-%d:%2d %.3f %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num,
                       (*tag_ptr).num, i + 1 as libc::c_int,
                       (*ctm_ptr_0).score,
                       (*(*ctm_ptr_0).cf_ptr).cf_id.as_mut_ptr());
                j = 0 as libc::c_int;
                while j < (*ctm_ptr_0).result_num {
                    printf(b" %s%s:%s%d\x00" as *const u8 as
                               *const libc::c_char,
                           if j < (*ctm_ptr_0).case_result_num {
                               b"\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"*\x00" as *const u8 as *const libc::c_char
                           },
                           pp_code_to_kstr((*(*ctm_ptr_0).cf_ptr).pp[(*ctm_ptr_0).cf_element_num[j
                               as
                               usize]
                               as
                               usize][0
                               as
                               libc::c_int
                               as
                               usize]),
                           (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr_0).entity_num[j
                               as
                               usize]
                               as
                               isize)).name.as_mut_ptr(),
                           (*entity_manager.entity.as_mut_ptr().offset((*ctm_ptr_0).entity_num[j
                               as
                               usize]
                               as
                               isize)).num);
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < (*(*ctm_ptr_0).cf_ptr).element_num {
                    if (*ctm_ptr_0).filled_element[j as usize] == 0 &&
                        match_ellipsis_case(pp_code_to_kstr((*(*ctm_ptr_0).cf_ptr).pp[j
                            as
                            usize][0
                            as
                            libc::c_int
                            as
                            usize]),
                                            0 as *mut *mut libc::c_char) !=
                            0 {
                        printf(b" %s:%s\x00" as *const u8 as
                                   *const libc::c_char,
                               pp_code_to_kstr((*(*ctm_ptr_0).cf_ptr).pp[j as
                                   usize][0
                                   as
                                   libc::c_int
                                   as
                                   usize]),
                               if (*(*ctm_ptr_0).cf_ptr).oblig[j as usize] !=
                                   0 {
                                   b"\xc3\x97\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"-\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    }
                    j += 1
                }
                printf(b" (0:%.2f*%.2f\x00" as *const u8 as
                           *const libc::c_char,
                       (*ctm_ptr_0).overt_arguments_score,
                       overt_arguments_weight);
                printf(b" 1:%.2f\x00" as *const u8 as *const libc::c_char,
                       (*ctm_ptr_0).all_arguments_score);
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    printf(b"|%s\x00" as *const u8 as *const libc::c_char,
                           ELLIPSIS_CASE_LIST_VERB[j as usize]);
                    k = 0 as libc::c_int;
                    while k <
                        2 as libc::c_int *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int)
                                + 3 as libc::c_int + 2 as libc::c_int
                                + 8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int
                                + 15 as libc::c_int * 3 as libc::c_int
                                + 5 as libc::c_int * 4 as libc::c_int
                                + 5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int)
                                *
                                (3 as libc::c_int + 5 as libc::c_int))
                    {
                        if (*ctm_ptr_0).omit_feature[j as usize][k as usize]
                            != -(10000 as libc::c_int) as libc::c_double &&
                            (*ctm_ptr_0).omit_feature[j as
                                usize][k as
                                usize]
                                != 0 as libc::c_int as libc::c_double &&
                            case_feature_weight[j as usize][k as usize] !=
                                0 as libc::c_int as libc::c_double {
                            let mut feature_name: [libc::c_char; 5120] =
                                *::std::mem::transmute::<&[u8; 5120],
                                    &mut [libc::c_char; 5120]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                == 15 as libc::c_int {
                                strcpy(feature_name.as_mut_ptr(),
                                       b"A\x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                == 16 as libc::c_int {
                                strcpy(feature_name.as_mut_ptr(),
                                       b"R\x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int {
                                sprintf(feature_name.as_mut_ptr(),
                                        b"L%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int)) /
                                            3 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int {
                                sprintf(feature_name.as_mut_ptr(),
                                        b"M%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int)) /
                                            2 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int * 2 as libc::c_int
                            {
                                sprintf(feature_name.as_mut_ptr(),
                                        b"H%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int)) /
                                            2 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int * 2 as libc::c_int
                                        + 4 as libc::c_int +
                                        10 as libc::c_int {
                                sprintf(feature_name.as_mut_ptr(),
                                        b"C%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int));
                            }
                            printf(b",%s:%d:%d:%.2f*%.2f\x00" as *const u8 as
                                       *const libc::c_char,
                                   feature_name.as_mut_ptr(), k,
                                   k %
                                       (4 as libc::c_int +
                                           (33 as libc::c_int +
                                               5 as libc::c_int) +
                                           3 as libc::c_int +
                                           2 as libc::c_int +
                                           8 as libc::c_int +
                                           135 as libc::c_int *
                                               3 as libc::c_int +
                                           15 as libc::c_int *
                                               3 as libc::c_int +
                                           5 as libc::c_int *
                                               4 as libc::c_int +
                                           5 as libc::c_int +
                                           11 as libc::c_int *
                                               2 as libc::c_int +
                                           3 as libc::c_int *
                                               2 as libc::c_int +
                                           4 as libc::c_int +
                                           10 as libc::c_int),
                                   (*ctm_ptr_0).omit_feature[j as
                                       usize][k as
                                       usize],
                                   case_feature_weight[j as
                                       usize][k as
                                       usize]);
                        }
                        k += 1
                    }
                    j += 1
                }
                printf(b")\x00" as *const u8 as *const libc::c_char);
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            i += 1
        }
    }
    if case_candidate_ctm[0 as libc::c_int as usize].score ==
        -(10000 as libc::c_int) as libc::c_double {
        return 0 as libc::c_int;
    }
    if OptDisplay == 3 as libc::c_int || OptExpress == 16 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if OptReadFeature & 64 as libc::c_int != 0 ||
                OptAnaphora & 262144 as libc::c_int != 0 {
                if case_candidate_ctm[i as usize].score ==
                    -(10000 as libc::c_int) as libc::c_double ||
                    i > 0 as libc::c_int &&
                        case_candidate_ctm[i as usize].score <
                            case_candidate_ctm[(i - 1 as libc::c_int) as
                                usize].score -
                                4.6f64 / 2 as libc::c_int as libc::c_double
                {
                    break;
                }
            }
            if case_candidate_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double {
                break;
            }
            printf(b";;\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe5\x80\x99\xe8\xa3\x9c%d-%d:%2d %.3f %s\x00"
                       as *const u8 as *const libc::c_char,
                   (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num,
                   (*tag_ptr).num, i + 1 as libc::c_int,
                   case_candidate_ctm[i as usize].score,
                   (*case_candidate_ctm[i as
                       usize].cf_ptr).cf_id.as_mut_ptr());
            j = 0 as libc::c_int;
            while j < case_candidate_ctm[i as usize].result_num {
                printf(b" %s%s:%s\x00" as *const u8 as *const libc::c_char,
                       if (*case_candidate_ctm[i as
                           usize].cf_ptr).adjacent[case_candidate_ctm[i
                           as
                           usize].cf_element_num[j
                           as
                           usize]
                           as
                           usize]
                           != 0 {
                           b"*\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"-\x00" as *const u8 as *const libc::c_char
                       },
                       pp_code_to_kstr((*case_candidate_ctm[i as
                           usize].cf_ptr).pp[case_candidate_ctm[i
                           as
                           usize].cf_element_num[j
                           as
                           usize]
                           as
                           usize][0
                           as
                           libc::c_int
                           as
                           usize]),
                       (*(*case_candidate_ctm[i as
                           usize].elem_b_ptr[j as
                           usize]).head_ptr).Goi2.as_mut_ptr());
                j += 1
            }
            j = 0 as libc::c_int;
            while j < (*case_candidate_ctm[i as usize].cf_ptr).element_num {
                if case_candidate_ctm[i as usize].filled_element[j as usize]
                    == 0 &&
                    match_ellipsis_case(pp_code_to_kstr((*case_candidate_ctm[i
                        as
                        usize].cf_ptr).pp[j
                        as
                        usize][0
                        as
                        libc::c_int
                        as
                        usize]),
                                        0 as *mut *mut libc::c_char) != 0 {
                    printf(b" %s:\xc3\x97\x00" as *const u8 as
                               *const libc::c_char,
                           pp_code_to_kstr((*case_candidate_ctm[i as
                               usize].cf_ptr).pp[j
                               as
                               usize][0
                               as
                               libc::c_int
                               as
                               usize]));
                }
                j += 1
            }
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            i += 1
        }
    }
    if OptAnaphora & 16 as libc::c_int != 0 {
        make_gresult_strings(tag_ptr, gresult.as_mut_ptr());
    } else {
        gresult[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        // let mut e_num: libc::c_int = 0;
        if OptReadFeature & 64 as libc::c_int != 0 ||
            OptAnaphora & 262144 as libc::c_int != 0 {
            if case_candidate_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double ||
                i > 0 as libc::c_int &&
                    case_candidate_ctm[i as usize].score <
                        case_candidate_ctm[(i - 1 as libc::c_int) as
                            usize].score -
                            4.6f64 / 2 as libc::c_int as libc::c_double {
                break;
            }
        }
        if i > 0 as libc::c_int &&
            case_candidate_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double {
            break;
        }
        copy_ctm(&mut *case_candidate_ctm.as_mut_ptr().offset(i as isize),
                 ctm_ptr);
        ellipsis_analysis(tag_ptr, ctm_ptr, 0 as libc::c_int,
                          (*ctm_ptr).result_num, gresult.as_mut_ptr());
        i += 1
    }
    if OptAnaphora & 16 as libc::c_int != 0 {
        rnum_check_flag = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int {
            if ellipsis_result_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double {
                break;
            }
            if ellipsis_result_ctm[i as usize].result_num -
                ellipsis_result_ctm[i as usize].case_result_num >
                0 as libc::c_int {
                rnum_check_flag = 1 as libc::c_int;
                break;
            } else { i += 1 }
        }
        if rnum_check_flag != 0 {
            i = 0 as libc::c_int;
            while i < 10 as libc::c_int {
                if ellipsis_result_ctm[i as usize].score ==
                    -(10000 as libc::c_int) as libc::c_double {
                    break;
                }
                make_aresult_string(&mut *ellipsis_result_ctm.as_mut_ptr().offset(i
                    as
                    isize),
                                    aresult.as_mut_ptr());
                if gs_ctm_ptr.is_null() &&
                    strcmp(aresult.as_mut_ptr(), gresult.as_mut_ptr()) == 0
                {
                    gs_ctm_ptr =
                        ellipsis_result_ctm.as_mut_ptr().offset(i as isize)
                }
                if !((relax_compare_result(aresult.as_mut_ptr(),
                                           gresult.as_mut_ptr()) ==
                    0 as libc::c_int || rnum_check_flag == 0) &&
                    ellipsis_result_ctm[i as usize].result_num -
                        ellipsis_result_ctm[i as usize].case_result_num
                        == 0 as libc::c_int) {
                    strcpy(cf_aresult.as_mut_ptr(),
                           (*ellipsis_result_ctm[i as
                               usize].cf_ptr).cf_id.as_mut_ptr());
                    strcat(cf_aresult.as_mut_ptr(),
                           b" \x00" as *const u8 as *const libc::c_char);
                    strcat(cf_aresult.as_mut_ptr(), aresult.as_mut_ptr());
                    if svm_feaature_opt != 1 as libc::c_int {
                        printf(b";;<%s>%d FEATURE: %d, %f, %f, \x00" as
                                   *const u8 as *const libc::c_char,
                               cf_aresult.as_mut_ptr(), i,
                               if strcmp(aresult.as_mut_ptr(),
                                         gresult.as_mut_ptr()) == 0 {
                                   1 as libc::c_int
                               } else { 0 as libc::c_int },
                               ellipsis_result_ctm[i as
                                   usize].overt_arguments_score,
                               ellipsis_result_ctm[i as
                                   usize].all_arguments_score);
                        j = 0 as libc::c_int;
                        while j < 4 as libc::c_int {
                            k = 0 as libc::c_int;
                            while k <
                                2 as libc::c_int *
                                    ((4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int) *
                                        (3 as libc::c_int +
                                            5 as libc::c_int)) {
                                if ellipsis_result_ctm[i as
                                    usize].omit_feature[j
                                    as
                                    usize][k
                                    as
                                    usize]
                                    ==
                                    -(10000 as libc::c_int) as
                                        libc::c_double {
                                    printf(b" 0,\x00" as *const u8 as
                                        *const libc::c_char);
                                } else {
                                    if ellipsis_result_ctm[i as
                                        usize].omit_feature[j
                                        as
                                        usize][k
                                        as
                                        usize]
                                        == 0.0f64 {
                                        printf(b" 0,\x00" as *const u8 as
                                            *const libc::c_char);
                                    } else {
                                        if ellipsis_result_ctm[i as
                                            usize].omit_feature[j
                                            as
                                            usize][k
                                            as
                                            usize]
                                            == 1.0f64 {
                                            printf(b" 1,\x00" as *const u8 as
                                                *const libc::c_char);
                                        } else {
                                            printf(b" %f,\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   ellipsis_result_ctm[i as
                                                       usize].omit_feature[j
                                                       as
                                                       usize][k
                                                       as
                                                       usize]);
                                        };
                                    };
                                };
                                k += 1
                            }
                            j += 1
                        }
                        printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    } else {
                        printf(b";;<%s>%d FEATURE: %d,\x00" as *const u8 as
                                   *const libc::c_char,
                               cf_aresult.as_mut_ptr(), i,
                               if relax_compare_result(aresult.as_mut_ptr(),
                                                       gresult.as_mut_ptr())
                                   != 0 {
                                   1 as libc::c_int
                               } else { 0 as libc::c_int });
                        printf(b"%d:%f %d:%f \x00" as *const u8 as
                                   *const libc::c_char, 1 as libc::c_int,
                               ellipsis_result_ctm[i as
                                   usize].overt_arguments_score,
                               2 as libc::c_int,
                               ellipsis_result_ctm[i as
                                   usize].all_arguments_score);
                        j = 0 as libc::c_int;
                        while j < 4 as libc::c_int {
                            k = 0 as libc::c_int;
                            while k <
                                2 as libc::c_int *
                                    ((4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int) *
                                        (3 as libc::c_int +
                                            5 as libc::c_int)) {
                                if ellipsis_result_ctm[i as
                                    usize].omit_feature[j
                                    as
                                    usize][k
                                    as
                                    usize]
                                    == 0.0f64 ||
                                    ellipsis_result_ctm[i as
                                        usize].omit_feature[j
                                        as
                                        usize][k
                                        as
                                        usize]
                                        ==
                                        -(10000 as libc::c_int) as
                                            libc::c_double {
                                    if j ==
                                        4 as libc::c_int - 1 as libc::c_int
                                        &&
                                        k ==
                                            2 as libc::c_int *
                                                ((4 as libc::c_int +
                                                    (33 as libc::c_int +
                                                        5 as
                                                            libc::c_int)
                                                    + 3 as libc::c_int +
                                                    2 as libc::c_int +
                                                    8 as libc::c_int +
                                                    135 as libc::c_int *
                                                        3 as libc::c_int
                                                    +
                                                    15 as libc::c_int *
                                                        3 as libc::c_int
                                                    +
                                                    5 as libc::c_int *
                                                        4 as libc::c_int
                                                    + 5 as libc::c_int +
                                                    11 as libc::c_int *
                                                        2 as libc::c_int
                                                    +
                                                    3 as libc::c_int *
                                                        2 as libc::c_int
                                                    + 4 as libc::c_int +
                                                    10 as libc::c_int) *
                                                    (3 as libc::c_int +
                                                        5 as
                                                            libc::c_int))
                                                - 1 as libc::c_int {
                                        printf(b" %d:%d \x00" as *const u8 as
                                                   *const libc::c_char,
                                               j *
                                                   (2 as libc::c_int *
                                                       ((4 as libc::c_int +
                                                           (33 as
                                                               libc::c_int
                                                               +
                                                               5 as
                                                                   libc::c_int)
                                                           +
                                                           3 as libc::c_int
                                                           +
                                                           2 as libc::c_int
                                                           +
                                                           8 as libc::c_int
                                                           +
                                                           135 as
                                                               libc::c_int
                                                               *
                                                               3 as
                                                                   libc::c_int
                                                           +
                                                           15 as
                                                               libc::c_int
                                                               *
                                                               3 as
                                                                   libc::c_int
                                                           +
                                                           5 as libc::c_int
                                                               *
                                                               4 as
                                                                   libc::c_int
                                                           +
                                                           5 as libc::c_int
                                                           +
                                                           11 as
                                                               libc::c_int
                                                               *
                                                               2 as
                                                                   libc::c_int
                                                           +
                                                           3 as libc::c_int
                                                               *
                                                               2 as
                                                                   libc::c_int
                                                           +
                                                           4 as libc::c_int
                                                           +
                                                           10 as
                                                               libc::c_int)
                                                           *
                                                           (3 as libc::c_int
                                                               +
                                                               5 as
                                                                   libc::c_int)))
                                                   + k + 3 as libc::c_int,
                                               0 as libc::c_int);
                                    }
                                } else {
                                    printf(b" %d:%f \x00" as *const u8 as
                                               *const libc::c_char,
                                           j *
                                               (2 as libc::c_int *
                                                   ((4 as libc::c_int +
                                                       (33 as libc::c_int +
                                                           5 as
                                                               libc::c_int)
                                                       + 3 as libc::c_int +
                                                       2 as libc::c_int +
                                                       8 as libc::c_int +
                                                       135 as libc::c_int *
                                                           3 as libc::c_int
                                                       +
                                                       15 as libc::c_int *
                                                           3 as libc::c_int
                                                       +
                                                       5 as libc::c_int *
                                                           4 as libc::c_int
                                                       + 5 as libc::c_int +
                                                       11 as libc::c_int *
                                                           2 as libc::c_int
                                                       +
                                                       3 as libc::c_int *
                                                           2 as libc::c_int
                                                       + 4 as libc::c_int +
                                                       10 as libc::c_int) *
                                                       (3 as libc::c_int +
                                                           5 as
                                                               libc::c_int)))
                                               + k + 3 as libc::c_int,
                                           ellipsis_result_ctm[i as
                                               usize].omit_feature[j
                                               as
                                               usize][k
                                               as
                                               usize]);
                                }
                                k += 1
                            }
                            j += 1
                        }
                        printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    if relax_compare_result(aresult.as_mut_ptr(),
                                            gresult.as_mut_ptr()) != 0 {
                        rnum_check_flag = 0 as libc::c_int
                    }
                }
                i += 1
            }
            if svm_feaature_opt != 1 as libc::c_int {
                printf(b";;<dummy %s> FEATURE: -1,\x00" as *const u8 as
                           *const libc::c_char, gresult.as_mut_ptr());
                j = 0 as libc::c_int;
                while j <
                    4 as libc::c_int *
                        (2 as libc::c_int *
                            ((4 as libc::c_int +
                                (33 as libc::c_int +
                                    5 as libc::c_int) +
                                3 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int
                                +
                                15 as libc::c_int * 3 as libc::c_int
                                + 5 as libc::c_int * 4 as libc::c_int
                                + 5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int
                                + 3 as libc::c_int * 2 as libc::c_int
                                + 4 as libc::c_int +
                                10 as libc::c_int) *
                                (3 as libc::c_int +
                                    5 as libc::c_int))) +
                        2 as libc::c_int {
                    printf(b" 0,\x00" as *const u8 as *const libc::c_char);
                    j += 1
                }
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b";;<dummy %s> FEATURE: -1,\x00" as *const u8 as
                           *const libc::c_char, gresult.as_mut_ptr());
                printf(b" %d:%d\x00" as *const u8 as *const libc::c_char,
                       4 as libc::c_int *
                           (2 as libc::c_int *
                               ((4 as libc::c_int +
                                   (33 as libc::c_int + 5 as libc::c_int) +
                                   3 as libc::c_int + 2 as libc::c_int +
                                   8 as libc::c_int +
                                   135 as libc::c_int * 3 as libc::c_int +
                                   15 as libc::c_int * 3 as libc::c_int +
                                   5 as libc::c_int * 4 as libc::c_int +
                                   5 as libc::c_int +
                                   11 as libc::c_int * 2 as libc::c_int +
                                   3 as libc::c_int * 2 as libc::c_int +
                                   4 as libc::c_int + 10 as libc::c_int) *
                                   (3 as libc::c_int + 5 as libc::c_int))) +
                           2 as libc::c_int, 0 as libc::c_int);
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    if OptDisplay == 3 as libc::c_int || OptExpress == 16 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int {
            if ellipsis_result_ctm[i as usize].score ==
                -(10000 as libc::c_int) as libc::c_double {
                break;
            }
            if !(ellipsis_result_ctm[i as usize].score ==
                0 as libc::c_int as libc::c_double) {
                make_aresult_string(&mut *ellipsis_result_ctm.as_mut_ptr().offset(i
                    as
                    isize),
                                    aresult.as_mut_ptr());
                printf(b";;\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe5\x80\x99\xe8\xa3\x9c%d-%d:%2d %.3f %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num,
                       (*tag_ptr).num, i + 1 as libc::c_int,
                       ellipsis_result_ctm[i as usize].score,
                       (*ellipsis_result_ctm[i as
                           usize].cf_ptr).cf_id.as_mut_ptr());
                j = 0 as libc::c_int;
                while j < ellipsis_result_ctm[i as usize].result_num {
                    printf(b" %s%s:%s%d\x00" as *const u8 as
                               *const libc::c_char,
                           if j <
                               ellipsis_result_ctm[i as
                                   usize].case_result_num
                           {
                               b"\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"*\x00" as *const u8 as *const libc::c_char
                           },
                           pp_code_to_kstr((*ellipsis_result_ctm[i as
                               usize].cf_ptr).pp[ellipsis_result_ctm[i
                               as
                               usize].cf_element_num[j
                               as
                               usize]
                               as
                               usize][0
                               as
                               libc::c_int
                               as
                               usize]),
                           (*entity_manager.entity.as_mut_ptr().offset(ellipsis_result_ctm[i
                               as
                               usize].entity_num[j
                               as
                               usize]
                               as
                               isize)).name.as_mut_ptr(),
                           (*entity_manager.entity.as_mut_ptr().offset(ellipsis_result_ctm[i
                               as
                               usize].entity_num[j
                               as
                               usize]
                               as
                               isize)).num);
                    j += 1
                }
                j = 0 as libc::c_int;
                while j <
                    (*ellipsis_result_ctm[i as
                        usize].cf_ptr).element_num
                {
                    if ellipsis_result_ctm[i as
                        usize].filled_element[j as
                        usize]
                        == 0 &&
                        match_ellipsis_case(pp_code_to_kstr((*ellipsis_result_ctm[i
                            as
                            usize].cf_ptr).pp[j
                            as
                            usize][0
                            as
                            libc::c_int
                            as
                            usize]),
                                            0 as *mut *mut libc::c_char) !=
                            0 {
                        printf(b" %s:%s\x00" as *const u8 as
                                   *const libc::c_char,
                               pp_code_to_kstr((*ellipsis_result_ctm[i as
                                   usize].cf_ptr).pp[j
                                   as
                                   usize][0
                                   as
                                   libc::c_int
                                   as
                                   usize]),
                               if (*ellipsis_result_ctm[i as
                                   usize].cf_ptr).oblig[j
                                   as
                                   usize]
                                   != 0 {
                                   b"\xc3\x97\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"-\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    }
                    j += 1
                }
                printf(b" (0:%.2f*%.2f\x00" as *const u8 as
                           *const libc::c_char,
                       ellipsis_result_ctm[i as usize].overt_arguments_score,
                       overt_arguments_weight);
                printf(b" 1:%.2f\x00" as *const u8 as *const libc::c_char,
                       ellipsis_result_ctm[i as usize].all_arguments_score);
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    printf(b"|%s\x00" as *const u8 as *const libc::c_char,
                           ELLIPSIS_CASE_LIST_VERB[j as usize]);
                    k = 0 as libc::c_int;
                    while k <
                        2 as libc::c_int *
                            ((4 as libc::c_int +
                                (33 as libc::c_int + 5 as libc::c_int)
                                + 3 as libc::c_int + 2 as libc::c_int
                                + 8 as libc::c_int +
                                135 as libc::c_int * 3 as libc::c_int
                                + 15 as libc::c_int * 3 as libc::c_int
                                + 5 as libc::c_int * 4 as libc::c_int
                                + 5 as libc::c_int +
                                11 as libc::c_int * 2 as libc::c_int +
                                3 as libc::c_int * 2 as libc::c_int +
                                4 as libc::c_int + 10 as libc::c_int)
                                *
                                (3 as libc::c_int + 5 as libc::c_int))
                    {
                        if ellipsis_result_ctm[i as
                            usize].omit_feature[j as
                            usize][k
                            as
                            usize]
                            != -(10000 as libc::c_int) as libc::c_double &&
                            ellipsis_result_ctm[i as
                                usize].omit_feature[j
                                as
                                usize][k
                                as
                                usize]
                                != 0 as libc::c_int as libc::c_double {
                            let mut feature_name_0: [libc::c_char; 5120] =
                                *::std::mem::transmute::<&[u8; 5120],
                                    &mut [libc::c_char; 5120]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                == 15 as libc::c_int {
                                strcpy(feature_name_0.as_mut_ptr(),
                                       b"A\x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                == 16 as libc::c_int {
                                strcpy(feature_name_0.as_mut_ptr(),
                                       b"R\x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int {
                                sprintf(feature_name_0.as_mut_ptr(),
                                        b"L%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int)) /
                                            3 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int {
                                sprintf(feature_name_0.as_mut_ptr(),
                                        b"M%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int)) /
                                            2 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int * 2 as libc::c_int
                            {
                                sprintf(feature_name_0.as_mut_ptr(),
                                        b"H%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        (k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int)) /
                                            2 as libc::c_int);
                            }
                            if k %
                                (4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int
                                    + 8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int
                                    + 15 as libc::c_int * 3 as libc::c_int
                                    + 5 as libc::c_int * 4 as libc::c_int
                                    + 5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int + 10 as libc::c_int)
                                >
                                4 as libc::c_int +
                                    (33 as libc::c_int + 5 as libc::c_int)
                                    + 3 as libc::c_int + 2 as libc::c_int +
                                    8 as libc::c_int +
                                    135 as libc::c_int * 3 as libc::c_int +
                                    15 as libc::c_int * 3 as libc::c_int +
                                    5 as libc::c_int * 4 as libc::c_int +
                                    5 as libc::c_int +
                                    11 as libc::c_int * 2 as libc::c_int +
                                    3 as libc::c_int * 2 as libc::c_int +
                                    4 as libc::c_int &&
                                (k %
                                    (4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int +
                                        2 as libc::c_int +
                                        8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int *
                                            4 as libc::c_int +
                                        5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int *
                                            2 as libc::c_int +
                                        4 as libc::c_int +
                                        10 as libc::c_int)) <
                                    4 as libc::c_int +
                                        (33 as libc::c_int +
                                            5 as libc::c_int) +
                                        3 as libc::c_int + 2 as libc::c_int
                                        + 8 as libc::c_int +
                                        135 as libc::c_int *
                                            3 as libc::c_int +
                                        15 as libc::c_int *
                                            3 as libc::c_int +
                                        5 as libc::c_int * 4 as libc::c_int
                                        + 5 as libc::c_int +
                                        11 as libc::c_int *
                                            2 as libc::c_int +
                                        3 as libc::c_int * 2 as libc::c_int
                                        + 4 as libc::c_int +
                                        10 as libc::c_int {
                                sprintf(feature_name_0.as_mut_ptr(),
                                        b"C%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        k %
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int +
                                                10 as libc::c_int) -
                                            (4 as libc::c_int +
                                                (33 as libc::c_int +
                                                    5 as libc::c_int) +
                                                3 as libc::c_int +
                                                2 as libc::c_int +
                                                8 as libc::c_int +
                                                135 as libc::c_int *
                                                    3 as libc::c_int +
                                                15 as libc::c_int *
                                                    3 as libc::c_int +
                                                5 as libc::c_int *
                                                    4 as libc::c_int +
                                                5 as libc::c_int +
                                                11 as libc::c_int *
                                                    2 as libc::c_int +
                                                3 as libc::c_int *
                                                    2 as libc::c_int +
                                                4 as libc::c_int));
                            }
                            printf(b",%s:%d:%d:%.2f*%.2f\x00" as *const u8 as
                                       *const libc::c_char,
                                   feature_name_0.as_mut_ptr(), k,
                                   k %
                                       (4 as libc::c_int +
                                           (33 as libc::c_int +
                                               5 as libc::c_int) +
                                           3 as libc::c_int +
                                           2 as libc::c_int +
                                           8 as libc::c_int +
                                           135 as libc::c_int *
                                               3 as libc::c_int +
                                           15 as libc::c_int *
                                               3 as libc::c_int +
                                           5 as libc::c_int *
                                               4 as libc::c_int +
                                           5 as libc::c_int +
                                           11 as libc::c_int *
                                               2 as libc::c_int +
                                           3 as libc::c_int *
                                               2 as libc::c_int +
                                           4 as libc::c_int +
                                           10 as libc::c_int),
                                   ellipsis_result_ctm[i as
                                       usize].omit_feature[j
                                       as
                                       usize][k
                                       as
                                       usize],
                                   case_feature_weight[j as
                                       usize][k as
                                       usize]);
                        }
                        k += 1
                    }
                    j += 1
                }
                printf(b")\x00" as *const u8 as *const libc::c_char);
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            i += 1
        }
    }
    if ellipsis_result_ctm[0 as libc::c_int as usize].score ==
        -(10000 as libc::c_int) as libc::c_double {
        return 0 as libc::c_int;
    }
    if gs_ctm_ptr.is_null() {
        copy_ctm(&mut *ellipsis_result_ctm.as_mut_ptr().offset(0 as
            libc::c_int
            as isize),
                 (*tag_ptr).ctm_ptr);
        strcpy((*tag_ptr).mention_mgr.cf_id.as_mut_ptr(),
               (*ellipsis_result_ctm[0 as libc::c_int as usize].cf_ptr).cf_id.as_mut_ptr());
        (*tag_ptr).mention_mgr.cf_ptr =
            ellipsis_result_ctm[0 as libc::c_int as usize].cf_ptr
    } else {
        copy_ctm(gs_ctm_ptr, (*tag_ptr).ctm_ptr);
        strcpy((*tag_ptr).mention_mgr.cf_id.as_mut_ptr(),
               (*(*gs_ctm_ptr).cf_ptr).cf_id.as_mut_ptr());
        (*tag_ptr).mention_mgr.cf_ptr = (*gs_ctm_ptr).cf_ptr
    }
    free(cf_array as *mut libc::c_void);
    return (0 as libc::c_int == 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn make_each_unnamed_entity(mut name: *mut libc::c_char,
                                                  mut num: libc::c_int)
                                                  -> *mut ENTITY {
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut temp: [libc::c_char; 128] = [0; 128];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(temp.as_mut_ptr(), name);
    cp = temp.as_mut_ptr();
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp as libc::c_int == ':' as i32 {
            *cp = '-' as i32 as libc::c_char
        }
        cp = cp.offset(1)
    }
    entity_ptr =
        entity_manager.entity.as_mut_ptr().offset(entity_manager.num as
            isize);
    (*entity_ptr).output_num = entity_manager.num;
    (*entity_ptr).num = (*entity_ptr).output_num;
    entity_manager.num += 1;
    (*entity_ptr).hypothetical_flag = 1 as libc::c_int;
    (*entity_ptr).hypothetical_entity = num;
    (*entity_ptr).real_entity = -(1 as libc::c_int);
    (*entity_ptr).link_entity = -(1 as libc::c_int);
    (*entity_ptr).mentioned_num = 0 as libc::c_int;
    (*entity_ptr).first_appearance = 0 as libc::c_int;
    strcpy((*entity_ptr).named_entity.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    strcpy((*entity_ptr).name.as_mut_ptr(), temp.as_mut_ptr());
    strcpy((*entity_ptr).hypothetical_name.as_mut_ptr(), name);
    if OptAnaphora & 32 as libc::c_int != 0 {
        (*entity_ptr).skip_flag = 0 as libc::c_int
    } else { (*entity_ptr).skip_flag = 1 as libc::c_int }
    (*entity_ptr).salience_score = 1 as libc::c_int as libc::c_double;
    (*entity_ptr).corefer_id = -(1 as libc::c_int);
    (*entity_ptr).rep_tag_num = -(1 as libc::c_int);
    (*entity_ptr).rep_sen_num = -(1 as libc::c_int);
    return entity_ptr;
}


pub unsafe extern "C" fn  make_new_entity(mut tag_ptr: *mut TAG_DATA, mut mention_mgr: *mut MENTION_MGR){
    let mut cp: *mut libc::c_char = "" as *mut libc::c_char;
    let temp: [libc::c_char; DATA_LEN as usize] = [];
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;

    entity_ptr = entity_manager.entity + entity_manager.num;
    entity_ptr.output_num = entity_manager.num;
    entity_ptr.num = entity_ptr.output_num;
    entity_manager.num += 1;
    entity_ptr.mention[0] = mention_mgr.mention as *mut mention;
    entity_ptr.mentioned_num = 1;
    entity_ptr.hypothetical_flag=0;
    entity_ptr.real_entity = -1;
    entity_ptr.hypothetical_entity=-1;
    strcpy(entity_ptr.hypothetical_name as *mut libc::c_char, "" as *const libc::c_char);
    entity_ptr.skip_flag = 0;
    entity_ptr.link_entity = -1;
    entity_ptr.corefer_id = -1;
    entity_ptr.first_appearance = mention_mgr.mention.sent_num;
    entity_ptr.rep_sen_num = mention_mgr.mention.sent_num;
    entity_ptr.rep_tag_num = tag_ptr.num;

    /* 先行詞になりやすさ(基本的に文節主辞なら1) */
    entity_ptr.salience_score = if tag_ptr.inum > 0 ||
        !check_feature(tag_ptr.f, "照応詞候補" as *mut libc::c_char) as bool ||
        check_feature(tag_ptr.f, "NE内" as *mut libc::c_char) as bool {
        0 as libc::c_double
    } else {
        if (check_feature(tag_ptr.f, "ハ" as *mut libc::c_char) ||
            check_feature(tag_ptr.f, "モ" as *mut libc::c_char)) &&
            !check_feature(tag_ptr.f, "括弧終" as *mut libc::c_char) as bool ||
            check_feature(tag_ptr.f, "文末" as *mut libc::c_char) as bool {
            SALIENCE_THEMA as libc::c_double
        } else {
            if check_feature(tag_ptr.f, "読点" as *mut libc::c_char)
                && (tag_ptr.para_type != PARA_NORMAL as libc::c_char) as *mut libc::c_char
                || check_feature(tag_ptr.b_ptr.f, "文頭" as *mut libc::c_char) as bool
                || check_feature(tag_ptr.f, "係:ガ格" as *mut libc::c_char) as bool
                || check_feature(tag_ptr.f, "係:ヲ格" as *mut libc::c_char) as bool {
                SALIENCE_CANDIDATE as libc::c_double
            } else {
                SALIENCE_NORMAL as libc::c_double
            }
        }
    };
    if check_feature(tag_ptr.f, "係:ニ格" as *mut libc::c_char) || check_feature(tag_ptr.f, "係:ノ格" as *mut libc::c_char) {
        entity_ptr.tmp_salience_flag = 1;
    }
    entity_ptr.mention[0].static_salience_score = entity_ptr.salience_score;
    /* ENTITYの名前 */
    if cp == check_feature(tag_ptr.f, "NE" as *mut libc::c_char) {
        strcpy(entity_ptr.named_entity as *mut libc::c_char, cp + strlen("NE:" as *const libc::c_char));
        strcpy(temp as *mut libc::c_char,cp);
        abbreviate_NE(temp as *mut libc::c_char);
        strcpy(entity_ptr.name as *mut libc::c_char, temp + strlen("NE:" as *const libc::c_char));
    } else if cp == check_feature(tag_ptr.f, "照応詞候補" as *mut libc::c_char) {
        strcpy(entity_ptr.name as *mut libc::c_char, cp + strlen("照応詞候補:" as *const libc::c_char));
    } else {
        strcpy(entity_ptr.name as *mut libc::c_char, tag_ptr.head_ptr.Goi2 as *const libc::c_char);
    }

    mention_mgr.mention.entity = entity_ptr;
    mention_mgr.mention.explicit_mention = None;
    strcpy(mention_mgr.mention.cpp_string, "＊" as *const libc::c_char);
    if cp == check_feature(tag_ptr.f, "係" as *mut libc::c_char) {
        strcpy(mention_mgr.mention.spp_string, cp + strlen("係:" as *const libc::c_char));
    } else {
        strcpy(mention_mgr.mention.spp_string, "＊" as *const libc::c_char);
    }
    mention_mgr.mention.type_0 = 'S'; /* 自分自身 */
    if OptReadFeature & OPT_COREFER_AUTO {
        if cp == check_feature(tag_ptr.f, "COREFER_ID" as *mut libc::c_char) {
            sscanf(cp as *mut libc::c_char, "COREFER_ID:%d" as *const libc::c_char, &corefer_id);
            entity_ptr.corefer_id = corefer_id;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn make_unnamed_entity() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        make_each_unnamed_entity(unnamed_entity[i as usize], i);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_all_location_category(mut tag_ptr:
                                                     *mut TAG_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut diff_sen: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_char = 0;
    let mut rel: [libc::c_char; 128] = [0; 128];
    let mut loc_name: [libc::c_char; 128] = [0; 128];
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    i = 0 as libc::c_int;
    while i < entity_manager.num {
        mention_ptr =
            (*substance_tag_ptr(tag_ptr)).mention_mgr.mention.as_mut_ptr();
        entity_ptr = entity_manager.entity.as_mut_ptr().offset(i as isize);
        if !((*entity_ptr).salience_score ==
            0 as libc::c_int as libc::c_double) {
            /* 何文以内にmentionを持っているかどうかのチェック */
            diff_sen = 4 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*entity_ptr).mentioned_num {
                if !((*mention_ptr).sent_num ==
                    (*(*entity_ptr).mention[j as usize]).sent_num &&
                    loc_category[(*(*(*(*entity_ptr).mention[j as
                        usize]).tag_ptr).b_ptr).num
                        as usize] == 0 as libc::c_int) {
                    if (*mention_ptr).sent_num -
                        (*(*entity_ptr).mention[j as usize]).sent_num <
                        diff_sen {
                        diff_sen =
                            (*mention_ptr).sent_num -
                                (*(*entity_ptr).mention[j as usize]).sent_num
                    }
                }
                j += 1
            }
            j = 0 as libc::c_int;
            while j < (*entity_ptr).mentioned_num {
                /* もっとも近くの文に出現したmentionのみ出力 */
                if !((*mention_ptr).sent_num -
                    (*(*entity_ptr).mention[j as usize]).sent_num >
                    diff_sen) {
                    if !((*(*entity_ptr).mention[j as usize]).sent_num ==
                        (*mention_ptr).sent_num &&
                        loc_category[(*(*(*(*entity_ptr).mention[j as
                            usize]).tag_ptr).b_ptr).num
                            as usize] == 0 as libc::c_int) {
                        if get_location(loc_name.as_mut_ptr(),
                                        (*mention_ptr).sent_num,
                                        if check_analyze_tag(tag_ptr,
                                                             0 as libc::c_int)
                                            == 1 as libc::c_int {
                                            b"\xe5\x8b\x95\x00" as *const u8
                                                as *const libc::c_char
                                        } else {
                                            b"\xe5\x90\x8d\x00" as *const u8
                                                as *const libc::c_char
                                        } as *mut libc::c_char,
                                        (*entity_ptr).mention[j as usize],
                                        0 as libc::c_int) != 0 {
                            printf(b";;LOCATION-ALL: %s\x00" as *const u8 as
                                       *const libc::c_char,
                                   loc_name.as_mut_ptr());
                            printf(b" %s \x00" as *const u8 as
                                       *const libc::c_char,
                                   (*entity_ptr).name.as_mut_ptr());
                            cp =
                                check_feature((*tag_ptr).f,
                                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !cp.is_null() {
                                cp =
                                    strchr(cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                                        as *const u8
                                        as
                                        *const libc::c_char)
                                        as isize),
                                           ':' as
                                               i32).offset(1 as libc::c_int as
                                        isize);
                                while *cp != 0 {
                                    if *cp as libc::c_int == ':' as i32 ||
                                        *cp as libc::c_int == ';' as i32 {
                                        if sscanf(cp.offset(1 as libc::c_int
                                            as isize),
                                                  b"%[^/]/%c/\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  rel.as_mut_ptr(),
                                                  &mut type_0 as
                                                      *mut libc::c_char) != 0
                                            &&
                                            match_ellipsis_case(rel.as_mut_ptr(),
                                                                0 as
                                                                    *mut *mut libc::c_char)
                                                != 0 &&
                                            (type_0 as libc::c_int ==
                                                'C' as i32 ||
                                                type_0 as libc::c_int ==
                                                    'N' as i32) {
                                            printf(b" -%s\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   rel.as_mut_ptr());
                                        }
                                    }
                                    cp = cp.offset(1)
                                }
                            }
                            printf(b"\n\x00" as *const u8 as
                                *const libc::c_char);
                        }
                    }
                }
                j += 1
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_context_structure(mut sp: *mut SENTENCE_DATA)
                                                -> libc::c_int
/*==================================================================*/
{
    /* 共参照解析結果を読み込み、省略解析を行い文の構造を構築する */
    let mut i: libc::c_int = 0; //文節を解析する順番を保持する
    let mut j: libc::c_int = 0;
    let mut check_result: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut analysis_bnst_cueue: [libc::c_int; 200] = [0; 200];
    let mut bnst_cueue_idx: libc::c_int = 0;
    /*一文毎の解析のオプションを切り分けた時に条件を付ける*/
    set_bnst_cueue(analysis_bnst_cueue.as_mut_ptr(), sp);
    /* 省略解析を行う場合 */
    bnst_cueue_idx = 0 as libc::c_int;
    while bnst_cueue_idx < 200 as libc::c_int {
        let mut bnst_idx: libc::c_int =
            analysis_bnst_cueue[bnst_cueue_idx as usize];
        let mut tag_num: libc::c_int = 0;
        if !(bnst_idx == -(1 as libc::c_int)) {
            let mut current_block_97: u64;
            tag_num =
                (*(*sp).bnst_data.offset(bnst_idx as isize)).tag_num -
                    1 as libc::c_int;
            while tag_num >= 0 as libc::c_int {
                i =
                    (*(*(*sp).bnst_data.offset(bnst_idx as
                        isize)).tag_ptr.offset(tag_num
                        as
                        isize)).num;
                tag_ptr =
                    substance_tag_ptr((*sp).tag_data.offset(i as isize));
                check_result = check_analyze_tag(tag_ptr, 0 as libc::c_int);
                if !(OptReadFeature & 256 as libc::c_int == 0 &&
                    check_result == 0) {
                    /* 解析対象格の設定 */
                    ELLIPSIS_CASE_LIST =
                        if check_result == 1 as libc::c_int {
                            ELLIPSIS_CASE_LIST_VERB.as_mut_ptr()
                        } else { ELLIPSIS_CASE_LIST_NOUN.as_mut_ptr() };
                    /* 省略のMENTIONの処理 */
                    /* 入力から正解を読み込む場合 */
                    if OptAnaphora & 16 as libc::c_int != 0 {
                        j = 0 as libc::c_int;
                        while j < entity_manager.num {
                            entity_manager.entity[j as usize].salience_mem =
                                0 as libc::c_int as libc::c_double;
                            j += 1
                        }
                    }
                    if OptReadFeature & 256 as libc::c_int != 0 ||
                        check_result == 1 as libc::c_int &&
                            OptReadFeature & 1 as libc::c_int != 0 ||
                        check_result == 1 as libc::c_int &&
                            OptReadFeature & 16 as libc::c_int != 0 ||
                        check_result == 2 as libc::c_int &&
                            OptReadFeature & 4 as libc::c_int != 0 {
                        /* この時点での各EntityのSALIENCE出力 */
                        if OptDisplay == 3 as libc::c_int ||
                            OptExpress == 16 as libc::c_int {
                            printf(b";;SALIENCE-%d-%d\x00" as *const u8 as
                                       *const libc::c_char, (*sp).Sen_num, i);
                            j = 0 as libc::c_int;
                            while j < entity_manager.num {
                                printf(b":%.3f\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*entity_manager.entity.as_mut_ptr().offset(j
                                           as
                                           isize)).salience_score);
                                if j == 0 as libc::c_int &&
                                    OptAnaphora & 512 as libc::c_int != 0
                                    && OptAnaphora & 32 as libc::c_int != 0
                                {
                                    printf(b";%.3f\x00" as *const u8 as
                                               *const libc::c_char,
                                           author_score);
                                }
                                if j == 1 as libc::c_int &&
                                    OptAnaphora & 1024 as libc::c_int != 0
                                    && OptAnaphora & 32 as libc::c_int != 0
                                {
                                    printf(b";%.3f\x00" as *const u8 as
                                               *const libc::c_char,
                                           reader_score);
                                }
                                j += 1
                            }
                            printf(b"\n\x00" as *const u8 as
                                *const libc::c_char);
                        }
                        if OptAnaphora & 131072 as libc::c_int == 0 ||
                            analysis_flag == 0 {
                            analysis_flags[(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num
                                as
                                usize][(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).tag_num
                                as usize] =
                                1 as libc::c_int;
                            /* featureから格解析結果を取得 */
                            cp =
                                check_feature((*tag_ptr).f,
                                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !cp.is_null() {
                                /* 共参照関係にある表現は格解析結果を取得しない */
                                if !check_feature((*tag_ptr).f,
                                                  b"\xe4\xbd\x93\xe8\xa8\x80\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char).is_null()
                                    &&
                                    (!strstr(cp,
                                             b"=/\x00" as *const u8 as
                                                 *const libc::c_char).is_null()
                                        ||
                                        !strstr(cp,
                                                b"=\xe6\xa7\x8b/\x00" as
                                                    *const u8 as
                                                    *const libc::c_char).is_null()
                                        ||
                                        !strstr(cp,
                                                b"=\xe5\xbd\xb9/\x00" as
                                                    *const u8 as
                                                    *const libc::c_char).is_null())
                                {
                                    assign_cfeature(&mut (*tag_ptr).f,
                                                    b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    0 as libc::c_int);
                                    current_block_97 = 11050875288958768710;
                                } else {
                                    cp =
                                        strchr(cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char)
                                            as isize),
                                               ':' as
                                                   i32).offset(1 as
                                            libc::c_int
                                            as isize);
                                    while *cp != 0 {
                                        if *cp as libc::c_int == ':' as i32 ||
                                            *cp as libc::c_int ==
                                                ';' as i32 {
                                            read_one_annotation(sp, tag_ptr,
                                                                cp.offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize),
                                                                0 as
                                                                    libc::c_int);
                                        }
                                        cp = cp.offset(1)
                                    }
                                    current_block_97 = 11793792312832361944;
                                }
                            } else {
                                current_block_97 = 11793792312832361944;
                            }
                        } else { current_block_97 = 11793792312832361944; }
                    } else { current_block_97 = 11793792312832361944; }
                    match current_block_97 {
                        11050875288958768710 => {}
                        _ => {
                            /* 省略解析を行う場合、または、素性を出力する場合 */
                            if check_result == 1 as libc::c_int &&
                                OptReadFeature & 1 as libc::c_int == 0 ||
                                check_result == 2 as libc::c_int &&
                                    OptReadFeature & 4 as libc::c_int == 0
                                ||
                                OptAnaphora & 16 as libc::c_int != 0 &&
                                    (OptAnaphora & 131072 as libc::c_int ==
                                        0 || analysis_flag != 0) {
                                if !(*tag_ptr).cf_ptr.is_null() {
                                    assign_cfeature(&mut (*tag_ptr).f,
                                                    b"\xef\xbc\xb4\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    0 as libc::c_int);
                                    /* cpm_ptrの作成(基本的にはtcf_ptrを使用するが、set_tag_case_frameの呼び出し、および、
					   get_ex_probability_with_para内でtcf_ptr->cf.pred_b_ptr->cpm_ptrとして使用している) */
                                    cpm_ptr =
                                        malloc_data(::std::mem::size_of::<CF_PRED_MGR>()
                                                        as libc::c_ulong,
                                                    b"make_context_structure: cpm_ptr\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) as
                                            *mut CF_PRED_MGR;
                                    init_case_frame(&mut (*cpm_ptr).cf);
                                    (*cpm_ptr).pred_b_ptr = tag_ptr;
                                    /* tag_ptr->tcf_ptrを作成 */
                                    (*tag_ptr).tcf_ptr =
                                        malloc_data(::std::mem::size_of::<TAG_CASE_FRAME>()
                                                        as libc::c_ulong,
                                                    b"make_context_structure: tcf_ptr\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) as
                                            *mut TAG_CASE_FRAME;
                                    set_tag_case_frame(sp, tag_ptr, cpm_ptr);
                                    /* 位置カテゴリの生成 */
                                    mark_loc_category(sp, tag_ptr);
                                    if OptAnaphora & 16 as libc::c_int != 0 {
                                        /* 存在するすべての位置カテゴリを出力 */
                                        print_all_location_category(tag_ptr);
                                    }
                                    /* この時点での各EntityのSALIENCE出力 */
                                    if OptDisplay == 3 as libc::c_int ||
                                        OptExpress == 16 as libc::c_int {
                                        printf(b";;SALIENCE-%d-%d\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               (*sp).Sen_num, i);
                                        j = 0 as libc::c_int;
                                        while j < entity_manager.num {
                                            printf(b":%.3f\x00" as *const u8
                                                       as *const libc::c_char,
                                                   (*entity_manager.entity.as_mut_ptr().offset(j
                                                       as
                                                       isize)).salience_score);
                                            if j == 0 as libc::c_int &&
                                                OptAnaphora &
                                                    512 as libc::c_int != 0
                                                &&
                                                OptAnaphora &
                                                    32 as libc::c_int != 0
                                            {
                                                printf(b";%.3f\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       author_score);
                                            }
                                            if j == 1 as libc::c_int &&
                                                OptAnaphora &
                                                    1024 as libc::c_int !=
                                                    0 &&
                                                OptAnaphora &
                                                    32 as libc::c_int != 0
                                            {
                                                printf(b";%.3f\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       reader_score);
                                            }
                                            j += 1
                                        }
                                        printf(b"\n\x00" as *const u8 as
                                            *const libc::c_char);
                                    }
                                    /* 省略解析メイン */
                                    (*tag_ptr).ctm_ptr =
                                        malloc_data(::std::mem::size_of::<CF_TAG_MGR>()
                                                        as libc::c_ulong,
                                                    b"make_context_structure: ctm_ptr\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) as
                                            *mut CF_TAG_MGR;
                                    (*(*tag_ptr).ctm_ptr).score =
                                        -(10000 as libc::c_int) as
                                            libc::c_double;
                                    ellipsis_analysis_main(tag_ptr);
                                    if OptAnaphora & 16 as libc::c_int == 0 &&
                                        (*(*tag_ptr).ctm_ptr).score !=
                                            -(10000 as libc::c_int) as
                                                libc::c_double {
                                        let mut ellipsis_score:
                                            libc::c_double =
                                            (*(*tag_ptr).ctm_ptr).score;
                                        if OptReadFeature & 64 as libc::c_int
                                            == 0 {
                                            expand_result_to_parallel_entity(tag_ptr);
                                            /* 並列要素を展開する */
                                        } /* 解析結果をENTITYと関連付ける */
                                        (*tag_ptr).score_diff =
                                            100 as libc::c_int as
                                                libc::c_double;
                                        j = 1 as libc::c_int;
                                        while j < 10 as libc::c_int {
                                            if ellipsis_result_ctm[j as
                                                usize].score
                                                ==
                                                -(10000 as libc::c_int) as
                                                    libc::c_double {
                                                break;
                                            }
                                            if (*(*tag_ptr).ctm_ptr).ga_entity
                                                !=
                                                ellipsis_result_ctm[j as
                                                    usize].ga_entity
                                            {
                                                (*tag_ptr).ga_score_diff =
                                                    ellipsis_score -
                                                        ellipsis_result_ctm[j
                                                            as
                                                            usize].score
                                            }
                                            j += 1
                                        }
                                        (*tag_ptr).score_diff =
                                            ellipsis_result_ctm[0 as
                                                libc::c_int
                                                as
                                                usize].score
                                                -
                                                ellipsis_result_ctm[1 as
                                                    libc::c_int
                                                    as
                                                    usize].score;
                                        if OptAnaphora & 131072 as libc::c_int
                                            == 0 {
                                            anaphora_result_to_entity(tag_ptr);
                                            analysis_flags[(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num
                                                as
                                                usize][(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).tag_num
                                                as
                                                usize]
                                                = 1 as libc::c_int
                                        } else if (*tag_ptr).score_diff >
                                            max_reliabirity &&
                                            check_feature((*tag_ptr).f,
                                                          b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\xb8\x88\xe3\x81\xbf\x00"
                                                              as
                                                              *const u8
                                                              as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char).is_null()
                                        {
                                            if !max_reliabirity_tag_ptr.is_null()
                                            {
                                                free((*max_reliabirity_tag_ptr).ctm_ptr
                                                    as
                                                    *mut libc::c_void);
                                                free((*max_reliabirity_tag_ptr).tcf_ptr
                                                    as
                                                    *mut libc::c_void);
                                            }
                                            max_reliabirity =
                                                (*tag_ptr).score_diff;
                                            max_reliabirity_tag_ptr = tag_ptr
                                        } else {
                                            free((*tag_ptr).ctm_ptr as
                                                *mut libc::c_void);
                                            free((*tag_ptr).tcf_ptr as
                                                *mut libc::c_void);
                                        }
                                    }
                                    /* メモリを解放 */
                                    clear_case_frame(&mut (*cpm_ptr).cf);
                                    free((*tag_ptr).cpm_ptr as
                                        *mut libc::c_void);
                                }
                            }
                        }
                    }
                }
                tag_num -= 1
            }
        }
        bnst_cueue_idx += 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_entities(mut sen_idx: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut entity_ptr: *mut ENTITY = 0 as *mut ENTITY;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut m: MRPH_DATA =
        MRPH_DATA {
            type_0: 0,
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
            Type: [0; 9],
        };
    printf(b";;\n;;SENTENCE %d\n\x00" as *const u8 as *const libc::c_char,
           sen_idx + base_sentence_num);
    let mut current_block_92: u64;
    i = 0 as libc::c_int;
    while i < entity_manager.num {
        entity_ptr = entity_manager.entity.as_mut_ptr().offset(i as isize);
        if OptAnaphora & 32 as libc::c_int == 0 &&
            (OptAnaphora & 64 as libc::c_int != 0 ||
                OptAnaphora & 16 as libc::c_int != 0) {
            if i < 5 as libc::c_int {
                current_block_92 = 7095457783677275021;
            } else { current_block_92 = 13109137661213826276; }
        } else { current_block_92 = 13109137661213826276; }
        match current_block_92 {
            13109137661213826276 => {
                if OptZeroPronoun == 1 as libc::c_int {
                    /*--------------------------------------------------------------*/
                    //entity 全てのEntity_numを1にすることで、Zero-pronounの精度を測る
                    printf(b";; ENTITY %d [ %s ] %f {\n\x00" as *const u8 as
                               *const libc::c_char, 1 as libc::c_int,
                           (*entity_ptr).name.as_mut_ptr(),
                           (*entity_ptr).salience_score);
                    /*--------------------------------------------------------------*/
                } else if OptAnaphora & 32 as libc::c_int == 0 &&
                    (OptAnaphora & 64 as libc::c_int != 0 ||
                        OptAnaphora & 16 as libc::c_int != 0) {
                    printf(b";; ENTITY %d [ %s ] %f {\n\x00" as *const u8 as
                               *const libc::c_char,
                           (*entity_ptr).output_num + base_entity_num -
                               5 as libc::c_int,
                           (*entity_ptr).name.as_mut_ptr(),
                           (*entity_ptr).salience_score);
                } else {
                    printf(b";; ENTITY %d [ %s ] %f {\n\x00" as *const u8 as
                               *const libc::c_char,
                           (*entity_ptr).output_num + base_entity_num,
                           (*entity_ptr).name.as_mut_ptr(),
                           (*entity_ptr).salience_score);
                }
                j = 0 as libc::c_int;
                while j < (*entity_ptr).mentioned_num {
                    mention_ptr = (*entity_ptr).mention[j as usize];
                    if !(OptAnaphora & 32 as libc::c_int == 0 &&
                        (*mention_ptr).type_0 as libc::c_int ==
                            'E' as i32) {
                        printf(b";;\tMENTION%3d {\x00" as *const u8 as
                                   *const libc::c_char, j);
                        printf(b" SEN:%3d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).sent_num + base_sentence_num);
                        printf(b" TAG:%3d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).tag_num);
                        printf(b" (%3d)\x00" as *const u8 as
                                   *const libc::c_char,
                               (*(*(*mention_ptr).tag_ptr).head_ptr).Num);
                        printf(b" CPP: %4s\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).cpp_string.as_mut_ptr());
                        printf(b" SPP: %4s\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).spp_string.as_mut_ptr());
                        printf(b" TYPE: %c\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).type_0 as libc::c_int);
                        printf(b" SS: %.3f\x00" as *const u8 as
                                   *const libc::c_char,
                               (*mention_ptr).salience_score);
                        printf(b" WORD: %s\x00" as *const u8 as
                                   *const libc::c_char,
                               (*(*(*mention_ptr).tag_ptr).head_ptr).Goi2.as_mut_ptr());
                        /* 格フレームのカバレッジを調べる際に必要となる情報 */
                        if OptDisplay == 2 as libc::c_int {
                            /* 用言の場合 */
                            if !check_feature((*(*mention_ptr).tag_ptr).f,
                                              b"\xe7\x94\xa8\xe8\xa8\x80\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                                &&
                                ((*mention_ptr).type_0 as libc::c_int ==
                                    'E' as i32 ||
                                    (*mention_ptr).type_0 as libc::c_int
                                        == 'C' as i32 ||
                                    (*mention_ptr).type_0 as libc::c_int
                                        == 'N' as i32 ||
                                    (*mention_ptr).type_0 as libc::c_int
                                        == 'O' as i32) {
                                printf(b" POS: %s\x00" as *const u8 as
                                           *const libc::c_char,
                                       check_feature((*(*mention_ptr).tag_ptr).f,
                                                     b"\xe7\x94\xa8\xe8\xa8\x80\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char).offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00"
                                           as
                                           *const u8
                                           as
                                           *const libc::c_char)
                                           as
                                           isize));
                                if OptCaseFlag & 131072 as libc::c_int != 0 {
                                    cp =
                                        make_pred_string_from_mrph((*mention_ptr).tag_ptr,
                                                                   0 as
                                                                       *mut MRPH_DATA,
                                                                   0 as
                                                                       *mut libc::c_char,
                                                                   OptCaseFlag
                                                                       &
                                                                       32 as
                                                                           libc::c_int,
                                                                   1 as
                                                                       libc::c_int,
                                                                   0 as
                                                                       libc::c_int)
                                } else {
                                    cp =
                                        make_pred_string((*mention_ptr).tag_ptr,
                                                         0 as *mut MRPH_DATA,
                                                         0 as
                                                             *mut libc::c_char,
                                                         OptCaseFlag &
                                                             32 as
                                                                 libc::c_int,
                                                         1 as libc::c_int,
                                                         0 as libc::c_int)
                                }
                                printf(b" KEY: %s\x00" as *const u8 as
                                           *const libc::c_char, cp);
                                /* 代表表記が曖昧な用言の場合 */
                                if !check_feature((*(*(*mention_ptr).tag_ptr).head_ptr).f,
                                                  b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char).is_null()
                                {
                                    fp =
                                        (*(*(*mention_ptr).tag_ptr).head_ptr).f;
                                    while !fp.is_null() {
                                        if strncmp((*fp).cp,
                                                   b"ALT-\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   4 as libc::c_int as
                                                       libc::c_ulong) == 0 {
                                            sscanf((*fp).cp.offset(4 as
                                                libc::c_int
                                                as
                                                isize),
                                                   b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m.Goi2.as_mut_ptr(),
                                                   m.Yomi.as_mut_ptr(),
                                                   m.Goi.as_mut_ptr(),
                                                   &mut m.Hinshi as
                                                       *mut libc::c_int,
                                                   &mut m.Bunrui as
                                                       *mut libc::c_int,
                                                   &mut m.Katuyou_Kata as
                                                       *mut libc::c_int,
                                                   &mut m.Katuyou_Kei as
                                                       *mut libc::c_int,
                                                   m.Imi.as_mut_ptr());
                                            if OptCaseFlag &
                                                131072 as libc::c_int != 0
                                            {
                                                cp =
                                                    make_pred_string_from_mrph((*mention_ptr).tag_ptr,
                                                                               &mut m,
                                                                               0
                                                                                   as
                                                                                   *mut libc::c_char,
                                                                               OptCaseFlag
                                                                                   &
                                                                                   32
                                                                                       as
                                                                                       libc::c_int,
                                                                               1
                                                                                   as
                                                                                   libc::c_int,
                                                                               0
                                                                                   as
                                                                                   libc::c_int)
                                            } else {
                                                cp =
                                                    make_pred_string((*mention_ptr).tag_ptr,
                                                                     &mut m,
                                                                     0 as
                                                                         *mut libc::c_char,
                                                                     OptCaseFlag
                                                                         &
                                                                         32 as
                                                                             libc::c_int,
                                                                     1 as
                                                                         libc::c_int,
                                                                     0 as
                                                                         libc::c_int)
                                            }
                                            printf(b"-%s\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   cp);
                                        }
                                        fp = (*fp).next
                                    }
                                }
                                if (*(*mention_ptr).tag_ptr).voice &
                                    1 as libc::c_int != 0 ||
                                    !check_feature((*(*mention_ptr).tag_ptr).f,
                                                   b"\xe6\x85\x8b:\xe4\xbd\xbf\xe5\xbd\xb9\x00"
                                                       as *const u8 as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                {
                                    printf(b" VOICE: C\x00" as *const u8 as
                                        *const libc::c_char);
                                } else if (*(*mention_ptr).tag_ptr).voice &
                                    2 as libc::c_int != 0 ||
                                    !check_feature((*(*mention_ptr).tag_ptr).f,
                                                   b"\xe6\x85\x8b:\xe5\x8f\x97\xe5\x8b\x95\x00"
                                                       as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                {
                                    printf(b" VOICE: P\x00" as *const u8 as
                                        *const libc::c_char);
                                } else {
                                    printf(b" VOICE: N\x00" as *const u8 as
                                        *const libc::c_char);
                                }
                                /* 直接の格要素の基本句番号 */
                                if !(*mention_ptr).explicit_mention.is_null()
                                {
                                    printf(b" CTAG: %d\x00" as *const u8 as
                                               *const libc::c_char,
                                           (*(*mention_ptr).explicit_mention).tag_num);
                                }
                            } else if (*mention_ptr).type_0 as libc::c_int ==
                                'S' as i32 ||
                                (*mention_ptr).type_0 as libc::c_int
                                    == '=' as i32 {
                                if (*(*mention_ptr).tag_ptr).head_ptr ==
                                    (*(*(*mention_ptr).tag_ptr).b_ptr).head_ptr
                                {
                                    /* 格要素の場合 */
                                    /* 文節主辞であるかどうか */
                                    cp =
                                        get_bnst_head_canonical_rep((*(*mention_ptr).tag_ptr).b_ptr,
                                                                    OptCaseFlag
                                                                        &
                                                                        512 as
                                                                            libc::c_int)
                                } else {
                                    cp =
                                        check_feature((*(*mention_ptr).tag_ptr).f,
                                                      b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    if !cp.is_null() {
                                        cp =
                                            cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                                as *const u8
                                                as
                                                *const libc::c_char)
                                                as isize)
                                    }
                                }
                                printf(b" POS: %s\x00" as *const u8 as
                                           *const libc::c_char,
                                       Class[(*(*(*mention_ptr).tag_ptr).head_ptr).Hinshi
                                           as
                                           usize][(*(*(*mention_ptr).tag_ptr).head_ptr).Bunrui
                                           as usize].id);
                                printf(b" KEY: %s\x00" as *const u8 as
                                           *const libc::c_char, cp);
                                if !check_feature((*(*mention_ptr).tag_ptr).f,
                                                  b"\xe8\xa3\x9c\xe6\x96\x87\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char).is_null()
                                {
                                    printf(b" GE: \xe8\xa3\x9c\xe6\x96\x87\x00"
                                        as *const u8 as
                                        *const libc::c_char);
                                } else if !check_feature((*(*mention_ptr).tag_ptr).f,
                                                         b"\xe6\x99\x82\xe9\x96\x93\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char).is_null()
                                {
                                    printf(b" GE: \xe6\x99\x82\xe9\x96\x93\x00"
                                        as *const u8 as
                                        *const libc::c_char);
                                } else if !check_feature((*(*mention_ptr).tag_ptr).f,
                                                         b"\xe6\x95\xb0\xe9\x87\x8f\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char).is_null()
                                {
                                    printf(b" GE: \xe6\x95\xb0\xe9\x87\x8f\x00"
                                        as *const u8 as
                                        *const libc::c_char);
                                }
                                cp =
                                    check_feature((*(*(*mention_ptr).tag_ptr).head_ptr).f,
                                                  b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                if !cp.is_null() {
                                    printf(b" CT: %s\x00" as *const u8 as
                                               *const libc::c_char,
                                           cp.offset(strlen(b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa:\x00"
                                               as *const u8
                                               as
                                               *const libc::c_char)
                                               as isize));
                                }
                                cp =
                                    check_feature((*(*mention_ptr).tag_ptr).f,
                                                  b"NE\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                if !cp.is_null() {
                                    printf(b" NE: %s\x00" as *const u8 as
                                               *const libc::c_char,
                                           cp.offset(strlen(b"NE:\x00" as
                                               *const u8 as
                                               *const libc::c_char)
                                               as isize));
                                }
                            }
                        }
                        if OptAnaphora & 131072 as libc::c_int != 0 {
                            cp =
                                check_feature((*(*mention_ptr).tag_ptr).f,
                                              b"\xe8\xa7\xa3\xe6\x9e\x90\xe9\xa0\x86\xe5\xba\x8f\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !cp.is_null() {
                                printf(b" ITE: %s\x00" as *const u8 as
                                           *const libc::c_char,
                                       cp.offset(strlen(b"\xe8\xa7\xa3\xe6\x9e\x90\xe9\xa0\x86\xe5\xba\x8f:\x00"
                                           as *const u8 as
                                           *const libc::c_char)
                                           as isize));
                            }
                        }
                        printf(b" }\n\x00" as *const u8 as
                            *const libc::c_char);
                    }
                    j += 1
                }
                printf(b";; }\n;;\n\x00" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_anaphora_result(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    /* 照応解析結果を基本句のfeatureに付与 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    // let mut count: libc::c_int = 0;
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        tag_ptr = substance_tag_ptr((*sp).tag_data.offset(i as isize));
        sprintf(buf.as_mut_ptr(),
                b"EID:%d\x00" as *const u8 as *const libc::c_char,
                (*(*(*tag_ptr).mention_mgr.mention.as_mut_ptr()).entity).num +
                    base_entity_num);
        assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(),
                        0 as libc::c_int);
        buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        if check_feature((*tag_ptr).f,
                         b"\xef\xbc\xb4\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() {
            if OptDisplay != 5 as libc::c_int {
                if !check_feature((*tag_ptr).f,
                                  b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    convert_case_result(buf.as_mut_ptr(),
                                        check_feature((*tag_ptr).f,
                                                      b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char),
                                        (*sp).Sen_num);
                    assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(),
                                    0 as libc::c_int);
                    delete_cfeature(&mut (*tag_ptr).f,
                                    b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                }
            }
        } else if !(*tag_ptr).mention_mgr.cf_ptr.is_null() {
            let mut ga2_write_flag: libc::c_int = 0 as libc::c_int;
            let mut current_block_79: u64;
            j = 0 as libc::c_int;
            while j < (*(*tag_ptr).mention_mgr.cf_ptr).element_num {
                let mut filled_flag: libc::c_int = 0 as libc::c_int;
                let mut ga2_flag: libc::c_int = 0 as libc::c_int;
                if ga2_write_flag == 0 as libc::c_int {
                    if MatchPP((*(*tag_ptr).mention_mgr.cf_ptr).pp[j as
                        usize][0
                        as
                        libc::c_int
                        as
                        usize],
                               b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char)
                        == 0 {
                        if j ==
                            (*(*tag_ptr).mention_mgr.cf_ptr).element_num -
                                1 as libc::c_int {
                            ga2_write_flag = 1 as libc::c_int;
                            ga2_flag = 1 as libc::c_int;
                            j = -(1 as libc::c_int)
                        }
                        current_block_79 = 5948590327928692120;
                    } else {
                        ga2_write_flag = 1 as libc::c_int;
                        ga2_flag = 1 as libc::c_int;
                        current_block_79 = 14136749492126903395;
                    }
                } else if MatchPP((*(*tag_ptr).mention_mgr.cf_ptr).pp[j as
                    usize][0
                    as
                    libc::c_int
                    as
                    usize],
                                  b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char) != 0 {
                    current_block_79 = 5948590327928692120;
                } else { current_block_79 = 14136749492126903395; }
                match current_block_79 {
                    14136749492126903395 => {
                        k = 0 as libc::c_int;
                        while k < (*tag_ptr).mention_mgr.num {
                            mention_ptr =
                                (*tag_ptr).mention_mgr.mention.as_mut_ptr().offset(k
                                    as
                                    isize);
                            if (*mention_ptr).type_0 as libc::c_int ==
                                'N' as i32 ||
                                (*mention_ptr).type_0 as libc::c_int ==
                                    'C' as i32 ||
                                (*mention_ptr).type_0 as libc::c_int ==
                                    'O' as i32 ||
                                (*mention_ptr).type_0 as libc::c_int ==
                                    'D' as i32 ||
                                (*mention_ptr).type_0 as libc::c_int ==
                                    'E' as i32 {
                                if pp_kstr_to_code((*mention_ptr).cpp_string.as_mut_ptr())
                                    ==
                                    (*(*tag_ptr).mention_mgr.cf_ptr).pp[j
                                        as
                                        usize][0
                                        as
                                        libc::c_int
                                        as
                                        usize]
                                {
                                    filled_flag = 1 as libc::c_int;
                                    if OptDisplay == 5 as libc::c_int {
                                        let mut ellipsis_flag:
                                            [libc::c_char; 128] =
                                            *::std::mem::transmute::<&[u8; 128],
                                                &mut [libc::c_char; 128]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                                        if (*mention_ptr).type_0 as
                                            libc::c_int == 'O' as i32 ||
                                            (*mention_ptr).type_0 as
                                                libc::c_int == 'E' as i32 {
                                            strcpy(ellipsis_flag.as_mut_ptr(),
                                                   b"*\x00" as *const u8 as
                                                       *const libc::c_char);
                                        }
                                        if buf[0 as libc::c_int as usize] == 0
                                        {
                                            sprintf(buf.as_mut_ptr(),
                                                    b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0:\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            strcat(buf.as_mut_ptr(),
                                                   b";\x00" as *const u8 as
                                                       *const libc::c_char);
                                        }
                                        sprintf(tmp.as_mut_ptr(),
                                                b"%s%s/%s\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (*mention_ptr).cpp_string.as_mut_ptr(),
                                                ellipsis_flag.as_mut_ptr(),
                                                (*(*mention_ptr).entity).name.as_mut_ptr());
                                        strcat(buf.as_mut_ptr(),
                                               tmp.as_mut_ptr());
                                    } else {
                                        if buf[0 as libc::c_int as usize] == 0
                                        {
                                            sprintf(buf.as_mut_ptr(),
                                                    b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0:%s:\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    if OptReadFeature &
                                                        1 as libc::c_int !=
                                                        0 {
                                                        b"?\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    } else {
                                                        (*tag_ptr).mention_mgr.cf_id.as_mut_ptr()
                                                            as
                                                            *const libc::c_char
                                                    });
                                        } else {
                                            strcat(buf.as_mut_ptr(),
                                                   b";\x00" as *const u8 as
                                                       *const libc::c_char);
                                        }
                                        /* 直接係り受けをもっている場合 */
                                        if !(*mention_ptr).explicit_mention.is_null()
                                        {
                                            let mut name: *mut libc::c_char =
                                                make_print_string((*(*(*mention_ptr).explicit_mention).tag_ptr).b_ptr
                                                                      as
                                                                      *mut TAG_DATA,
                                                                  1 as
                                                                      libc::c_int);
                                            if !name.is_null() {
                                                sprintf(tmp.as_mut_ptr(),
                                                        b"%s/%c/%s/%d/%d/%d\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        (*mention_ptr).cpp_string.as_mut_ptr(),
                                                        (*mention_ptr).type_0
                                                            as libc::c_int,
                                                        name,
                                                        (*sp).Sen_num -
                                                            (*(*mention_ptr).explicit_mention).sent_num,
                                                        (*(*mention_ptr).explicit_mention).tag_num,
                                                        (*(*mention_ptr).entity).num);
                                                free(name as
                                                    *mut libc::c_void);
                                            }
                                        } else {
                                            sprintf(tmp.as_mut_ptr(),
                                                    b"%s/%c/%s/%d/%d/%d\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    (*mention_ptr).cpp_string.as_mut_ptr(),
                                                    (*mention_ptr).type_0 as
                                                        libc::c_int,
                                                    (*(*mention_ptr).entity).name.as_mut_ptr(),
                                                    (*sp).Sen_num -
                                                        (*(*mention_ptr).entity).rep_sen_num,
                                                    (*(*mention_ptr).entity).rep_tag_num,
                                                    (*(*mention_ptr).entity).num);
                                        }
                                        strcat(buf.as_mut_ptr(),
                                               tmp.as_mut_ptr());
                                    }
                                }
                            }
                            k += 1
                        }
                        if filled_flag == 0 as libc::c_int {
                            if OptDisplay == 5 as libc::c_int {
                                if (*(*tag_ptr).mention_mgr.cf_ptr).oblig[j as
                                    usize]
                                    ==
                                    (0 as libc::c_int == 0) as libc::c_int
                                    &&
                                    MatchPP((*(*tag_ptr).mention_mgr.cf_ptr).pp[j
                                        as
                                        usize][0
                                        as
                                        libc::c_int
                                        as
                                        usize],
                                            b"\xe4\xbf\xae\xe9\xa3\xbe\x00"
                                                as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) == 0 &&
                                    MatchPP((*(*tag_ptr).mention_mgr.cf_ptr).pp[j
                                        as
                                        usize][0
                                        as
                                        libc::c_int
                                        as
                                        usize],
                                            b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) == 0 {
                                    if buf[0 as libc::c_int as usize] == 0 {
                                        sprintf(buf.as_mut_ptr(),
                                                b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0:\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        strcat(buf.as_mut_ptr(),
                                               b";\x00" as *const u8 as
                                                   *const libc::c_char);
                                    }
                                    sprintf(tmp.as_mut_ptr(),
                                            b"%s/-\x00" as *const u8 as
                                                *const libc::c_char,
                                            pp_code_to_kstr((*(*tag_ptr).mention_mgr.cf_ptr).pp[j
                                                as
                                                usize][0
                                                as
                                                libc::c_int
                                                as
                                                usize]));
                                    strcat(buf.as_mut_ptr(),
                                           tmp.as_mut_ptr());
                                }
                            } else {
                                if buf[0 as libc::c_int as usize] == 0 {
                                    sprintf(buf.as_mut_ptr(),
                                            b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0:%s:\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            if OptReadFeature &
                                                1 as libc::c_int != 0 {
                                                b"?\x00" as *const u8 as
                                                    *const libc::c_char
                                            } else {
                                                (*tag_ptr).mention_mgr.cf_id.as_mut_ptr()
                                                    as *const libc::c_char
                                            });
                                } else {
                                    strcat(buf.as_mut_ptr(),
                                           b";\x00" as *const u8 as
                                               *const libc::c_char);
                                }
                                sprintf(tmp.as_mut_ptr(),
                                        b"%s/-/-/-/-/-\x00" as *const u8 as
                                            *const libc::c_char,
                                        pp_code_to_kstr((*(*tag_ptr).mention_mgr.cf_ptr).pp[j
                                            as
                                            usize][0
                                            as
                                            libc::c_int
                                            as
                                            usize]));
                                strcat(buf.as_mut_ptr(), tmp.as_mut_ptr());
                            }
                        }
                        if ga2_flag == 1 as libc::c_int {
                            j = -(1 as libc::c_int)
                        }
                    }
                    _ => {}
                }
                j += 1
            }
            if buf[0 as libc::c_int as usize] != 0 {
                assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(),
                                0 as libc::c_int);
                delete_cfeature(&mut (*tag_ptr).f,
                                b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                sprintf(buf.as_mut_ptr(),
                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe4\xbf\xa1\xe9\xa0\xbc\xe5\xba\xa6:%.3f\x00"
                            as *const u8 as *const libc::c_char,
                        (*tag_ptr).score_diff);
                assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(),
                                0 as libc::c_int);
                sprintf(buf.as_mut_ptr(),
                        b"\xe3\x82\xac\xe6\xa0\xbc\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe4\xbf\xa1\xe9\xa0\xbc\xe5\xba\xa6:%.3f\x00"
                            as *const u8 as *const libc::c_char,
                        (*tag_ptr).ga_score_diff);
                assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(),
                                0 as libc::c_int);
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn decay_entity()
/*==================================================================*/
{
    /* ENTITYの活性値を減衰させる */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < entity_manager.num {
        entity_manager.entity[i as usize].salience_score *= 0.5f64;
        entity_manager.entity[i as usize].tmp_salience_flag =
            0 as libc::c_int;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calculate_salience_score(mut sen_idx: libc::c_int)
/*==================================================================*/
{
    /* 各ENTITYのその文でのsalience_scoreを計算する */
    let mut entity_num: libc::c_int = 0;
    let mut mention_num: libc::c_int = 0;
    let mut sentence_distance: libc::c_int = 0;
    entity_num = 0 as libc::c_int;
    while entity_num < entity_manager.num {
        if OptAnaphora & 32 as libc::c_int != 0 &&
            (entity_num < 5 as libc::c_int ||
                entity_manager.entity[entity_num as
                    usize].hypothetical_entity !=
                    -(1 as libc::c_int) &&
                    entity_manager.entity[entity_num as
                        usize].hypothetical_entity <
                        5 as libc::c_int ||
                strcmp(entity_manager.entity[entity_num as
                    usize].named_entity.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char) != 0)
        {
            entity_manager.entity[entity_num as usize].salience_score =
                1 as libc::c_int as libc::c_double
        } else {
            entity_manager.entity[entity_num as usize].salience_score =
                0 as libc::c_int as libc::c_double
        }
        mention_num = 0 as libc::c_int;
        while mention_num <
            entity_manager.entity[entity_num as usize].mentioned_num {
            if !entity_manager.entity[entity_num as
                usize].mention[mention_num as
                usize].is_null()
            {
                sentence_distance =
                    sen_idx -
                        (*entity_manager.entity[entity_num as
                            usize].mention[mention_num
                            as
                            usize]).sent_num;
                if sentence_distance >= 0 as libc::c_int {
                    entity_manager.entity[entity_num as usize].salience_score
                        +=
                        (*entity_manager.entity[entity_num as
                            usize].mention[mention_num
                            as
                            usize]).static_salience_score
                            * pow(0.5f64, sentence_distance as libc::c_double)
                }
            }
            mention_num += 1
        }
        entity_manager.entity[entity_num as usize].tmp_salience_flag =
            0 as libc::c_int;
        entity_num += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn anaphora_analysis(mut sen_idx: libc::c_int)
/*==================================================================*/
{
    if OptAnaphora & 16384 as libc::c_int == 0 {
        calculate_salience_score(sen_idx);
        set_candidate_entities(sen_idx);
        make_context_structure(sentence_data.as_mut_ptr().offset(sen_idx as
            isize).offset(-(1
            as
            libc::c_int
            as
            isize)));
    }
    assign_anaphora_result(sentence_data.as_mut_ptr().offset(sen_idx as
        isize).offset(-(1
        as
        libc::c_int
        as
        isize)));
    if OptReadFeature & 64 as libc::c_int == 0 &&
        OptAnaphora & 16384 as libc::c_int == 0 {
        if OptAnaphora & 2 as libc::c_int != 0 { print_entities(sen_idx); }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn modify_weight()
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if ModifyWeight[0 as libc::c_int as usize] != 0. {
        i = 0 as libc::c_int;
        while i <
            2 as libc::c_int *
                ((4 as libc::c_int +
                    (33 as libc::c_int + 5 as libc::c_int) +
                    3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int +
                    5 as libc::c_int +
                    11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int +
                    4 as libc::c_int + 10 as libc::c_int) *
                    (3 as libc::c_int + 5 as libc::c_int)) {
            if i %
                (4 as libc::c_int + (33 as libc::c_int + 5 as libc::c_int)
                    + 3 as libc::c_int + 2 as libc::c_int +
                    8 as libc::c_int +
                    135 as libc::c_int * 3 as libc::c_int +
                    15 as libc::c_int * 3 as libc::c_int +
                    5 as libc::c_int * 4 as libc::c_int + 5 as libc::c_int
                    + 11 as libc::c_int * 2 as libc::c_int +
                    3 as libc::c_int * 2 as libc::c_int + 4 as libc::c_int
                    + 10 as libc::c_int) == 9 as libc::c_int {
                case_feature_weight[0 as libc::c_int as usize][i as usize] +=
                    ModifyWeight[0 as libc::c_int as usize];
                case_feature_weight[1 as libc::c_int as usize][i as usize] +=
                    ModifyWeight[1 as libc::c_int as usize];
                case_feature_weight[2 as libc::c_int as usize][i as usize] +=
                    ModifyWeight[2 as libc::c_int as usize]
            }
            i += 1
        }
        ModifyWeight[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_double
        /*複数回呼ばれても変化しないようにフラグ管理もかねる */
    };
}

#[no_mangle]
pub unsafe extern "C" fn count_yobikake(mut last_sp: *mut SENTENCE_DATA) {
    let mut sp: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut sen_idx: libc::c_int = 0;
    let mut mrph_idx: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    sen_idx = 1 as libc::c_int;
    while sen_idx < (*last_sp).Sen_num {
        sp =
            sentence_data.as_mut_ptr().offset(sen_idx as
                isize).offset(-(1 as
                libc::c_int
                as
                isize));
        mrph_idx = 0 as libc::c_int;
        while mrph_idx < (*sp).Mrph_num {
            cp =
                check_feature((*(*sp).mrph_data.offset(mrph_idx as isize)).f,
                              b"\xe5\x91\xbc\xe6\x8e\x9b\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
            if !cp.is_null() { yobikake_count += 1 }
            mrph_idx += 1
        }
        sen_idx += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn count_modality_keigo(mut last_sp:
                                              *mut SENTENCE_DATA) {
    let mut sp: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut sen_idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 11 as libc::c_int {
        modality_count[j as usize] = 0 as libc::c_int;
        j += 1
    }
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        keigo_count[j as usize] = 0 as libc::c_int;
        j += 1
    }
    sen_idx = 1 as libc::c_int;
    while sen_idx < (*last_sp).Sen_num {
        sp =
            sentence_data.as_mut_ptr().offset(sen_idx as
                isize).offset(-(1 as
                libc::c_int
                as
                isize));
        i = (*sp).Tag_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
            tag_ptr = substance_tag_ptr((*sp).tag_data.offset(i as isize));
            j = 0 as libc::c_int;
            while j < 11 as libc::c_int {
                let mut mod_0: [libc::c_char; 5120] =
                    *::std::mem::transmute::<&[u8; 5120],
                        &mut [libc::c_char; 5120]>(b"\xe3\x83\xa2\xe3\x83\x80\xe3\x83\xaa\xe3\x83\x86\xe3\x82\xa3-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                strcat(mod_0.as_mut_ptr(), modality[j as usize]);
                cp = check_feature((*tag_ptr).f, mod_0.as_mut_ptr());
                if !cp.is_null() { modality_count[j as usize] += 1 }
                j += 1
            }
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                let mut kei: [libc::c_char; 5120] =
                    *::std::mem::transmute::<&[u8; 5120],
                        &mut [libc::c_char; 5120]>(b"\xe6\x95\xac\xe8\xaa\x9e:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                strcat(kei.as_mut_ptr(), keigo[j as usize]);
                cp_0 = check_feature((*tag_ptr).f, kei.as_mut_ptr());
                if !cp_0.is_null() { keigo_count[j as usize] += 1 }
                j += 1
            }
            i -= 1
        }
        sen_idx += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_context_feature() {
    let mut entity_idx: libc::c_int = 0;
    entity_idx = 0 as libc::c_int;
    while entity_idx < entity_manager.num {
        let mut mention_idx: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int {
            context_feature[entity_manager.entity[entity_idx as usize].num as
                usize][i as usize] =
                0 as libc::c_int as libc::c_double;
            i += 1
        }
        if !(entity_manager.entity[entity_idx as usize].skip_flag ==
            1 as libc::c_int) {
            /*
		  一文目での出現
		  文末 判定詞0
		  文末 体言止め1
		  文末 2
		  文頭 3
		  文頭 ハ4
		  文内 5
		  文内 ガ格6
		  文内 ヲ格7
		  文内 ニ格8
		  文内 ハ9
		*/
            mention_idx = 0 as libc::c_int;
            while mention_idx <
                entity_manager.entity[entity_idx as usize].mentioned_num
            {
                let mut mention_ptr: *mut MENTION = 0 as *mut MENTION;
                mention_ptr =
                    entity_manager.entity[entity_idx as
                        usize].mention[mention_idx as
                        usize];
                if (*mention_ptr).sent_num == 1 as libc::c_int {
                    if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                      b"\xe6\x96\x87\xe6\x9c\xab\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                        if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                          b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            context_feature[entity_manager.entity[entity_idx
                                as
                                usize].num
                                as
                                usize][0 as libc::c_int as
                                usize] =
                                1 as libc::c_int as libc::c_double
                        }
                        if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                          b"\xe4\xbd\x93\xe8\xa8\x80\xe6\xad\xa2\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            context_feature[entity_manager.entity[entity_idx
                                as
                                usize].num
                                as
                                usize][1 as libc::c_int as
                                usize] =
                                1 as libc::c_int as libc::c_double
                        }
                        context_feature[entity_manager.entity[entity_idx as
                            usize].num
                            as
                            usize][2 as libc::c_int as usize]
                            = 1 as libc::c_int as libc::c_double
                    }
                    if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                      b"\xe6\x96\x87\xe9\xa0\xad\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                        context_feature[entity_manager.entity[entity_idx as
                            usize].num
                            as
                            usize][3 as libc::c_int as usize]
                            = 1 as libc::c_int as libc::c_double;
                        if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                          b"\xe3\x83\x8f\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            context_feature[entity_manager.entity[entity_idx
                                as
                                usize].num
                                as
                                usize][4 as libc::c_int as
                                usize] =
                                1 as libc::c_int as libc::c_double
                        }
                    }
                    //context_feature[entity_manager.entity[entity_idx].num][5] = 1;
                    if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                      b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                        context_feature[entity_manager.entity[entity_idx as
                            usize].num
                            as
                            usize][6 as libc::c_int as usize]
                            = 1 as libc::c_int as libc::c_double
                    }
                    !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                   b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null();
                    !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                   b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null();
                    if !check_feature((*(*(*mention_ptr).tag_ptr).b_ptr).f,
                                      b"\xe3\x83\x8f\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                        context_feature[entity_manager.entity[entity_idx as
                            usize].num
                            as
                            usize][9 as libc::c_int as usize]
                            = 1 as libc::c_int as libc::c_double
                    }
                }
                mention_idx += 1
            }
        }
        entity_idx += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_author_reader() {
    let mut i: libc::c_int = 0;
    if OptAnaphora & 16 as libc::c_int != 0 ||
        OptReadFeature & 1 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            entity_manager.entity[i as usize].skip_flag = 0 as libc::c_int;
            i += 1
        }
    } else if OptAnaphora & 32768 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            entity_manager.entity[i as usize].skip_flag = 1 as libc::c_int;
            i += 1
        }
    } else {
        if OptAnaphora & 128 as libc::c_int != 0 ||
            OptAnaphora & 32 as libc::c_int == 0 {
            entity_manager.entity[0 as libc::c_int as usize].skip_flag =
                1 as libc::c_int
        } else {
            entity_manager.entity[0 as libc::c_int as usize].skip_flag =
                0 as libc::c_int
        }
        if OptAnaphora & 256 as libc::c_int != 0 ||
            OptAnaphora & 32 as libc::c_int == 0 {
            entity_manager.entity[1 as libc::c_int as usize].skip_flag =
                1 as libc::c_int
        } else {
            entity_manager.entity[1 as libc::c_int as usize].skip_flag =
                0 as libc::c_int
        }
        i = 2 as libc::c_int;
        while i < 5 as libc::c_int {
            entity_manager.entity[i as usize].skip_flag = 0 as libc::c_int;
            i += 1
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn each_sentence_anaphora_analysis(mut sp:
                                                         *mut SENTENCE_DATA) {
    // let mut sen_idx: libc::c_int = 0;
    let sen_max: libc::c_int = (*sp).Sen_num;
    if sen_max == 1 as libc::c_int {
        set_param();
        /* if((OptAnaphora & OPT_UNNAMED_ENTITY) || (OptAnaphora & OPT_TRAIN) ||(OptAnaphora & OPT_GS) ) */
        make_unnamed_entity();
        set_author_reader();
        if OptAnaphora & 512 as libc::c_int != 0 {
            author_score = author_score / sen_max as libc::c_double
        }
        if OptAnaphora & 1024 as libc::c_int != 0 {
            reader_score = reader_score / sen_max as libc::c_double
        }
    }
    count_yobikake(sp);
    count_modality_keigo(sp);
    make_entity_from_coreference(sentence_data.as_mut_ptr().offset(sen_max as
        isize).offset(-(1
        as
        libc::c_int
        as
        isize)));
    if OptAnaphora & 8192 as libc::c_int == 0 ||
        OptAnaphora & 16 as libc::c_int != 0 ||
        OptAnaphora & 64 as libc::c_int != 0 {
        if OptReadFeature & 128 as libc::c_int != 0 {
            merge_hypo_real_entity_auto();
        }
        if !(OptReadFeature & 32 as libc::c_int != 0) {
            if OptReadFeature & 128 as libc::c_int != 0 {
                reset_hypo_information();
                merge_hypo_real_entity_auto();
            } else { author_detect(); }
        }
    }
    get_context_feature();
    anaphora_analysis(sen_max);
    if OptAnaphora & 2 as libc::c_int != 0 { print_entities(sen_max); };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn all_sentence_anaphora_analysis(mut sp:
                                                        *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut sen_idx: libc::c_int = 0;
    let sen_max: libc::c_int = (*sp).Sen_num;
    let mut buf: [libc::c_char; 128] = [0; 128];
    set_param();
    (OptAnaphora & 16 as libc::c_int) == 0;
    //OptAnaphora |= OPT_ITERATIVE;
    count_yobikake(sp);
    count_modality_keigo(sp);
    if OptAnaphora & 32 as libc::c_int != 0 ||
        OptAnaphora & 16 as libc::c_int != 0 ||
        OptAnaphora & 64 as libc::c_int != 0 {
        make_unnamed_entity();
    }
    sen_idx = 1 as libc::c_int;
    while sen_idx < sen_max {
        make_entity_from_coreference(sentence_data.as_mut_ptr().offset(sen_idx
            as
            isize).offset(-(1
            as
            libc::c_int
            as
            isize)));
        sen_idx += 1
    }
    if OptAnaphora & 512 as libc::c_int != 0 {
        author_score =
            author_score / (sen_max - 1 as libc::c_int) as libc::c_double
    }
    if OptAnaphora & 1024 as libc::c_int != 0 {
        reader_score =
            reader_score / (sen_max - 1 as libc::c_int) as libc::c_double
    }
    if OptReadFeature & 128 as libc::c_int != 0 &&
        OptAnaphora & 64 as libc::c_int == 0 &&
        OptAnaphora & 16 as libc::c_int == 0 {
        OptReadFeature &= !(32 as libc::c_int)
    }
    if OptAnaphora & 8192 as libc::c_int == 0 ||
        OptAnaphora & 16 as libc::c_int != 0 ||
        OptAnaphora & 64 as libc::c_int != 0 {
        set_author_reader();
        if OptReadFeature & 8 as libc::c_int != 0 {
            merge_hypo_real_entity();
        }
        if !(OptReadFeature & 32 as libc::c_int != 0) {
            if OptReadFeature & 128 as libc::c_int != 0 {
                reset_hypo_information();
                merge_hypo_real_entity_auto();
            } else { author_detect(); }
        }
    }
    get_context_feature();
    analysis_flag = 0 as libc::c_int;
    ite_count = 0 as libc::c_int;
    loop {
        max_reliabirity = -(10000 as libc::c_int) as libc::c_double;
        max_reliabirity_tag_ptr = 0 as *mut TAG_DATA;
        sen_idx = 1 as libc::c_int;
        while sen_idx < sen_max {
            anaphora_analysis(sen_idx);
            sen_idx += 1
        }
        if !(OptAnaphora & 131072 as libc::c_int != 0) { break; }
        if OptAnaphora & 16 as libc::c_int != 0 {
            if !(analysis_flag == 0 as libc::c_int) { break; }
            analysis_flag = 1 as libc::c_int
        } else {
            if max_reliabirity == -(10000 as libc::c_int) as libc::c_double {
                break;
            }
            analysis_flags[(*(*max_reliabirity_tag_ptr).mention_mgr.mention.as_mut_ptr()).sent_num
                as
                usize][(*(*max_reliabirity_tag_ptr).mention_mgr.mention.as_mut_ptr()).tag_num
                as usize] = 1 as libc::c_int;
            anaphora_result_to_entity(max_reliabirity_tag_ptr);
            assign_cfeature(&mut (*max_reliabirity_tag_ptr).f,
                            b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe6\xb8\x88\xe3\x81\xbf\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            sprintf(buf.as_mut_ptr(),
                    b"\xe8\xa7\xa3\xe6\x9e\x90\xe9\xa0\x86\xe5\xba\x8f:%d\x00"
                        as *const u8 as *const libc::c_char, ite_count);
            assign_cfeature(&mut (*max_reliabirity_tag_ptr).f,
                            buf.as_mut_ptr(), 0 as libc::c_int);
            free((*max_reliabirity_tag_ptr).ctm_ptr as *mut libc::c_void);
            free((*max_reliabirity_tag_ptr).tcf_ptr as *mut libc::c_void);
            ite_count += 1
        }
    }
    if OptAnaphora & 8192 as libc::c_int != 0 &&
        (OptAnaphora & 32 as libc::c_int != 0 ||
            OptAnaphora & 16 as libc::c_int != 0 ||
            OptAnaphora & 64 as libc::c_int != 0) {
        sen_idx = 1 as libc::c_int;
        while sen_idx < sen_max {
            link_hypo_enity_after_analysis(sentence_data.as_mut_ptr().offset(sen_idx
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize)));
            sen_idx += 1
        }
        if OptReadFeature & 128 as libc::c_int != 0 {
            merge_hypo_real_entity_auto();
        } else if OptReadFeature & 32 as libc::c_int != 0 {
            merge_hypo_real_entity();
        } else { author_detect(); }
        if OptAnaphora & 2 as libc::c_int != 0 {
            print_entities(sen_max - 1 as libc::c_int);
        }
    };
}
