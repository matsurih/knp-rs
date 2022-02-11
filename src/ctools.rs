use std::str;

use crate::{structs, tools, types};
use crate::structs::{CF_MATCH_MGR, DPND, DpndRule, FEATURE_PATTERN, KoouRule, MRPH_DATA, QUOTE_DATA, REGEXPBNSTS, REGEXPMRPHS, stat};
use crate::types::{__off_t, BNST_DATA, CASE_FRAME, CELL, CF_PRED_MGR, DBM_FILE, FEATURE, FILE, SENTENCE_DATA, size_t, SMLIST, TAG_DATA, time_t, z_streamp};

pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum cdb_rl {}
pub enum internal_state {}

extern "C" {
    #[no_mangle]
    static mut SMExist: libc::c_int;
    #[no_mangle]
    static mut SMP2SMGExist: libc::c_int;
    #[no_mangle]
    static mut used_auto_dic_features: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut used_auto_dic_features_num: libc::c_int;
    #[no_mangle]
    static mut CRFFileNE: *mut libc::c_char;
    #[no_mangle]
    static mut DTFile: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut SynonymFile: *mut libc::c_char;
    #[no_mangle]
    static mut DistSimFile: *mut libc::c_char;
    #[no_mangle]
    static mut DistSimDB: *mut libc::c_char;
    #[no_mangle]
    static mut DistSimWordList: *mut libc::c_char;
    #[no_mangle]
    static mut DiscAddedCases: [libc::c_int; 0];
    #[no_mangle]
    static mut LocationLimit: [libc::c_int; 0];
    #[no_mangle]
    static mut PrevSentenceLimit: libc::c_int;
    #[no_mangle]
    static mut LocationOrder: [[libc::c_int; 21]; 0];
    #[no_mangle]
    static mut AntecedentDecideThresholdPredGeneral: libc::c_float;
    #[no_mangle]
    static mut AntecedentDecideThresholdForGa: libc::c_float;
    #[no_mangle]
    static mut AntecedentDecideThresholdForNi: libc::c_float;
    #[no_mangle]
    static mut Jumangram_Dirname: [libc::c_char; 0];
    #[no_mangle]
    static mut OptMergeCFResult: libc::c_int;
    #[no_mangle]
    static mut OptDiscPredMethod: libc::c_int;
    #[no_mangle]
    static mut OptDiscNounMethod: libc::c_int;
    #[no_mangle]
    static mut OptLearn: libc::c_int;
    #[no_mangle]
    static mut OptDiscFlag: libc::c_int;
    #[no_mangle]
    static mut OptAddSvmFeatureUtype: libc::c_int;
    #[no_mangle]
    static mut OptAddSvmFeatureDiscourseDepth: libc::c_int;
    #[no_mangle]
    static mut OptAddSvmFeatureObjectRecognition: libc::c_int;
    #[no_mangle]
    static mut OptAddSvmFeatureReferedNum: libc::c_int;
    #[no_mangle]
    static mut OptNoCandidateBehind: libc::c_int;
    #[no_mangle]
    static mut OptAnaphoraBaseline: libc::c_int;
    #[no_mangle]
    static mut EX_match_score: [libc::c_int; 0];
    #[no_mangle]
    static mut OptCorefer: libc::c_int;
    #[no_mangle]
    static mut OptSemanticHead: libc::c_int;
    #[no_mangle]
    static mut DpndRuleArray: [DpndRule; 0];
    #[no_mangle]
    static mut CurDpndRuleSize: libc::c_int;
    #[no_mangle]
    static mut CurKoouRuleSize: libc::c_int;
    #[no_mangle]
    static mut KoouRuleArray: [KoouRule; 0];
    #[no_mangle]
    static mut WarningComment: *mut libc::c_char;
    #[no_mangle]
    static mut path_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut OptArticle: libc::c_int;
    #[no_mangle]
    static mut OptCopula: libc::c_int;
    #[no_mangle]
    static mut OptRecoverPerson: libc::c_int;
    #[no_mangle]
    static mut CLASS_num: libc::c_int;
    #[no_mangle]
    static mut PrintNum: libc::c_int;
    #[no_mangle]
    static mut quote_data: QUOTE_DATA;
    #[no_mangle]
    static mut LineNo: libc::c_int;
    #[no_mangle]
    static mut LineNoForError: libc::c_int;
    #[no_mangle]
    static mut Case_name: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut OptUseRN: libc::c_int;
    #[no_mangle]
    static mut Type: [tools::TYPE; 128];
    #[no_mangle]
    static mut OptExpandP: libc::c_int;
    #[no_mangle]
    static mut Revised_para_num: libc::c_int;
    #[no_mangle]
    static mut restrict_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut D_found_array: [libc::c_int; 200];
    #[no_mangle]
    static mut OptDisplayNE: libc::c_int;
    #[no_mangle]
    static mut OptNECRF: libc::c_int;
    #[no_mangle]
    static mut OptNEcache: libc::c_int;
    #[no_mangle]
    static mut OptNEend: libc::c_int;
    #[no_mangle]
    static mut OptNEcase: libc::c_int;
    #[no_mangle]
    static mut OptNEparent: libc::c_int;
    #[no_mangle]
    static mut current_sentence_data: types::SENTENCE_DATA;
    #[no_mangle]
    static mut OptUseCF: libc::c_int;
    #[no_mangle]
    static mut OptUseNCF: libc::c_int;
    #[no_mangle]
    static mut OptUseCPNCF: libc::c_int;
    #[no_mangle]
    static mut VerboseLevel: types::VerboseType;
    #[no_mangle]
    static mut smlist: [SMLIST; 0];
    #[no_mangle]
    static mut Dpnd_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Quote_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Mask_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut ETAG_name: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut EX_match_exact: libc::c_int;
    #[no_mangle]
    static mut OptCKY: libc::c_int;
    #[no_mangle]
    static mut OptCFMode: libc::c_int;
    #[no_mangle]
    static mut DICT: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut Work_mgr: structs::TOTAL_MGR;
    #[no_mangle]
    static mut Possibility: libc::c_int;
    #[no_mangle]
    static mut MAX_Case_frame_num: libc::c_int;
    #[no_mangle]
    static mut Para_matrix: [[[libc::c_double; 200]; 200]; 0];
    #[no_mangle]
    static mut Chi_dpnd_matrix: [[structs::CHI_DPND; 200]; 0];
    #[no_mangle]
    static mut Chi_pos_matrix: [structs::CHI_POS; 0];
    #[no_mangle]
    static mut Chi_root_prob_matrix: [structs::CHI_ROOT; 0];
    #[no_mangle]
    static mut Chi_word_pos: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut left_arg: [libc::c_int; 0];
    #[no_mangle]
    static mut right_arg: [libc::c_int; 0];
    #[no_mangle]
    static mut Chi_pa_matrix: [[libc::c_double; 200]; 0];
    #[no_mangle]
    static mut Chi_np_start_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Chi_np_end_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Chi_quote_start_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Chi_quote_end_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    static mut Chi_root: libc::c_int;
    #[no_mangle]
    static mut OptPostProcess: libc::c_int;
    #[no_mangle]
    static mut OptParaFix: libc::c_int;
    #[no_mangle]
    static mut OptParaNoFixFlag: libc::c_int;
    #[no_mangle]
    static mut OptNbest: libc::c_int;
    #[no_mangle]
    static mut OptBeam: libc::c_int;
    #[no_mangle]
    static mut OptChiGenerative: libc::c_int;
    #[no_mangle]
    static mut CFSimExist: libc::c_int;
    #[no_mangle]
    static mut entity_manager: types::ENTITY_MGR;
    #[no_mangle]
    static mut corefer_id: libc::c_int;
    #[no_mangle]
    static mut OptGeneralCF: libc::c_int;
    #[no_mangle]
    static mut OptCaseFlag: libc::c_int;
    #[no_mangle]
    static mut author_score: libc::c_double;
    #[no_mangle]
    static mut reader_score: libc::c_double;
    #[no_mangle]
    static mut author_sen: libc::c_int;
    #[no_mangle]
    static mut author_tag: libc::c_int;
    #[no_mangle]
    static mut reader_sen: libc::c_int;
    #[no_mangle]
    static mut reader_tag: libc::c_int;
    #[no_mangle]
    static mut SM2CODEExist: libc::c_int;
    #[no_mangle]
    static mut Options: *mut *mut libc::c_char;
    #[no_mangle]
    static mut OptExpress: libc::c_int;
    #[no_mangle]
    static mut OptPosModification: libc::c_int;
    #[no_mangle]
    static mut matched_ptr: *mut libc::c_void;
    #[no_mangle]
    pub static mut stderr: *mut types::FILE;
    #[no_mangle]
    pub static mut stdout: *mut types::FILE;
    #[no_mangle]
    pub static mut stdin: *mut types::FILE;
    #[no_mangle]
    pub static mut AntecedentDecideThresholdForNoun: libc::c_float;
    #[no_mangle]
    static mut Thesaurus: libc::c_int;
    #[no_mangle]
    static mut ParaThesaurus: libc::c_int;
    #[no_mangle]
    static mut OptUseScase: libc::c_int;
    #[no_mangle]
    static mut OptUseSmfix: libc::c_int;
    #[no_mangle]
    static mut Class: [[types::CLASS; 129]; 129];
    #[no_mangle]
    static mut ContRuleArray: [structs::BnstRule; 0];
    #[no_mangle]
    static mut ContRuleSize: libc::c_int;
    #[no_mangle]
    static mut DBforNE: *mut libc::c_char;
    #[no_mangle]
    static mut EX_PRINT_NUM: libc::c_int;
    #[no_mangle]
    static mut PrintFrequency: libc::c_int;
    #[no_mangle]
    static mut PrintDeletedSM: libc::c_int;
    #[no_mangle]
    static mut PrintFeatures: libc::c_int;
    #[no_mangle]
    static mut PrintEx: libc::c_int;
    #[no_mangle]
    static mut CFSimThreshold: libc::c_float;
    #[no_mangle]
    static mut RULE: *mut types::RuleVector;
    #[no_mangle]
    static mut CurrentRuleNum: libc::c_int;
    #[no_mangle]
    static mut SOTO_THRESHOLD: libc::c_int;
    #[no_mangle]
    static mut DISTANCE_STEP: libc::c_int;
    #[no_mangle]
    static mut RENKAKU_STEP: libc::c_int;
    #[no_mangle]
    static mut STRONG_V_COST: libc::c_int;
    #[no_mangle]
    static mut ADJACENT_TOUTEN_COST: libc::c_int;
    #[no_mangle]
    static mut LEVELA_COST: libc::c_int;
    #[no_mangle]
    static mut TEIDAI_STEP: libc::c_int;
    #[no_mangle]
    static mut EX_match_qua: libc::c_int;
    #[no_mangle]
    static mut EX_match_unknown: libc::c_int;
    #[no_mangle]
    static mut EX_match_sentence: libc::c_int;
    #[no_mangle]
    static mut EX_match_tim: libc::c_int;
    #[no_mangle]
    static mut EX_match_subject: libc::c_int;
    #[no_mangle]
    static mut ErrorComment: *mut libc::c_char;
    #[no_mangle]
    static mut PM_Memo: [libc::c_char; 0];
    #[no_mangle]
    static mut sentence_data: [tools::SENTENCE_DATA; 0];
    #[no_mangle]
    static mut Chi_word_type: [*mut libc::c_char; 0];
    #[no_mangle]
    pub(crate) static mut OptAnalysis: libc::c_int;
    #[no_mangle]
    static mut OptEllipsis: libc::c_int;
    #[no_mangle]
    static mut OptInput: libc::c_int;
    #[no_mangle]
    static mut OptDisplay: libc::c_int;
    #[no_mangle]
    static mut OptKatakanaNormalize: libc::c_int;
    #[no_mangle]
    static mut OptIgnoreChar: libc::c_char;
    #[no_mangle]
    static mut OptReadFeature: libc::c_int;
    #[no_mangle]
    static mut OptNE: libc::c_int;
    #[no_mangle]
    pub static mut Language: libc::c_int;
    #[no_mangle]
    static mut OptNElearn: libc::c_int;
    #[no_mangle]
    static mut OptAnaphora: libc::c_int;
    #[no_mangle]
    pub static mut OptChiPos: libc::c_int;
    #[no_mangle]
    static mut Form: [[tools::FORM; 128]; 128];
    #[no_mangle]
    static mut HomoRuleArray: [tools::HomoRule; 0];
    #[no_mangle]
    static mut CurHomoRuleSize: libc::c_int;
    #[no_mangle]
    static mut GeneralRuleNum: libc::c_int;
    #[no_mangle]
    static mut GeneralRuleArray: *mut tools::GeneralRuleType;
    #[no_mangle]
    pub static mut Outfp: *mut tools::FILE;
    #[no_mangle]
    static mut OptMode: libc::c_int;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut types::time_t) -> types::time_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn getpid() -> types::__pid_t;
    #[no_mangle]
    fn setuid(__uid: types::__uid_t) -> libc::c_int;
    #[no_mangle]
    fn setgid(__gid: types::__gid_t) -> libc::c_int;
    #[no_mangle]
    fn fork() -> types::__pid_t;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut types::FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut types::FILE) -> libc::c_int;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut types::FILE) -> libc::c_ulong;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: types::__sighandler_t) -> types::__sighandler_t;
    #[no_mangle]
    fn sigfillset(__set: *mut types::sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(__how: libc::c_int, __set: *const types::sigset_t, __oset: *mut types::sigset_t) -> libc::c_int;
    #[no_mangle]
    fn _setjmp(_: *mut structs::__jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut structs::__jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    fn umask(__mask: types::__mode_t) -> types::__mode_t;
    #[no_mangle]
    fn init_crf_for_NE() -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut types::FILE;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut types::FILE;
    #[no_mangle]
    fn fflush(__stream: *mut types::FILE) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut types::FILE) -> libc::c_int;
    #[no_mangle]
    pub fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    pub fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn bind(__fd: libc::c_int, __addr: *const structs::sockaddr, __len: types::socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn connect(__fd: libc::c_int, __addr: *const structs::sockaddr, __len: types::socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn accept(__fd: libc::c_int, __addr: *mut structs::sockaddr, __addr_len: *mut types::socklen_t) -> libc::c_int;
    #[no_mangle]
    pub fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn gethostbyname(__name: *const libc::c_char) -> *mut structs::hostent;
    #[no_mangle]
    pub fn waitpid(__pid: types::__pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> types::__pid_t;
    #[no_mangle]
    pub fn getpwnam(__name: *const libc::c_char) -> *mut structs::passwd;
    #[no_mangle]
    pub fn getgrgid(__gid: types::__gid_t) -> *mut structs::group;
    #[no_mangle]
    pub fn setgroups(__n: types::size_t, __groups: *const types::__gid_t) -> libc::c_int;
    #[no_mangle]
    pub fn grammar(fp_out: *mut types::FILE);
    #[no_mangle]
    fn katuyou(fp: *mut types::FILE);
    #[no_mangle]
    fn assign_mrph_num(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn all_sentence_anaphora_analysis(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn clear_context(sp: *mut types::SENTENCE_DATA, init_flag: libc::c_int);
    #[no_mangle]
    fn calc_match_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn init_case_analysis_cmm();
    #[no_mangle]
    fn record_all_case_analisys(sp: *mut types::SENTENCE_DATA, temp_assign_flag: libc::c_int);
    #[no_mangle]
    fn init_cf();
    #[no_mangle]
    fn init_mrph2id();
    #[no_mangle]
    fn init_soto_txt();
    #[no_mangle]
    fn init_case_analysis_cpm(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn close_cf();
    #[no_mangle]
    fn set_frame_num_max(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn set_caseframes(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn clear_cf(flag: libc::c_int);
    #[no_mangle]
    fn assign_pred_feature_to_bp(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn init_configfile(opfile: *mut libc::c_char);
    #[no_mangle]
    fn server_read_rc(fp: *mut types::FILE);
    #[no_mangle]
    fn PreserveCPM(sp_new: *mut types::SENTENCE_DATA, sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn PreserveSentence(sp: *mut types::SENTENCE_DATA) -> *mut types::SENTENCE_DATA;
    #[no_mangle]
    fn DiscourseAnalysis(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn init_chi_dpnd_db();
    #[no_mangle]
    fn close_chi_dpnd_db();
    #[no_mangle]
    fn init_chi_type();
    #[no_mangle]
    fn free_chi_type();
    #[no_mangle]
    fn init_chi_pos();
    #[no_mangle]
    fn free_chi_pos();
    #[no_mangle]
    fn dpnd_info_to_bnst(sp: *mut types::SENTENCE_DATA, dp: *mut structs::DPND);
    #[no_mangle]
    fn dpnd_info_to_tag(sp: *mut types::SENTENCE_DATA, dp: *mut structs::DPND);
    #[no_mangle]
    fn dpnd_info_to_mrph(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn after_decide_dpnd(sp: *mut types::SENTENCE_DATA, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn calc_dpnd_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn calc_chi_dpnd_matrix_forProbModel(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn calc_chi_dpnd_matrix_wpos(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn calc_chi_pos_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn relax_dpnd_matrix(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn para_postprocess(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn detect_dpnd_case_struct(sp: *mut types::SENTENCE_DATA, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn when_no_dpnd_struct(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn check_candidates(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn memo_by_program(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn calc_gigaword_pa_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    pub fn check_feature(fp: *mut types::FEATURE, fname: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn assign_cfeature(fpp: *mut *mut types::FEATURE, fname: *mut libc::c_char, temp_assign_flag: libc::c_int);
    #[no_mangle]
    fn clear_feature(fpp: *mut *mut types::FEATURE);
    #[no_mangle]
    fn delete_cfeature(fpp: *mut *mut types::FEATURE, type_0: *mut libc::c_char);
    #[no_mangle]
    fn copy_feature(dst_fpp: *mut *mut types::FEATURE, src_fp: *mut types::FEATURE);
    #[no_mangle]
    fn koou(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn print_matrix(sp: *mut types::SENTENCE_DATA, type_0: libc::c_int, L_B: libc::c_int);
    #[no_mangle]
    fn print_result(sp: *mut types::SENTENCE_DATA, case_print_flag: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    fn check_bnst(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn print_para_relation(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn assign_para_similarity_feature(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn prepare_all_entity(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn do_postprocess(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn print_all_result(sp: *mut types::SENTENCE_DATA, eos_flag: libc::c_int);
    #[no_mangle]
    fn print_bnst_with_mrphs(sp: *mut types::SENTENCE_DATA, have_dpnd_flag: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    fn init_scase();
    #[no_mangle]
    fn close_scase();
    #[no_mangle]
    fn fix_sm_person(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn init_mask_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn init_para_matrix(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn check_dpnd_in_para(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn para_recovery(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn check_para_key(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn detect_all_para_scope(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn detect_para_scope(sp: * muttypes::SENTENCE_DATA, para_num: libc::c_int, restrict_p: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn detect_para_relation(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn quote(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn process_input_paren(sp: *mut types::SENTENCE_DATA, paren_spp: *mut *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn prepare_paren_sentence(sp: *mut types::SENTENCE_DATA, paren_sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn read_mrph(sp: *mut types::SENTENCE_DATA, fp: *mut types::FILE) -> libc::c_int;
    #[no_mangle]
    fn assign_general_feature(data: *mut libc::c_void, size: libc::c_int, flag: libc::c_int, also_assign_flag: libc::c_int, temp_assign_flag: libc::c_int);
    #[no_mangle]
    fn make_bunsetsu(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn make_bunsetsu_pm(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn assign_dpnd_rule(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn make_tag_units(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn assign_cc_feature_to_bp(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn assign_cc_feature_to_bnst(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn preprocess_mrph(sp: *mut types::SENTENCE_DATA);
    #[no_mangle]
    fn read_homo_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    fn read_general_rule(rule: *mut types::RuleVector);
    #[no_mangle]
    fn read_dpnd_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    fn read_dpnd_rule_for_chinese(file_name: *mut libc::c_char);
    #[no_mangle]
    fn read_koou_rule(file_name: *mut libc::c_char);
    #[no_mangle]
    fn read_bnst_rule(file_name: *mut libc::c_char, rp: *mut structs::BnstRule, count: *mut libc::c_int, max: libc::c_int);
    #[no_mangle]
    fn init_thesaurus();
    #[no_mangle]
    fn close_thesaurus();
    #[no_mangle]
    fn get_bnst_code_all(ptr: *mut types::BNST_DATA);
    #[no_mangle]
    pub fn malloc_data(size: types::size_t, comment: *mut libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn init_hash();
    #[no_mangle]
    fn make_dpnd_tree(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn cky(sp: *mut types::SENTENCE_DATA, Best_mgr_0: *mut structs::TOTAL_MGR, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn base_phrase(sp: *mut types::SENTENCE_DATA, is_frag_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fragment(sp: *mut types::SENTENCE_DATA) -> libc::c_int;
    #[no_mangle]
    fn init_hownet();
    #[no_mangle]
    fn init_distsim();
    #[no_mangle]
    fn fgetc(__stream: *mut tools::FILE) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pp_kstr_to_code(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pp_code_to_kstr(num: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ClearSentences(sp: *mut tools::SENTENCE_DATA);
    #[no_mangle]
    fn sm2feature(sp: *mut tools::SENTENCE_DATA);
    #[no_mangle]
    fn regexpmrph_match(ptr1: *mut tools::REGEXPMRPH, ptr2: *mut tools::MRPH_DATA) -> libc::c_int;
    #[no_mangle]
    fn delete_cfeature_from_mrphs(m_ptr: *mut tools::MRPH_DATA, length: libc::c_int, type_0: *mut libc::c_char);
    #[no_mangle]
    fn delete_alt_feature(fpp: *mut *mut tools::FEATURE);
    #[no_mangle]
    fn append_feature(fpp: *mut *mut tools::FEATURE, afp: *mut tools::FEATURE);
    #[no_mangle]
    fn assign_feature(fpp1: *mut *mut tools::FEATURE, fpp2: *mut *mut tools::FEATURE, ptr: *mut libc::c_void, offset: libc::c_int, length: libc::c_int, temp_assign_flag: libc::c_int);
    #[no_mangle]
    fn regexpbnstrule_match(r_ptr: *mut tools::BnstRule, d_ptr: *mut tools::BNST_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn regexptagrule_match(r_ptr: *mut tools::BnstRule, d_ptr: *mut tools::TAG_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn regexpmrphrule_match(r_ptr: *mut tools::MrphRule, d_ptr: *mut tools::MRPH_DATA, bw_length: libc::c_int, fw_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub(crate) fn string_length(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn realloc_data(ptr: *mut libc::c_void, size: tools::size_t, comment: *mut libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn car(cell: *mut types::CELL) -> *mut types::CELL;
    #[no_mangle]
    fn cdr(cell: *mut types::CELL) -> *mut types::CELL;
    #[no_mangle]
    fn subordinate_level_comp(ptr1: *mut types::BNST_DATA, ptr2: *mut types::BNST_DATA) -> libc::c_int;
    #[no_mangle]
    fn subordinate_level_check(cp: *mut libc::c_char, f: *mut types::FEATURE) -> libc::c_int;
    #[no_mangle]
    fn make_fukugoji_id(b_ptr: *mut types::BNST_DATA) -> *mut libc::c_char;
    #[no_mangle]
    fn make_fukugoji_case_string(b_ptr: *mut types::TAG_DATA) -> *mut libc::c_char;
    #[no_mangle]
    fn set_pred_voice(b_ptr: *mut types::BNST_DATA);
    #[no_mangle]
    fn bgh_match_check(pat: *mut libc::c_char, codes: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _sm_match_score(cpp: *mut libc::c_char, cpd: *mut libc::c_char, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_scase_code(ptr: *mut types::BNST_DATA);
    #[no_mangle]
    fn sm_all_match(c: *mut libc::c_char, target: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sm2code(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn sm_match_check(pat: *mut libc::c_char, codes: *mut libc::c_char, expand: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn assign_sm(bp: *mut types::BNST_DATA, cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn check_auto_dic(m_ptr: *mut structs::MRPH_DATA, assign_pos: libc::c_int, m_length: libc::c_int, rule_value: *mut libc::c_char, temp_assign_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn change_mrph(m_ptr: *mut structs::MRPH_DATA, f: *mut types::FEATURE);
    #[no_mangle]
    fn check_nv_mi_parent_and_children(v_ptr: *mut types::TAG_DATA, rank_threshold: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn change_one_mrph_rep(m_ptr: *mut structs::MRPH_DATA, modify_feature_flag: libc::c_int, suffix_char: libc::c_char);
    #[no_mangle]
    fn case2num(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut types::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn get_bnst_head_canonical_rep(ptr: *mut types::BNST_DATA, compound_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn make_pred_string(t_ptr: *mut types::TAG_DATA, m_ptr: *mut structs::MRPH_DATA, orig_form: *mut libc::c_char, use_rep_flag: libc::c_int, cf_type: libc::c_int, cpncf_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn make_pred_string_from_mrph(t_ptr: *mut types::TAG_DATA, m_ptr: *mut structs::MRPH_DATA, orig_form: *mut libc::c_char, use_rep_flag: libc::c_int, cf_type: libc::c_int, cpncf_flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn MatchPP(n: libc::c_int, pp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn make_print_string(bp: *mut types::TAG_DATA, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn get_case_probability(as2: libc::c_int, cfp: *mut types::CASE_FRAME, aflag: libc::c_int, para_cpm_ptr: *mut types::CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    fn get_case_function_probability_for_pred(as1: libc::c_int, cfd: *mut types::CASE_FRAME, as2: libc::c_int, cfp: *mut types::CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn get_ex_probability_with_para(as1: libc::c_int, cfd: *mut types::CASE_FRAME, as2: libc::c_int, cfp: *mut types::CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn get_ex_probability(as1: libc::c_int, cfd: *mut types::CASE_FRAME, dp: *mut types::TAG_DATA, as2: libc::c_int, cfp: *mut types::CASE_FRAME, sm_flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn get_ex_ne_probability(cp: *mut libc::c_char, as2: libc::c_int, cfp: *mut types::CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn _get_ex_probability_internal(key: *mut libc::c_char, as2: libc::c_int, cfp: *mut types::CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn get_cf_probability_for_pred(cfd: *mut types::CASE_FRAME, cfp: *mut types::CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn get_general_probability(key1: *mut libc::c_char, key2: *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn get_key_probability(tag_ptr: *mut types::TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn get_class_probability(key: *mut libc::c_char, as2: libc::c_int, cfp: *mut types::CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn check_str_type(ucp: *mut libc::c_uchar, allowed_type: libc::c_int, length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_cfs_similarity(cf1: *mut libc::c_char, cf2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn CheckCF(key: *mut libc::c_char) -> *mut types::CFLIST;
    #[no_mangle]
    fn get_pred_id(cfid: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn make_data_cframe(sp: *mut types::SENTENCE_DATA, cpm_ptr: *mut types::CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    fn init_case_frame(cf: *mut types::CASE_FRAME);
    #[no_mangle]
    fn db_get(db: types::DBM_FILE, buf: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn list2feature_pattern(f: *mut structs::FEATURE_PATTERN, cell: *mut types::CELL);
    #[no_mangle]
    fn feature_pattern_match(fr: *mut structs::FEATURE_PATTERN, fd: *mut types::FEATURE, p1: *mut libc::c_void, p2: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn cons(car_0: *mut libc::c_void, cdr_0: *mut libc::c_void) -> *mut types::CELL;
    #[no_mangle]
    fn length(list: *mut types::CELL) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn print_kakari(sp: *mut types::SENTENCE_DATA, type_0: libc::c_int, eos_flag: libc::c_int);
    #[no_mangle]
    fn get_case_prob_wpos(sp: *mut types::SENTENCE_DATA, head: libc::c_int, left_arg_num: libc::c_int, right_arg_num: libc::c_int, pos_index_pre: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn get_case_prob(sp: *mut types::SENTENCE_DATA, head: libc::c_int, left_arg_num: libc::c_int, right_arg_num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn print_crrspnd(cpm_ptr: *mut types::CF_PRED_MGR, cmm_ptr: *mut structs::CF_MATCH_MGR);
    #[no_mangle]
    fn print_data_cframe(cpm_ptr: *mut types::CF_PRED_MGR, cmm_ptr: *mut structs::CF_MATCH_MGR);
    #[no_mangle]
    fn get_noun_co_num_probability(gp: *mut types::TAG_DATA, num: libc::c_int, para_cky_ptr: *mut types::CKY) -> libc::c_double;
    #[no_mangle]
    fn get_noun_co_ex_probability(dp: *mut types::TAG_DATA, gp: *mut types::TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn get_para_ex_probability(para_key: *mut libc::c_char, score: libc::c_double, dp: *mut types::TAG_DATA, gp: *mut types::TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn get_para_exist_probability(para_key: *mut libc::c_char, score: libc::c_double, flag: libc::c_int, dp: *mut types::TAG_DATA, gp: *mut types::TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn calc_adv_modifying_num_probability(t_ptr: *mut types::TAG_DATA, cfp: *mut types::CASE_FRAME, num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn calc_adv_modifying_probability(gp: *mut types::TAG_DATA, cfp: *mut types::CASE_FRAME, dp: *mut types::TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn calc_vp_modifying_num_probability(t_ptr: *mut types::TAG_DATA, cfp: *mut types::CASE_FRAME, num: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn calc_vp_modifying_probability(gp: *mut types::TAG_DATA, g_cf: *mut types::CASE_FRAME, dp: *mut types::TAG_DATA, d_cf: *mut types::CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn _make_data_cframe_ex(cpm_ptr: *mut types::CF_PRED_MGR, b_ptr: *mut types::TAG_DATA);
    #[no_mangle]
    fn _make_data_cframe_sm(cpm_ptr: *mut types::CF_PRED_MGR, b_ptr: *mut types::TAG_DATA);
    #[no_mangle]
    fn _make_data_cframe_pp(cpm_ptr: *mut types::CF_PRED_MGR, b_ptr: *mut types::TAG_DATA, flag: libc::c_int) -> *mut types::TAG_DATA;
    #[no_mangle]
    fn make_data_cframe_child(sp: *mut types::SENTENCE_DATA, cpm_ptr: *mut types::CF_PRED_MGR, child_ptr: *mut types::TAG_DATA, child_num: libc::c_int, closest_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn find_best_cf(sp: *mut types::SENTENCE_DATA, cpm_ptr: *mut types::CF_PRED_MGR, closest: libc::c_int, decide: libc::c_int, para_cpm_ptr: *mut types::CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    fn get_closest_case_component(sp: *mut types::SENTENCE_DATA, cpm_ptr: *mut types::CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    fn noun_lexical_disambiguation_by_case_analysis(cpm_ptr: *mut types::CF_PRED_MGR);
    #[no_mangle]
    fn verb_lexical_disambiguation_by_case_analysis(cpm_ptr: *mut types::CF_PRED_MGR);
    #[no_mangle]
    fn copy_cpm(dst: *mut types::CF_PRED_MGR, src: *mut types::CF_PRED_MGR, flag: libc::c_int);
    #[no_mangle]
    fn record_case_analysis(sp: *mut types::SENTENCE_DATA, cpm_ptr: *mut types::CF_PRED_MGR, em_ptr: *mut types::ELLIPSIS_MGR, temp_assign_flag: libc::c_int);
    #[no_mangle]
    fn pp_hstr_to_code(cp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub(crate) fn get_mrph_rep_from_f(m_ptr: *mut structs::MRPH_DATA, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn db_read_open(filename: *mut libc::c_char) -> types::DBM_FILE;
    #[no_mangle]
    pub(crate) fn check_dict_filename(file: *mut libc::c_char, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut THESAURUS: [types::THESAURUS_FILE; 0];
    #[no_mangle]
    static mut match_matrix: [[libc::c_int; 200]; 0];
    #[no_mangle]
    fn similarity_chinese(str1: *mut libc::c_char, str2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn general_code_match(th: *mut types::THESAURUS_FILE, c1: *mut libc::c_char, c2: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn ntt_code_match(c1: *mut libc::c_char, c2: *mut libc::c_char, flag: libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn bgh_code_match(c1: *mut libc::c_char, c2: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn calc_distsim_from_bnst(ptr1: *mut types::BNST_DATA, ptr2: *mut types::BNST_DATA) -> libc::c_int;
    #[no_mangle]
    fn malloc_db_buf(size: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn open_dict(dic_num: libc::c_int, dic_name: *mut libc::c_char, exist: *mut libc::c_int) -> types::DBM_FILE;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: types::size_t, __size: types::size_t, __compar: types::__compar_fn_t);
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn count_pat_element(cfp: *mut types::CASE_FRAME, list2: *mut structs::LIST) -> libc::c_int;
    #[no_mangle]
    fn case_frame_match(cpm_ptr: *mut types::CF_PRED_MGR, cmm_ptr: *mut structs::CF_MATCH_MGR, flag: libc::c_int, closest: libc::c_int, para_cpm_ptr: *mut types::CF_PRED_MGR) -> libc::c_int;
    #[no_mangle]
    fn compare_dpnd(sp: *mut types::SENTENCE_DATA, new_mgr: *mut structs::TOTAL_MGR, best_mgr: *mut structs::TOTAL_MGR) -> libc::c_int;
    #[no_mangle]
    fn cf_match_element(d: *mut libc::c_char, target: *mut libc::c_char, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn count_assigned_adjacent_element(cfp: *mut types::CASE_FRAME, list2: *mut structs::LIST) -> libc::c_int;
    #[no_mangle]
    fn _calc_similarity_sm_cf(exd: *mut libc::c_char, expand: libc::c_int, unmatch_word: *mut libc::c_char, cfp: *mut types::CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn cf_match_exactly(word: *mut libc::c_char, word_len: libc::c_int, ex_list: *mut *mut libc::c_char, ex_num: libc::c_int, pos: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn init_mgr_cf(tmp: *mut structs::TOTAL_MGR);
    #[no_mangle]
    fn _make_ipal_cframe_pp(c_ptr: *mut types::CASE_FRAME, cp: *mut libc::c_uchar, num: libc::c_int, flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn check_cf_case(cfp: *mut types::CASE_FRAME, pp: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_mrph_rep_length(rep_strt: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn make_mrph_rn(m_ptr: *mut structs::MRPH_DATA) -> *mut libc::c_char;
    #[no_mangle]
    fn get_mrph_rep(m_ptr: *mut structs::MRPH_DATA) -> *mut libc::c_char;
    #[no_mangle]
    fn assign_feature_alt_mrph(fpp: *mut *mut types::FEATURE, m_ptr: *mut structs::MRPH_DATA);
    #[no_mangle]
    fn delete_existing_features(m_ptr: *mut structs::MRPH_DATA);
    #[no_mangle]
    fn copy_mrph(dst: *mut structs::MRPH_DATA, src: *mut structs::MRPH_DATA, imi2feature_flag: libc::c_int);
    #[no_mangle]
    fn get_str_code_with_len(cp: *mut libc::c_char, len: libc::c_int, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn hash(key: *mut libc::c_uchar, keylen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub(crate) fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
    #[no_mangle]
    fn realloc_cmm();
    #[no_mangle]
    fn CF_MatchPP(c: libc::c_int, cf: *mut tools::CASE_FRAME) -> libc::c_int;
    #[no_mangle]
    fn get_dist_from_work_mgr(bp: *mut tools::BNST_DATA, hp: *mut tools::BNST_DATA) -> libc::c_int;
    #[no_mangle]
    fn code2sm(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn get_str_code(cp: *mut libc::c_uchar, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn sms_match(cpp: *mut libc::c_char, cpd: *mut libc::c_char, expand: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn check_category(fp: *mut tools::FEATURE, fname: *mut libc::c_char, strict_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_crf_prob(i: libc::c_int, j: libc::c_int, prob: *mut libc::c_double);
    #[no_mangle]
    fn crf_parse();
    #[no_mangle]
    fn clear_crf();
    #[no_mangle]
    fn crf_add(line: *mut libc::c_char);
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn _check_para_d_struct(sp: *mut SENTENCE_DATA, str: libc::c_int, end: libc::c_int, extend_p: libc::c_int, limit: libc::c_int, s_p: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rep2id(rep: *mut libc::c_char, rep_len: libc::c_int, buffer: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _get_bgh(cp: *mut libc::c_char, arg: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn bgh_code_match_for_case(cp1: *mut libc::c_char, cp2: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn init_bgh();
    #[no_mangle]
    fn close_bgh();
    #[no_mangle]
    fn _get_ntt(cp: *mut libc::c_char, arg: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn init_ntt();
    #[no_mangle]
    fn close_ntt();
    #[no_mangle]
    fn delete_specified_sm(sm: *mut libc::c_char, del: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn check_noun_sm(key: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn katakana2hiragana(cp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    #[no_mangle]
    fn calc_distsim(word1: *mut libc::c_char, word2: *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn comp_sm(cpp: *mut libc::c_char, cpd: *mut libc::c_char, start: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn code_depth(cp: *mut libc::c_char, code_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_most_similar_code(exd: *mut libc::c_char, exp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn s_feof(fp: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn s_read(fp: *mut FILE) -> *mut CELL;
    #[no_mangle]
    fn store_regexpbnsts(bspp: *mut *mut REGEXPBNSTS, cell: *mut CELL);
    #[no_mangle]
    fn store_regexpmrphs(mspp: *mut *mut REGEXPMRPHS, cell: *mut CELL);
    #[no_mangle]
    fn check_rule_filename(file: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn list2feature(cp: *mut CELL, fpp: *mut *mut FEATURE);
    #[no_mangle]
    fn usage();
    #[no_mangle]
    fn revise_para_rel(sp: *mut SENTENCE_DATA, pre: libc::c_int, pos: libc::c_int);
    #[no_mangle]
    fn print_bnst(ptr: *mut BNST_DATA, cp: *mut libc::c_char);
    #[no_mangle]
    fn revise_para_kakari(sp: *mut SENTENCE_DATA, num: libc::c_int, array: *mut libc::c_int);
    #[no_mangle]
    fn feature_AND_match(fp: *mut FEATURE, fd: *mut FEATURE, p1: *mut libc::c_void, p2: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn string2feature_pattern(f: *mut FEATURE_PATTERN, cp: *mut libc::c_char);
    #[no_mangle]
    fn db_write_open(filename: *mut libc::c_char) -> DBM_FILE;
    #[no_mangle]
    fn db_put(db: DBM_FILE, buf: *mut libc::c_char, value: *mut libc::c_char, Separator: *mut libc::c_char, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn db_close(db: DBM_FILE);
    #[no_mangle]
    fn hiragana2katakana(cp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    #[no_mangle]
    fn init_bnst_tree_property(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t, __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn print_pa_structure(sp: *mut tools::SENTENCE_DATA, eos_flag: libc::c_int);
    #[no_mangle]
    fn tag_bnst_postprocess(sp: *mut tools::SENTENCE_DATA, flag: libc::c_int);
    #[no_mangle]
    fn undo_tag_bnst_postprocess(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    fn print_feature(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    fn print_feature2(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    fn print_some_feature(fp: *mut FEATURE, filep: *mut FILE);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _regexpbnst_match(r_ptr: *mut REGEXPMRPHS, b_ptr: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    fn call_case_analysis(sp: *mut SENTENCE_DATA, dpnd: DPND, eos_flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn InitCPMcache();
    #[no_mangle]
    fn ClearCPMcache();
    #[no_mangle]
    fn check_examples(cp: *mut libc::c_char, cp_len: libc::c_int, ex_list: *mut *mut libc::c_char, ex_num: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_chi_pa(ptr1: *mut BNST_DATA, ptr2: *mut BNST_DATA, dist: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn find_head_mrph_from_dpnd_bnst(dep_ptr: *mut BNST_DATA, gov_ptr: *mut BNST_DATA) -> *mut MRPH_DATA;
    #[no_mangle]
    fn assign_ga_subject(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    fn specify_sm_from_cf(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    fn fix_sm_place(sp: *mut SENTENCE_DATA, cpm_ptr: *mut CF_PRED_MGR);
    #[no_mangle]
    fn calc_bnst_length(sp: *mut SENTENCE_DATA, b_ptr: *mut BNST_DATA) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn deflateInit_(strm: z_streamp, level: libc::c_int, version: *const libc::c_char, stream_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cdb_init(cdbp: *mut tools::cdb, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cdb_read(cdbp: *const tools::cdb, buf: *mut libc::c_void, len: libc::c_uint, pos: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn cdb_find(cdbp: *mut tools::cdb, key: *const libc::c_void, klen: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_start(cdbmp: *mut tools::cdb_make, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_add(cdbmp: *mut tools::cdb_make, key: *const libc::c_void, klen: libc::c_uint, val: *const libc::c_void, vlen: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn cdb_make_finish(cdbmp: *mut tools::cdb_make) -> libc::c_int;
    #[no_mangle]
    fn get_cf_event_value(cf1: *mut CASE_FRAME, cf2: *mut CASE_FRAME) -> libc::c_float;
    #[no_mangle]
    fn pp_code_to_kstr_in_context(cpm_ptr: *mut CF_PRED_MGR, num: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn copy_cf_with_alloc(dst: *mut CASE_FRAME, src: *mut CASE_FRAME);
    #[no_mangle]
    fn clear_mgr_cf(sp: *mut SENTENCE_DATA);
    #[no_mangle]
    fn cf_match_sm_thesaurus(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn calc_similarity_word_cf(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn calc_similarity_word_cf_with_sm(tp: *mut TAG_DATA, cfp: *mut CASE_FRAME, n: libc::c_int, pos: *mut libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn print_good_crrspnds(cpm_ptr: *mut CF_PRED_MGR, cmm_ptr: *mut CF_MATCH_MGR, ipal_num: libc::c_int);
    #[no_mangle]
    fn ClearSMList();
    #[no_mangle]
    fn dt_classify(data: *mut libc::c_char, pp: libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn strdup_with_check(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn loc_name_to_code(loc: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn loc_code_to_str(loc: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char, __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn print_eos(eos_flag: libc::c_int);
    #[no_mangle]
    fn _print_bnst(ptr: *mut TAG_DATA);
    #[no_mangle]
    fn get_cf_probability(cfd: *mut CASE_FRAME, cfp: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn get_case_function_probability(as1: libc::c_int, cfd: *mut CASE_FRAME, as2: libc::c_int, cfp: *mut CASE_FRAME, flag: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn get_case_probability_from_str(case_str: *mut libc::c_char, cfp: *mut CASE_FRAME, aflag: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    fn get_case_num_probability(cfp: *mut CASE_FRAME, num: libc::c_int, para_cpm_ptr: *mut CF_PRED_MGR) -> libc::c_double;
    #[no_mangle]
    fn get_np_modifying_probability(as1: libc::c_int, cfd: *mut CASE_FRAME) -> libc::c_double;
    #[no_mangle]
    fn get_topic_generating_probability(have_topic: libc::c_int, g_ptr: *mut TAG_DATA) -> libc::c_double;
    #[no_mangle]
    fn _smp2smg(cp: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn smp2smg(cpd: *mut libc::c_char, flag: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn calc_sm_words_similarity(smd: *mut libc::c_char, exp: *mut *mut libc::c_char, num: libc::c_int, pos: *mut libc::c_int, del: *mut libc::c_char, expand: libc::c_int, unmatch_word: *mut libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn calc_similarity(exd: *mut libc::c_char, exp: *mut libc::c_char, expand: libc::c_int) -> libc::c_float;
}

#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}

#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}

#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: types::__uint16_t) -> types::__uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
        (__bsx as libc::c_int & 0xff as libc::c_int) <<
            8 as libc::c_int) as types::__uint16_t;
}

#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: types::__uint32_t) -> types::__uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
        (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
        (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
        (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
