#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! KNP
use libc;
use crate::anaphora::{all_sentence_anaphora_analysis, assign_mrph_num, clear_context};
use crate::base_phrase::fragment;
use crate::bnst_compare::calc_match_matrix;
use crate::case_analysis::{ADJACENT_TOUTEN_COST, DISTANCE_STEP, init_case_analysis_cmm, LEVELA_COST, record_all_case_analisys, RENKAKU_STEP, STRONG_V_COST, TEIDAI_STEP};
use crate::case_ipal::{assign_pred_feature_to_bp, clear_cf, close_cf, init_case_analysis_cpm, init_cf, init_mrph2id, init_soto_txt, PrintDeletedSM, set_caseframes, set_frame_num_max};
use crate::case_match::{EX_match_qua, EX_match_sentence, EX_match_subject, EX_match_tim, EX_match_unknown, SOTO_THRESHOLD};
use crate::case_print::{EX_PRINT_NUM, PrintFrequency};
use crate::configfile::{init_configfile, server_read_rc};
use crate::consts::SOCK_STREAM;
use crate::context::{CFSimThreshold, DiscourseAnalysis, OptUseSmfix, PreserveCPM, PreserveSentence, PrintEx, PrintFeatures};
use crate::corefer::corefer_analysis;
use crate::ctools::{__bswap_16, __bswap_32, __errno_location, _setjmp, accept, alarm, AntecedentDecideThresholdForNoun, assign_cfeature, atof, atoi, atol, bind, check_feature, Class, close, connect, exit, fclose, fdopen, fflush, fgets, fopen, fork, fprintf, fputs, free, fwrite, getgrgid, gethostbyname, getpid, getpwnam, grammar, Infp, init_crf_for_NE, init_distsim, katuyou, Language, listen, longjmp, malloc_data, memset, OptAnalysis, OptCFMode, OptChiPos, Options, OptKatakanaNormalize, OptMergeCFResult, OptNbest, OptNEend, Outfp, perror, printf, setgid, setgroups, setuid, shutdown, sigfillset, signal, sigprocmask, sleep, socket, sprintf, sscanf, stderr, stdin, stdout, strcasecmp, strcat, strchr, strcmp, strcpy, strdup, strerror, strlen, strncasecmp, strncmp, strstr, umask, waitpid};
use crate::dpnd_analysis::{after_decide_dpnd, assign_dpnd_rule, calc_chi_dpnd_matrix_forProbModel, calc_chi_dpnd_matrix_wpos, calc_chi_pos_matrix, calc_dpnd_matrix, calc_gigaword_pa_matrix, check_candidates, close_chi_dpnd_db, detect_dpnd_case_struct, dpnd_info_to_bnst, dpnd_info_to_mrph, dpnd_info_to_tag, free_chi_pos, free_chi_type, init_chi_dpnd_db, init_chi_pos, init_chi_type, memo_by_program, para_postprocess, relax_dpnd_matrix, when_no_dpnd_struct};
use crate::feature::{clear_feature, copy_feature, delete_cfeature};
use crate::lib_print::{assign_para_similarity_feature, check_bnst, do_postprocess, prepare_all_entity, print_all_result, print_bnst_with_mrphs, print_matrix, print_para_relation, print_result};
use crate::lib_scase::{close_scase, init_scase, OptUseScase};
use crate::lib_sm::fix_sm_person;
use crate::para_analysis::{check_para_key, detect_all_para_scope, detect_para_scope, para_recovery};
use crate::para_dpnd::{check_dpnd_in_para, init_mask_matrix, init_para_matrix};
use crate::para_relation::detect_para_relation;
use crate::proper::DBforNE;
use crate::quote::{prepare_paren_sentence, process_input_paren};
use crate::read_data::{assign_cc_feature_to_bnst, assign_cc_feature_to_bp, assign_general_feature, make_bunsetsu, make_bunsetsu_pm, make_tag_units, preprocess_mrph, read_mrph};
use crate::read_rule::{ContRuleArray, ContRuleSize, read_bnst_rule, read_dpnd_rule, read_dpnd_rule_for_chinese, read_general_rule, read_homo_rule, read_koou_rule};
use crate::similarity::init_hownet;
use crate::structs::{__sigset_t, _FEATURE, CF_ALIGNMENT, cf_def, CF_MATCH_MGR, CHI_DPND, CHI_POS, cpm_def, ctm_def, DPND, DpndRule, entity, FEATURE_PATTERN, group, hostent, in_addr, LIST, mention, MRPH_DATA, node_para_manager, passwd, sentence, sockaddr, sockaddr_in, tcf_def, tnode_b, tnode_t, TOTAL_MGR};
use crate::thesaurus::{close_thesaurus, get_bnst_code_all, init_thesaurus};
use crate::tools::{author_score, author_sen, author_tag, CLASS_num, CurrentRuleNum, fd, init_hash, is_frag, Opt_knprc, OptAddSvmFeatureDiscourseDepth, OptAddSvmFeatureObjectRecognition, OptAddSvmFeatureReferedNum, OptAddSvmFeatureUtype, OptAnaphora, OptAnaphoraBaseline, OptArticle, OptBeam, OptCaseFlag, OptCheck, OptChiGenerative, OptCKY, OptCopula, OptCorefer, OptDiscFlag, OptDiscNounMethod, OptDiscPredMethod, OptDisplay, OptDisplayNE, OptEllipsis, OptExpandP, OptExpress, OptGeneralCF, OptHostname, OptIgnoreChar, OptInput, OptLearn, OptMode, OptNE, OptNEcache, OptNEcase, OptNECRF, OptNEdelete, OptNElearn, OptNEparent, OptNoCandidateBehind, OptParaFix, OptParaNoFixFlag, OptPort, OptPosModification, OptPostProcess, OptProcessParen, OptReadFeature, OptReadNE, OptRecoverPerson, OptSemanticHead, OptServerFlag, OptTimeoutExit, OptUseCF, OptUseCPNCF, OptUseNCF, OptUseRN, ParaThesaurus, ParseTimeout, PrintNum, reader_score, reader_sen, reader_tag, RULE, sen_num, sfd, Thesaurus, timeout, VerboseLevel};
use crate::tree_conv::make_dpnd_tree;
use crate::types::{__mode_t, __sighandler_t, __uint16_t, BNST_DATA, CASE_FRAME, CF_PRED_MGR, CHECK_DATA, ENTITY, ENTITY_MGR, FEATURE, FEATUREptr, FILE, gid_t, in_addr_t, MENTION, MENTION_MGR, PARA_DATA, PARA_MANAGER, sa_family_t, SENTENCE_DATA, sigset_t, size_t, socklen_t, TAG_DATA, VerboseType};

mod anaphora;
mod base_phrase;
mod bnst_compare;
mod case_analysis;
mod case_data;
mod case_ipal;
mod case_match;
mod case_print;
mod cky;
mod configfile;
mod context;
mod corefer;
mod db;
mod dic;
mod dpnd_analysis;
mod feature;
mod ipal;
mod koou;
mod lib_bgh;
mod lib_dt;
mod lib_event;
mod lib_print;
mod lib_scase;
mod lib_sm;
mod make_db;
mod nv_mi;
mod para_analysis;
mod para_dpnd;
mod para_relation;
mod para_revision;
mod proper;
mod quote;
mod read_data;
mod read_rule;
mod regexp;
mod similarity;
mod thesaurus;
mod tools;
mod tree_conv;
mod lib_ps;
mod types;
mod ctools;
mod consts;
mod structs;
mod juman;
mod cdb;

#[no_mangle]
pub static mut current_sentence_data: SENTENCE_DATA = SENTENCE_DATA{
    Sen_num: 0,
    available: 0,
    Mrph_num: 0,
    New_Mrph_num: 0,
    Bnst_num: 0,
    New_Bnst_num: 0,
    Max_New_Bnst_num: 0,
    Tag_num: 0,
    New_Tag_num: 0,
    Para_M_num: 0,
    Para_num: 0,
    frame_num_max: 0,
    mrph_data: 0 as *const MRPH_DATA as *mut MRPH_DATA,
    bnst_data: 0 as *const BNST_DATA as *mut BNST_DATA,
    tag_data: 0 as *const TAG_DATA as *mut TAG_DATA,
    para_data: 0 as *const PARA_DATA as *mut PARA_DATA,
    para_manager: 0 as *const PARA_MANAGER as *mut PARA_MANAGER,
    cpm: 0 as *const CF_PRED_MGR as *mut CF_PRED_MGR,
    cf: 0 as *const CASE_FRAME as *mut CASE_FRAME,
    Best_mgr: 0 as *const TOTAL_MGR as *mut TOTAL_MGR,
    KNPSID: 0 as *const libc::c_char as *mut libc::c_char,
    Comment: 0 as *const libc::c_char as *mut libc::c_char,
    score: 0.,
};
#[no_mangle]
pub static mut sentence_data: [SENTENCE_DATA; 512] =
    [SENTENCE_DATA{Sen_num: 0,
                   available: 0,
                   Mrph_num: 0,
                   New_Mrph_num: 0,
                   Bnst_num: 0,
                   New_Bnst_num: 0,
                   Max_New_Bnst_num: 0,
                   Tag_num: 0,
                   New_Tag_num: 0,
                   Para_M_num: 0,
                   Para_num: 0,
                   frame_num_max: 0,
                   mrph_data: 0 as *const MRPH_DATA as *mut MRPH_DATA,
                   bnst_data: 0 as *const BNST_DATA as *mut BNST_DATA,
                   tag_data: 0 as *const TAG_DATA as *mut TAG_DATA,
                   para_data: 0 as *const PARA_DATA as *mut PARA_DATA,
                   para_manager: 0 as *const PARA_MANAGER as *mut PARA_MANAGER,
                   cpm: 0 as *const CF_PRED_MGR as *mut CF_PRED_MGR,
                   cf: 0 as *const CASE_FRAME as *mut CASE_FRAME,
                   Best_mgr: 0 as *const TOTAL_MGR as *mut TOTAL_MGR,
                   KNPSID: 0 as *const libc::c_char as *mut libc::c_char,
                   Comment: 0 as *const libc::c_char as *mut libc::c_char,
                   score: 0.,}; 512];
#[no_mangle]
pub static mut paren_sentence_data: *mut SENTENCE_DATA = 0 as *const SENTENCE_DATA as *mut SENTENCE_DATA;
#[no_mangle]
pub static mut mrph_data: [MRPH_DATA; 200] =
    [MRPH_DATA{type_0: 0,
               num: 0,
               parent: 0 as *const tnode_b as *mut tnode_b,
               child: [0 as *const tnode_b as *mut tnode_b; 32],
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
               f: 0 as *const _FEATURE as *mut _FEATURE,
               Num: 0,
               SM: 0 as *const libc::c_char as *mut libc::c_char,
               Pos: [0; 4],
               Type: [0; 9],}; 200];
/* 形態素データ */
#[no_mangle]
pub static mut bnst_data: [BNST_DATA; 200] =
    [BNST_DATA{type_0: 0,
               num: 0,
               parent: 0 as *const tnode_b as *mut tnode_b,
               child: [0 as *const tnode_b as *mut tnode_b; 32],
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
               sp_level: 0,
               mrph_num: 0,
               preserve_mrph_num: 0,
               mrph_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
               head_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
               BGH_code: [0; 2817],
               BGH_num: 0,
               SM_code: [0; 3073],
               SM_num: 0,
               voice: 0,
               cf_num: 0,
               cf_ptr: 0 as *const cf_def as *mut cf_def,
               cpm_ptr: 0 as *const cpm_def as *mut cpm_def,
               pred_num: 0,
               f: 0 as *const _FEATURE as *mut _FEATURE,
               pred_b_ptr: 0 as *const tnode_b as *mut tnode_b,
               is_para: 0,
               SCASE_code: [0; 11],
               Jiritu_Go: [0; 256],
               dpnd_rule: 0 as *const DpndRule as *mut DpndRule,
               tag_ptr: 0 as *const tnode_t as *mut tnode_t,
               tag_num: 0,}; 200];
