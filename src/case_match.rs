#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::case_analysis::{CF_MatchPP, MatchPP, MatchPP2, pp_code_to_kstr, pp_kstr_to_code};
use crate::ctools::{check_feature, get_mrph_rep_from_f, Outfp, stderr};
use crate::lib_bgh::bgh_match_check;
use crate::lib_sm::{_smp2smg, sm2code, sm_match_check, smp2smg};
use crate::structs::{CDB_FILE, CF_MATCH_MGR, LIST};
use crate::{_FEATURE, FEATURE, MRPH_DATA, TAG_DATA, tnode_b};
use crate::case_ipal::{check_examples, get_case_function_probability, get_case_function_probability_for_pred, get_case_num_probability, get_case_probability, get_case_probability_from_str, get_cf_probability, get_ex_probability_with_para, get_np_modifying_probability, get_topic_generating_probability};
use crate::read_data::{get_bnst_head_canonical_rep, get_mrph_rep, get_mrph_rep_length, make_mrph_rn};
use crate::thesaurus::{calc_similarity, calc_sm_words_similarity, code_depth};
use crate::tools::{OptCaseFlag, OptDisplay, OptEllipsis, SMExist, SMP2SMGExist, Thesaurus};
use crate::types::{CASE_FRAME, CF_PRED_MGR, DBM_FILE};


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
/*====================================================================

			格構造解析: マッチング

                                               S.Kurohashi 93. 5.31

    $Id$
====================================================================*/
#[no_mangle]
pub static mut Current_max_score: libc::c_double = 0.;
/* 得点 */
#[no_mangle]
pub static mut Current_pure_score: [libc::c_double; 10] = [0.; 10];
/* 正規化する前の得点 */
#[no_mangle]
pub static mut Current_sufficiency: libc::c_double = 0.;
/* 埋まりぐあい */
#[no_mangle]
pub static mut Current_max_m_e: libc::c_int = 0;
/* 要素数 */
#[no_mangle]
pub static mut Current_max_m_p: libc::c_int = 0;
/* 要素の位置 */
#[no_mangle]
pub static mut Current_max_c_e: libc::c_int = 0;
/* 交差数 */
#[no_mangle]
pub static mut Current_max_num: libc::c_int = 0;
#[no_mangle]
pub static mut Current_max_list1: [LIST; 10] =
    [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],}; 10];
#[no_mangle]
pub static mut Current_max_list2: [LIST; 10] =
    [LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],}; 10];
#[no_mangle]
pub static mut SM_match_score: [libc::c_int; 13] =
    [0 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
     10 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
     10 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
     10 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
     10 as libc::c_int];
/*{0, 5, 8, 10, 12};*/
/* ＳＭ対応スコア */
#[no_mangle]
pub static mut SM_match_unknown: libc::c_int = 10 as libc::c_int;
/* データ未知     */
/* int 	EX_match_score[] = {0, 0, 5, 7, 8, 9, 10, 11}; */
/* int 	EX_match_score[] = {0, 0, 0, 1, 3, 5, 10, 11}; */
#[no_mangle]
pub static mut EX_match_score: [libc::c_int; 8] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
     3 as libc::c_int, 5 as libc::c_int, 8 as libc::c_int, 11 as libc::c_int];
#[no_mangle]
pub static mut EX_match_score2: [libc::c_int; 8] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
     2 as libc::c_int, 4 as libc::c_int, 7 as libc::c_int, 11 as libc::c_int];
