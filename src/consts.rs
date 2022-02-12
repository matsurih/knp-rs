use crate::types::__socket_type;
use crate::VerboseType;

pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;

pub const VERBOSE5: VerboseType = 5;
pub const VERBOSE4: VerboseType = 4;
pub const VERBOSE3: VerboseType = 3;
pub const VERBOSE2: VerboseType = 2;
pub const VERBOSE1: VerboseType = 1;
pub const VERBOSE0: VerboseType = 0;


// pub const Max(x,y) (x < y ? y : x)
// pub const Min(x,y) (x < y ? x : y)
// 
// pub const str_eq(c1, c2) ( ! strcmp(c1, c2) )
// pub const L_Jiritu_M(ptr) (ptr->jiritu_ptr + ptr->jiritu_num - 1)

/*====================================================================
  LENGTH
====================================================================*/
pub const MRPH_MAX: libc::c_int = 200;
pub const BNST_MAX: libc::c_int = 200; /* 日本語の場合は64ぐらいで十分 */
pub const BNST_LENGTH_MAX: libc::c_int = 256;
pub const TAG_MAX: libc::c_int = 200;
pub const PAREN_MAX: libc::c_int = 100;
pub const PARA_MAX: libc::c_int = 32;
pub const PARA_PART_MAX: libc::c_int = 32;
pub const WORD_LEN_MAX: libc::c_int = 128;
pub const REPNAME_LEN_MAX: libc::c_int = 256;
pub const CF_ID_LEN_MAX: libc::c_int = 280;
pub const SENTENCE_MAX: libc::c_int = 512;
pub const PRi32_WIDTH: libc::c_int = 100;
pub const PARENT_MAX: libc::c_int = 20;
pub const BROTHER_MAX: libc::c_int = 20;
pub const TEIDAI_TYPES: libc::c_int = 5;
pub const HOMO_MAX: libc::c_int = 30;
pub const HOMO_MRPH_MAX: libc::c_int = 10;
pub const PP_STRING_MAX: libc::c_int = 16; /* 最大となるのは"無格従属:-1"など */
pub const FUKUGOJI_START: libc::c_int = 9;  /* PP_STR_TO_CODEで複合辞が始まる番号 */
pub const FUKUGOJI_END: libc::c_int = 37; /* PP_STR_TO_CODEで複合辞が終わる番号 */

pub const BGH_CODE_SIZE: libc::c_int = 11;
pub const SM_CODE_SIZE: libc::c_int = 12;
pub const SCASE_CODE_SIZE: libc::c_int = 11;

pub const HomoRule_MAX: libc::c_int = 128;
pub const BonusRule_MAX: libc::c_int = 16;
pub const KoouRule_MAX: libc::c_int = 124;
pub const DpndRule_MAX: libc::c_int = 128;
pub const DpndRule_G_MAX: libc::c_int = 35;
pub const ContRule_MAX: libc::c_int = 256;
pub const DicForRule_MAX: libc::c_int = 1024;
pub const NERule_MAX: libc::c_int = 512;
pub const CNRule_MAX: libc::c_int = 512;
pub const EtcRule_MAX: libc::c_int = 1024;
pub const GeneralRule_MAX: libc::c_int = 1024;

pub const IsMrphRule: libc::c_int = 1;
pub const IsBnstRule: libc::c_int = 2;
pub const IsMrph2Rule: libc::c_int = 3;

#[cfg(SMALL)]
pub const ALL_CASE_FRAME_MAX: libc::c_int = 1024;
#[cfg(not(SMALL))]
pub const ALL_CASE_FRAME_MAX: libc::c_int = 0;

pub const FRAME_NUM_MAX_NORMAL: libc::c_int = 4096;
pub const FRAME_NUM_MAX_SMALL: libc::c_int = 128;
pub const CF_ELEMENT_MAX: libc::c_int = 24;  /* 基本的には20までだがmerge_emで必要 */
pub const PP_ELEMENT_MAX: libc::c_int = 10;
pub const SM_ELEMENT_MAX: libc::c_int = 256;
pub const EX_ELEMENT_MAX: libc::c_int = 256;
pub const MAX_MATCH_MAX: libc::c_int = 10;

pub const CF_ALIGNMENT_MAX: libc::c_int = 5;

pub const CMM_MAX: libc::c_int = 5;  /* 最適格フレーム数 */
pub const CPM_MAX: libc::c_int = 64;  /* 文内述語数 */
pub const TM_MAX: libc::c_int = 5;  /* 最適依存構造数 */

pub const CLASS_NUM: libc::c_int = 2000;  /* 単語クラスの数 */
pub const MENTION_MAX: libc::c_int = 8;   /* 1つの基本句が持つ照応詞数(ゼロ照応含む) */
pub const ENTITY_MAX: libc::c_int = 4096;  /* ENTITYの数 */
pub const MENTIONED_MAX: libc::c_int = 256; /* 1つのENTITYが言及される回数 */