/* 文節データ */
#[no_mangle]
pub static mut tag_data: [TAG_DATA; 200] =
    [TAG_DATA{type_0: 0,
              num: 0,
              parent: 0 as *const tnode_t as *mut tnode_t,
              child: [0 as *const tnode_t as *mut tnode_t; 32],
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
              sp_level: 0,
              mrph_num: 0,
              preserve_mrph_num: 0,
              mrph_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
              head_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
              BGH_code: [0; 2817],
              BGH_num: 0,
              SM_code: [0; 3073],
              SM_num: 0,
              voice: 0,
              cf_num: 0,
              cf_ptr: 0 as *const cf_def as *mut cf_def,
              cpm_ptr: 0 as *const cpm_def as *mut cpm_def,
              pred_num: 0,
              f: 0 as *const _FEATURE as *mut _FEATURE,
              pred_b_ptr: 0 as *const tnode_t as *mut tnode_t,
              is_para: 0,
              SCASE_code: [0; 11],
              bnum: 0,
              inum: 0,
              b_ptr: 0 as *const BNST_DATA as *mut BNST_DATA,
              settou_num: 0,
              jiritu_num: 0,
              fuzoku_num: 0,
              settou_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
              jiritu_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
              fuzoku_ptr: 0 as *const MRPH_DATA as *mut MRPH_DATA,
              e_cf_num: 0,
              c_cpm_ptr: 0 as *const cpm_def as *mut cpm_def,
              next: 0 as *const tnode_t as *mut tnode_t,
              mention_mgr:
                  MENTION_MGR{num: 0,
                              cf_id: [0; 280],
                              cf_ptr: 0 as *const cf_def as *mut cf_def,
                              mention:
                                  [MENTION{sent_num: 0,
                                           tag_num: 0,
                                           cpp_string: [0; 16],
                                           spp_string: [0; 16],
                                           type_0: 0,
                                           salience_score: 0.,
                                           static_salience_score: 0.,
                                           tag_ptr:
                                               0 as *const tnode_t as
                                                   *mut tnode_t,
                                           entity:
                                               0 as *const entity as
                                                   *mut entity,
                                           explicit_mention:
                                               0 as *const mention as
                                                   *mut mention,}; 8],},
              tcf_ptr: 0 as *const tcf_def as *mut tcf_def,
              ctm_ptr: 0 as *const ctm_def as *mut ctm_def,
              score_diff: 0.,
              ga_score_diff: 0.,}; 200];
/* タグ単位データ */
#[no_mangle]
pub static mut para_data: [PARA_DATA; 32] =
    [PARA_DATA{para_char: 0,
               type_0: 0,
               max_num: 0,
               key_pos: 0,
               iend_pos: 0,
               jend_pos: 0,
               max_path: [0; 200],
               f_pattern:
                   FEATURE_PATTERN{fp:
                                       [0 as *const FEATURE as *mut FEATURE;
                                           16],},
               max_score: 0.,
               pure_score: 0.,
               status: 0,
               manager_ptr:
                   0 as *const node_para_manager as *mut node_para_manager,};
        32];
/* 並列データ */
#[no_mangle]
pub static mut para_manager: [PARA_MANAGER; 32] =
    [PARA_MANAGER{para_num: 0,
                  para_data_num: [0; 32],
                  part_num: 0,
                  start: [0; 32],
                  end: [0; 32],
                  parent:
                      0 as *const node_para_manager as *mut node_para_manager,
                  child:
                      [0 as *const node_para_manager as
                           *mut node_para_manager; 32],
                  child_num: 0,
                  bnst_ptr: 0 as *const BNST_DATA as *mut BNST_DATA,
                  status: 0,}; 32];
/* 並列管理データ */
#[no_mangle]
pub static mut Best_mgr: TOTAL_MGR =
    TOTAL_MGR{dpnd:
                  DPND{head: [0; 200],
                       type_0: [0; 200],
                       dflt: [0; 200],
                       mask: [0; 200],
                       pos: 0,
                       check:
                           [CHECK_DATA{num: 0, def: 0, pos: [0; 200],}; 200],
                       f: [0 as *const FEATURE as *mut FEATURE; 200],},
              pssb: 0,
              dflt: 0,
              score: 0.,
              pred_num: 0,
              cpm:
                  [CF_PRED_MGR{cf:
                  CASE_FRAME{type_0: 0,
                                              type_flag: 0,
                                              element_num: 0,
                                              oblig: [0; 24],
                                              adjacent: [0; 24],
                                              pp: [[0; 10]; 24],
                                              sp: [0; 24],
                                              pp_str:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_delete:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_delete_size: [0; 24],
                                              sm_delete_num: [0; 24],
                                              sm_specify:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_specify_size: [0; 24],
                                              sm_specify_num: [0; 24],
                                              ex:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              ex_list:
                                                  [0 as
                                                       *const *mut libc::c_char
                                                       as
                                                       *mut *mut libc::c_char;
                                                      24],
                                              ex_freq:
                                                  [0 as *const libc::c_int as
                                                       *mut libc::c_int; 24],
                                              ex_size: [0; 24],
                                              ex_num: [0; 24],
                                              freq: [0; 24],
                                              semantics:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              gex_list:
                                                  [0 as
                                                       *const *mut libc::c_char
                                                       as
                                                       *mut *mut libc::c_char;
                                                      24],
                                              gex_freq:
                                                  [0 as *const libc::c_double
                                                       as *mut libc::c_double;
                                                      24],
                                              gex_size: [0; 24],
                                              gex_num: [0; 24],
                                              voice: 0,
                                              cf_address: 0,
                                              cf_size: 0,
                                              cf_id: [0; 280],
                                              pred_type: [0; 4],
                                              entry:
                                                  0 as *const libc::c_char as
                                                      *mut libc::c_char,
                                              imi: [0; 128],
                                              etcflag: 0,
                                              feature:
                                                  0 as *const libc::c_char as
                                                      *mut libc::c_char,
                                              weight: [0; 24],
                                              samecase: [[0; 2]; 24],
                                              cf_align:
                                                  [CF_ALIGNMENT{cf_id:
                                                                    0 as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                aligned_case:
                                                                    [[0; 2];
                                                                        24],};
                                                      5],
                                              pred_b_ptr:
                                                  0 as *const TAG_DATA as
                                                      *mut TAG_DATA,
                                              cf_similarity: 0.,},
                               pred_b_ptr:
                                   0 as *const TAG_DATA as *mut TAG_DATA,
                               elem_b_ptr:
                                   [0 as *const TAG_DATA as *mut TAG_DATA;
                                       24],
                               para_b_ptr:
                                   [0 as *const TAG_DATA as *mut TAG_DATA;
                                       24],
                               elem_s_ptr:
                                   [0 as *const sentence as *mut sentence;
                                       24],
                               elem_b_num: [0; 24],
                               score: 0.,
                               result_num: 0,
                               tie_num: 0,
                               cmm:
                                   [CF_MATCH_MGR{cf_ptr:
                                                     0 as *const CASE_FRAME as
                                                         *mut CASE_FRAME,
                                                 score: 0.,
                                                 pure_score: [0.; 10],
                                                 sufficiency: 0.,
                                                 result_num: 0,
                                                 result_lists_p:
                                                     [LIST{flag: [0; 24],
                                                           score: [0.; 24],
                                                           pos: [0; 24],};
                                                         10],
                                                 result_lists_d:
                                                     [LIST{flag: [0; 24],
                                                           score: [0.; 24],
                                                           pos: [0; 24],};
                                                         10],
                                                 cpm:
                                                     0 as *const cpm_def as
                                                         *mut cpm_def,}; 5],
                               decided: 0,}; 64],
              ID: 0,};
/* 依存・格解析管理データ */
#[no_mangle]
pub static mut Op_Best_mgr: TOTAL_MGR =
    TOTAL_MGR{dpnd:
                  DPND{head: [0; 200],
                       type_0: [0; 200],
                       dflt: [0; 200],
                       mask: [0; 200],
                       pos: 0,
                       check:
                           [CHECK_DATA{num: 0, def: 0, pos: [0; 200],}; 200],
                       f: [0 as *const FEATURE as *mut FEATURE; 200],},
              pssb: 0,
              dflt: 0,
              score: 0.,
              pred_num: 0,
              cpm:
                  [CF_PRED_MGR{cf:
                                   CASE_FRAME{type_0: 0,
                                              type_flag: 0,
                                              element_num: 0,
                                              oblig: [0; 24],
                                              adjacent: [0; 24],
                                              pp: [[0; 10]; 24],
                                              sp: [0; 24],
                                              pp_str:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_delete:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_delete_size: [0; 24],
                                              sm_delete_num: [0; 24],
                                              sm_specify:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              sm_specify_size: [0; 24],
                                              sm_specify_num: [0; 24],
                                              ex:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              ex_list:
                                                  [0 as
                                                       *const *mut libc::c_char
                                                       as
                                                       *mut *mut libc::c_char;
                                                      24],
                                              ex_freq:
                                                  [0 as *const libc::c_int as
                                                       *mut libc::c_int; 24],
                                              ex_size: [0; 24],
                                              ex_num: [0; 24],
                                              freq: [0; 24],
                                              semantics:
                                                  [0 as *const libc::c_char as
                                                       *mut libc::c_char; 24],
                                              gex_list:
                                                  [0 as
                                                       *const *mut libc::c_char
                                                       as
                                                       *mut *mut libc::c_char;
                                                      24],
                                              gex_freq:
                                                  [0 as *const libc::c_double
                                                       as *mut libc::c_double;
                                                      24],
                                              gex_size: [0; 24],
                                              gex_num: [0; 24],
                                              voice: 0,
                                              cf_address: 0,
                                              cf_size: 0,
                                              cf_id: [0; 280],
                                              pred_type: [0; 4],
                                              entry:
                                                  0 as *const libc::c_char as
                                                      *mut libc::c_char,
                                              imi: [0; 128],
                                              etcflag: 0,
                                              feature:
                                                  0 as *const libc::c_char as
                                                      *mut libc::c_char,
                                              weight: [0; 24],
                                              samecase: [[0; 2]; 24],
                                              cf_align:
                                                  [CF_ALIGNMENT{cf_id:
                                                                    0 as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                aligned_case:
                                                                    [[0; 2];
                                                                        24],};
                                                      5],
                                              pred_b_ptr:
                                                  0 as *const TAG_DATA as
                                                      *mut TAG_DATA,
                                              cf_similarity: 0.,},
                               pred_b_ptr:
                                   0 as *const TAG_DATA as *mut TAG_DATA,
                               elem_b_ptr:
                                   [0 as *const TAG_DATA as *mut TAG_DATA;
                                       24],
                               para_b_ptr:
                                   [0 as *const TAG_DATA as *mut TAG_DATA;
                                       24],
                               elem_s_ptr:
                                   [0 as *const sentence as *mut sentence;
                                       24],
                               elem_b_num: [0; 24],
                               score: 0.,
                               result_num: 0,
                               tie_num: 0,
                               cmm:
                                   [CF_MATCH_MGR{cf_ptr:
                                                     0 as *const CASE_FRAME as
                                                         *mut CASE_FRAME,
                                                 score: 0.,
                                                 pure_score: [0.; 10],
                                                 sufficiency: 0.,
                                                 result_num: 0,
                                                 result_lists_p:
                                                     [LIST{flag: [0; 24],
                                                           score: [0.; 24],
                                                           pos: [0; 24],};
                                                         10],
                                                 result_lists_d:
                                                     [LIST{flag: [0; 24],
                                                           score: [0.; 24],
                                                           pos: [0; 24],};
                                                         10],
                                                 cpm:
                                                     0 as *const cpm_def as
                                                         *mut cpm_def,}; 5],
                               decided: 0,}; 64],
              ID: 0,};
#[no_mangle]
pub static mut entity_manager: ENTITY_MGR =
    ENTITY_MGR{num: 0,
               entity:
                   [ENTITY{num: 0,
                           output_num: 0,
                           mentioned_num: 0,
                           link_entity: 0,
                           first_appearance: 0,
                           salience_score: 0.,
                           salience_mem: 0.,
                           tmp_salience_flag: 0,
                           hypothetical_flag: 0,
                           real_entity: 0,
                           hypothetical_entity: 0,
                           skip_flag: 0,
                           hypothetical_name: [0; 129],
                           mention:
                               [0 as *const MENTION as *mut MENTION; 256],
                           named_entity: [0; 128],
                           name: [0; 129],
                           corefer_id: 0,
                           rep_sen_num: 0,
                           rep_tag_num: 0,}; 4096],};
/* ENTITY管理データ */
#[no_mangle]
pub static mut Revised_para_num: libc::c_int = 0;
#[no_mangle]
pub static mut ErrorComment: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* エラーコメント */
#[no_mangle]
pub static mut WarningComment: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* 警告コメント */
#[no_mangle]
pub static mut PM_Memo: [libc::c_char; 5120] = [0; 5120];
/* パターンマッチ結果 */
#[no_mangle]
pub static mut match_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut path_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut restrict_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Dpnd_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
/* 係り可能性 0, D, P, A */
#[no_mangle]
pub static mut Quote_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
/* 括弧マスク 0, 1 */
#[no_mangle]
pub static mut Mask_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
/* 並列マスク
						    0:係り受け禁止
						    1:係り受けOK
						    2:並列のhead間,
						    3:並列のgapとhead間 */
#[no_mangle]
pub static mut Para_matrix: [[[libc::c_double; 200]; 200]; 32] = [[[0.; 200]; 200]; 32];
#[no_mangle]
pub static mut Chi_pa_matrix: [[libc::c_double; 200]; 200] = [[0.; 200]; 200];
#[no_mangle]
pub static mut Chi_np_start_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Chi_np_end_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Chi_quote_start_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Chi_quote_end_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Chi_dpnd_matrix: [[CHI_DPND; 200]; 200] =
    [[CHI_DPND{direction: [0; 10],
               prob_LtoR: [0.; 10],
               prob_RtoL: [0.; 10],
               prob_pos_LtoR: 0.,
               prob_pos_RtoL: 0.,
               type_0: [[0; 10]; 10],
               occur_pos: 0.,
               prob_dis_comma_LtoR: [0.; 10],
               prob_dis_comma_RtoL: [0.; 10],
               lex_prob_LtoR: [0.; 10],
               lex_prob_RtoL: [0.; 10],
               lex_prob_dis_comma_LtoR: [0.; 10],
               lex_prob_dis_comma_RtoL: [0.; 10],
               left_pos_index: [0; 10],
               right_pos_index: [0; 10],
               count: 0,}; 200]; 200];