/* 用例対応スコア */
#[no_mangle]
pub static mut EX_match_unknown: libc::c_int = 6 as libc::c_int;
/* データ未知     */
#[no_mangle]
pub static mut EX_match_sentence: libc::c_int = 10 as libc::c_int;
/* 格要素 -- 文   */
#[no_mangle]
pub static mut EX_match_tim: libc::c_int = 0 as libc::c_int;
/* 格要素 -- 時間:時間格 */
#[no_mangle]
pub static mut EX_match_tim2: libc::c_int = 12 as libc::c_int;
/* 格要素 -- 時間:その他の格 */
#[no_mangle]
pub static mut EX_match_tim3: libc::c_int = 8 as libc::c_int;
/* 格要素 -- 時間:格選択時 */
#[no_mangle]
pub static mut EX_match_qua: libc::c_int = 9 as libc::c_int;
/* 10; */			/* 格要素 -- 数量 */
#[no_mangle]
pub static mut EX_match_exact: libc::c_int = 12 as libc::c_int;
#[no_mangle]
pub static mut EX_match_subject: libc::c_int = 8 as libc::c_int;
#[no_mangle]
pub static mut EX_match_modification: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut EX_match_demonstrative: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut SOTO_THRESHOLD: libc::c_int = 0 as libc::c_int;
/* if -probcase; otherwise DEFAULT_SOTO_THRESHOLD */
/* int	NOUN_THRESHOLD = 5; 橋渡し指示関係の閾値 */
#[no_mangle]
pub static mut NOUN_THRESHOLD: libc::c_int = 5 as libc::c_int;
/* 橋渡し指示関係の閾値 */
#[no_mangle]
pub static mut CASE_ASSIGN_THRESHOLD: libc::c_int = 0 as libc::c_int;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_assign(mut list: *mut LIST,
                                      mut cf: *mut CASE_FRAME) 
 /*==================================================================*/
 {
    /* 対応リストの表示 */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf).element_num {
        if (*list).flag[i as usize] == -(2 as libc::c_int) {
            fprintf(Outfp, b"  X\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp, b"%3d\x00" as *const u8 as *const libc::c_char,
                    (*list).flag[i as usize] + 1 as libc::c_int);
        }
        i += 1
    }
    fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn comp_sm(mut cpp: *mut libc::c_char,
                                 mut cpd: *mut libc::c_char,
                                 mut start: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    /* start からチェックする
       普通は 1 
       品詞ごとチェックするときは 0 */
    let mut i: libc::c_int = 0;
    i = start;
    while i < 12 as libc::c_int {
        if *cpp.offset(i as isize) as libc::c_int == '*' as i32 {
            return i
        } else {
            if *cpd.offset(i as isize) as libc::c_int == '*' as i32 {
                return 0 as libc::c_int
            } else {
                if *cpp.offset(i as isize) as libc::c_int !=
                       *cpd.offset(i as isize) as libc::c_int {
                    return 0 as libc::c_int
                }
            }
        }
        i += 1
    }
    return 12 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _sm_match_score(mut cpp: *mut libc::c_char,
                                         mut cpd: *mut libc::c_char,
                                         mut flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /*
      NTTの意味素の一桁目は品詞情報
      格フレーム <-----> データ
       x(補文)   <-----> xだけOK
       1(名詞)   <-----> x以外OK
                         (名詞以外のものはget_smの時点で排除 99/01/13)
    */
    /* 
       flag == SM_EXPAND_NE    : 固有名詞意味属性を一般名詞意味属性に変換する
       flag == SM_NO_EXPAND_NE : 固有名詞意味属性を一般名詞意味属性に変換しない
       flag == SM_CHECK_FULL   : コードの一文字目からチェックする
     */
    let mut current_score: libc::c_int = 0;
    let mut score: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if flag == 3 as libc::c_int { return comp_sm(cpp, cpd, 0 as libc::c_int) }
    if *cpp.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32 {
        return if *cpd.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
        {
            12 as libc::c_int
        } else { 0 as libc::c_int }
    } else {
        if *cpd.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
           {
            return 0 as libc::c_int
        }
    }
    /* 意味マーカのマッチング度の計算

       ・パターンが先に* --- マッチ
       ・データが先に* --- マッチしない
       ・最後まで一致 --- マッチ

         マッチ : マッチする階層の深さを返す
	 マッチしないとき : 0を返す
    */
    /* データが固有名詞のとき */
    return if *cpd.offset(0 as libc::c_int as isize) as libc::c_int == '2' as i32 {
        if flag == 2 as libc::c_int &&
            *cpp.offset(0 as libc::c_int as isize) as libc::c_int !=
                '2' as i32 {
            if SMP2SMGExist == 0 as libc::c_int {
                fprintf(stderr,
                        b";;; Cannot open smp2smg table!\n\x00" as *const u8
                            as *const libc::c_char);
                0 as libc::c_int
            } else {
                let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
                start = _smp2smg(cpd);
                if start.is_null() { return score }
                cp = start;
                while *cp != 0 {
                    if *cp as libc::c_int == '/' as i32 {
                        cp = cp.offset(1)
                    } else if cp != start {
                        fprintf(stderr,
                                b";;; Invalid delimiter! <%c> (%s)\n\x00" as
                                    *const u8 as *const libc::c_char,
                                *cp as libc::c_int, start);
                    }
                    /* 副作用フラグがある意味素変換は行わない */
                    if strncmp(cp.offset(12 as libc::c_int as isize),
                               b" side-effect\x00" as *const u8 as
                                   *const libc::c_char,
                               12 as libc::c_int as libc::c_ulong) == 0 {
                        cp = cp.offset(12 as libc::c_int as isize)
                    } else {
                        current_score =
                            comp_sm(cpp, cp,
                                    1 as
                                        libc::c_int); /* " side-effect" の分進める */
                        if current_score > score { score = current_score }
                    }
                    cp = cp.offset(12 as libc::c_int as isize)
                }
                free(start as *mut libc::c_void);
                score
            }
        } else if flag == 1 as libc::c_int &&
            *cpp.offset(0 as libc::c_int as isize) as libc::c_int ==
                '2' as i32 {
            comp_sm(cpp, cpd, 1 as libc::c_int)
        } else { 0 as libc::c_int }
    } else if *cpp.offset(0 as libc::c_int as isize) as libc::c_int !=
        '2' as i32 {
        comp_sm(cpp, cpd, 1 as libc::c_int)
    } else { 0 as libc::c_int };
}
/* 両方とも一般名詞のとき */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn sms_match(mut cpp: *mut libc::c_char,
                                   mut cpd: *mut libc::c_char,
                                   mut expand: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    // let mut i: libc::c_int = 0;
    if Thesaurus == 1 as libc::c_int {
        return bgh_match_check(cpp, cpd)
    } else {
        if Thesaurus == 2 as libc::c_int {
            return sm_match_check(cpp, cpd, expand)
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _cf_match_element(mut d: *mut libc::c_char,
                                           mut p: *mut libc::c_char,
                                           mut start: libc::c_int,
                                           mut len: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    if Thesaurus == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while *d.offset(i as isize) != 0 {
            j = 0 as libc::c_int;
            while *p.offset(j as isize) != 0 {
                if strncmp(d.offset(i as isize).offset(start as isize),
                           p.offset(j as isize).offset(start as isize),
                           len as libc::c_ulong) == 0 {
                    return (0 as libc::c_int == 0) as libc::c_int
                }
                j += 11 as libc::c_int
            }
            i += 11 as libc::c_int
        }
    } else if Thesaurus == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while *d.offset(i as isize) != 0 {
            /* 固有名詞体系 */
            if *d.offset(i as isize) as libc::c_int == '2' as i32 {
                /* 一般体系にマッピング 
		   ※ side-effect を無視する */
                code =
                    smp2smg(d.offset(i as isize),
                            (0 as libc::c_int == 0) as libc::c_int);
                if !code.is_null() {
                    j = 0 as libc::c_int;
                    while *code.offset(j as isize) != 0 {
                        if strncmp(code.offset(j as
                                                   isize).offset(start as
                                                                     isize),
                                   p.offset(start as isize),
                                   len as libc::c_ulong) == 0 {
                            free(code as *mut libc::c_void);
                            return (0 as libc::c_int == 0) as libc::c_int
                        }
                        j += 12 as libc::c_int
                    }
                    free(code as *mut libc::c_void);
                }
            } else if strncmp(d.offset(i as isize).offset(start as isize),
                              p.offset(start as isize), len as libc::c_ulong)
                          == 0 {
                return (0 as libc::c_int == 0) as libc::c_int
            }
            i += 12 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_element(mut d: *mut libc::c_char,
                                          mut target: *mut libc::c_char,
                                          mut flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    /* flag == TRUE  : その意味素と exact match
       flag == FALSE : その意味素以下にあれば match */
    if d.is_null() { return 0 as libc::c_int }
    if Thesaurus == 1 as libc::c_int {
        size = 11 as libc::c_int
    } else if Thesaurus == 2 as libc::c_int { size = 12 as libc::c_int }
    code = sm2code(target);
    return if flag == (0 as libc::c_int == 0) as libc::c_int {
        _cf_match_element(d, code, 0 as libc::c_int, size)
    } else {
        /* ※ コードが2文字以上ある必要がある */
        _cf_match_element(d, code, 1 as libc::c_int,
                          code_depth(code, size))
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_both_element(mut d: *mut libc::c_char,
                                               mut p: *mut libc::c_char,
                                               mut target: *mut libc::c_char,
                                               mut flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut len: libc::c_int = 0;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 両方に target が存在するかチェック */
    /* flag == TRUE  : その意味素と exact match
       flag == FALSE : その意味素以下にあれば match */
    if p.is_null() { return 0 as libc::c_int }
    code = sm2code(target);
    if Thesaurus == 1 as libc::c_int {
        return if _cf_match_element(d, code, 0 as libc::c_int, 11 as libc::c_int) ==
            (0 as libc::c_int == 0) as libc::c_int &&
            _cf_match_element(p, code, 0 as libc::c_int, 11 as libc::c_int)
                == (0 as libc::c_int == 0) as libc::c_int {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else {
        if Thesaurus == 2 as libc::c_int {
            return if flag == (0 as libc::c_int == 0) as libc::c_int {
                if _cf_match_element(d, code, 0 as libc::c_int,
                                     12 as libc::c_int) ==
                    (0 as libc::c_int == 0) as libc::c_int &&
                    _cf_match_element(p, code, 0 as libc::c_int,
                                      12 as libc::c_int) ==
                        (0 as libc::c_int == 0) as libc::c_int {
                    (0 as libc::c_int == 0) as libc::c_int
                } else { 0 as libc::c_int }
            } else {
                /* ※ コードが 2 文字以上ある必要がある
	       1文字目(品詞)を無視して、与えられたコード以下にあるかどうかチェック */
                len = code_depth(code, 12 as libc::c_int);
                if _cf_match_element(d, code, 1 as libc::c_int, len) ==
                    (0 as libc::c_int == 0) as libc::c_int &&
                    _cf_match_element(p, code, 1 as libc::c_int, len) ==
                        (0 as libc::c_int == 0) as libc::c_int {
                    (0 as libc::c_int == 0) as libc::c_int
                } else { 0 as libc::c_int }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_sm(mut as1: libc::c_int,
                                     mut cfd: *mut CASE_FRAME,
                                     mut as2: libc::c_int,
                                     mut cfp: *mut CASE_FRAME,
                                     mut pos: *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    if cf_match_both_element((*cfd).sm[as1 as usize], (*cfp).sm[as2 as usize],
                             b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             (0 as libc::c_int == 0) as libc::c_int) != 0 ||
           cf_match_both_element((*cfd).sm[as1 as usize],
                                 (*cfp).sm[as2 as usize],
                                 b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                                 (0 as libc::c_int == 0) as libc::c_int) != 0
           ||
           cf_match_both_element((*cfd).sm[as1 as usize],
                                 (*cfp).sm[as2 as usize],
                                 b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                                 (0 as libc::c_int == 0) as libc::c_int) != 0
           ||
           cf_match_both_element((*cfd).sm[as1 as usize],
                                 (*cfp).sm[as2 as usize],
                                 b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                                 (0 as libc::c_int == 0) as libc::c_int) != 0
       {
        *pos = -(1 as libc::c_int);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn elmnt_match_score_each_sm(mut as1: libc::c_int,
                                                   mut cfd: *mut CASE_FRAME,
                                                   mut as2: libc::c_int,
                                                   mut cfp: *mut CASE_FRAME,
                                                   mut pos: *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 意味素 : 格要素 -- 補文 */
    if cf_match_both_element((*cfd).sm[as1 as usize], (*cfp).sm[as2 as usize],
                             b"\xe8\xa3\x9c\xe6\x96\x87\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             (0 as libc::c_int == 0) as libc::c_int) != 0 {
        return EX_match_sentence
    } else {
        /* 意味素 : 格要素 -- 時間 */
        if cf_match_both_element((*cfd).sm[as1 as usize],
                                 (*cfp).sm[as2 as usize],
                                 b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                                 (0 as libc::c_int == 0) as libc::c_int) != 0
           {
            /* 格フレーム側が時間格の場合はスコアを低く */
               return if MatchPP((*cfp).pp[as2 as usize][0 as libc::c_int as usize],
                                 b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char) != 0 {
                   EX_match_tim
               } else if (*cfd).pp[as1 as usize][1 as libc::c_int as usize] !=
                   -(10 as libc::c_int) {
                   EX_match_tim3
               } else { EX_match_tim2 }
        } else {
            /* 格フレーム:時間格以外, 入力側:格選択時
	   格が曖昧なときは
	   1. <時間>時間格 : <時間>時間格 (score == 0)
	   2. 「用例」普通の格 : 「用例」普通の格
	   3. <時間>普通の格 : <時間>普通の格 (here) */
            /* 意味素 : 格要素 -- 数量 */
            if cf_match_both_element((*cfd).sm[as1 as usize],
                                     (*cfp).sm[as2 as usize],
                                     b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char,
                                     (0 as libc::c_int == 0) as libc::c_int)
                   != 0 {
                return EX_match_qua
            }
        }
    }
    return -(100 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_sm_thesaurus(mut tp: *mut TAG_DATA,
                                               mut cfp: *mut CASE_FRAME,
                                               mut n: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut step: libc::c_int = 0;
    let mut expand: libc::c_int = 0;
    let mut non_subj_flag: libc::c_int = 0 as libc::c_int;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    if Thesaurus == 1 as libc::c_int {
        step = 11 as libc::c_int
    } else if Thesaurus == 2 as libc::c_int { step = 12 as libc::c_int }
    if !check_feature((*tp).f,
                      b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        expand = 1 as libc::c_int
    } else { expand = 2 as libc::c_int }
    if !check_feature((*tp).f,
                      b"\xe9\x9d\x9e\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8
                          as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        non_subj_flag = 1 as libc::c_int
    }
    if Thesaurus == 1 as libc::c_int {
        code = (*tp).BGH_code.as_mut_ptr()
    } else { code = (*tp).SM_code.as_mut_ptr() }
    /* 意味属性のマッチング */
    if !(*cfp).sm[n as usize].is_null() {
        // let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while *(*cfp).sm[n as usize].offset(j as isize) != 0 {
            /* 格フレーム-主体, 人, 組織
	       主体 <=> <主体>, 人名, 組織名
	       人   <=> <人>, 人名
	       組織 <=> <組織>, 組織名
	    ※ 人名には<人>, 組織名には<組織>をruleで付与ずみ */
            if strncmp((*cfp).sm[n as usize].offset(j as isize),
                       sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char),
                       step as libc::c_ulong) == 0 ||
                   strncmp((*cfp).sm[n as usize].offset(j as isize),
                           sm2code(b"\xe4\xba\xba\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                           step as libc::c_ulong) == 0 ||
                   strncmp((*cfp).sm[n as usize].offset(j as isize),
                           sm2code(b"\xe7\xb5\x84\xe7\xb9\x94\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char),
                           step as libc::c_ulong) == 0 {
                if non_subj_flag == 0 as libc::c_int &&
                       MatchPP((*cfp).pp[n as
                                             usize][0 as libc::c_int as
                                                        usize],
                               b"\xe3\x83\xb2\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char)
                           == 0 &&
                       sms_match((*cfp).sm[n as usize].offset(j as isize),
                                 code, expand) != 0 {
                    return 1 as libc::c_int
                }
            } else if Thesaurus == 2 as libc::c_int &&
                          strncmp((*cfp).sm[n as usize].offset(j as isize),
                                  sm2code(b"\xe5\x8b\x95\xe4\xbd\x9c\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char),
                                  12 as libc::c_int as libc::c_ulong) == 0 {
                if sms_match(sm2code(b"\xe5\x90\x8d(\xe8\xbb\xa2\xe7\x94\x9f)\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_char), code,
                             3 as libc::c_int) != 0 ||
                       sms_match(sm2code(b"\xe3\x82\xb5\xe5\xa4\x89\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char), code,
                                 3 as libc::c_int) != 0 {
                    return 1 as libc::c_int
                }
            } else if strncmp((*cfp).sm[n as usize].offset(j as isize),
                              sm2code(b"\xe5\xa0\xb4\xe6\x89\x80\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char),
                              step as libc::c_ulong) == 0 {
                if sms_match((*cfp).sm[n as usize].offset(j as isize), code,
                             expand) != 0 {
                    return 1 as libc::c_int
                }
            }
            j += step
        }
    }
    return 0 as libc::c_int;
}
/* 格フレーム-動作 <=> <名(転生)>, <サ変> */
/* 格フレーム-場所 <=> <場所> */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_exactly(mut word: *mut libc::c_char,
                                          mut word_len: libc::c_int,
                                          mut ex_list: *mut *mut libc::c_char,
                                          mut ex_num: libc::c_int,
                                          mut pos: *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut ret_pos: libc::c_int = 0;
    ret_pos = check_examples(word, word_len, ex_list, ex_num);
    if ret_pos >= 0 as libc::c_int { *pos = ret_pos; return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn cf_match_exactly_for_canonical_rep(mut word:
                                                                *mut libc::c_char,
                                                            mut word_len:
                                                                libc::c_int,
                                                            mut ex_list:
                                                                *mut *mut libc::c_char,
                                                            mut ex_num:
                                                                libc::c_int,
                                                            mut pos:
                                                                *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 正規化代表表記の場合は?で切ってチェック */
    token = strtok(word, b"?\x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        if cf_match_exactly(token, strlen(token) as libc::c_int, ex_list,
                            ex_num, pos) != 0 {
            return 1 as libc::c_int
        }
        token =
            strtok(0 as *mut libc::c_char,
                   b"?\x00" as *const u8 as *const libc::c_char)
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _calc_similarity_sm_cf(mut exd: *mut libc::c_char,
                                                mut expand: libc::c_int,
                                                mut unmatch_word:
                                                    *mut libc::c_char,
                                                mut cfp: *mut CASE_FRAME,
                                                mut n: libc::c_int,
                                                mut pos: *mut libc::c_int)
 -> libc::c_float 
 /*==================================================================*/
 {
    /* 類似度計算: 意味素群 - 格フレームの格
       unmatch_word: マッチさせたくない単語 */
    return if !(*cfp).sm_specify[n as usize].is_null() {
        /* 意味素制限 */
        calc_similarity(exd, (*cfp).sm_specify[n as usize], expand)
    } else {
        calc_sm_words_similarity(exd, (*cfp).ex_list[n as usize],
                                 (*cfp).ex_num[n as usize], pos,
                                 (*cfp).sm_delete[n as usize], expand,
                                 unmatch_word)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_similarity_word_cf(mut tp: *mut TAG_DATA,
                                                 mut cfp: *mut CASE_FRAME,
                                                 mut n: libc::c_int,
                                                 mut pos: *mut libc::c_int)
 -> libc::c_float 
 /*==================================================================*/
 {
    let mut exd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expand: libc::c_int = 0;
    let mut strp_malloc_flag: libc::c_int = 0 as libc::c_int;
    let mut rep_length: libc::c_int = 0;
    let mut exact_matched_flag: libc::c_int = 0 as libc::c_int;
    // let mut ex_score: libc::c_float = 0 as libc::c_int as libc::c_float;
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
    if Thesaurus == 1 as libc::c_int {
        exd = (*tp).BGH_code.as_mut_ptr()
    } else if Thesaurus == 2 as libc::c_int {
        exd = (*tp).SM_code.as_mut_ptr()
    }
    if !check_feature((*tp).f,
                      b"\xef\xbc\xb4\xe5\x9b\xba\xe6\x9c\x89\xe4\xb8\x80\xe8\x88\xac\xe5\xb1\x95\xe9\x96\x8b\xe7\xa6\x81\xe6\xad\xa2\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        expand = 1 as libc::c_int
    } else { expand = 2 as libc::c_int }
    if OptCaseFlag & 32 as libc::c_int != 0 {
        if OptCaseFlag & 1024 as libc::c_int != 0 &&
               {
                   cp =
                       get_bnst_head_canonical_rep((*tp).b_ptr,
                                                   OptCaseFlag &
                                                       512 as libc::c_int);
                   !cp.is_null()
               } {
            strp = strdup(cp);
            strp_malloc_flag = 1 as libc::c_int
        } else {
            strp = get_mrph_rep_from_f((*tp).head_ptr, 0 as libc::c_int);
            if strp.is_null() {
                /* feature中の代表表記 */
                strp = make_mrph_rn((*tp).head_ptr); /* なければ作る */
                strp_malloc_flag = 1 as libc::c_int
            }
        }
    } else { strp = (*(*tp).head_ptr).Goi.as_mut_ptr() }
    /* exact match */
    if check_feature((*tp).f,
                     b"\xe5\xbd\xa2\xe5\x89\xaf\xe5\x90\x8d\xe8\xa9\x9e\x00"
                         as *const u8 as *const libc::c_char as
                         *mut libc::c_char).is_null() {
        if cf_match_exactly_for_canonical_rep(strp,
                                              strlen(strp) as libc::c_int,
                                              (*cfp).ex_list[n as usize],
                                              (*cfp).ex_num[n as usize], pos)
               != 0 {
            exact_matched_flag = 1 as libc::c_int
        } else if OptCaseFlag & 32 as libc::c_int != 0 &&
                      OptCaseFlag & 1024 as libc::c_int == 0 {
            fp = (*(*tp).head_ptr).f;
            while !fp.is_null() {
                if strncmp((*fp).cp,
                           b"ALT-\x00" as *const u8 as *const libc::c_char,
                           4 as libc::c_int as libc::c_ulong) == 0 {
                    if strp_malloc_flag != 0 {
                        free(strp as *mut libc::c_void);
                        strp_malloc_flag = 0 as libc::c_int
                    }
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
                    /* 正規化代表表記を使わない代表表記の場合はALTも調べる */
                    strp = get_mrph_rep(&mut m); /* 代表表記 */
                    rep_length = get_mrph_rep_length(strp);
                    if rep_length == 0 as libc::c_int {
                        /* なければ作る */
                        strp = make_mrph_rn(&mut m);
                        rep_length = strlen(strp) as libc::c_int;
                        strp_malloc_flag = 1 as libc::c_int
                    }
                    if cf_match_exactly(strp, rep_length,
                                        (*cfp).ex_list[n as usize],
                                        (*cfp).ex_num[n as usize], pos) != 0 {
                        exact_matched_flag = 1 as libc::c_int;
                        break ;
                    }
                }
                fp = (*fp).next
            }
        }
    }
    if strp_malloc_flag != 0 { free(strp as *mut libc::c_void); }
    return if exact_matched_flag != 0 {
        1.1f64 as libc::c_float
    } else if *exd.offset(0 as libc::c_int as isize) == 0 {
        -(1 as libc::c_int) as libc::c_float
    } else {
        /* 意味素なし
       候補にするために -1 を返す */
        /* 意味素 match */
        _calc_similarity_sm_cf(exd, expand, 0 as *mut libc::c_char,
                               cfp, n, pos)
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_similarity_word_cf_with_sm(mut tp:
                                                             *mut TAG_DATA,
                                                         mut cfp:
                                                             *mut CASE_FRAME,
                                                         mut n: libc::c_int,
                                                         mut pos:
                                                             *mut libc::c_int)
 -> libc::c_float 
 /*==================================================================*/
 {
    let mut ex_rawscore: libc::c_float = 0.;
    ex_rawscore = calc_similarity_word_cf(tp, cfp, n, pos);
    /* exactマッチ */
    if ex_rawscore as libc::c_double > 1.0f64 { return ex_rawscore }
    /* 主体マッチ */
    if cf_match_sm_thesaurus(tp, cfp, n) != 0 {
        *pos = -(1 as libc::c_int);
        return EX_match_subject as libc::c_float /
                   11 as libc::c_int as libc::c_float
    }
    return ex_rawscore;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn elmnt_match_score(mut as1: libc::c_int,
                                           mut cfd: *mut CASE_FRAME,
                                           mut as2: libc::c_int,
                                           mut cfp: *mut CASE_FRAME,
                                           mut flag: libc::c_int,
                                           mut pos: *mut libc::c_int,
                                           mut score: *mut libc::c_double,
                                           mut para_cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 意味マーカのマッチング度の計算 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    let mut exd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut match_score: *mut libc::c_int = 0 as *mut libc::c_int;
    *score = -(100 as libc::c_int) as libc::c_double;
    exd = (*cfd).ex[as1 as usize];
    exp = (*cfp).ex[as2 as usize];
    match_score = EX_match_score.as_mut_ptr();
    if flag == 1 as libc::c_int {
        let mut tmp_score: libc::c_int = 0;
        if SMExist == 0 as libc::c_int ||
               *(*cfd).sm[as1 as usize].offset(0 as libc::c_int as isize) as
                   libc::c_int == '\u{0}' as i32 ||
               (*cfp).sm[as2 as usize].is_null() ||
               *(*cfp).sm[as2 as usize].offset(0 as libc::c_int as isize) as
                   libc::c_int == '\u{0}' as i32 {
            *score = SM_match_unknown as libc::c_double;
            return (0 as libc::c_int == 0) as libc::c_int
        }
        j = 0 as libc::c_int;
        while *(*cfp).sm[as2 as usize].offset(j as isize) != 0 {
            /* 具体的な用例が書いてある場合 */
            if strncmp((*cfp).sm[as2 as usize].offset(j as isize),
                       sm2code(b"\xe2\x86\x92\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char),
                       12 as libc::c_int as libc::c_ulong) == 0 {
                tmp_score =
                    calc_similarity(exd, exp, 0 as libc::c_int) as
                        libc::c_int;
                if tmp_score == 1 as libc::c_int {
                    *score = 10 as libc::c_int as libc::c_double;
                    return (0 as libc::c_int == 0) as libc::c_int
                }
            } else {
                /* 選択制限によるマッチ (NTTシソーラスがある場合) */
                i = 0 as libc::c_int;
                while *(*cfd).sm[as1 as usize].offset(i as isize) != 0 {
                    tmp_score =
                        SM_match_score[_sm_match_score((*cfp).sm[as2 as
                                                                     usize].offset(j
                                                                                       as
                                                                                       isize),
                                                       (*cfd).sm[as1 as
                                                                     usize].offset(i
                                                                                       as
                                                                                       isize),
                                                       1 as libc::c_int) as
                                           usize];
                    if tmp_score as libc::c_double > *score {
                        *score = tmp_score as libc::c_double
                    }
                    i += 12 as libc::c_int
                }
            }
            j += 12 as libc::c_int
        }
        return (0 as libc::c_int == 0) as libc::c_int
    } else {
        if flag == 2 as libc::c_int {
            let mut ex_score: libc::c_int = 0;
            let mut ex_rawscore: libc::c_float = 0.;
            /* 確率的格解析のとき */
            if OptCaseFlag & 16 as libc::c_int != 0 {
                /* マッチを調べるとき *
	       cf_match_exactly(cfd->pred_b_ptr->cpm_ptr->elem_b_ptr[as1]->head_ptr->Goi, 
	       strlen(cfd->pred_b_ptr->cpm_ptr->elem_b_ptr[as1]->head_ptr->Goi), 
	       cfp->ex_list[as2], cfp->ex_num[as2], pos);
	       cf_match_sm(as1, cfd, as2, cfp, pos);
	    */
                *score =
                    get_ex_probability_with_para(as1, cfd, as2, cfp) +
                        get_case_probability(as2, cfp,
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int, para_cpm_ptr) +
                        get_case_function_probability(as1, cfd, as2, cfp,
                                                      0 as libc::c_int);
                return (0 as libc::c_int == 0) as libc::c_int
            }
            /* 修飾格のとき */
            if MatchPP((*cfd).pp[as1 as usize][0 as libc::c_int as usize],
                       b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char) != 0 {
                *score = EX_match_modification as libc::c_double;
                return (0 as libc::c_int == 0) as libc::c_int
            }
            /* 指示詞のとき */
            if !check_feature((*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1
                                                                               as
                                                                               usize]).f,
                              b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                *score = EX_match_demonstrative as libc::c_double;
                return (0 as libc::c_int == 0) as libc::c_int
            }
            /* 主体マッチ -- ガ格で意味素なしのとき固有名詞だと思う *
	    (cfd->ex[as1][0] == '\0' && 
	     cf_match_element(cfp->sm[as2], "主体", TRUE))) {
	*/
            if (*(*(*cfd).pred_b_ptr).cpm_ptr).cf.type_0 == 1 as libc::c_int {
                /* 用例のマッチング */
                ex_rawscore =
                    calc_similarity_word_cf_with_sm((*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1
                                                                                                   as
                                                                                                   usize],
                                                    cfp, as2, pos)
            } else {
                ex_rawscore =
                    calc_similarity_word_cf((*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[as1
                                                                                           as
                                                                                           usize],
                                            cfp, as2, pos)
            }
            if MatchPP((*cfp).pp[as2 as usize][0 as libc::c_int as usize],
                       b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char) != 0 {
                /* 外の関係のときシソーラスを使わない */
                return if ex_rawscore as libc::c_double > 1.0f64 {
                    *score =
                        *match_score.offset(7 as libc::c_int as isize) as
                            libc::c_double;
                    (0 as libc::c_int == 0) as libc::c_int
                } else {
                    *score = 0 as libc::c_int as libc::c_double;
                    0 as libc::c_int
                }
            } else {
                /* exact match */
                if ex_rawscore as libc::c_double > 1.0f64 {
                    *score =
                        EX_match_exact as
                            libc::c_double; /* (int)(ex_rawscore * EX_match_score[7]) */
                    return (0 as libc::c_int == 0) as libc::c_int
                }
            }
            /* <主体>共通スコア */
            if *pos == -(1 as libc::c_int) {
                *score = EX_match_subject as libc::c_double;
                return (0 as libc::c_int == 0) as libc::c_int
            } else {
                /* <補文>, <時間>, <数量> */
                *score =
                    elmnt_match_score_each_sm(as1, cfd, as2, cfp, pos) as
                        libc::c_double
            }
            /* 入力側の用例の意味属性がない場合 */
            if *exd as libc::c_int == '\u{0}' as i32 &&
                   *(*cfd).sm[as1 as usize] as libc::c_int == '\u{0}' as i32 {
                ex_rawscore =
                    0 as libc::c_int as
                        libc::c_float; /* ex_rawscore == -1 のはず */
                *score = EX_match_unknown as libc::c_double
            }
            /* 格解析用スコアに変換 */
            ex_score =
                *match_score.offset((ex_rawscore *
                                         7 as libc::c_int as libc::c_float) as
                                        libc::c_int as isize);
            /*
	if (Thesaurus == USE_NTT && 
	    sm_check_match_max(exd, exp, 0, sm2code("抽象"))) { * <抽象>のマッチを低く *
	    ex_score = EX_match_score2[(int)(ex_rawscore * 7)];
	}
	*/
            /* 大きい方をかえす */
            if ex_score as libc::c_double > *score {
                *score = ex_score as libc::c_double
            }
            /* 用例, 意味素のマッチが不成功 */
            return if *score > CASE_ASSIGN_THRESHOLD as libc::c_double {
                (0 as libc::c_int == 0) as libc::c_int
            } else { 0 as libc::c_int }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn count_pat_element(mut cfp: *mut CASE_FRAME,
                                           mut list2: *mut LIST)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut pat_element: libc::c_int = 0 as libc::c_int;
    /* すべての格が任意格だと 0 を返して 0 で除算してしまう */
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if !((*cfp).oblig[i as usize] == 0 as libc::c_int &&
                 (*list2).flag[i as usize] == -(1 as libc::c_int)) {
            pat_element += 1
        }
        i += 1
    }
    return pat_element;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn count_assigned_adjacent_element(mut cfp:
                                                             *mut CASE_FRAME,
                                                         mut list2: *mut LIST)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    /* 割り当てがある必須格の数を数える (格フレーム側) */
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if (*cfp).oblig[i as usize] == (0 as libc::c_int == 0) as libc::c_int
               && (*list2).flag[i as usize] != -(1 as libc::c_int) {
            count += 1
        }
        i += 1
    }
    return count;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_same_case(mut dp: libc::c_int,
                                         mut pp: libc::c_int,
                                         mut cf: *mut CASE_FRAME)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    if dp < pp { p1 = dp; p2 = pp } else { p1 = pp; p2 = dp }
    i = 0 as libc::c_int;
    while (*cf).samecase[i as usize][0 as libc::c_int as usize] !=
              -(10 as libc::c_int) {
        if (*cf).samecase[i as usize][0 as libc::c_int as usize] == p1 &&
               (*cf).samecase[i as usize][1 as libc::c_int as usize] == p2 {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_case(mut cf: *mut CASE_FRAME,
                                    mut c: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cf).element_num {
        j = 0 as libc::c_int;
        while (*cf).pp[i as usize][j as usize] != -(10 as libc::c_int) {
            if (*cf).pp[i as usize][j as usize] == c { return i }
            j += 1
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_adjacent_assigned(mut cfd: *mut CASE_FRAME,
                                                 mut cfp: *mut CASE_FRAME,
                                                 mut list1: *mut LIST)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        if (*cfd).adjacent[i as usize] ==
               (0 as libc::c_int == 0) as libc::c_int &&
               (*list1).flag[i as usize] != -(2 as libc::c_int) &&
               (*cfp).adjacent[(*list1).flag[i as usize] as usize] ==
                   (0 as libc::c_int == 0) as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn eval_assign_score(mut cfd: *mut CASE_FRAME,
                                           mut list1: *mut LIST,
                                           mut cfp: *mut CASE_FRAME,
                                           mut list2: *mut LIST,
                                           mut score: libc::c_int,
                                           mut closest: libc::c_int) 
 /*==================================================================*/
 {
    /* フレームのマッチング度の計算(格明示部分を除く) */
    let mut i: libc::c_int = 0; /* データ側割り当て数 */
    let mut j: libc::c_int = 0;
    let mut local_m_e: libc::c_int = 0 as libc::c_int;
    let mut local_m_p: libc::c_int = 0 as libc::c_int;
    let mut local_c_e: libc::c_int = 0 as libc::c_int;
    let mut pat_element: libc::c_int = 0;
    let mut dat_element: libc::c_int = 0 as libc::c_int;
    let mut cf_element: libc::c_int = 0 as libc::c_int;
    let mut lastpp: libc::c_int = 0;
    let mut unassigned_ga: libc::c_int = 0 as libc::c_int;
    let mut local_score: libc::c_float = 0.;
    local_score = score as libc::c_float;
    /* 要素数，要素の位置，交差数 */
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        if (*list1).flag[i as usize] != -(2 as libc::c_int) {
            local_m_e += 1;
            local_m_p += i;
            j = i + 1 as libc::c_int;
            while j < (*cfd).element_num {
                if (*list1).flag[j as usize] != -(2 as libc::c_int) &&
                       (*list1).flag[j as usize] < (*list1).flag[i as usize] {
                    local_c_e -= 1
                }
                j += 1
            }
        }
        i += 1
    }
    /* 文中の要素数(任意でマッチしていない要素以外) */
    /* ※ 埋め込み文の被修飾語は任意扱い */
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        if !((*cfd).oblig[i as usize] == 0 as libc::c_int &&
                 (*list1).flag[i as usize] == -(2 as libc::c_int)) {
            dat_element += 1
        }
        i += 1
    }
    /* 格フレーム中の要素数(任意でマッチしていない要素以外) */
    pat_element = count_pat_element(cfp, list2);
    /* 格フレーム中の要素数 */
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if (*list2).flag[i as usize] != -(1 as libc::c_int) {
            cf_element += 1;
            lastpp = (*cfp).pp[i as usize][0 as libc::c_int as usize]
        } else if MatchPP((*cfp).pp[i as usize][0 as libc::c_int as usize],
                          b"\xe3\x82\xac\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char) != 0 {
            unassigned_ga = 1 as libc::c_int
        }
        i += 1
    }
    if local_m_e < dat_element ||
           closest > -(1 as libc::c_int) &&
               (*cfd).oblig[closest as usize] ==
                   (0 as libc::c_int == 0) as libc::c_int &&
               (*list1).flag[closest as usize] == -(2 as libc::c_int) ||
           OptEllipsis == 0 && cf_element == 1 as libc::c_int &&
               MatchPP(lastpp,
                       b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char) != 0 {
        local_score = -(1 as libc::c_int) as libc::c_float
    } else if dat_element == 0 as libc::c_int ||
                  pat_element == 0 as libc::c_int ||
                  local_m_e == 0 as libc::c_int {
        local_score = 0 as libc::c_int as libc::c_float
    } else {
        /* 割り当てのないガ格がある */
        /* local_score = local_score * sqrt((double)local_m_e)
	   / sqrt((double)dat_element * pat_element);*/
        /* local_score = local_score * local_m_e
	   / (dat_element * sqrt((double)pat_element)); */
        /* 同じ格フレームでの対応付けに影響 */
        local_score =
            (local_score as libc::c_double /
                 sqrt(pat_element as libc::c_double)) as libc::c_float
        /* corpus based case analysis 00/01/04 */
	/* local_score /= 10;	* 正規化しない,最大11に */
    }
    /* corpus based case analysis 00/01/04 */
    /* 任意格に加点 */
    /* 並列の expand を行ったときのスコアを考慮する必要がある */
    /* local_score += (cfd->element_num - dat_element) * OPTIONAL_CASE_SCORE; */
    if 0 as libc::c_int != 0 && OptEllipsis != 0 {
        if local_score as libc::c_double > Current_max_score {
            Current_max_list1[0 as libc::c_int as usize] = *list1;
            Current_max_list2[0 as libc::c_int as usize] = *list2;
            Current_max_score = local_score as libc::c_double;
            Current_pure_score[0 as libc::c_int as usize] =
                score as libc::c_double;
            Current_sufficiency =
                (cf_element as libc::c_float /
                     (*cfp).element_num as libc::c_float) as libc::c_double;
            Current_max_m_e = local_m_e;
            Current_max_m_p = local_m_p;
            Current_max_c_e = local_c_e;
            Current_max_num = 1 as libc::c_int
        } else if local_score as libc::c_double == Current_max_score &&
                      Current_max_num < 10 as libc::c_int {
            Current_max_list1[Current_max_num as usize] = *list1;
            Current_max_list2[Current_max_num as usize] = *list2;
            Current_pure_score[Current_max_num as usize] =
                score as libc::c_double;
            Current_max_num += 1
        }
    } else if local_score as libc::c_double > Current_max_score ||
                  local_score as libc::c_double == Current_max_score &&
                      local_m_e > Current_max_m_e ||
                  local_score as libc::c_double == Current_max_score &&
                      local_m_e == Current_max_m_e &&
                      local_m_p > Current_max_m_p ||
                  local_score as libc::c_double == Current_max_score &&
                      local_m_e == Current_max_m_e &&
                      local_m_p == Current_max_m_p &&
                      local_c_e > Current_max_c_e ||
                  local_score as libc::c_double == Current_max_score &&
                      local_m_e == Current_max_m_e &&
                      local_m_p == Current_max_m_p &&
                      local_c_e == Current_max_c_e &&
                      unassigned_ga == 0 as libc::c_int {
        Current_max_list1[0 as libc::c_int as usize] = *list1;
        Current_max_list2[0 as libc::c_int as usize] = *list2;
        Current_max_score = local_score as libc::c_double;
        Current_pure_score[0 as libc::c_int as usize] =
            score as libc::c_double;
        Current_sufficiency =
            (cf_element as libc::c_float /
                 (*cfp).element_num as libc::c_float) as libc::c_double;
        Current_max_m_e = local_m_e;
        Current_max_m_p = local_m_p;
        Current_max_c_e = local_c_e;
        Current_max_num = 1 as libc::c_int
    } else if local_score as libc::c_double == Current_max_score &&
                  local_m_e == Current_max_m_e && local_m_p == Current_max_m_p
                  && local_c_e == Current_max_c_e &&
                  Current_max_num < 10 as libc::c_int {
        Current_max_list1[Current_max_num as usize] = *list1;
        Current_max_list2[Current_max_num as usize] = *list2;
        Current_pure_score[Current_max_num as usize] =
            score as libc::c_double;
        Current_max_num += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn eval_assign_prob(mut cfd: *mut CASE_FRAME,
                                          mut list1: *mut LIST,
                                          mut cfp: *mut CASE_FRAME,
                                          mut list2: *mut LIST,
                                          mut score: libc::c_double,
                                          mut closest: libc::c_int,
                                          mut para_cpm_ptr: *mut CF_PRED_MGR) 
 /*==================================================================*/
 {
    /* フレームのマッチング度の評価 (確率版) */
    let mut i: libc::c_int = 0;
    let mut cf_element: libc::c_int = 0 as libc::c_int;
    let mut have_topic: libc::c_int = 0 as libc::c_int;
    let mut local_score: libc::c_double = 0.;
    /* 格フレーム確率 */
    if (*cfp).type_0 == 1 as libc::c_int {
        /* とりあえず、用言のみ */
        score = get_cf_probability(cfd, cfp)
    } else { score = 0 as libc::c_int as libc::c_double }
    /* 入力側チェック */
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        cf_element += 1;
        if (*list1).flag[i as usize] == -(2 as libc::c_int) &&
               (*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[i as usize]).num <
                   (*(*cfd).pred_b_ptr).num &&
               !check_feature((*(*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_ptr[i
                                                                               as
                                                                               usize]).f,
                              b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
            have_topic = 1 as libc::c_int
        }
        /* 連体修飾節生成確率 */
        score += get_np_modifying_probability(i, cfd);
        score += (*list1).score[i as usize];
        if !(MatchPP((*cfd).pp[i as usize][0 as libc::c_int as usize],
                     b"\xcf\x86\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char) != 0 ||
                 MatchPP((*cfd).pp[i as usize][0 as libc::c_int as usize],
                         b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) != 0) {
            /* 割り当てなし */
            if (*list1).flag[i as usize] == -(2 as libc::c_int) {
                if CF_MatchPP((*cfd).pp[i as
                                            usize][0 as libc::c_int as usize],
                              cfp) != 0 {
                    score += -(20 as libc::c_int) as libc::c_double
                } else {
                    /* 対応する格スロットがない場合 => 仮想的に格スロットを作成して割り当て */
                    score +=
                        get_case_probability_from_str(pp_code_to_kstr((*cfd).pp[i
                                                                                    as
                                                                                    usize][0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               usize]),
                                                      cfp,
                                                      (0 as libc::c_int == 0)
                                                          as libc::c_int,
                                                      para_cpm_ptr);
                    score += -(20 as libc::c_int) as libc::c_double
                }
            }
        }
        i += 1
    }
    score += get_topic_generating_probability(have_topic, (*cfd).pred_b_ptr);
    /* 格フレームの格生成確率 */
    i = 0 as libc::c_int;
    while i < (*cfp).element_num {
        if !((*list2).flag[i as usize] != -(1 as libc::c_int)) {
            /* 割り当てなし */
            (*list2).score[i as usize] =
                get_case_probability(i, cfp, 0 as libc::c_int,
                                     para_cpm_ptr); /* 割り当てのある個数 */
            score += (*list2).score[i as usize]
        }
        i += 1
    }
    score += get_case_num_probability(cfp, cf_element, para_cpm_ptr);
    local_score = score;
    /* (入力側)必須格の直前格のマッチを条件とする */
    if closest > -(1 as libc::c_int) &&
           (*cfd).oblig[closest as usize] ==
               (0 as libc::c_int == 0) as libc::c_int &&
           (*list1).flag[closest as usize] == -(2 as libc::c_int) {
        local_score -= 500 as libc::c_int as libc::c_double
    }
    if local_score > Current_max_score {
        Current_max_list1[0 as libc::c_int as usize] = *list1;
        Current_max_list2[0 as libc::c_int as usize] = *list2;
        Current_max_score = local_score;
        Current_pure_score[0 as libc::c_int as usize] = score;
        Current_max_num = 1 as libc::c_int
    } else if local_score == Current_max_score &&
                  (*cfp).type_0 == 1 as libc::c_int &&
                  Current_max_num < 10 as libc::c_int {
        Current_max_list1[Current_max_num as usize] = *list1;
        Current_max_list2[Current_max_num as usize] = *list2;
        Current_pure_score[Current_max_num as usize] = score;
        Current_max_num += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _assign_list(mut cfd: *mut CASE_FRAME,
                                      mut list1: LIST,
                                      mut cfp: *mut CASE_FRAME,
                                      mut list2: LIST,
                                      mut score: libc::c_double,
                                      mut flag: libc::c_int,
                                      mut assign_flag: libc::c_int,
                                      mut closest: libc::c_int,
                                      mut para_cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 
       文中の格要素と格フレームの格要素の対応付け

       ・この関数の一回の呼び出しで処理するのは文中の格要素一つ

       ・明示されている格要素(ガ格,ヲ格など)があれば，それを処理,
         なければ明示されていない格要素(未格,埋込文など)を処理

       ・list.flag[i]にはi番目の格要素の対応付けの状況を保持

	  UNASSINGED ---- 対応付けまだ
	  NIL_ASSINGED -- 対応付けしないことを決定
          j(その他)------ 相手のj番目と対応付け

       ・明示されている格助詞の必須格で，対応する格スロットがあるのに
         意味マーカ不一致の場合，格フレームが文に対して不適当というよ
	 りは意味マーカの指定が硬すぎるので，一応対応付けを行って処理
	 を進める．
    */
    let mut target: libc::c_int =
        -(1 as libc::c_int); /* データ側の処理対象の格要素 */
    let mut target_pp: libc::c_int = 0 as libc::c_int;
    let mut gaflag: libc::c_int = 0 as libc::c_int;
    let mut sotoflag: libc::c_int = 0 as libc::c_int;
    let mut toflag: libc::c_int = 0 as libc::c_int;
    let mut match_result: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut case_available: libc::c_int = 0 as libc::c_int;
    let mut elmnt_score: libc::c_double = 0.;
    /* まだ割り当てのない格助詞のチェック */
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        if list1.flag[i as usize] == -(1 as libc::c_int) {
            /* if ((OptCaseFlag & OPT_CASE_USE_PROBABILITY) && 
		(MatchPP(cfd->pp[i][0], "修飾") || 
		MatchPP(cfd->pp[i][0], "φ"))) {
		list1.flag[i] = NIL_ASSIGNED;
		continue;
		} */
            if assign_flag == (0 as libc::c_int == 0) as libc::c_int &&
                   (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_num[i as usize] !=
                       -(1 as libc::c_int) ||
                   assign_flag == 0 as libc::c_int &&
                       (*(*(*cfd).pred_b_ptr).cpm_ptr).elem_b_num[i as usize]
                           == -(1 as libc::c_int) {
                target = i;
                break ;
            }
        }
        i += 1
    }
    if target >= 0 as libc::c_int {
        /* すでにガ格に割り当てがあるかどうか (ガ２割り当て可能かどうか) */
        i = 0 as libc::c_int;
        while i < (*cfp).element_num {
            if list2.flag[i as usize] != -(1 as libc::c_int) &&
                   MatchPP2((*cfp).pp[i as usize].as_mut_ptr(),
                            b"\xe3\x82\xac\x00" as *const u8 as *const libc::c_char) != 0 {
                gaflag = 1 as libc::c_int;
                break ;
            } else { i += 1 }
        }
        /* <主体>かどうか (外の関係割り当て可能かどうか) */
        if cf_match_element((*cfd).sm[target as usize],
                            b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as libc::c_int) == 0 {
            sotoflag = 1 as libc::c_int
        }
        /* すでに補文ト格に割り当てがあるかどうか (ヲ格割り当て可能かどうか) */
        i = 0 as libc::c_int;
        while i < (*cfp).element_num {
            if list2.flag[i as usize] != -(1 as libc::c_int) &&
                   MatchPP2((*cfp).pp[i as usize].as_mut_ptr(),
                            b"\xe3\x83\x88\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                /* cf_match_element(cfp->sm[i], "補文", TRUE)) { */
                toflag = 1 as libc::c_int;
                break ;
            } else { i += 1 }
        }
        /* 格フレームの格ループ */
        i = 0 as libc::c_int;
        while i < (*cfp).element_num {
            /* 格フレームの空いている格 */
            if list2.flag[i as usize] == -(1 as libc::c_int) {
                /* 解釈されうる格のループ */
                target_pp = 0 as libc::c_int;
                while (*cfd).pp[target as usize][target_pp as usize] !=
                          -(10 as libc::c_int) {
                    j = 0 as libc::c_int;
                    while (*cfp).pp[i as usize][j as usize] >=
                              0 as libc::c_int {
                        /* 自動構築格フレームには複数の格はない */
                        if (*cfd).pp[target as usize][target_pp as usize] ==
                               (*cfp).pp[i as usize][j as usize] &&
                               !((*cfp).pp[i as usize][j as usize] ==
                                     pp_kstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                         as *mut libc::c_char)
                                     && sotoflag == 0 ||
                                     (*cfp).pp[i as usize][j as usize] ==
                                         pp_kstr_to_code(b"\xe3\x82\xac\xef\xbc\x92\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char)
                                         && gaflag == 0 ||
                                     (*cfp).pp[i as usize][j as usize] ==
                                         pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char)
                                         &&
                                         check_adjacent_assigned(cfd, cfp,
                                                                 &mut list1)
                                             == 0 as libc::c_int) ||
                               (*cfd).pp[target as usize][target_pp as usize]
                                   ==
                                   pp_kstr_to_code(b"\xe6\x9c\xaa\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char) &&
                                   check_same_case((*cfd).sp[target as usize],
                                                   (*cfp).pp[i as
                                                                 usize][j as
                                                                            usize],
                                                   cfp) != 0 {
                            case_available = 1 as libc::c_int;
                            pos = -(2 as libc::c_int);
                            match_result =
                                elmnt_match_score(target, cfd, i, cfp, flag,
                                                  &mut pos, &mut elmnt_score,
                                                  para_cpm_ptr);
                            if OptCaseFlag & 16 as libc::c_int != 0 ||
                                   (*cfp).pp[i as usize][j as usize] !=
                                       pp_kstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char)
                                       &&
                                       (*cfp).pp[i as usize][j as usize] !=
                                           pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                               as *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char)
                                       &&
                                       (*(*(*cfd).pred_b_ptr).cpm_ptr).cf.type_0
                                           != 2 as libc::c_int ||
                                   ((*cfp).pp[i as usize][j as usize] ==
                                        pp_kstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char)
                                        ||
                                        (*cfp).pp[i as usize][j as usize] ==
                                            pp_kstr_to_code(b"\xe3\x83\x8e\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char))
                                       &&
                                       elmnt_score >=
                                           SOTO_THRESHOLD as libc::c_double ||
                                   (*(*(*cfd).pred_b_ptr).cpm_ptr).cf.type_0
                                       == 2 as libc::c_int &&
                                       elmnt_score >=
                                           NOUN_THRESHOLD as libc::c_double {
                                if flag == 2 as libc::c_int ||
                                       flag == 1 as libc::c_int &&
                                           elmnt_score >=
                                               0 as libc::c_int as
                                                   libc::c_double {
                                    /* 対応付けをして，残りの格要素の処理に進む
				       ※ flag == SEMANTIC_MARKER && elmnt_score == 0
				       すなわち，格助詞の対応する格スロットがあるのに
				       意味マーカ不一致の場合も，処理を進める */
                                    if (*cfd).weight[target as usize] != 0 {
                                        elmnt_score /=
                                            (*cfd).weight[target as usize] as
                                                libc::c_double
                                    }
                                    list1.flag[target as usize] = i;
                                    list2.flag[i as usize] = target;
                                    list1.score[target as usize] =
                                        elmnt_score;
                                    list2.score[i as usize] = elmnt_score;
                                    list2.pos[i as usize] = pos;
                                    assign_list(cfd, list1, cfp, list2,
                                                score + elmnt_score, flag,
                                                closest, para_cpm_ptr);
                                    list2.flag[i as usize] =
                                        -(1 as libc::c_int);
                                    list2.pos[i as usize] =
                                        -(2 as libc::c_int)
                                }
                            }
                            break ;
                        } else { j += 1 }
                    }
                    target_pp += 1
                }
            }
            i += 1
        }
        if !((*(*(*cfd).pred_b_ptr).cpm_ptr).cf.type_0 == 1 as libc::c_int &&
                 OptCaseFlag & 16 as libc::c_int != 0) || case_available == 0
           {
            /* target番目の格要素には対応付けを行わないマーク */
            list1.flag[target as usize] = -(2 as libc::c_int);
            /* 任意格とし対応付けを行わない場合
	       ※ 同じ表層格が格フレームにある場合，対応付けをすることは
	       すでに上で試されている
	       if (cfd->oblig[target] == FALSE) */
	    /* 必須格で対応無(表層格の一致するものがない)の場合
	       => eval_assignで不許可
	       必須格で対応有の場合
	       => 後ろに同じ格助詞があれば対応付けをしない可能性も試す? */
            /* 割り当てなしのスコア */
            elmnt_score =
                -13.815511f64 +
                    get_case_function_probability_for_pred(target, cfd,
                                                           -(2 as
                                                                 libc::c_int),
                                                           cfp,
                                                           0 as libc::c_int);
            if (*cfd).weight[target as usize] != 0 {
                elmnt_score /=
                    (*cfd).weight[target as usize] as libc::c_double
            }
            list1.score[target as usize] = elmnt_score;
            assign_list(cfd, list1, cfp, list2,
                        if OptCaseFlag & 16 as libc::c_int != 0 {
                            (score) + elmnt_score
                        } else { score }, flag, closest, para_cpm_ptr);
        }
        return 0 as libc::c_int
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_list(mut cfd: *mut CASE_FRAME,
                                     mut list1: LIST,
                                     mut cfp: *mut CASE_FRAME,
                                     mut list2: LIST,
                                     mut score: libc::c_double,
                                     mut flag: libc::c_int,
                                     mut closest: libc::c_int,
                                     mut para_cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 未格, 連格以外を先に割り当て */
    if _assign_list(cfd, list1, cfp, list2, score, flag,
                    (0 as libc::c_int == 0) as libc::c_int, closest,
                    para_cpm_ptr) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if _assign_list(cfd, list1, cfp, list2, score, flag, 0 as libc::c_int,
                    closest, para_cpm_ptr) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* 評価 : すべての対応付けが終わった場合 */
    if OptCaseFlag & 16 as libc::c_int != 0 {
        eval_assign_prob(cfd, &mut list1, cfp, &mut list2, score, closest,
                         para_cpm_ptr);
    } else {
        eval_assign_score(cfd, &mut list1, cfp, &mut list2,
                          score as libc::c_int, closest);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn case_frame_match(mut cpm_ptr: *mut CF_PRED_MGR,
                                          mut cmm_ptr: *mut CF_MATCH_MGR,
                                          mut flag: libc::c_int,
                                          mut closest: libc::c_int,
                                          mut para_cpm_ptr: *mut CF_PRED_MGR)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 格フレームのマッチング */
    let mut assign_d_list: LIST =
        LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],};
    let mut assign_p_list: LIST =
        LIST{flag: [0; 24], score: [0.; 24], pos: [0; 24],};
    let mut i: libc::c_int = 0;
    let mut cfd: *mut CASE_FRAME = &mut (*cpm_ptr).cf;
    /* 初期化 */
    Current_max_num = 0 as libc::c_int;
    Current_max_score =
        if OptCaseFlag & 16 as libc::c_int != 0 {
            -(1001 as libc::c_int)
        } else { -(2 as libc::c_int) } as libc::c_double;
    Current_sufficiency = 0 as libc::c_int as libc::c_double;
    Current_max_m_e = 0 as libc::c_int;
    Current_max_m_p = 0 as libc::c_int;
    Current_max_c_e = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*cfd).element_num {
        assign_d_list.flag[i as usize] = -(1 as libc::c_int);
        assign_d_list.score[i as usize] =
            -(1 as libc::c_int) as libc::c_double;
        i += 1
    }
    /* for (i = 0; i < cmm_ptr->cf_ptr->element_num; i++) { */
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        assign_p_list.flag[i as usize] = -(1 as libc::c_int);
        assign_p_list.score[i as usize] =
            -(1 as libc::c_int) as libc::c_double;
        assign_p_list.pos[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* 処理 */
    /* flag: 例 or 意味コード */
    assign_list(cfd, assign_d_list, (*cmm_ptr).cf_ptr, assign_p_list,
                0 as libc::c_int as libc::c_double, flag, closest,
                para_cpm_ptr);
    /* 後処理 */
    if Current_max_num == 10 as libc::c_int && OptDisplay == 3 as libc::c_int
       {
        fprintf(stderr,
                b"; Too many case matching result !\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    (*cmm_ptr).sufficiency = Current_sufficiency;
    (*cmm_ptr).result_num = Current_max_num;
    i = 0 as libc::c_int;
    while i < Current_max_num {
        (*cmm_ptr).result_lists_p[i as usize] = Current_max_list2[i as usize];
        (*cmm_ptr).result_lists_d[i as usize] = Current_max_list1[i as usize];
        (*cmm_ptr).pure_score[i as usize] = Current_pure_score[i as usize];
        i += 1
    }
    /* 直前格要素のスコアのみを用いるとき */
    if closest > -(1 as libc::c_int) &&
           Current_max_score >= 0 as libc::c_int as libc::c_double &&
           Current_max_list1[0 as libc::c_int as usize].flag[closest as usize]
               != -(2 as libc::c_int) {
        /* 直前格要素の割り当てがあることが条件 */
        (*cmm_ptr).score =
            Current_max_list1[0 as libc::c_int as
                                  usize].score[closest as usize]
    } else { (*cmm_ptr).score = Current_max_score }
    return 1 as libc::c_int;
}
/*====================================================================
                               END
====================================================================*/