#[cfg(IMI_MAX)]
pub const IMI_MAX: libc::c_int = 1024;  /* defined in "juman.h" */

pub const DATA_LEN: libc::c_int = 5120;
pub const SMALL_DATA_LEN: libc::c_int = 128;
pub const SMALL_DATA_LEN2: libc::c_int = 256;
pub const ALLOCATION_STEP: libc::c_int = 1024;
pub const DEFAULT_PARSETIMEOUT: libc::c_int = 180;

pub const TBLSIZE: libc::c_int = 1024;
pub const NSEED: libc::c_int = 32;  /* 乱数表の種類。2 の羃乗でなければならない。 */
pub const NSIZE: libc::c_int = 256;

#[cfg(BYTES4CHAR)]
pub const BYTES4CHAR: libc::c_int = if cfg!(IO_ENCODING_EUC) || cfg!(IO_ENCODING_SJIS) { 2 } else { 3 };

pub const TREE_WIDTH_MAX: libc::c_int = 100;   /* Chinese parse tree width */
pub const CHI_WORD_LEN_MAX: libc::c_int = 30; /* maximum Chinese word length */
pub const CHI_POS_LEN_MAX: libc::c_int = 3;  /* maximum Chinese pos length */
pub const CHI_DPND_TYPE_MAX: libc::c_int = 10; /* maximum dpnd type of Chinese word pair */
pub const CHI_DPND_TYPE_LEN_MAX: libc::c_int = 10; /* maximum length of dpnd type */
pub const CHI_DPND_RULE_LEN_MAX: libc::c_int = 50; /* maximum length of dpnd rule */
pub const CHI_CKY_MAX: libc::c_int = 10; /* number of cky reserved for Chinese for each word pair */
pub const DOUBLE_MIN: f32 = 0.0000000000000001;
pub const CHI_TYPE_LEN_MAX: libc::c_int = 8;
pub const CHI_POS_MAX: libc::c_int = 33;
pub const CHI_ARG_NUM_MAX: libc::c_int = 30;
pub const CHI_POS_NBEST: libc::c_int = 2;
pub const CHI_DET_PENALTY: f32 = 0.3;

/*====================================================================
  SIMILARITY
====================================================================*/

pub const HOWNET_TRAN_MAX: libc::c_int = 100;
pub const HOWNET_CONCEPT_MAX: libc::c_int = 50;

/*====================================================================
  DEFINE
====================================================================*/
pub const JAPANESE: libc::c_int = 1;
pub const CHINESE: libc::c_int = 2;
pub const ENGLISH: libc::c_int = 3;

pub const OPT_CASE: libc::c_int = 1;
pub const OPT_CASE2: libc::c_int = 6;
pub const OPT_DPND: libc::c_int = 2;
pub const OPT_BNST: libc::c_int = 3;
pub const OPT_AssignF: libc::c_int = 4;
pub const OPT_FILTER: libc::c_int = 5;
pub const OPT_ELLIPSIS: libc::c_int = 1;
pub const OPT_DEMO: libc::c_int = 2;
pub const OPT_REL_NOUN: libc::c_int = 4;
pub const OPT_COREFER: libc::c_int = 8;
pub const OPT_CASE_ANALYSIS: libc::c_int = 16;
pub const OPT_AUTHOR: libc::c_int = 32;
pub const OPT_COREFER_AUTO: libc::c_int = 64;
pub const OPT_AUTHOR_AUTO: libc::c_int = 128;
pub const OPT_ALL_CASE: libc::c_int = 256;
pub const OPT_RAW: libc::c_int = 0;
pub const OPT_INPUT_PARSED: libc::c_int = 1;
pub const OPT_INPUT_BNST: libc::c_int = 2;
pub const OPT_INPUT_CHUNKED: libc::c_int = 4;
pub const OPT_TREE: libc::c_int = 1;
pub const OPT_TREEF: libc::c_int = 65;
pub const OPT_SEXP: libc::c_int = 8;
pub const OPT_TAB: libc::c_int = 0; /* TAG */
pub const OPT_NOTAG: libc::c_int = 2; /* BNST */
pub const OPT_MRPH: libc::c_int = 4;
pub const OPT_TABLE: libc::c_int = 16;
pub const OPT_PA: libc::c_int = 32;
pub const OPT_BNSTTREE: libc::c_int = 3; /* NOTAGTREE */
pub const OPT_MRPHTREE: libc::c_int = 5;
pub const OPT_NORMAL: libc::c_int = 1;
pub const OPT_DETAIL: libc::c_int = 2;
pub const OPT_DEBUG: libc::c_int = 3;
pub const OPT_ENTITY: libc::c_int = 4;
pub const OPT_SIMPLE: libc::c_int = 5;
pub const OPT_PARA_DETAIL: libc::c_int = 6;
pub const OPT_SVM: libc::c_int = 2;
pub const OPT_DT: libc::c_int = 3;
pub const OPT_SERV_FORE: libc::c_int = 1;
pub const OPT_CF_NE: libc::c_int = 1;
pub const OPT_CF_CATEGORY: libc::c_int = 2;
pub const OPT_CF_CLASS: libc::c_int = 4;
pub const OPT_ANAPHORA: libc::c_int = 1;
pub const OPT_PRi32_ENTITY: libc::c_int = 2;
pub const OPT_ANAPHORA_COPULA: libc::c_int = 4;
pub const OPT_ANAPHORA_PROB: libc::c_int = 8;
pub const OPT_TRAIN: libc::c_int = 16;
pub const OPT_UNNAMED_ENTITY: libc::c_int = 32;
pub const OPT_GS: libc::c_int = 64;
pub const OPT_NO_AUTHOR_ENTITY: libc::c_int = 128;
pub const OPT_NO_READER_ENTITY: libc::c_int = 256;
pub const OPT_AUTHOR_SCORE: libc::c_int = 512;
pub const OPT_READER_SCORE: libc::c_int = 1024;
pub const OPT_AUTHOR_ESTIMATE: libc::c_int = 2048;
pub const OPT_READER_ESTIMATE: libc::c_int = 4096;
pub const OPT_AUTHOR_AFTER: libc::c_int = 8192;
pub const OPT_ONLY_ENTITY: libc::c_int = 16384;
pub const OPT_NO_PSEUDO: libc::c_int = 32768;
pub const OPT_EACH_SENTENCE: libc::c_int = 65536;
pub const OPT_ITERATIVE: libc::c_int = 131072;
pub const OPT_PRUNING: libc::c_int = 262144;

