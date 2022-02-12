#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]


use libc;

use crate::{_FEATURE, atof, atoi, BNST_DATA, Class, fclose, FEATURE, fopen, fprintf, fputs, free, memset, MRPH_DATA, sprintf, sscanf, strcat, strchr, strcmp, strcpy, strdup, strlen, strncmp, strstr, TAG_DATA, tnode_b, TOTAL_MGR};
use crate::case_analysis::{CF_MatchPP, get_dist_from_work_mgr, init_case_frame, MatchPP, MatchPP2, pp_code_to_kstr, pp_kstr_to_code, realloc_cmm};
use crate::case_data::set_pred_voice;
use crate::case_match::{cf_match_element, sms_match};
use crate::case_print::EX_PRINT_NUM;
use crate::configfile::open_dict;
use crate::consts::{VERBOSE1, VERBOSE2, VERBOSE3};
use crate::ctools::{assign_cfeature, cdb_unpack, check_dict_filename, check_feature, current_sentence_data, exit, exp, fread, fseek, fseeko, ftell, get_mrph_rep_from_f, Language, log, malloc, malloc_data, OptAnalysis, Outfp, read, realloc, stderr, strncat, strncpy, strtok};
use crate::db::{db_close, db_get, db_read_open};
use crate::feature::{check_category, check_str_type};
use crate::lib_sm::{code2sm, sm2code};
use crate::read_data::{get_bnst_head_canonical_rep, get_mrph_rep, get_mrph_rep_length, make_mrph_rn};
use crate::structs::{CDB_FILE, CF_ALIGNMENT, CF_CASE_SLOT, cf_frame_def};
use crate::thesaurus::get_str_code;
use crate::tools::{DICT, hash, OptAnaphora, OptCaseFlag, OptDisplay, OptEllipsis, OptGeneralCF, OptParaFix, OptParaNoFixFlag, OptUseCF, OptUseCPNCF, OptUseNCF, realloc_data, smlist, Thesaurus, VerboseLevel};
use crate::types::{__off_t, CASE_FRAME, CF_FRAME, CF_PRED_MGR, CF_ptr, CKY, DBM_FILE, FILE, SENTENCE_DATA, size_t, SMLIST};

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
pub static mut cf_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut cf_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut cf_noun_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut cf_noun_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut cf_sim_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut cf_case_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut cf_ex_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut case_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut cfp_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut renyou_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut adverb_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut para_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut noun_co_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut chi_pa_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut mrph2id_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut Case_frame_array: *mut CASE_FRAME = 0 as *const CASE_FRAME as *mut CASE_FRAME;
/* 格フレーム */
#[no_mangle]
pub static mut Case_frame_num: libc::c_int = 0;
/* 格フレーム数 */
#[no_mangle]
pub static mut MAX_Case_frame_num: libc::c_int = 0 as libc::c_int;
/* 最大格フレーム数 */
#[no_mangle]
pub static mut GENERAL_SOTO_WORDS: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut db_buf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut db_buf_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut CF_frame: CF_FRAME = CF_FRAME {
    address: 0,
    cyomi: 0 as *const libc::c_char as *mut libc::c_char,
    hyoki: 0 as *const libc::c_char as *mut libc::c_char,
    feature: 0 as *const libc::c_char as *mut libc::c_char,
    pred_type: [0; 4],
    voice: 0,
    etcflag: 0,
    casenum: 0,
    cs: [CF_CASE_SLOT {
        kaku_keishiki: 0 as *const libc::c_char as *mut libc::c_char,
        meishiku: 0 as *const libc::c_char as *mut libc::c_char,
        imisosei: 0 as *const libc::c_char as *mut libc::c_char,
    }; 20],
    samecase: [[0; 2]; 24],
    cf_align: [CF_ALIGNMENT {
        cf_id: 0 as *const libc::c_char as *mut libc::c_char,
        aligned_case: [[0; 2]; 24],
    };
        5
    ],
    DATA: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    next: 0 as *const cf_frame_def as *mut cf_frame_def,
};
#[no_mangle]
pub static mut MAX_cf_frame_length: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cf_str_buf: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut CFExist: libc::c_int = 0;
#[no_mangle]
pub static mut CFNounExist: libc::c_int = 0;
#[no_mangle]
pub static mut CFSimExist: libc::c_int = 0;
#[no_mangle]
pub static mut CFCaseExist: libc::c_int = 0;
#[no_mangle]
pub static mut CFExExist: libc::c_int = 0;
#[no_mangle]
pub static mut CaseExist: libc::c_int = 0;
#[no_mangle]
pub static mut CfpExist: libc::c_int = 0;
#[no_mangle]
pub static mut RenyouExist: libc::c_int = 0;
#[no_mangle]
pub static mut AdverbExist: libc::c_int = 0;
#[no_mangle]
pub static mut ParaExist: libc::c_int = 0;
#[no_mangle]
pub static mut NounCoExist: libc::c_int = 0;
#[no_mangle]
pub static mut CHISpecPAExist: libc::c_int = 0;
#[no_mangle]
pub static mut CHIPAExist: libc::c_int = 0;
#[no_mangle]
pub static mut Mrph2idExist: libc::c_int = 0;
#[no_mangle]
pub static mut PrintDeletedSM: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut SM_AGENT_THRESHOLD: libc::c_int = 0.40f64 as libc::c_int;
#[no_mangle]
pub static mut ClassProb: [libc::c_double; 2000] = [0.; 2000];
#[no_mangle]
pub static mut CFcache: [*mut CF_FRAME; 1024] =
    [0 as *const CF_FRAME as *mut CF_FRAME; 1024];
#[no_mangle]
pub static mut static_buffer: [libc::c_char; 5120] = [0; 5120];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_cf_structure(mut p: *mut CASE_FRAME,
                                           mut size: libc::c_int)