#[no_mangle]
pub static mut Chi_pos_matrix: [CHI_POS; 200] =
    [CHI_POS{pos: [0 as *const libc::c_char as *mut libc::c_char; 33],
             prob: [0.; 33],
             prob_pos_index: [0.; 33],
             pos_index: [0; 33],
             pos_max: 0,}; 200];

/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn usage() 
 /*==================================================================*/
 {
    fprintf(stderr,
            b"Usage: knp [-case|dpnd|dpnd-fast|bnst|anaphora] [-ne-crf]\n           [-tree|bnsttree|sexp|tab|bnsttab|mrphtab]\n           [-normal|detail|debug]\n           [-expand] [-semantic-head]\n           [-C host:port] [-S|F] [-N port]\n           [-timeout second] [-r rcfile]\n\x00"
                as *const u8 as *const libc::c_char);
     exit(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn option_proc(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    /* 引数処理 */
    Language = 1 as libc::c_int;
    OptAnalysis = 1 as libc::c_int;
    OptCKY = (0 as libc::c_int == 0) as libc::c_int;
    OptEllipsis = 0 as libc::c_int;
    OptGeneralCF = 0 as libc::c_int;
    OptCorefer = 4 as libc::c_int;
    OptInput = 0 as libc::c_int;
    OptExpress = 65 as libc::c_int;
    OptDisplay = 5 as libc::c_int;
    OptDisplayNE = 1 as libc::c_int;
    OptArticle = 0 as libc::c_int;
    OptExpandP = 0 as libc::c_int;
    OptCFMode = 2 as libc::c_int;
    OptProcessParen = 0 as libc::c_int;
    OptCheck = 0 as libc::c_int;
    OptUseCF = (0 as libc::c_int == 0) as libc::c_int;
    OptUseNCF = (0 as libc::c_int == 0) as libc::c_int;
    OptUseCPNCF = (0 as libc::c_int == 0) as libc::c_int;
    OptMergeCFResult = (0 as libc::c_int == 0) as libc::c_int;
    OptUseRN = 32 as libc::c_int;
    OptUseScase = (0 as libc::c_int == 0) as libc::c_int;
    OptUseSmfix = (0 as libc::c_int == 0) as libc::c_int;
    OptKatakanaNormalize = (0 as libc::c_int == 0) as libc::c_int;
    OptDiscPredMethod = 1 as libc::c_int;
    OptDiscNounMethod = 1 as libc::c_int;
    OptLearn = 0 as libc::c_int;
    OptCaseFlag = 32 as libc::c_int | 1024 as libc::c_int | 512 as libc::c_int | 16 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int | 2048 as libc::c_int | 32768 as libc::c_int | 131072 as libc::c_int | 1048576 as libc::c_int | 65536 as libc::c_int | 8192 as libc::c_int;
    OptDiscFlag = 0 as libc::c_int;
    OptServerFlag = 0 as libc::c_int;
    OptIgnoreChar = '\u{0}' as i32 as libc::c_char;
    OptReadFeature = 0 as libc::c_int;
    OptAddSvmFeatureUtype = 0 as libc::c_int;
    OptAddSvmFeatureDiscourseDepth = 0 as libc::c_int;
    OptAddSvmFeatureObjectRecognition = 0 as libc::c_int;
    OptAddSvmFeatureReferedNum = 0 as libc::c_int;
    OptNoCandidateBehind = 0 as libc::c_int;
    OptCopula = 0 as libc::c_int;
    OptPostProcess = 0 as libc::c_int;
    OptRecoverPerson = 0 as libc::c_int;
    OptNE = 0 as libc::c_int;
    OptNECRF = 0 as libc::c_int;
    OptReadNE = 0 as libc::c_int;
    OptNEcache = 0 as libc::c_int;
    OptNEend = 0 as libc::c_int;
    OptNEdelete = 0 as libc::c_int;
    OptNEcase = 0 as libc::c_int;
    OptNElearn = 0 as libc::c_int;
    OptNEparent = 0 as libc::c_int;
    OptAnaphora = 0 as libc::c_int;
    OptAnaphoraBaseline = 0 as libc::c_int;
    OptTimeoutExit = 0 as libc::c_int;
    OptParaFix = (0 as libc::c_int == 0) as libc::c_int;
    OptParaNoFixFlag = 0 as libc::c_int;
    OptNbest = 0 as libc::c_int;
    OptBeam = 0 as libc::c_int;
    OptSemanticHead = 0 as libc::c_int;
    OptChiGenerative = 0 as libc::c_int;
    OptChiPos = 0 as libc::c_int;
    OptSemanticHead = 0 as libc::c_int;
    OptPosModification = (0 as libc::c_int == 0) as libc::c_int;
    /* オプションの保存 */
    Options =
        malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                         libc::c_ulong).wrapping_mul(argc as libc::c_ulong),
                    b"option_proc\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '-' as i32 {
            let fresh0 = count;
            count = count + 1;
            let ref mut fresh1 = *Options.offset(fresh0 as isize);
            *fresh1 = strdup((*argv.offset(i as isize)).offset(1 as libc::c_int as isize))
        }
        i += 1
    }
    let ref mut fresh2 = *Options.offset(count as isize);
    *fresh2 = 0 as *mut libc::c_char;
    loop  {
        argc -= 1;
        if !(argc > 0 as libc::c_int &&
                 {
                     argv = argv.offset(1);
                     (*(*argv).offset(0 as libc::c_int as isize) as
                          libc::c_int) == '-' as i32
                 }) {
            break ;
        }
        if strcmp(*argv.offset(0 as libc::c_int as isize),
                  b"-case\x00" as *const u8 as *const libc::c_char) == 0 {
            OptAnalysis = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-case2\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptAnalysis = 6 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cfsm\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptCFMode = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-sexp\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptExpress = 8 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-notag\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptExpress = 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-notagtab\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptExpress = 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-bnsttab\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptExpress = 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-bnsttree\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptExpress = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-pa\x00" as *const u8 as *const libc::c_char) == 0
         {
             OptExpress = 32 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-table\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptExpress = 16 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-entity\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptDisplay = 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-article\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptArticle = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-normal\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptDisplay = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-detail\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptDisplay = 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-para-detail\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptDisplay = 6 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-debug\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptDisplay = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-nbest\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptNbest = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-expand\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptExpandP = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-S\x00" as *const u8 as *const libc::c_char) == 0 {
            OptMode = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptUseCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-ncf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptUseNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-process-paren\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptProcessParen = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-suppress-katakana-normalization\x00" as *const u8
                             as *const libc::c_char) == 0 {
            OptKatakanaNormalize = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-tab\x00" as *const u8 as *const libc::c_char) == 0
         {
            if OptDisplay == 5 as libc::c_int {
                /* if it's still default */
                OptDisplay = 1 as libc::c_int
            }
             OptExpress = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-mrphtab\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if OptDisplay == 5 as libc::c_int {
                /* if it's still default */
                OptDisplay = 1 as libc::c_int
            }
            OptExpress = 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-tree\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptExpress = 65 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-mrphtree\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if OptSemanticHead != 0 { usage(); }
            OptExpress = 5 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-dpnd\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptAnalysis = 2 as libc::c_int;
            OptUseCF = 0 as libc::c_int;
            OptUseNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-dpnd-fast\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnalysis = 2 as libc::c_int;
            OptUseCF = 0 as libc::c_int;
            OptUseNCF = 0 as libc::c_int;
            ParaThesaurus = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-dpnd-use-ncf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnalysis = 2 as libc::c_int;
            OptUseCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-fallback-to-dpnd\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(65536 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-filter\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptAnalysis = 5 as libc::c_int;
            OptUseCF = 0 as libc::c_int;
            OptUseNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-bnst\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptAnalysis = 3 as libc::c_int;
            OptUseCF = 0 as libc::c_int;
            OptUseNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-assignf\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptAnalysis = 4 as libc::c_int;
            OptUseCF = 0 as libc::c_int;
            OptUseNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-check\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptCheck = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-simpletab\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptDisplay = 5 as libc::c_int;
            OptExpress = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-probcase\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptAnalysis = 1 as libc::c_int;
            OptCaseFlag |= 16 as libc::c_int;
            SOTO_THRESHOLD = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-probcase\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(16 as libc::c_int);
            SOTO_THRESHOLD = 8 as libc::c_int;
            OptCKY = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-probcase-use-ncf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 4096 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cf-ne\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptGeneralCF |= 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cf-category\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptGeneralCF |= 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-cf-ne\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptGeneralCF &= !(1 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-cf-category\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptGeneralCF &= !(2 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-para\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            /* -no-parafix-synchronize-generate-both -generate-from-eos -beam 5 */
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 4 as libc::c_int;
            OptParaNoFixFlag |= 8 as libc::c_int;
            OptCaseFlag |= 256 as libc::c_int;
            OptBeam = 5 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix-generate-all\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix-generate-both\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 8 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix-generate-sim\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 2 as libc::c_int;
            OptParaNoFixFlag |= 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix-synchronize\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-parafix-synchronize-generate-both\x00" as
                             *const u8 as *const libc::c_char) == 0 {
            OptParaFix = 0 as libc::c_int;
            OptParaNoFixFlag |= 4 as libc::c_int;
            OptParaNoFixFlag |= 8 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cfcase-format-denominator\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 262144 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-analyze-deverbative-noun\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 524288 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-postprocess-pa\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 16777216 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-generalize-agent\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(128 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-generate-from-eos\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 256 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-chi-generative\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptChiGenerative = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-chi-pos\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptChiPos = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cky\x00" as *const u8 as *const libc::c_char) == 0
         {
            OptCKY = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-cky\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptCKY = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-beam\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            OptBeam = atoi(*argv.offset(0 as libc::c_int as isize))
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-language\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                          b"chinese\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                Language = 2 as libc::c_int;
                OptAnalysis = 2 as libc::c_int;
                OptCKY = (0 as libc::c_int == 0) as libc::c_int
            } else if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                                 b"japaense\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                Language = 1 as libc::c_int
            } else { usage(); }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ellipsis\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptEllipsis |= 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-demonstrative\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-iterative\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 131072 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-detail\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |=
                1 as libc::c_int | 2 as libc::c_int | 32 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptAnaphora |=
                1 as libc::c_int | 65536 as libc::c_int | 32 as libc::c_int |
                    262144 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-each-sentence\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |=
                1 as libc::c_int | 65536 as libc::c_int | 32 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-train\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |=
                1 as libc::c_int | 2 as libc::c_int | 16 as libc::c_int |
                    32 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-gs\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |=
                1 as libc::c_int | 2 as libc::c_int | 16 as libc::c_int |
                    64 as libc::c_int | 32 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-entity\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-entity-mention\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 16384 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-pseudo-entity\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 32768 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-author\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 128 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-reader\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 256 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-author-score\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            OptAnaphora |= 512 as libc::c_int;
            author_score = atof(*argv.offset(0 as libc::c_int as isize))
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-reader-score\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int {
                usage();
            } else {
                OptAnaphora |= 1024 as libc::c_int;
                reader_score = atof(*argv.offset(0 as libc::c_int as isize))
            }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-author-mention\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int {
                usage();
            } else {
                OptAnaphora |= 2048 as libc::c_int;
                sscanf(*argv.offset(0 as libc::c_int as isize),
                       b"%d-%d\x00" as *const u8 as *const libc::c_char,
                       &mut author_sen as *mut libc::c_int,
                       &mut author_tag as *mut libc::c_int);
            }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-reader-mention\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int {
                usage();
            } else if *(*argv.offset(0 as libc::c_int as
                                         isize)).offset(0 as libc::c_int as
                                                            isize) as
                          libc::c_int != '-' as i32 {
                OptAnaphora |= 4096 as libc::c_int;
                sscanf(*argv.offset(0 as libc::c_int as isize),
                       b"%d-%d\x00" as *const u8 as *const libc::c_char,
                       &mut reader_sen as *mut libc::c_int,
                       &mut reader_tag as *mut libc::c_int);
            }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-link-author-after\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 8192 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-prob\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |=
                1 as libc::c_int | 2 as libc::c_int | 8 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |= 1 as libc::c_int | 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-normal\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 1 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int;
            OptGeneralCF |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-copula\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptAnaphora |= 1 as libc::c_int | 4 as libc::c_int;
            OptEllipsis |= 1 as libc::c_int | 8 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-read-ne\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptReadNE = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ne-debug\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptDisplayNE = 3 as libc::c_int;
            OptNECRF = 1 as libc::c_int;
            OptNE = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ne-parent\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            //else if (str_eq(argv[0], "-anaphora-normal") || str_eq(argv[0], "-anaphora")) {
            /* 親の情報を用いない */
            OptNE = 1 as libc::c_int;
            OptNECRF = 1 as libc::c_int;
            OptNEparent = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ne-cache\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* キャッシュを使用しない */
            OptNE = 1 as libc::c_int;
            OptNECRF = 1 as libc::c_int;
            OptNEcache = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ne-learn\x00" as *const u8 as *const libc::c_char)
                      == 0 ||
                      strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-ne-train\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            /* NEの学習用featureを出力する */
            OptNE = 1 as libc::c_int;
            OptNECRF = 1 as libc::c_int;
            OptNElearn = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ne-crf\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptNE = 1 as libc::c_int;
            OptNECRF = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ellipsis-dt\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 1 as libc::c_int;
            OptDiscPredMethod = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-ellipsis-dt-only\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 1 as libc::c_int;
            OptDiscPredMethod = 3 as libc::c_int;
            OptDiscFlag |= 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-demonstrative-dt\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 2 as libc::c_int;
            OptDiscPredMethod = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-dt\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 1 as libc::c_int;
            OptEllipsis |= 2 as libc::c_int;
            OptDiscPredMethod = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-noun\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 4 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-noun-only\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 4 as libc::c_int;
            OptEllipsis &= !(1 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-comp-noun\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 4 as libc::c_int;
            OptUseCPNCF = (0 as libc::c_int == 0) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-no-comp-noun\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptUseCPNCF = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptCorefer = 4 as libc::c_int
            /* 文節間の修飾を考慮しない */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer1\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptUseNCF = (0 as libc::c_int == 0) as libc::c_int;
            OptCorefer = 1 as libc::c_int
            /* 名詞格フレームを用いる */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer2\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptCorefer = 2 as libc::c_int
            /* 主辞と同様に扱う */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer3\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptCorefer = 3 as libc::c_int
            /* 主辞以外は修飾されないと考える */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer4\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptCorefer = 4 as libc::c_int
            /* 文節間の修飾を考慮しない */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-corefer5\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            /* 係り受け判定のオプション */
            OptEllipsis |= 8 as libc::c_int;
            OptCorefer = 5 as libc::c_int
            /* 修飾を考慮しない */
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-noun-best\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 4 as libc::c_int;
            OptDiscFlag |= 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-relation-noun-dt\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 4 as libc::c_int;
            OptDiscNounMethod = 3 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-baseline\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 1 as libc::c_int;
            OptEllipsis |= 2 as libc::c_int;
            OptDiscFlag |= 48 as libc::c_int;
            OptAnaphoraBaseline = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-anaphora-baseline-cook\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptEllipsis |= 1 as libc::c_int;
            OptEllipsis |= 2 as libc::c_int;
            OptDiscFlag |= 48 as libc::c_int;
            OptAnaphoraBaseline = 2 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-learn\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            OptLearn = (0 as libc::c_int == 0) as libc::c_int;
            OptDiscFlag |= 1 as libc::c_int;
            PrintFeatures = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-wo-to\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptDiscFlag |= 64 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-candidate-behind\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptNoCandidateBehind = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-i\x00" as *const u8 as *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            OptIgnoreChar = **argv.offset(0 as libc::c_int as isize)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-f\x00" as *const u8 as *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            Infp = fopen(*argv.offset(0 as libc::c_int as isize), b"r\x00" as *const u8 as *const libc::c_char);
            if Infp.is_null() {
                usage();
            }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-use-ex-all\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 8 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-case-score\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 2097152 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-case-oblig\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 4194304 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-case-all-slot\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 8388608 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-ex-all\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            EX_PRINT_NUM = -(1 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-deleted-sm\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            PrintDeletedSM = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-frequency\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            PrintFrequency = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-ex\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            PrintEx = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-print-num\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            PrintNum = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-N\x00" as *const u8 as *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            OptPort =
                atol(*argv.offset(0 as libc::c_int as isize)) as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-C\x00" as *const u8 as *const libc::c_char) == 0 {
            OptMode = 2 as libc::c_int;
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            strcpy(OptHostname.as_mut_ptr(),
                   *argv.offset(0 as libc::c_int as isize));
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-F\x00" as *const u8 as *const libc::c_char) == 0 {
            OptMode = 1 as libc::c_int;
            OptServerFlag = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-timeout\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            ParseTimeout = atoi(*argv.offset(0 as libc::c_int as isize))
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-timeout-exit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptTimeoutExit = 1 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-scode\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                          b"ntt\x00" as *const u8 as *const libc::c_char) == 0
               {
                Thesaurus = 2 as libc::c_int
            } else if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                                 b"bgh\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                Thesaurus = 1 as libc::c_int
            } else { usage(); }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-para-scode\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                          b"ntt\x00" as *const u8 as *const libc::c_char) == 0
               {
                ParaThesaurus = 2 as libc::c_int
            } else if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                                 b"bgh\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                ParaThesaurus = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                                 b"distsim\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                ParaThesaurus = 64 as libc::c_int
            } else if strcasecmp(*argv.offset(0 as libc::c_int as isize),
                                 b"none\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                ParaThesaurus = -(1 as libc::c_int)
            } else { usage(); }
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-rn-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(32 as libc::c_int);
            OptUseRN = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-rn\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptUseRN = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-crn-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            /* daemonにしない場合 */
            /* KNPが生成する主辞代表表記を使わない格フレーム */
            OptCaseFlag &= !(1024 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-cn-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            /* KNPが生成する複合名詞(主辞’)代表表記を使わない格フレーム */
            OptCaseFlag &= !(1024 as libc::c_int);
            OptCaseFlag &= !(512 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-cv-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            /* 用言代表表記(複合動詞を連結)を使わない格フレーム (20110228まで) */
            OptCaseFlag &= !(131072 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-use-id-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            /* IDを使わない格フレーム (20111124まで) */
            OptCaseFlag &= !(1048576 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-fix-cf-search\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(2048 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-clear-cf\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(32768 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-cf-cache\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag &= !(8192 as libc::c_int)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-cf-on-memory\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            OptCaseFlag |= 16384 as libc::c_int;
            OptCaseFlag |= 8192 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-scase\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptUseScase = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-no-smfix\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            OptUseSmfix = 0 as libc::c_int
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-r\x00" as *const u8 as *const libc::c_char) == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            Opt_knprc = *argv.offset(0 as libc::c_int as isize)
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-verbose\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            argv = argv.offset(1);
            argc -= 1;
            if argc < 1 as libc::c_int { usage(); }
            VerboseLevel =
                atoi(*argv.offset(0 as libc::c_int as isize)) as VerboseType
        } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                         b"-v\x00" as *const u8 as *const libc::c_char) == 0 {
            fprintf(stderr,
                    b"%s %s (Revision.%s)\n\x00" as *const u8 as
                        *const libc::c_char,
                    b"knp\x00" as *const u8 as *const libc::c_char,
                    b"5.0\x00" as *const u8 as *const libc::c_char,
                    b"eb641498 on 2021-12-01\x00" as *const u8 as
                        *const libc::c_char);
            exit(0 as libc::c_int);
        } else {
            /* 格解析用オプション */
            if strcmp(*argv.offset(0 as libc::c_int as isize),
                      b"-assign-ga-subj\x00" as *const u8 as
                          *const libc::c_char) == 0 {
                OptCaseFlag |= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-no\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                OptCaseFlag |= 4 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-use-cf-included-soto-words\x00" as *const u8
                                 as *const libc::c_char) == 0 {
                OptCaseFlag &= !(64 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-disc-or-cf\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptDiscFlag |= 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-ellipsis-or-cf\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptEllipsis |= 1 as libc::c_int;
                OptDiscFlag |= 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-noun-th\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                AntecedentDecideThresholdForNoun =
                    atof(*argv.offset(0 as libc::c_int as isize)) as
                        libc::c_float
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-cffix-th\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                CFSimThreshold =
                    atof(*argv.offset(0 as libc::c_int as isize)) as
                        libc::c_float
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-sototh\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                SOTO_THRESHOLD = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-dcost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                DISTANCE_STEP = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-rcost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                RENKAKU_STEP = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-svcost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                STRONG_V_COST = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-atcost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                ADJACENT_TOUTEN_COST =
                    atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-lacost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                LEVELA_COST = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-tscost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                TEIDAI_STEP = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-quacost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                EX_match_qua = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-unknowncost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                EX_match_unknown =
                    atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-sentencecost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                EX_match_sentence =
                    atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-timecost\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                EX_match_tim = atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-score-agent\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                argv = argv.offset(1);
                argc -= 1;
                if argc < 1 as libc::c_int { usage(); }
                EX_match_subject =
                    atoi(*argv.offset(0 as libc::c_int as isize))
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 1 as libc::c_int;
                OptReadFeature |= 4 as libc::c_int;
                OptReadFeature |= 8 as libc::c_int;
                /* 以下コスト調整用 */
                //OptReadFeature |= OPT_CASE_ANALYSIS;
                OptReadFeature |= 32 as libc::c_int;
                OptEllipsis &= !(8 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-corefer\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 8 as libc::c_int;
                //OptReadFeature |= OPT_CASE_ANALYSIS;
                OptReadFeature |= 32 as libc::c_int;
                OptEllipsis &= !(8 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-corefer-auto\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 64 as libc::c_int;
                OptEllipsis &= !(8 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-author-auto\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 128 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-ellipsis\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 1 as libc::c_int;
                OptEllipsis &= !(8 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-noun\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 4 as libc::c_int;
                OptEllipsis &= !(8 as libc::c_int)
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-read-feature-all-case\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptReadFeature |= 256 as libc::c_int
                //解析対象以外のタグ、格も読み込む
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-add-svmfeature-utype\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptAddSvmFeatureUtype = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-add-svmfeature-discourse-depth\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                OptAddSvmFeatureDiscourseDepth = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-add-svmfeature-object-recognition\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                OptAddSvmFeatureObjectRecognition = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-add-svmfeature-referred-num\x00" as *const u8
                                 as *const libc::c_char) == 0 {
                OptAddSvmFeatureReferedNum = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-copula\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptCopula = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-postprocess\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptPostProcess = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-recover-person\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptRecoverPerson = 1 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-semantic-head\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptSemanticHead = 1 as libc::c_int;
                OptExpress = 4 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-disable-pos-modification\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                OptPosModification = 0 as libc::c_int
            } else if !(strcmp(*argv.offset(0 as libc::c_int as isize),
                               b"-def-sentence\x00" as *const u8 as
                                   *const libc::c_char) == 0) {
                if !(strcmp(*argv.offset(0 as libc::c_int as isize),
                            b"-use-smallest-phrase\x00" as *const u8 as
                                *const libc::c_char) == 0) {
                    if !(strcmp(*argv.offset(0 as libc::c_int as isize),
                                b"-disable-emoticon-recognition\x00" as
                                    *const u8 as *const libc::c_char) == 0) {
                        if !(strcmp(*argv.offset(0 as libc::c_int as isize),
                                    b"-disable-segmentation-modification\x00"
                                        as *const u8 as *const libc::c_char)
                                 == 0) {
                            if !(strcmp(*argv.offset(0 as libc::c_int as
                                                         isize),
                                        b"-disable-levelA-clause-segmentation\x00"
                                            as *const u8 as
                                            *const libc::c_char) == 0) {
                                if !(strcmp(*argv.offset(0 as libc::c_int as
                                                             isize),
                                            b"-no-wikipedia\x00" as *const u8
                                                as *const libc::c_char) == 0)
                                   {
                                    usage();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if argc != 0 as libc::c_int { usage(); }
    /* 文脈解析のときは必ず格解析を行う (CASE2)
       解析済みデータのときは read_mrph() で CASE2 にしている
       ただし、共参照解析のみを行う場合は除く */
    if OptEllipsis != 0 && OptEllipsis != 8 as libc::c_int {
        if OptAnalysis != 1 as libc::c_int && OptAnalysis != 6 as libc::c_int
           {
            OptAnalysis = 6 as libc::c_int
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_juman() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    /* rcfile をさがす順
       1. -r で指定されたファイル
       2. $HOME/.knprc
       3. KNP_RC_DEFAULT (compile時)
       → rcfileがなければエラー
    */
    grammar(0 as *mut FILE); /* 文法辞書 */
    katuyou(0 as *mut FILE); /* 活用辞書 */
    i = 1 as libc::c_int;
    while !Class[i as usize][0 as libc::c_int as usize].id.is_null() {
        i += 1
    }
    CLASS_num = i;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_rules() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < CurrentRuleNum {
        /* 同形異義語ルール */
        if (*RULE.offset(i as isize)).type_0 == 3 as libc::c_int {
            read_homo_rule((*RULE.offset(i as isize)).file);
        } else if (*RULE.offset(i as isize)).type_0 == 1 as libc::c_int ||
                      (*RULE.offset(i as isize)).type_0 == 16 as libc::c_int
                      || (*RULE.offset(i as isize)).type_0 == 6 as libc::c_int
                      ||
                      (*RULE.offset(i as isize)).type_0 == 11 as libc::c_int
                      || (*RULE.offset(i as isize)).type_0 == 2 as libc::c_int
                      ||
                      (*RULE.offset(i as isize)).type_0 == 12 as libc::c_int
                      ||
                      (*RULE.offset(i as isize)).type_0 == 13 as libc::c_int
                      ||
                      (*RULE.offset(i as isize)).type_0 == 14 as libc::c_int {
            read_general_rule(RULE.offset(i as isize));
        } else if (*RULE.offset(i as isize)).type_0 == 4 as libc::c_int {
            if Language == 2 as libc::c_int {
                read_dpnd_rule_for_chinese((*RULE.offset(i as isize)).file);
            } else { read_dpnd_rule((*RULE.offset(i as isize)).file); }
        } else if (*RULE.offset(i as isize)).type_0 == 5 as libc::c_int {
            read_koou_rule((*RULE.offset(i as isize)).file);
        } else if (*RULE.offset(i as isize)).type_0 == 10 as libc::c_int {
            read_bnst_rule((*RULE.offset(i as isize)).file,
                           ContRuleArray.as_mut_ptr(), &mut ContRuleSize,
                           256 as libc::c_int);
        }
        i += 1
    };
}
/* 形態素ルール or 文節ルール */
/* 係り受けルール */
/* 呼応表現ルール */
/* 文脈処理のルール */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_all() 
 /*==================================================================*/
 {
    close_cf();
    case_ipal::close_noun_cf();
    close_thesaurus();
    close_scase();
    dic::close_auto_dic();
    nv_mi::close_nv_mi();
    if Language == 2 as libc::c_int {
        close_chi_dpnd_db();
        free_chi_type();
        free_chi_pos();
    }
    if OptEllipsis != 0 { lib_event::close_event(); }
    if OptEllipsis & 8 as libc::c_int != 0 { corefer::close_Synonym_db(); };
}
/*==================================================================*/
unsafe extern "C" fn timeout_function(mut sig: libc::c_int) 
 /*==================================================================*/
 {
    longjmp(timeout.as_mut_ptr(), 1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_timeout_signal() 
 /*==================================================================*/
 {
    let mut set: sigset_t = __sigset_t{__val: [0; 16],};
    if -(1 as libc::c_int) == sigfillset(&mut set) {
        perror(b"sigfullset:\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if -(1 as libc::c_int) == sigprocmask(1 as libc::c_int, &mut set, 0 as *mut sigset_t) {
        perror(b"sigprocmask:\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    signal(14 as libc::c_int,
           Some(timeout_function as unsafe extern "C" fn(_: libc::c_int) -> ()));
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_all() 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    /* 初期化 */
    init_hash(); /* 各種ファイル設定初期化 */
    if OptReadNE != 0 || OptNE != 0 { proper::init_tagposition(); }
    init_configfile(Opt_knprc);
    if OptNECRF == 0 && DBforNE.is_null() { OptNE = 0 as libc::c_int }
    if OptReadNE == 0 && OptNE != 0 {
        proper::init_ne_cache();
        if OptNECRF == 0 { proper::init_db_for_NE(); }
        /* NE用 */
    }
    if OptEllipsis & 8 as libc::c_int != 0 {
        /* 共参照用同義語辞書オープン */
        corefer::init_Synonym_db();
        /* init_entity_cache(); */
    } /* JUMAN関係 */
    if Language == 2 as libc::c_int {
        init_hownet();
        init_chi_dpnd_db();
        init_chi_type();
        init_chi_pos();
    }
    init_juman();
    if OptUseCF != 0 {
        init_cf();
        /* 格フレームオープン */
    } /* 形態素IDマップオープン */
    init_mrph2id(); /* 外の関係ファイルオープン */
    init_soto_txt();
    if OptUseNCF != 0 {
        case_ipal::init_noun_cf();
        /* 名詞格フレームオープン */
    } /* シソーラスオープン */
    init_thesaurus(); /* 表層格辞書オープン */
    init_scase(); /* 自動獲得辞書オープン */
    dic::init_auto_dic(); /* 名詞動詞相互情報量DBオープン */
    nv_mi::init_nv_mi(); /* 分布類似度オープン */
    if ParaThesaurus == 64 as libc::c_int || Thesaurus == 64 as libc::c_int {
        init_distsim();
    }
    if OptEllipsis != 0 {
        if OptDiscPredMethod == 3 as libc::c_int ||
               OptDiscNounMethod == 3 as libc::c_int {
            lib_dt::init_dt();
        }
        lib_event::init_event();
    }
    if OptNE != 0 && OptNElearn == 0 && OptNECRF != 0 { init_crf_for_NE(); }
    /* 形態素, 文節情報の初期化 */
    memset(mrph_data.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<MRPH_DATA>() as
                libc::c_ulong).wrapping_mul(200 as libc::c_int as
                                                libc::c_ulong)); /* これだけは増えていく */
    memset(bnst_data.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<BNST_DATA>() as
                libc::c_ulong).wrapping_mul(200 as libc::c_int as
                                                libc::c_ulong));
    memset(tag_data.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<MRPH_DATA>() as
                libc::c_ulong).wrapping_mul(200 as libc::c_int as
                                                libc::c_ulong));
    current_sentence_data.mrph_data = mrph_data.as_mut_ptr();
    current_sentence_data.bnst_data = bnst_data.as_mut_ptr();
    current_sentence_data.tag_data = tag_data.as_mut_ptr();
    current_sentence_data.para_data = para_data.as_mut_ptr();
    current_sentence_data.para_manager = para_manager.as_mut_ptr();
    current_sentence_data.Sen_num = 0 as libc::c_int;
    current_sentence_data.Mrph_num = 0 as libc::c_int;
    current_sentence_data.Bnst_num = 0 as libc::c_int;
    current_sentence_data.New_Bnst_num = 0 as libc::c_int;
    current_sentence_data.Tag_num = 0 as libc::c_int;
    current_sentence_data.Best_mgr = &mut Best_mgr;
    current_sentence_data.KNPSID = 0 as *mut libc::c_char;
    current_sentence_data.Comment = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh3 =
            (*current_sentence_data.bnst_data.offset(i as isize)).f;
        *fresh3 = 0 as FEATUREptr;
        i += 1
    }
    if OptEllipsis != 0 {
        context::InitContextHash();
        entity_manager.num = 0 as libc::c_int
    }
    set_timeout_signal();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_mrph_features(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if OptEllipsis != 0 {
        /* 中身は保存しておくので */
        i = 0 as libc::c_int;
        while i < (*sp).Mrph_num + (*sp).New_Mrph_num {
            let ref mut fresh4 = (*(*sp).mrph_data.offset(i as isize)).f;
            *fresh4 = 0 as FEATUREptr;
            i += 1
            /* (sp->mrph_data+i)->fはClearSentenceで解放 */
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*sp).Mrph_num {
            clear_feature(&mut (*(*sp).mrph_data.offset(i as isize)).f);
            i += 1
        }
        /* New_Mrphはもともとpointer */
        i = (*sp).Mrph_num;
        while i < (*sp).Mrph_num + (*sp).New_Mrph_num {
            let ref mut fresh5 = (*(*sp).mrph_data.offset(i as isize)).f;
            *fresh5 = 0 as FEATUREptr;
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_bnst_features(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if OptEllipsis != 0 {
        /* 中身は保存しておくので */
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num + (*sp).Max_New_Bnst_num {
            let ref mut fresh6 = (*(*sp).bnst_data.offset(i as isize)).f;
            *fresh6 = 0 as FEATUREptr;
            i += 1
            /* (sp->bnst_data+i)->fはClearSentenceで解放 */
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            clear_feature(&mut (*(*sp).bnst_data.offset(i as isize)).f);
            if Language == 2 as libc::c_int {
                (*(*sp).bnst_data.offset(i as isize)).is_para =
                    -(1 as libc::c_int)
            }
            i += 1
        }
        /* New_Bnstはもともとpointer */
        i = (*sp).Bnst_num;
        while i < (*sp).Bnst_num + (*sp).Max_New_Bnst_num {
            let ref mut fresh7 = (*(*sp).bnst_data.offset(i as isize)).f;
            *fresh7 = 0 as FEATUREptr;
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_bp_features(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if OptEllipsis != 0 {
        /* 中身は保存しておくので */
        i = 0 as libc::c_int;
        while i < (*sp).Tag_num + (*sp).New_Tag_num {
            let ref mut fresh8 = (*(*sp).tag_data.offset(i as isize)).f;
            *fresh8 = 0 as FEATUREptr;
            i += 1
            /* (sp->tag_data+i)->fはClearSentenceで解放 */
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*sp).Tag_num {
            clear_feature(&mut (*(*sp).tag_data.offset(i as isize)).f);
            i += 1
        }
        /* New_Tagはもともとpointer */
        i = (*sp).Tag_num;
        while i < (*sp).Tag_num + (*sp).New_Tag_num {
            let ref mut fresh9 = (*(*sp).tag_data.offset(i as isize)).f;
            *fresh9 = 0 as FEATUREptr;
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_all_features(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    clear_mrph_features(sp);
    clear_bnst_features(sp);
    clear_bp_features(sp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn one_sentence_analysis(mut sp: *mut SENTENCE_DATA,
                                               mut eos_flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut current_block: u64;
    let mut flag: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut relation_error: libc::c_int = 0;
    let mut d_struct_error: libc::c_int = 0;
    /* 形態素列の前処理だけの場合 */
    if OptAnalysis == 5 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    /* 形態素へのFEATURE付与 */
    assign_cfeature(&mut (*(*sp).mrph_data.offset(0 as libc::c_int as
                                                      isize)).f,
                    b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int);
    assign_cfeature(&mut (*(*sp).mrph_data.offset(((*sp).Mrph_num -
                                                       1 as libc::c_int) as
                                                      isize)).f,
                    b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int);
    assign_general_feature((*sp).mrph_data as *mut libc::c_void,
                           (*sp).Mrph_num, 1 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
    /* 正規化代表表記を形態素に付与 
       ※代表表記変更後に行う */
    read_data::assign_canonical_rep_to_mrph(sp);
    /* 固有表現認識を行う */
    if OptReadNE != 0 {
        proper::read_ne(sp);
    } else if OptNE != 0 && OptNEcase == 0 && OptNEparent != 0 {
        proper::ne_analysis(sp);
    }
    /* 形態素を文節にまとめる */
    if OptInput == 0 as libc::c_int {
        if make_bunsetsu(sp) == 0 as libc::c_int {
            clear_bnst_features(sp);
            clear_bp_features(sp);
            (*sp).available = 0 as libc::c_int;
            (*sp).Bnst_num = 0 as libc::c_int;
            (*sp).Tag_num = 0 as libc::c_int;
            ErrorComment =
                strdup(b"Cannot make bunsetsu\x00" as *const u8 as
                           *const libc::c_char);
            return (0 as libc::c_int == 0) as libc::c_int
        }
    } else if make_bunsetsu_pm(sp) == 0 as libc::c_int {
        clear_bnst_features(sp);
        clear_bp_features(sp);
        (*sp).available = 0 as libc::c_int;
        (*sp).Bnst_num = 0 as libc::c_int;
        (*sp).Tag_num = 0 as libc::c_int;
        ErrorComment =
            strdup(b"Cannot make bunsetsu\x00" as *const u8 as
                       *const libc::c_char);
        return (0 as libc::c_int == 0) as libc::c_int
    }
    /* 文節化だけの場合 */
    if OptAnalysis == 3 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    /* 文節への意味情報付与 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        read_data::decide_head_ptr((*sp).bnst_data.offset(i as isize));
        read_data::make_Jiritu_Go(sp, (*sp).bnst_data.offset(i as isize));
        get_bnst_code_all((*sp).bnst_data.offset(i as isize));
        i += 1
    }
    /* 文節へのFEATURE付与 */
    assign_cfeature(&mut (*(*sp).bnst_data.offset(0 as libc::c_int as
                                                      isize)).f,
                    b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int);
    if (*sp).Bnst_num > 0 as libc::c_int {
        assign_cfeature(&mut (*(*sp).bnst_data.offset(((*sp).Bnst_num -
                                                           1 as libc::c_int)
                                                          as isize)).f,
                        b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
    } else {
        assign_cfeature(&mut (*(*sp).bnst_data.offset(0 as libc::c_int as
                                                          isize)).f,
                        b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
    }
    assign_general_feature((*sp).bnst_data as *mut libc::c_void,
                           (*sp).Bnst_num, 2 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
    /* サ変動詞以外の動詞の意味素を引くのは意味がない
       ルール適用前には、featureがないためにチェックできない
       ※ ルール適用後に意味素を引かないのは:
           => 意味素はルールで使うかもしれないので、ルール適用前に与えておく */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                         b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                             *const libc::c_char as
                             *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() {
            (*(*sp).bnst_data.offset(i as
                                         isize)).SM_code[0 as libc::c_int as
                                                             usize] =
                '\u{0}' as i32 as libc::c_char;
            delete_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            b"SM\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char);
        }
        if Language == 2 as libc::c_int {
            copy_feature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                         (*(*(*sp).bnst_data.offset(i as isize)).mrph_ptr).f);
        }
        i += 1
    }
    /* タグ単位作成 (-notag時もscaseを引くために行う) */
    if OptInput == 0 as libc::c_int || OptInput & 2 as libc::c_int != 0 {
        make_tag_units(sp); /* 正規化代表表記を基本句に付与 */
    } else {
        read_data::make_tag_units_pm(sp); /* 正規化代表表記を文節に付与 */
    }
    assign_cc_feature_to_bp(sp);
    assign_cc_feature_to_bnst(sp);
    /* 固有表現認識結果をタグに付与 */
    if OptReadNE != 0 ||
           OptNE != 0 && OptNEcase == 0 && OptNElearn == 0 && OptNEparent != 0
       {
        proper::assign_ne_feature_tag(sp);
    }
    /* 入力した正解情報をチェック */
    if OptReadFeature != 0 { read_data::check_annotation(sp); }
    if OptDisplay == 2 as libc::c_int || OptDisplay == 3 as libc::c_int {
        print_bnst_with_mrphs(sp, 0 as libc::c_int, eos_flag);
    }
    fix_sm_person(sp);
    /* FEATURE付与だけの場合 */
    if OptAnalysis == 4 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    } /* 係り受け規則 */
    assign_dpnd_rule(sp); /* 用言代表表記を基本句に付与 */
    assign_pred_feature_to_bp(sp);
    /* 格フレーム取得 */
    set_frame_num_max(sp);
    if (OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int ||
            OptUseNCF != 0) &&
           (OptAnaphora == 0 || OptAnaphora & 16 as libc::c_int != 0 ||
                OptReadFeature & 1 as libc::c_int == 0 ||
                OptReadFeature & 4 as libc::c_int == 0) {
        set_caseframes(sp);
    }
    /*この時点の文節情報を表示 */
    if OptDisplay == 3 as libc::c_int { check_bnst(sp); }
    /* *************/
    /* 本格的解析 */
    /* *************/
    if Language == 2 as libc::c_int && OptChiGenerative == 0 ||
           Language != 2 as libc::c_int {
        calc_dpnd_matrix(sp);
    } else if Language == 2 as libc::c_int && OptChiGenerative != 0 {
        if OptChiPos == 0 as libc::c_int {
            calc_chi_dpnd_matrix_forProbModel(sp);
        } else { calc_chi_pos_matrix(sp); calc_chi_dpnd_matrix_wpos(sp); }
    }
    /* 依存可能性計算 */
    if OptDisplay == 3 as libc::c_int {
        print_matrix(sp, 1 as libc::c_int, 0 as libc::c_int);
    }
    if Language == 2 as libc::c_int && OptChiGenerative == 0 {
        calc_gigaword_pa_matrix(sp);
        /* get count of gigaword pa for Chinese */
    }
    /* 呼応表現の処理 */
    if ctools::koou(sp) == (0 as libc::c_int == 0) as libc::c_int &&
           OptDisplay == 3 as libc::c_int {
        print_matrix(sp, 1 as libc::c_int, 0 as libc::c_int);
    }
    /* fragment for Chinese */
    if Language == 2 as libc::c_int {
        if OptChiPos == 0 &&
               fragment(sp) == (0 as libc::c_int == 0) as libc::c_int {
            if OptDisplay == 3 as libc::c_int {
                print_matrix(sp, 1 as libc::c_int, 0 as libc::c_int);
            }
            is_frag = 1 as libc::c_int
        }
    }
    /* 鍵括弧の処理 */
    flag = ctools::quote(sp);
    if flag == (0 as libc::c_int == 0) as libc::c_int &&
           OptDisplay == 3 as libc::c_int {
        print_matrix(sp, 3 as libc::c_int, 0 as libc::c_int);
    }
    /* 返り値がCONTINUEとなりquote()が失敗したときは、QUOTE MATRIXがすべて1となり制約なしとなる */
    /* base phrase for Chinese */
    if Language == 2 as libc::c_int && OptChiPos == 0 {
        ctools::base_phrase(sp, is_frag);
        print_matrix(sp, 1 as libc::c_int, 0 as libc::c_int);
    }
    /* 係り受け関係がない場合の弛緩 */
    if Language != 2 as libc::c_int &&
           relax_dpnd_matrix(sp) == (0 as libc::c_int == 0) as libc::c_int &&
           OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Relaxation ... \n\x00" as *const u8 as
                    *const libc::c_char); /* 文節間類似度計算 */
        print_matrix(sp, 1 as libc::c_int,
                     0 as libc::c_int); /* 並列構造推定 */
    }
    if OptInput & 1 as libc::c_int != 0 {
        if OptCheck == (0 as libc::c_int == 0) as libc::c_int {
            dpnd_analysis::call_count_dpnd_candidates(sp, &mut (*(*sp).Best_mgr).dpnd);
        }
        dpnd_info_to_bnst(sp, &mut (*(*sp).Best_mgr).dpnd);
        (*sp).Para_num = 0 as libc::c_int;
        (*sp).Para_M_num = 0 as libc::c_int;
        if check_para_key(sp) != 0 {
            calc_match_matrix(sp);
            detect_all_para_scope(sp);
            assign_para_similarity_feature(sp);
            if OptDisplay == 2 as libc::c_int ||
                   OptDisplay == 3 as libc::c_int ||
                   OptDisplay == 6 as libc::c_int {
                print_matrix(sp, 0 as libc::c_int, 0 as libc::c_int);
            }
        }
        para_recovery(sp);
        para_postprocess(sp);
        assign_para_similarity_feature(sp);
        after_decide_dpnd(sp, eos_flag);
        if OptCheck == (0 as libc::c_int == 0) as libc::c_int {
            check_candidates(sp);
        }
    } else {
        /* ***************/
    /* 並列構造解析 */
    /* ***************/
        init_mask_matrix(sp); /* 文節間類似度計算 */
        (*sp).Para_num = 0 as libc::c_int; /* 並列構造推定 */
        (*sp).Para_M_num = 0 as libc::c_int;
        relation_error = 0 as libc::c_int;
        d_struct_error = 0 as libc::c_int;
        Revised_para_num = -(1 as libc::c_int);
        flag = check_para_key(sp);
        if flag > 0 as libc::c_int {
            init_para_matrix(sp);
            calc_match_matrix(sp);
            detect_all_para_scope(sp);
            loop  {
                assign_para_similarity_feature(sp);
                if OptDisplay == 2 as libc::c_int ||
                       OptDisplay == 3 as libc::c_int ||
                       OptDisplay == 6 as libc::c_int {
                    print_matrix(sp, 0 as libc::c_int, 0 as libc::c_int);
                    /* 並列構造解析成功 */
                    /*
		  print_matrix2ps(sp, PRINT_PARA, 0);
		  exit(0);
		*/
                }
                /* 並列構造間の重なり解析 */
                if detect_para_relation(sp) == 0 as libc::c_int {
                    relation_error += 1
                } else {
                    if OptDisplay == 3 as libc::c_int {
                        print_para_relation(sp);
                    }
                    /* 並列構造内の依存構造チェック */
                    if check_dpnd_in_para(sp) == 0 as libc::c_int {
                        d_struct_error += 1
                    } else {
                        if OptDisplay == 3 as libc::c_int {
                            print_matrix(sp, 2 as libc::c_int,
                                         0 as
                                             libc::c_int); /* 各conjunctのheadを提題の係り先に */
                        }
                        current_block = 5224649233774865596;
                        break ;
                    }
                }
                if !(relation_error <= 3 as libc::c_int &&
                         d_struct_error <= 3 as libc::c_int &&
                         detect_para_scope(sp, Revised_para_num,
                                           (0 as libc::c_int == 0) as
                                               libc::c_int) ==
                             (0 as libc::c_int == 0) as libc::c_int) {
                    current_block = 12223373342341601825;
                    break ;
                }
            }
            match current_block {
                5224649233774865596 => { }
                _ => {
                    WarningComment =
                        strdup(b"Cannot detect consistent CS scopes\x00" as
                                   *const u8 as *const libc::c_char);
                    init_mask_matrix(sp);
                }
            }
        } else { (flag) == -(1 as libc::c_int); }
        /* *******************/
    /* 依存・格構造解析 */
    /* *******************/
        para_postprocess(sp);
        if OptCKY != 0 {
            /* CKY */
            if ctools::cky(sp, (*sp).Best_mgr, eos_flag) == 0 as libc::c_int {
                if Language == 2 as libc::c_int {
                    printf(b"sentence %d cannot be parsed\n\x00" as *const u8
                               as *const libc::c_char, sen_num);
                    return 0 as libc::c_int
                } else {
                    if OptAnalysis == 1 as libc::c_int &&
                           OptCaseFlag & 65536 as libc::c_int != 0 {
                        /* fallback to dpnd */
                        OptAnalysis = 2 as libc::c_int;
                        if ctools::cky(sp, (*sp).Best_mgr, eos_flag) ==
                               0 as libc::c_int {
                            (*sp).available = 0 as libc::c_int;
                            ErrorComment =
                                strdup(b"Cannot detect dependency structure\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                            when_no_dpnd_struct(sp);
                        } else {
                            WarningComment =
                                strdup(b"Fell back to dependency analysis\x00"
                                           as *const u8 as
                                           *const libc::c_char)
                        }
                        OptAnalysis = 1 as libc::c_int
                    } else {
                        (*sp).available = 0 as libc::c_int;
                        ErrorComment =
                            strdup(b"Cannot detect dependency structure\x00"
                                       as *const u8 as *const libc::c_char);
                        when_no_dpnd_struct(sp);
                    }
                }
            } else if OptCheck == (0 as libc::c_int == 0) as libc::c_int {
                check_candidates(sp);
            }
        } else {
            alarm(ParseTimeout as libc::c_uint);
            /* 依存・格構造解析の呼び出し */
            if detect_dpnd_case_struct(sp, eos_flag) == 0 as libc::c_int {
                (*sp).available = 0 as libc::c_int;
                ErrorComment =
                    strdup(b"Cannot detect dependency structure\x00" as
                               *const u8 as *const libc::c_char);
                when_no_dpnd_struct(sp);
                /* 係り受け構造が求まらない場合
					   すべて文節が隣に係ると扱う */
            } else if OptCheck == (0 as libc::c_int == 0) as libc::c_int {
                check_candidates(sp);
            }
            alarm(0 as libc::c_int as libc::c_uint);
        }
    }
    /* 係り受け情報を bnst 構造体に記憶 */
    dpnd_info_to_bnst(sp, &mut (*(*sp).Best_mgr).dpnd);
    para_recovery(sp);
    if OptExpress & 2 as libc::c_int == 0 {
        dpnd_info_to_tag(sp, &mut (*(*sp).Best_mgr).dpnd);
        if OptExpress & 4 as libc::c_int != 0 { dpnd_info_to_mrph(sp); }
    }
    if OptNE != 0 && (OptNEparent == 0 || OptNEcase != 0) {
        /* 固有表現認識に必要なfeatureを与える */
        proper::for_ne_analysis(sp);
        /* 格解析後に固有表現認識を行う */
        proper::ne_analysis(sp);
        if OptNElearn == 0 { proper::assign_ne_feature_tag(sp); }
    }
    /* 構造決定後のルール適用 */
    assign_general_feature((*sp).bnst_data as *mut libc::c_void,
                           (*sp).Bnst_num, 12 as libc::c_int,
                           0 as libc::c_int, 0 as libc::c_int);
    assign_general_feature((*sp).tag_data as *mut libc::c_void, (*sp).Tag_num,
                           13 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
    /* 照応解析に必要なFEATUREの付与 */
    if OptEllipsis != 0 { corefer::assign_anaphor_feature(sp); }
    /* 文節情報の表示 */
    if OptDisplay == 2 as libc::c_int || OptDisplay == 3 as libc::c_int {
        check_bnst(sp); /* メモへの書き込み */
    }
    memo_by_program(sp);
    /* 後処理 */
    if OptEllipsis == 0 && OptPostProcess != 0 { do_postprocess(sp); }
    /* 格解析結果をfeatureへ */
    if (*sp).available != 0 &&
           (OptAnalysis == 1 as libc::c_int ||
                OptAnalysis == 6 as libc::c_int) && OptReadFeature == 0 {
        record_all_case_analisys(sp, 0 as libc::c_int);
    }
    /* ***********/
    /* 文脈解析 */
    /* ***********/
    if OptEllipsis != 0 {
        assign_mrph_num(sp);
        make_dpnd_tree(sp);
        if OptAnaphora != 0 && (*sp).Sen_num > 512 as libc::c_int {
            fprintf(stderr,
                    b";; Sentence buffer (%d) overflowed! ... Initialized context!\n\x00"
                        as *const u8 as *const libc::c_char,
                    512 as libc::c_int);
            clear_context(sp, 0 as libc::c_int);
        } else if OptAnaphora != 0 &&
                      entity_manager.num + 200 as libc::c_int >=
                          4096 as libc::c_int - 1 as libc::c_int {
            /* 1文で生成されるENITYT数はTAG_MAX以下なのでここ以外でEntity bufferが溢れることはない */
            fprintf(stderr,
                    b";; Entity buffer (%d) overflowed! ... Initialized context!\n\x00"
                        as *const u8 as *const libc::c_char,
                    4096 as
                        libc::c_int); /* 文情報を"sentence_data + sp->Sen_num - 1"に保存 */
            clear_context(sp, 0 as libc::c_int); /* 共参照解析 */
        }
        PreserveSentence(sp);
        if OptEllipsis & 8 as libc::c_int != 0 { corefer_analysis(sp); }
        //if (OptAnaphora) anaphora_analysis(sp->Sen_num);
        if OptAnaphora & 65536 as libc::c_int != 0 {
            anaphora::each_sentence_anaphora_analysis(sp);
        }
        if OptEllipsis != 8 as libc::c_int && OptAnaphora == 0 {
            DiscourseAnalysis(sp);
        }
        if OptArticle == 0 && OptPostProcess != 0 {
            /* 後処理 */
            do_postprocess(sp);
        }
    }
    /* entity 情報の feature の作成 */
    if OptDisplay == 4 as libc::c_int { prepare_all_entity(sp); }
    /* 入力した正解情報をクリア */
    if OptReadFeature != 0 {
        i = 0 as libc::c_int;
        while i < (*sp).Tag_num {
            if !(*(*sp).tag_data.offset(i as isize)).c_cpm_ptr.is_null() {
                free((*(*sp).tag_data.offset(i as isize)).c_cpm_ptr as
                         *mut libc::c_void);
            }
            i += 1
        }
    }
    /* 固有表現認識のためのキャッシュ作成 */
    if OptNE != 0 { proper::make_ne_cache(sp); }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_for_one_sentence_analysis(mut sp: *mut SENTENCE_DATA)
 /*==================================================================*/
 {
    /* 格フレームの初期化 */
    if OptCaseFlag & 32768 as libc::c_int != 0 &&
           (OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int
                || OptUseNCF != 0) {
        clear_cf(0 as libc::c_int);
    }
    /* 初期化 */
    if !(*sp).KNPSID.is_null() {
        free((*sp).KNPSID as *mut libc::c_void);
        (*sp).KNPSID = 0 as *mut libc::c_char
    }
    if !(*sp).Comment.is_null() {
        free((*sp).Comment as *mut libc::c_void);
        (*sp).Comment = 0 as *mut libc::c_char
    }
    /* FEATURE の初期化 */
    clear_all_features(sp);
    (*sp).available = 1 as libc::c_int;
    (*sp).frame_num_max = 4096 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn one_line_analysis(mut sp: *mut SENTENCE_DATA,
                                           mut input: *mut FILE)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut paren_num: libc::c_int = 0 as libc::c_int;
    /* initialization */
    init_for_one_sentence_analysis(sp);
    /* get sentence id for Chinese */
    if Language == 2 as libc::c_int {
        sen_num += 1;
        is_frag = 0 as libc::c_int
    }
    (*sp).Sen_num += 1;
    /* 形態素の読み込み */
    flag = read_mrph(sp, input);
    if flag == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if flag == 0 as libc::c_int {
        /* EOSしかない空の文 */
        clear_all_features(sp);
        (*sp).available = 0 as libc::c_int;
        (*sp).Mrph_num = 0 as libc::c_int;
        (*sp).Bnst_num = 0 as libc::c_int;
        (*sp).Tag_num = 0 as libc::c_int;
        ErrorComment =
            strdup(b"Cannot make mrph\x00" as *const u8 as
                       *const libc::c_char)
    } else {
        /* 形態素読み込み成功 */
        /* 形態素列の前処理 */
        preprocess_mrph(sp);
        /* 括弧を処理して文として分割する場合 */
        if OptProcessParen != 0 {
            paren_num = process_input_paren(sp, &mut paren_sentence_data)
        }
        /* 一文構文・格解析 */
        flag =
            one_sentence_analysis(sp,
                                  (if paren_num != 0 {
                                       0 as libc::c_int
                                   } else {
                                       1 as libc::c_int
                                   })); /* 解析失敗時には文の数を増やさない */
        if flag == 0 as libc::c_int {
            (*sp).Sen_num -= 1;
            return 0 as libc::c_int
        }
    }
    /* ***********/
    /* 結果表示 */
    /* ***********/
    if OptAnaphora == 0 || OptAnaphora & 65536 as libc::c_int != 0 ||
           OptExpress == 16 as libc::c_int {
        print_all_result(sp,
                         if paren_num != 0 {
                             0 as libc::c_int
                         } else { 1 as libc::c_int });
        /* 括弧含む文: EOP(0); 通常文: EOS(1) */
    }
    /* 括弧文の解析 */
    if paren_num != 0 {
        i = 0 as libc::c_int;
        while i < paren_num {
            /* initialization */
            init_for_one_sentence_analysis(sp);
            /* last -> EOS(1); otherwise -> EOP(0) */
            prepare_paren_sentence(sp,
                                   paren_sentence_data.offset(i as
                                                                  isize)); /* spに設定 */
            free((*paren_sentence_data.offset(i as isize)).mrph_data as
                     *mut libc::c_void);
            flag =
                one_sentence_analysis(sp,
                                      (if i != paren_num - 1 as libc::c_int {
                                           0 as libc::c_int
                                       } else { 1 as libc::c_int }));
            if !(flag == 0 as libc::c_int) {
                print_all_result(sp,
                                 if i != paren_num - 1 as libc::c_int {
                                     0 as libc::c_int
                                 } else { 1 as libc::c_int });
            }
            i += 1
        }
        free(paren_sentence_data as *mut libc::c_void);
    }
    return flag;
}
/* 解析失敗 */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_knp_main() 
 /*==================================================================*/
 {
    /* 格解析の準備 */
    init_case_analysis_cpm(&mut current_sentence_data);
    init_case_analysis_cmm();
    /* 意味クラスを用いた照応解析を行う場合クラスごとの出現確率を読み込む */
    if OptAnaphora != 0 && OptGeneralCF & 4 as libc::c_int != 0 {
        case_ipal::init_class_prob();
    }
    /* ルール読み込み
       Server Mode において、読み込むルールの変更がありえるので、ここで行う */
    read_rules();
    if OptExpress == 16 as libc::c_int {
        fprintf(Outfp,
                b"%%%% title=KNP\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\n\x00"
                    as *const u8 as *const libc::c_char);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn knp_main() {
    let mut i: libc::c_int = 0;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut flag: libc::c_int = 0;
    // let mut Jumanfp: *mut FILE = 0 as *mut FILE;
    // let mut sp_new: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut sp: *mut SENTENCE_DATA = &mut current_sentence_data;
    loop  {
        /* Server Mode の場合 前回の出力が成功してない場合は 
	   ERROR とはく Server/Client モードの場合は,出力の同期をこれで行う */
        if success == 0 && OptMode == 1 as libc::c_int {
            fprintf(Outfp,
                    b"EOS ERROR\n\x00" as *const u8 as *const libc::c_char);
            fflush(Outfp);
        }
        /* OK 成功 */
        if _setjmp(timeout.as_mut_ptr()) != 0 {
            /* *******************/
	/* 前の解析の後始末 */
	/* *******************/
            /* タイムアウト時 */
            /* timeoutした文をstderrに出力 */
            fprintf(stderr,
                    b";; Parse timeout.\n;; %s (\x00" as *const u8 as
                        *const libc::c_char, (*sp).KNPSID);
            i = 0 as libc::c_int;
            while i < (*sp).Mrph_num {
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*sp).mrph_data.offset(i as
                                                     isize)).Goi2.as_mut_ptr());
                i += 1
            }
            fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
            ErrorComment =
                strdup(b"Parse timeout\x00" as *const u8 as
                           *const libc::c_char);
            (*sp).available = 0 as libc::c_int;
            when_no_dpnd_struct(sp);
            dpnd_info_to_bnst(sp, &mut (*(*sp).Best_mgr).dpnd);
            if OptExpress & 2 as libc::c_int == 0 {
                dpnd_info_to_tag(sp, &mut (*(*sp).Best_mgr).dpnd);
                if OptExpress & 4 as libc::c_int != 0 {
                    dpnd_info_to_mrph(sp);
                }
            }
            if OptEllipsis == 0 {
                if OptPostProcess != 0 {
                    /* 後処理 */
                    do_postprocess(sp);
                }
                if OptAnaphora == 0 || OptExpress != 0 as libc::c_int {
                    print_result(sp, 1 as libc::c_int, 1 as libc::c_int);
                }
            } else if OptEllipsis != 8 as libc::c_int && OptAnaphora == 0 {
                PreserveCPM(PreserveSentence(sp), sp);
            }
            fflush(Outfp);
            /* OptTimeoutExit == 1 または格・省略解析のときは終わる */
            if OptTimeoutExit != 0 ||
                   (OptAnalysis == 1 as libc::c_int ||
                        OptAnalysis == 6 as libc::c_int) {
                exit(100 as libc::c_int);
            }
            set_timeout_signal();
        } else {
            /* *************/
	/* メイン解析 */
	/* *************/
            success = 0 as libc::c_int;
            flag = one_line_analysis(sp, Infp);
            if flag == -(1 as libc::c_int) { break ; }
            if flag == 0 as libc::c_int { continue ; }
            success = 1 as libc::c_int
        }
    }
    if OptAnaphora != 0 && OptAnaphora & 65536 as libc::c_int == 0 {
        all_sentence_anaphora_analysis(sp);
        if OptExpress == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*sp).Sen_num - 1 as libc::c_int {
                print_result(sentence_data.as_mut_ptr().offset(i as isize),
                             0 as libc::c_int, 1 as libc::c_int);
                i += 1
            }
        }
    }
    if OptArticle != 0 && OptEllipsis != 0 {
        i = 0 as libc::c_int;
        while i < (*sp).Sen_num - 1 as libc::c_int {
            if OptPostProcess != 0 {
                /* 後処理 */
                do_postprocess(sentence_data.as_mut_ptr().offset(i as isize));
            }
            print_result(sentence_data.as_mut_ptr().offset(i as isize),
                         1 as libc::c_int, 1 as libc::c_int);
            i += 1
        }
    };
}
/* シグナル処理 */
unsafe extern "C" fn sig_child() {
    let mut status: libc::c_int = 0;
    while waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int) >
              0 as libc::c_int {
    }
    signal(17 as libc::c_int,
           ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn() -> (), unsafe extern "C" fn() -> ()>(sig_child))));
}
unsafe extern "C" fn sig_term() {
    shutdown(sfd, 2 as libc::c_int);
    shutdown(fd, 2 as libc::c_int);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn clean_and_exit(mut status: libc::c_int) { exit(status); }
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn server_mode() 
 /*==================================================================*/
 {
    /* サーバモード */
    let mut i: libc::c_int = 0;
    let mut run_count: libc::c_int = 0 as libc::c_int;
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut pidfile: *mut FILE = 0 as *mut FILE;
    let mut ent_pw: *mut passwd = 0 as *mut passwd;
    if OptServerFlag != 1 as libc::c_int {
        /* parent */
        i = fork();
        if i > 0 as libc::c_int {
            return
        } else {
            if i == -(1 as libc::c_int) {
                fprintf(stderr,
                        b";; unable to fork new process\n\x00" as *const u8 as
                            *const libc::c_char);
                return
            }
        }
        /* child */
    }
    signal(1 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    signal(13 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    signal(15 as libc::c_int,
           ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                    ->
                                                                                        (),
                                                                                unsafe extern "C" fn()
                                                                                    ->
                                                                                        ()>(sig_term))));
    signal(2 as libc::c_int,
           ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                    ->
                                                                                        (),
                                                                                unsafe extern "C" fn()
                                                                                    ->
                                                                                        ()>(sig_term))));
    signal(3 as libc::c_int,
           ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                    ->
                                                                                        (),
                                                                                unsafe extern "C" fn()
                                                                                    ->
                                                                                        ()>(sig_term))));
    signal(17 as libc::c_int,
           ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                    ->
                                                                                        (),
                                                                                unsafe extern "C" fn()
                                                                                    ->
                                                                                        ()>(sig_child))));
    sfd =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if sfd < 0 as libc::c_int {
        fprintf(stderr,
                b";; socket error: %s\n\x00" as *const u8 as
                    *const libc::c_char, strerror(*__errno_location()));
        clean_and_exit(1 as libc::c_int);
    }
    memset(&mut sin as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    sin.sin_port = __bswap_16(OptPort as __uint16_t);
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    /* bind */
    if bind(sfd, &mut sin as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) < 0 as libc::c_int {
        fprintf(stderr,
                b";; bind error\n\x00" as *const u8 as *const libc::c_char);
        close(sfd);
        clean_and_exit(1 as libc::c_int);
    }
    /* listen */
    if listen(sfd, 4096 as libc::c_int) < 0 as libc::c_int {
        fprintf(stderr,
                b";; listen error\n\x00" as *const u8 as *const libc::c_char);
        close(sfd);
        clean_and_exit(1 as libc::c_int);
    }
    /* make pid file */
    umask(0o22 as libc::c_int as __mode_t);
    pidfile =
        fopen(b"/var/run/knp.pid\x00" as *const u8 as *const libc::c_char,
              b"w\x00" as *const u8 as *const libc::c_char);
    if pidfile.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b";; can\'t write pidfile: /var/run/knp.pid\n\x00" as
                      *const u8 as *const libc::c_char, stderr);
        }
    } else {
        fprintf(pidfile, b"%d\n\x00" as *const u8 as *const libc::c_char,
                getpid());
        fclose(pidfile);
    }
    umask(0 as libc::c_int as __mode_t);
    /* change uid and gid for security */
    ent_pw = getpwnam(b"nobody\x00" as *const u8 as *const libc::c_char);
    if !ent_pw.is_null() {
        let mut dummy: gid_t = 0;
        let mut gp: *mut group = 0 as *mut group;
        /* remove all supplementary groups */
        setgroups(0 as libc::c_int as size_t, &mut dummy);
        gp = getgrgid((*ent_pw).pw_gid);
        if !gp.is_null() { setgid((*gp).gr_gid); }
        /* finally drop root */
        setuid((*ent_pw).pw_uid);
    }
    loop 
         /* accept loop */
         {
        let mut pid: libc::c_int = 0 as libc::c_int;
        let mut from: sockaddr_in =
            sockaddr_in{sin_family: 0,
                        sin_port: 0,
                        sin_addr: in_addr{s_addr: 0,},
                        sin_zero: [0; 8],};
        let mut from_len: libc::c_int =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                libc::c_int;
        if OptDisplay == 3 as libc::c_int {
            fprintf(stderr,
                    b";; accepting ... \x00" as *const u8 as
                        *const libc::c_char);
        }
        fd =
            accept(sfd, &mut from as *mut sockaddr_in as *mut sockaddr,
                   &mut from_len as *mut libc::c_int as *mut socklen_t);
        if fd < 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int { continue ; }
            fprintf(stderr,
                    b";; accept error\n\x00" as *const u8 as
                        *const libc::c_char);
            close(sfd);
            clean_and_exit(1 as libc::c_int);
        }
        if OptDisplay == 3 as libc::c_int {
            fprintf(stderr,
                    b"done.\n\x00" as *const u8 as *const libc::c_char);
        }
        pid = fork();
        if pid < 0 as libc::c_int {
            fprintf(stderr,
                    b";; fork error\n\x00" as *const u8 as
                        *const libc::c_char);
            sleep(1 as libc::c_int as libc::c_uint);
        } else {
            /* 子供 */
            if pid == 0 as libc::c_int {
                let mut buf: [libc::c_char; 1024] = [0; 1024];
                Infp =
                    fdopen(fd, b"r\x00" as *const u8 as *const libc::c_char);
                Outfp =
                    fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
                if Infp.is_null() || Outfp.is_null() {
                    fprintf(stderr,
                            b";; fdopen error\n\x00" as *const u8 as
                                *const libc::c_char);
                    close(sfd);
                    clean_and_exit(1 as libc::c_int);
                }
                /* 挨拶 */
                fprintf(Outfp,
                        b"200 Running KNP Server\n\x00" as *const u8 as
                            *const libc::c_char);
                fflush(Outfp);
                /* オプション解析 */
                while !fgets(buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                 libc::c_ulong as libc::c_int, Infp).is_null()
                      {
                    /* QUIT */
                    if strncasecmp(buf.as_mut_ptr(),
                                   b"QUIT\x00" as *const u8 as
                                       *const libc::c_char,
                                   4 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        fprintf(Outfp,
                                b"200 OK Quit\n\x00" as *const u8 as
                                    *const libc::c_char);
                        fflush(Outfp);
                        break ;
                    } else if strncasecmp(buf.as_mut_ptr(),
                                          b"RC\x00" as *const u8 as
                                              *const libc::c_char,
                                          2 as libc::c_int as libc::c_ulong)
                                  == 0 as libc::c_int {
                        server_read_rc(Infp);
                        fprintf(Outfp,
                                b"200 OK\n\x00" as *const u8 as
                                    *const libc::c_char);
                        fflush(Outfp);
                    } else if strncasecmp(buf.as_mut_ptr(),
                                          b"RUN\x00" as *const u8 as
                                              *const libc::c_char,
                                          3 as libc::c_int as libc::c_ulong)
                                  == 0 as libc::c_int {
                        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                        if !strstr(buf.as_mut_ptr(),
                                   b"-case\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptAnalysis = 1 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-case2\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptAnalysis = 6 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-dpnd\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptAnalysis = 2 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-bnst\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptAnalysis = 3 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-ellipsis\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptEllipsis |= 1 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-tree\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptExpress = 1 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-sexp\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptExpress = 8 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-tab\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptExpress = 0 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-normal\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptDisplay = 1 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-detail\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptDisplay = 2 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-debug\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptDisplay = 3 as libc::c_int
                        }
                        if !strstr(buf.as_mut_ptr(),
                                   b"-expand\x00" as *const u8 as
                                       *const libc::c_char).is_null() {
                            OptExpandP =
                                (0 as libc::c_int == 0) as libc::c_int
                        }
                        p =
                            strstr(buf.as_mut_ptr(),
                                   b"-i\x00" as *const u8 as
                                       *const libc::c_char);
                        if !p.is_null() {
                            p = p.offset(3 as libc::c_int as isize);
                            while *p as libc::c_int != '\u{0}' as i32 &&
                                      (*p as libc::c_int == ' ' as i32 ||
                                           *p as libc::c_int == '\t' as i32) {
                                p = p.offset(1)
                            }
                            if *p as libc::c_int != '\u{0}' as i32 {
                                OptIgnoreChar = *p
                            }
                        }
                        fprintf(Outfp,
                                b"200 OK option=[Analysis=%d Express=%d Display=%d]\n\x00"
                                    as *const u8 as *const libc::c_char,
                                OptAnalysis, OptExpress, OptDisplay);
                        fflush(Outfp);
                        /* RUN */
		/* Option 解析は strstr なんかでかなりいいかげん 
		   つまり間違ったオプションはエラーにならない */
                        /* 解析 */
                        init_knp_main();
                        knp_main();
                        run_count += 1;
                        break ;
                    } else {
                        fprintf(Outfp,
                                b"500 What?\n\x00" as *const u8 as
                                    *const libc::c_char);
                        fflush(Outfp);
                    }
                }
                /* 後処理 */
                shutdown(fd, 2 as libc::c_int);
                fclose(Infp);
                fclose(Outfp);
                close(sfd);
                close(fd);
                clean_and_exit(0 as libc::c_int);
            }
            /* 親 */
            close(fd);
        }
    };
}
/* 文字列を送って、ステータスコードを返す */
unsafe extern "C" fn send_string(mut fi: *mut FILE, mut fo: *mut FILE,
                                 mut str: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if !str.is_null() {
        fwrite(str as *const libc::c_void,
               ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
               strlen(str), fo);
        fflush(fo);
    }
    while !fgets(buf.as_mut_ptr(),
                 (::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong) as
                     libc::c_int, fi).is_null() {
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        if !(len >= 3 as libc::c_int &&
                 buf[3 as libc::c_int as usize] as libc::c_int == ' ' as i32)
           {
            continue ;
        }
        buf[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        result =
            atoi(&mut *buf.as_mut_ptr().offset(0 as libc::c_int as isize));
        break ;
    }
    return result;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn client_mode() 
 /*==================================================================*/
 {
    /* クライアントモード (TCP/IPで接続するだけ) */
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut fd_0: libc::c_int = 0;
    let mut fi: *mut FILE = 0 as *mut FILE;
    let mut fo: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut option: [libc::c_char; 1024] = [0; 1024];
    let mut port: libc::c_int = 31000 as libc::c_int;
    let mut strnum: libc::c_int = 0 as libc::c_int;
    /* host:port という形の場合 */
    p = strchr(OptHostname.as_mut_ptr(), ':' as i32);
    if !p.is_null() {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = '\u{0}' as i32 as libc::c_char;
        port = atoi(p)
    }
    /* つなげる準備 */
    hp = gethostbyname(OptHostname.as_mut_ptr());
    if hp.is_null() {
        fprintf(stderr,
                b";; host unkown\n\x00" as *const u8 as *const libc::c_char);
        clean_and_exit(1 as libc::c_int);
    }
    fd_0 =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if fd_0 < 0 as libc::c_int {
        fprintf(stderr,
                b";; socket error\n\x00" as *const u8 as *const libc::c_char);
        clean_and_exit(1 as libc::c_int);
    }
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin.sin_port = __bswap_16(port as __uint16_t);
    sin.sin_addr =
        *(*(*hp).h_addr_list.offset(0 as libc::c_int as isize) as *mut in_addr);
    if connect(
        fd_0,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t
    ) < 0 as libc::c_int {
        fprintf(stderr, b";; connect error\n\x00" as *const u8 as *const libc::c_char);
        clean_and_exit(1 as libc::c_int);
    }
    fi = fdopen(fd_0, b"r\x00" as *const u8 as *const libc::c_char);
    if fi.is_null() || {
        fo = fdopen(fd_0, b"w\x00" as *const u8 as *const libc::c_char);
        fo.is_null()
    } {
        close(fd_0);
        fprintf(stderr, b";; fd error\n\x00" as *const u8 as *const libc::c_char);
        clean_and_exit(1 as libc::c_int);
    }
    /* 挨拶 */
    if send_string(fi, fo, 0 as *mut libc::c_char) != 200 as libc::c_int {
        fprintf(stderr, b";; greet error\n\x00" as *const u8 as *const libc::c_char);
        clean_and_exit(1 as libc::c_int);
    }
    /* オプション解析 (いいかげん) */
    option[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    match OptAnalysis {
        1 => {
            strcat(option.as_mut_ptr(), b" -case\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            strcat(option.as_mut_ptr(), b" -dpnd\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            strcat(option.as_mut_ptr(), b" -bnst\x00" as *const u8 as *const libc::c_char);
        }
        _ => { }
    }
    match OptExpress {
        1 => {
            strcat(option.as_mut_ptr(), b" -tree\x00" as *const u8 as *const libc::c_char);
        }
        8 => {
            strcat(option.as_mut_ptr(), b" -sexp\x00" as *const u8 as *const libc::c_char);
        }
        0 => {
            strcat(option.as_mut_ptr(), b" -tab\x00" as *const u8 as *const libc::c_char);
        }
        _ => { }
    }
    match OptDisplay {
        1 => {
            strcat(option.as_mut_ptr(),
                   b" -normal\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            strcat(option.as_mut_ptr(),
                   b" -detail\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            strcat(option.as_mut_ptr(),
                   b" -debug\x00" as *const u8 as *const libc::c_char);
        }
        _ => { }
    }
    if OptExpandP != 0 {
        strcat(option.as_mut_ptr(),
               b" -expand\x00" as *const u8 as *const libc::c_char);
    }
    if OptIgnoreChar == 0 {
        sprintf(buf.as_mut_ptr(),
                b" -i %c\x00" as *const u8 as *const libc::c_char,
                OptIgnoreChar as libc::c_int);
        strcat(option.as_mut_ptr(), buf.as_mut_ptr());
    }
    /* これから動作 */
    sprintf(buf.as_mut_ptr(),
            b"RUN%s\n\x00" as *const u8 as *const libc::c_char,
            option.as_mut_ptr());
    if send_string(fi, fo, buf.as_mut_ptr()) != 200 as libc::c_int {
        fprintf(stderr,
                b";; argument error OK? [%s]\n\x00" as *const u8 as
                    *const libc::c_char, option.as_mut_ptr());
        close(fd_0);
        clean_and_exit(1 as libc::c_int);
    }
    /* LOOP */
    strnum = 0 as libc::c_int;
    while !fgets(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong as libc::c_int, stdin).is_null() {
        if strncmp(buf.as_mut_ptr(),
                   b"EOS\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            if strnum != 0 as libc::c_int {
                fwrite(buf.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                       strlen(buf.as_mut_ptr()), fo);
                fflush(fo);
                strnum = 0 as libc::c_int;
                while !fgets(buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 8192]>() as
                                 libc::c_ulong as libc::c_int, fi).is_null() {
                    fwrite(buf.as_mut_ptr() as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_char>() as
                               libc::c_ulong, strlen(buf.as_mut_ptr()),
                           stdout);
                    fflush(stdout);
                    if strncmp(buf.as_mut_ptr(),
                               b"EOS\x00" as *const u8 as *const libc::c_char,
                               3 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        break ;
                    }
                }
            }
        } else {
            fwrite(buf.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                   strlen(buf.as_mut_ptr()), fo);
            fflush(fo);
            strnum += 1
        }
    }
    /* 終了処理 */
    fprintf(fo, b"\n%c\nQUIT\n\x00" as *const u8 as *const libc::c_char,
            0xb as libc::c_int);
    fclose(fo);
    fclose(fi);
    close(fd_0);
    clean_and_exit(0 as libc::c_int);
}
/*==================================================================*/
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int  {
    option_proc(argc, argv);
    /* モードによって処理を分岐 */
    if OptMode == 0 as libc::c_int {
        init_all();
        init_knp_main();
        knp_main();
        close_all();
    } else if OptMode == 1 as libc::c_int {
        init_all();
        server_mode();
        close_all();
    } else if OptMode == 2 as libc::c_int { client_mode(); }
    if Infp != stdin { fclose(Infp); }
    exit(0 as libc::c_int);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