pub const OPT_CASE_ASSIGN_GA_SUBJ: libc::c_int = 2;
pub const OPT_CASE_NO: libc::c_int = 4;
pub const OPT_CASE_USE_EX_ALL: libc::c_int = 8;
pub const OPT_CASE_USE_PROBABILITY: libc::c_int = 16;
pub const OPT_CASE_USE_REP_CF: libc::c_int = 32;
pub const OPT_CASE_ADD_SOTO_WORDS: libc::c_int = 64;
pub const OPT_CASE_GENERALIZE_AGENT: libc::c_int = 128;
pub const OPT_CASE_GENERATE_EOS: libc::c_int = 256;
pub const OPT_CASE_USE_CN_CF: libc::c_int = 512;
pub const OPT_CASE_USE_CREP_CF: libc::c_int = 1024; /* 常にOPT_CASE_USE_REP_CF */
pub const OPT_CASE_FIX_CF_SEARCH: libc::c_int = 2048;
pub const OPT_CASE_USE_NCF: libc::c_int = 4096;
pub const OPT_CASE_CF_CACHE: libc::c_int = 8192;
pub const OPT_CASE_CF_ON_MEMORY: libc::c_int = 16384;
pub const OPT_CASE_CLEAR_CF: libc::c_int = 32768;
pub const OPT_CASE_FALLBACK_TO_DPND: libc::c_int = 65536;
pub const OPT_CASE_USE_CV_CF: libc::c_int = 131072;
pub const OPT_CASE_CFCASE_FORMAT_DENOMINATOR: libc::c_int = 262144;
pub const OPT_CASE_ANALYZE_DEVERBATIVE_NOUN: libc::c_int = 524288;
pub const OPT_CASE_CF_USE_ID: libc::c_int = 1048576;
pub const OPT_CASE_PRi32_SCORE: libc::c_int = 2097152;
pub const OPT_CASE_PRi32_OBLIG: libc::c_int = 4194304;
pub const OPT_CASE_PRi32_ALL_SLOT: libc::c_int = 8388608;
pub const OPT_CASE_POSTPROCESS_PA: libc::c_int = 16777216;

pub const OPT_DISC_OR_CF: libc::c_int = 1;
pub const OPT_DISC_BEST: libc::c_int = 2;
pub const OPT_DISC_CLASS_ONLY: libc::c_int = 4;
pub const OPT_DISC_FLAT: libc::c_int = 8;
pub const OPT_DISC_TWIN_CAND: libc::c_int = 16;
pub const OPT_DISC_RANKING: libc::c_int = 48;
pub const OPT_DISC_NO_WO_TO: libc::c_int = 64;

pub const OPT_BASELINE_NORMAL: libc::c_int = 1;
pub const OPT_BASELINE_COOK: libc::c_int = 2;