/*==================================================================*/
{
    memset(p as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<CASE_FRAME>() as
               libc::c_ulong).wrapping_mul(size as libc::c_ulong));
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn realloc_cf()
/*==================================================================*/
{
    Case_frame_array =
        realloc_data(Case_frame_array as *mut libc::c_void,
                     (::std::mem::size_of::<CASE_FRAME>() as
                         libc::c_ulong).wrapping_mul((MAX_Case_frame_num +
                         1024 as
                             libc::c_int) as
                         libc::c_ulong),
                     b"realloc_cf\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char) as *mut CASE_FRAME;
    init_cf_structure(Case_frame_array.offset(MAX_Case_frame_num as isize),
                      1024 as libc::c_int);
    MAX_Case_frame_num += 1024 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_cf()
/*==================================================================*/
{
    let mut index_db_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*DICT.as_mut_ptr().offset(7 as libc::c_int as isize)).is_null() {
        data_filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(7 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        data_filename =
            check_dict_filename(b"ebcf/cf.dat\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if !(*DICT.as_mut_ptr().offset(6 as libc::c_int as isize)).is_null() {
        index_db_filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(6 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        index_db_filename =
            check_dict_filename(b"ebcf/cf.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                data_filename);
    }
    cf_fp =
        fopen(data_filename, b"rb\x00" as *const u8 as *const libc::c_char);
    if cf_fp.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        CFExist = 0 as libc::c_int
    } else {
        cf_db = db_read_open(index_db_filename);
        if cf_db.is_null() {
            if OptDisplay == 3 as libc::c_int {
                fprintf(Outfp,
                        b"done.\nOpening %s ... failed.\n\x00" as *const u8 as
                            *const libc::c_char, index_db_filename);
            }
            fprintf(stderr,
                    b";; Cannot open CF INDEX Database <%s>.\n\x00" as
                        *const u8 as *const libc::c_char, index_db_filename);
            /* 格フレーム DATA は読めるのに、DB が読めないときは終わる */
            exit(1 as libc::c_int);
        } else {
            if OptDisplay == 3 as libc::c_int {
                fprintf(Outfp,
                        b"done.\nOpening %s ... done.\n\x00" as *const u8 as
                            *const libc::c_char, index_db_filename);
            }
            CFExist = (0 as libc::c_int == 0) as libc::c_int
        }
    }
    free(data_filename as *mut libc::c_void);
    free(index_db_filename as *mut libc::c_void);
    /* 格フレーム類似度DB (cfsim.db) */
    cf_sim_db =
        open_dict(16 as libc::c_int,
                  b"ebcf/cfsim.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut CFSimExist);
    /* 格確率DB (cfcase.db) */
    cf_case_db =
        open_dict(17 as libc::c_int,
                  b"ebcf/cfcase.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut CFCaseExist);
    /* 用例確率DB (cfex.db) *
    cf_ex_db = open_dict(CF_EX_DB, CF_EX_DB_NAME, &CFExExist);
    */
    CFExExist = 0 as libc::c_int;
    /* 格フレーム選択確率DB (cfp.db) */
    cfp_db =
        open_dict(20 as libc::c_int,
                  b"ebcf/cfp.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut CfpExist);
    /* 格解釈確率DB (case.db) */
    case_db =
        open_dict(19 as libc::c_int,
                  b"ebcf/case.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut CaseExist);
    /* 連用確率DB (renyou.db) */
    renyou_db =
        open_dict(21 as libc::c_int,
                  b"ebcf/renyou.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut RenyouExist);
    /* 副詞確率DB (adverb.db) */
    adverb_db =
        open_dict(22 as libc::c_int,
                  b"ebcf/adverb.db\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut AdverbExist);
    if OptParaFix == 0 as libc::c_int {
        /* 並列確率DB (para.db) */
        para_db =
            open_dict(23 as libc::c_int,
                      b"ebcf/para.db\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, &mut ParaExist)
    }
    /* 名詞共起確率DB (noun_co.db) */
    noun_co_db =
        open_dict(24 as libc::c_int,
                  b"ebcf/noun_co.db\x00" as *const u8 as *const libc::c_char
                      as *mut libc::c_char, &mut NounCoExist);
    if Language == 2 as libc::c_int {
        /* Chinese CHI_PA DB (chi_pa.db) */
        chi_pa_db =
            open_dict(32 as libc::c_int,
                      b"ebcf/chi_pa.db\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      &mut CHIPAExist)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_mrph2id()
/*==================================================================*/
{
    /* 形態素IDマップDB (mrph2id.db) */
    mrph2id_db =
        open_dict(38 as libc::c_int,
                  b"ebcf/mrph2id.db\x00" as *const u8 as *const libc::c_char
                      as *mut libc::c_char, &mut Mrph2idExist);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_soto_txt()
/*==================================================================*/
{
    /* 外の関係ファイル (case_soto.txt) */
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*DICT.as_mut_ptr().offset(39 as libc::c_int as isize)).is_null() {
        filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(39 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        filename =
            check_dict_filename(b"ebcf/case_soto.txt\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    let mut fp: *mut FILE =
        fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let size: size_t = ftell(fp) as size_t;
    fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    GENERAL_SOTO_WORDS = malloc(size) as *mut libc::c_char;
    fread(GENERAL_SOTO_WORDS as *mut libc::c_void,
          1 as libc::c_int as libc::c_ulong, size, fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_noun_cf()
/*==================================================================*/
{
    let mut index_db_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*DICT.as_mut_ptr().offset(15 as libc::c_int as isize)).is_null() {
        data_filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(15 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        data_filename =
            check_dict_filename(b"ebcf/noun.dat\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if !(*DICT.as_mut_ptr().offset(14 as libc::c_int as isize)).is_null() {
        index_db_filename =
            check_dict_filename(*DICT.as_mut_ptr().offset(14 as libc::c_int as
                isize),
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        index_db_filename =
            check_dict_filename(b"ebcf/noun.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... \x00" as *const u8 as *const libc::c_char,
                data_filename);
    }
    cf_noun_fp =
        fopen(data_filename, b"rb\x00" as *const u8 as *const libc::c_char);
    if cf_noun_fp.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char,
                  Outfp);
        }
        CFNounExist = 0 as libc::c_int
    } else {
        cf_noun_db = db_read_open(index_db_filename);
        if cf_noun_db.is_null() {
            if OptDisplay == 3 as libc::c_int {
                fprintf(Outfp,
                        b"done.\nOpening %s ... failed.\n\x00" as *const u8 as
                            *const libc::c_char, index_db_filename);
            }
            fprintf(stderr,
                    b";; Cannot open CF(noun) INDEX Database <%s>.\n\x00" as
                        *const u8 as *const libc::c_char, index_db_filename);
            /* 格フレーム DATA は読めるのに、DB が読めないときは終わる */
            exit(1 as libc::c_int);
        } else {
            if OptDisplay == 3 as libc::c_int {
                fprintf(Outfp,
                        b"done.\nOpening %s ... done.\n\x00" as *const u8 as
                            *const libc::c_char, index_db_filename);
            }
            CFNounExist = (0 as libc::c_int == 0) as libc::c_int
        }
    }
    free(data_filename as *mut libc::c_void);
    free(index_db_filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_mgr_cf(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 24 as libc::c_int {
            free((*(*sp).Best_mgr).cpm[i as usize].cf.ex[j as usize] as
                *mut libc::c_void);
            (*(*sp).Best_mgr).cpm[i as usize].cf.ex[j as usize] =
                0 as *mut libc::c_char;
            free((*(*sp).Best_mgr).cpm[i as usize].cf.sm[j as usize] as
                *mut libc::c_void);
            (*(*sp).Best_mgr).cpm[i as usize].cf.sm[j as usize] =
                0 as *mut libc::c_char;
            free(*(*(*sp).Best_mgr).cpm[i as
                usize].cf.ex_list[j as
                usize].offset(0
                as
                libc::c_int
                as
                isize)
                as *mut libc::c_void);
            free((*(*sp).Best_mgr).cpm[i as usize].cf.ex_list[j as usize] as
                *mut libc::c_void);
            free((*(*sp).Best_mgr).cpm[i as usize].cf.ex_freq[j as usize] as
                *mut libc::c_void);
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_mgr_cf(mut tmp: *mut TOTAL_MGR)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        init_case_frame(&mut (*(*tmp).cpm.as_mut_ptr().offset(i as
            isize)).cf);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_case_analysis_cpm(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int ||
        OptUseNCF != 0 {
        /* 格フレーム領域確保 */
        Case_frame_array =
            malloc_data((::std::mem::size_of::<CASE_FRAME>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"init_case_analysis_cpm\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut CASE_FRAME;
        MAX_Case_frame_num = 1024 as libc::c_int;
        init_cf_structure(Case_frame_array, MAX_Case_frame_num);
        /* Best_mgrのcpm領域確保 */
        init_mgr_cf((*sp).Best_mgr);
        /* 名詞-意味素HASHの初期化 */
        memset(smlist.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<SMLIST>() as
                   libc::c_ulong).wrapping_mul(1024 as libc::c_int as
                   libc::c_ulong));
        /* 格フレームcacheのHASHの初期化 */
        memset(CFcache.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<*mut CF_FRAME>() as
                   libc::c_ulong).wrapping_mul(1024 as libc::c_int as
                   libc::c_ulong));
        /* 格フレームすべてをメモリに読み込む場合 */
        if OptCaseFlag & 16384 as libc::c_int != 0 {
            list_db_and_register_caseframe(cf_db, 1 as libc::c_int);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_cf()
/*==================================================================*/
{
    if CFExist == (0 as libc::c_int == 0) as libc::c_int {
        fclose(cf_fp);
        db_close(cf_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_noun_cf()
/*==================================================================*/
{
    if CFNounExist == (0 as libc::c_int == 0) as libc::c_int {
        fclose(cf_noun_fp);
        db_close(cf_noun_db);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_ipal_address(mut word: *mut libc::c_uchar,
                                          mut flag: libc::c_int)
                                          -> *mut libc::c_char
/*==================================================================*/
{
    return if flag == 1 as libc::c_int {
        if CFExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
        db_get(cf_db, word as *mut libc::c_char)
    } else {
        if CFNounExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
        db_get(cf_noun_db, word as *mut libc::c_char)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cf_alignment(mut str: *mut libc::c_char,
                                          mut cf_aligned_num: libc::c_int)
/*==================================================================*/
{
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    buf = strdup(str);
    token = strtok(buf, b";\x00" as *const u8 as *const libc::c_char);
    CF_frame.cf_align[cf_aligned_num as usize].cf_id = strdup(token);
    token =
        strtok(0 as *mut libc::c_char,
               b";\x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        cp =
            strstr(token,
                   b"\xef\xbc\x9d\x00" as *const u8 as *const libc::c_char);
        if !cp.is_null() {
            *cp = '\u{0}' as i32 as libc::c_char;
            c1 = pp_kstr_to_code(token);
            c2 =
                pp_kstr_to_code(cp.offset(strlen(b"\xef\xbc\x9d\x00" as
                    *const u8 as
                    *const libc::c_char) as
                    isize));
            if c1 == -(10 as libc::c_int) {
                /* c2はNILのときEND_Mになる */
                if OptDisplay == 3 as libc::c_int {
                    fprintf(stderr,
                            b";; Can\'t understand <%s> as cf alignment\n\x00"
                                as *const u8 as *const libc::c_char, token);
                }
            } else if count < 24 as libc::c_int - 1 as libc::c_int {
                /* 溢れチェック */
                CF_frame.cf_align[cf_aligned_num as
                    usize].aligned_case[count as
                    usize][0 as
                    libc::c_int
                    as
                    usize]
                    = c1;
                CF_frame.cf_align[cf_aligned_num as
                    usize].aligned_case[count as
                    usize][1 as
                    libc::c_int
                    as
                    usize]
                    = c2;
                count += 1
            }
        }
        token =
            strtok(0 as *mut libc::c_char,
                   b";\x00" as *const u8 as *const libc::c_char)
    }
    free(buf as *mut libc::c_void);
    CF_frame.cf_align[cf_aligned_num as
        usize].aligned_case[count as
        usize][0 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
    CF_frame.cf_align[cf_aligned_num as
        usize].aligned_case[count as
        usize][1 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cf_frame(mut dst: *mut CF_FRAME,
                                       mut src: *mut CF_FRAME)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*dst).address = (*src).address;
    (*dst).yomi = strdup((*src).yomi);
    (*dst).hyoki = strdup((*src).hyoki);
    (*dst).feature = strdup((*src).feature);
    strcpy((*dst).pred_type.as_mut_ptr(), (*src).pred_type.as_mut_ptr());
    (*dst).voice = (*src).voice;
    (*dst).etcflag = (*src).etcflag;
    (*dst).casenum = (*src).casenum;
    i = 0 as libc::c_int;
    while i < (*dst).casenum {
        (*dst).cs[i as usize].kaku_keishiki =
            strdup((*src).cs[i as usize].kaku_keishiki);
        (*dst).cs[i as usize].meishiku =
            strdup((*src).cs[i as usize].meishiku);
        (*dst).cs[i as usize].imisosei =
            strdup((*src).cs[i as usize].imisosei);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        (*dst).samecase[i as usize][0 as libc::c_int as usize] =
            (*src).samecase[i as usize][0 as libc::c_int as usize];
        (*dst).samecase[i as usize][1 as libc::c_int as usize] =
            (*src).samecase[i as usize][1 as libc::c_int as usize];
        if (*src).samecase[i as usize][0 as libc::c_int as usize] ==
            -(10 as libc::c_int) {
            break;
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if (*src).cf_align[i as usize].cf_id.is_null() {
            (*dst).cf_align[i as usize].cf_id = 0 as *mut libc::c_char;
            break;
        } else {
            (*dst).cf_align[i as usize].cf_id =
                strdup((*src).cf_align[i as usize].cf_id);
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                (*dst).cf_align[i as
                    usize].aligned_case[j as
                    usize][0 as
                    libc::c_int
                    as
                    usize]
                    =
                    (*src).cf_align[i as
                        usize].aligned_case[j as
                        usize][0 as
                        libc::c_int
                        as
                        usize];
                (*dst).cf_align[i as
                    usize].aligned_case[j as
                    usize][1 as
                    libc::c_int
                    as
                    usize]
                    =
                    (*src).cf_align[i as
                        usize].aligned_case[j as
                        usize][1 as
                        libc::c_int
                        as
                        usize];
                if (*src).cf_align[i as
                    usize].aligned_case[j as
                    usize][0 as
                    libc::c_int
                    as
                    usize]
                    == -(10 as libc::c_int) {
                    break;
                }
                j += 1
            }
            i += 1
        }
    }
    (*dst).DATA =
        strdup((*src).DATA as *const libc::c_char) as *mut libc::c_uchar;
    /* actally, cf_id */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn register_caseframe(mut address: libc::c_ulonglong,
                                            mut i_ptr: *mut CF_FRAME)
/*==================================================================*/
{
    let mut num: libc::c_int = 0;
    let mut cfcmpp: *mut *mut CF_FRAME = 0 as *mut *mut CF_FRAME;
    let mut key: [libc::c_char; 128] = [0; 128];
    sprintf(key.as_mut_ptr(), b"%llu\x00" as *const u8 as *const libc::c_char,
            address);
    num =
        hash(key.as_mut_ptr() as *mut libc::c_uchar,
             strlen(key.as_mut_ptr()) as libc::c_int);
    cfcmpp =
        &mut *CFcache.as_mut_ptr().offset(num as isize) as *mut *mut CF_FRAME;
    while !(*cfcmpp).is_null() { cfcmpp = &mut (**cfcmpp).next }
    *cfcmpp =
        malloc_data(::std::mem::size_of::<CF_FRAME>() as libc::c_ulong,
                    b"register_caseframe\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut CF_FRAME;
    copy_cf_frame(*cfcmpp, i_ptr);
    (**cfcmpp).next = 0 as *mut cf_frame_def;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn lookup_caseframe(mut address: libc::c_ulonglong)
                                          -> *mut CF_FRAME
/*==================================================================*/
{
    let mut num: libc::c_int = 0;
    let mut cfcmp: *mut CF_FRAME = 0 as *mut CF_FRAME;
    let mut key: [libc::c_char; 128] = [0; 128];
    sprintf(key.as_mut_ptr(), b"%llu\x00" as *const u8 as *const libc::c_char,
            address);
    num =
        hash(key.as_mut_ptr() as *mut libc::c_uchar,
             strlen(key.as_mut_ptr()) as libc::c_int);
    cfcmp = CFcache[num as usize];
    while !cfcmp.is_null() {
        if address == (*cfcmp).address { return cfcmp; }
        cfcmp = (*cfcmp).next
    }
    return 0 as *mut CF_FRAME;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_ipal_frame(mut address: libc::c_ulonglong,
                                        mut size: libc::c_int,
                                        mut flag: libc::c_int)
                                        -> *mut CF_FRAME
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut cf_aligned_num: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if flag == 1 as libc::c_int { fp = cf_fp } else { fp = cf_noun_fp }
    if size > MAX_cf_frame_length {
        MAX_cf_frame_length +=
            1024 as libc::c_int *
                ((size - MAX_cf_frame_length) / 1024 as libc::c_int +
                    1 as libc::c_int);
        CF_frame.DATA =
            realloc_data(CF_frame.DATA as *mut libc::c_void,
                         (::std::mem::size_of::<libc::c_uchar>() as
                             libc::c_ulong).wrapping_mul(MAX_cf_frame_length
                             as
                             libc::c_ulong),
                         b"get_ipal_frame\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_uchar;
        cf_str_buf =
            realloc_data(cf_str_buf as *mut libc::c_void,
                         (::std::mem::size_of::<libc::c_uchar>() as
                             libc::c_ulong).wrapping_mul(MAX_cf_frame_length
                             as
                             libc::c_ulong),
                         b"get_ipal_frame\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_uchar
    }
    fseeko(fp, address as __off_t, 0 as libc::c_int);
    if fread(CF_frame.DATA as *mut libc::c_void, size as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong, fp) <
        1 as libc::c_int as libc::c_ulong {
        fprintf(stderr,
                b";; Error in fread.\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    CF_frame.address = address;
    /* 読み, 表記, 素性を設定 */
    CF_frame.casenum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < size - 1 as libc::c_int {
        if *CF_frame.DATA.offset(i as isize) as libc::c_int == '\u{0}' as i32
        {
            if count == 0 as libc::c_int {
                CF_frame.yomi =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char
            } else if count == 1 as libc::c_int {
                CF_frame.hyoki =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char
            } else if count == 2 as libc::c_int {
                CF_frame.feature =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char
            } else if count % 3 as libc::c_int == 0 as libc::c_int {
                CF_frame.cs[(count / 3 as libc::c_int - 1 as libc::c_int) as
                    usize].kaku_keishiki =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char;
                if CF_frame.casenum + 1 as libc::c_int > 20 as libc::c_int {
                    fprintf(stderr,
                            b";; # of cases is more than MAX (%d) for %llu.\n\x00"
                                as *const u8 as *const libc::c_char,
                            20 as libc::c_int, address);
                    break;
                } else { CF_frame.casenum += 1 }
            } else if count % 3 as libc::c_int == 1 as libc::c_int {
                CF_frame.cs[(count / 3 as libc::c_int - 1 as libc::c_int) as
                    usize].meishiku =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char
            } else if count % 3 as libc::c_int == 2 as libc::c_int {
                CF_frame.cs[(count / 3 as libc::c_int - 1 as libc::c_int) as
                    usize].imisosei =
                    CF_frame.DATA.offset(i as
                        isize).offset(1 as libc::c_int as
                        isize) as
                        *mut libc::c_char
            }
            count += 1
        }
        i += 1
    }
    count = 0 as libc::c_int;
    CF_frame.voice = 0 as libc::c_int;
    CF_frame.etcflag = 0 as libc::c_int;
    if *CF_frame.feature != 0 {
        let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        string = strdup(CF_frame.feature);
        token = strtok(string, b" \x00" as *const u8 as *const libc::c_char);
        while !token.is_null() {
            if strcmp(token,
                      b"\xe5\x92\x8c\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\x00"
                          as *const u8 as *const libc::c_char) == 0 {
                CF_frame.etcflag |= 1 as libc::c_int
            } else if strcmp(token,
                             b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe5\xa4\x89\xe5\x8c\x96\x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                CF_frame.etcflag |= 4 as libc::c_int
            } else if strcmp(token,
                             b"\xe3\x83\xb2\xe4\xbd\xbf\xe5\xbd\xb9\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                CF_frame.voice |= 1 as libc::c_int
            } else if strcmp(token,
                             b"\xe3\x83\x8b\xe4\xbd\xbf\xe5\xbd\xb9\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                CF_frame.voice |= 2 as libc::c_int
            } else if strcmp(token,
                             b"\xe7\x9b\xb4\xe5\x8f\x971\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 4 as libc::c_int
            } else if strcmp(token,
                             b"\xe7\x9b\xb4\xe5\x8f\x972\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 8 as libc::c_int
            } else if strcmp(token,
                             b"\xe9\x96\x93\xe5\x8f\x97\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 16 as libc::c_int
            } else if strcmp(token,
                             b"\xe5\x8f\xaf\xe8\x83\xbd\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 32 as libc::c_int
            } else if strcmp(token,
                             b"\xe5\xb0\x8a\xe6\x95\xac\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 64 as libc::c_int
            } else if strcmp(token,
                             b"\xe8\x87\xaa\xe7\x99\xba\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                CF_frame.voice |= 128 as libc::c_int
            } else if strncmp(token,
                              b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe5\xaf\xbe\xe5\xbf\x9c:\x00"
                                  as *const u8 as *const libc::c_char,
                              strlen(b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe5\xaf\xbe\xe5\xbf\x9c:\x00"
                                  as *const u8 as *const libc::c_char))
                == 0 {
                if cf_aligned_num < 5 as libc::c_int - 1 as libc::c_int {
                    let fresh0 = cf_aligned_num;
                    cf_aligned_num = cf_aligned_num + 1;
                    get_cf_alignment(token.offset(strlen(b"\xe6\xa0\xbc\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe5\xaf\xbe\xe5\xbf\x9c:\x00"
                        as *const u8 as
                        *const libc::c_char)
                        as isize), fresh0);
                }
            } else {
                /* merged cases */
                cp =
                    strstr(token,
                           b"\xef\xbc\x9d\x00" as *const u8 as
                               *const libc::c_char);
                if !cp.is_null() && flag == 1 as libc::c_int {
                    buf = strdup(token);
                    cp =
                        buf.offset(cp.wrapping_offset_from(token) as
                            libc::c_long as isize);
                    *cp = '\u{0}' as i32 as libc::c_char;
                    /* if (!strncmp(buf+strlen(buf)-2, "格", 2)) *(buf+strlen(buf)-2) = '\0';
		if (!strncmp(cp+strlen(cp+2), "格", 2)) *(cp+strlen(cp+2)) = '\0'; */
                    c1 = pp_kstr_to_code(buf);
                    c2 =
                        pp_kstr_to_code(cp.offset(strlen(b"\xef\xbc\x9d\x00"
                            as *const u8 as
                            *const libc::c_char)
                            as isize));
                    free(buf as *mut libc::c_void);
                    if c1 == -(10 as libc::c_int) ||
                        c2 == -(10 as libc::c_int) {
                        if OptDisplay == 3 as libc::c_int {
                            fprintf(stderr,
                                    b";; Can\'t understand <%s> as merged cases\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    token);
                        }
                    } else if count < 24 as libc::c_int - 1 as libc::c_int {
                        /* 溢れチェック */
                        /* 数が小さい格を前に入れる */
                        if c1 > c2 {
                            CF_frame.samecase[count as
                                usize][0 as libc::c_int as
                                usize] = c2;
                            CF_frame.samecase[count as
                                usize][1 as libc::c_int as
                                usize] = c1
                        } else {
                            CF_frame.samecase[count as
                                usize][0 as libc::c_int as
                                usize] = c1;
                            CF_frame.samecase[count as
                                usize][1 as libc::c_int as
                                usize] = c2
                        }
                        count += 1
                    }
                }
            }
            token =
                strtok(0 as *mut libc::c_char,
                       b" \x00" as *const u8 as *const libc::c_char)
        }
        free(string as *mut libc::c_void);
    }
    CF_frame.cf_align[cf_aligned_num as usize].cf_id = 0 as *mut libc::c_char;
    CF_frame.cf_align[cf_aligned_num as
        usize].aligned_case[0 as libc::c_int as
        usize][0 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
    CF_frame.cf_align[cf_aligned_num as
        usize].aligned_case[0 as libc::c_int as
        usize][1 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
    CF_frame.samecase[count as usize][0 as libc::c_int as usize] =
        -(10 as libc::c_int);
    CF_frame.samecase[count as usize][1 as libc::c_int as usize] =
        -(10 as libc::c_int);
    cp = strchr(CF_frame.DATA as *const libc::c_char, ':' as i32);
    if !cp.is_null() {
        strncpy(CF_frame.pred_type.as_mut_ptr(),
                cp.offset(1 as libc::c_int as isize),
                3 as libc::c_int as libc::c_ulong);
        CF_frame.pred_type[3 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    } else {
        CF_frame.pred_type[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    if OptCaseFlag & 8192 as libc::c_int != 0 {
        /* 格フレームcacheを使う場合は登録 */
        register_caseframe(address, &mut CF_frame);
    }
    return &mut CF_frame;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn extract_ipal_str(mut dat: *mut libc::c_uchar,
                                          mut ret: *mut libc::c_uchar,
                                          mut flag: libc::c_int)
                                          -> *mut libc::c_uchar
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    // let mut freq: libc::c_int = 0;
    /* flag == TRUE: 頻度付きで返す */
    if *dat as libc::c_int == '\u{0}' as i32 ||
        strcmp(dat as *const libc::c_char,
               b"\xef\xbc\x8a\x00" as *const u8 as *const libc::c_char) ==
            0 {
        return 0 as *mut libc::c_uchar;
    }
    loop {
        if *dat as libc::c_int == '\u{0}' as i32 {
            *ret = '\u{0}' as i32 as libc::c_uchar;
            return dat;
        } else {
            /* 頻度が記述してある場合 */
            if *dat as libc::c_int == ':' as i32 {
                if flag != 0 {
                    let fresh1 = ret;
                    ret = ret.offset(1);
                    *fresh1 = *dat
                } else {
                    /* flag == FALSE: 頻度を返さない */
                    let fresh2 = ret;
                    ret = ret.offset(1);
                    *fresh2 = '\u{0}' as i32 as libc::c_uchar
                    /* ':' -> '\0' */
                }
                dat = dat.offset(1)
            } else if *dat as libc::c_int == ' ' as i32 {
                *ret = '\u{0}' as i32 as libc::c_uchar;
                return dat.offset(1 as libc::c_int as isize);
            } else {
                if (*dat as libc::c_int) < 0x80 as libc::c_int {
                    /* 空白でも切る */
                    /* OK? -> ★UTF-8: 下のBYTES4CHARと併せて修正★ */
                    let fresh3 = dat;
                    dat = dat.offset(1);
                    let fresh4 = ret;
                    ret = ret.offset(1);
                    *fresh4 = *fresh3
                } else if strncmp(dat as *const libc::c_char,
                                  b"\xef\xbc\x8f\x00" as *const u8 as
                                      *const libc::c_char,
                                  strlen(b"\xef\xbc\x8f\x00" as *const u8 as
                                      *const libc::c_char)) == 0 ||
                    strncmp(dat as *const libc::c_char,
                            b"\xef\xbc\x8c\x00" as *const u8 as
                                *const libc::c_char,
                            strlen(b"\xef\xbc\x8c\x00" as *const u8
                                as *const libc::c_char)) == 0
                    ||
                    strncmp(dat as *const libc::c_char,
                            b"\xe3\x80\x81\x00" as *const u8 as
                                *const libc::c_char,
                            strlen(b"\xe3\x80\x81\x00" as *const u8
                                as *const libc::c_char)) == 0
                    ||
                    strncmp(dat as *const libc::c_char,
                            b"\xef\xbc\x8a\x00" as *const u8 as
                                *const libc::c_char,
                            strlen(b"\xef\xbc\x8a\x00" as *const u8
                                as *const libc::c_char)) == 0
                {
                    *ret = '\u{0}' as i32 as libc::c_uchar;
                    return dat.offset(strlen(b"\xef\xbc\x8f\x00" as *const u8
                        as *const libc::c_char) as
                        isize);
                    /* OK? */
                } else {
                    i = 0 as libc::c_int;
                    while i < 3 as libc::c_int {
                        let fresh5 = dat;
                        dat = dat.offset(1);
                        let fresh6 = ret;
                        ret = ret.offset(1);
                        *fresh6 = *fresh5;
                        i += 1
                    }
                }
            }
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_ipal_cframe_pp(mut c_ptr: *mut CASE_FRAME,
                                              mut cp: *mut libc::c_uchar,
                                              mut num: libc::c_int,
                                              mut flag: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    /* 助詞の読みだし */
    let mut point: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pp_num: libc::c_int = 0 as libc::c_int;
    /* 直前格 */
    if *cp.offset(strlen(cp as *const libc::c_char) as
        isize).offset(-(1 as libc::c_int as isize)) as
        libc::c_int == '*' as i32 {
        (*c_ptr).adjacent[num as usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        *cp.offset(strlen(cp as *const libc::c_char) as
            isize).offset(-(1 as libc::c_int as isize)) =
            '\u{0}' as i32 as libc::c_uchar
    } else if strcmp(cp.offset(strlen(cp as *const libc::c_char) as
        isize).offset(-(strlen(b"\xef\xbc\xa0\x00"
        as *const u8 as
        *const libc::c_char)
        as isize)) as
                         *const libc::c_char,
                     b"\xef\xbc\xa0\x00" as *const u8 as *const libc::c_char)
        == 0 {
        (*c_ptr).adjacent[num as usize] =
            (0 as libc::c_int == 0) as libc::c_int;
        *cp.offset(strlen(cp as *const libc::c_char) as
            isize).offset(-(strlen(b"\xef\xbc\xa0\x00" as *const u8
            as *const libc::c_char) as
            isize)) =
            '\u{0}' as i32 as libc::c_uchar
    }
    /* 任意格 */
    if strcmp(cp.offset(strlen(cp as *const libc::c_char) as
        isize).offset(-(strlen(b"\xef\xbc\x8a\x00" as
        *const u8 as
        *const libc::c_char) as
        isize)) as
                  *const libc::c_char,
              b"\xef\xbc\x8a\x00" as *const u8 as *const libc::c_char) == 0 {
        (*c_ptr).oblig[num as usize] = 0 as libc::c_int
    } else {
        (*c_ptr).oblig[num as usize] = (0 as libc::c_int == 0) as libc::c_int
    }
    point = cp;
    loop {
        point = extract_ipal_str(point, cf_str_buf, 0 as libc::c_int);
        if point.is_null() { break; }
        if flag == 1 as libc::c_int {
            (*c_ptr).pp[num as usize][pp_num as usize] =
                pp_kstr_to_code(cf_str_buf as *mut libc::c_char);
            if (*c_ptr).pp[num as usize][pp_num as usize] ==
                -(10 as libc::c_int) {
                /*                if (OptDisplay == OPT_DEBUG) {*/
                /*                    fprintf(stderr, ";; Unknown case (%s) in PP!\n", cf_str_buf);*/
                /*                }*/
                return 0 as libc::c_int;
            }
            (*c_ptr).pp_str[num as usize] = 0 as *mut libc::c_char
        } else {
            (*c_ptr).pp[num as usize][pp_num as usize] = 0 as libc::c_int;
            (*c_ptr).pp_str[num as usize] =
                strdup(cf_str_buf as *const libc::c_char);
            (*c_ptr).oblig[num as usize] =
                (0 as libc::c_int == 0) as libc::c_int
        }
        pp_num += 1
    }
    (*c_ptr).pp[num as usize][pp_num as usize] = -(10 as libc::c_int);
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn append_str(mut dst: *mut *mut libc::c_char,
                                    mut src: *mut libc::c_char,
                                    mut delim: *mut libc::c_char)
                                    -> *mut libc::c_char
/*==================================================================*/
{
    if !src.is_null() && *src as libc::c_int != 0 {
        if (*dst).is_null() {
            *dst = strdup(src)
        } else {
            *dst =
                realloc_data(*dst as *mut libc::c_void,
                             strlen(*dst).wrapping_add((if !delim.is_null() {
                                 strlen(delim)
                             } else {
                                 0 as libc::c_int
                                     as
                                     libc::c_ulong
                             })).wrapping_add(strlen(src)).wrapping_add(1
                                 as
                                 libc::c_int
                                 as
                                 libc::c_ulong),
                             b"append_str\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            if !delim.is_null() { strcat(*dst, delim); }
            strcat(*dst, src);
        }
    }
    return *dst;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_ipal_cframe_sm(mut c_ptr: *mut CASE_FRAME,
                                              mut cp: *mut libc::c_uchar,
                                              mut num: libc::c_int,
                                              mut flag: libc::c_int)
/*==================================================================*/
{
    /* 意味マーカの読みだし */
    let mut point: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_int = 0;
    let mut sm_num: libc::c_int = 0 as libc::c_int;
    let mut sm_print_num: libc::c_int = 0 as libc::c_int;
    // let mut mlength: libc::c_int = 0;
    let mut sm_delete_sm_max: libc::c_int = 0 as libc::c_int;
    let mut sm_specify_sm_max: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 3072] = [0; 3072];
    let mut sm_delete_sm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sm_specify_sm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if *cp as libc::c_int == '\u{0}' as i32 { return; }
    if flag & 1 as libc::c_int != 0 {
        size = 11 as libc::c_int
    } else if flag & 2 as libc::c_int != 0 {
        size = 12 as libc::c_int
    } else { return; }
    str = strdup(cp as *const libc::c_char);
    *str = '\u{0}' as i32 as libc::c_char;
    point = cp;
    buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    loop {
        point = extract_ipal_str(point, cf_str_buf, 0 as libc::c_int);
        if point.is_null() { break; }
        /* 意味素制限 */
        if *cf_str_buf.offset(0 as libc::c_int as isize) as libc::c_int ==
            '+' as i32 {
            if (*c_ptr).sm_specify[num as usize].is_null() {
                (*c_ptr).sm_specify_size[num as usize] = 256 as libc::c_int;
                (*c_ptr).sm_specify[num as usize] =
                    malloc_data((::std::mem::size_of::<libc::c_char>() as
                        libc::c_ulong).wrapping_mul((*c_ptr).sm_specify_size[num
                        as
                        usize]
                        as
                        libc::c_ulong).wrapping_mul(size
                        as
                        libc::c_ulong).wrapping_add(1
                        as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"_make_ipal_cframe_sm\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                *(*c_ptr).sm_specify[num as usize] =
                    '\u{0}' as i32 as libc::c_char;
                if flag & 2 as libc::c_int != 0 {
                    sm_specify_sm_max =
                        (::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(1024 as libc::c_int
                            as libc::c_ulong)
                            as libc::c_int;
                    sm_specify_sm =
                        malloc_data(sm_specify_sm_max as size_t,
                                    b"_make_ipal_cframe_sm\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char) as
                            *mut libc::c_char;
                    strcpy(sm_specify_sm,
                           b"\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe5\x88\xb6\xe9\x99\x90:\x00"
                               as *const u8 as *const libc::c_char);
                } else {
                    sm_specify_sm =
                        strdup(b"\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe5\x88\xb6\xe9\x99\x90\x00"
                            as *const u8 as *const libc::c_char)
                }
            } else if (*c_ptr).sm_specify_num[num as usize] >=
                (*c_ptr).sm_specify_size[num as usize] {
                (*c_ptr).sm_specify_size[num as usize] <<= 1 as libc::c_int;
                (*c_ptr).sm_specify[num as usize] =
                    realloc_data((*c_ptr).sm_specify[num as usize] as
                                     *mut libc::c_void,
                                 (::std::mem::size_of::<libc::c_char>() as
                                     libc::c_ulong).wrapping_mul((*c_ptr).sm_specify_size[num
                                     as
                                     usize]
                                     as
                                     libc::c_ulong).wrapping_mul(size
                                     as
                                     libc::c_ulong).wrapping_add(1
                                     as
                                     libc::c_int
                                     as
                                     libc::c_ulong),
                                 b"_make_ipal_cframe_sm\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char
            }
            /* codeが書いてあるとき */
            if *cf_str_buf.offset(1 as libc::c_int as isize) as libc::c_int ==
                '1' as i32 {
                strcat((*c_ptr).sm_specify[num as usize],
                       &mut *cf_str_buf.offset(1 as libc::c_int as isize) as
                           *mut libc::c_uchar as *const libc::c_char);
                if flag & 2 as libc::c_int != 0 {
                    /* 表示用の意味素名への変換 (NTTのみ) */
                    temp =
                        code2sm(&mut *cf_str_buf.offset(1 as libc::c_int as
                            isize) as
                            *mut libc::c_uchar as *mut libc::c_char);
                    if *temp.offset(0 as libc::c_int as isize) != 0 {
                        /* -1 ではないのは '/' の分 */
                        if strlen(sm_specify_sm).wrapping_add(strlen(temp)) >
                            (sm_specify_sm_max - 2 as libc::c_int) as
                                libc::c_ulong {
                            sm_specify_sm_max <<= 1 as libc::c_int;
                            sm_specify_sm =
                                realloc_data(sm_specify_sm as
                                                 *mut libc::c_void,
                                             sm_specify_sm_max as size_t,
                                             b"_make_ipal_cframe_sm\x00" as
                                                 *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char) as
                                    *mut libc::c_char
                        }
                        strcat(sm_specify_sm,
                               b"/\x00" as *const u8 as *const libc::c_char);
                        strcat(sm_specify_sm, temp);
                    }
                }
            } else if flag & 2 as libc::c_int != 0 {
                /* 意味素名での指定 (NTTのみ) */
                strcat((*c_ptr).sm_specify[num as usize],
                       sm2code(&mut *cf_str_buf.offset(1 as libc::c_int as
                           isize) as
                           *mut libc::c_uchar as *mut libc::c_char));
            }
            (*c_ptr).sm_specify_num[num as usize] += 1
        } else if *cf_str_buf.offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32 {
            if (*c_ptr).sm_delete[num as usize].is_null() {
                (*c_ptr).sm_delete_size[num as usize] = 256 as libc::c_int;
                (*c_ptr).sm_delete[num as usize] =
                    malloc_data((::std::mem::size_of::<libc::c_char>() as
                        libc::c_ulong).wrapping_mul((*c_ptr).sm_delete_size[num
                        as
                        usize]
                        as
                        libc::c_ulong).wrapping_mul(size
                        as
                        libc::c_ulong).wrapping_add(1
                        as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"_make_ipal_cframe_sm\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                *(*c_ptr).sm_delete[num as usize] =
                    '\u{0}' as i32 as libc::c_char;
                if PrintDeletedSM != 0 && flag & 2 as libc::c_int != 0 {
                    sm_delete_sm_max =
                        (::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong).wrapping_mul(1024 as libc::c_int
                            as libc::c_ulong)
                            as libc::c_int;
                    sm_delete_sm =
                        malloc_data(sm_delete_sm_max as size_t,
                                    b"_make_ipal_cframe_sm\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char) as
                            *mut libc::c_char;
                    strcpy(sm_delete_sm,
                           b"\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe5\x89\x8a\xe9\x99\xa4:\x00"
                               as *const u8 as *const libc::c_char);
                } else {
                    sm_delete_sm =
                        strdup(b"\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe5\x89\x8a\xe9\x99\xa4\x00"
                            as *const u8 as *const libc::c_char)
                }
            } else if (*c_ptr).sm_delete_num[num as usize] >=
                (*c_ptr).sm_delete_size[num as usize] {
                (*c_ptr).sm_delete_size[num as usize] <<= 1 as libc::c_int;
                (*c_ptr).sm_delete[num as usize] =
                    realloc_data((*c_ptr).sm_delete[num as usize] as
                                     *mut libc::c_void,
                                 (::std::mem::size_of::<libc::c_char>() as
                                     libc::c_ulong).wrapping_mul((*c_ptr).sm_delete_size[num
                                     as
                                     usize]
                                     as
                                     libc::c_ulong).wrapping_mul(size
                                     as
                                     libc::c_ulong).wrapping_add(1
                                     as
                                     libc::c_int
                                     as
                                     libc::c_ulong),
                                 b"_make_ipal_cframe_sm\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char
            }
            /* 使ってはいけない意味素 */
            /* codeが書いてあるとき */
            if *cf_str_buf.offset(1 as libc::c_int as isize) as libc::c_int ==
                '1' as i32 {
                strcat((*c_ptr).sm_delete[num as usize],
                       &mut *cf_str_buf.offset(1 as libc::c_int as isize) as
                           *mut libc::c_uchar as *const libc::c_char);
                if PrintDeletedSM != 0 && flag & 2 as libc::c_int != 0 {
                    /* 表示用の意味素名への変換 (NTTのみ) */
                    temp =
                        code2sm(&mut *cf_str_buf.offset(1 as libc::c_int as
                            isize) as
                            *mut libc::c_uchar as *mut libc::c_char);
                    if *temp.offset(0 as libc::c_int as isize) != 0 {
                        /* -1 ではないのは '/' の分 */
                        if strlen(sm_delete_sm).wrapping_add(strlen(temp)) >
                            (sm_delete_sm_max - 2 as libc::c_int) as
                                libc::c_ulong {
                            sm_delete_sm_max <<= 1 as libc::c_int;
                            sm_delete_sm =
                                realloc_data(sm_delete_sm as
                                                 *mut libc::c_void,
                                             sm_delete_sm_max as size_t,
                                             b"_make_ipal_cframe_sm\x00" as
                                                 *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char) as
                                    *mut libc::c_char
                        }
                        strcat(sm_delete_sm,
                               b"/\x00" as *const u8 as *const libc::c_char);
                        strcat(sm_delete_sm, temp);
                    }
                }
            } else if flag & 2 as libc::c_int != 0 {
                /* 意味素名での指定 (NTTのみ) */
                strcat((*c_ptr).sm_delete[num as usize],
                       sm2code(&mut *cf_str_buf.offset(1 as libc::c_int as
                           isize) as
                           *mut libc::c_uchar as *mut libc::c_char));
            }
            (*c_ptr).sm_delete_num[num as usize] += 1
        } else {
            /* 普通の意味素 */
            sm_num += 1;
            sm_print_num += 1;
            if sm_num >= 256 as libc::c_int { break; }
            if strncmp(cf_str_buf as *const libc::c_char,
                       b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                           *const libc::c_char,
                       strlen(b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                           *const libc::c_char)) == 0 {
                /* 前回も<数量>のときは入れない */
                if sm_num > 1 as libc::c_int &&
                    strncmp(&mut *buf.as_mut_ptr().offset((size *
                        (sm_num -
                            2 as
                                libc::c_int))
                        as isize),
                            sm2code(b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char),
                            size as libc::c_ulong) == 0 {
                    sm_num -= 1
                } else {
                    strcat(buf.as_mut_ptr(),
                           sm2code(b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char));
                }
            } else if strncmp(cf_str_buf as *const libc::c_char,
                              b"\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00" as
                                  *const u8 as *const libc::c_char,
                              strlen(b"\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00"
                                  as *const u8 as *const libc::c_char))
                == 0 {
                strcat(buf.as_mut_ptr(),
                       sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char));
                if MatchPP((*c_ptr).pp[num as
                    usize][0 as libc::c_int as usize],
                           b"\xe3\x82\xac\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) != 0
                {
                    /* 今は、ガ格以外に<主体準>を与えても<主体>と同じになる */
                    (*c_ptr).etcflag |= 2 as libc::c_int
                }
            } else {
                strcat(buf.as_mut_ptr(),
                       sm2code(cf_str_buf as *mut libc::c_char));
            }
            if flag & 4 as libc::c_int != 0 &&
                (EX_PRINT_NUM < 0 as libc::c_int ||
                    sm_print_num <= EX_PRINT_NUM) {
                if *str.offset(0 as libc::c_int as isize) != 0 {
                    strcat(str, b"/\x00" as *const u8 as *const libc::c_char);
                }
                strcat(str, cf_str_buf as *const libc::c_char);
            }
        }
    }
    if buf[0 as libc::c_int as usize] != 0 {
        append_str(&mut *(*c_ptr).sm.as_mut_ptr().offset(num as isize),
                   buf.as_mut_ptr(), 0 as *mut libc::c_char);
    }
    if flag & 4 as libc::c_int != 0 {
        append_str(&mut *(*c_ptr).semantics.as_mut_ptr().offset(num as isize),
                   str,
                   b"/\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
        append_str(&mut *(*c_ptr).semantics.as_mut_ptr().offset(num as isize),
                   sm_delete_sm,
                   b"/\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
        append_str(&mut *(*c_ptr).semantics.as_mut_ptr().offset(num as isize),
                   sm_specify_sm,
                   b"/\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
        if EX_PRINT_NUM >= 0 as libc::c_int && sm_print_num > EX_PRINT_NUM {
            append_str(&mut *(*c_ptr).semantics.as_mut_ptr().offset(num as
                isize),
                       b"...\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                       b"/\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
        }
    }
    free(str as *mut libc::c_void);
    if !sm_delete_sm.is_null() { free(sm_delete_sm as *mut libc::c_void); }
    if !sm_specify_sm.is_null() { free(sm_specify_sm as *mut libc::c_void); };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn split_freq(mut cp: *mut libc::c_uchar)
                                    -> libc::c_int
/*==================================================================*/
{
    let mut freq: libc::c_int = 0;
    loop {
        if *cp as libc::c_int == ':' as i32 {
            sscanf(cp.offset(1 as libc::c_int as isize) as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   &mut freq as *mut libc::c_int);
            *cp = '\u{0}' as i32 as libc::c_uchar;
            return freq;
        } else {
            if *cp as libc::c_int == '\u{0}' as i32 {
                return 1 as libc::c_int;
            }
        }
        cp = cp.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn split_freq_for_gex(mut cp: *mut libc::c_uchar)
                                            -> libc::c_double
/*==================================================================*/
{
    let mut freq: libc::c_double =
        0.; /* 先頭の"<NE:"、"<TH:", "<CT:", "<CL:"を読み飛ばす */
    cp = cp.offset(4 as libc::c_int as isize); /* ">"を除く */
    loop {
        if *cp as libc::c_int == ':' as i32 {
            sscanf(cp.offset(1 as libc::c_int as isize) as
                       *const libc::c_char,
                   b"%lf\x00" as *const u8 as *const libc::c_char,
                   &mut freq as *mut libc::c_double);
            *cp.offset(-(1 as libc::c_int as isize)) =
                '\u{0}' as i32 as libc::c_uchar;
            return freq;
        } else {
            if *cp as libc::c_int == '\u{0}' as i32 {
                return 1 as libc::c_int as libc::c_double;
            }
        }
        cp = cp.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _register_ex_to_cframe(mut c_ptr: *mut CASE_FRAME,
                                                mut num: libc::c_int,
                                                mut token: *mut libc::c_char,
                                                mut freq: libc::c_int)
/*==================================================================*/
{
    if (*c_ptr).ex_size[num as usize] == 0 as libc::c_int {
        (*c_ptr).ex_size[num as usize] =
            10 as libc::c_int; /* 初期確保数 */
        (*c_ptr).ex_list[num as usize] =
            malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                as
                usize]
                as
                libc::c_ulong),
                        b"_make_ipal_cframe_ex\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut *mut libc::c_char;
        (*c_ptr).ex_freq[num as usize] =
            malloc_data((::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                as
                usize]
                as
                libc::c_ulong),
                        b"_make_ipal_cframe_ex\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_int
    } else if (*c_ptr).ex_num[num as usize] >= (*c_ptr).ex_size[num as usize]
    {
        (*c_ptr).ex_size[num as usize] <<= 1 as libc::c_int;
        (*c_ptr).ex_list[num as usize] =
            realloc_data((*c_ptr).ex_list[num as usize] as *mut libc::c_void,
                         (::std::mem::size_of::<*mut libc::c_char>() as
                             libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                             as
                             usize]
                             as
                             libc::c_ulong),
                         b"_make_ipal_cframe_ex\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut *mut libc::c_char;
        (*c_ptr).ex_freq[num as usize] =
            realloc_data((*c_ptr).ex_freq[num as usize] as *mut libc::c_void,
                         (::std::mem::size_of::<libc::c_int>() as
                             libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                             as
                             usize]
                             as
                             libc::c_ulong),
                         b"_make_ipal_cframe_ex\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_int
    }
    let ref mut fresh7 =
        *(*c_ptr).ex_list[num as
            usize].offset((*c_ptr).ex_num[num as usize] as
            isize);
    *fresh7 = strdup(token);
    let fresh8 = (*c_ptr).ex_num[num as usize];
    (*c_ptr).ex_num[num as usize] = (*c_ptr).ex_num[num as usize] + 1;
    *(*c_ptr).ex_freq[num as usize].offset(fresh8 as isize) = freq;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_ipal_cframe_ex(mut c_ptr: *mut CASE_FRAME,
                                              mut cp: *mut libc::c_uchar,
                                              mut num: libc::c_int,
                                              mut flag: libc::c_int,
                                              mut fflag: libc::c_int,
                                              mut init_flag: libc::c_int)
/*==================================================================*/
{
    /* 例の読みだし */
    /* fflag: 頻度1を使うかどうか
              格が外の関係のときだけ使う */
    let mut point: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut point2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut max: libc::c_int = 0;
    let mut thesaurus: libc::c_int = 2 as libc::c_int;
    let mut freq: libc::c_int = 0;
    let mut over_flag: libc::c_int = 0 as libc::c_int;
    let mut agent_count: libc::c_int = 0 as libc::c_int;
    let mut sub_agent_flag: libc::c_int = 0 as libc::c_int;
    let mut ex_agent_flag: libc::c_int = 0;
    let mut freq_gex: libc::c_double = 0.;
    let mut agent_ratio: libc::c_double =
        -(1 as libc::c_int) as libc::c_double;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut destination: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 用例合計数の初期化 (外の関係を追加するとき以外は初期化する) */
    if init_flag != 0 { (*c_ptr).freq[num as usize] = 0 as libc::c_int }
    if *cp as libc::c_int == '\u{0}' as i32 { return; }
    /* 引くリソースによって関数などをセット */
    destination =
        &mut *(*c_ptr).ex.as_mut_ptr().offset(num as isize) as
            *mut *mut libc::c_char;
    if flag & 1 as libc::c_int != 0 {
        thesaurus = 1 as libc::c_int;
        max = 256 as libc::c_int * 11 as libc::c_int
    } else if flag & 2 as libc::c_int != 0 {
        thesaurus = 2 as libc::c_int;
        max = 256 as libc::c_int * 12 as libc::c_int
    }
    /* 最大値やめないといけません */
    buf =
        malloc_data((::std::mem::size_of::<libc::c_char>() as
            libc::c_ulong).wrapping_mul(max as libc::c_ulong),
                    b"_make_ipal_cframe_ex\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    point = cp;
    *buf = '\u{0}' as i32 as libc::c_char;
    loop
    /* fprintf(stderr, "%s:%.10f freq_gex\n", point2 + 1, freq_gex); */
    {
        point =
            extract_ipal_str(point, cf_str_buf,
                             (0 as libc::c_int == 0) as libc::c_int);
        if point.is_null() { break; }
        point2 = cf_str_buf;
        /* 用例中に記された汎化素性の読み込み */
        if strncmp(point2 as *const libc::c_char,
                   b"<TH:\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 ||
            strncmp(point2 as *const libc::c_char,
                    b"<NE:\x00" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong) == 0 ||
            strncmp(point2 as *const libc::c_char,
                    b"<CT:\x00" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong) == 0 ||
            strncmp(point2 as *const libc::c_char,
                    b"<CL:\x00" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong) == 0 {
            /* 頻度の抽出 */
            freq_gex = split_freq_for_gex(point2);
            if strcmp(point2 as *const libc::c_char,
                      b"<TH:\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                          *const libc::c_char) == 0 {
                /* <主体>の割合を下で利用 */
                agent_ratio = freq_gex
            } /* 初期確保数 */
            if (*c_ptr).gex_size[num as usize] == 0 as libc::c_int {
                (*c_ptr).gex_size[num as usize] = 10 as libc::c_int;
                (*c_ptr).gex_list[num as usize] =
                    malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong).wrapping_mul((*c_ptr).gex_size[num
                        as
                        usize]
                        as
                        libc::c_ulong),
                                b"_make_ipal_cframe_ex\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut *mut libc::c_char;
                (*c_ptr).gex_freq[num as usize] =
                    malloc_data((::std::mem::size_of::<libc::c_double>() as
                        libc::c_ulong).wrapping_mul((*c_ptr).gex_size[num
                        as
                        usize]
                        as
                        libc::c_ulong),
                                b"_make_ipal_cframe_ex\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_double
            } else if (*c_ptr).gex_num[num as usize] >=
                (*c_ptr).gex_size[num as usize] {
                (*c_ptr).gex_size[num as usize] <<= 1 as libc::c_int;
                (*c_ptr).gex_list[num as usize] =
                    realloc_data((*c_ptr).gex_list[num as usize] as
                                     *mut libc::c_void,
                                 (::std::mem::size_of::<*mut libc::c_char>()
                                     as
                                     libc::c_ulong).wrapping_mul((*c_ptr).gex_size[num
                                     as
                                     usize]
                                     as
                                     libc::c_ulong),
                                 b"_make_ipal_cframe_ex\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char)
                        as *mut *mut libc::c_char;
                (*c_ptr).gex_freq[num as usize] =
                    realloc_data((*c_ptr).gex_freq[num as usize] as
                                     *mut libc::c_void,
                                 (::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong).wrapping_mul((*c_ptr).gex_size[num
                                     as
                                     usize]
                                     as
                                     libc::c_ulong),
                                 b"_make_ipal_cframe_ex\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_double
            }
            let ref mut fresh9 =
                *(*c_ptr).gex_list[num as
                    usize].offset((*c_ptr).gex_num[num as
                    usize]
                    as isize);
            *fresh9 =
                strdup(point2.offset(1 as libc::c_int as isize) as
                    *const libc::c_char);
            let fresh10 = (*c_ptr).gex_num[num as usize];
            (*c_ptr).gex_num[num as usize] =
                (*c_ptr).gex_num[num as usize] + 1;
            *(*c_ptr).gex_freq[num as usize].offset(fresh10 as isize) =
                freq_gex
        } else {
            /* 頻度の抽出 */
            freq = split_freq(point2);
            if strcmp(point2 as *const libc::c_char,
                      b"<\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96>\x00" as
                          *const u8 as *const libc::c_char) == 0 {
                sub_agent_flag = 1 as libc::c_int
            } else {
                /* fflag == TRUE: 低頻度を削除 */
                if fflag != 0 && freq < 10 as libc::c_int {
                    continue; /* 全体を格納 */
                }
                if *point2 as libc::c_int != '\u{0}' as i32 {
                    _register_ex_to_cframe(c_ptr, num,
                                           point2 as *mut libc::c_char, freq);
                    if agent_ratio > 0 as libc::c_int as libc::c_double {
                        /* <TH:主体>があるならば、動的<主体>チェックはしない */
                        ex_agent_flag = 1 as libc::c_int
                    } else { ex_agent_flag = 0 as libc::c_int }
                    if !strchr(point2 as *const libc::c_char,
                               '?' as i32).is_null() {
                        /* "?"で結合されたものは切って格納 */
                        token =
                            strtok(point2 as *mut libc::c_char,
                                   b"?\x00" as *const u8 as
                                       *const libc::c_char);
                        while !token.is_null() {
                            code =
                                get_str_code(token as *mut libc::c_uchar,
                                             thesaurus);
                            if !code.is_null() {
                                /* <主体>のチェック (for backward compatibility -> will be deleted) */
                                if ex_agent_flag == 0 as libc::c_int &&
                                    cf_match_element(code,
                                                     (if flag &
                                                         1 as
                                                             libc::c_int
                                                         != 0 {
                                                         sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                             as
                                                             *const u8
                                                             as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char)
                                                             as
                                                             *const libc::c_char
                                                     } else {
                                                         b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                             as *const u8
                                                             as
                                                             *const libc::c_char
                                                     }) as
                                                         *mut libc::c_char,
                                                     0 as libc::c_int) != 0
                                {
                                    agent_count += freq;
                                    ex_agent_flag = 1 as libc::c_int
                                }
                                if over_flag == 0 {
                                    if strlen(buf).wrapping_add(strlen(code))
                                        >= max as libc::c_ulong {
                                        /* fprintf(stderr, "Too many EX <%s> (%2dth).\n", cf_str_buf, count); */
                                        over_flag = 1 as libc::c_int
                                    } else { strcat(buf, code); }
                                }
                                free(code as *mut libc::c_void);
                            }
                            _register_ex_to_cframe(c_ptr, num, token, freq);
                            token =
                                strtok(0 as *mut libc::c_char,
                                       b"?\x00" as *const u8 as
                                           *const libc::c_char)
                        }
                    }
                    (*c_ptr).freq[num as usize] += freq
                }
            }
        }
    }
    /* <主体>の割合を格フレームの<TH:主体>から取得する
       格フレーム旧バージョン(200706版以前): <TH:主体>がないので、上で動的に計算
       格フレーム新バージョン: <TH:主体>から取得するが、ない場合(0.01未満)は上で計算したagent_countになる
                               (上の動的計算は撲滅予定) */
    if agent_ratio > 0 as libc::c_int as libc::c_double {
        agent_count =
            (agent_ratio * (*c_ptr).freq[num as usize] as libc::c_double) as
                libc::c_int;
        /* <主体>を意味素に追加 (-no-probcase用) */
        if sub_agent_flag == 0 &&
            agent_ratio > SM_AGENT_THRESHOLD as libc::c_double {
            _make_ipal_cframe_sm(c_ptr,
                                 b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_uchar, num, flag);
        }
    }
    /* <主体>の追加 */
    if agent_count != 0 || sub_agent_flag != 0 {
        if (*c_ptr).ex_size[num as usize] == 0 as libc::c_int {
            (*c_ptr).ex_size[num as usize] =
                1 as libc::c_int; /* 初期確保数 */
            (*c_ptr).ex_list[num as usize] =
                malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                    libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                    as
                    usize]
                    as
                    libc::c_ulong),
                            b"_make_ipal_cframe_ex\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut *mut libc::c_char;
            (*c_ptr).ex_freq[num as usize] =
                malloc_data((::std::mem::size_of::<libc::c_int>() as
                    libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                    as
                    usize]
                    as
                    libc::c_ulong),
                            b"_make_ipal_cframe_ex\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_int
        } else if (*c_ptr).ex_num[num as usize] >=
            (*c_ptr).ex_size[num as usize] {
            (*c_ptr).ex_size[num as usize] <<= 1 as libc::c_int;
            (*c_ptr).ex_list[num as usize] =
                realloc_data((*c_ptr).ex_list[num as usize] as
                                 *mut libc::c_void,
                             (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                                 as
                                 usize]
                                 as
                                 libc::c_ulong),
                             b"_make_ipal_cframe_ex\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) as
                    *mut *mut libc::c_char;
            (*c_ptr).ex_freq[num as usize] =
                realloc_data((*c_ptr).ex_freq[num as usize] as
                                 *mut libc::c_void,
                             (::std::mem::size_of::<libc::c_int>() as
                                 libc::c_ulong).wrapping_mul((*c_ptr).ex_size[num
                                 as
                                 usize]
                                 as
                                 libc::c_ulong),
                             b"_make_ipal_cframe_ex\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_int
        }
        let ref mut fresh11 =
            *(*c_ptr).ex_list[num as
                usize].offset((*c_ptr).ex_num[num as usize]
                as isize);
        *fresh11 =
            strdup(b"<\xe4\xb8\xbb\xe4\xbd\x93>\x00" as *const u8 as
                *const libc::c_char);
        let fresh12 = (*c_ptr).ex_num[num as usize];
        (*c_ptr).ex_num[num as usize] = (*c_ptr).ex_num[num as usize] + 1;
        *(*c_ptr).ex_freq[num as usize].offset(fresh12 as isize) =
            if agent_count != 0 { agent_count } else { 1 as libc::c_int };
        if (*c_ptr).freq[num as usize] == 0 as libc::c_int {
            (*c_ptr).freq[num as usize] = 1 as libc::c_int
        }
    }
    *destination = strdup(buf);
    free(buf as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn rep2id(mut rep: *mut libc::c_char,
                                mut rep_len: libc::c_int,
                                mut buffer: *mut libc::c_char)
                                -> *mut libc::c_char
/*==================================================================*/
{
    /* MRPH_MAX * 9(max8桁+"+"の分)以上あるので溢れない */
    *buffer.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    if !rep.is_null() && rep_len > 0 as libc::c_int &&
        Mrph2idExist == (0 as libc::c_int == 0) as libc::c_int {
        let mut token_start: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut copied_rep: *mut libc::c_char =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                libc::c_ulong).wrapping_mul(rep_len as
                libc::c_ulong).wrapping_add(1
                as
                libc::c_int
                as
                libc::c_ulong),
                        b"rep2id\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char) as *mut libc::c_char;
        strncpy(copied_rep, rep, rep_len as libc::c_ulong);
        *copied_rep.offset(rep_len as isize) = '\u{0}' as i32 as libc::c_char;
        token_start =
            strtok(copied_rep, b"+?\x00" as *const u8 as *const libc::c_char);
        token = token_start;
        while !token.is_null() {
            value = db_get(mrph2id_db, token);
            if !value.is_null() {
                if *buffer.offset(0 as libc::c_int as isize) != 0 {
                    /* fprintf(stderr, ";; %s -> %s\n", rep, buffer); */
                    /* 2つ目以降 */
                    strncat(buffer,
                            rep.offset(token.offset(-(1 as libc::c_int as
                                isize)).wrapping_offset_from(token_start)
                                as libc::c_long as isize),
                            1 as libc::c_int as libc::c_ulong);
                }
                strcat(buffer, value);
                free(value as *mut libc::c_void);
            }
            token =
                strtok(0 as *mut libc::c_char,
                       b"+?\x00" as *const u8 as *const libc::c_char)
        }
        free(copied_rep as *mut libc::c_void);
    }
    return buffer;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_examples(mut cp: *mut libc::c_char,
                                        mut cp_len: libc::c_int,
                                        mut ex_list: *mut *mut libc::c_char,
                                        mut ex_num: libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut rep_id: *mut libc::c_char = 0 as *mut libc::c_char;
    if ex_list.is_null() { return -(1 as libc::c_int); }
    rep_id =
        rep2id(cp, cp_len,
               &mut *static_buffer.as_mut_ptr().offset(0 as libc::c_int as
                   isize));
    if *rep_id.offset(0 as libc::c_int as isize) != 0 {
        i = 0 as libc::c_int;
        while i < ex_num {
            if strcmp(rep_id, *ex_list.offset(i as isize)) == 0 { return i; }
            i += 1
        }
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_agentive(mut cp: *mut libc::c_uchar)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut point: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    point = cp;
    loop {
        point = extract_ipal_str(point, cf_str_buf, 0 as libc::c_int);
        if point.is_null() { break; }
        if strcmp(cf_str_buf as *const libc::c_char,
                  b"\xef\xbc\xa1\x00" as *const u8 as *const libc::c_char) ==
            0 {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_ipal_cframe(mut i_ptr: *mut CF_FRAME,
                                           mut cf_ptr: *mut CASE_FRAME,
                                           mut address: libc::c_ulonglong,
                                           mut size: libc::c_int,
                                           mut verb: *mut libc::c_char,
                                           mut flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int =
        0; /* 格フレームの用言表記 (代表表記) */
    let mut j: libc::c_int = 0;
    let mut ga_p: libc::c_int = 0 as libc::c_int;
    // let mut c1: libc::c_int = 0;
    // let mut c2: libc::c_int = 0;
    // let mut count: libc::c_int = 0 as libc::c_int;
    let mut ast_cap: [libc::c_char; 32] = [0; 32];
    // let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    (*cf_ptr).type_0 = flag;
    (*cf_ptr).cf_address = address;
    (*cf_ptr).cf_size = size;
    strcpy((*cf_ptr).cf_id.as_mut_ptr(),
           (*i_ptr).DATA as *const libc::c_char);
    (*cf_ptr).etcflag = (*i_ptr).etcflag;
    (*cf_ptr).entry = strdup((*cf_ptr).cf_id.as_mut_ptr());
    sscanf((*cf_ptr).cf_id.as_mut_ptr(),
           b"%[^:]\x00" as *const u8 as *const libc::c_char, (*cf_ptr).entry);
    strcpy((*cf_ptr).pred_type.as_mut_ptr(), (*i_ptr).pred_type.as_mut_ptr());
    i = 0 as libc::c_int;
    while !(*i_ptr).cf_align[i as usize].cf_id.is_null() {
        (*cf_ptr).cf_align[i as usize].cf_id =
            strdup((*i_ptr).cf_align[i as usize].cf_id);
        j = 0 as libc::c_int;
        while (*i_ptr).cf_align[i as
            usize].aligned_case[j as
            usize][0 as
            libc::c_int
            as
            usize]
            != -(10 as libc::c_int) {
            (*cf_ptr).cf_align[i as
                usize].aligned_case[j as
                usize][0 as
                libc::c_int
                as
                usize] =
                (*i_ptr).cf_align[i as
                    usize].aligned_case[j as
                    usize][0 as
                    libc::c_int
                    as
                    usize];
            (*cf_ptr).cf_align[i as
                usize].aligned_case[j as
                usize][1 as
                libc::c_int
                as
                usize] =
                (*i_ptr).cf_align[i as
                    usize].aligned_case[j as
                    usize][1 as
                    libc::c_int
                    as
                    usize];
            j += 1
        }
        (*cf_ptr).cf_align[i as
            usize].aligned_case[j as
            usize][0 as libc::c_int
            as usize] =
            -(10 as libc::c_int);
        (*cf_ptr).cf_align[i as
            usize].aligned_case[j as
            usize][1 as libc::c_int
            as usize] =
            -(10 as libc::c_int);
        i += 1
    }
    (*cf_ptr).cf_align[i as usize].cf_id = 0 as *mut libc::c_char;
    (*cf_ptr).cf_align[i as
        usize].aligned_case[0 as libc::c_int as
        usize][0 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
    (*cf_ptr).cf_align[i as
        usize].aligned_case[0 as libc::c_int as
        usize][1 as libc::c_int as
        usize] =
        -(10 as libc::c_int);
    i = 0 as libc::c_int;
    while (*i_ptr).samecase[i as usize][0 as libc::c_int as usize] !=
        -(10 as libc::c_int) {
        (*cf_ptr).samecase[i as usize][0 as libc::c_int as usize] =
            (*i_ptr).samecase[i as usize][0 as libc::c_int as usize];
        (*cf_ptr).samecase[i as usize][1 as libc::c_int as usize] =
            (*i_ptr).samecase[i as usize][1 as libc::c_int as usize];
        i += 1
    }
    (*cf_ptr).samecase[i as usize][0 as libc::c_int as usize] =
        -(10 as libc::c_int);
    (*cf_ptr).samecase[i as usize][1 as libc::c_int as usize] =
        -(10 as libc::c_int);
    if *(*i_ptr).feature != 0 {
        (*cf_ptr).feature = strdup((*i_ptr).feature)
    } else { (*cf_ptr).feature = 0 as *mut libc::c_char }
    (*cf_ptr).cf_similarity = 0 as libc::c_int as libc::c_float;
    /* 格要素の追加 */
    j = 0 as libc::c_int;
    if (*cf_ptr).voice == 2 as libc::c_int ||
        (*cf_ptr).voice == 5 as libc::c_int ||
        (*cf_ptr).voice == 6 as libc::c_int ||
        (*cf_ptr).voice == 7 as libc::c_int {
        _make_ipal_cframe_pp(cf_ptr,
                             b"\xe3\x82\xac\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_uchar, j,
                             flag);
        _make_ipal_cframe_sm(cf_ptr,
                             b"\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_uchar, j,
                             if Thesaurus == 2 as libc::c_int {
                                 6 as libc::c_int
                             } else { 5 as libc::c_int });
        _make_ipal_cframe_ex(cf_ptr,
                             b"\xe5\xbd\xbc\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_uchar, j,
                             Thesaurus, 0 as libc::c_int,
                             (0 as libc::c_int == 0) as libc::c_int);
        j += 1
    } else if (*cf_ptr).voice == 8 as libc::c_int {
        _make_ipal_cframe_pp(cf_ptr,
                             b"\xe3\x83\x8b\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_uchar, j,
                             flag);
        _make_ipal_cframe_sm(cf_ptr,
                             b"\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_uchar, j,
                             if Thesaurus == 2 as libc::c_int {
                                 6 as libc::c_int
                             } else { 5 as libc::c_int });
        _make_ipal_cframe_ex(cf_ptr,
                             b"\xe5\xbd\xbc\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_uchar, j,
                             Thesaurus, 0 as libc::c_int,
                             (0 as libc::c_int == 0) as libc::c_int);
        j += 1
    }
    /* 各格要素の処理 */
    i = 0 as libc::c_int;
    while i < (*i_ptr).casenum && j < 20 as libc::c_int {
        (*cf_ptr).adjacent[j as usize] = 0 as libc::c_int;
        if _make_ipal_cframe_pp(cf_ptr,
                                (*i_ptr).cs[i as usize].kaku_keishiki as
                                    *mut libc::c_uchar, j, flag) ==
            0 as libc::c_int {
            j -= 1
        } else {
            if Thesaurus == 1 as libc::c_int {
                _make_ipal_cframe_ex(cf_ptr,
                                     (*i_ptr).cs[i as usize].meishiku as
                                         *mut libc::c_uchar, j,
                                     5 as libc::c_int,
                                     if OptCaseFlag & 8 as libc::c_int != 0 {
                                         0 as libc::c_int
                                     } else {
                                         MatchPP((*cf_ptr).pp[j as
                                             usize][0 as
                                             libc::c_int
                                             as
                                             usize],
                                                 b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char)
                                     },
                                     (0 as libc::c_int == 0) as libc::c_int);
                _make_ipal_cframe_sm(cf_ptr,
                                     (*i_ptr).cs[i as usize].imisosei as
                                         *mut libc::c_uchar, j,
                                     5 as libc::c_int);
            } else if Thesaurus == 2 as libc::c_int {
                _make_ipal_cframe_ex(cf_ptr,
                                     (*i_ptr).cs[i as usize].meishiku as
                                         *mut libc::c_uchar, j,
                                     6 as libc::c_int,
                                     if OptCaseFlag & 8 as libc::c_int != 0 {
                                         0 as libc::c_int
                                     } else {
                                         MatchPP((*cf_ptr).pp[j as
                                             usize][0 as
                                             libc::c_int
                                             as
                                             usize],
                                                 b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char)
                                     },
                                     (0 as libc::c_int == 0) as libc::c_int);
                _make_ipal_cframe_sm(cf_ptr,
                                     (*i_ptr).cs[i as usize].imisosei as
                                         *mut libc::c_uchar, j,
                                     6 as libc::c_int);
            }
            /* 能動 : Agentive ガ格を任意的とする場合
	if (cf_ptr->voice == FRAME_ACTIVE &&
	    i == 0 && 
	    cf_ptr->pp[i][0] == pp_kstr_to_code("ガ") &&
	    check_agentive(i_ptr->DATA+i_ptr->jyutugoso) == TRUE)
	  cf_ptr->oblig[i] = FALSE;
	*/
            if (*cf_ptr).voice == 5 as libc::c_int ||
                (*cf_ptr).voice == 6 as libc::c_int ||
                (*cf_ptr).voice == 7 as libc::c_int {
                /* ガ → ヲ，ニ */
                if (*cf_ptr).pp[j as usize][0 as libc::c_int as usize] ==
                    pp_kstr_to_code(b"\xe3\x82\xac\x00" as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char) {
                    if ga_p == 0 as libc::c_int {
                        ga_p = (0 as libc::c_int == 0) as libc::c_int;
                        if (*cf_ptr).voice == 5 as libc::c_int {
                            _make_ipal_cframe_pp(cf_ptr,
                                                 b"\xe3\x83\xb2\xef\xbc\x8f\xe3\x83\x8b\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_uchar, j,
                                                 flag);
                        } else if (*cf_ptr).voice == 6 as libc::c_int {
                            _make_ipal_cframe_pp(cf_ptr,
                                                 b"\xe3\x83\xb2\x00" as
                                                     *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_uchar, j,
                                                 flag);
                        } else if (*cf_ptr).voice == 7 as libc::c_int {
                            _make_ipal_cframe_pp(cf_ptr,
                                                 b"\xe3\x83\x8b\x00" as
                                                     *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_uchar, j,
                                                 flag);
                        }
                    } else {
                        _make_ipal_cframe_pp(cf_ptr,
                                             b"\xe3\x83\xb2\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut libc::c_uchar, j, flag);
                        /* ガ・ガ構文 */
                    }
                }
            } else if (*cf_ptr).voice == 2 as libc::c_int ||
                (*cf_ptr).voice == 3 as libc::c_int ||
                (*cf_ptr).voice == 4 as libc::c_int {
                /* 間接 ガ→ニ，直接 ガ→ニ／ニヨッテ／．． */
                if strcmp((*i_ptr).cs[i as usize].kaku_keishiki,
                          b"\xe3\x82\xac\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                    if (*cf_ptr).voice == 2 as libc::c_int {
                        _make_ipal_cframe_pp(cf_ptr,
                                             b"\xe3\x83\x8b\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut libc::c_uchar, j, flag);
                    } else if (*cf_ptr).voice == 3 as libc::c_int {
                        _make_ipal_cframe_pp(cf_ptr,
                                             b"\xe3\x83\x8b\xef\xbc\x8f\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\xab\xef\xbc\x8f\xe3\x82\xab\xe3\x83\xa9\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_uchar, j, flag);
                    } else if (*cf_ptr).voice == 4 as libc::c_int {
                        _make_ipal_cframe_pp(cf_ptr,
                                             b"\xe3\x83\x8b\xef\xbc\x8f\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\xab\xef\xbc\x8f\xe3\x82\xab\xe3\x83\xa9\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_uchar, j, flag);
                    }
                } else if (*cf_ptr).voice == 3 as libc::c_int &&
                    (strcmp((*i_ptr).cs[i as usize].kaku_keishiki,
                            b"\xe3\x83\xb2\x00" as *const u8 as
                                *const libc::c_char) == 0 ||
                        sprintf(ast_cap.as_mut_ptr(),
                                b"\xe3\x83\xb2\xef\xbc\x8a\x00" as
                                    *const u8 as
                                    *const libc::c_char) != 0 &&
                            strcmp((*i_ptr).cs[i as
                                usize].kaku_keishiki,
                                   ast_cap.as_mut_ptr()) == 0) ||
                    (*cf_ptr).voice == 4 as libc::c_int &&
                        (strcmp((*i_ptr).cs[i as
                            usize].kaku_keishiki,
                                b"\xe3\x83\x8b\x00" as *const u8 as
                                    *const libc::c_char) == 0 ||
                            sprintf(ast_cap.as_mut_ptr(),
                                    b"\xe3\x83\x8b\xef\xbc\x8a\x00"
                                        as *const u8 as
                                        *const libc::c_char) != 0
                                &&
                                strcmp((*i_ptr).cs[i as
                                    usize].kaku_keishiki,
                                       ast_cap.as_mut_ptr()) == 0)
                {
                    _make_ipal_cframe_pp(cf_ptr,
                                         b"\xe3\x82\xac\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_uchar, j, flag);
                }
            } else if (*cf_ptr).voice == 9 as libc::c_int {
                /* 直接 ニ／ニヨッテ／．．→ガ */
                /* 可能 */
                if strcmp((*i_ptr).cs[i as usize].kaku_keishiki,
                          b"\xe3\x83\xb2\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                    _make_ipal_cframe_pp(cf_ptr,
                                         b"\xe3\x82\xac\xef\xbc\x8f\xe3\x83\xb2\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_uchar, j, flag);
                }
            } else if (*cf_ptr).voice == 11 as libc::c_int {
                /* 自発 */
                if strcmp((*i_ptr).cs[i as usize].kaku_keishiki,
                          b"\xe3\x83\xb2\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                    _make_ipal_cframe_pp(cf_ptr,
                                         b"\xe3\x82\xac\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_uchar, j, flag);
                }
            }
        }
        i += 1;
        j += 1
    }
    (*cf_ptr).element_num = j;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_quasi_closest_case_component(mut t_ptr:
                                                          *mut TAG_DATA,
                                                          mut pre_ptr:
                                                          *mut TAG_DATA)
                                                          -> *mut TAG_DATA
/*==================================================================*/
{
    if (*t_ptr).num < 1 as libc::c_int ||
        (*t_ptr).type_0 == 2 as libc::c_int &&
            (*t_ptr).inum != 0 as libc::c_int {
        return 0 as *mut TAG_DATA;
    }
    if !check_feature((*t_ptr).f,
                      b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        return t_ptr;
    }
    if (*pre_ptr).type_0 == 2 as libc::c_int &&
        (*pre_ptr).inum != 0 as libc::c_int {
        return 0 as *mut TAG_DATA;
    }
    if check_feature((*pre_ptr).f,
                     b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char).is_null() {
        return 0 as *mut TAG_DATA;
    }
    if !check_feature((*pre_ptr).f,
                      b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00" as *const u8
                          as *const libc::c_char as
                          *mut libc::c_char).is_null() ||
        (*pre_ptr).SM_code[0 as libc::c_int as usize] as libc::c_int ==
            '\u{0}' as i32 &&
            !check_feature((*pre_ptr).f,
                           b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
        return 0 as *mut TAG_DATA;
    }
    if !check_feature((*pre_ptr).f,
                      b"\xe4\xbf\x82:\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() ||
        !check_feature((*pre_ptr).f,
                       b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        cf_match_element((*pre_ptr).SM_code.as_mut_ptr(),
                         b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         0 as libc::c_int) == 0 &&
            (!check_feature((*pre_ptr).f,
                            b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*pre_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x82\xab\xe3\x83\xa9\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*pre_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x98\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*pre_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\xa8\xe3\x83\xaa\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*pre_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x88\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*pre_ptr).f,
                               b"\xe4\xbf\x82:\xe3\x83\x9e\xe3\x83\x87\xe6\xa0\xbc\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
        return pre_ptr;
    }
    return 0 as *mut TAG_DATA;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn feature2case(mut tp: *mut TAG_DATA)
                                      -> *mut libc::c_char
/*==================================================================*/
{
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if !check_feature((*tp).f,
                      b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        buffer =
            malloc_data(strlen(b"\xe3\x83\x8b\x00" as *const u8 as
                *const libc::c_char).wrapping_add(1 as
                libc::c_int
                as
                libc::c_ulong),
                        b"feature2case\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        strcpy(buffer,
               b"\xe3\x83\x8b\x00" as *const u8 as *const libc::c_char);
        return buffer;
    } else {
        cp =
            check_feature((*tp).f,
                          b"\xe4\xbf\x82\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            buffer =
                strdup(cp.offset(strlen(b"\xe4\xbf\x82:\x00" as *const u8 as
                    *const libc::c_char) as isize));
            if strncmp(buffer.offset(strlen(buffer) as
                isize).offset(-(strlen(b"\xe6\xa0\xbc\x00"
                as
                *const u8
                as
                *const libc::c_char)
                as isize)),
                       b"\xe6\xa0\xbc\x00" as *const u8 as
                           *const libc::c_char,
                       strlen(b"\xe6\xa0\xbc\x00" as *const u8 as
                           *const libc::c_char)) == 0 {
                *buffer.offset(strlen(buffer) as
                    isize).offset(-(strlen(b"\xe6\xa0\xbc\x00"
                    as *const u8 as
                    *const libc::c_char)
                    as isize)) =
                    '\u{0}' as i32 as libc::c_char;
                if pp_kstr_to_code(buffer) != -(10 as libc::c_int) {
                    return buffer;
                }
            }
            free(buffer as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn f_num_inc(mut start: libc::c_int,
                                   mut f_num_p: *mut libc::c_int)
/*==================================================================*/
{
    *f_num_p += 1;
    if start + *f_num_p >= MAX_Case_frame_num {
        realloc_cf();
        realloc_cmm();
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_cf_element(mut cf_ptr: *mut CASE_FRAME,
                                          mut num: libc::c_int)
/*==================================================================*/
{
    let mut k: libc::c_int = 0;
    if !(*cf_ptr).pp_str[num as usize].is_null() {
        free((*cf_ptr).pp_str[num as usize] as *mut libc::c_void);
        (*cf_ptr).pp_str[num as usize] = 0 as *mut libc::c_char
    }
    if !(*cf_ptr).ex[num as usize].is_null() {
        free((*cf_ptr).ex[num as usize] as *mut libc::c_void);
        (*cf_ptr).ex[num as usize] = 0 as *mut libc::c_char
    }
    if !(*cf_ptr).sm[num as usize].is_null() {
        free((*cf_ptr).sm[num as usize] as *mut libc::c_void);
        (*cf_ptr).sm[num as usize] = 0 as *mut libc::c_char
    }
    if !(*cf_ptr).sm_delete[num as usize].is_null() {
        free((*cf_ptr).sm_delete[num as usize] as *mut libc::c_void);
        (*cf_ptr).sm_delete[num as usize] = 0 as *mut libc::c_char;
        (*cf_ptr).sm_delete_size[num as usize] = 0 as libc::c_int;
        (*cf_ptr).sm_delete_num[num as usize] = 0 as libc::c_int
    }
    if !(*cf_ptr).sm_specify[num as usize].is_null() {
        free((*cf_ptr).sm_specify[num as usize] as *mut libc::c_void);
        (*cf_ptr).sm_specify[num as usize] = 0 as *mut libc::c_char;
        (*cf_ptr).sm_specify_size[num as usize] = 0 as libc::c_int;
        (*cf_ptr).sm_specify_num[num as usize] = 0 as libc::c_int
    }
    if !(*cf_ptr).ex_list[num as usize].is_null() {
        k = 0 as libc::c_int;
        while k < (*cf_ptr).ex_num[num as usize] {
            if !(*(*cf_ptr).ex_list[num as
                usize].offset(k as isize)).is_null() {
                free(*(*cf_ptr).ex_list[num as usize].offset(k as isize) as
                    *mut libc::c_void);
            }
            k += 1
        }
        free((*cf_ptr).ex_list[num as usize] as *mut libc::c_void);
        free((*cf_ptr).ex_freq[num as usize] as *mut libc::c_void);
        (*cf_ptr).ex_list[num as usize] = 0 as *mut *mut libc::c_char;
        (*cf_ptr).ex_size[num as usize] = 0 as libc::c_int;
        (*cf_ptr).ex_num[num as usize] = 0 as libc::c_int
    }
    (*cf_ptr).freq[num as usize] = 0 as libc::c_int;
    if !(*cf_ptr).gex_list[num as usize].is_null() {
        k = 0 as libc::c_int;
        while k < (*cf_ptr).gex_num[num as usize] {
            if !(*(*cf_ptr).gex_list[num as
                usize].offset(k as isize)).is_null()
            {
                free(*(*cf_ptr).gex_list[num as usize].offset(k as isize) as
                    *mut libc::c_void);
            }
            k += 1
        }
        free((*cf_ptr).gex_list[num as usize] as *mut libc::c_void);
        free((*cf_ptr).gex_freq[num as usize] as *mut libc::c_void);
        (*cf_ptr).gex_list[num as usize] = 0 as *mut *mut libc::c_char;
        (*cf_ptr).gex_size[num as usize] = 0 as libc::c_int;
        (*cf_ptr).gex_num[num as usize] = 0 as libc::c_int
    }
    if !(*cf_ptr).semantics[num as usize].is_null() {
        free((*cf_ptr).semantics[num as usize] as *mut libc::c_void);
        (*cf_ptr).semantics[num as usize] = 0 as *mut libc::c_char
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _make_ipal_cframe_subcontract(mut sp:
                                                       *mut SENTENCE_DATA,
                                                       mut t_ptr:
                                                       *mut TAG_DATA,
                                                       mut start: libc::c_int,
                                                       mut verb:
                                                       *mut libc::c_char,
                                                       mut voice: libc::c_int,
                                                       mut flag: libc::c_int,
                                                       mut use_closest_cc:
                                                       libc::c_int)
                                                       -> libc::c_int
/*==================================================================*/
{
    let mut i_ptr: *mut CF_FRAME = 0 as *mut CF_FRAME;
    let mut cf_ptr: *mut CASE_FRAME = 0 as *mut CASE_FRAME;
    let mut cbp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut f_num: libc::c_int = 0 as libc::c_int;
    let mut break_flag: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut address: libc::c_ulonglong = 0;
    let mut pre_pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut address_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    if verb.is_null() { return f_num; }
    cf_ptr = Case_frame_array.offset(start as isize);
    /* 直前格要素をくっつけて検索 */
    if use_closest_cc != 0 {
        /* ひらがなで曖昧性のあるときは、格解析で曖昧性解消するために
	   ここではすべての格フレームを検索しておく */
        if check_str_type((*(*t_ptr).head_ptr).Goi.as_mut_ptr() as
                              *mut libc::c_uchar, 2 as libc::c_int,
                          0 as libc::c_int) != 0 &&
            !check_feature((*(*t_ptr).head_ptr).f,
                           b"\xe5\x93\x81\xe6\x9b\x96\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() {
            address_str = get_ipal_address(verb as *mut libc::c_uchar, flag)
        } else {
            cbp =
                get_quasi_closest_case_component(t_ptr,
                                                 if (*t_ptr).num <
                                                     1 as libc::c_int {
                                                     0 as *mut TAG_DATA
                                                 } else {
                                                     t_ptr.offset(-(1 as
                                                         libc::c_int
                                                         as
                                                         isize))
                                                 });
            if OptCaseFlag & 2048 as libc::c_int != 0 && !cbp.is_null() {
                let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cbp_str: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cbp_str_malloc_flag: libc::c_int = 0 as libc::c_int;
                pp = feature2case(cbp);
                if !pp.is_null() {
                    if OptCaseFlag & 32 as libc::c_int != 0 {
                        if !(OptCaseFlag & 1024 as libc::c_int != 0 &&
                            {
                                cbp_str =
                                    get_bnst_head_canonical_rep((*cbp).b_ptr,
                                                                OptCaseFlag
                                                                    &
                                                                    512
                                                                        as
                                                                        libc::c_int);
                                !cbp_str.is_null()
                            }) {
                            cbp_str =
                                get_mrph_rep_from_f((*cbp).head_ptr,
                                                    0 as libc::c_int);
                            if cbp_str.is_null() {
                                /* feature中の代表表記 */
                                cbp_str =
                                    make_mrph_rn((*cbp).head_ptr); /* なければ作る */
                                cbp_str_malloc_flag = 1 as libc::c_int
                            }
                        }
                    } else { cbp_str = (*(*cbp).head_ptr).Goi.as_mut_ptr() }
                    buffer =
                        malloc_data(strlen(cbp_str).wrapping_add(strlen(pp)).wrapping_add(strlen(verb)).wrapping_add(3
                            as
                            libc::c_int
                            as
                            libc::c_ulong),
                                    b"_make_ipal_cframe_subcontract\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char) as
                            *mut libc::c_char;
                    sprintf(buffer,
                            b"%s-%s-%s\x00" as *const u8 as
                                *const libc::c_char, cbp_str, pp, verb);
                    address_str =
                        get_ipal_address(buffer as *mut libc::c_uchar, flag);
                    free(buffer as *mut libc::c_void);
                    free(pp as *mut libc::c_void);
                    if cbp_str_malloc_flag != 0 {
                        free(cbp_str as *mut libc::c_void);
                    }
                }
                if pp.is_null() || address_str.is_null() {
                    address_str =
                        get_ipal_address(verb as *mut libc::c_uchar, flag)
                }
            } else {
                address_str =
                    get_ipal_address(verb as *mut libc::c_uchar, flag)
            }
        }
    } else {
        address_str = get_ipal_address(verb as *mut libc::c_uchar, flag)
    }
    /* なければ */
    if address_str.is_null() { return f_num; }
    if flag == 2 as libc::c_int &&
        {
            vtype =
                check_feature((*t_ptr).f,
                              b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                  as *const libc::c_char as
                                  *mut libc::c_char);
            !vtype.is_null()
        } {
        vtype =
            b"\xe5\x90\x8d\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else {
        vtype =
            check_feature((*t_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !vtype.is_null() {
            vtype =
                vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as
                    *const u8 as *const libc::c_char) as
                    isize)
        } else {
            vtype =
                check_feature((*t_ptr).f,
                              b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !vtype.is_null() {
                vtype =
                    vtype.offset(strlen(b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90:\x00"
                        as *const u8 as
                        *const libc::c_char) as isize)
            }
        }
    }
    let mut current_block_175: u64;
    pre_pos = address_str;
    cp = pre_pos;
    while f_num < (*sp).frame_num_max {
        if *cp as libc::c_int == '/' as i32 ||
            *cp as libc::c_int == '\u{0}' as i32 {
            if *cp as libc::c_int == '\u{0}' as i32 {
                break_flag = 1 as libc::c_int
            } else { *cp = '\u{0}' as i32 as libc::c_char }
            /* 格フレームの読みだし */
            match_0 =
                sscanf(pre_pos,
                       b"%llu:%d\x00" as *const u8 as *const libc::c_char,
                       &mut address as *mut libc::c_ulonglong,
                       &mut size as *mut libc::c_int);
            if match_0 != 2 as libc::c_int {
                fprintf(stderr,
                        b";; CaseFrame Dictionary Index error (it seems version 1.).\n\x00"
                            as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            if !(OptCaseFlag & 8192 as libc::c_int != 0 &&
                {
                    i_ptr = lookup_caseframe(address);
                    !i_ptr.is_null()
                })
            {
                /* 格フレームcacheを使う場合は、cacheを引く */
                i_ptr = get_ipal_frame(address, size, flag)
            }
            pre_pos = cp.offset(1 as libc::c_int as isize);
            /* 用言のタイプがマッチしなければ (準用言なら通過) */
            if !vtype.is_null() {
                if strncmp(vtype, (*i_ptr).pred_type.as_mut_ptr(),
                           strlen(b"\xe5\x90\x8d\x00" as *const u8 as
                               *const libc::c_char)) != 0 {
                    if break_flag != 0 { break; }
                    current_block_175 = 13321564401369230990;
                } else { current_block_175 = 2606304779496145856; }
            } else { current_block_175 = 2606304779496145856; }
            match current_block_175 {
                13321564401369230990 => {}
                _ => {
                    /* 能動態 or 格フレームに態が含まれる場合 */
                    if voice == 0 as libc::c_int {
                        /* CF_NOUNの場合、ここでマッチ */
                        (*cf_ptr.offset(f_num as isize)).voice =
                            1 as libc::c_int;
                        _make_ipal_cframe(i_ptr,
                                          cf_ptr.offset(f_num as isize),
                                          address, size, verb, flag);
                        /* 用言のときは、一般的外の関係名詞を追加 */
                        if OptCaseFlag & 64 as libc::c_int != 0 &&
                            flag == 1 as libc::c_int {
                            /* 外の関係がないとき */
                            c =
                                check_cf_case(cf_ptr.offset(f_num as isize),
                                              b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if c < 0 as libc::c_int {
                                if (*cf_ptr.offset(f_num as
                                    isize)).element_num ==
                                    24 as libc::c_int {
                                    /* 格の数が上限 */
                                    let ref mut fresh13 =
                                        (*cf_ptr.offset(f_num as
                                            isize)).element_num; /* 最後の格を削除 */
                                    *fresh13 -= 1;
                                    clear_cf_element(cf_ptr.offset(f_num as
                                        isize),
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num);
                                }
                                _make_ipal_cframe_pp(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     flag);
                                _make_ipal_cframe_ex(cf_ptr.offset(f_num as
                                    isize),
                                                     GENERAL_SOTO_WORDS as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     Thesaurus,
                                                     0 as libc::c_int,
                                                     (0 as libc::c_int == 0)
                                                         as libc::c_int);
                                let ref mut fresh14 =
                                    (*cf_ptr.offset(f_num as
                                        isize)).element_num;
                                *fresh14 += 1
                            } else {
                                /* 外の関係がすでにあるときは用例を追加 */
                                _make_ipal_cframe_ex(cf_ptr.offset(f_num as
                                    isize),
                                                     GENERAL_SOTO_WORDS as
                                                         *mut libc::c_uchar,
                                                     c, Thesaurus,
                                                     0 as libc::c_int,
                                                     0 as libc::c_int);
                            }
                        }
                        /* 以下 flag == CF_PRED のはず */
                        /* 格フレーム使役/格フレーム使役&受身*/
                        if (*t_ptr).voice & 1 as libc::c_int != 0 ||
                            (*t_ptr).voice & 4 as libc::c_int != 0 {
                            /* ニ格がないとき */
                            c =
                                check_cf_case(cf_ptr.offset(f_num as isize),
                                              b"\xe3\x83\x8b\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char);
                            if c < 0 as libc::c_int {
                                _make_ipal_cframe_pp(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe3\x83\x8b\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     flag);
                                _make_ipal_cframe_sm(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     if Thesaurus ==
                                                         2 as libc::c_int {
                                                         6 as libc::c_int
                                                     } else {
                                                         5 as libc::c_int
                                                     });
                                let ref mut fresh15 =
                                    (*cf_ptr.offset(f_num as
                                        isize)).element_num;
                                *fresh15 += 1
                            } else if sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                as *const u8 as
                                *const libc::c_char
                                as
                                *mut libc::c_char),
                                                (*cf_ptr.offset(f_num as
                                                    isize)).sm[c
                                                    as
                                                    usize],
                                                1 as libc::c_int) ==
                                0 as libc::c_int {
                                _make_ipal_cframe_sm(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     c,
                                                     if Thesaurus ==
                                                         2 as libc::c_int {
                                                         6 as libc::c_int
                                                     } else {
                                                         5 as libc::c_int
                                                     });
                            }
                            if (*t_ptr).voice & 1 as libc::c_int != 0 {
                                (*cf_ptr.offset(f_num as isize)).voice =
                                    7 as libc::c_int
                            } else if (*t_ptr).voice & 4 as libc::c_int != 0 {
                                (*cf_ptr.offset(f_num as isize)).voice =
                                    8 as libc::c_int
                            }
                        } else if (*t_ptr).voice & 2 as libc::c_int != 0 ||
                            (*t_ptr).voice & 32 as libc::c_int != 0
                        {
                            /* ニ格はあるけど<主体>がないとき */
                            /* 格フレーム受身 */
                            /* ニ/ニヨル/カラ格がないとき */
                            c =
                                check_cf_case(cf_ptr.offset(f_num as isize),
                                              b"\xe3\x83\x8b\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char);
                            if c < 0 as libc::c_int &&
                                {
                                    c =
                                        check_cf_case(cf_ptr.offset(f_num
                                            as
                                            isize),
                                                      b"\xe3\x83\x8b\xe3\x83\xa8\xe3\x83\xab\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    (c) < 0 as libc::c_int
                                } &&
                                {
                                    c =
                                        check_cf_case(cf_ptr.offset(f_num
                                            as
                                            isize),
                                                      b"\xe3\x82\xab\xe3\x83\xa9\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    (c) < 0 as libc::c_int
                                } {
                                _make_ipal_cframe_pp(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe3\x83\x8b\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     flag);
                                _make_ipal_cframe_sm(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     (*cf_ptr.offset(f_num as
                                                         isize)).element_num,
                                                     if Thesaurus ==
                                                         2 as libc::c_int {
                                                         6 as libc::c_int
                                                     } else {
                                                         5 as libc::c_int
                                                     });
                                let ref mut fresh16 =
                                    (*cf_ptr.offset(f_num as
                                        isize)).element_num;
                                *fresh16 += 1
                            } else if sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                as *const u8 as
                                *const libc::c_char
                                as
                                *mut libc::c_char),
                                                (*cf_ptr.offset(f_num as
                                                    isize)).sm[c
                                                    as
                                                    usize],
                                                1 as libc::c_int) ==
                                0 as libc::c_int {
                                _make_ipal_cframe_sm(cf_ptr.offset(f_num as
                                    isize),
                                                     b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_uchar,
                                                     c,
                                                     if Thesaurus ==
                                                         2 as libc::c_int {
                                                         6 as libc::c_int
                                                     } else {
                                                         5 as libc::c_int
                                                     });
                            }
                            (*cf_ptr.offset(f_num as isize)).voice =
                                3 as libc::c_int
                        }
                        f_num_inc(start, &mut f_num);
                        cf_ptr = Case_frame_array.offset(start as isize)
                    }
                    /* ニ/ニヨル/カラ格はあるけど<主体>がないとき */
                    /* 使役 */
                    if voice & 1 as libc::c_int != 0 {
                        if (*i_ptr).voice & 1 as libc::c_int != 0 &&
                            (*i_ptr).voice & 2 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                5 as libc::c_int
                        } else if (*i_ptr).voice & 1 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                6 as libc::c_int
                        } else if (*i_ptr).voice & 2 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                7 as libc::c_int
                        }
                        _make_ipal_cframe(i_ptr,
                                          cf_ptr.offset(f_num as isize),
                                          address, size, verb, flag);
                        f_num_inc(start, &mut f_num);
                        cf_ptr = Case_frame_array.offset(start as isize)
                    }
                    /* 受身 */
                    if voice & 2 as libc::c_int != 0 {
                        /* 直接受身１ */
                        if (*i_ptr).voice & 4 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                3 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                        /* 直接受身２ */
                        if (*i_ptr).voice & 8 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                4 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                        /* 間接受身 */
                        if (*i_ptr).voice & 16 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                2 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                    }
                    /* もらう/ほしい */
                    if voice & 8 as libc::c_int != 0 ||
                        voice & 16 as libc::c_int != 0 {
                        /* ニ使役 (間接受身でも同じ */
                        if (*i_ptr).voice & 2 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                7 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                    }
                    /* せられる/させられる */
                    if voice & 4 as libc::c_int != 0 {
                        (*cf_ptr.offset(f_num as isize)).voice =
                            8 as libc::c_int;
                        _make_ipal_cframe(i_ptr,
                                          cf_ptr.offset(f_num as isize),
                                          address, size, verb, flag);
                        f_num_inc(start, &mut f_num);
                        cf_ptr = Case_frame_array.offset(start as isize)
                    }
                    /* 可能，尊敬，自発 */
                    if voice & 2 as libc::c_int != 0 {
                        if (*i_ptr).voice & 32 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                9 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                        if (*i_ptr).voice & 64 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                10 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                        if (*i_ptr).voice & 128 as libc::c_int != 0 {
                            (*cf_ptr.offset(f_num as isize)).voice =
                                11 as libc::c_int;
                            _make_ipal_cframe(i_ptr,
                                              cf_ptr.offset(f_num as isize),
                                              address, size, verb, flag);
                            f_num_inc(start, &mut f_num);
                            cf_ptr = Case_frame_array.offset(start as isize)
                        }
                    }
                    if break_flag != 0 { break; }
                }
            }
        }
        cp = cp.offset(1)
    }
    free(address_str as *mut libc::c_void);
    return f_num;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_ipal_cframe_subcontract(mut sp:
                                                      *mut SENTENCE_DATA,
                                                      mut t_ptr:
                                                      *mut TAG_DATA,
                                                      mut m_ptr:
                                                      *mut MRPH_DATA,
                                                      mut orig_form:
                                                      *mut libc::c_char,
                                                      mut start: libc::c_int,
                                                      mut flag: libc::c_int)
                                                      -> libc::c_int
/*==================================================================*/
{
    let mut f_num: libc::c_int = 0 as libc::c_int;
    let mut plus_num: libc::c_int = 0;
    let mut verb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pred_string: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptCaseFlag & 131072 as libc::c_int != 0 {
        pred_string =
            make_pred_string_from_mrph(t_ptr, m_ptr, orig_form,
                                       OptCaseFlag & 32 as libc::c_int, flag,
                                       0 as libc::c_int)
    } else {
        pred_string =
            make_pred_string(t_ptr, m_ptr, orig_form,
                             OptCaseFlag & 32 as libc::c_int, flag,
                             0 as libc::c_int)
    }
    if flag == 2 as libc::c_int {
        f_num =
            _make_ipal_cframe_subcontract(sp, t_ptr, start, pred_string,
                                          0 as libc::c_int, flag,
                                          0 as libc::c_int);
        free(pred_string as *mut libc::c_void);
        return f_num;
    } else {
        if OptCaseFlag & 131072 as libc::c_int != 0 {
            /* 用言代表表記版(受身など込み)格フレーム */
            /* CF_PRED: 直前格要素をくっつけて検索 → ない場合はくっつけないで検索 */
            f_num =
                _make_ipal_cframe_subcontract(sp, t_ptr, start, pred_string,
                                              0 as libc::c_int, flag,
                                              (0 as libc::c_int == 0) as
                                                  libc::c_int);
            if f_num == 0 as libc::c_int {
                f_num =
                    _make_ipal_cframe_subcontract(sp, t_ptr, start,
                                                  pred_string,
                                                  0 as libc::c_int, flag,
                                                  0 as libc::c_int);
                if f_num == 0 as libc::c_int {
                    free(pred_string as *mut libc::c_void);
                    /* 主辞だけで用言表記を作り、態変換をする */
                    pred_string =
                        make_pred_string_from_mrph(t_ptr, m_ptr, orig_form,
                                                   OptCaseFlag &
                                                       32 as libc::c_int,
                                                   flag,
                                                   (0 as libc::c_int == 0) as
                                                       libc::c_int); /* 能動態でtry */
                    f_num =
                        _make_ipal_cframe_subcontract(sp, t_ptr, start,
                                                      pred_string,
                                                      (*t_ptr).voice, flag,
                                                      0 as libc::c_int)
                }
            }
            free(pred_string as *mut libc::c_void);
            return f_num;
        }
    }
    verb =
        malloc_data(strlen(pred_string).wrapping_add(4 as libc::c_int as
            libc::c_ulong),
                    b"make_ipal_cframe_subcontract\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    strcpy(verb, pred_string);
    if (*t_ptr).voice == 32 as libc::c_int {
        (*t_ptr).voice = 0 as libc::c_int;
        f_num =
            _make_ipal_cframe_subcontract(sp, t_ptr, start, verb,
                                          0 as libc::c_int, flag,
                                          (0 as libc::c_int == 0) as
                                              libc::c_int);
        /* 今のところ受身の場合を考えない */
        free(verb as *mut libc::c_void);
        free(pred_string as *mut libc::c_void);
        return f_num;
    }
    /* 受身, 使役の格フレーム */
    if (*t_ptr).voice == 32 as libc::c_int ||
        (*t_ptr).voice & 2 as libc::c_int != 0 ||
        (*t_ptr).voice & 1 as libc::c_int != 0 ||
        (*t_ptr).voice & 4 as libc::c_int != 0 {
        let mut suffix: libc::c_int = 0 as libc::c_int;
        if (*t_ptr).voice & 1 as libc::c_int != 0 {
            strcat(verb, b":C\x00" as *const u8 as *const libc::c_char);
            suffix = 2 as libc::c_int
        } else if (*t_ptr).voice & 2 as libc::c_int != 0 ||
            (*t_ptr).voice & 32 as libc::c_int != 0 {
            strcat(verb, b":P\x00" as *const u8 as *const libc::c_char);
            suffix = 2 as libc::c_int
        } else if (*t_ptr).voice & 4 as libc::c_int != 0 {
            strcat(verb, b":PC\x00" as *const u8 as *const libc::c_char);
            suffix = 3 as libc::c_int
        }
        plus_num =
            _make_ipal_cframe_subcontract(sp, t_ptr, start + f_num, verb,
                                          0 as libc::c_int, flag,
                                          (0 as libc::c_int == 0) as
                                              libc::c_int);
        if plus_num != 0 as libc::c_int {
            free(verb as *mut libc::c_void);
            free(pred_string as *mut libc::c_void);
            return f_num + plus_num;
        }
        *verb.offset(strlen(verb) as isize).offset(-(suffix as isize)) =
            '\u{0}' as i32 as libc::c_char
        /* みつからなかったらもとにもどす */
    }
    if (*t_ptr).voice == 32 as libc::c_int {
        f_num +=
            _make_ipal_cframe_subcontract(sp, t_ptr, start + f_num, verb,
                                          2 as libc::c_int, flag,
                                          (0 as libc::c_int == 0) as
                                              libc::c_int)
        /* 受身 */
    } else {
        f_num =
            _make_ipal_cframe_subcontract(sp, t_ptr, start, verb,
                                          (*t_ptr).voice, flag,
                                          (0 as libc::c_int == 0) as
                                              libc::c_int)
    }
    free(verb as *mut libc::c_void);
    free(pred_string as *mut libc::c_void);
    return f_num;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_pred_type(mut t_ptr: *mut TAG_DATA)
                                        -> *mut libc::c_char
/*==================================================================*/
{
    /* 用言タイプをかえす */
    return if (*t_ptr).voice == 32 as libc::c_int {
        /* 文末のサ変名詞など */
        b":?\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    } else if (*t_ptr).voice & 1 as libc::c_int != 0 {
        b":C\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    } else if (*t_ptr).voice & 2 as libc::c_int != 0 {
        b":P\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    } else if (*t_ptr).voice & 4 as libc::c_int != 0 {
        b":PC\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    } else {
        b"\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_pred_string(mut t_ptr: *mut TAG_DATA,
                                          mut m_ptr: *mut MRPH_DATA,
                                          mut orig_form: *mut libc::c_char,
                                          mut use_rep_flag: libc::c_int,
                                          mut cf_type: libc::c_int,
                                          mut cpncf_flag: libc::c_int)
                                          -> *mut libc::c_char
/*==================================================================*/
{
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut main_pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_length: libc::c_int = 0;
    let mut main_pred_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /* orig_form == 1: 可能動詞のもとの形を用いるとき */
    /* m_ptr == NULL: 本動詞の形態素は t_ptr->head_ptr を用いる
       otherwise    : 本動詞の形態素として m_ptr を用いる (ALTのもの) */
    /* cpncf_flag: 複合名詞格フレーム (下で設定) or 
                   用言表記 == 主辞のみ (引数)   ならTRUE
		   ※ for future extension */
    /* 用言タイプ, voiceの分(7)も確保しておく */
    /* 代表表記を使う場合で代表表記があるとき */
    if use_rep_flag != 0 {
        if !m_ptr.is_null() {
            rep_strt = get_mrph_rep(m_ptr);
            rep_length = get_mrph_rep_length(rep_strt);
            if rep_length != 0 {
                main_pred =
                    malloc_data((rep_length + 1 as libc::c_int) as size_t,
                                b"make_pred_string\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
                *main_pred.offset(rep_length as isize) =
                    '\u{0}' as i32 as libc::c_char;
                main_pred_malloc_flag = 1 as libc::c_int
            }
        } else if cf_type == 2 as libc::c_int && OptAnaphora != 0 &&
            {
                cp =
                    check_feature((*(*t_ptr).b_ptr).f,
                                  b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                      as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                !cp.is_null()
            } {
            cpncf_flag = (0 as libc::c_int == 0) as libc::c_int;
            rep_strt =
                cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                    as *const u8 as *const libc::c_char) as
                    isize);
            rep_length = strlen(rep_strt) as libc::c_int;
            main_pred =
                malloc_data((rep_length + 1 as libc::c_int) as size_t,
                            b"make_pred_string\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
            *main_pred.offset(rep_length as isize) =
                '\u{0}' as i32 as libc::c_char;
            main_pred_malloc_flag = 1 as libc::c_int
        } else if cf_type == 2 as libc::c_int && OptUseCPNCF != 0 &&
            check_feature((*(*t_ptr).head_ptr).f,
                          b"\xe7\x89\xb9\xe6\xae\x8a\xe9\x9d\x9e\xe8\xa6\x8b\xe5\x87\xba\xe8\xaa\x9e\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            {
                cp =
                    check_feature((*t_ptr).f,
                                  b"BGH\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                !cp.is_null()
            } &&
            strstr(cp,
                   b"|\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            /* 連想照応解析をする場合は正規化代表表記を使用する */
            /* 複合名詞格フレームを用いる場合で、分類語彙表が引けている場合 */
            /* ただし、BGH中に"|"が含まれている場合は除く */
            /* 形容詞語幹の場合は通常の代表表記を使用する */
            if !check_feature((*t_ptr).f,
                              b"\xe5\x90\x8d\xe8\xa9\x9e\xe7\x9a\x84\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\xe8\xaa\x9e\xe5\xb9\xb9\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                rep_strt = get_mrph_rep((*t_ptr).head_ptr);
                rep_length = get_mrph_rep_length(rep_strt)
            } else {
                /* 複合名詞格フレームを用いる場合は分類語彙表の見出しを用いる */
                cpncf_flag = (0 as libc::c_int == 0) as libc::c_int;
                rep_strt =
                    cp.offset(strlen(b"BGH:\x00" as *const u8 as
                        *const libc::c_char) as isize);
                rep_length = strlen(rep_strt) as libc::c_int;
                /* 後方の基本句の分類語彙表の見出しに含まれる基本句は省略解析の対象としない */
                /* 分類語彙表の見出しが処理対象の基本句より長い場合のみ実行*/
                cp = get_mrph_rep((*t_ptr).head_ptr);
                if 0 as libc::c_int != 0 && !cp.is_null() &&
                    strncmp(cp, rep_strt,
                            strlen(cp).wrapping_sub(1 as libc::c_int as
                                libc::c_ulong)) !=
                        0 {
                    i = 1 as libc::c_int;
                    while !t_ptr.offset(-(i as isize)).is_null() &&
                        !(*t_ptr.offset(-(i as
                            isize))).head_ptr.is_null()
                    {
                        assign_cfeature(&mut (*t_ptr.offset(-(i as isize))).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                        if !check_feature((*(*t_ptr.offset(-(i as
                            isize))).head_ptr).f,
                                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            break;
                        }
                        cp =
                            get_mrph_rep((*t_ptr.offset(-(i as
                                isize))).head_ptr);
                        if cp.is_null() ||
                            strncmp(cp, rep_strt,
                                    strlen(cp).wrapping_sub(1 as
                                        libc::c_int
                                        as
                                        libc::c_ulong))
                                == 0 {
                            break;
                        }
                        i += 1
                    }
                }
            }
            if rep_length != 0 {
                main_pred =
                    malloc_data((rep_length + 1 as libc::c_int) as size_t,
                                b"make_pred_string\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
                *main_pred.offset(rep_length as isize) =
                    '\u{0}' as i32 as libc::c_char;
                main_pred_malloc_flag = 1 as libc::c_int
            }
        } else if cf_type & 1 as libc::c_int != 0 &&
            strcmp(Class[(*(*t_ptr).head_ptr).Hinshi as
                usize][(*(*t_ptr).head_ptr).Bunrui as
                usize].id as
                       *const libc::c_char,
                   b"\xe5\x90\x8d\xe8\xa9\x9e\xe6\x80\xa7\xe5\x90\x8d\xe8\xa9\x9e\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                       as *const u8 as *const libc::c_char) == 0 {
            main_pred =
                get_mrph_rep_from_f((*t_ptr).head_ptr.offset(-(1 as
                    libc::c_int
                    as isize)),
                                    (cf_type & 1 as libc::c_int != 0 &&
                                        !check_feature((*(*t_ptr).head_ptr.offset(-(1
                                            as
                                            libc::c_int
                                            as
                                            isize))).f,
                                                       b"\xef\xbc\xb4\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\xe5\x89\x8d\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char).is_null())
                                        as libc::c_int);
            if main_pred.is_null() {
                main_pred =
                    make_mrph_rn((*t_ptr).head_ptr.offset(-(1 as libc::c_int
                        as isize)));
                main_pred_malloc_flag = 1 as libc::c_int
            }
        } else {
            /* 用言のとき、末尾の名詞性名詞接尾辞は無視する */
            /* 用言のとき、a化している形容詞語幹は、元の代表表記で引く(e.g., 「平和/条約」の「平和」) */
            main_pred =
                get_mrph_rep_from_f((*t_ptr).head_ptr,
                                    (cf_type & 1 as libc::c_int != 0 &&
                                        !check_feature((*(*t_ptr).head_ptr).f,
                                                       b"\xef\xbc\xb4\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\xe5\x89\x8d\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char).is_null())
                                        as libc::c_int);
            if main_pred.is_null() {
                main_pred = make_mrph_rn((*t_ptr).head_ptr);
                main_pred_malloc_flag = 1 as libc::c_int
            }
        }
    }
    if main_pred.is_null() {
        main_pred =
            if !m_ptr.is_null() {
                (*m_ptr).Goi.as_mut_ptr()
            } else { (*(*t_ptr).head_ptr).Goi.as_mut_ptr() }
    }
    /* 「（〜を）〜に」 のときは 「する」 で探す */
    if !check_feature((*t_ptr).f,
                      b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() && cpncf_flag == 0 {
        buffer =
            malloc_data(strlen(b"\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00"
                as *const u8 as
                *const libc::c_char).wrapping_add(8 as
                libc::c_int
                as
                libc::c_ulong),
                        b"make_pred_string\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char; /* 9(euc) + 8 */
        if use_rep_flag != 0 {
            strcpy(buffer,
                   b"\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00" as
                       *const u8 as *const libc::c_char);
        } else {
            strcpy(buffer,
                   b"\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
                       *const libc::c_char);
        }
    } else if !check_feature((*t_ptr).f,
                             b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe2\x86\x92\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
        cpncf_flag == 0 &&
        (check_feature((*(*t_ptr).head_ptr).f,
                       b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() ||
            {
                cp =
                    get_mrph_rep_from_f((*t_ptr).head_ptr.offset(1
                        as
                        libc::c_int
                        as
                        isize),
                                        cf_type);
                (!cp.is_null()) &&
                    strcmp(cp,
                           b"\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00"
                               as *const u8 as *const libc::c_char)
                        != 0
            }) {
        if use_rep_flag != 0 {
            cp =
                get_mrph_rep_from_f((*t_ptr).head_ptr.offset(1 as libc::c_int
                    as isize),
                                    cf_type);
            if !cp.is_null() {
                buffer =
                    malloc_data(strlen(main_pred).wrapping_add(strlen(cp)).wrapping_add(9
                        as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"make_pred_string\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                strcpy(buffer, main_pred);
                strcat(buffer, b"+\x00" as *const u8 as *const libc::c_char);
                strcat(buffer, cp);
            } else {
                buffer =
                    malloc_data(strlen(main_pred).wrapping_add(strlen((*(*t_ptr).head_ptr.offset(1
                        as
                        libc::c_int
                        as
                        isize)).Goi.as_mut_ptr())).wrapping_add(9
                        as
                        libc::c_int
                        as
                        libc::c_ulong),
                                b"make_pred_string\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                strcpy(buffer, main_pred);
                strcat(buffer, b"+\x00" as *const u8 as *const libc::c_char);
                strcat(buffer,
                       (*(*t_ptr).head_ptr.offset(1 as libc::c_int as
                           isize)).Goi.as_mut_ptr());
            }
        } else {
            buffer =
                malloc_data(strlen((*(*t_ptr).head_ptr).Goi2.as_mut_ptr()).wrapping_add(strlen((*(*t_ptr).head_ptr.offset(1
                    as
                    libc::c_int
                    as
                    isize)).Goi.as_mut_ptr())).wrapping_add(8
                    as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"make_pred_string\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            strcpy(buffer, (*(*t_ptr).head_ptr).Goi2.as_mut_ptr());
            strcat(buffer,
                   (*(*t_ptr).head_ptr.offset(1 as libc::c_int as
                       isize)).Goi.as_mut_ptr());
        }
    } else if !check_feature((*t_ptr).f,
                             b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe2\x86\x90\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
        cpncf_flag == 0 &&
        check_feature((*(*t_ptr).head_ptr.offset(-(1 as libc::c_int
            as
            isize))).f,
                      b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as *const u8
                          as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        if use_rep_flag != 0 &&
            {
                cp =
                    get_mrph_rep_from_f((*t_ptr).head_ptr.offset(-(1 as
                        libc::c_int
                        as
                        isize)),
                                        0 as libc::c_int);
                !cp.is_null()
            } {
            buffer =
                malloc_data(strlen(cp).wrapping_add(strlen(main_pred)).wrapping_add(9
                    as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"make_pred_string\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            strcpy(buffer, cp);
            strcat(buffer, b"+\x00" as *const u8 as *const libc::c_char);
        } else {
            buffer =
                malloc_data(strlen((*(*t_ptr).head_ptr.offset(-(1 as
                    libc::c_int
                    as
                    isize))).Goi2.as_mut_ptr()).wrapping_add(strlen(main_pred)).wrapping_add(8
                    as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"make_pred_string\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            strcpy(buffer,
                   (*(*t_ptr).head_ptr.offset(-(1 as libc::c_int as
                       isize))).Goi2.as_mut_ptr());
        }
        strcat(buffer, main_pred);
    } else if !orig_form.is_null() {
        buffer =
            malloc_data(strlen(orig_form).wrapping_add(8 as libc::c_int as
                libc::c_ulong),
                        b"make_pred_string\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        strcpy(buffer, orig_form);
    } else {
        buffer =
            malloc_data(strlen(main_pred).wrapping_add(8 as libc::c_int as
                libc::c_ulong),
                        b"make_pred_string\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        strcpy(buffer, main_pred);
    }
    if main_pred_malloc_flag != 0 { free(main_pred as *mut libc::c_void); }
    return buffer;
}
/* 「形容詞+なる」など */
/* 「形容詞語幹+的だ」など */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_pred_string_from_mrph(mut t_ptr: *mut TAG_DATA,
                                                    mut m_ptr: *mut MRPH_DATA,
                                                    mut orig_form:
                                                    *mut libc::c_char,
                                                    mut use_rep_flag:
                                                    libc::c_int,
                                                    mut cf_type: libc::c_int,
                                                    mut cpncf_flag:
                                                    libc::c_int)
                                                    -> *mut libc::c_char
/*==================================================================*/
{
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut main_pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_length: libc::c_int = 0;
    let mut main_pred_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /* orig_form == 1: 可能動詞のもとの形を用いるとき */
    /* m_ptr == NULL: 本動詞の形態素は t_ptr->head_ptr を用いる
       otherwise    : 本動詞の形態素として m_ptr を用いる (ALTのもの) */
    /* cpncf_flag: 複合名詞格フレーム (下で設定) or 
                   用言表記 == 主辞のみ (引数)   ならTRUE */
    /* 用言タイプ, voiceの分(7)も確保しておく */
    /* 代表表記を使う場合で代表表記があるとき */
    if use_rep_flag != 0 {
        if !m_ptr.is_null() {
            rep_strt = get_mrph_rep(m_ptr);
            rep_length = get_mrph_rep_length(rep_strt);
            if rep_length != 0 {
                main_pred =
                    malloc_data((rep_length + 1 as libc::c_int) as size_t,
                                b"make_pred_string_from_mrph\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char) as *mut libc::c_char;
                strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
                *main_pred.offset(rep_length as isize) =
                    '\u{0}' as i32 as libc::c_char;
                main_pred_malloc_flag = 1 as libc::c_int
            }
        } else if cf_type == 2 as libc::c_int && OptAnaphora != 0 &&
            {
                cp =
                    check_feature((*(*t_ptr).b_ptr).f,
                                  b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                      as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                !cp.is_null()
            } {
            cpncf_flag = (0 as libc::c_int == 0) as libc::c_int;
            rep_strt =
                cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                    as *const u8 as *const libc::c_char) as
                    isize);
            rep_length = strlen(rep_strt) as libc::c_int;
            main_pred =
                malloc_data((rep_length + 1 as libc::c_int) as size_t,
                            b"make_pred_string_from_mrph\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
            *main_pred.offset(rep_length as isize) =
                '\u{0}' as i32 as libc::c_char;
            main_pred_malloc_flag = 1 as libc::c_int
        } else if cf_type == 2 as libc::c_int && OptUseCPNCF != 0 &&
            check_feature((*(*t_ptr).head_ptr).f,
                          b"\xe7\x89\xb9\xe6\xae\x8a\xe9\x9d\x9e\xe8\xa6\x8b\xe5\x87\xba\xe8\xaa\x9e\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            {
                cp =
                    check_feature((*t_ptr).f,
                                  b"BGH\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                !cp.is_null()
            } &&
            strstr(cp,
                   b"|\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            /* 連想照応解析をする場合は正規化代表表記を使用する */
            /* 複合名詞格フレームを用いる場合で、分類語彙表が引けている場合 */
            /* ただし、BGH中に"|"が含まれている場合は除く */
            /* 形容詞語幹の場合は通常の代表表記を使用する */
            if !check_feature((*t_ptr).f,
                              b"\xe5\x90\x8d\xe8\xa9\x9e\xe7\x9a\x84\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\xe8\xaa\x9e\xe5\xb9\xb9\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                rep_strt = get_mrph_rep((*t_ptr).head_ptr);
                rep_length = get_mrph_rep_length(rep_strt)
            } else {
                /* 複合名詞格フレームを用いる場合は分類語彙表の見出しを用いる */
                cpncf_flag = (0 as libc::c_int == 0) as libc::c_int;
                rep_strt =
                    cp.offset(strlen(b"BGH:\x00" as *const u8 as
                        *const libc::c_char) as isize);
                rep_length = strlen(rep_strt) as libc::c_int;
                /* 後方の基本句の分類語彙表の見出しに含まれる基本句は省略解析の対象としない */
                /* 分類語彙表の見出しが処理対象の基本句より長い場合のみ実行*/
                cp = get_mrph_rep((*t_ptr).head_ptr);
                if 0 as libc::c_int != 0 && !cp.is_null() &&
                    strncmp(cp, rep_strt,
                            strlen(cp).wrapping_sub(1 as libc::c_int as
                                libc::c_ulong)) !=
                        0 {
                    i = 1 as libc::c_int;
                    while !t_ptr.offset(-(i as isize)).is_null() &&
                        !(*t_ptr.offset(-(i as
                            isize))).head_ptr.is_null()
                    {
                        assign_cfeature(&mut (*t_ptr.offset(-(i as isize))).f,
                                        b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        0 as libc::c_int);
                        if !check_feature((*(*t_ptr.offset(-(i as
                            isize))).head_ptr).f,
                                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            break;
                        }
                        cp =
                            get_mrph_rep((*t_ptr.offset(-(i as
                                isize))).head_ptr);
                        if cp.is_null() ||
                            strncmp(cp, rep_strt,
                                    strlen(cp).wrapping_sub(1 as
                                        libc::c_int
                                        as
                                        libc::c_ulong))
                                == 0 {
                            break;
                        }
                        i += 1
                    }
                }
            }
            if rep_length != 0 {
                main_pred =
                    malloc_data((rep_length + 1 as libc::c_int) as size_t,
                                b"make_pred_string_from_mrph\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char) as *mut libc::c_char;
                strncpy(main_pred, rep_strt, rep_length as libc::c_ulong);
                *main_pred.offset(rep_length as isize) =
                    '\u{0}' as i32 as libc::c_char;
                main_pred_malloc_flag = 1 as libc::c_int
            }
        } else {
            /* 用言のとき、a化している形容詞語幹は、元の代表表記で引く(e.g., 「平和/条約」の「平和」) */
            main_pred =
                get_mrph_rep_from_f((*t_ptr).head_ptr,
                                    (cf_type & 1 as libc::c_int != 0 &&
                                        !check_feature((*(*t_ptr).head_ptr).f,
                                                       b"\xef\xbc\xb4\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\xe5\x89\x8d\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char).is_null())
                                        as libc::c_int);
            if main_pred.is_null() {
                main_pred = make_mrph_rn((*t_ptr).head_ptr);
                main_pred_malloc_flag = 1 as libc::c_int
            }
        }
    }
    if main_pred.is_null() {
        main_pred =
            if !m_ptr.is_null() {
                (*m_ptr).Goi.as_mut_ptr()
            } else { (*(*t_ptr).head_ptr).Goi.as_mut_ptr() }
    }
    /* 「（〜を）〜に」 のときは 「する」 で探す */
    if !check_feature((*t_ptr).f,
                      b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() && cpncf_flag == 0 {
        buffer =
            malloc_data(strlen(b"\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00"
                as *const u8 as
                *const libc::c_char).wrapping_add(8 as
                libc::c_int
                as
                libc::c_ulong),
                        b"make_pred_string_from_mrph\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char; /* 9(euc) + 8 */
        if use_rep_flag != 0 {
            strcpy(buffer,
                   b"\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00" as
                       *const u8 as *const libc::c_char);
        } else {
            strcpy(buffer,
                   b"\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
                       *const libc::c_char);
        }
    } else {
        /* メモリ領域計算 */
        let mut mrph_ptr: *mut MRPH_DATA = (*t_ptr).head_ptr;
        let mut length: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*t_ptr).mrph_num {
            if !(use_rep_flag != 0 &&
                {
                    cp =
                        get_mrph_rep_from_f((*t_ptr).mrph_ptr.offset(i as
                            isize),
                                            0 as libc::c_int);
                    !cp.is_null()
                }) {
                if (*t_ptr).mrph_ptr.offset(i as isize) < (*t_ptr).head_ptr {
                    /* 主辞より前側 */
                    cp =
                        (*(*t_ptr).mrph_ptr.offset(i as
                            isize)).Goi2.as_mut_ptr()
                } else {
                    cp =
                        (*(*t_ptr).mrph_ptr.offset(i as
                            isize)).Goi.as_mut_ptr()
                }
            }
            length =
                (length as
                    libc::c_ulong).wrapping_add(strlen(cp).wrapping_add(1 as
                    libc::c_int
                    as
                    libc::c_ulong))
                    as libc::c_int as libc::c_int;
            cp =
                check_feature((*(*t_ptr).mrph_ptr.offset(i as isize)).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                cp =
                    cp.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e:\x00"
                        as *const u8 as *const libc::c_char)
                        as isize);
                length =
                    (length as
                        libc::c_ulong).wrapping_add(strlen(cp).wrapping_add(1
                        as
                        libc::c_int
                        as
                        libc::c_ulong))
                        as libc::c_int as libc::c_int
            }
            i += 1
        }
        buffer =
            malloc_data((length as
                libc::c_ulong).wrapping_add(strlen(main_pred)).wrapping_add(8
                as
                libc::c_int
                as
                libc::c_ulong),
                        b"make_pred_string_from_mrph\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        *buffer.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        let mut is_assigned_pred_begin_feature: libc::c_int =
            0 as libc::c_int;
        /* 前側に延ばす場合: 「形容詞語幹+的だ」など */
        if cpncf_flag == 0 {
            mrph_ptr = (*t_ptr).mrph_ptr;
            while mrph_ptr < (*t_ptr).head_ptr &&
                !check_feature((*mrph_ptr.offset(1 as libc::c_int as
                    isize)).f,
                               b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe2\x86\x90\x00"
                                   as *const u8 as *const libc::c_char
                                   as *mut libc::c_char).is_null() {
                if is_assigned_pred_begin_feature == 0 {
                    /* 用言代表表記の先頭形態素にマーク */
                    assign_cfeature(&mut (*mrph_ptr).f,
                                    b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa1\xa8\xe8\xa8\x98\xe5\x85\x88\xe9\xa0\xad\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                    is_assigned_pred_begin_feature = 1 as libc::c_int
                }
                if use_rep_flag != 0 &&
                    {
                        cp =
                            get_mrph_rep_from_f(mrph_ptr,
                                                0 as libc::c_int);
                        !cp.is_null()
                    } {
                    strcat(buffer, cp);
                } else {
                    strcat(buffer, (*mrph_ptr).Goi2.as_mut_ptr());
                    /* 出現形 */
                }
                strcat(buffer, b"+\x00" as *const u8 as *const libc::c_char);
                mrph_ptr = mrph_ptr.offset(1)
            }
        }
        /* 主辞 */
        if cpncf_flag == 0 && is_assigned_pred_begin_feature == 0 {
            /* 用言代表表記の先頭形態素にマーク */
            assign_cfeature(&mut (*(*t_ptr).head_ptr).f,
                            b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa1\xa8\xe8\xa8\x98\xe5\x85\x88\xe9\xa0\xad\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            is_assigned_pred_begin_feature = 1 as libc::c_int
        }
        if !orig_form.is_null() {
            strcat(buffer, orig_form);
        } else { strcat(buffer, main_pred); }
        /* 用言見出接辞(テ形)の区別: ~テ形 */
        cp =
            check_feature((*mrph_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            cp =
                cp.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e:\x00"
                    as *const u8 as *const libc::c_char) as
                    isize);
            strcat(buffer, b"~\x00" as *const u8 as *const libc::c_char);
            strcat(buffer, cp);
        }
        /* 後側に延ばす場合: 「形容詞+なる」など */
        if cpncf_flag == 0 {
            mrph_ptr = (*t_ptr).head_ptr;
            while mrph_ptr <
                (*t_ptr).mrph_ptr.offset((*t_ptr).mrph_num as
                    isize).offset(-(1 as
                    libc::c_int
                    as
                    isize))
                &&
                !check_feature((*mrph_ptr).f,
                               b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe2\x86\x92\x00"
                                   as *const u8 as *const libc::c_char
                                   as *mut libc::c_char).is_null() {
                if strcmp(Class[(*mrph_ptr.offset(1 as libc::c_int as
                    isize)).Hinshi as
                    usize][0 as libc::c_int as usize].id as
                              *const libc::c_char,
                          b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                    /* 助詞はスキップ (「売ってはない」など) */
                    mrph_ptr = mrph_ptr.offset(1)
                } else {
                    strcat(buffer,
                           b"+\x00" as *const u8 as *const libc::c_char);
                    if use_rep_flag != 0 &&
                        {
                            cp =
                                get_mrph_rep_from_f(mrph_ptr.offset(1 as
                                    libc::c_int
                                    as
                                    isize),
                                                    cf_type &
                                                        1 as libc::c_int);
                            !cp.is_null()
                        } {
                        strcat(buffer, cp);
                    } else {
                        strcat(buffer,
                               (*mrph_ptr.offset(1 as libc::c_int as
                                   isize)).Goi.as_mut_ptr());
                    }
                    /* 用言見出接辞(テ形)の区別: ~テ形 */
                    cp =
                        check_feature((*mrph_ptr.offset(1 as libc::c_int as
                            isize)).f,
                                      b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char);
                    if !cp.is_null() {
                        cp =
                            cp.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa6\x8b\xe5\x87\xba\xe6\x8e\xa5\xe8\xbe\x9e:\x00"
                                as *const u8 as
                                *const libc::c_char) as
                                isize);
                        strcat(buffer,
                               b"~\x00" as *const u8 as *const libc::c_char);
                        strcat(buffer, cp);
                    }
                    mrph_ptr = mrph_ptr.offset(1)
                }
            }
            /* 用言代表表記の末尾形態素にマーク */
            if check_feature((*mrph_ptr).f,
                             b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa1\xa8\xe8\xa8\x98\xe6\x9c\xab\xe5\xb0\xbe\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
                assign_cfeature(&mut (*mrph_ptr).f,
                                b"\xe7\x94\xa8\xe8\xa8\x80\xe8\xa1\xa8\xe8\xa8\x98\xe6\x9c\xab\xe5\xb0\xbe\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
            }
            /* 用言意味表記末尾を与える (現状は<否定>をもつ最後の形態素) */
            let mut semantic_mrph_ptr: *mut MRPH_DATA = mrph_ptr;
            i = (*t_ptr).mrph_num - 1 as libc::c_int;
            while (*t_ptr).mrph_ptr.offset(i as isize) > mrph_ptr {
                if !check_feature((*(*t_ptr).mrph_ptr.offset(i as isize)).f,
                                  b"\xe5\x90\xa6\xe5\xae\x9a\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    semantic_mrph_ptr = (*t_ptr).mrph_ptr.offset(i as isize);
                    break;
                } else { i -= 1 }
            }
            assign_cfeature(&mut (*semantic_mrph_ptr).f,
                            b"\xe7\x94\xa8\xe8\xa8\x80\xe6\x84\x8f\xe5\x91\xb3\xe8\xa1\xa8\xe8\xa8\x98\xe6\x9c\xab\xe5\xb0\xbe\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
        }
    }
    if main_pred_malloc_flag != 0 { free(main_pred as *mut libc::c_void); }
    return buffer;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_ipal_cframe(mut sp: *mut SENTENCE_DATA,
                                          mut t_ptr: *mut TAG_DATA,
                                          mut start: libc::c_int,
                                          mut flag: libc::c_int)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut f_num: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 自立語末尾語を用いて格フレーム辞書を引く */
    if (*t_ptr).jiritu_ptr.is_null() { return f_num; }
    f_num +=
        make_ipal_cframe_subcontract(sp, t_ptr, 0 as *mut MRPH_DATA,
                                     0 as *mut libc::c_char, start, flag);
    /* 代表表記が曖昧な用言の場合 */
    if !check_feature((*(*t_ptr).head_ptr).f,
                      b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        flag == 1 as libc::c_int {
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
        // let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        fp = (*(*t_ptr).head_ptr).f;
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
                f_num +=
                    make_ipal_cframe_subcontract(sp, t_ptr, &mut m,
                                                 0 as *mut libc::c_char,
                                                 start + f_num, flag)
            }
            fp = (*fp).next
        }
    }
    /* ないときで、可能動詞のときは、もとの形を使う */
    if OptCaseFlag & 131072 as libc::c_int == 0 && f_num == 0 as libc::c_int
        &&
        {
            cp =
                check_feature((*(*t_ptr).head_ptr).f,
                              b"\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8b\x95\xe8\xa9\x9e\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            !cp.is_null()
        } {
        f_num +=
            make_ipal_cframe_subcontract(sp, t_ptr, 0 as *mut MRPH_DATA,
                                         cp.offset(strlen(b"\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8b\x95\xe8\xa9\x9e:\x00"
                                             as *const u8 as
                                             *const libc::c_char)
                                             as isize), start, flag)
    }
    Case_frame_num += f_num;
    return f_num;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_pred_feature_to_bp(mut sp:
                                                   *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut pred_merged_rep_size: libc::c_int = 5120 as libc::c_int;
    let mut pred_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_pred_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pred_merged_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* 自立語末尾語を用いて用言代表表記を作成 */
    pred_merged_rep =
        malloc_data(pred_merged_rep_size as size_t,
                    b"assign_pred_feature_to_bp\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        t_ptr = (*sp).tag_data.offset(i as isize);
        if !(check_feature((*t_ptr).f,
                           b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*t_ptr).f,
                           b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null()) {
            pred_string =
                make_pred_string_from_mrph(t_ptr, 0 as *mut MRPH_DATA,
                                           0 as *mut libc::c_char,
                                           OptCaseFlag & 32 as libc::c_int,
                                           1 as libc::c_int,
                                           0 as libc::c_int);
            strcpy(pred_merged_rep,
                   b"\xe7\x94\xa8\xe8\xa8\x80\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                       as *const u8 as *const libc::c_char);
            strcat(pred_merged_rep, pred_string);
            /* 代表表記が曖昧な用言の場合 */
            if !check_feature((*(*t_ptr).head_ptr).f,
                              b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
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
                // let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
                fp = (*(*t_ptr).head_ptr).f;
                while !fp.is_null() {
                    if strncmp((*fp).cp,
                               b"ALT-\x00" as *const u8 as
                                   *const libc::c_char,
                               4 as libc::c_int as libc::c_ulong) == 0 {
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
                        new_pred_string =
                            make_pred_string_from_mrph(t_ptr, &mut m,
                                                       0 as *mut libc::c_char,
                                                       OptCaseFlag &
                                                           32 as libc::c_int,
                                                       1 as libc::c_int,
                                                       0 as libc::c_int);
                        /* 代表と異なるもの */
                        if strcmp(pred_string, new_pred_string) != 0 {
                            if strlen(pred_merged_rep).wrapping_add(strlen(new_pred_string)).wrapping_add(2
                                as
                                libc::c_int
                                as
                                libc::c_ulong)
                                > pred_merged_rep_size as libc::c_ulong {
                                pred_merged_rep_size *= 2 as libc::c_int;
                                pred_merged_rep =
                                    realloc_data(pred_merged_rep as
                                                     *mut libc::c_void,
                                                 pred_merged_rep_size as
                                                     size_t,
                                                 b"assign_pred_feature_to_bp\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char) as
                                        *mut libc::c_char
                            }
                            strcat(pred_merged_rep,
                                   b"?\x00" as *const u8 as
                                       *const libc::c_char);
                            strcat(pred_merged_rep, new_pred_string);
                        }
                        free(new_pred_string as *mut libc::c_void);
                    }
                    fp = (*fp).next
                }
            }
            free(pred_string as *mut libc::c_void);
            assign_cfeature(&mut (*t_ptr).f, pred_merged_rep,
                            0 as libc::c_int);
        }
        i += 1
    }
    free(pred_merged_rep as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_default_cframe(mut t_ptr: *mut TAG_DATA,
                                             mut start: libc::c_int)
                                             -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut f_num: libc::c_int = 0 as libc::c_int;
    let mut rep_name_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut cf_ptr: *mut CASE_FRAME = 0 as *mut CASE_FRAME;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_name: *mut libc::c_char = 0 as *mut libc::c_char;
    cf_ptr = Case_frame_array.offset(start as isize);
    (*cf_ptr).type_0 = 1 as libc::c_int;
    if MAX_cf_frame_length == 0 as libc::c_int {
        MAX_cf_frame_length += 1024 as libc::c_int;
        cf_str_buf =
            malloc_data((::std::mem::size_of::<libc::c_uchar>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"make_default_cframe\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_uchar;
        CF_frame.DATA =
            malloc_data((::std::mem::size_of::<libc::c_uchar>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int
                as
                libc::c_ulong),
                        b"make_default_cframe\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_uchar
    }
    (*cf_ptr).pred_type[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    (*cf_ptr).cf_address = -(1 as libc::c_int) as libc::c_ulonglong;
    cp =
        check_feature((*t_ptr).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        _make_ipal_cframe_pp(cf_ptr,
                             b"\xe3\x82\xac\xef\xbc\x8a\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_uchar,
                             num, 1 as libc::c_int);
        let fresh17 = num;
        num = num + 1;
        _make_ipal_cframe_sm(cf_ptr,
                             b"\xe4\xb8\xbb\xe4\xbd\x93\xe6\xba\x96\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_uchar, fresh17,
                             if Thesaurus == 2 as libc::c_int {
                                 6 as libc::c_int
                             } else { 5 as libc::c_int });
        if strcmp(cp,
                  b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as *const u8 as
                      *const libc::c_char) == 0 {
            strcpy((*cf_ptr).pred_type.as_mut_ptr(),
                   b"\xe5\x88\xa4\x00" as *const u8 as *const libc::c_char);
        } else if strcmp(cp,
                         b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00" as
                             *const u8 as *const libc::c_char) == 0 {
            strcpy((*cf_ptr).pred_type.as_mut_ptr(),
                   b"\xe5\x8b\x95\x00" as *const u8 as *const libc::c_char);
            let fresh18 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\xb2\xef\xbc\x8a\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_uchar, fresh18,
                                 1 as libc::c_int);
            let fresh19 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\x8b\xef\xbc\x8a\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_uchar, fresh19,
                                 1 as libc::c_int);
            let fresh20 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\x98\xef\xbc\x8a\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_uchar, fresh20,
                                 1 as libc::c_int);
            let fresh21 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\xa8\xe3\x83\xaa\xef\xbc\x8a\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_uchar, fresh21,
                                 1 as libc::c_int);
        } else if strcmp(cp,
                         b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00" as
                             *const u8 as *const libc::c_char) == 0 {
            strcpy((*cf_ptr).pred_type.as_mut_ptr(),
                   b"\xe5\xbd\xa2\x00" as *const u8 as *const libc::c_char);
            let fresh22 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\x8b\xef\xbc\x8a\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_uchar, fresh22,
                                 1 as libc::c_int);
            let fresh23 = num;
            num = num + 1;
            _make_ipal_cframe_pp(cf_ptr,
                                 b"\xe3\x83\xa8\xe3\x83\xaa\xef\xbc\x8a\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_uchar, fresh23,
                                 1 as libc::c_int);
        } else { return 0 as libc::c_int; }
    } else { return 0 as libc::c_int; }
    (*cf_ptr).element_num = num;
    (*cf_ptr).etcflag = 0 as libc::c_int;
    /* 代表表記格フレームのときは、IDなどを代表表記にする */
    if OptCaseFlag & 32 as libc::c_int != 0 {
        rep_name = get_mrph_rep_from_f((*t_ptr).head_ptr, 0 as libc::c_int);
        if rep_name.is_null() {
            rep_name = make_mrph_rn((*t_ptr).head_ptr);
            rep_name_malloc_flag = 1 as libc::c_int
        }
        if strlen(rep_name) >= 256 as libc::c_int as libc::c_ulong {
            /* 長すぎるとき */
            *rep_name.offset(256 as libc::c_int as
                isize).offset(-(1 as libc::c_int as isize)) =
                '\u{0}' as i32 as libc::c_char
        }
        sprintf((*cf_ptr).cf_id.as_mut_ptr(),
                b"%s:%s0\x00" as *const u8 as *const libc::c_char, rep_name,
                (*cf_ptr).pred_type.as_mut_ptr());
        if rep_name_malloc_flag != 0 {
            (*cf_ptr).entry = rep_name
        } else { (*cf_ptr).entry = strdup(rep_name) }
    } else {
        sprintf((*cf_ptr).cf_id.as_mut_ptr(),
                b"%s:%s0\x00" as *const u8 as *const libc::c_char,
                (*(*t_ptr).head_ptr).Goi.as_mut_ptr(),
                (*cf_ptr).pred_type.as_mut_ptr());
        (*cf_ptr).entry = strdup((*(*t_ptr).head_ptr).Goi.as_mut_ptr())
    }
    i = 0 as libc::c_int;
    while i < num {
        (*cf_ptr).pp[i as usize][1 as libc::c_int as usize] =
            -(10 as libc::c_int);
        i += 1
    }
    (*cf_ptr).samecase[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        -(10 as libc::c_int);
    (*cf_ptr).samecase[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        -(10 as libc::c_int);
    f_num_inc(start, &mut f_num);
    Case_frame_num += 1;
    (*t_ptr).cf_num += 1;
    assign_cfeature(&mut (*t_ptr).f,
                    b"CF_NOT_FOUND\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 0 as libc::c_int);
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_caseframes(mut sp: *mut SENTENCE_DATA,
                                         mut t_ptr: *mut TAG_DATA,
                                         mut flag: libc::c_int)
/*==================================================================*/
{
    let mut current_cf_num: libc::c_int = 0;
    current_cf_num = make_ipal_cframe(sp, t_ptr, Case_frame_num, flag);
    (*t_ptr).cf_num += current_cf_num;
    /* ないときで用言のときは、defaultの格フレームをつくる */
    if current_cf_num == 0 as libc::c_int && flag == 1 as libc::c_int {
        if OptDisplay == 3 as libc::c_int {
            let mut i: libc::c_int = 0;
            fprintf(stderr,
                    b";; %s: Cannot find case frame for: \x00" as *const u8 as
                        *const libc::c_char,
                    if !(*sp).KNPSID.is_null() {
                        (*sp).KNPSID.offset(5 as libc::c_int as isize) as
                            *const libc::c_char
                    } else { b"?\x00" as *const u8 as *const libc::c_char });
            i = 0 as libc::c_int;
            while i < (*t_ptr).mrph_num {
                fputs((*(*t_ptr).mrph_ptr.offset(i as
                    isize)).Goi2.as_mut_ptr(),
                      stderr);
                i += 1
            }
            fputs(b"\n\x00" as *const u8 as *const libc::c_char, stderr);
        }
        make_default_cframe(t_ptr, Case_frame_num);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cf_pointer(mut dst: *mut CASE_FRAME,
                                         mut src: *mut CASE_FRAME)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*dst).type_0 = (*src).type_0;
    (*dst).type_flag = (*src).type_flag;
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
        (*dst).sp[i as usize] = (*src).sp[i as usize];
        (*dst).pp_str[i as usize] = (*src).pp_str[i as usize];
        (*src).pp_str[i as usize] = 0 as *mut libc::c_char;
        if !(*src).sm[i as usize].is_null() {
            (*dst).sm[i as usize] = (*src).sm[i as usize];
            (*src).sm[i as usize] = 0 as *mut libc::c_char
        }
        if !(*src).ex[i as usize].is_null() {
            (*dst).ex[i as usize] = (*src).ex[i as usize];
            (*src).ex[i as usize] = 0 as *mut libc::c_char
        }
        if !(*src).sm_delete[i as usize].is_null() {
            (*dst).sm_delete[i as usize] = (*src).sm_delete[i as usize];
            (*src).sm_delete[i as usize] = 0 as *mut libc::c_char;
            (*dst).sm_delete_size[i as usize] =
                (*src).sm_delete_size[i as usize];
            (*src).sm_delete_size[i as usize] = 0 as libc::c_int;
            (*dst).sm_delete_num[i as usize] =
                (*src).sm_delete_num[i as usize];
            (*src).sm_delete_num[i as usize] = 0 as libc::c_int
        }
        if !(*src).sm_specify[i as usize].is_null() {
            (*dst).sm_specify[i as usize] = (*src).sm_specify[i as usize];
            (*src).sm_specify[i as usize] = 0 as *mut libc::c_char;
            (*dst).sm_specify_size[i as usize] =
                (*src).sm_specify_size[i as usize];
            (*src).sm_specify_size[i as usize] = 0 as libc::c_int;
            (*dst).sm_specify_num[i as usize] =
                (*src).sm_specify_num[i as usize];
            (*src).sm_specify_num[i as usize] = 0 as libc::c_int
        }
        (*dst).ex_list[i as usize] = (*src).ex_list[i as usize];
        (*src).ex_list[i as usize] = 0 as *mut *mut libc::c_char;
        (*dst).ex_freq[i as usize] = (*src).ex_freq[i as usize];
        (*src).ex_freq[i as usize] = 0 as *mut libc::c_int;
        (*dst).ex_size[i as usize] = (*src).ex_size[i as usize];
        (*src).ex_size[i as usize] = 0 as libc::c_int;
        (*dst).ex_num[i as usize] = (*src).ex_num[i as usize];
        (*src).ex_num[i as usize] = 0 as libc::c_int;
        (*dst).gex_list[i as usize] = (*src).gex_list[i as usize];
        (*src).gex_list[i as usize] = 0 as *mut *mut libc::c_char;
        (*dst).gex_freq[i as usize] = (*src).gex_freq[i as usize];
        (*src).gex_freq[i as usize] = 0 as *mut libc::c_double;
        (*dst).gex_size[i as usize] = (*src).gex_size[i as usize];
        (*src).gex_size[i as usize] = 0 as libc::c_int;
        (*dst).gex_num[i as usize] = (*src).gex_num[i as usize];
        (*src).gex_num[i as usize] = 0 as libc::c_int;
        (*dst).freq[i as usize] = (*src).freq[i as usize];
        (*dst).semantics[i as usize] = (*src).semantics[i as usize];
        (*src).semantics[i as usize] = 0 as *mut libc::c_char;
        (*dst).weight[i as usize] = (*src).weight[i as usize];
        (*dst).samecase[i as usize][0 as libc::c_int as usize] =
            (*src).samecase[i as usize][0 as libc::c_int as usize];
        (*dst).samecase[i as usize][1 as libc::c_int as usize] =
            (*src).samecase[i as usize][1 as libc::c_int as usize];
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
    (*src).feature = 0 as *mut libc::c_char;
    (*dst).entry = (*src).entry;
    (*src).entry = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while !(*src).cf_align[i as usize].cf_id.is_null() {
        (*dst).cf_align[i as usize].cf_id = (*src).cf_align[i as usize].cf_id;
        (*src).cf_align[i as usize].cf_id = 0 as *mut libc::c_char;
        i += 1
    }
    (*dst).pred_b_ptr = (*src).pred_b_ptr;
    (*dst).cf_similarity = (*src).cf_similarity;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_cf_array(mut start_ptr: *mut CASE_FRAME,
                                       mut num: libc::c_int)
                                       -> *mut CASE_FRAME
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut dst_ptr: *mut CASE_FRAME = 0 as *mut CASE_FRAME;
    /* CASE_FRAME構造体を個数分確保し、Case_frame_arrayからポインタをコピー */
    dst_ptr =
        malloc_data((::std::mem::size_of::<CASE_FRAME>() as
            libc::c_ulong).wrapping_mul(num as libc::c_ulong),
                    b"copy_cf_array\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char) as *mut CASE_FRAME;
    init_cf_structure(dst_ptr, num);
    i = 0 as libc::c_int;
    while i < num {
        copy_cf_pointer(dst_ptr.offset(i as isize),
                        start_ptr.offset(i as isize));
        i += 1
    }
    return dst_ptr;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_frame_num_max(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut suru_num: libc::c_int = 0;
    suru_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\xe3\x81\x99\xe3\x82\x8b/\xe3\x81\x99\xe3\x82\x8b\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            suru_num += 1
        }
        i += 1
    }
    /* suru_num > 2 の場合と、suru_num = 1 で sp->Tag_num > 10 の場合は  */
    /* 格フレーム数を制限(実際には"する/する"の格フレームのみ制限される) */
    if suru_num > 1 as libc::c_int ||
        suru_num == 1 as libc::c_int && (*sp).Tag_num > 10 as libc::c_int {
        (*sp).frame_num_max = 128 as libc::c_int
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_caseframes(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut hiragana_count: libc::c_int = 0;
    let mut pred_num: libc::c_int = 0 as libc::c_int;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    Case_frame_num = 0 as libc::c_int;
    start = Case_frame_num;
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        /* 正解コーパスを入力したときに自立語がない場合がある */
        (*t_ptr).cf_num = 0 as libc::c_int; /* ヴォイス */
        if !(*t_ptr).jiritu_ptr.is_null() &&
            check_feature((*t_ptr).f,
                          b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if OptUseNCF != 0 &&
                !check_feature((*t_ptr).f,
                               b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                make_caseframes(sp, t_ptr, 2 as libc::c_int);
            }
            if OptUseCF != 0 &&
                (!check_feature((*t_ptr).f,
                                b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char).is_null() ||
                    !check_feature((*t_ptr).f,
                                   b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null() &&
                        (OptCaseFlag & 16 as libc::c_int == 0 ||
                            OptCaseFlag & 524288 as libc::c_int != 0 ||
                            OptEllipsis & 1 as libc::c_int != 0 ||
                            OptAnaphora != 0 ||
                            (*t_ptr).inum == 1 as libc::c_int &&
                                !check_feature((*(*t_ptr).b_ptr).f,
                                               b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97:-1\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null()))
            {
                set_pred_voice(t_ptr as *mut BNST_DATA);
                make_caseframes(sp, t_ptr, 1 as libc::c_int);
                (*t_ptr).e_cf_num = (*t_ptr).cf_num
            }
        } else { (*t_ptr).cf_ptr = 0 as CF_ptr }
        i += 1;
        t_ptr = t_ptr.offset(1)
    }
    /* 各タグ単位から格フレームへのリンク付け */
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        if (*t_ptr).cf_num != 0 {
            if OptCaseFlag & 32768 as libc::c_int != 0 {
                /* 格フレームを文単位でクリアする場合 (default) */
                (*t_ptr).cf_ptr = Case_frame_array.offset(start as isize)
            } else {
                /* 格フレームをクリアしない場合: 格フレームを新たに確保しコピーする */
                (*t_ptr).cf_ptr =
                    copy_cf_array(Case_frame_array.offset(start as isize),
                                  (*t_ptr).cf_num)
            }
            let fresh24 = pred_num;
            pred_num = pred_num + 1;
            (*t_ptr).pred_num = fresh24;
            /* 表記がひらがなの場合: 
	       格フレームの表記がひらがなの場合が多ければひらがなの格フレームのみを対象に、
	       ひらがな以外が多ければひらがな以外のみを対象にするためのfeatureを付与 */
            if OptCaseFlag & 32 as libc::c_int == 0 &&
                check_str_type((*(*t_ptr).head_ptr).Goi.as_mut_ptr() as
                                   *mut libc::c_uchar, 2 as libc::c_int,
                               0 as libc::c_int) != 0 {
                hiragana_count = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < (*t_ptr).cf_num {
                    if check_str_type((*(*t_ptr).cf_ptr.offset(j as
                        isize)).entry
                                          as *mut libc::c_uchar,
                                      2 as libc::c_int, 0 as libc::c_int) != 0
                    {
                        hiragana_count += 1
                    }
                    j += 1
                }
                if 2 as libc::c_int * hiragana_count > (*t_ptr).cf_num {
                    assign_cfeature(&mut (*t_ptr).f,
                                    b"\xe4\xbb\xa3\xe8\xa1\xa8\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
            }
            start += (*t_ptr).cf_num
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_cf(mut flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    end = if flag != 0 { MAX_Case_frame_num } else { Case_frame_num };
    i = 0 as libc::c_int;
    while i < end {
        j = 0 as libc::c_int;
        while j < 24 as libc::c_int {
            clear_cf_element(Case_frame_array.offset(i as isize), j);
            j += 1
        }
        if !(*Case_frame_array.offset(i as isize)).entry.is_null() {
            free((*Case_frame_array.offset(i as isize)).entry as
                *mut libc::c_void);
            let ref mut fresh25 =
                (*Case_frame_array.offset(i as isize)).entry;
            *fresh25 = 0 as *mut libc::c_char
        }
        if !(*Case_frame_array.offset(i as isize)).feature.is_null() {
            free((*Case_frame_array.offset(i as isize)).feature as
                *mut libc::c_void);
            let ref mut fresh26 =
                (*Case_frame_array.offset(i as isize)).feature;
            *fresh26 = 0 as *mut libc::c_char
        }
        j = 0 as libc::c_int;
        while !(*Case_frame_array.offset(i as
            isize)).cf_align[j as
            usize].cf_id.is_null()
        {
            free((*Case_frame_array.offset(i as
                isize)).cf_align[j as
                usize].cf_id
                as *mut libc::c_void);
            let ref mut fresh27 =
                (*Case_frame_array.offset(i as
                    isize)).cf_align[j as
                    usize].cf_id;
            *fresh27 = 0 as *mut libc::c_char;
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_cf_case(mut cfp: *mut CASE_FRAME,
                                       mut pp: *mut libc::c_char)
                                       -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if MatchPP2((*cfp).pp[i as usize].as_mut_ptr(), pp) != 0 { return i; }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn malloc_db_buf(mut size: libc::c_int)
                                       -> *mut libc::c_char
/*==================================================================*/
{
    if db_buf_size == 0 as libc::c_int {
        db_buf_size = 5120 as libc::c_int;
        db_buf =
            malloc_data(db_buf_size as size_t,
                        b"malloc_db_buf\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char
    }
    while db_buf_size < size {
        db_buf_size <<= 1 as libc::c_int;
        db_buf =
            realloc_data(db_buf as *mut libc::c_void, db_buf_size as size_t,
                         b"malloc_db_buf\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char
    }
    return db_buf;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cfs_similarity(mut cf1: *mut libc::c_char,
                                            mut cf2: *mut libc::c_char)
                                            -> libc::c_float
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verb1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verb2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_float = 0.;
    let mut id1: libc::c_int = 0;
    let mut id2: libc::c_int = 0;
    if CFSimExist == 0 as libc::c_int || cf1.is_null() || cf2.is_null() {
        return 0 as libc::c_int as libc::c_float;
    }
    /* 同じとき */
    if strcmp(cf1, cf2) == 0 { return 1.0f64 as libc::c_float; }
    verb1 = strdup(cf1);
    verb2 = strdup(cf2);
    sscanf(cf1, b"%[^0-9]%d\x00" as *const u8 as *const libc::c_char, verb1,
           &mut id1 as *mut libc::c_int);
    sscanf(cf2, b"%[^0-9]%d\x00" as *const u8 as *const libc::c_char, verb2,
           &mut id2 as *mut libc::c_int);
    key =
        malloc_data((::std::mem::size_of::<libc::c_char>() as
            libc::c_ulong).wrapping_mul(strlen(cf1).wrapping_add(strlen(cf2)).wrapping_add(2
            as
            libc::c_int
            as
            libc::c_ulong)),
                    b"get_cfs_similarity\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    /* 前側のidが小さくなるようにkeyを生成 */
    if id1 > id2 {
        sprintf(key, b"%s%d-%s%d\x00" as *const u8 as *const libc::c_char,
                verb2, id2, verb1, id1);
    } else {
        sprintf(key, b"%s%d-%s%d\x00" as *const u8 as *const libc::c_char,
                verb1, id1, verb2, id2);
    }
    value = db_get(cf_sim_db, key);
    if !value.is_null() {
        ret = atof(value) as libc::c_float;
        free(value as *mut libc::c_void);
    }
    free(key as *mut libc::c_void);
    free(verb1 as *mut libc::c_void);
    free(verb2 as *mut libc::c_void);
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cf_probability_internal(mut key:
                                                     *mut libc::c_char)
                                                     -> libc::c_double
/*==================================================================*/
{
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    value = db_get(cfp_db, key);
    if !value.is_null() {
        ret = atof(value);
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (CF) P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, ret);
        }
        free(value as *mut libc::c_void);
        ret = log(ret)
    } else {
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (CF) P(%s) = 0\n\x00" as *const u8 as
                        *const libc::c_char, key);
        }
        ret = -11.512925f64
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cf_probability_for_pred(mut cfd: *mut CASE_FRAME,
                                                     mut cfp: *mut CASE_FRAME)
                                                     -> libc::c_double
/*==================================================================*/
{
    /* 格フレーム選択確率 P(食べる/たべる:動2|食べる/たべる:動)
       KNP格解析結果 (cfp.prob) */
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut voice: [libc::c_char; 3] = [0; 3];
    let mut num: libc::c_int = 0;
    let mut tp: *mut TAG_DATA = (*cfd).pred_b_ptr;
    if CfpExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    vtype =
        check_feature((*tp).f, b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !vtype.is_null() {
        vtype = vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as *const u8 as *const libc::c_char) as isize)
    } else {
        vtype = check_feature((*tp).f, b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !vtype.is_null() {
            vtype = vtype.offset(strlen(b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90:\x00" as *const u8 as *const libc::c_char) as isize)
        } else {
            return -11.512925f64;
        }
    }
    if OptCaseFlag & 131072 as libc::c_int != 0 {
        /* 用言代表表記版(受身など込み)格フレーム */
        let mut pred_string: *mut libc::c_char = 0 as *mut libc::c_char;
        pred_string = check_feature((*tp).f, b"\xe7\x94\xa8\xe8\xa8\x80\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !pred_string.is_null() {
            pred_string = pred_string.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as *const u8 as *const libc::c_char) as isize);
            key = malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(strlen(pred_string)).wrapping_add(8 as libc::c_int as libc::c_ulong) as libc::c_int);
            sprintf(key, b"%s|%s:%s\x00" as *const u8 as *const libc::c_char, (*cfp).cf_id.as_mut_ptr(), pred_string, vtype);
        } else {
            return -11.512925f64;
        }
    } else {
        /* 用言表記を格フレームIDから抽出しない場合 */
        key =
            malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(strlen((*(*tp).head_ptr).Goi.as_mut_ptr())).wrapping_add(8
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        num =
            sscanf((*cfp).cf_id.as_mut_ptr(),
                   b"%*[^:]:%*[^:]:%[PC]%*d\x00" as *const u8 as
                       *const libc::c_char, voice.as_mut_ptr());
        if num == 1 as libc::c_int {
            sprintf(key,
                    b"%s|%s:%s:%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr(),
                    (*(*tp).head_ptr).Goi.as_mut_ptr(), vtype,
                    voice.as_mut_ptr());
        } else {
            sprintf(key, b"%s|%s:%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr(),
                    (*(*tp).head_ptr).Goi.as_mut_ptr(), vtype);
        }
    }
    return get_cf_probability_internal(key);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cf_probability_for_noun(mut cfd: *mut CASE_FRAME,
                                                     mut cfp: *mut CASE_FRAME)
                                                     -> libc::c_double
/*==================================================================*/
{
    /* 格フレーム選択確率 P(レバー/ればー:名1|レバー/ればー:名)
       KNP格解析結果 (cfp.prob) */
    let mut num: libc::c_int = 0;
    let mut pred_id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if CfpExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    pred_id = strdup((*cfp).cf_id.as_mut_ptr());
    num =
        sscanf((*cfp).cf_id.as_mut_ptr(),
               b"%[^0-9]%*d\x00" as *const u8 as *const libc::c_char,
               pred_id);
    return if num == 1 as libc::c_int {
        key =
            malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(strlen(pred_id)).wrapping_add(2
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                (*cfp).cf_id.as_mut_ptr(), pred_id);
        free(pred_id as *mut libc::c_void);
        get_cf_probability_internal(key)
    } else {
        free(pred_id as *mut libc::c_void);
        -11.512925f64
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_cf_probability(mut cfd: *mut CASE_FRAME,
                                            mut cfp: *mut CASE_FRAME)
                                            -> libc::c_double
/*==================================================================*/
{
    return if (*cfp).type_0 == 1 as libc::c_int {
        /* 用言格フレーム */
        get_cf_probability_for_pred(cfd, cfp)
    } else {
        /* 名詞格フレーム */
        get_cf_probability_for_noun(cfd, cfp)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_probability_from_str(mut case_str:
                                                       *mut libc::c_char,
                                                       mut cfp:
                                                       *mut CASE_FRAME,
                                                       mut aflag: libc::c_int,
                                                       mut para_cpm_ptr:
                                                       *mut CF_PRED_MGR)
                                                       -> libc::c_double
/*==================================================================*/
{
    /* 格確率 P(ガ格○|食べる:動2)
       KNP格解析結果から計算 (cfcases.prob) */
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut para_cond: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pred_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut cf_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut denominator: libc::c_int = 0 as libc::c_int;
    let mut num: libc::c_int = 0;
    if CFCaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if OptParaNoFixFlag & 4 as libc::c_int != 0 && !para_cpm_ptr.is_null() &&
        (*para_cpm_ptr).result_num != 0 as libc::c_int &&
        !(*para_cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr.is_null() &&
        (*(*para_cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).cf_address
            != -(1 as libc::c_int) as libc::c_ulonglong &&
        (*para_cpm_ptr).cmm[0 as libc::c_int as usize].score !=
            -(1001 as libc::c_int) as libc::c_double {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i <
            (*(*para_cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).element_num {
            if MatchPP((*(*para_cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).pp[i as
                usize][0
                as
                libc::c_int
                as
                usize],
                       case_str) != 0 {
                num =
                    (*para_cpm_ptr).cmm[0 as libc::c_int as
                        usize].result_lists_p[0 as
                        libc::c_int
                        as
                        usize].flag[i
                        as
                        usize];
                if num == -(1 as libc::c_int) {
                    /* 並列述語の格フレームに対象格があるが、割り当てなし */
                    para_cond =
                        b"PX\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                } else {
                    para_cond =
                        b"PO\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                }
                break;
            } else { i += 1 }
        }
        if para_cond.is_null() {
            para_cond =
                b"PX\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
            /* 並列述語の格フレームに対象格がない */
        }
    } else {
        para_cond =
            b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        /* 述語並列が存在しない */
    }
    /* 格フレーム */
    key =
        malloc_db_buf(strlen(case_str).wrapping_mul(2 as libc::c_int as
            libc::c_ulong).wrapping_add(strlen((*cfp).cf_id.as_mut_ptr())).wrapping_add(6
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    if OptParaNoFixFlag & 4 as libc::c_int != 0 {
        sprintf(key, b"%s|%s,%s,%s\x00" as *const u8 as *const libc::c_char,
                case_str, (*cfp).cf_id.as_mut_ptr(), case_str, para_cond);
    } else if OptCaseFlag & 262144 as libc::c_int != 0 {
        sprintf(key, b"%s|%s,%s\x00" as *const u8 as *const libc::c_char,
                case_str, (*cfp).cf_id.as_mut_ptr(), case_str);
    } else {
        sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                case_str, (*cfp).cf_id.as_mut_ptr());
    }
    value = db_get(cf_case_db, key);
    if !value.is_null() {
        if sscanf(value, b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                  &mut cf_ret as *mut libc::c_double,
                  &mut denominator as *mut libc::c_int) != 2 as libc::c_int {
            /* 分母のないフォーマット */
            cf_ret = atof(value);
            denominator = -(1 as libc::c_int)
            /* 格フレームに値が存在した印 */
        }
        free(value as *mut libc::c_void);
    } else {
        /* obtain the denominator */
        if OptParaNoFixFlag & 4 as libc::c_int != 0 {
            sprintf(key,
                    b"NIL|%s,%s,%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr(), case_str, para_cond);
        } else if OptCaseFlag & 262144 as libc::c_int != 0 {
            sprintf(key, b"NIL|%s,%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr(), case_str);
        } else {
            sprintf(key, b"NIL|%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr());
        }
        value = db_get(cf_case_db, key);
        if !value.is_null() {
            /* cf_ret should be 0 */
            sscanf(value, b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                   &mut cf_ret as *mut libc::c_double,
                   &mut denominator as *mut libc::c_int);
            cf_ret = 0 as libc::c_int as libc::c_double;
            free(value as *mut libc::c_void);
        }
    }
    /* 用言表記 */
    verb = strdup((*cfp).cf_id.as_mut_ptr());
    sscanf((*cfp).cf_id.as_mut_ptr(),
           b"%[^0-9]%*d\x00" as *const u8 as *const libc::c_char, verb);
    if OptParaNoFixFlag & 4 as libc::c_int != 0 {
        sprintf(key, b"%s|%s,%s,%s\x00" as *const u8 as *const libc::c_char,
                case_str, verb, case_str, para_cond);
    } else if OptCaseFlag & 262144 as libc::c_int != 0 {
        sprintf(key, b"%s|%s,%s\x00" as *const u8 as *const libc::c_char,
                case_str, verb, case_str);
    } else {
        sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                case_str, verb);
    }
    value = db_get(cf_case_db, key);
    if !value.is_null() {
        cp = strchr(value, '/' as i32);
        if !cp.is_null() { cp = 0 as *mut libc::c_char }
        pred_ret = atof(value);
        free(value as *mut libc::c_void);
    }
    /* interpolation between cf and pred */
    if denominator > 0 as libc::c_int {
        let mut lambda: libc::c_double =
            denominator as libc::c_double /
                (denominator + 1 as libc::c_int) as libc::c_double;
        if VerboseLevel as libc::c_uint >=
            VERBOSE1 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (C) lambda * P(%s|%s,%s) + (1 - lambda) * P(%s|%s,%s) = %lf * %lf + %lf * %lf\n\x00"
                        as *const u8 as *const libc::c_char, case_str,
                    (*cfp).cf_id.as_mut_ptr(), para_cond, case_str, verb,
                    para_cond, lambda, cf_ret,
                    1 as libc::c_int as libc::c_double - lambda, pred_ret);
        }
        pred_ret *= 1 as libc::c_int as libc::c_double - lambda;
        pred_ret += lambda * cf_ret
    } else if pred_ret > 0 as libc::c_int as libc::c_double {
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (C) P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, pred_ret);
        }
    } else if denominator < 0 as libc::c_int {
        /* 用言のみ */
        /* 格フレームのみ値が存在 */
        pred_ret = cf_ret;
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (C) P(%s|%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, case_str,
                    (*cfp).cf_id.as_mut_ptr(), pred_ret);
        }
    } else if VerboseLevel as libc::c_uint >=
        VERBOSE3 as libc::c_int as libc::c_uint {
        fprintf(Outfp,
                b";; (C) P(%s) = 0\n\x00" as *const u8 as *const libc::c_char,
                key);
    }
    free(verb as *mut libc::c_void);
    if aflag == 0 as libc::c_int {
        pred_ret = 1 as libc::c_int as libc::c_double - pred_ret
    }
    if pred_ret == 0 as libc::c_int as libc::c_double {
        pred_ret = -11.512925f64
    } else { pred_ret = log(pred_ret) }
    return pred_ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_probability(mut as2: libc::c_int,
                                              mut cfp: *mut CASE_FRAME,
                                              mut aflag: libc::c_int,
                                              mut para_cpm_ptr:
                                              *mut CF_PRED_MGR)
                                              -> libc::c_double
/*==================================================================*/
{
    /* 格確率 P(ガ格○|食べる/たべる:動2)
       KNP格解析結果から計算 (cfcase.prob) */
    return if (*cfp).type_0 == 1 as libc::c_int {
        /* 用言格フレーム */
        get_case_probability_from_str(
            pp_code_to_kstr((*cfp).pp[as2 as usize][0 as libc::c_int as usize]),
            cfp,
            aflag,
            para_cpm_ptr,
        )
    } else {
        /* 名詞格フレーム */
        get_case_probability_from_str(
            (*cfp).pp_str[as2 as usize],
            cfp,
            aflag,
            para_cpm_ptr,
        )
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_num_probability(mut cfp: *mut CASE_FRAME,
                                                  mut num: libc::c_int,
                                                  mut para_cpm_ptr:
                                                  *mut CF_PRED_MGR)
                                                  -> libc::c_double
/*==================================================================*/
{
    /* 格の個数確率 P(2|食べる/たべる:動2)
       KNP格解析結果から計算 (cfcase.prob) */
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut para_cond: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cf_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pred_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut denominator: libc::c_int = 0 as libc::c_int;
    /* 名詞の場合は個数を生成しない */
    if CFCaseExist == 0 as libc::c_int || (*cfp).type_0 == 2 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if OptParaNoFixFlag & 4 as libc::c_int != 0 && !para_cpm_ptr.is_null() &&
        (*para_cpm_ptr).result_num != 0 as libc::c_int &&
        !(*para_cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr.is_null() &&
        (*(*para_cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).cf_address
            != -(1 as libc::c_int) as libc::c_ulonglong &&
        (*para_cpm_ptr).cmm[0 as libc::c_int as usize].score !=
            -(1001 as libc::c_int) as libc::c_double {
        if (*(*para_cpm_ptr).cmm[0 as libc::c_int as
            usize].cf_ptr).element_num >
            0 as libc::c_int {
            para_cond =
                b"PO\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
            /* 項が1つ以上 */
        } else {
            para_cond =
                b"PX\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    } else {
        para_cond =
            b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        /* 述語並列が存在しない */
    }
    /* 格フレーム */
    key =
        malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(9 as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    if OptParaNoFixFlag & 4 as libc::c_int != 0 {
        sprintf(key, b"%d|N:%s,%s\x00" as *const u8 as *const libc::c_char,
                num, (*cfp).cf_id.as_mut_ptr(), para_cond);
    } else {
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                (*cfp).cf_id.as_mut_ptr());
    }
    value = db_get(cf_case_db, key);
    if !value.is_null() {
        if sscanf(value, b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                  &mut cf_ret as *mut libc::c_double,
                  &mut denominator as *mut libc::c_int) != 2 as libc::c_int {
            /* 分母のないフォーマット */
            cf_ret = atof(value);
            denominator = -(1 as libc::c_int)
            /* 格フレームに値が存在した印 */
        }
        free(value as *mut libc::c_void);
    } else {
        /* obtain the denominator */
        if OptParaNoFixFlag & 4 as libc::c_int != 0 {
            sprintf(key,
                    b"NIL|N:%s,%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr(), para_cond);
        } else {
            sprintf(key, b"NIL|N:%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr());
        }
        value = db_get(cf_case_db, key);
        if !value.is_null() {
            /* cf_ret should be 0 */
            sscanf(value, b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                   &mut cf_ret as *mut libc::c_double,
                   &mut denominator as *mut libc::c_int);
            cf_ret = 0 as libc::c_int as libc::c_double;
            free(value as *mut libc::c_void);
        }
    }
    /* 用言表記 */
    verb = strdup((*cfp).cf_id.as_mut_ptr());
    sscanf((*cfp).cf_id.as_mut_ptr(),
           b"%[^0-9]%*d\x00" as *const u8 as *const libc::c_char, verb);
    if OptParaNoFixFlag & 4 as libc::c_int != 0 {
        sprintf(key, b"%d|N:%s,%s\x00" as *const u8 as *const libc::c_char,
                num, verb, para_cond);
    } else {
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                verb);
    }
    value = db_get(cf_case_db, key);
    if !value.is_null() {
        cp = strchr(value, '/' as i32);
        if !cp.is_null() { cp = 0 as *mut libc::c_char }
        pred_ret = atof(value);
        free(value as *mut libc::c_void);
    }
    /* interpolation between cf and pred */
    if denominator > 0 as libc::c_int {
        let mut lambda: libc::c_double =
            denominator as libc::c_double /
                (denominator + 1 as libc::c_int) as libc::c_double;
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (CN) lambda * P(%d|N:%s,%s) + (1 - lambda) * P(%d|N:%s,%s) = %lf * %lf + %lf * %lf\n\x00"
                        as *const u8 as *const libc::c_char, num,
                    (*cfp).cf_id.as_mut_ptr(), para_cond, num, verb,
                    para_cond, lambda, cf_ret,
                    1 as libc::c_int as libc::c_double - lambda, pred_ret);
        }
        pred_ret *= 1 as libc::c_int as libc::c_double - lambda;
        pred_ret += lambda * cf_ret
    } else if pred_ret > 0 as libc::c_int as libc::c_double {
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (CN) P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, pred_ret);
        }
    } else if denominator < 0 as libc::c_int {
        /* 用言のみ */
        /* 格フレームのみ値が存在 */
        pred_ret = cf_ret;
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (CN) P(%d|N:%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, num, (*cfp).cf_id.as_mut_ptr(),
                    pred_ret);
        }
    } else if VerboseLevel as libc::c_uint >=
        VERBOSE3 as libc::c_int as libc::c_uint {
        fprintf(Outfp,
                b";; (CN) P(%s) = 0\n\x00" as *const u8 as
                    *const libc::c_char, key);
    }
    free(verb as *mut libc::c_void);
    if pred_ret == 0 as libc::c_int as libc::c_double {
        pred_ret = -11.512925f64
    } else { pred_ret = log(pred_ret) }
    return pred_ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_ex_category_probability(mut key:
                                                      *mut libc::c_char,
                                                      mut as2: libc::c_int,
                                                      mut cfp:
                                                      *mut CASE_FRAME,
                                                      mut fp: *mut FEATURE)
                                                      -> libc::c_double
/*==================================================================*/
{
    /* カテゴリ-用例確率 
       P(リンゴ/りんご|人工物-食べ物)*P(人工物-食べ物|食べる:動2:ガ格) */
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut prob: libc::c_double = 0.;
    let mut category: [libc::c_char; 128] = [0; 128];
    let mut ex_category: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*cfp).gex_num[as2 as usize] {
        /* 格フレームに含まれているカテゴリ情報を抽出 */
        if strncmp(*(*cfp).gex_list[as2 as usize].offset(i as isize),
                   b"CT\x00" as *const u8 as *const libc::c_char,
                   2 as libc::c_int as libc::c_ulong) == 0 {
            strcpy(category.as_mut_ptr(),
                   (*(*cfp).gex_list[as2 as
                       usize].offset(i as
                       isize)).offset(3 as
                       libc::c_int
                       as
                       isize));
            if !strchr(category.as_mut_ptr(), '>' as i32).is_null() {
                *strchr(category.as_mut_ptr(), '>' as i32) =
                    '\u{0}' as i32 as libc::c_char
            }
            /* 該当するカテゴリが対象の語のfeatureに含まれていた場合 */
            if check_category(fp, category.as_mut_ptr(), 0 as libc::c_int) !=
                0 {
                ex_category =
                    malloc_data((::std::mem::size_of::<libc::c_char>() as
                        libc::c_ulong).wrapping_mul(strlen(key).wrapping_add(strlen(category.as_mut_ptr())).wrapping_add(5
                        as
                        libc::c_int
                        as
                        libc::c_ulong)),
                                b"get_ex_category_probability\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char) as *mut libc::c_char;
                sprintf(ex_category,
                        b"%s|CT:%s\x00" as *const u8 as *const libc::c_char,
                        key, category.as_mut_ptr());
                value = db_get(case_db, ex_category);
                if !value.is_null() {
                    if VerboseLevel as libc::c_uint >=
                        VERBOSE3 as libc::c_int as libc::c_uint {
                        fprintf(Outfp,
                                b";; (EX-CATEGORY)%s %f %f\n\x00" as *const u8
                                    as *const libc::c_char, ex_category,
                                atof(value),
                                *(*cfp).gex_freq[as2 as
                                    usize].offset(i as
                                    isize));
                    }
                    prob =
                        atof(value) *
                            *(*cfp).gex_freq[as2 as usize].offset(i as isize);
                    if ret < prob { ret = prob }
                    free(value as *mut libc::c_void);
                }
                free(ex_category as *mut libc::c_void);
            }
        }
        i += 1
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_ex_ne_probability(mut cp: *mut libc::c_char,
                                               mut as2: libc::c_int,
                                               mut cfp: *mut CASE_FRAME,
                                               mut flag: libc::c_int)
                                               -> libc::c_double
/*==================================================================*/
{
    /* 固有表現-用例確率 
       P(京大|LOCATION)*P(LOCATION|行く:動2:ニ格) */
    /* 入力key=NE:LOCATION:京大 */
    /* flagがたっている場合P(LOCATION|行く:動2:ニ格)を返す */
    /* またflagがたっている場合は、固有表現以外でも使用可 */
    let mut i: libc::c_int = 0; /* key = 京大 */
    let mut value: *mut libc::c_char =
        0 as *mut libc::c_char; /* tag = NE:LOCATION */
    let mut tag: [libc::c_char; 128] = [0; 128];
    let mut key: [libc::c_char; 256] = [0; 256];
    strcpy(tag.as_mut_ptr(), cp);
    if flag == 0 {
        strcpy(key.as_mut_ptr(),
               strchr(tag.as_mut_ptr().offset(3 as libc::c_int as isize),
                      ':' as i32).offset(1 as libc::c_int as isize));
    }
    *strchr(tag.as_mut_ptr().offset(3 as libc::c_int as isize), ':' as i32) =
        '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*cfp).gex_num[as2 as usize] {
        if strcmp(tag.as_mut_ptr(),
                  *(*cfp).gex_list[as2 as usize].offset(i as isize)) == 0 {
            if flag != 0 {
                return *(*cfp).gex_freq[as2 as usize].offset(i as isize);
            }
            strcat(key.as_mut_ptr(),
                   b"|\x00" as *const u8 as *const libc::c_char);
            strcat(key.as_mut_ptr(), tag.as_mut_ptr());
            value = db_get(case_db, key.as_mut_ptr());
            if !value.is_null() {
                if VerboseLevel as libc::c_uint >=
                    VERBOSE3 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (EX-NE)%s %f %f\n\x00" as *const u8 as
                                *const libc::c_char, key.as_mut_ptr(),
                            atof(value),
                            *(*cfp).gex_freq[as2 as
                                usize].offset(i as isize));
                }
                return atof(value) *
                    *(*cfp).gex_freq[as2 as usize].offset(i as isize);
            }
        }
        i += 1
    }
    return 0 as libc::c_int as libc::c_double;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_ex_probability_internal(mut key:
                                                      *mut libc::c_char,
                                                      mut as2: libc::c_int,
                                                      mut cfp:
                                                      *mut CASE_FRAME)
                                                      -> libc::c_double
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    if OptCaseFlag & 1048576 as libc::c_int != 0 {
        /* 代表表記をIDに変換 */
        let mut rep_id: *mut libc::c_char =
            rep2id(key,
                   if !key.is_null() {
                       strlen(key)
                   } else { 0 as libc::c_int as libc::c_ulong } as
                       libc::c_int,
                   &mut *static_buffer.as_mut_ptr().offset(0 as libc::c_int as
                       isize));
        if *rep_id.offset(0 as libc::c_int as isize) != 0 { key = rep_id }
    }
    i = 0 as libc::c_int;
    while i < (*cfp).ex_num[as2 as usize] {
        if strcmp(key, *(*cfp).ex_list[as2 as usize].offset(i as isize)) == 0
        {
            ret =
                *(*cfp).ex_freq[as2 as usize].offset(i as isize) as
                    libc::c_double /
                    (*cfp).freq[as2 as usize] as libc::c_double;
            if VerboseLevel as libc::c_uint >=
                VERBOSE3 as libc::c_int as libc::c_uint {
                fprintf(Outfp,
                        b";; (EX) P(%s) = %lf\n\x00" as *const u8 as
                            *const libc::c_char, key, ret);
            }
            return ret;
        }
        i += 1
    }
    if VerboseLevel as libc::c_uint >= VERBOSE3 as libc::c_int as libc::c_uint
    {
        fprintf(Outfp,
                b";; (EX) P(%s) = 0\n\x00" as *const u8 as
                    *const libc::c_char, key);
    }
    return 0 as libc::c_int as libc::c_double;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_ex_probability(mut key: *mut libc::c_char)
                                             -> libc::c_double
/*==================================================================*/
{
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    if CFExExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    value = db_get(cf_ex_db, key);
    if !value.is_null() {
        ret = exp(-(1 as libc::c_int) as libc::c_double * atof(value));
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, ret);
        }
        free(value as *mut libc::c_void);
    } else if VerboseLevel as libc::c_uint >=
        VERBOSE3 as libc::c_int as libc::c_uint {
        fprintf(Outfp,
                b";; P(%s) = 0\n\x00" as *const u8 as *const libc::c_char,
                key);
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_sm_probability(mut dp: *mut TAG_DATA,
                                             mut as2: libc::c_int,
                                             mut cfp: *mut CASE_FRAME)
                                             -> libc::c_double
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_double = 0.;
    let mut max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sm_code: *mut libc::c_char = (*dp).SM_code.as_mut_ptr();
    let mut code: [libc::c_char; 13] = [0; 13];
    code[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    key =
        malloc_db_buf((12 as libc::c_int as
            libc::c_ulong).wrapping_add(strlen((*cfp).cf_id.as_mut_ptr())).wrapping_add(strlen(pp_code_to_kstr((*cfp).pp[as2
            as
            usize][0
            as
            libc::c_int
            as
            usize]))).wrapping_add(3
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    /* 各意味素ごとに調べて、maxをとる */
    i = 0 as libc::c_int;
    while *sm_code.offset(i as isize) != 0 {
        strncpy(code.as_mut_ptr(), sm_code.offset(i as isize),
                12 as libc::c_int as libc::c_ulong);
        code[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
        sprintf(key, b"%s|%s,%s\x00" as *const u8 as *const libc::c_char,
                code.as_mut_ptr(), (*cfp).cf_id.as_mut_ptr(),
                pp_code_to_kstr((*cfp).pp[as2 as
                    usize][0 as libc::c_int as
                    usize]));
        ret = _get_ex_probability(key);
        if ret > max { max = ret }
        i += 12 as libc::c_int
    }
    return max;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _get_soto_default_probability(mut dp: *mut TAG_DATA, mut as2: libc::c_int, mut cfp: *mut CASE_FRAME) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_double = 0.;
    let mut max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sm_code: *mut libc::c_char = (*dp).SM_code.as_mut_ptr();
    let mut code: [libc::c_char; 13] = [0; 13];
    code[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if CFExExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if strlen((*(*dp).head_ptr).Goi.as_mut_ptr()) >
        12 as libc::c_int as libc::c_ulong {
        key =
            malloc_db_buf(strlen((*(*dp).head_ptr).Goi.as_mut_ptr()).wrapping_add(18 as libc::c_int as libc::c_ulong) as libc::c_int)
    } else { key = malloc_db_buf(12 as libc::c_int + 18 as libc::c_int) }
    /* 表記でsearch */
    sprintf(key,
            b"%s|DEFAULT,\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                as *const u8 as *const libc::c_char,
            (*(*dp).head_ptr).Goi.as_mut_ptr());
    ret = _get_ex_probability(key);
    if ret != 0. { return ret; }
    /* 意味素でsearch: maxをとる */
    i = 0 as libc::c_int;
    while *sm_code.offset(i as isize) != 0 {
        strncpy(code.as_mut_ptr(), sm_code.offset(i as isize),
                12 as libc::c_int as libc::c_ulong);
        code[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
        sprintf(key,
                b"%s|DEFAULT,\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                    as *const u8 as *const libc::c_char, code.as_mut_ptr());
        ret =
            (_get_ex_probability(key) != 0. && ret > max) as libc::c_int as
                libc::c_double;
        if ret != 0. { max = ret }
        i += 12 as libc::c_int
    }
    return max;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dat_match_agent(mut as1: libc::c_int,
                                         mut cfd: *mut CASE_FRAME,
                                         mut tp: *mut TAG_DATA)
                                         -> libc::c_int
/*==================================================================*/
{
    if tp.is_null() {
        tp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize]
    }
    if !check_feature((*tp).f,
                      b"\xe9\x9d\x9e\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8
                          as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        return 0 as libc::c_int;
    } else {
        if !check_feature((*tp).f,
                          b"SM-\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_ex_probability(mut as1: libc::c_int,
                                            mut cfd: *mut CASE_FRAME,
                                            mut dp: *mut TAG_DATA,
                                            mut as2: libc::c_int,
                                            mut cfp: *mut CASE_FRAME,
                                            mut sm_flag: libc::c_int)
                                            -> libc::c_double
/*==================================================================*/
{
    /* 用例確率 P(弁当|食べる:動2,ヲ格)
       格フレームから計算 (cfex.prob) */
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut ne_prob: [libc::c_char; 256] = [0; 256];
    // let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = -13.815511f64;
    let mut prob: libc::c_double = 0.;
    let mut rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    /* dpの指定がなければ、as1とcfdから作る */
    if dp.is_null() {
        dp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize]
    }
    key =
        malloc_db_buf(strlen(b"<\xe8\xa3\x9c\xe6\x96\x87>\x00" as *const u8 as
            *const libc::c_char).wrapping_add(strlen((*cfp).cf_id.as_mut_ptr())).wrapping_add(strlen(pp_code_to_kstr((*cfp).pp[as2
            as
            usize][0
            as
            libc::c_int
            as
            usize]))).wrapping_add(3
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    *key = '\u{0}' as i32 as libc::c_char;
    if !check_feature((*dp).f,
                      b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        sprintf(key,
                b"<\xe8\xa3\x9c\xe6\x96\x87>\x00" as *const u8 as
                    *const libc::c_char);
    } else if sm_flag != 0 && OptCaseFlag & 128 as libc::c_int != 0 &&
        dat_match_agent(as1, cfd, dp) != 0 {
        sprintf(key,
                b"<\xe4\xb8\xbb\xe4\xbd\x93>\x00" as *const u8 as
                    *const libc::c_char);
    } else if strcmp(pp_code_to_kstr((*cfp).pp[as2 as
        usize][0 as libc::c_int as
        usize]),
                     b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                         *const libc::c_char) == 0 &&
        !check_feature((*dp).f,
                       b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        /* 時間格のみ<時間>を考慮 */
        sprintf(key,
                b"<\xe6\x99\x82\xe9\x96\x93>\x00" as *const u8 as
                    *const libc::c_char);
    } else if !check_feature((*dp).f,
                             b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        sprintf(key,
                b"<\xe6\x95\xb0\xe9\x87\x8f>\x00" as *const u8 as
                    *const libc::c_char);
    }
    if *key != 0 {
        /* if (ret = _get_ex_probability(key)) { */
        prob = _get_ex_probability_internal(key, as2, cfp);
        if prob != 0. { if ret < log(prob) { ret = log(prob) } }
    }
    if OptCaseFlag & 32 as libc::c_int != 0 {
        if OptCaseFlag & 1024 as libc::c_int != 0 &&
            {
                cp =
                    get_bnst_head_canonical_rep((*dp).b_ptr,
                                                OptCaseFlag &
                                                    512 as libc::c_int);
                !cp.is_null()
            } {
            mrph_str = strdup(cp);
            rep_malloc_flag = 1 as libc::c_int
        } else {
            mrph_str = get_mrph_rep_from_f((*dp).head_ptr, 0 as libc::c_int);
            if mrph_str.is_null() {
                mrph_str = make_mrph_rn((*dp).head_ptr);
                rep_malloc_flag = 1 as libc::c_int
            }
        }
    } else { mrph_str = (*(*dp).head_ptr).Goi.as_mut_ptr() }
    key =
        malloc_db_buf(strlen(mrph_str).wrapping_add(strlen((*cfp).cf_id.as_mut_ptr())).wrapping_add(strlen(pp_code_to_kstr((*cfp).pp[as2
            as
            usize][0
            as
            libc::c_int
            as
            usize]))).wrapping_add(3
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    sprintf(key, b"%s\x00" as *const u8 as *const libc::c_char, mrph_str);
    /* sprintf(key, "%s|%s,%s", dp->head_ptr->Goi, 
       cfp->cf_id, pp_code_to_kstr(cfp->pp[as2][0])); */
    if rep_malloc_flag != 0 {
        free(mrph_str as *mut libc::c_void);
        rep_malloc_flag = 0 as libc::c_int
    }
    /* if (ret = _get_ex_probability(key)) { */
    prob = _get_ex_probability_internal(key, as2, cfp);
    if prob != 0. {
        if ret < log(prob) { ret = log(prob) }
    } else if OptCaseFlag & 32 as libc::c_int != 0 {
        let mut rep_length: libc::c_int = 0;
        let mut fp: *mut FEATURE = (*(*dp).head_ptr).f;
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
        mrph_str = get_mrph_rep_from_f((*dp).head_ptr, 0 as libc::c_int);
        if mrph_str.is_null() {
            /* 代表表記の場合はALTも調べる */
            /* なければ作る */
            mrph_str = make_mrph_rn((*dp).head_ptr); /* 代表表記 */
            rep_malloc_flag = 1 as libc::c_int
        }
        strcpy(key, mrph_str);
        if rep_malloc_flag != 0 {
            free(mrph_str as *mut libc::c_void);
            rep_malloc_flag = 0 as libc::c_int
        }
        prob = _get_ex_probability_internal(key, as2, cfp);
        if prob != 0. {
            if ret < log(prob) { ret = log(prob) }
        } else {
            while !fp.is_null() {
                if strncmp((*fp).cp,
                           b"ALT-\x00" as *const u8 as *const libc::c_char,
                           4 as libc::c_int as libc::c_ulong) == 0 {
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
                    mrph_str = get_mrph_rep(&mut m);
                    rep_length = get_mrph_rep_length(mrph_str);
                    if rep_length == 0 as libc::c_int {
                        /* なければ作る */
                        mrph_str = make_mrph_rn(&mut m);
                        rep_length = strlen(mrph_str) as libc::c_int;
                        rep_malloc_flag = 1 as libc::c_int
                    }
                    key = malloc_db_buf(rep_length + 1 as libc::c_int);
                    strncpy(key, mrph_str, rep_length as libc::c_ulong);
                    *key.offset(rep_length as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    if rep_malloc_flag != 0 {
                        free(mrph_str as *mut libc::c_void);
                        rep_malloc_flag = 0 as libc::c_int
                    }
                    prob = _get_ex_probability_internal(key, as2, cfp);
                    if prob != 0. { if ret < log(prob) { ret = log(prob) } }
                }
                fp = (*fp).next
            }
        }
    }
    if sm_flag == 0 { return ret; }
    /* 固有表現の場合 */
    if OptGeneralCF & 1 as libc::c_int != 0 &&
        {
            cp =
                check_feature((*dp).f,
                              b"NE\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
            !cp.is_null()
        } {
        prob = get_ex_ne_probability(cp, as2, cfp, 0 as libc::c_int);
        if prob != 0. { if ret < log(prob) { ret = log(prob) } }
    }
    /* 代表表記を用いる場合はカテゴリを参照する */
    if OptGeneralCF & 2 as libc::c_int != 0 &&
        OptCaseFlag & 32 as libc::c_int != 0 {
        mrph_str = get_mrph_rep_from_f((*dp).head_ptr, 0 as libc::c_int);
        if mrph_str.is_null() {
            mrph_str = make_mrph_rn((*dp).head_ptr);
            rep_malloc_flag = 1 as libc::c_int
        }
        key =
            malloc_db_buf(strlen(mrph_str).wrapping_add(strlen((*cfp).cf_id.as_mut_ptr())).wrapping_add(strlen(pp_code_to_kstr((*cfp).pp[as2
                as
                usize][0
                as
                libc::c_int
                as
                usize]))).wrapping_add(3
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key, b"%s\x00" as *const u8 as *const libc::c_char, mrph_str);
        prob =
            _get_ex_category_probability(key, as2, cfp, (*(*dp).head_ptr).f);
        if prob != 0. { if ret < log(prob) { ret = log(prob) } }
        if rep_malloc_flag != 0 {
            free(mrph_str as *mut libc::c_void);
            rep_malloc_flag = 0 as libc::c_int
        }
    }
    /* else if (ret = _get_sm_probability(dp, as2, cfp)) { * 意味素にback-off *
	ret = log(ret);
    } */
    /* else if (MatchPP(cfp->pp[as2][0], "外の関係") && 
	     (ret = _get_soto_default_probability(dp, as2, cfp))) {
	ret = log(ret);
    } */
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_ex_probability_with_para(mut as1: libc::c_int,
                                                      mut cfd:
                                                      *mut CASE_FRAME,
                                                      mut as2: libc::c_int,
                                                      mut cfp:
                                                      *mut CASE_FRAME)
                                                      -> libc::c_double
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 1 as libc::c_int;
    let mut np_modifying_flag: libc::c_int = 0;
    let mut clause_modified_flag: libc::c_int = 0 as libc::c_int;
    let mut tp: *mut TAG_DATA =
        (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize];
    let mut score: libc::c_double = 0.;
    let mut sub_score: libc::c_double = 0 as libc::c_int as libc::c_double;
    if (*(*cfd).pred_b_ptr).num < (*tp).num {
        /* 述語が連体修飾 */
        np_modifying_flag = 1 as libc::c_int
    } else {
        np_modifying_flag = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while !(*tp).child[j as usize].is_null() {
            /* 項が連体修飾されているかどうかをチェック */
            if !check_feature((*(*tp).child[j as usize]).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                clause_modified_flag = 1 as libc::c_int;
                break;
            } else { j += 1 }
        }
    }
    /* 自分自身 */
    score =
        get_ex_probability(as1, cfd, 0 as *mut TAG_DATA, as2, cfp,
                           (0 as libc::c_int == 0) as libc::c_int);
    if OptParaNoFixFlag & 2 as libc::c_int != 0 {
        /* 類似度生成モデルのときは、並列要素を述語から生成しない */
        return score;
    }
    /* 並列の要素 */
    while !(*tp).next.is_null() {
        sub_score +=
            get_ex_probability(-(1 as libc::c_int), cfd, (*tp).next, as2, cfp,
                               (0 as libc::c_int == 0) as libc::c_int);
        count += 1;
        tp = (*tp).next
    }
    /* 自分と並列の要素 *
    if (tp->para_top_p) {
	for (j = 1; tp->child[j]; j++) { * 0は自分と同じ *
	    if (tp->child[j]->para_type == PARA_NORMAL) {
		score += get_ex_probability(-1, cfd, tp->child[j], as2, cfp);
		count++;
	    }
	}
    } */
    if VerboseLevel as libc::c_uint >= VERBOSE3 as libc::c_int as libc::c_uint {
        fprintf(Outfp,
                b";; (EX) is divided by %d => %.5f\n\x00" as *const u8 as *const libc::c_char,
                count,
                (score + sub_score) / count as libc::c_double,
        );
    }
    /* 並列確率的解析時: 並列要素間生成する場合と被連体修飾名詞は正規化
       並列決定的解析時: 常に正規化 */
    return if OptParaFix == (0 as libc::c_int == 0) as libc::c_int {
        if np_modifying_flag != 0 || clause_modified_flag != 0 {
            /* 被連体修飾詞を生成するときは平均をとる */
            (score + sub_score) / (count * 2 as libc::c_int) as libc::c_double
        } else {
            (score + sub_score) / count as libc::c_double
        }
    } else if OptParaNoFixFlag & 1 as libc::c_int != 0 || np_modifying_flag != 0 {
        (score + sub_score) / count as libc::c_double
    } else if OptParaNoFixFlag & 8 as libc::c_int != 0 {
        score + sub_score / 2 as libc::c_int as libc::c_double
    } else {
        score + sub_score
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_np_modifying_probability(mut as1: libc::c_int,
                                                      mut cfd:
                                                      *mut CASE_FRAME)
                                                      -> libc::c_double
/*==================================================================*/
{
    let mut dist: libc::c_int = 0 as libc::c_int;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    /* tp -> hp */
    if (*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize]).num >
        (*(*cfd).pred_b_ptr).num {
        /* 連体修飾 */
        type_0 =
            check_feature((*(*cfd).pred_b_ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !type_0.is_null() {
            type_0 =
                type_0.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as
                    *const u8 as *const libc::c_char) as
                    isize)
        }
        /* 候補チェック */
        dist =
            get_dist_from_work_mgr((*(*cfd).pred_b_ptr).b_ptr,
                                   (*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1
                                       as
                                       usize]).b_ptr);
        if dist <= 0 as libc::c_int {
            return -11.512925f64;
        } else { if dist > 1 as libc::c_int { dist = 2 as libc::c_int } }
    }
    key = malloc_db_buf(10 as libc::c_int);
    sprintf(key, b"%s,%d|R\x00" as *const u8 as *const libc::c_char,
            if !type_0.is_null() {
                type_0 as *const libc::c_char
            } else { b"NIL\x00" as *const u8 as *const libc::c_char }, dist);
    value = db_get(case_db, key);
    if !value.is_null() {
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (RE) %s -> %s: P(%s,%d|R) = %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    if !type_0.is_null() {
                        (*(*(*cfd).pred_b_ptr).head_ptr).Goi.as_mut_ptr() as
                            *const libc::c_char
                    } else { b"NIL\x00" as *const u8 as *const libc::c_char },
                    (*(*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as
                        usize]).head_ptr).Goi.as_mut_ptr(),
                    if !type_0.is_null() {
                        type_0 as *const libc::c_char
                    } else { b"NIL\x00" as *const u8 as *const libc::c_char },
                    dist, value);
        }
        ret = log(atof(value));
        free(value as *mut libc::c_void);
    } else { ret = -11.512925f64 }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_topic_generating_probability(mut have_topic:
                                                          libc::c_int,
                                                          mut g_ptr:
                                                          *mut TAG_DATA)
                                                          -> libc::c_double
/*==================================================================*/
{
    let mut topic_score: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    /* 提題スコア */
    cp =
        check_feature((*g_ptr).f,
                      b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        sscanf(cp, b"%*[^:]:%d\x00" as *const u8 as *const libc::c_char,
               &mut topic_score as *mut libc::c_int);
        if topic_score > 0 as libc::c_int && topic_score < 30 as libc::c_int {
            topic_score = 10 as libc::c_int
        }
    }
    key = malloc_db_buf(7 as libc::c_int);
    sprintf(key, b"%d|W:%d\x00" as *const u8 as *const libc::c_char,
            have_topic, topic_score);
    value = db_get(case_db, key);
    if !value.is_null() {
        if VerboseLevel as libc::c_uint >=
            VERBOSE3 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (W) %s: P(%d|W:%d) = %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*(*g_ptr).head_ptr).Goi.as_mut_ptr(), have_topic,
                    topic_score, value);
        }
        ret = log(atof(value));
        free(value as *mut libc::c_void);
    } else { ret = -11.512925f64 }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_key_probability(mut tag_ptr: *mut TAG_DATA)
                                             -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    key =
        malloc_db_buf(strlen(b"<\xe8\xa3\x9c\xe6\x96\x87>\x00" as *const u8 as
            *const libc::c_char).wrapping_add(3 as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    *key = '\u{0}' as i32 as libc::c_char;
    if !check_feature((*tag_ptr).f,
                      b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        sprintf(key,
                b"<\xe8\xa3\x9c\xe6\x96\x87>\x00" as *const u8 as
                    *const libc::c_char);
    } else if !check_feature((*tag_ptr).f,
                             b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        sprintf(key,
                b"<\xe6\x99\x82\xe9\x96\x93>\x00" as *const u8 as
                    *const libc::c_char);
    } else if !check_feature((*tag_ptr).f,
                             b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        sprintf(key,
                b"<\xe6\x95\xb0\xe9\x87\x8f>\x00" as *const u8 as
                    *const libc::c_char);
    }
    if *key != 0 {
        return get_general_probability(key,
                                       b"KEY\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char);
    }
    if OptCaseFlag & 1024 as libc::c_int != 0 &&
        {
            cp =
                get_bnst_head_canonical_rep((*tag_ptr).b_ptr,
                                            OptCaseFlag &
                                                512 as libc::c_int);
            !cp.is_null()
        } {
        mrph_str = strdup(cp);
        rep_malloc_flag = 1 as libc::c_int
    } else {
        mrph_str = get_mrph_rep_from_f((*tag_ptr).head_ptr, 0 as libc::c_int);
        if mrph_str.is_null() {
            mrph_str = make_mrph_rn((*tag_ptr).head_ptr);
            rep_malloc_flag = 1 as libc::c_int
        }
    }
    key =
        malloc_db_buf(strlen(mrph_str).wrapping_add(3 as libc::c_int as
            libc::c_ulong) as
            libc::c_int);
    sprintf(key, b"%s\x00" as *const u8 as *const libc::c_char, mrph_str);
    if rep_malloc_flag != 0 {
        free(mrph_str as *mut libc::c_void);
        rep_malloc_flag = 0 as libc::c_int
    }
    return get_general_probability(key,
                                   b"KEY\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_class_prob()
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: [libc::c_char; 8] = [0; 8];
    i = 0 as libc::c_int;
    while i < 2000 as libc::c_int {
        sprintf(cp.as_mut_ptr(),
                b"CL:%d\x00" as *const u8 as *const libc::c_char, i);
        ClassProb[i as usize] =
            exp(get_general_probability(cp.as_mut_ptr(),
                                        b"KEY\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char));
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_class_probability(mut key: *mut libc::c_char,
                                               mut as_0: libc::c_int,
                                               mut cfp: *mut CASE_FRAME)
                                               -> libc::c_double
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut key_class_prob: [libc::c_double; 2000] = [0.; 2000];
    let mut prob: libc::c_double = 0.;
    let mut ret: libc::c_double = 0.;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut key2: *mut libc::c_char = 0 as *mut libc::c_char;
    /* keyのclass情報を読み込み */
    cp = db_get(case_db, key);
    if !cp.is_null() {
        i = 0 as libc::c_int;
        while i < 2000 as libc::c_int {
            key_class_prob[i as usize] = 0 as libc::c_int as libc::c_double;
            i += 1
        }
        while !cp.is_null() &&
            sscanf(cp,
                   b"%d:%lf\x00" as *const u8 as *const libc::c_char,
                   &mut i as *mut libc::c_int,
                   &mut prob as *mut libc::c_double) != 0 {
            key_class_prob[i as usize] = prob;
            cp = strstr(cp, b",\x00" as *const u8 as *const libc::c_char);
            if !cp.is_null() { cp = cp.offset(1) }
        }
    } else { return 0 as libc::c_int as libc::c_double; }
    ret = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*cfp).gex_num[as_0 as usize] {
        cp = *(*cfp).gex_list[as_0 as usize].offset(i as isize);
        if strncmp(cp, b"CL:\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 &&
            key_class_prob[atoi(cp.offset(3 as libc::c_int as isize)) as
                usize] > 0 as libc::c_int as libc::c_double
        {
            ret +=
                *(*cfp).gex_freq[as_0 as usize].offset(i as isize) /
                    ClassProb[atoi(cp.offset(3 as libc::c_int as isize)) as
                        usize] *
                    key_class_prob[atoi(cp.offset(3 as libc::c_int as isize))
                        as usize]
        }
        i += 1
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_general_probability(mut key1: *mut libc::c_char,
                                                 mut key2: *mut libc::c_char)
                                                 -> libc::c_double
/*==================================================================*/
{
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    //return FREQ0_ASSINED_SCORE / 2;
    key =
        malloc_db_buf(strlen(key1).wrapping_add(strlen(key2)).wrapping_add(2
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char, key1,
            key2);
    value = db_get(case_db, key);
    return if !value.is_null() {
        ret = atof(value);
        ret = log(ret);
        free(value as *mut libc::c_void);
        if ret < -13.815511f64 { ret = -13.815511f64 }
        ret
    } else { -13.815511f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_interpret_probability(mut scase:
                                                        *mut libc::c_char,
                                                        mut cfcase:
                                                        *mut libc::c_char)
                                                        -> libc::c_double
/*==================================================================*/
{
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    key =
        malloc_db_buf(strlen(scase).wrapping_add(strlen(cfcase)).wrapping_add(20
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    sprintf(key, b"%s|C:%s\x00" as *const u8 as *const libc::c_char, scase,
            cfcase);
    value = db_get(case_db, key);
    return if !value.is_null() {
        ret = atof(value);
        ret = log(ret);
        free(value as *mut libc::c_void);
        ret
    } else { -11.512925f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_punctuation_generating_probability(mut np_modifying_flag:
                                                                libc::c_int,
                                                                mut touten_flag:
                                                                libc::c_int,
                                                                mut dist:
                                                                libc::c_int,
                                                                mut closest_pred_flag:
                                                                libc::c_int,
                                                                mut topic_score:
                                                                libc::c_int,
                                                                mut wa_flag:
                                                                libc::c_int,
                                                                mut genitive_flag:
                                                                libc::c_int)
                                                                -> libc::c_double
/*==================================================================*/
{
    /* 読点の生成 */
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    key = malloc_db_buf(20 as libc::c_int);
    if genitive_flag != 0 {
        /* ノ格など */
        sprintf(key,
                b"%d|P\xe9\x80\xa3\xe4\xbd\x93:%d,%d\x00" as *const u8 as
                    *const libc::c_char, touten_flag, dist,
                closest_pred_flag);
    } else if np_modifying_flag != 0 {
        sprintf(key,
                b"%d|P\xe9\x80\xa3\xe6\xa0\xbc:%d\x00" as *const u8 as
                    *const libc::c_char, touten_flag, dist);
    } else {
        sprintf(key,
                b"%d|P:%d,%d,%d,%d\x00" as *const u8 as *const libc::c_char,
                touten_flag, dist, closest_pred_flag, topic_score, wa_flag);
    }
    value = db_get(case_db, key);
    if VerboseLevel as libc::c_uint >= VERBOSE3 as libc::c_int as libc::c_uint
    {
        fprintf(Outfp,
                b";; (T) P(%s) = %s\n\x00" as *const u8 as
                    *const libc::c_char, key, value);
    }
    return if !value.is_null() {
        ret = atof(value);
        ret = log(ret);
        free(value as *mut libc::c_void);
        ret
    } else { -11.512925f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_function_probability_for_noun(mut as1:
                                                                libc::c_int,
                                                                mut cfd:
                                                                *mut CASE_FRAME,
                                                                mut as2:
                                                                libc::c_int,
                                                                mut cfp:
                                                                *mut CASE_FRAME,
                                                                mut ellipsis_flag:
                                                                libc::c_int)
                                                                -> libc::c_double
/*==================================================================*/
{
    let mut dist: libc::c_int = 0;
    let mut np_modifying_flag: libc::c_int = 0;
    let mut touten_flag: libc::c_int = 0;
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut hp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* tp -> hp */
    if (*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize]).num >
        (*(*cfd).pred_b_ptr).num {
        /* 連体修飾 */
        tp = (*cfd).pred_b_ptr;
        hp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize];
        np_modifying_flag = 1 as libc::c_int
    } else {
        tp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize];
        hp = (*cfd).pred_b_ptr;
        np_modifying_flag = 0 as libc::c_int
    }
    touten_flag =
        if !check_feature((*(*tp).b_ptr).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    dist = get_dist_from_work_mgr((*tp).b_ptr, (*hp).b_ptr);
    if dist < 0 as libc::c_int {
        ret = -11.512925f64
    } else {
        let mut closest_ok_flag: libc::c_int = 0 as libc::c_int;
        if get_dist_from_work_mgr((*tp).b_ptr,
                                  (*tp).b_ptr.offset(1 as libc::c_int as
                                      isize)) >
            0 as libc::c_int {
            closest_ok_flag = 1 as libc::c_int
        }
        ret =
            get_punctuation_generating_probability(np_modifying_flag,
                                                   touten_flag, dist,
                                                   closest_ok_flag,
                                                   0 as libc::c_int,
                                                   0 as libc::c_int,
                                                   1 as libc::c_int)
        /* 
	if (VerboseLevel >= VERBOSE1) {
	    fprintf(Outfp, ";; (NOUN_N) [%s -> %s] : P(%s) = %lf\n", tp->head_ptr->Goi, hp->head_ptr->Goi, key, tmp_ret);
	    } */
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_wa_generating_probability(mut np_modifying_flag:
                                                       libc::c_int,
                                                       mut touten_flag:
                                                       libc::c_int,
                                                       mut dist: libc::c_int,
                                                       mut closest_pred_flag:
                                                       libc::c_int,
                                                       mut topic_score:
                                                       libc::c_int,
                                                       mut wa_flag:
                                                       libc::c_int,
                                                       mut negation_flag:
                                                       libc::c_int,
                                                       mut vtype:
                                                       *mut libc::c_char)
                                                       -> libc::c_double
/*==================================================================*/
{
    /* 「は」の生成 */
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if np_modifying_flag != 0 || vtype.is_null() {
        value =
            b"1.0\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else {
        key = malloc_db_buf(22 as libc::c_int);
        /* sprintf(key, "%d|T:%d,%d,%d,%d,%d,%d", wa_flag, dist, closest_pred_flag, topic_score, touten_flag, negation_flag, strcmp(vtype, "判") == 0 ? 1 : 0); */
        sprintf(key,
                b"%d|T:%d,%d,%d,%d,%d,%s\x00" as *const u8 as
                    *const libc::c_char, wa_flag, dist, closest_pred_flag,
                topic_score, touten_flag, negation_flag, vtype);
        value = db_get(case_db, key)
    }
    return if !value.is_null() {
        ret = atof(value);
        ret = log(ret);
        if np_modifying_flag == 0 && !vtype.is_null() {
            free(value as *mut libc::c_void);
        }
        ret
    } else { -11.512925f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_cf_case_string(mut as1: libc::c_int,
                                             mut cfd: *mut CASE_FRAME,
                                             mut as2: libc::c_int,
                                             mut cfp: *mut CASE_FRAME)
                                             -> *mut libc::c_char
/*==================================================================*/
{
    /* 格フレームの格スロット表記をかえす */
    return if as2 != -(2 as libc::c_int) {
        pp_code_to_kstr((*cfp).pp[as2 as
            usize][0 as libc::c_int as
            usize])
    } else if CF_MatchPP((*cfd).pp[as1 as usize][0 as libc::c_int as usize],
                         cfp) != 0 {
        b"--\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char
    } else {
        /* 格スロットがあるのにNIL_ASSIGNED */
        /* 格フレームがないとき、仮想的に格スロットを作ると考える */
        pp_code_to_kstr((*cfd).pp[as1 as
            usize][0 as libc::c_int as
            usize])
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_function_probability_for_pred(mut as1:
                                                                libc::c_int,
                                                                mut cfd:
                                                                *mut CASE_FRAME,
                                                                mut as2:
                                                                libc::c_int,
                                                                mut cfp:
                                                                *mut CASE_FRAME,
                                                                mut ellipsis_flag:
                                                                libc::c_int)
                                                                -> libc::c_double
/*==================================================================*/
{
    let mut wa_flag: libc::c_int = 0;
    let mut topic_score: libc::c_int = 0 as libc::c_int;
    let mut touten_flag: libc::c_int = 0;
    // let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut negation_flag: libc::c_int = 0;
    let mut np_modifying_flag: libc::c_int = 0;
    let mut closest_pred_flag: libc::c_int = 0 as libc::c_int;
    let mut scase: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cfcase: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score1: libc::c_double = 0.;
    let mut score2: libc::c_double = 0.;
    let mut score3: libc::c_double = 0.;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut tp2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut hp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    /* tp -> hp */
    if (*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize]).num >
        (*(*cfd).pred_b_ptr).num {
        /* 連体修飾 */
        tp = (*cfd).pred_b_ptr;
        hp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize];
        np_modifying_flag = 1 as libc::c_int
    } else {
        tp = (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1 as usize];
        hp = (*cfd).pred_b_ptr;
        np_modifying_flag = 0 as libc::c_int
    }
    vtype =
        check_feature((*hp).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
    if !vtype.is_null() {
        vtype =
            vtype.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80:\x00" as *const u8
                as *const libc::c_char) as isize)
    }
    /* 複合辞 */
    if (*cfd).pp[as1 as usize][0 as libc::c_int as usize] >= 9 as libc::c_int
        &&
        (*cfd).pp[as1 as usize][0 as libc::c_int as usize] <=
            37 as libc::c_int {
        scase =
            pp_code_to_kstr((*cfd).pp[as1 as
                usize][0 as libc::c_int as usize]);
        tp2 =
            &mut *current_sentence_data.tag_data.offset(((*tp).num +
                1 as libc::c_int)
                as isize) as
                *mut TAG_DATA /* 入力側の表層格 */
        /* 読点や「は」などをチェックするタグ単位 */
    } else {
        if np_modifying_flag != 0 {
            scase =
                b"\xe9\x80\xa3\xe6\xa0\xbc\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        } else {
            scase =
                check_feature((*tp).f,
                              b"\xe4\xbf\x82\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
            if scase.is_null() {
                return -11.512925f64;
            } else {
                scase =
                    scase.offset(strlen(b"\xe4\xbf\x82:\x00" as *const u8 as
                        *const libc::c_char) as isize)
                /* 入力側の表層格 */
            }
        }
        tp2 = tp
    }
    /* 隣に用言があるかどうか */
    if np_modifying_flag == 0 as libc::c_int {
        if get_dist_from_work_mgr((*tp2).b_ptr,
                                  (*current_sentence_data.tag_data.offset(((*tp2).num
                                      +
                                      1
                                          as
                                          libc::c_int)
                                      as
                                      isize)).b_ptr)
            > 0 as libc::c_int {
            closest_pred_flag = 1 as libc::c_int
        }
    }
    /* 「は」, 読点, 否定のチェック */
    wa_flag =
        if !check_feature((*tp2).f,
                          b"\xe3\x83\x8f\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    touten_flag =
        if !check_feature((*tp2).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    negation_flag =
        if !check_feature((*hp).f,
                          b"\xe5\x90\xa6\xe5\xae\x9a\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else if !check_feature((*hp).f,
                                 b"\xe6\xba\x96\xe5\x90\xa6\xe5\xae\x9a\xe8\xa1\xa8\xe7\x8f\xbe\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    /* 提題スコア */
    cp =
        check_feature((*hp).f,
                      b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char);
    if !cp.is_null() {
        sscanf(cp, b"%*[^:]:%d\x00" as *const u8 as *const libc::c_char,
               &mut topic_score as *mut libc::c_int);
        if topic_score > 0 as libc::c_int && topic_score < 30 as libc::c_int {
            topic_score = 10 as libc::c_int
        }
    }
    /* 候補チェック */
    dist = get_dist_from_work_mgr((*tp2).b_ptr, (*hp).b_ptr);
    if dist <= 0 as libc::c_int && ellipsis_flag == 0 {
        return -11.512925f64;
    } else { if dist > 1 as libc::c_int { dist = 2 as libc::c_int } }
    /* 格の解釈 */
    cfcase = make_cf_case_string(as1, cfd, as2, cfp);
    score1 = get_case_interpret_probability(scase, cfcase);
    if ellipsis_flag != 0 { return score1; }
    /* 読点の生成 */
    score2 =
        get_punctuation_generating_probability(np_modifying_flag, touten_flag,
                                               dist, closest_pred_flag,
                                               topic_score, wa_flag,
                                               0 as libc::c_int);
    /* 「は」の生成 */
    score3 =
        get_wa_generating_probability(np_modifying_flag, touten_flag, dist,
                                      closest_pred_flag, topic_score, wa_flag,
                                      negation_flag, vtype);
    if VerboseLevel as libc::c_uint >= VERBOSE3 as libc::c_int as libc::c_uint
    {
        fprintf(Outfp,
                b";; (CC) %s -> %s: P(%s,%d,%d|%s,%d,%d,%d,%d) = %lf (C:%lf * P:%lf * T:%lf)\n\x00"
                    as *const u8 as *const libc::c_char,
                (*(*tp).head_ptr).Goi.as_mut_ptr(),
                (*(*hp).head_ptr).Goi.as_mut_ptr(), scase, touten_flag,
                wa_flag,
                if as2 == -(2 as libc::c_int) {
                    b"--\x00" as *const u8 as *const libc::c_char
                } else {
                    pp_code_to_kstr((*cfp).pp[as2 as
                        usize][0 as libc::c_int as
                        usize]) as
                        *const libc::c_char
                }, dist, closest_pred_flag, topic_score, negation_flag,
                score1 + score2 + score3, exp(score1), exp(score2),
                exp(score3));
    }
    return score1 + score2 + score3;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_case_function_probability(mut as1: libc::c_int,
                                                       mut cfd:
                                                       *mut CASE_FRAME,
                                                       mut as2: libc::c_int,
                                                       mut cfp:
                                                       *mut CASE_FRAME,
                                                       mut ellipsis_flag:
                                                       libc::c_int)
                                                       -> libc::c_double
/*==================================================================*/
{
    return if (*cfp).type_0 == 1 as libc::c_int {
        /* 用言格フレーム */
        get_case_function_probability_for_pred(as1, cfd, as2, cfp,
                                               0 as libc::c_int)
    } else {
        /* 名詞格フレーム */
        get_case_function_probability_for_noun(as1, cfd, as2, cfp,
                                               0 as libc::c_int)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_vp_modifying_probability(mut gp: *mut TAG_DATA,
                                                       mut g_cf:
                                                       *mut CASE_FRAME,
                                                       mut dp: *mut TAG_DATA,
                                                       mut d_cf:
                                                       *mut CASE_FRAME)
                                                       -> libc::c_double
/*==================================================================*/
{
    let mut touten_flag: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut closest_pred_flag: libc::c_int = 0 as libc::c_int;
    let mut g_pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d_pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g_id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d_id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut g_level: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret3: libc::c_double = 0 as libc::c_int as libc::c_double;
    /* EOS -> 文末 */
    if gp.is_null() {
        g_pred = strdup(b"EOS\x00" as *const u8 as *const libc::c_char)
    } else if !g_cf.is_null() {
        g_pred = strdup((*g_cf).cf_id.as_mut_ptr());
        sscanf((*g_cf).cf_id.as_mut_ptr(),
               b"%[^0-9]:%*d\x00" as *const u8 as *const libc::c_char,
               g_pred);
    } else { g_pred = 0 as *mut libc::c_char }
    /* 用言 -> 用言 */
    if RenyouExist != 0 && !g_pred.is_null() && !d_cf.is_null() &&
        (!gp.is_null() || OptCaseFlag & 256 as libc::c_int != 0) {
        /* 文末からの生成は-generate-eos(言語モデル用)時のみ */
        d_pred = strdup((*d_cf).cf_id.as_mut_ptr());
        sscanf((*d_cf).cf_id.as_mut_ptr(),
               b"%[^0-9]:%*d\x00" as *const u8 as *const libc::c_char,
               d_pred);
        key =
            malloc_db_buf(strlen(g_pred).wrapping_add(strlen(d_pred)).wrapping_add(3
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char, d_pred,
                g_pred);
        value = db_get(renyou_db, key);
        if !value.is_null() {
            ret3 = atof(value);
            free(value as *mut libc::c_void);
        } else {
            sprintf(key, b"NIL|%s\x00" as *const u8 as *const libc::c_char,
                    g_pred);
            value = db_get(renyou_db, key);
            if !value.is_null() {
                ret3 = atof(value);
                free(value as *mut libc::c_void);
            }
        }
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (R) %s: P(%s|%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char,
                    if !gp.is_null() {
                        (*(*gp).head_ptr).Goi.as_mut_ptr() as
                            *const libc::c_char
                    } else { b"EOS\x00" as *const u8 as *const libc::c_char },
                    d_pred, g_pred, ret3);
        }
        free(d_pred as *mut libc::c_void);
    } else { ret3 = 1 as libc::c_int as libc::c_double }
    if !g_pred.is_null() { free(g_pred as *mut libc::c_void); }
    if gp.is_null() {
        return if ret3 != 0. { log(ret3) } else { -16.118096f64 };
    }
    touten_flag =
        if !check_feature((*(*dp).b_ptr).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    dist = get_dist_from_work_mgr((*dp).b_ptr, (*gp).b_ptr);
    if dist < 0 as libc::c_int { return -16.118096f64; }
    if get_dist_from_work_mgr((*dp).b_ptr,
                              (*dp).b_ptr.offset(1 as libc::c_int as isize)) >
        0 as libc::c_int {
        closest_pred_flag = 1 as libc::c_int
    }
    /* 読点の生成 */
    key =
        malloc_db_buf(strlen(b"\xe9\x80\xa3\xe7\x94\xa8\x00" as *const u8 as
            *const libc::c_char).wrapping_add(8 as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    sprintf(key,
            b"%d|P\xe9\x80\xa3\xe7\x94\xa8:%d,%d\x00" as *const u8 as
                *const libc::c_char, touten_flag, dist, closest_pred_flag);
    value = db_get(case_db, key);
    if !value.is_null() {
        ret1 = atof(value);
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (R_P) %s: P(%d|P\xe9\x80\xa3\xe7\x94\xa8:%d,%d) = %lf\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*(*gp).head_ptr).Goi.as_mut_ptr(), touten_flag, dist,
                    closest_pred_flag, ret1);
        }
        free(value as *mut libc::c_void);
    }
    if RenyouExist == 0 as libc::c_int {
        return if ret1 != 0. { log(ret1) } else { -16.118096f64 };
    }
    /* ID -> ID (未使用) */
    if 0 as libc::c_int != 0 &&
        {
            g_id =
                check_feature((*gp).f,
                              b"ID\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
            !g_id.is_null()
        } &&
        {
            d_id =
                check_feature((*dp).f,
                              b"ID\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
            !d_id.is_null()
        } {
        g_id = g_id.offset(3 as libc::c_int as isize);
        d_id = d_id.offset(3 as libc::c_int as isize);
        key =
            malloc_db_buf(strlen(g_id).wrapping_add(strlen(d_id)).wrapping_add(3
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char, d_id,
                g_id);
        value = db_get(renyou_db, key);
        if !value.is_null() {
            ret2 = atof(value);
            free(value as *mut libc::c_void);
        } else {
            sprintf(key, b"NIL|%s\x00" as *const u8 as *const libc::c_char,
                    g_id);
            value = db_get(renyou_db, key);
            if !value.is_null() {
                ret2 = atof(value);
                free(value as *mut libc::c_void);
            }
        }
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (R) %s: P(%s|%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*(*gp).head_ptr).Goi.as_mut_ptr(), d_id, g_id, ret2);
        }
    } else { ret2 = 1 as libc::c_int as libc::c_double }
    /* 格フレームID => 格フレームID *
    if (0 && g_cf && d_cf) {
	key = malloc_db_buf(strlen(g_cf->cf_id) + strlen(d_cf->cf_id) + 2);
	sprintf(key, "%s|%s", d_cf->cf_id, g_cf->cf_id);
	value = db_get(renyou_db, key);
	if (value) {
	    ret3 = atof(value);
	    if (VerboseLevel >= VERBOSE3) {
		fprintf(Outfp, ";; (R) %s: P(%s|%s) = %lf\n", gp->head_ptr->Goi, d_cf->cf_id, g_cf->cf_id, ret3);
	    }
	    free(value);
	}
    }
    else {
	ret3 = 1;
    }
    */
    /* レベル => レベル *
    if ((g_level = check_feature(gp->f, "レベル")) == NULL) {
	return UNKNOWN_RENYOU_SCORE;
    }
    g_level += 7;

    key = malloc_db_buf(strlen(g_level) + 6);
    sprintf(key, "%d|%d,%s", check_feature(dp->f, "読点") ? 1 : 0, dist, g_level);
    value = db_get(renyou_func_db, key);
    if (value) {
	ret3 = atof(value);
	if (VerboseLevel >= VERBOSE3) {
	    fprintf(Outfp, ";; (R) %s -> %s: P(,=%d|%d,%s) = %lf\n", dp->head_ptr->Goi, gp->head_ptr->Goi, 
		    check_feature(dp->f, "読点") ? 1 : 0, dist, g_level, ret3);
	}
	free(value);
    }
    ret3 = 1;
    */
    if ret1 != 0. { ret1 = log(ret1) } else { ret1 = -16.118096f64 }
    if ret2 != 0. { ret2 = log(ret2) } else { ret2 = -16.118096f64 }
    if ret3 != 0. { ret3 = log(ret3) } else { ret3 = -16.118096f64 }
    return ret1 + ret2 + ret3;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_vp_modifying_num_probability(mut t_ptr:
                                                           *mut TAG_DATA,
                                                           mut cfp:
                                                           *mut CASE_FRAME,
                                                           mut num:
                                                           libc::c_int)
                                                           -> libc::c_double
/*==================================================================*/
{
    let mut pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret2: libc::c_double = 0 as libc::c_int as libc::c_double;
    /* 個数は未使用 */
    if 1 as libc::c_int != 0 || RenyouExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    /* ID */
    if 0 as libc::c_int != 0 &&
        {
            id =
                check_feature((*t_ptr).f,
                              b"ID\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
            !id.is_null()
        } {
        id = id.offset(3 as libc::c_int as isize);
        key =
            malloc_db_buf(strlen(id).wrapping_add(6 as libc::c_int as
                libc::c_ulong) as
                libc::c_int);
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                id);
        value = db_get(renyou_db, key);
        if !value.is_null() {
            ret1 = atof(value);
            if VerboseLevel as libc::c_uint >=
                VERBOSE2 as libc::c_int as libc::c_uint {
                fprintf(Outfp,
                        b";; (RN) %s: P(%d|%s) = %lf\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*(*t_ptr).head_ptr).Goi.as_mut_ptr(), num, id, ret1);
            }
            free(value as *mut libc::c_void);
        }
    } else { ret1 = 1 as libc::c_int as libc::c_double }
    /* 用言 (未使用) */
    if !cfp.is_null() {
        pred = strdup((*cfp).cf_id.as_mut_ptr());
        sscanf((*cfp).cf_id.as_mut_ptr(),
               b"%[^0-9]:%*d\x00" as *const u8 as *const libc::c_char, pred);
        key =
            malloc_db_buf(strlen(pred).wrapping_add(6 as libc::c_int as
                libc::c_ulong) as
                libc::c_int);
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                pred);
        value = db_get(renyou_db, key);
        if !value.is_null() {
            ret2 = atof(value);
            free(value as *mut libc::c_void);
        }
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (RN) %s: P(%d|%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*(*t_ptr).head_ptr).Goi.as_mut_ptr(), num, pred, ret2);
        }
        free(pred as *mut libc::c_void);
    } else { ret2 = 1 as libc::c_int as libc::c_double }
    /* 格フレーム *
    if (0 && cfp) {
	key = malloc_db_buf(strlen(cfp->cf_id) + 6);
	sprintf(key, "%d|N:%s", num, cfp->cf_id);
	value = db_get(renyou_db, key);
	if (value) {
	    ret2 = atof(value);
	    if (VerboseLevel >= VERBOSE3) {
		fprintf(Outfp, ";; (R) %s: P(%d|%s) = %lf\n", t_ptr->head_ptr->Goi, num, cfp->cf_id, ret2);
	    }
	    free(value);
	}
    }
    else {
	ret2 = 1;
    }
    */
    return if ret1 != 0. && ret2 != 0. {
        log(ret1) + log(ret2)
    } else { -16.118096f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_adv_modifying_probability(mut gp: *mut TAG_DATA,
                                                        mut cfp:
                                                        *mut CASE_FRAME,
                                                        mut dp: *mut TAG_DATA)
                                                        -> libc::c_double
/*==================================================================*/
{
    let mut pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut touten_flag: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut pred_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut cf_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut punc_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut denominator: libc::c_int = 0 as libc::c_int;
    let mut rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    /* 副詞 -> 格フレーム or 用言 */
    return if !cfp.is_null() {
        if AdverbExist == (0 as libc::c_int == 0) as libc::c_int {
            /* 副詞表記の取得 */
            if OptCaseFlag & 32 as libc::c_int != 0 {
                if OptCaseFlag & 1024 as libc::c_int != 0 &&
                    {
                        cp =
                            get_bnst_head_canonical_rep((*dp).b_ptr,
                                                        OptCaseFlag &
                                                            512 as
                                                                libc::c_int);
                        !cp.is_null()
                    } {
                    mrph_str = strdup(cp);
                    rep_malloc_flag = 1 as libc::c_int
                } else {
                    mrph_str =
                        get_mrph_rep_from_f((*dp).head_ptr, 0 as libc::c_int);
                    if mrph_str.is_null() {
                        mrph_str = make_mrph_rn((*dp).head_ptr);
                        rep_malloc_flag = 1 as libc::c_int
                    }
                }
            } else { mrph_str = (*(*dp).head_ptr).Goi.as_mut_ptr() }
            /* 格フレーム */
            key =
                malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(strlen(mrph_str)).wrapping_add(3
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    as libc::c_int);
            sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                    mrph_str, (*cfp).cf_id.as_mut_ptr());
            value = db_get(adverb_db, key);
            if !value.is_null() {
                if sscanf(value,
                          b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                          &mut cf_ret as *mut libc::c_double,
                          &mut denominator as *mut libc::c_int) !=
                    2 as libc::c_int {
                    /* 分母のないフォーマット */
                    cf_ret = atof(value)
                    /* denominator = 0; -> 用言のみで */
                }
                free(value as *mut libc::c_void);
            } else {
                /* obtain the denominator */
                sprintf(key,
                        b"NIL|%s\x00" as *const u8 as *const libc::c_char,
                        (*cfp).cf_id.as_mut_ptr());
                value = db_get(adverb_db, key);
                if !value.is_null() {
                    /* cf_ret should be 0 */
                    if sscanf(value,
                              b"%lf/%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut cf_ret as *mut libc::c_double,
                              &mut denominator as *mut libc::c_int) !=
                        2 as libc::c_int {
                        /* 分母のないフォーマット */
                        cf_ret = atof(value)
                        /* 0 */
                        /* denominator = 0; -> 用言のみで */
                    }
                    free(value as *mut libc::c_void);
                }
            }
            /* 用言表記 */
            pred = strdup((*cfp).cf_id.as_mut_ptr());
            sscanf((*cfp).cf_id.as_mut_ptr(),
                   b"%[^0-9]:%*d\x00" as *const u8 as *const libc::c_char,
                   pred);
            sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                    mrph_str, pred);
            value = db_get(adverb_db, key);
            if !value.is_null() {
                cp = strchr(value, '/' as i32);
                if !cp.is_null() { cp = 0 as *mut libc::c_char }
                pred_ret = atof(value);
                free(value as *mut libc::c_void);
            }
            /* interpolation between cf and pred */
            if denominator > 0 as libc::c_int {
                let mut lambda: libc::c_double =
                    denominator as libc::c_double /
                        (denominator + 1 as libc::c_int) as libc::c_double;
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (A) lambda * P(%s|%s) + (1 - lambda) * P(%s|%s) = %lf * %lf + %lf * %lf\n\x00"
                                as *const u8 as *const libc::c_char, mrph_str,
                            (*cfp).cf_id.as_mut_ptr(), mrph_str, pred, lambda,
                            cf_ret,
                            1 as libc::c_int as libc::c_double - lambda,
                            pred_ret);
                }
                pred_ret *= 1 as libc::c_int as libc::c_double - lambda;
                pred_ret += lambda * cf_ret
            } else if VerboseLevel as libc::c_uint >=
                VERBOSE2 as libc::c_int as libc::c_uint {
                fprintf(Outfp,
                        b";; (A) P(%s) = %lf\n\x00" as *const u8 as
                            *const libc::c_char, key, pred_ret);
            }
            if rep_malloc_flag != 0 { free(mrph_str as *mut libc::c_void); }
            free(pred as *mut libc::c_void);
            if pred_ret == 0 as libc::c_int as libc::c_double {
                pred_ret = -16.118096f64
            } else { pred_ret = log(pred_ret) }
        }
        /* 読点の生成 */
        touten_flag =
            if !check_feature((*(*dp).b_ptr).f,
                              b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        dist = get_dist_from_work_mgr((*dp).b_ptr, (*gp).b_ptr);
        if dist < 0 as libc::c_int {
            punc_ret = -16.118096f64
        } else {
            let mut closest_pred_flag: libc::c_int = 0 as libc::c_int;
            if get_dist_from_work_mgr((*dp).b_ptr,
                                      (*dp).b_ptr.offset(1 as libc::c_int as
                                          isize)) >
                0 as libc::c_int {
                closest_pred_flag = 1 as libc::c_int
            }
            key =
                malloc_db_buf(strlen(b"\xe5\x89\xaf\xe8\xa9\x9e\x00" as
                    *const u8 as
                    *const libc::c_char).wrapping_add(8
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    as libc::c_int);
            sprintf(key,
                    b"%d|P\xe5\x89\xaf\xe8\xa9\x9e:%d,%d\x00" as *const u8 as
                        *const libc::c_char, touten_flag, dist,
                    closest_pred_flag);
            value = db_get(case_db, key);
            if !value.is_null() {
                punc_ret = atof(value);
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (A_P) [%s -> %s] : P(%s) = %lf\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key,
                            punc_ret);
                }
                free(value as *mut libc::c_void);
                punc_ret = log(punc_ret)
            } else {
                punc_ret = -16.118096f64;
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (A_P) [%s -> %s] : P(%s) = 0\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key);
                }
            }
        }
        pred_ret + punc_ret
    } else { 0 as libc::c_int as libc::c_double };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_adv_modifying_num_probability(mut t_ptr:
                                                            *mut TAG_DATA,
                                                            mut cfp:
                                                            *mut CASE_FRAME,
                                                            mut num:
                                                            libc::c_int)
                                                            -> libc::c_double
/*==================================================================*/
{
    let mut pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cf_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pred_ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut denominator: libc::c_int = 0 as libc::c_int;
    if AdverbExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    return if !cfp.is_null() {
        /* 格フレーム */
        key =
            malloc_db_buf(strlen((*cfp).cf_id.as_mut_ptr()).wrapping_add(6 as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                (*cfp).cf_id.as_mut_ptr());
        value = db_get(adverb_db, key);
        if !value.is_null() {
            if sscanf(value,
                      b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                      &mut cf_ret as *mut libc::c_double,
                      &mut denominator as *mut libc::c_int) !=
                2 as libc::c_int {
                /* 分母のないフォーマット */
                cf_ret = atof(value)
                /* denominator = 0; -> 用言のみで */
            }
            free(value as *mut libc::c_void);
        } else {
            /* obtain the denominator */
            sprintf(key, b"NIL|N:%s\x00" as *const u8 as *const libc::c_char,
                    (*cfp).cf_id.as_mut_ptr());
            value = db_get(adverb_db, key);
            if !value.is_null() {
                /* cf_ret should be 0 */
                if sscanf(value,
                          b"%lf/%d\x00" as *const u8 as *const libc::c_char,
                          &mut cf_ret as *mut libc::c_double,
                          &mut denominator as *mut libc::c_int) !=
                    2 as libc::c_int {
                    /* 分母のないフォーマット */
                    cf_ret = atof(value)
                    /* 0 */
                    /* denominator = 0; -> 用言のみで */
                }
                free(value as *mut libc::c_void);
            }
        }
        /* 用言表記 */
        pred = strdup((*cfp).cf_id.as_mut_ptr());
        sscanf((*cfp).cf_id.as_mut_ptr(),
               b"%[^0-9]:%*d\x00" as *const u8 as *const libc::c_char, pred);
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                pred);
        value = db_get(adverb_db, key);
        if !value.is_null() {
            cp = strchr(value, '/' as i32);
            if !cp.is_null() { cp = 0 as *mut libc::c_char }
            pred_ret = atof(value);
            free(value as *mut libc::c_void);
        }
        /* interpolation between cf and pred */
        if denominator > 0 as libc::c_int {
            let mut lambda: libc::c_double =
                denominator as libc::c_double /
                    (denominator + 1 as libc::c_int) as libc::c_double;
            if VerboseLevel as libc::c_uint >=
                VERBOSE2 as libc::c_int as libc::c_uint {
                fprintf(Outfp,
                        b";; (AN) lambda * P(%d|N:%s) + (1 - lambda) * P(%d|N:%s) = %lf * %lf + %lf * %lf\n\x00"
                            as *const u8 as *const libc::c_char, num,
                        (*cfp).cf_id.as_mut_ptr(), num, pred, lambda, cf_ret,
                        1 as libc::c_int as libc::c_double - lambda,
                        pred_ret);
            }
            pred_ret *= 1 as libc::c_int as libc::c_double - lambda;
            pred_ret += lambda * cf_ret
        } else if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (AN) P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, pred_ret);
        }
        free(pred as *mut libc::c_void);
        if pred_ret == 0 as libc::c_int as libc::c_double {
            -16.118096f64
        } else { log(pred_ret) }
    } else { 0 as libc::c_int as libc::c_double };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn bin_sim_score(mut score: libc::c_double)
                                       -> libc::c_int
/*==================================================================*/
{
    return if score < 1.0f64 {
        0 as libc::c_int
    } else if score < 2.0f64 {
        1 as libc::c_int
    } else if score < 3.0f64 {
        2 as libc::c_int
    } else if score < 4.0f64 {
        3 as libc::c_int
    } else { 4 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_para_exist_probability(mut para_key:
                                                    *mut libc::c_char,
                                                    mut score: libc::c_double,
                                                    mut flag: libc::c_int,
                                                    mut dp: *mut TAG_DATA,
                                                    mut gp: *mut TAG_DATA)
                                                    -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut binned_score: libc::c_int = bin_sim_score(score);
    let mut touten_flag: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    if CaseExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    key =
        malloc_db_buf(strlen(para_key).wrapping_add(12 as libc::c_int as
            libc::c_ulong) as
            libc::c_int);
    if flag != 0 {
        if OptParaNoFixFlag & 2 as libc::c_int != 0 {
            sprintf(key,
                    b"1,%d|PARA:%s\x00" as *const u8 as *const libc::c_char,
                    binned_score, para_key);
        } else {
            sprintf(key, b"1|PARA:%s\x00" as *const u8 as *const libc::c_char,
                    para_key);
        }
    } else {
        sprintf(key, b"0|PARA:%s\x00" as *const u8 as *const libc::c_char,
                para_key);
    }
    value = db_get(case_db, key);
    if !value.is_null() {
        ret1 = atof(value);
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (PARA) : P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, ret1);
        }
        free(value as *mut libc::c_void);
    } else if VerboseLevel as libc::c_uint >=
        VERBOSE2 as libc::c_int as libc::c_uint {
        fprintf(Outfp,
                b";; (PARA) : P(%s) = 0\n\x00" as *const u8 as
                    *const libc::c_char, key);
    }
    /* 
    sprintf(key, "%s|PTYPE:%d", para_key, binned_score);
    value = db_get(case_db, key);
    if (value) {
	ret2 = atof(value);
	if (VerboseLevel >= VERBOSE2) {
	    fprintf(Outfp, ";; (PTYPE) : P(%s) = %lf\n", key, ret2);
	}
	free(value);
    }
    else {
	if (VerboseLevel >= VERBOSE2) {
	    fprintf(Outfp, ";; (PTYPE) : P(%s) = 0\n", key);
	}
    }
    */
    /* 名詞並列のときは読点を生成 */
    if flag != 0 &&
        check_feature((*dp).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as
                          *mut libc::c_char).is_null() {
        if !check_feature((*(*dp).b_ptr).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            check_feature((*(*dp).b_ptr).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\xe4\xb8\xa6\xe3\x82\xad\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            touten_flag = 1 as libc::c_int
        } else { touten_flag = 0 as libc::c_int }
        dist = get_dist_from_work_mgr((*dp).b_ptr, (*gp).b_ptr);
        if dist < 0 as libc::c_int {
            ret2 = -16.118096f64
        } else {
            key =
                malloc_db_buf(strlen(b"\xe4\xb8\xa6\xe5\x88\x97\x00" as
                    *const u8 as
                    *const libc::c_char).wrapping_add(6
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    as libc::c_int);
            sprintf(key,
                    b"%d|P\xe4\xb8\xa6\xe5\x88\x97:%d\x00" as *const u8 as
                        *const libc::c_char, touten_flag, dist);
            value = db_get(case_db, key);
            if !value.is_null() {
                ret2 = atof(value);
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (PARA_P) [%s -> %s] : P(%s) = %lf\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key, ret2);
                }
                free(value as *mut libc::c_void);
                ret2 = log(ret2)
            } else {
                ret2 = -16.118096f64;
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (PARA_P) [%s -> %s] : P(%s) = 0\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key);
                }
            }
        }
    }
    if ret1 != 0. { ret1 = log(ret1) } else { ret1 = -16.118096f64 }
    /* if (ret2) {
	ret2 = log(ret2);
    }
    else {
	ret2 = UNKNOWN_RENYOU_SCORE;
	} */
    return ret1 + ret2;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_para_ex_probability(mut para_key:
                                                 *mut libc::c_char,
                                                 mut score: libc::c_double,
                                                 mut dp: *mut TAG_DATA,
                                                 mut gp: *mut TAG_DATA)
                                                 -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gp_mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp_mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gp_rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut dp_rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_double = 0.;
    if ParaExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if OptCaseFlag & 32 as libc::c_int != 0 {
        /* 代表表記 */
        if OptCaseFlag & 1024 as libc::c_int != 0 &&
            {
                cp =
                    get_bnst_head_canonical_rep((*gp).b_ptr,
                                                OptCaseFlag &
                                                    512 as libc::c_int);
                !cp.is_null()
            } {
            gp_mrph_str = strdup(cp);
            gp_rep_malloc_flag = 1 as libc::c_int
        } else {
            gp_mrph_str =
                get_mrph_rep_from_f((*gp).head_ptr, 0 as libc::c_int);
            if gp_mrph_str.is_null() {
                gp_mrph_str = make_mrph_rn((*gp).head_ptr);
                gp_rep_malloc_flag = 1 as libc::c_int
            }
        }
    } else { gp_mrph_str = (*(*gp).head_ptr).Goi.as_mut_ptr() }
    if OptCaseFlag & 32 as libc::c_int != 0 {
        /* 代表表記 */
        if OptCaseFlag & 1024 as libc::c_int != 0 &&
            {
                cp =
                    get_bnst_head_canonical_rep((*dp).b_ptr,
                                                OptCaseFlag &
                                                    512 as libc::c_int);
                !cp.is_null()
            } {
            dp_mrph_str = strdup(cp);
            dp_rep_malloc_flag = 1 as libc::c_int
        } else {
            dp_mrph_str =
                get_mrph_rep_from_f((*dp).head_ptr, 0 as libc::c_int);
            if dp_mrph_str.is_null() {
                dp_mrph_str = make_mrph_rn((*dp).head_ptr);
                dp_rep_malloc_flag = 1 as libc::c_int
            }
        }
    } else { dp_mrph_str = (*(*dp).head_ptr).Goi.as_mut_ptr() }
    /* 同じものが並列されている -> 確率1 */
    /* if (!strcmp(dp_mrph_str, gp_mrph_str) || */
    if strcmp((*(*dp).head_ptr).Goi.as_mut_ptr(),
              (*(*gp).head_ptr).Goi.as_mut_ptr()) == 0 {
        return 0 as libc::c_int as libc::c_double;
    }
    key =
        malloc_db_buf(strlen(dp_mrph_str).wrapping_add(strlen(para_key)).wrapping_add(strlen(gp_mrph_str)).wrapping_add(5
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    if OptParaNoFixFlag & 2 as libc::c_int != 0 {
        sprintf(key, b"%s|%d,%s,%s\x00" as *const u8 as *const libc::c_char,
                dp_mrph_str, bin_sim_score(score), para_key, gp_mrph_str);
    } else {
        sprintf(key, b"%s|%s,%s\x00" as *const u8 as *const libc::c_char,
                dp_mrph_str, para_key, gp_mrph_str);
    }
    if gp_rep_malloc_flag != 0 { free(gp_mrph_str as *mut libc::c_void); }
    if dp_rep_malloc_flag != 0 { free(dp_mrph_str as *mut libc::c_void); }
    value = db_get(para_db, key);
    if !value.is_null() {
        ret = atof(value);
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (PARA_EX) : P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, ret);
        }
        free(value as *mut libc::c_void);
    } else {
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (PARA_EX) : P(%s) = 0\n\x00" as *const u8 as
                        *const libc::c_char, key);
        }
        return -13.815511f64;
    }
    return if ret != 0. { log(ret) } else { -13.815511f64 };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_noun_co_ex_probability(mut dp: *mut TAG_DATA,
                                                    mut gp: *mut TAG_DATA)
                                                    -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gp_mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp_mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut touten_flag: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut elem_num: libc::c_int = 0 as libc::c_int;
    let mut g_elem_num: libc::c_int = 0 as libc::c_int;
    let mut gp_rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut dp_rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut ret1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sub_ret1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tmp_ret: libc::c_double = 0.;
    let mut tmp_dp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut tmp_gp: *mut TAG_DATA = gp;
    if NounCoExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    while !tmp_gp.is_null() {
        if OptCaseFlag & 32 as libc::c_int != 0 {
            /* 代表表記 */
            if OptCaseFlag & 1024 as libc::c_int != 0 &&
                {
                    cp =
                        get_bnst_head_canonical_rep((*tmp_gp).b_ptr,
                                                    OptCaseFlag &
                                                        512 as
                                                            libc::c_int);
                    !cp.is_null()
                } {
                gp_mrph_str = strdup(cp);
                gp_rep_malloc_flag = 1 as libc::c_int
            } else {
                gp_mrph_str =
                    get_mrph_rep_from_f((*tmp_gp).head_ptr, 0 as libc::c_int);
                if gp_mrph_str.is_null() {
                    gp_mrph_str = make_mrph_rn((*tmp_gp).head_ptr);
                    gp_rep_malloc_flag = 1 as libc::c_int
                }
            }
        } else { gp_mrph_str = (*(*tmp_gp).head_ptr).Goi.as_mut_ptr() }
        tmp_dp = dp;
        while !tmp_dp.is_null() {
            if (*tmp_dp).num > (*tmp_gp).num { continue; }
            if OptCaseFlag & 32 as libc::c_int != 0 {
                /* 代表表記 */
                if OptCaseFlag & 1024 as libc::c_int != 0 &&
                    {
                        cp =
                            get_bnst_head_canonical_rep((*tmp_dp).b_ptr,
                                                        OptCaseFlag &
                                                            512 as
                                                                libc::c_int);
                        !cp.is_null()
                    } {
                    dp_mrph_str = strdup(cp);
                    dp_rep_malloc_flag = 1 as libc::c_int
                } else {
                    dp_mrph_str =
                        get_mrph_rep_from_f((*tmp_dp).head_ptr,
                                            0 as libc::c_int);
                    if dp_mrph_str.is_null() {
                        dp_mrph_str = make_mrph_rn((*tmp_dp).head_ptr);
                        dp_rep_malloc_flag = 1 as libc::c_int
                    }
                }
            } else { dp_mrph_str = (*(*tmp_dp).head_ptr).Goi.as_mut_ptr() }
            key =
                malloc_db_buf(strlen(dp_mrph_str).wrapping_add(strlen(gp_mrph_str)).wrapping_add(2
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    as libc::c_int);
            sprintf(key, b"%s|%s\x00" as *const u8 as *const libc::c_char,
                    dp_mrph_str, gp_mrph_str);
            if dp_rep_malloc_flag != 0 {
                free(dp_mrph_str as *mut libc::c_void);
            }
            value = db_get(noun_co_db, key);
            if !value.is_null() {
                tmp_ret = atof(value);
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (NOUN_EX) : P(%s) = %lf\n\x00" as *const u8
                                as *const libc::c_char, key, tmp_ret);
                }
                free(value as *mut libc::c_void);
                if tmp_dp == dp {
                    /* 一番後の係り元(loopの最初) */
                    ret1 += log(tmp_ret)
                } else { sub_ret1 += log(tmp_ret) }
            } else {
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (NOUN_EX) : P(%s) = 0\n\x00" as *const u8 as
                                *const libc::c_char, key);
                }
                if tmp_dp == dp {
                    /* 一番後の係り元(loopの最初) */
                    ret1 += -13.815511f64
                } else { sub_ret1 += -13.815511f64 }
            }
            elem_num += 1;
            if OptParaNoFixFlag & 2 as libc::c_int != 0 { break; }
            tmp_dp = (*tmp_dp).next
        }
        if gp_rep_malloc_flag != 0 { free(gp_mrph_str as *mut libc::c_void); }
        g_elem_num += 1;
        if OptParaNoFixFlag & 2 as libc::c_int != 0 { break; }
        tmp_gp = (*tmp_gp).next
    }
    if OptParaNoFixFlag & 8 as libc::c_int != 0 {
        sub_ret1 /= 2 as libc::c_int as libc::c_double;
        ret1 += sub_ret1;
        ret1 /= g_elem_num as libc::c_double
        /* 非連体修飾名詞の方は常に正規化 */
    } else {
        ret1 += sub_ret1;
        ret1 /=
            if OptParaNoFixFlag & 1 as libc::c_int != 0 {
                elem_num as libc::c_double
            } else { g_elem_num as libc::c_double }
        /* 非連体修飾名詞の方は常に正規化 */
    }
    if VerboseLevel as libc::c_uint >= VERBOSE2 as libc::c_int as libc::c_uint
    {
        fprintf(Outfp,
                b";; (NOUN_EX) is divided by %d => %.5f\n\x00" as *const u8 as
                    *const libc::c_char,
                if OptParaNoFixFlag & 1 as libc::c_int != 0 {
                    elem_num
                } else { g_elem_num }, ret1);
    }
    /* 読点の生成
       係側が並列の場合は複数個 */
    elem_num = 0 as libc::c_int;
    tmp_dp = dp;
    while !tmp_dp.is_null() {
        touten_flag =
            if !check_feature((*(*tmp_dp).b_ptr).f,
                              b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        dist = get_dist_from_work_mgr((*tmp_dp).b_ptr, (*gp).b_ptr);
        if dist < 0 as libc::c_int {
            ret2 += -13.815511f64
        } else {
            let mut closest_ok_flag: libc::c_int = 0 as libc::c_int;
            if get_dist_from_work_mgr((*tmp_dp).b_ptr,
                                      (*tmp_dp).b_ptr.offset(1 as libc::c_int
                                          as isize)) >
                0 as libc::c_int {
                closest_ok_flag = 1 as libc::c_int
            }
            key =
                malloc_db_buf(strlen(b"\xe9\x80\xa3\xe4\xbd\x93\x00" as
                    *const u8 as
                    *const libc::c_char).wrapping_add(8
                    as
                    libc::c_int
                    as
                    libc::c_ulong)
                    as libc::c_int);
            sprintf(key,
                    b"%d|P\xe9\x80\xa3\xe4\xbd\x93:%d,%d\x00" as *const u8 as
                        *const libc::c_char, touten_flag, dist,
                    closest_ok_flag);
            value = db_get(case_db, key);
            if !value.is_null() {
                tmp_ret = atof(value);
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (NOUN_N) [%s -> %s] : P(%s) = %lf\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*tmp_dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key, tmp_ret);
                }
                free(value as *mut libc::c_void);
                ret2 += log(tmp_ret)
            } else {
                ret2 += -13.815511f64;
                if VerboseLevel as libc::c_uint >=
                    VERBOSE2 as libc::c_int as libc::c_uint {
                    fprintf(Outfp,
                            b";; (NOUN_N) [%s -> %s] : P(%s) = 0\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*(*tmp_dp).head_ptr).Goi.as_mut_ptr(),
                            (*(*gp).head_ptr).Goi.as_mut_ptr(), key);
                }
            }
        }
        elem_num += 1;
        if OptParaNoFixFlag & 2 as libc::c_int != 0 { break; }
        tmp_dp = (*tmp_dp).next
    }
    if OptParaNoFixFlag & 1 as libc::c_int != 0 {
        ret2 /= elem_num as libc::c_double;
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (NOUN_N) is divided by %d => %.5f\n\x00" as *const u8
                        as *const libc::c_char, elem_num, ret2);
        }
    }
    return ret1 + ret2;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_noun_co_num_probability(mut gp: *mut TAG_DATA,
                                                     mut num: libc::c_int,
                                                     mut para_cky_ptr:
                                                     *mut CKY)
                                                     -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mrph_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut para_cond: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_double = 0.;
    if NounCoExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    if OptParaNoFixFlag & 4 as libc::c_int != 0 && !para_cky_ptr.is_null() {
        while !para_cky_ptr.is_null() {
            if !(*para_cky_ptr).left.is_null() &&
                (*para_cky_ptr).para_flag == 0 as libc::c_int &&
                !(*(*para_cky_ptr).left).b_ptr.is_null() &&
                (!check_feature((*(*(*para_cky_ptr).left).b_ptr).f,
                                b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char).is_null() ||
                    !check_feature((*(*(*para_cky_ptr).left).b_ptr).f,
                                   b"\xe4\xbf\x82:\xe9\x80\xa3\xe4\xbd\x93\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null()) {
                para_cond =
                    b"PO\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                break;
            } else { para_cky_ptr = (*para_cky_ptr).right }
        }
        if para_cond.is_null() {
            para_cond =
                b"PX\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    } else {
        para_cond =
            b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if OptCaseFlag & 32 as libc::c_int != 0 {
        /* 代表表記 */
        if OptCaseFlag & 1024 as libc::c_int != 0 &&
            {
                cp =
                    get_bnst_head_canonical_rep((*gp).b_ptr,
                                                OptCaseFlag &
                                                    512 as libc::c_int);
                !cp.is_null()
            } {
            mrph_str = strdup(cp);
            rep_malloc_flag = 1 as libc::c_int
        } else {
            mrph_str = get_mrph_rep_from_f((*gp).head_ptr, 0 as libc::c_int);
            if mrph_str.is_null() {
                mrph_str = make_mrph_rn((*gp).head_ptr);
                rep_malloc_flag = 1 as libc::c_int
            }
        }
    } else { mrph_str = (*(*gp).head_ptr).Goi.as_mut_ptr() }
    key =
        malloc_db_buf(strlen(mrph_str).wrapping_add(9 as libc::c_int as
            libc::c_ulong) as
            libc::c_int);
    if OptParaNoFixFlag & 4 as libc::c_int != 0 {
        sprintf(key, b"%d|N:%s,%s\x00" as *const u8 as *const libc::c_char,
                num, mrph_str, para_cond);
    } else {
        sprintf(key, b"%d|N:%s\x00" as *const u8 as *const libc::c_char, num,
                mrph_str);
    }
    if rep_malloc_flag != 0 { free(mrph_str as *mut libc::c_void); }
    value = db_get(noun_co_db, key);
    if !value.is_null() {
        ret = atof(value);
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (NOUN_NUM) : P(%s) = %lf\n\x00" as *const u8 as
                        *const libc::c_char, key, ret);
        }
        free(value as *mut libc::c_void);
        ret = log(ret)
    } else {
        if VerboseLevel as libc::c_uint >=
            VERBOSE2 as libc::c_int as libc::c_uint {
            fprintf(Outfp,
                    b";; (NOUN_NUM) : P(%s) = 0\n\x00" as *const u8 as
                        *const libc::c_char, key);
        }
        ret = -13.815511f64
    }
    return ret;
}
/* get pa count for Chinese from gigaword */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_chi_pa(mut ptr1: *mut BNST_DATA,
                                    mut ptr2: *mut BNST_DATA,
                                    mut dist: libc::c_int) -> libc::c_double
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_double = 0.;
    if CHIPAExist == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    key =
        malloc_db_buf(strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr()).wrapping_add(strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr())).wrapping_add(strlen((*(*ptr1).head_ptr).Pos.as_mut_ptr())).wrapping_add(strlen((*(*ptr2).head_ptr).Pos.as_mut_ptr())).wrapping_add(11
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    /* 用言表記でやった方がよいみたい */
    if (*ptr1).num < (*ptr2).num {
        sprintf(key,
                b"%s_%s_%s_%s_R_%d\x00" as *const u8 as *const libc::c_char,
                (*(*ptr1).head_ptr).Pos.as_mut_ptr(),
                (*(*ptr1).head_ptr).Goi.as_mut_ptr(),
                (*(*ptr2).head_ptr).Pos.as_mut_ptr(),
                (*(*ptr2).head_ptr).Goi.as_mut_ptr(), dist);
    } else {
        sprintf(key,
                b"%s_%s_%s_%s_L_%d\x00" as *const u8 as *const libc::c_char,
                (*(*ptr2).head_ptr).Pos.as_mut_ptr(),
                (*(*ptr2).head_ptr).Goi.as_mut_ptr(),
                (*(*ptr1).head_ptr).Pos.as_mut_ptr(),
                (*(*ptr1).head_ptr).Goi.as_mut_ptr(), dist);
    }
    value = db_get(chi_pa_db, key);
    if !value.is_null() {
        ret = atof(value);
        free(value as *mut libc::c_void);
    } else { ret = 0.0f64 }
    return ret;
}

static mut cdb_buf: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut cdb_blen: libc::c_uint = 0;

unsafe extern "C" fn allocbuf(mut len: libc::c_uint) {
    if cdb_blen < len {
        cdb_buf =
            if !cdb_buf.is_null() {
                realloc(cdb_buf as *mut libc::c_void, len as libc::c_ulong)
            } else { malloc(len as libc::c_ulong) } as *mut libc::c_uchar;
        if cdb_buf.is_null() {
            fprintf(stderr,
                    b"unable to allocate %u bytes\n\x00" as *const u8 as
                        *const libc::c_char, len);
        }
        cdb_blen = len
    };
}

unsafe extern "C" fn fget(mut f: libc::c_int, mut b: *mut libc::c_uchar,
                          mut len: libc::c_uint, mut posp: *mut libc::c_uint,
                          mut limit: libc::c_uint) {
    if !posp.is_null() && limit.wrapping_sub(*posp) < len {
        fprintf(stderr,
                b"invalid database format\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if read(f, b as *mut libc::c_void, len as size_t) != len as libc::ssize_t {
        fprintf(stderr,
                b"unable to read: short file\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(2 as libc::c_int);
    }
    if !posp.is_null() { *posp = (*posp).wrapping_add(len) };
}

unsafe extern "C" fn fcpy(mut fi: libc::c_int, mut len: libc::c_uint,
                          mut posp: *mut libc::c_uint,
                          mut limit: libc::c_uint) -> *mut libc::c_uchar {
    let mut retbuf: *mut libc::c_uchar =
        malloc_data(len.wrapping_add(1 as libc::c_int as libc::c_uint) as
                        size_t,
                    b"fcpy\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut libc::c_char as
            *mut libc::c_uchar;
    fget(fi, retbuf, len, posp, limit);
    *retbuf.offset(len as isize) = '\u{0}' as i32 as libc::c_uchar;
    return retbuf;
}

#[no_mangle]
pub unsafe extern "C" fn list_db_and_register_caseframe(mut db: DBM_FILE,
                                                        mut flag:
                                                        libc::c_int) {
    let mut eod: libc::c_uint = 0;
    let mut klen: libc::c_uint = 0;
    let mut vlen: libc::c_uint = 0;
    let mut address: libc::c_ulonglong = 0;
    let mut pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pre_pos: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut match_0: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut break_flag: libc::c_int = 0;
    allocbuf(2048 as libc::c_int as libc::c_uint);
    fget((*db).fd, cdb_buf, 2048 as libc::c_int as libc::c_uint, &mut pos,
         2048 as libc::c_int as libc::c_uint);
    eod = cdb_unpack(cdb_buf as *const libc::c_uchar);
    while pos < eod {
        fget((*db).fd, cdb_buf, 8 as libc::c_int as libc::c_uint, &mut pos,
             eod);
        klen = cdb_unpack(cdb_buf as *const libc::c_uchar);
        vlen =
            cdb_unpack(cdb_buf.offset(4 as libc::c_int as isize) as
                *const libc::c_uchar);
        key = fcpy((*db).fd, klen, &mut pos, eod);
        val = fcpy((*db).fd, vlen, &mut pos, eod);
        fprintf(stderr, b"%s %s\n\t\x00" as *const u8 as *const libc::c_char,
                key as *mut libc::c_char, val as *mut libc::c_char);
        break_flag = 0 as libc::c_int;
        pre_pos = val;
        cp = pre_pos;
        loop {
            if *cp as libc::c_int == '/' as i32 ||
                *cp as libc::c_int == '\u{0}' as i32 {
                if *cp as libc::c_int == '\u{0}' as i32 {
                    break_flag = 1 as libc::c_int
                } else { *cp = '\u{0}' as i32 as libc::c_uchar }
                match_0 =
                    sscanf(pre_pos as *mut libc::c_char,
                           b"%llu:%d\x00" as *const u8 as *const libc::c_char,
                           &mut address as *mut libc::c_ulonglong,
                           &mut size as *mut libc::c_int);
                if match_0 != 2 as libc::c_int {
                    fprintf(stderr,
                            b";; CaseFrame Dictionary Index error (it seems version 1.).\n\x00"
                                as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                if lookup_caseframe(address).is_null() {
                    /* if not registered yet */
                    get_ipal_frame(address, size, flag);
                    fprintf(stderr,
                            b" %llu\x00" as *const u8 as *const libc::c_char,
                            address);
                }
                pre_pos = cp.offset(1 as libc::c_int as isize);
                if break_flag != 0 { break; }
            }
            cp = cp.offset(1)
        }
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        free(key as *mut libc::c_void);
        free(val as *mut libc::c_void);
    }
    if pos != eod {
        fprintf(stderr,
                b"invalid cdb file format\n\x00" as *const u8 as
                    *const libc::c_char);
    };
}
