use std::str;
use libc::{ssize_t, tm};

use crate::structs::{__jmp_buf_tag, BnstRule, cdb, cdb_make, CF_MATCH_MGR, CHI_DPND, CHI_ROOT, DPND, DpndRule, FEATURE_PATTERN, GeneralRuleType, HomoRule, KoouRule, MRPH_DATA, MrphRule, QUOTE_DATA, REGEXPBNSTS, REGEXPMRPH, REGEXPMRPHS, stat};
use crate::types::{__compar_fn_t, __gid_t, __off_t, __pid_t, __uid_t, __uint32_t, BNST_DATA, CASE_FRAME, CELL, CF_PRED_MGR, CFLIST, CKY, CLASS, DBM_FILE, ELLIPSIS_MGR, ENTITY_MGR, FEATURE, FILE, RuleVector, SENTENCE_DATA, size_t, SMLIST, TAG_DATA, THESAURUS_FILE, time_t, TYPE, z_streamp};
use crate::{__mode_t, __sighandler_t, __uint16_t, CHI_POS, group, hostent, LIST, passwd, sigset_t, sockaddr, socklen_t, TOTAL_MGR, VerboseType};
use crate::juman::types::FORM;

pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum cdb_rl {}
pub enum internal_state {}

extern "C" {
    #[no_mangle]
    pub static mut SMExist: libc::c_int;
    #[no_mangle]
    pub static mut SMP2SMGExist: libc::c_int;
    #[no_mangle]
    pub static mut used_auto_dic_features: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut used_auto_dic_features_num: libc::c_int;
    #[no_mangle]
    pub static mut CRFFileNE: *mut libc::c_char;
    #[no_mangle]
    pub static mut DTFile: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut SynonymFile: *mut libc::c_char;
    #[no_mangle]
    pub static mut DistSimFile: *mut libc::c_char;
    #[no_mangle]
    pub static mut DistSimDB: *mut libc::c_char;
    #[no_mangle]
    pub static mut DistSimWordList: *mut libc::c_char;
    #[no_mangle]
    pub static mut DiscAddedCases: [libc::c_int; 0];
    #[no_mangle]
    pub static mut LocationLimit: [libc::c_int; 0];
    #[no_mangle]
    pub static mut PrevSentenceLimit: libc::c_int;
    #[no_mangle]
    pub static mut LocationOrder: [[libc::c_int; 21]; 0];
    #[no_mangle]
    pub static mut AntecedentDecideThresholdPredGeneral: libc::c_float;
    #[no_mangle]
    pub static mut AntecedentDecideThresholdForGa: libc::c_float;
    #[no_mangle]
    pub static mut AntecedentDecideThresholdForNi: libc::c_float;
    #[no_mangle]
    pub static mut Jumangram_Dirname: [libc::c_char; 0];
    #[no_mangle]
    pub static mut OptMergeCFResult: libc::c_int;
    #[no_mangle]
    pub static mut OptDiscPredMethod: libc::c_int;
    #[no_mangle]
    pub static mut OptDiscNounMethod: libc::c_int;
    #[no_mangle]
    pub static mut OptLearn: libc::c_int;
    #[no_mangle]
    pub static mut OptDiscFlag: libc::c_int;
    #[no_mangle]
    pub static mut OptAddSvmFeatureUtype: libc::c_int;
    #[no_mangle]
    pub static mut OptAddSvmFeatureDiscourseDepth: libc::c_int;
    #[no_mangle]
    pub static mut OptAddSvmFeatureObjectRecognition: libc::c_int;
    #[no_mangle]
    pub static mut OptAddSvmFeatureReferedNum: libc::c_int;
    #[no_mangle]
    pub static mut OptNoCandidateBehind: libc::c_int;
    #[no_mangle]
    pub static mut OptAnaphoraBaseline: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_score: [libc::c_int; 0];
    #[no_mangle]
    pub static mut OptCorefer: libc::c_int;
    #[no_mangle]
    pub static mut OptSemanticHead: libc::c_int;
    #[no_mangle]
    pub static mut DpndRuleArray: [DpndRule; 0];
    #[no_mangle]
    pub static mut CurDpndRuleSize: libc::c_int;
    #[no_mangle]
    pub static mut CurKoouRuleSize: libc::c_int;
    #[no_mangle]
    pub static mut KoouRuleArray: [KoouRule; 0];
    #[no_mangle]
    pub static mut WarningComment: *mut libc::c_char;
    #[no_mangle]
    pub static mut path_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut OptArticle: libc::c_int;
    #[no_mangle]
    pub static mut OptCopula: libc::c_int;
    #[no_mangle]
    pub static mut OptRecoverPerson: libc::c_int;
    #[no_mangle]
    pub static mut CLASS_num: libc::c_int;
    #[no_mangle]
    pub static mut PrintNum: libc::c_int;
    #[no_mangle]
    pub static mut quote_data: QUOTE_DATA;
    #[no_mangle]
    pub static mut LineNo: libc::c_int;
    #[no_mangle]
    pub static mut LineNoForError: libc::c_int;
    #[no_mangle]
    pub static mut Case_name: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut OptUseRN: libc::c_int;
    #[no_mangle]
    pub static mut Type: [TYPE; 128];
    #[no_mangle]
    pub static mut OptExpandP: libc::c_int;
    #[no_mangle]
    pub static mut Revised_para_num: libc::c_int;
    #[no_mangle]
    pub static mut restrict_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut D_found_array: [libc::c_int; 200];
    #[no_mangle]
    pub static mut OptDisplayNE: libc::c_int;
    #[no_mangle]
    pub static mut OptNECRF: libc::c_int;
    #[no_mangle]
    pub static mut OptNEcache: libc::c_int;
    #[no_mangle]
    pub static mut OptNEend: libc::c_int;
    #[no_mangle]
    pub static mut OptNEcase: libc::c_int;
    #[no_mangle]
    pub static mut OptNEparent: libc::c_int;
    #[no_mangle]
    pub static mut current_sentence_data: SENTENCE_DATA;
    #[no_mangle]
    pub static mut OptUseCF: libc::c_int;
    #[no_mangle]
    pub static mut OptUseNCF: libc::c_int;
    #[no_mangle]
    pub static mut OptUseCPNCF: libc::c_int;
    #[no_mangle]
    pub static mut VerboseLevel: VerboseType;
    #[no_mangle]
    pub static mut smlist: [SMLIST; 0];
    #[no_mangle]
    pub static mut Dpnd_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Quote_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Mask_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut ETAG_name: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut EX_match_exact: libc::c_int;
    #[no_mangle]
    pub static mut OptCKY: libc::c_int;
    #[no_mangle]
    pub static mut OptCFMode: libc::c_int;
    #[no_mangle]
    pub static mut DICT: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut Work_mgr: TOTAL_MGR;
    #[no_mangle]
    pub static mut Possibility: libc::c_int;
    #[no_mangle]
    pub static mut MAX_Case_frame_num: libc::c_int;
    #[no_mangle]
    pub static mut Para_matrix: [[[libc::c_double; 200]; 200]; 0];
    #[no_mangle]
    pub static mut Chi_dpnd_matrix: [[CHI_DPND; 200]; 0];
    #[no_mangle]
    pub static mut Chi_pos_matrix: [CHI_POS; 0];
    #[no_mangle]
    pub static mut Chi_root_prob_matrix: [CHI_ROOT; 0];
    #[no_mangle]
    pub static mut Chi_word_pos: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut left_arg: [libc::c_int; 0];
    #[no_mangle]
    pub static mut right_arg: [libc::c_int; 0];
    #[no_mangle]
    pub static mut Chi_pa_matrix: [[libc::c_double; 200]; 0];
    #[no_mangle]
    pub static mut Chi_np_start_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Chi_np_end_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Chi_quote_start_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Chi_quote_end_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub static mut Chi_root: libc::c_int;
    #[no_mangle]
    pub static mut OptPostProcess: libc::c_int;
    #[no_mangle]
    pub static mut OptParaFix: libc::c_int;
    #[no_mangle]
    pub static mut OptParaNoFixFlag: libc::c_int;
    #[no_mangle]
    pub static mut OptNbest: libc::c_int;
    #[no_mangle]
    pub static mut OptBeam: libc::c_int;
    #[no_mangle]
    pub static mut OptChiGenerative: libc::c_int;
    #[no_mangle]
    pub static mut CFSimExist: libc::c_int;
    #[no_mangle]
    pub static mut entity_manager: ENTITY_MGR;
    #[no_mangle]
    pub static mut corefer_id: libc::c_int;
    #[no_mangle]
    pub static mut OptGeneralCF: libc::c_int;
    #[no_mangle]
    pub static mut OptCaseFlag: libc::c_int;
    #[no_mangle]
    pub static mut author_score: libc::c_double;
    #[no_mangle]
    pub static mut reader_score: libc::c_double;
    #[no_mangle]
    pub static mut author_sen: libc::c_int;
    #[no_mangle]
    pub static mut author_tag: libc::c_int;
    #[no_mangle]
    pub static mut reader_sen: libc::c_int;
    #[no_mangle]
    pub static mut reader_tag: libc::c_int;
    #[no_mangle]
    pub static mut SM2CODEExist: libc::c_int;
    #[no_mangle]
    pub static mut Options: *mut *mut libc::c_char;
    #[no_mangle]
    pub static mut OptExpress: libc::c_int;
    #[no_mangle]
    pub static mut OptPosModification: libc::c_int;
    #[no_mangle]
    pub static mut matched_ptr: *mut libc::c_void;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub static mut stdout: *mut FILE;
    #[no_mangle]
    pub static mut stdin: *mut FILE;
    #[no_mangle]
    pub static mut AntecedentDecideThresholdForNoun: libc::c_float;
    #[no_mangle]
    pub static mut Thesaurus: libc::c_int;
    #[no_mangle]
    pub static mut ParaThesaurus: libc::c_int;
    #[no_mangle]
    pub static mut OptUseScase: libc::c_int;
    #[no_mangle]
    pub static mut OptUseSmfix: libc::c_int;
    #[no_mangle]
    pub static mut Class: [[CLASS; 129]; 129];
    #[no_mangle]
    pub static mut ContRuleArray: [BnstRule; 0];
    #[no_mangle]
    pub static mut ContRuleSize: libc::c_int;
    #[no_mangle]
    pub static mut DBforNE: *mut libc::c_char;
    #[no_mangle]
    pub static mut EX_PRINT_NUM: libc::c_int;
    #[no_mangle]
    pub static mut PrintFrequency: libc::c_int;
    #[no_mangle]
    pub static mut PrintDeletedSM: libc::c_int;
    #[no_mangle]
    pub static mut PrintFeatures: libc::c_int;
    #[no_mangle]
    pub static mut PrintEx: libc::c_int;
    #[no_mangle]
    pub static mut CFSimThreshold: libc::c_float;
    #[no_mangle]
    pub static mut RULE: *mut RuleVector;
    #[no_mangle]
    pub static mut CurrentRuleNum: libc::c_int;
    #[no_mangle]
    pub static mut SOTO_THRESHOLD: libc::c_int;
    #[no_mangle]
    pub static mut DISTANCE_STEP: libc::c_int;
    #[no_mangle]
    pub static mut RENKAKU_STEP: libc::c_int;
    #[no_mangle]
    pub static mut STRONG_V_COST: libc::c_int;
    #[no_mangle]
    pub static mut ADJACENT_TOUTEN_COST: libc::c_int;
    #[no_mangle]
    pub static mut LEVELA_COST: libc::c_int;
    #[no_mangle]
    pub static mut TEIDAI_STEP: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_qua: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_unknown: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_sentence: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_tim: libc::c_int;
    #[no_mangle]
    pub static mut EX_match_subject: libc::c_int;
    #[no_mangle]
    pub static mut ErrorComment: *mut libc::c_char;
    #[no_mangle]
    pub static mut PM_Memo: [libc::c_char; 0];
    #[no_mangle]
    pub static mut sentence_data: [SENTENCE_DATA; 0];
    #[no_mangle]
    pub static mut Chi_word_type: [*mut libc::c_char; 0];
    #[no_mangle]
    pub static mut OptAnalysis: libc::c_int;
    #[no_mangle]
    pub static mut OptEllipsis: libc::c_int;
    #[no_mangle]
    pub static mut OptInput: libc::c_int;
    #[no_mangle]
    pub static mut OptDisplay: libc::c_int;
    #[no_mangle]
    pub static mut OptKatakanaNormalize: libc::c_int;
    #[no_mangle]
    pub static mut OptIgnoreChar: libc::c_char;
    #[no_mangle]
    pub static mut OptReadFeature: libc::c_int;
    #[no_mangle]
    pub static mut OptNE: libc::c_int;
    #[no_mangle]
    pub static mut Language: libc::c_int;
    #[no_mangle]
    pub static mut OptNElearn: libc::c_int;
    #[no_mangle]
    pub static mut OptAnaphora: libc::c_int;
    #[no_mangle]
    pub static mut OptChiPos: libc::c_int;
    #[no_mangle]
    pub static mut Form: [[FORM; 128]; 128];
    #[no_mangle]
    pub static mut HomoRuleArray: [HomoRule; 0];
    #[no_mangle]
    pub static mut CurHomoRuleSize: libc::c_int;
    #[no_mangle]
    pub static mut GeneralRuleNum: libc::c_int;
    #[no_mangle]
    pub static mut GeneralRuleArray: *mut GeneralRuleType;
    #[no_mangle]
    pub static mut Infp: *mut FILE;
    #[no_mangle]
    pub static mut Outfp: *mut FILE;
    #[no_mangle]
    pub static mut OptMode: libc::c_int;
    #[no_mangle]
    pub static mut THESAURUS: [THESAURUS_FILE; 0];
    #[no_mangle]
    pub static mut match_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    pub fn rand() -> libc::c_int;
    #[no_mangle]
    pub fn srand(__seed: libc::c_uint);
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    pub fn getpid() -> __pid_t;
    #[no_mangle]
    pub fn setuid(__uid: __uid_t) -> libc::c_int;
    #[no_mangle]
    pub fn setgid(__gid: __gid_t) -> libc::c_int;
    #[no_mangle]
    pub fn fork() -> __pid_t;
    #[no_mangle]
    pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    pub fn perror(__s: *const libc::c_char);
    #[no_mangle]
    pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    pub fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    #[no_mangle]
    pub fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    pub fn sigprocmask(__how: libc::c_int, __set: *const sigset_t, __oset: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    pub fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    pub fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    pub fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    pub fn init_crf_for_NE() -> libc::c_int;
    #[no_mangle]
    pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    pub fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn accept(__fd: libc::c_int, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    #[no_mangle]
    pub fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    pub fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    #[no_mangle]
    pub fn getgrgid(__gid: __gid_t) -> *mut group;
    #[no_mangle]
    pub fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    #[no_mangle]
    pub fn grammar(fp_out: *mut FILE);
    #[no_mangle]
    pub fn katuyou(fp: *mut FILE);
    #[no_mangle]
    pub fn assign_mrph_num(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn all_sentence_anaphora_analysis(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn clear_context(sp: *mut SENTENCE_DATA, init_flag: libc::c_int);
    #[no_mangle]
    pub fn calc_match_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn init_case_analysis_cmm();
    #[no_mangle]
    pub fn record_all_case_analisys(sp: *mut SENTENCE_DATA, temp_assign_flag: libc::c_int);
    #[no_mangle]
    pub fn init_cf();
    #[no_mangle]
    pub fn init_mrph2id();
    #[no_mangle]
    pub fn init_soto_txt();
    #[no_mangle]
    pub fn init_case_analysis_cpm(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn close_cf();
    #[no_mangle]
    pub fn set_frame_num_max(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn set_caseframes(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn clear_cf(flag: libc::c_int);
    #[no_mangle]
    pub fn assign_pred_feature_to_bp(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn init_configfile(opfile: *mut libc::c_char);
    #[no_mangle]
    pub fn server_read_rc(fp: *mut FILE);
    #[no_mangle]
    pub fn PreserveCPM(sp_new: *mut SENTENCE_DATA, sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn PreserveSentence(sp: *mut SENTENCE_DATA) -> *mut SENTENCE_DATA;
    #[no_mangle]
    pub fn DiscourseAnalysis(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn init_chi_dpnd_db();
    #[no_mangle]
    pub fn close_chi_dpnd_db();
    #[no_mangle]
    pub fn init_chi_type();
    #[no_mangle]
    pub fn free_chi_type();
    #[no_mangle]
    pub fn init_chi_pos();
    #[no_mangle]
    pub fn free_chi_pos();
    #[no_mangle]
    pub fn dpnd_info_to_bnst(sp: *mut SENTENCE_DATA, dp: *mut DPND);
    #[no_mangle]
    pub fn dpnd_info_to_tag(sp: *mut SENTENCE_DATA, dp: *mut DPND);
    #[no_mangle]
    pub fn dpnd_info_to_mrph(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn after_decide_dpnd(sp: *mut SENTENCE_DATA, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn calc_dpnd_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn calc_chi_dpnd_matrix_forProbModel(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn calc_chi_dpnd_matrix_wpos(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn calc_chi_pos_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn relax_dpnd_matrix(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn para_postprocess(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn detect_dpnd_case_struct(sp: *mut SENTENCE_DATA, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn when_no_dpnd_struct(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn check_candidates(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn memo_by_program(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn calc_gigaword_pa_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn check_feature(fp: *mut FEATURE, fname: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn assign_cfeature(fpp: *mut *mut FEATURE, fname: *mut libc::c_char, temp_assign_flag: libc::c_int);
    #[no_mangle]
    pub fn clear_feature(fpp: *mut *mut FEATURE);
    #[no_mangle]
    pub fn delete_cfeature(fpp: *mut *mut FEATURE, type_0: *mut libc::c_char);
    #[no_mangle]
    pub fn copy_feature(dst_fpp: *mut *mut FEATURE, src_fp: *mut FEATURE);
    #[no_mangle]
    pub fn koou(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn print_matrix(sp: *mut SENTENCE_DATA, type_0: libc::c_int, L_B: libc::c_int);
    #[no_mangle]
    pub fn print_result(sp: *mut SENTENCE_DATA, case_print_flag: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    pub fn check_bnst(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn print_para_relation(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn assign_para_similarity_feature(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn prepare_all_entity(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn do_postprocess(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn print_all_result(sp: *mut SENTENCE_DATA, eos_flag: libc::c_int);
    #[no_mangle]
    pub fn print_bnst_with_mrphs(sp: *mut SENTENCE_DATA, have_dpnd_flag: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    pub fn init_scase();
    #[no_mangle]
    pub fn close_scase();
    #[no_mangle]
    pub fn fix_sm_person(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn init_mask_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn init_para_matrix(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn check_dpnd_in_para(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn para_recovery(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn check_para_key(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn detect_all_para_scope(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn detect_para_scope(sp: *mut SENTENCE_DATA, para_num: libc::c_int, restrict_p: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn detect_para_relation(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn quote(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn process_input_paren(sp: *mut SENTENCE_DATA, paren_spp: *mut *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn prepare_paren_sentence(sp: *mut SENTENCE_DATA, paren_sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn read_mrph(sp: *mut SENTENCE_DATA, fp: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn assign_general_feature(data: *mut libc::c_void, size: libc::c_int, flag: libc::c_int, also_assign_flag: libc::c_int, temp_assign_flag: libc::c_int);
    #[no_mangle]
    pub fn make_bunsetsu(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn make_bunsetsu_pm(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn assign_dpnd_rule(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn make_tag_units(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn assign_cc_feature_to_bp(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn assign_cc_feature_to_bnst(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn preprocess_mrph(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn read_homo_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    pub fn read_general_rule(rule: *mut RuleVector);
    #[no_mangle]
    pub fn read_dpnd_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    pub fn read_dpnd_rule_for_chinese(file_name: *mut libc::c_char);
    #[no_mangle]
    pub fn read_koou_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    pub fn read_bnst_rule(file_name: *mut libc::c_char, rp: *mut BnstRule, count: *mut libc::c_int, max: libc::c_int);
    #[no_mangle]
    pub fn init_thesaurus();
    #[no_mangle]
    pub fn close_thesaurus();
    #[no_mangle]
    pub fn get_bnst_code_all(ptr: *mut BNST_DATA);
    #[no_mangle]
    pub fn malloc_data(size: size_t, comment: *mut libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    pub fn init_hash();
    #[no_mangle]
    pub fn make_dpnd_tree(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn cky(sp: *mut SENTENCE_DATA, Best_mgr_0: *mut TOTAL_MGR, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn base_phrase(sp: *mut SENTENCE_DATA, is_frag_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn fragment(sp: *mut SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn init_hownet();
    #[no_mangle]
    pub fn init_distsim();
    #[no_mangle]
    pub fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub fn pp_kstr_to_code(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn pp_code_to_kstr(num: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn ClearSentences(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn sm2feature(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn regexpmrph_match(ptr1: *mut REGEXPMRPH, ptr2: *mut MRPH_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn delete_cfeature_from_mrphs(m_ptr: *mut MRPH_DATA, length: libc::c_int, type_0: *mut libc::c_char);
    #[no_mangle]
    pub fn delete_alt_feature(fpp: *mut *mut FEATURE);
    #[no_mangle]
    pub fn append_feature(fpp: *mut *mut FEATURE, afp: *mut FEATURE);
    #[no_mangle]
    pub fn assign_feature(fpp1: *mut *mut FEATURE, fpp2: *mut *mut FEATURE, ptr: *mut libc::c_void, offset: libc::c_int, length: libc::c_int, temp_assign_flag: libc::c_int);
    #[no_mangle]
    pub fn regexpbnstrule_match(r_ptr: *mut BnstRule, d_ptr: *mut BNST_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn regexptagrule_match(r_ptr: *mut BnstRule, d_ptr: *mut TAG_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn regexpmrphrule_match(r_ptr: *mut MrphRule, d_ptr: *mut MRPH_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub(crate) fn string_length(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn realloc_data(ptr: *mut libc::c_void, size: size_t, comment: *mut libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    pub fn car(cell: *mut CELL) -> *mut CELL;
    #[no_mangle]
    pub fn cdr(cell: *mut CELL) -> *mut CELL;
    #[no_mangle]
    pub fn subordinate_level_comp(ptr1: *mut BNST_DATA, ptr2: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn subordinate_level_check(cp: *mut libc::c_char, f: *mut FEATURE) -> libc::c_int;
    #[no_mangle]
    pub fn make_fukugoji_id(b_ptr: *mut BNST_DATA) -> *mut libc::c_char;
    #[no_mangle]
    pub fn make_fukugoji_case_string(b_ptr: *mut TAG_DATA) -> *mut libc::c_char;
    #[no_mangle]
    pub fn set_pred_voice(b_ptr: *mut BNST_DATA);
    #[no_mangle]
    pub fn bgh_match_check(pat: *mut libc::c_char, codes: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn _sm_match_score(cpp: *mut libc::c_char, cpd: *mut libc::c_char, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn get_scase_code(ptr: *mut BNST_DATA);
    #[no_mangle]
    pub fn sm_all_match(c: *mut libc::c_char, target: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn sm2code(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn sm_match_check(pat: *mut libc::c_char, codes: *mut libc::c_char, expand: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn assign_sm(bp: *mut BNST_DATA, cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn check_auto_dic(m_ptr: *mut MRPH_DATA, assign_pos: libc::c_int, m_length: libc::c_int, rule_value: *mut libc::c_char, temp_assign_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn change_mrph(m_ptr: *mut MRPH_DATA, f: *mut FEATURE);
    #[no_mangle]
    pub fn check_nv_mi_parent_and_children(v_ptr: *mut TAG_DATA, rank_threshold: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn change_one_mrph_rep(m_ptr: *mut MRPH_DATA, modify_feature_flag: libc::c_int, suffix_char: libc::c_char);
    #[no_mangle]
    pub fn case2num(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub fn get_bnst_head_canonical_rep(ptr: *mut BNST_DATA, compound_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn make_pred_string(t_ptr: *mut TAG_DATA, m_ptr: *mut MRPH_DATA, orig_form: *mut libc::c_char, use_rep_flag: libc::c_int, cf_type: libc::c_int, cpncf_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn make_pred_string_from_mrph(t_ptr: *mut TAG_DATA, m_ptr: *mut MRPH_DATA, orig_form: *mut libc::c_char, use_rep_flag: libc::c_int, cf_type: libc::c_int, cpncf_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn MatchPP(n: libc::c_int, pp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn make_print_string(bp: *mut TAG_DATA, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn get_case_probability(as2: libc::c_int, cfp: *mut CASE_FRAME, aflag: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    pub fn get_case_function_probability_for_pred(as1: libc::c_int, cfd: *mut CASE_FRAME, as2: libc::c_int, cfp: *mut CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn get_ex_probability_with_para(as1: libc::c_int, cfd: *mut CASE_FRAME, as2: libc::c_int, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn get_ex_probability(as1: libc::c_int, cfd: *mut CASE_FRAME, dp: *mut TAG_DATA, as2: libc::c_int, cfp: *mut CASE_FRAME, sm_flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn get_ex_ne_probability(cp: *mut libc::c_char, as2: libc::c_int, cfp: *mut CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn _get_ex_probability_internal(key: *mut libc::c_char, as2: libc::c_int, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn get_cf_probability_for_pred(cfd: *mut CASE_FRAME, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn get_general_probability(key1: *mut libc::c_char, key2: *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn get_key_probability(tag_ptr: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn get_class_probability(key: *mut libc::c_char, as2: libc::c_int, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn check_str_type(ucp: *mut libc::c_uchar, allowed_type: libc::c_int, length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn get_cfs_similarity(cf1: *mut libc::c_char, cf2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    pub fn CheckCF(key: *mut libc::c_char) -> *mut CFLIST;
    #[no_mangle]
    pub fn get_pred_id(cfid: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn make_data_cframe(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    pub fn init_case_frame(cf: *mut CASE_FRAME);
    #[no_mangle]
    pub fn db_get(db: DBM_FILE, buf: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn list2feature_pattern(f: *mut FEATURE_PATTERN, cell: *mut CELL);
    #[no_mangle]
    pub fn feature_pattern_match(fr: *mut FEATURE_PATTERN, fd: *mut FEATURE, p1: *mut libc::c_void, p2: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    pub fn cons(car_0: *mut libc::c_void, cdr_0: *mut libc::c_void) -> *mut CELL;
    #[no_mangle]
    pub fn length(list: *mut CELL) -> libc::c_int;
    #[no_mangle]
    pub fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn print_kakari(sp: *mut SENTENCE_DATA, type_0: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    pub fn get_case_prob_wpos(sp: *mut SENTENCE_DATA, head: libc::c_int, left_arg_num: libc::c_int, right_arg_num: libc::c_int, pos_index_pre: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn get_case_prob(sp: *mut SENTENCE_DATA, head: libc::c_int, left_arg_num: libc::c_int, right_arg_num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn print_crrspnd(cpm_ptr: *mut CF_PRED_MGR, cmm_ptr: *mut CF_MATCH_MGR);
    #[no_mangle]
    pub fn print_data_cframe(cpm_ptr: *mut CF_PRED_MGR, cmm_ptr: *mut CF_MATCH_MGR);
    #[no_mangle]
    pub fn get_noun_co_num_probability(gp: *mut TAG_DATA, num: libc::c_int, para_cky_ptr: *mut CKY) -> libc::c_double;
    #[no_mangle]
    pub fn get_noun_co_ex_probability(dp: *mut TAG_DATA, gp: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn get_para_ex_probability(para_key: *mut libc::c_char, score: libc::c_double, dp: *mut TAG_DATA, gp: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn get_para_exist_probability(para_key: *mut libc::c_char, score: libc::c_double, flag: libc::c_int, dp: *mut TAG_DATA, gp: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn calc_adv_modifying_num_probability(t_ptr: *mut TAG_DATA, cfp: *mut CASE_FRAME, num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn calc_adv_modifying_probability(gp: *mut TAG_DATA, cfp: *mut CASE_FRAME, dp: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn calc_vp_modifying_num_probability(t_ptr: *mut TAG_DATA, cfp: *mut CASE_FRAME, num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn calc_vp_modifying_probability(gp: *mut TAG_DATA, g_cf: *mut CASE_FRAME, dp: *mut TAG_DATA, d_cf: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn _make_data_cframe_ex(cpm_ptr: *mut CF_PRED_MGR, b_ptr: *mut TAG_DATA);
    #[no_mangle]
    pub fn _make_data_cframe_sm(cpm_ptr: *mut CF_PRED_MGR, b_ptr: *mut TAG_DATA);
    #[no_mangle]
    pub fn _make_data_cframe_pp(cpm_ptr: *mut CF_PRED_MGR, b_ptr: *mut TAG_DATA, flag: libc::c_int) -> *mut TAG_DATA;
    #[no_mangle]
    pub fn make_data_cframe_child(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR, child_ptr: *mut TAG_DATA, child_num: libc::c_int, closest_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn find_best_cf(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR, closest: libc::c_int, decide: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    pub fn get_closest_case_component(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    pub fn noun_lexical_disambiguation_by_case_analysis(cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    pub fn verb_lexical_disambiguation_by_case_analysis(cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    pub fn copy_cpm(dst: *mut CF_PRED_MGR, src: *mut CF_PRED_MGR, flag: libc::c_int);
    #[no_mangle]
    pub fn record_case_analysis(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR, em_ptr: *mut ELLIPSIS_MGR, temp_assign_flag: libc::c_int);
    #[no_mangle]
    pub fn pp_hstr_to_code(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub(crate) fn get_mrph_rep_from_f(m_ptr: *mut MRPH_DATA, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn db_read_open(filename: *mut libc::c_char) -> DBM_FILE;
    #[no_mangle]
    pub(crate) fn check_dict_filename(file: *mut libc::c_char, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn similarity_chinese(str1: *mut libc::c_char, str2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    pub fn general_code_match(th: *mut THESAURUS_FILE, c1: *mut libc::c_char, c2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    pub fn ntt_code_match(c1: *mut libc::c_char, c2: *mut libc::c_char, flag: libc::c_int) -> libc::c_float;
    #[no_mangle]
    pub fn bgh_code_match(c1: *mut libc::c_char, c2: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn calc_distsim_from_bnst(ptr1: *mut BNST_DATA, ptr2: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn malloc_db_buf(size: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn open_dict(dic_num: libc::c_int, dic_name: *mut libc::c_char, exist: *mut libc::c_int) -> DBM_FILE;
    #[no_mangle]
    pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    pub fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub fn count_pat_element(cfp: *mut CASE_FRAME, list2: *mut LIST) -> libc::c_int;
    #[no_mangle]
    pub fn case_frame_match(cpm_ptr: *mut CF_PRED_MGR, cmm_ptr: *mut CF_MATCH_MGR, flag: libc::c_int, closest: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    pub fn compare_dpnd(sp: *mut SENTENCE_DATA, new_mgr: *mut TOTAL_MGR, best_mgr: *mut TOTAL_MGR) -> libc::c_int;
    #[no_mangle]
    pub fn cf_match_element(d: *mut libc::c_char, target: *mut libc::c_char, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn count_assigned_adjacent_element(cfp: *mut CASE_FRAME, list2: *mut LIST) -> libc::c_int;
    #[no_mangle]
    pub fn _calc_similarity_sm_cf(exd: *mut libc::c_char, expand: libc::c_int, unmatch_word: *mut libc::c_char, cfp: *mut CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    pub fn cf_match_exactly(word: *mut libc::c_char, word_len: libc::c_int, ex_list: *mut *mut libc::c_char, ex_num: libc::c_int, pos: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn init_mgr_cf(tmp: *mut TOTAL_MGR);
    #[no_mangle]
    pub fn _make_ipal_cframe_pp(c_ptr: *mut CASE_FRAME, cp: *mut libc::c_uchar, num: libc::c_int, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn check_cf_case(cfp: *mut CASE_FRAME, pp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn get_mrph_rep_length(rep_strt: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn make_mrph_rn(m_ptr: *mut MRPH_DATA) -> *mut libc::c_char;
    #[no_mangle]
    pub fn get_mrph_rep(m_ptr: *mut MRPH_DATA) -> *mut libc::c_char;
    #[no_mangle]
    pub fn assign_feature_alt_mrph(fpp: *mut *mut FEATURE, m_ptr: *mut MRPH_DATA);
    #[no_mangle]
    pub fn delete_existing_features(m_ptr: *mut MRPH_DATA);
    #[no_mangle]
    pub fn copy_mrph(dst: *mut MRPH_DATA, src: *mut MRPH_DATA, imi2feature_flag: libc::c_int);
    #[no_mangle]
    pub fn get_str_code_with_len(cp: *mut libc::c_char, len: libc::c_int, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn hash(key: *mut libc::c_uchar, keylen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    pub fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    pub fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub(crate) fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    pub fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
    #[no_mangle]
    pub fn realloc_cmm();
    #[no_mangle]
    pub fn CF_MatchPP(c: libc::c_int, cf: *mut CASE_FRAME) -> libc::c_int;
    #[no_mangle]
    pub fn get_dist_from_work_mgr(bp: *mut BNST_DATA, hp: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn code2sm(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn get_str_code(cp: *mut libc::c_uchar, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn sms_match(cpp: *mut libc::c_char, cpd: *mut libc::c_char, expand: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn check_category(fp: *mut FEATURE, fname: *mut libc::c_char, strict_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn get_crf_prob(i: libc::c_int, j: libc::c_int, prob: *mut libc::c_double);
    #[no_mangle]
    pub fn crf_parse();
    #[no_mangle]
    pub fn clear_crf();
    #[no_mangle]
    pub fn crf_add(line: *mut libc::c_char);
    #[no_mangle]
    pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn _check_para_d_struct(sp: *mut SENTENCE_DATA, str: libc::c_int, end: libc::c_int, extend_p: libc::c_int, limit: libc::c_int, s_p: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn rep2id(rep: *mut libc::c_char, rep_len: libc::c_int, buffer: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn _get_bgh(cp: *mut libc::c_char, arg: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn bgh_code_match_for_case(cp1: *mut libc::c_char, cp2: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn init_bgh();
    #[no_mangle]
    pub fn close_bgh();
    #[no_mangle]
    pub fn _get_ntt(cp: *mut libc::c_char, arg: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn init_ntt();
    #[no_mangle]
    pub fn close_ntt();
    #[no_mangle]
    pub fn delete_specified_sm(sm: *mut libc::c_char, del: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn check_noun_sm(key: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn katakana2hiragana(cp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    #[no_mangle]
    pub fn calc_distsim(word1: *mut libc::c_char, word2: *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn comp_sm(cpp: *mut libc::c_char, cpd: *mut libc::c_char, start: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn code_depth(cp: *mut libc::c_char, code_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn get_most_similar_code(exd: *mut libc::c_char, exp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn s_feof(fp: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn s_read(fp: *mut FILE) -> *mut CELL;
    #[no_mangle]
    pub fn store_regexpbnsts(bspp: *mut *mut REGEXPBNSTS, cell: *mut CELL);
    #[no_mangle]
    pub fn store_regexpmrphs(mspp: *mut *mut REGEXPMRPHS, cell: *mut CELL);
    #[no_mangle]
    pub fn check_rule_filename(file: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn list2feature(cp: *mut CELL, fpp: *mut *mut FEATURE);
    #[no_mangle]
    pub fn usage();
    #[no_mangle]
    pub fn revise_para_rel(sp: *mut SENTENCE_DATA, pre: libc::c_int, pos: libc::c_int);
    #[no_mangle]
    pub fn print_bnst(ptr: *mut BNST_DATA, cp: *mut libc::c_char);
    #[no_mangle]
    pub fn revise_para_kakari(sp: *mut SENTENCE_DATA, num: libc::c_int, array: *mut libc::c_int);
    #[no_mangle]
    pub fn feature_AND_match(fp: *mut FEATURE, fd: *mut FEATURE, p1: *mut libc::c_void, p2: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    pub fn string2feature_pattern(f: *mut FEATURE_PATTERN, cp: *mut libc::c_char);
    #[no_mangle]
    pub fn db_write_open(filename: *mut libc::c_char) -> DBM_FILE;
    #[no_mangle]
    pub fn db_put(db: DBM_FILE, buf: *mut libc::c_char, value: *mut libc::c_char, Separator: *mut libc::c_char, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn db_close(db: DBM_FILE);
    #[no_mangle]
    pub fn hiragana2katakana(cp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    #[no_mangle]
    pub fn init_bnst_tree_property(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn strftime(__s: *mut libc::c_char, __maxsize: size_t, __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    pub fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    pub fn print_pa_structure(sp: *mut SENTENCE_DATA, eos_flag: libc::c_int);
    #[no_mangle]
    pub fn tag_bnst_postprocess(sp: *mut SENTENCE_DATA, flag: libc::c_int);
    #[no_mangle]
    pub fn undo_tag_bnst_postprocess(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn print_feature(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    pub fn print_feature2(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    pub fn print_some_feature(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn _regexpbnst_match(r_ptr: *mut REGEXPMRPHS, b_ptr: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn call_case_analysis(sp: *mut SENTENCE_DATA, dpnd: DPND, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn InitCPMcache();
    #[no_mangle]
    pub fn ClearCPMcache();
    #[no_mangle]
    pub fn check_examples(cp: *mut libc::c_char, cp_len: libc::c_int, ex_list: *mut *mut libc::c_char, ex_num: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn get_chi_pa(ptr1: *mut BNST_DATA, ptr2: *mut BNST_DATA, dist: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn find_head_mrph_from_dpnd_bnst(dep_ptr: *mut BNST_DATA, gov_ptr: *mut BNST_DATA) -> *mut MRPH_DATA;
    #[no_mangle]
    pub fn assign_ga_subject(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    pub fn specify_sm_from_cf(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    pub fn fix_sm_place(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    pub fn calc_bnst_length(sp: *mut SENTENCE_DATA, b_ptr: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    pub fn deflateInit_(strm: z_streamp, level: libc::c_int, version: *const libc::c_char, stream_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_init(cdbp: *mut cdb, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_read(cdbp: *const cdb, buf: *mut libc::c_void, len: libc::c_uint, pos: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_find(cdbp: *mut cdb, key: *const libc::c_void, klen: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_make_start(cdbmp: *mut cdb_make, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_make_add(cdbmp: *mut cdb_make, key: *const libc::c_void, klen: libc::c_uint, val: *const libc::c_void, vlen: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    pub fn cdb_make_finish(cdbmp: *mut cdb_make) -> libc::c_int;
    #[no_mangle]
    pub fn get_cf_event_value(cf1: *mut CASE_FRAME, cf2: *mut CASE_FRAME) -> libc::c_float;
    #[no_mangle]
    pub fn pp_code_to_kstr_in_context(cpm_ptr: *mut CF_PRED_MGR, num: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn copy_cf_with_alloc(dst: *mut CASE_FRAME, src: *mut CASE_FRAME);
    #[no_mangle]
    pub fn clear_mgr_cf(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    pub fn cf_match_sm_thesaurus(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn calc_similarity_word_cf(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    pub fn calc_similarity_word_cf_with_sm(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    pub fn print_good_crrspnds(cpm_ptr: *mut CF_PRED_MGR, cmm_ptr: *mut CF_MATCH_MGR, ipal_num: libc::c_int);
    #[no_mangle]
    pub fn ClearSMList();
    #[no_mangle]
    pub fn dt_classify(data: *mut libc::c_char, pp: libc::c_int) -> libc::c_float;
    #[no_mangle]
    pub fn strdup_with_check(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn loc_name_to_code(loc: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn loc_code_to_str(loc: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char, __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    pub fn print_eos(eos_flag: libc::c_int);
    #[no_mangle]
    pub fn _print_bnst(ptr: *mut TAG_DATA);
    #[no_mangle]
    pub fn get_cf_probability(cfd: *mut CASE_FRAME, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn get_case_function_probability(as1: libc::c_int, cfd: *mut CASE_FRAME, as2: libc::c_int, cfp: *mut CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    pub fn get_case_probability_from_str(case_str: *mut libc::c_char, cfp: *mut CASE_FRAME, aflag: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    pub fn get_case_num_probability(cfp: *mut CASE_FRAME, num: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    pub fn get_np_modifying_probability(as1: libc::c_int, cfd: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    pub fn get_topic_generating_probability(have_topic: libc::c_int, g_ptr: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    pub fn _smp2smg(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn smp2smg(cpd: *mut libc::c_char, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn calc_sm_words_similarity(smd: *mut libc::c_char, exp: *mut *mut libc::c_char, num: libc::c_int, pos: *mut libc::c_int, del: *mut libc::c_char, expand: libc::c_int, unmatch_word: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    pub fn calc_similarity(exd: *mut libc::c_char, exp: *mut libc::c_char, expand: libc::c_int) -> libc::c_float;
}

#[inline]
pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}

#[inline]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}

#[inline]
pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
        (__bsx as libc::c_int & 0xff as libc::c_int) <<
            8 as libc::c_int) as __uint16_t;
}

#[inline]
pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
        (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
        (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
        (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}

#[inline]
pub(crate) unsafe extern "C" fn stat(mut __path: *const libc::c_char, mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