pub const OPT_PARA_MULTIPLY_ALL_EX: libc::c_int = 1;
pub const OPT_PARA_GENERATE_SIMILARITY: libc::c_int = 2;
pub const OPT_PARA_SYNCHRONIZE: libc::c_int = 4;
pub const OPT_PARA_MULTIPLY_AVE_EX: libc::c_int = 8;

pub const IS_BNST_DATA: libc::c_int = 1;
pub const IS_TAG_DATA: libc::c_int = 2;
pub const IS_MRPH_DATA: libc::c_int = 4;

pub const PP_NUMBER: libc::c_int = 44;
pub const LOC_NUMBER: libc::c_int = 21;
pub const UTYPE_NUMBER: libc::c_int = 12;
pub const NE_MODEL_NUMBER: libc::c_int = 33;

pub const DT_RULE_NUM_MAX: libc::c_int = 1000;
pub const FEATURE_MAX: libc::c_int = 1024;

pub const PARA_KEY_O: libc::c_int = 0;
pub const PARA_KEY_N: libc::c_int = 1;  /* 体言の並列 */
pub const PARA_KEY_P: libc::c_int = 2;  /* 用言の並列 */
pub const PARA_KEY_A: libc::c_int = 4;  /* 体言か用言か分からない並列 */
pub const PARA_KEY_I: libc::c_int = 3;  /* GAPのある並列 ？？ */

pub const PRi32_PARA: libc::c_int = 0;
pub const PRi32_DPND: libc::c_int = 1;
pub const PRi32_MASK: libc::c_int = 2;
pub const PRi32_QUOTE: libc::c_int = 3;
pub const PRi32_RSTR: libc::c_int = 4;
pub const PRi32_RSTD: libc::c_int = 5;
pub const PRi32_RSTQ: libc::c_int = 6;

pub const SEMANTIC_MARKER: libc::c_int = 1;
pub const EXAMPLE: libc::c_int = 2;

pub const VOICE_SHIEKI: libc::c_int = 1;
pub const VOICE_UKEMI: libc::c_int = 2;
pub const VOICE_SHIEKI_UKEMI: libc::c_int = 4;
pub const VOICE_MORAU: libc::c_int = 8;
pub const VOICE_HOSHII: libc::c_int = 16;
pub const VOICE_UNKNOWN: libc::c_int = 32;

pub const FRAME_ACTIVE: libc::c_int = 1;
pub const FRAME_PASSIVE_I: libc::c_int = 2;
pub const FRAME_PASSIVE_1: libc::c_int = 3;
pub const FRAME_PASSIVE_2: libc::c_int = 4;
pub const FRAME_CAUSATIVE_WO_NI: libc::c_int = 5;
pub const FRAME_CAUSATIVE_WO: libc::c_int = 6;
pub const FRAME_CAUSATIVE_NI: libc::c_int = 7;
pub const FRAME_CAUSATIVE_PASSIVE: libc::c_int = 8;

pub const FRAME_POSSIBLE: libc::c_int = 9;
pub const FRAME_POLITE: libc::c_int = 10;
pub const FRAME_SPONTANE: libc::c_int = 11;

pub const CF_CAUSATIVE_WO: libc::c_int = 1;
pub const CF_CAUSATIVE_NI: libc::c_int = 2;
pub const CF_PASSIVE_1: libc::c_int = 4;
pub const CF_PASSIVE_2: libc::c_int = 8;
pub const CF_PASSIVE_I: libc::c_int = 16;
pub const CF_POSSIBLE: libc::c_int = 32;
pub const CF_POLITE: libc::c_int = 64;
pub const CF_SPONTANE: libc::c_int = 128;

pub const UNASSIGNED: libc::c_int = -1;
pub const NIL_ASSIGNED: libc::c_int = -2;

pub const NIL_ASSINED_SCORE: libc::c_int = -20;
pub const FREQ0_ASSINED_SCORE: f32 = -13.815511; /* log(0.0000010) */
pub const UNKNOWN_CASE_SCORE: f32 = -11.512925; /* log(0.0000100) */
pub const UNKNOWN_CF_SCORE: f32 = -11.512925; /* log(0.0000100) */
pub const UNKNOWN_RENYOU_SCORE: f32 = -16.118096; /* log(0.0000001) */

pub const CASE_MATCH_FAILURE_SCORE: libc::c_int = -2;
pub const CASE_MATCH_FAILURE_PROB: libc::c_int = -1001;

pub const END_M: libc::c_int = -10;

pub const CONTINUE: libc::c_int = -1;
pub const GUARD: char = '\n';

pub const TYPE_KATAKANA: libc::c_int = 1;
pub const TYPE_HIRAGANA: libc::c_int = 2;
pub const TYPE_KANJI: libc::c_int = 4;
pub const TYPE_SUUJI: libc::c_int = 8;
pub const TYPE_EIGO: libc::c_int = 16;
pub const TYPE_KIGOU: libc::c_int = 32;
pub const TYPE_PUNC: libc::c_int = 64;

pub const SM_NO_EXPAND_NE: libc::c_int = 1;
pub const SM_EXPAND_NE: libc::c_int = 2;
pub const SM_CHECK_FULL: libc::c_int = 3;
pub const SM_EXPAND_NE_DATA: libc::c_int = 4;

pub const RLOOP_MRM: libc::c_int = 0;
pub const RLOOP_RMM: libc::c_int = 1;

pub const RLOOP_BREAK_NONE: libc::c_int = 0;
pub const RLOOP_BREAK_NORMAL: libc::c_int = 1;
pub const RLOOP_BREAK_JUMP: libc::c_int = 2;

pub const LtoR: libc::c_int = 1;
pub const RtoL: libc::c_int = -1;

pub const CF_DECIDE_THRESHOLD: libc::c_int = 7;
pub const DEFAULT_SOTO_THRESHOLD: libc::c_int = 8;

pub const PARA_NIL: libc::c_int = 0;
pub const PARA_NORMAL: libc::c_int = 1;  /* <P> */
pub const PARA_INCOMP: libc::c_int = 2;  /* <I> */

pub const REL_NOT: libc::c_int = 0; /* 重なりなし */
pub const REL_BIT: libc::c_int = 1; /* 少し重なる */
pub const REL_PRE: libc::c_int = 2; /* 前で重なる */
pub const REL_POS: libc::c_int = 3; /* 後で重なる */
pub const REL_PAR: libc::c_int = 4; /* 重複 */
pub const REL_REV: libc::c_int = 5; /* 前部の修正 */
pub const REL_IN1: libc::c_int = 6; /* 含まれる前  */
pub const REL_IN2: libc::c_int = 7; /* 含まれる後  */
pub const REL_BAD: libc::c_int = 8; /* 誤り */

pub const STAND_ALONE_MODE: libc::c_int = 0;
pub const SERVER_MODE: libc::c_int = 1;
pub const CLIENT_MODE: libc::c_int = 2;

pub const DEFAULT_PORT: libc::c_int = 31000;
pub const EOf: libc::c_int = 0x0b;

pub const KNP_SERVER_USER: &str = "nobody";
pub const KNP_PIDFILE: &str = "/var/run/knp.pid";

pub const RF_MAX: libc::c_int = 16;

pub const NOT_FLG: char = '^';
pub const MAT_FLG: char = '\0';
pub const AST_FLG: char = '*';
pub const QST_FLG: char = '?';
pub const NOT_STR: char = '^';
pub const AST_STR: char = '*';
pub const QST_STR: char = '?';
pub const FW_MATCHING: libc::c_int = 0;
pub const BW_MATCHING: libc::c_int = 1;
pub const ALL_MATCHING: libc::c_int = 0;
pub const PART_MATCHING: libc::c_int = 1;
pub const SHORT_MATCHING: libc::c_int = 0;
pub const LONG_MATCHING: libc::c_int = 1;

pub const RM_HINSHI_MAX: libc::c_int = 64;
pub const RM_BUNRUI_MAX: libc::c_int = 64;
pub const RM_KATA_MAX: libc::c_int = 64;
pub const RM_KEI_MAX: libc::c_int = 64;
pub const RM_GOI_MAX: libc::c_int = 64;

pub const LOOP_BREAK: libc::c_int = 0;
pub const LOOP_ALL: libc::c_int = 1;
pub const QUOTE_MAX: libc::c_int = 40;

/* KNP のルールファイル指定用 (.knprc) */
pub const DEF_JUMAN_GRAM_FILE: &str = "JUMAN文法ディレクトリ";

pub const DEF_KNP_FILE: &str = "KNPルールファイル";
pub const DEF_KNP_DIR: &str = "KNPルールディレクトリ";
pub const DEF_KNP_DICT_DIR: &str = "KNP辞書ディレクトリ";
pub const DEF_KNP_DICT_FILE: &str = "KNP辞書ファイル";

pub const DEF_THESAURUS: &str = "KNPシソーラス";
pub const DEF_CASE_THESAURUS: &str = "KNP格解析シソーラス";
pub const DEF_PARA_THESAURUS: &str = "KNP並列解析シソーラス";

pub const DEF_AUTO_DIC_FEATURES: &str = "KNP自動獲得辞書適用属性";

pub const DEF_DISC_CASES: &str = "KNP省略解析格";
pub const DEF_DISC_ORDER: &str = "KNP省略解析探索範囲";

pub const DEF_SVM_MODEL_FILE: &str = "SVMモデルファイル";
pub const DEF_DT_MODEL_FILE: &str = "決定木ファイル";

pub const DEF_SVM_FREQ_SD: &str = "SVM頻度標準偏差";
pub const DEF_SVM_FREQ_SD_NO: &str = "SVM頻度標準偏差ノ格";

pub const DEF_SVM_REFERRED_NUM_SURFACE_SD: &str = "SVM表層参照回数標準偏差";
pub const DEF_SVM_REFERRED_NUM_ELLIPSIS_SD: &str = "SVM省略参照回数標準偏差";

pub const DEF_DISC_LOC_ORDER: &str = "KNP省略解析探索順序";
pub const DEF_DISC_SEN_NUM: &str = "KNP省略解析探索文数";

pub const DEF_ANTECEDENT_DECIDE_TH: &str = "KNP省略解析探索閾値";

pub const DEF_NE_MODEL_DIR: &str = "NEモデルファイルディレクトリ"; /* SVM */
pub const DEF_NE_MODEL_FILE: &str = "NEモデルファイル"; /* CRF */
pub const DEF_SYNONYM_FILE: &str = "同義表現ファイル";
pub const DEF_DISTSIM_FILE: &str = "分布類似度ファイル";

pub const RuleIncrementStep: libc::c_int = 10;

/* 読み込み方法 */
pub const MorphRuleType: libc::c_int = 1;
pub const BnstRuleType: libc::c_int = 2;
pub const HomoRuleType: libc::c_int = 3;
pub const DpndRuleType: libc::c_int = 4;
pub const KoouRuleType: libc::c_int = 5;
pub const NeMorphRuleType: libc::c_int = 6;
pub const NePhrasePreRuleType: libc::c_int = 7;
pub const NePhraseRuleType: libc::c_int = 8;
pub const NePhraseAuxRuleType: libc::c_int = 9;
pub const ContextRuleType: libc::c_int = 10;
pub const TagRuleType: libc::c_int = 11;
pub const AfterDpndBnstRuleType: libc::c_int = 12;
pub const AfterDpndTagRuleType: libc::c_int = 13;
pub const PostProcessTagRuleType: libc::c_int = 14;
pub const CaseFrameRuleType: libc::c_int = 15;
pub const PreProcessMorphRuleType: libc::c_int = 16;

/* 辞書の最大数 */
pub const DICT_MAX: libc::c_int = 39;

/* 辞書の定義 */
pub const BGH_DB: libc::c_int = 1;
pub const SM_DB: libc::c_int = 2;
pub const SM2CODE_DB: libc::c_int = 3;
pub const SMP2SMG_DB: libc::c_int = 4;
pub const SCASE_DB: libc::c_int = 5;
pub const CF_INDEX_DB: libc::c_int = 6;
pub const CF_DATA: libc::c_int = 7;
pub const PROPER_DB: libc::c_int = 8;
pub const PROPERC_DB: libc::c_int = 9;
pub const PROPERCASE_DB: libc::c_int = 10;
pub const CODE2SM_DB: libc::c_int = 12;
pub const EVENT_DB: libc::c_int = 13;
pub const CF_NOUN_INDEX_DB: libc::c_int = 14;
pub const CF_NOUN_DATA: libc::c_int = 15;
pub const CF_SIM_DB: libc::c_int = 16;
pub const CF_CASE_DB: libc::c_int = 17;
pub const CF_EX_DB: libc::c_int = 18;
pub const CASE_DB: libc::c_int = 19;
pub const CFP_DB: libc::c_int = 20;
pub const RENYOU_DB: libc::c_int = 21;
pub const ADVERB_DB: libc::c_int = 22;
pub const PARA_DB: libc::c_int = 23;
pub const NOUN_CO_DB: libc::c_int = 24;
pub const CHI_DPND_DB: libc::c_int = 25;
pub const AUTO_DIC_DB: libc::c_int = 26;
pub const HOWNET_DEF_DB: libc::c_int = 27;
pub const HOWNET_TRAN_DB: libc::c_int = 28;
pub const HOWNET_ANTONYM_DB: libc::c_int = 29;
pub const HOWNET_CATEGORY_DB: libc::c_int = 30;
pub const HOWNET_SEM_DEF_DB: libc::c_int = 31;
pub const CHI_PA_DB: libc::c_int = 32;
pub const CHI_DPND_PROB_DB: libc::c_int = 33;
pub const CHI_DIS_COMMA_DB: libc::c_int = 34;
pub const CHI_CASE_DB: libc::c_int = 35;
pub const CHI_POS_DB: libc::c_int = 36;
pub const NV_MI_DB: libc::c_int = 37;
pub const MRPH2ID_DB: libc::c_int = 38;
pub const SOTO_TXT: libc::c_int = 39;

/* シソーラスの最大数 */
pub const THESAURUS_MAX: libc::c_int = 3;

/* 指定できる自動獲得辞書属性の最大数 */
pub const AUTO_DIC_FEATURES_MAX: libc::c_int = 10;

pub const CASE_MAX_NUM: libc::c_int = 20;
pub const CASE_TYPE_NUM: libc::c_int = 50;

pub const USE_NONE: libc::c_int = -1;
pub const USE_BGH: libc::c_int = 1;
pub const USE_NTT: libc::c_int = 2;
pub const STOREtoCF: libc::c_int = 4;
pub const USE_BGH_WITH_STORE: libc::c_int = 5;
pub const USE_NTT_WITH_STORE: libc::c_int = 6;
pub const USE_SUFFIX_SM: libc::c_int = 8;
pub const USE_PREFIX_SM: libc::c_int = 16;
pub const USE_RN: libc::c_int = 32;
pub const USE_BGH_WITH_RN: libc::c_int = 33;
pub const USE_NTT_WITH_RN: libc::c_int = 34;
pub const USE_DISTSIM: libc::c_int = 64;

pub const CF_PRED: libc::c_int = 1;
pub const CF_NOUN: libc::c_int = 2;

pub const CF_NORMAL: libc::c_int = 0;
pub const CF_SUM: libc::c_int = 1;  /* OR の格フレーム */
pub const CF_GA_SEMI_SUBJECT: libc::c_int = 2;
pub const CF_CHANGE: libc::c_int = 4;

pub const CF_UNDECIDED: libc::c_int = 0;
pub const CF_CAND_DECIDED: libc::c_int = 1;
pub const CF_DECIDED: libc::c_int = 2;

pub const MATCH_SUBJECT: libc::c_int = -1;
pub const MATCH_NONE: libc::c_int = -2;

pub const CREL: libc::c_int = 1;  /* 格関係 */
pub const EREL: libc::c_int = 2;  /* 省略関係 */

pub const ELLIPSIS_TAG_UNSPECIFIED_PEOPLE: libc::c_int = -2;  /* 不特定:人 */
pub const ELLIPSIS_TAG_I_WE: libc::c_int = -3;  /* 1人称 */
pub const ELLIPSIS_TAG_UNSPECIFIED_CASE: libc::c_int = -4;  /* 不特定:状況 */
pub const ELLIPSIS_TAG_PRE_SENTENCE: libc::c_int = -5;  /* 前文 */
pub const ELLIPSIS_TAG_POST_SENTENCE: libc::c_int = -6;  /* 後文 */
pub const ELLIPSIS_TAG_EXCEPTION: libc::c_int = -7;  /* 対象外 */
pub const FUNCTIONAL_TAG_MAX: libc::c_int = 5;
/* CF_TAG_MGR中のomit_feature用の定数 */
pub const ELLIPSIS_CASE_NUM: libc::c_int = 4;
pub const NO_ASSIGNMENT: libc::c_int = 0; /* ある格スロットが対応付けられない確率 */
pub const EX_PMI: libc::c_int = 1; /* 語PMI */
pub const CEX_PMI: libc::c_int = 2; /* カテゴリPMI */
pub const NEX_PMI: libc::c_int = 3; /* 固有表現PMI */
pub const CLS_PMI: libc::c_int = 4; /* クラスPMI */
pub const WA_IN_THE_SENT: libc::c_int = 5; /* 先行詞候補が同一文に出現かつ格助詞「は」を伴う */
pub const MAX_PMI: libc::c_int = 6; /*PMIで最大のもの*/
//pub const SALIENCE_CHECK: libc::c_int = 6; /* SALIENCEが一定以上あるかどうか */
pub const NE_PERSON: libc::c_int = 7; /* 先行詞候補が人名 */
pub const NE_CHECK: libc::c_int = 8; /* 先行詞候補がNE*/
pub const ASSIGNED: libc::c_int = 9; /* 埋まったかどうか */
pub const OVERT_APPEARANCE: libc::c_int = 10; /* 文章全体の出現回数 */
pub const BEFORE_OVERT_APPEARANCE: libc::c_int = 11; /* 前の文での出現回数 */
pub const AFTER_OVERT_APPEARANCE: libc::c_int = 12; /* 後の文での出現回数 */
pub const SAME_PRED: libc::c_int = 13; /*同じ動詞+格での出現*/
pub const ASSIGNMENT: libc::c_int = 14; /*ある格スロットが対応付けられる確率*/
pub const AUTHOR_SCORE: libc::c_int = 15;
pub const READER_SCORE: libc::c_int = 16;
pub const EX_CASE_PROB: libc::c_int = 17; /* 語が格スロットに入る確率 */
pub const EX_PROB: libc::c_int = 18; /* 語が出現する確率 */
pub const CEX_CASE_PROB: libc::c_int = 19; /* カテゴリPMI */
pub const CEX_PROB: libc::c_int = 20; /* カテゴリPMI */
pub const NEX_CASE_PROB: libc::c_int = 21; /* 固有表現PMI */
pub const NEX_PROB: libc::c_int = 22; /* 固有表現PMI */
pub const CLS_CASE_PROB: libc::c_int = 23; /* クラスPMI */
pub const CLS_PROB: libc::c_int = 24; /* クラスPMI */
pub const ALT_CT_PMI: libc::c_int = 25;
pub const ALT_CT_CASE_PROB: libc::c_int = 26;
pub const ALT_CT_PROB: libc::c_int = 27;
pub const CF_GA_FILLED_RATIO: libc::c_int = 28;
pub const EX_ASSIGNMENT: libc::c_int = 29; /*用例の割合からの埋まりやすさ*/
pub const OLD_TOPIC: libc::c_int = 30; /*「は」で出現したが以降で別の「は」が出現*/
pub const ABILITY: libc::c_int = 31;
pub const UNNAMED_CASE_PROB: libc::c_int = 32;
pub const VOICE_S: libc::c_int = 33;
pub const CLOSEST_APPEARANCE_S: libc::c_int = VOICE_S + VOICE_NUM;
pub const CLOSEST_APPEARANCE_NUM: libc::c_int = 4;
pub const PRED_DPND_TYPE_S: libc::c_int = CLOSEST_APPEARANCE_NUM + CLOSEST_APPEARANCE_S;
pub const PRED_DPND_TYPE_NUM: libc::c_int = 3;
pub const VERB_SITUATION_S: libc::c_int = PRED_DPND_TYPE_S + PRED_DPND_TYPE_NUM;
pub const VERB_SITUATION_NUM: libc::c_int = 2;
pub const NE_FEATURE_S: libc::c_int = VERB_SITUATION_S + VERB_SITUATION_NUM;
pub const NE_FEATURE_NUM: libc::c_int = NE_NUM;
pub const LOCATION_S: libc::c_int = NE_FEATURE_S + NE_FEATURE_NUM;  /* 位置カテゴリ素性の開始位置 */
pub const LOCATION_NUM: libc::c_int = 135; /* 位置カテゴリ素性の種類 + simple_loc_num */
pub const SIMPLE_LOCATION_S: libc::c_int = LOCATION_S + LOCATION_NUM * 3;
pub const SIMPLE_LOCATION_NUM: libc::c_int = 15;
pub const UNNAMED_NUM_S: libc::c_int = SIMPLE_LOCATION_S + SIMPLE_LOCATION_NUM * 3;
pub const YOBIKAKE_S: libc::c_int = UNNAMED_NUM_S + UNNAMED_ENTITY_NUM * ELLIPSIS_CASE_NUM;
pub const MODALITY_S: libc::c_int = YOBIKAKE_S + UNNAMED_ENTITY_NUM;
pub const MODALITY_F_NUM: libc::c_int = MODALITY_NUM * 2;
pub const KEIGO_S: libc::c_int = MODALITY_S + MODALITY_F_NUM;
pub const KEIGO_F_NUM: libc::c_int = KEIGO_NUM * 2;
pub const TENSE_S: libc::c_int = KEIGO_S + KEIGO_F_NUM;
pub const TENSE_NUM: libc::c_int = 4;
pub const CONTEXT_S: libc::c_int = TENSE_S + TENSE_NUM;
pub const CONTEXT_FEATURE_NUM: libc::c_int = 10;
pub const EACH_FEARUTE_NUM: libc::c_int = CONTEXT_S + CONTEXT_FEATURE_NUM;

pub const NE_NUM: libc::c_int = 8;

pub const REAL_S: libc::c_int = EACH_FEARUTE_NUM; /*普通の素性*/
pub const FILLED_S: libc::c_int = EACH_FEARUTE_NUM * 2; /*省略でない部分*/
pub const UNNAMED_S: libc::c_int = EACH_FEARUTE_NUM * 3; /*UNNAMED_ENTITYの素性*/
pub const ADJ_FEATURE_S: libc::c_int = EACH_FEARUTE_NUM * (3 + UNNAMED_ENTITY_NUM);
pub const O_FEATURE_NUM: libc::c_int = 2 * EACH_FEARUTE_NUM * (3 + UNNAMED_ENTITY_NUM);


pub const UNNAMED_ENTITY_NUM: libc::c_int = 5;
pub const MODALITY_NUM: libc::c_int = 11;
pub const VOICE_NUM: libc::c_int = 5;
pub const KEIGO_NUM: libc::c_int = 3;
pub const SENTENCE_CATEGORY_NUM: libc::c_int = 4; //共通、1文目、2、3文目、それ以降
