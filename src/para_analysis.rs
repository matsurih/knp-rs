#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{atoi, fprintf, match_matrix, Para_matrix, restrict_matrix, sscanf, strcmp, strlen, strncmp};
use crate::ctools::{check_feature, Language, stderr};
use crate::feature::{feature_AND_match, string2feature_pattern};
use crate::lib_print::print_matrix;
use crate::para_relation::detect_para_relation;
use crate::quote::quote_data;
use crate::structs::CDB_FILE;
use crate::tools::{OptDisplay, OptInput, OptParaFix};
use crate::types::{BNST_DATA, DBM_FILE, FEATURE, PARA_DATA, Para_M_ptr, SENTENCE_DATA};

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
pub static mut score_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut prepos_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut maxpos_array: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut maxsco_array: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut penalty_table: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut norm: [libc::c_float; 50] =
    [1.00f64 as libc::c_float, 1.00f64 as libc::c_float,
        1.59f64 as libc::c_float, 2.08f64 as libc::c_float,
        2.52f64 as libc::c_float, 2.92f64 as libc::c_float,
        3.30f64 as libc::c_float, 3.66f64 as libc::c_float,
        4.00f64 as libc::c_float, 4.33f64 as libc::c_float,
        4.64f64 as libc::c_float, 4.95f64 as libc::c_float,
        5.24f64 as libc::c_float, 5.53f64 as libc::c_float,
        5.81f64 as libc::c_float, 6.08f64 as libc::c_float,
        6.35f64 as libc::c_float, 6.61f64 as libc::c_float,
        6.87f64 as libc::c_float, 7.12f64 as libc::c_float,
        7.37f64 as libc::c_float, 7.61f64 as libc::c_float,
        7.85f64 as libc::c_float, 8.09f64 as libc::c_float,
        8.32f64 as libc::c_float, 8.55f64 as libc::c_float,
        8.78f64 as libc::c_float, 9.00f64 as libc::c_float,
        9.22f64 as libc::c_float, 9.44f64 as libc::c_float,
        9.65f64 as libc::c_float, 9.87f64 as libc::c_float,
        10.08f64 as libc::c_float, 10.29f64 as libc::c_float,
        10.50f64 as libc::c_float, 10.70f64 as libc::c_float,
        10.90f64 as libc::c_float, 11.10f64 as libc::c_float,
        11.30f64 as libc::c_float, 11.50f64 as libc::c_float,
        11.70f64 as libc::c_float, 11.89f64 as libc::c_float,
        12.08f64 as libc::c_float, 12.27f64 as libc::c_float,
        12.46f64 as libc::c_float, 12.65f64 as libc::c_float,
        12.84f64 as libc::c_float, 13.02f64 as libc::c_float,
        13.21f64 as libc::c_float, 13.39f64 as libc::c_float];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mask_quote_scope(mut sp: *mut SENTENCE_DATA,
                                          mut key_pos: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    /* 括弧がある場合に並列構造の範囲に制限を設ける */
    k = 0 as libc::c_int;
    while quote_data.in_num[k as usize] >= 0 as libc::c_int {
        let mut start: libc::c_int = quote_data.in_num[k as usize];
        let mut end: libc::c_int = quote_data.out_num[k as usize];
        /* 後に括弧がある場合 */
        if key_pos < start {
            i = 0 as libc::c_int;
            while i < start {
                j = start;
                while j < end {
                    (*restrict_matrix.as_mut_ptr().offset(i as
                        isize))[j as
                        usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                i += 1
            }
        } else if end <= key_pos {
            i = start + 1 as libc::c_int;
            while i <= end {
                j = end + 1 as libc::c_int;
                while j < (*sp).Bnst_num {
                    (*restrict_matrix.as_mut_ptr().offset(i as
                        isize))[j as
                        usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                i += 1
            }
        } else {
            /* 前に括弧がある場合 (キーが括弧内の末尾の場合も) */
            /* キーが括弧の中にある場合 */
            i = 0 as libc::c_int;
            while i <= end {
                j = start;
                while j < (*sp).Bnst_num {
                    if i < start || end < j {
                        (*restrict_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]
                            = 0 as libc::c_int
                    }
                    j += 1
                }
                i += 1
            }
            /* 括弧の中に句点がある場合 */
            l = start;
            while l < end {
                if !check_feature((*(*sp).bnst_data.offset(l as isize)).f,
                                  b"\xe4\xbf\x82:\xe6\x96\x87\xe6\x9c\xab\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    i = start;
                    while i <= l {
                        j = l + 1 as libc::c_int;
                        while j <= end {
                            (*restrict_matrix.as_mut_ptr().offset(i as
                                isize))[j
                                as
                                usize]
                                = 0 as libc::c_int;
                            j += 1
                        }
                        i += 1
                    }
                }
                l += 1
            }
        }
        k += 1
    }
    if k != 0 && OptDisplay == 3 as libc::c_int {
        print_matrix(sp, 6 as libc::c_int, key_pos);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn bnst_match(mut sp: *mut SENTENCE_DATA,
                                    mut pos1: libc::c_int,
                                    mut pos2: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut flag1: libc::c_int = 0;
    let mut flag2: libc::c_int = 0;
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr1: *mut BNST_DATA = &mut *(*sp).bnst_data.offset(pos1 as isize) as *mut BNST_DATA;
    let mut ptr2: *mut BNST_DATA = &mut *(*sp).bnst_data.offset(pos2 as isize) as *mut BNST_DATA;
    /*
      パスのスコア計算において区切りペナルティをcancelする条件
    	・係が同じ
	・用言であるかどうかが同じ
	・読点があるかないかが同じ
	
       ※ 条件は少し緩くしてある．問題があれば強める 
    */
    cp1 =
        check_feature((*ptr1).f,
                      b"\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char);
    cp2 =
        check_feature((*ptr2).f,
                      b"\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char);
    if cp1.is_null() || cp2.is_null() || strcmp(cp1, cp2) != 0 {
        return 0 as libc::c_int;
    }
    flag1 =
        if !check_feature((*ptr1).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    flag2 =
        if !check_feature((*ptr2).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    if flag1 != flag2 { return 0 as libc::c_int; }
    if !check_feature((*ptr1).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        cp1 =
            check_feature((*ptr1).f,
                          b"ID\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        cp2 =
            check_feature((*ptr2).f,
                          b"ID\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if cp1.is_null() || cp2.is_null() || strcmp(cp1, cp2) != 0 {
            return 0 as libc::c_int;
        }
    }
    flag1 =
        if !check_feature((*ptr1).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    flag2 =
        if !check_feature((*ptr2).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    if flag1 != flag2 { return 0 as libc::c_int; }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_static_level_penalty(mut sp: *mut SENTENCE_DATA,
                                                   mut key_pos: libc::c_int,
                                                   mut pos: libc::c_int)
                                                   -> libc::c_int
/*==================================================================*/
{
    let mut minus_score: libc::c_int = 0 as libc::c_int;
    let mut level1: libc::c_int =
        (*(*sp).bnst_data.offset(key_pos as isize)).sp_level;
    let mut level2: libc::c_int =
        (*(*sp).bnst_data.offset(pos as isize)).sp_level;
    if level1 <= level2 {
        minus_score = 7 as libc::c_int * (level2 - level1 + 1 as libc::c_int)
    }
    return minus_score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_dynamic_level_penalty(mut sp:
                                                    *mut SENTENCE_DATA,
                                                    mut key_pos: libc::c_int,
                                                    mut pos1: libc::c_int,
                                                    mut pos2: libc::c_int)
                                                    -> libc::c_int
/*==================================================================*/
{
    return if (*(*sp).bnst_data.offset(pos1 as isize)).sp_level ==
        (*(*sp).bnst_data.offset(pos2 as isize)).sp_level &&
        bnst_match(sp, pos1, pos2) != 0 &&
        bnst_match(sp, pos1, key_pos) == 0 {
        0 as libc::c_int
    } else if !check_feature((*(*sp).bnst_data.offset(pos1 as isize)).f,
                             b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
        !check_feature((*(*sp).bnst_data.offset(pos2 as isize)).f,
                       b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        0 as libc::c_int
    } else {
        /* 「〜は」の場合は読点の有無,レベルを無視 */
        penalty_table[pos1 as usize] + penalty_table[pos2 as usize]
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_starting_bonus_score(mut sp: *mut SENTENCE_DATA,
                                                   mut istart_pos:
                                                   libc::c_int,
                                                   mut p_ptr: *mut PARA_DATA)
                                                   -> libc::c_int
/*==================================================================*/
{
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    b_ptr =
        &mut *(*sp).bnst_data.offset(istart_pos as isize) as *mut BNST_DATA;
    return if (*p_ptr).type_0 == 3 as libc::c_int {
        0 as libc::c_int
    } else if (*p_ptr).type_0 == 1 as libc::c_int {
        if !check_feature((*b_ptr).f,
                          b"\xe5\x90\x8d\xe4\xb8\xa6\xe5\xa7\x8b\xe7\x82\xb9\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            2 as libc::c_int
        } else { 0 as libc::c_int }
    } else if (*p_ptr).type_0 == 2 as libc::c_int {
        if !check_feature((*b_ptr).f,
                          b"\xe8\xbf\xb0\xe4\xb8\xa6\xe5\xa7\x8b\xe7\x82\xb9\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            2 as libc::c_int
        } else { 0 as libc::c_int }
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_ending_bonus_score(mut sp: *mut SENTENCE_DATA,
                                                 mut jend_pos: libc::c_int,
                                                 mut p_ptr: *mut PARA_DATA)
                                                 -> libc::c_int
/*==================================================================*/
{
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    b_ptr = &mut *(*sp).bnst_data.offset(jend_pos as isize) as *mut BNST_DATA;
    return if (*p_ptr).type_0 == 3 as libc::c_int {
        0 as libc::c_int
    } else if (*p_ptr).type_0 == 1 as libc::c_int {
        if !check_feature((*(*sp).bnst_data.offset((*p_ptr).key_pos as
            isize)).f,
                          b"\xe4\xbf\x82:\xe3\x83\x88\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            !check_feature((*b_ptr).f,
                           b"\xef\xbc\xb4\xe5\x90\x8d\xe4\xb8\xa6\xe7\xb5\x82\xe7\x82\xb9\xe3\x80\x9c\xe3\x81\xa8\xe3\x80\x9c\xe3\x81\xa8\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            return 2 as libc::c_int;
        }
        if !check_feature((*b_ptr).f,
                          b"\xef\xbc\xb4\xe5\x90\x8d\xe4\xb8\xa6\xe7\xb5\x82\xe7\x82\xb9\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            2 as libc::c_int
        } else { 0 as libc::c_int }
    } else if (*p_ptr).type_0 == 2 as libc::c_int {
        if !check_feature((*b_ptr).f,
                          b"\xef\xbc\xb4\xe8\xbf\xb0\xe4\xb8\xa6\xe7\xb5\x82\xe7\x82\xb9\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            2 as libc::c_int
        } else { 0 as libc::c_int }
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dp_search_scope(mut sp: *mut SENTENCE_DATA,
                                         mut key_pos: libc::c_int,
                                         mut iend_pos: libc::c_int,
                                         mut jend_pos: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut current_max: libc::c_int = 0;
    let mut score_upward: libc::c_int = 0;
    let mut score_sideway: libc::c_int = 0;
    /* ＤＰマッチング */
    j = jend_pos;
    while j > key_pos {
        /* 最右列の処理 */
        if j == jend_pos {
            score_matrix[iend_pos as usize][jend_pos as usize] =
                (*match_matrix.as_mut_ptr().offset(iend_pos as
                    isize))[jend_pos as
                    usize];
            prepos_matrix[iend_pos as usize][jend_pos as usize] =
                -(1 as libc::c_int);
            i = iend_pos - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                score_matrix[i as usize][jend_pos as usize] =
                    -(1000 as libc::c_int);
                i -= 1
            }
        } else {
            /* 最下行の処理 */
            score_sideway =
                score_matrix[iend_pos as
                    usize][(j + 1 as libc::c_int) as usize] -
                    1 as libc::c_int - penalty_table[j as usize];
            score_matrix[iend_pos as usize][j as usize] = score_sideway;
            prepos_matrix[iend_pos as usize][j as usize] = iend_pos;
            /* 他の行の処理:下からと左からのスコアを比較 */
            i = iend_pos - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                score_upward =
                    if Language == 2 as libc::c_int {
                        ((*match_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize])
                            + maxsco_array[(i + 1 as libc::c_int) as usize]
                    } else {
                        ((*match_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]
                            + maxsco_array[(i + 1 as libc::c_int) as usize])
                            - calc_dynamic_level_penalty(sp, key_pos, i, j)
                    };
                score_sideway =
                    if Language == 2 as libc::c_int {
                        (score_matrix[i as
                            usize][(j + 1 as libc::c_int) as
                            usize]) -
                            1 as libc::c_int
                    } else {
                        (score_matrix[i as
                            usize][(j + 1 as libc::c_int) as
                            usize] -
                            1 as libc::c_int) - penalty_table[j as usize]
                    };
                if Language == 2 as libc::c_int &&
                    (!check_feature((*(*sp).bnst_data.offset(j as
                        isize)).f,
                                    b"CC\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() ||
                        !check_feature((*(*sp).bnst_data.offset(j as
                            isize)).f,
                                       b"PU\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null()) {
                    /* 中国語で並列のキーが後部の先頭の場合の例外処理 */
                    score_matrix[i as usize][j as usize] = score_sideway;
                    prepos_matrix[i as usize][j as usize] = i
                } else if score_upward >= score_sideway {
                    score_matrix[i as usize][j as usize] = score_upward;
                    prepos_matrix[i as usize][j as usize] =
                        maxpos_array[(i + 1 as libc::c_int) as usize]
                } else {
                    score_matrix[i as usize][j as usize] = score_sideway;
                    prepos_matrix[i as usize][j as usize] = i
                }
                i -= 1
            }
        }
        /* 次の列のために最大値，最大位置を計算 */
        current_max = score_matrix[iend_pos as usize][j as usize];
        maxpos_array[iend_pos as usize] = iend_pos;
        maxsco_array[iend_pos as usize] =
            score_matrix[iend_pos as usize][j as usize];
        i = iend_pos - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            current_max -=
                if Language == 2 as libc::c_int {
                    1 as libc::c_int
                } else { (1 as libc::c_int) + penalty_table[i as usize] };
            if current_max <= score_matrix[i as usize][j as usize] {
                current_max = score_matrix[i as usize][j as usize];
                maxpos_array[i as usize] = i;
                maxsco_array[i as usize] = current_max
            } else {
                maxpos_array[i as usize] =
                    maxpos_array[(i + 1 as libc::c_int) as usize];
                maxsco_array[i as usize] = current_max
            }
            i -= 1
        }
        j -= 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _detect_para_scope(mut sp: *mut SENTENCE_DATA,
                                            mut para_num: libc::c_int,
                                            mut ptr: *mut PARA_DATA,
                                            mut jend_pos: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut nth: libc::c_int = 0;
    let mut key_pos: libc::c_int = (*ptr).key_pos;
    let mut iend_pos: libc::c_int = (*ptr).iend_pos;
    let mut starting_bonus_score: libc::c_int = 0;
    let mut ending_bonus_score: libc::c_int = 0;
    let mut max_pos: libc::c_int = -(1 as libc::c_int);
    let mut current_score: libc::c_float = 0.;
    let mut sim_threshold: libc::c_float = 0.;
    let mut new_threshold: libc::c_float = 0.;
    let mut max_score: libc::c_float = -100.0f64 as libc::c_float;
    let mut pure_score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    /*							    */
    /* スタート位置(jend_pos)からの解析を本当に行うかどうか */
    /*							    */
    i = iend_pos;
    while i >= 0 as libc::c_int {
        (*Para_matrix.as_mut_ptr().offset(para_num as
            isize))[i as
            usize][jend_pos as
            usize] =
            -(2147483647 as libc::c_int) as libc::c_double;
        i -= 1
    }
    /* 類似度が0なら中止 */
    if (*match_matrix.as_mut_ptr().offset(iend_pos as
        isize))[jend_pos as usize] ==
        0 as libc::c_int {
        return;
    }
    /* restrict_matrixで可能性がない場合は中止 */
    flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= iend_pos {
        if (*restrict_matrix.as_mut_ptr().offset(i as
            isize))[jend_pos as
            usize] != 0 {
            flag = (0 as libc::c_int == 0) as libc::c_int;
            break;
        } else { i += 1 }
    }
    if flag == 0 as libc::c_int { return; }
    /* 「〜，それを」という並列は中止 */
    if key_pos + 1 as libc::c_int == jend_pos &&
        !check_feature((*(*sp).bnst_data.offset(jend_pos as isize)).f,
                       b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        return;
    }
    /* ルールによる制限(類似スコアの閾値を取得) */
    /* 条件がなければ閾値は0.0に */
    if (*ptr).f_pattern.fp[0 as libc::c_int as usize].is_null() {
        sim_threshold = 0.0f64 as libc::c_float
    } else {
        /* 条件があれば，マッチするものの中で最低の閾値に */
        sim_threshold = 100.0f64 as libc::c_float;
        nth = 0 as libc::c_int;
        loop {
            fp = (*ptr).f_pattern.fp[nth as usize];
            if fp.is_null() { break; }
            if feature_AND_match(fp,
                                 (*(*sp).bnst_data.offset(jend_pos as
                                     isize)).f,
                                 (*sp).bnst_data.offset(key_pos as isize) as
                                     *mut libc::c_void,
                                 (*sp).bnst_data.offset(jend_pos as isize) as
                                     *mut libc::c_void) ==
                (0 as libc::c_int == 0) as libc::c_int {
                cp =
                    check_feature(fp,
                                  b"&ST\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                if !cp.is_null() {
                    sscanf(cp,
                           b"&ST:%f\x00" as *const u8 as *const libc::c_char,
                           &mut new_threshold as *mut libc::c_float);
                } else { new_threshold = 0.0f64 as libc::c_float }
                if new_threshold < sim_threshold {
                    sim_threshold = new_threshold
                }
            }
            nth += 1
        }
        if sim_threshold as libc::c_double == 100.0f64 { return; }
    }
    /*		    */
    /* DP MATCHING  */
    /*		    */
    dp_search_scope(sp, key_pos, iend_pos, jend_pos);
    /* 最大パスの検出 */
    ending_bonus_score = calc_ending_bonus_score(sp, jend_pos, ptr);
    i = iend_pos;
    while i >= 0 as libc::c_int {
        starting_bonus_score = calc_starting_bonus_score(sp, i, ptr);
        if Language == 2 as libc::c_int &&
            starting_bonus_score != 0 as libc::c_int {
            current_score = max_score + starting_bonus_score as libc::c_float
        } else {
            current_score =
                if Language == 2 as libc::c_int {
                    (score_matrix[i as
                        usize][(key_pos + 2 as libc::c_int) as
                        usize] as libc::c_float /
                        norm[(jend_pos - i + 1 as libc::c_int) as usize] +
                        starting_bonus_score as libc::c_float) +
                        ending_bonus_score as libc::c_float
                } else {
                    (score_matrix[i as
                        usize][(key_pos + 1 as libc::c_int) as
                        usize] as libc::c_float /
                        norm[(jend_pos - i + 1 as libc::c_int) as usize] +
                        starting_bonus_score as libc::c_float) +
                        ending_bonus_score as libc::c_float
                }
        }
        if (*restrict_matrix.as_mut_ptr().offset(i as
            isize))[jend_pos as
            usize] != 0
            && max_score < current_score {
            max_score = current_score;
            pure_score =
                if Language == 2 as libc::c_int {
                    (score_matrix[i as
                        usize][(key_pos + 2 as libc::c_int) as
                        usize] as libc::c_float) /
                        norm[(jend_pos - i + 1 as libc::c_int) as usize]
                } else {
                    (score_matrix[i as
                        usize][(key_pos + 1 as libc::c_int) as
                        usize] as libc::c_float) /
                        norm[(jend_pos - i + 1 as libc::c_int) as usize]
                };
            /* pure_score は末尾表現のボーナスを除いた値 */
            max_pos = i
        }
        /* 確率的並列構造解析のために類似度を保存 */
        if (*restrict_matrix.as_mut_ptr().offset(i as
            isize))[jend_pos as
            usize] != 0
            && pure_score >= sim_threshold {
            (*Para_matrix.as_mut_ptr().offset(para_num as
                isize))[i as
                usize][jend_pos
                as
                usize]
                = current_score as libc::c_double
        }
        i -= 1
    }
    /* 類似度が0なら中止 01/07/12 */
    if (max_score as libc::c_double) < 0.0f64 { return; }
    /* ▼ (a...)(b)という並列は扱えない．括弧の制限などでこうならざる
       をえない場合は，並列とは認めないことにする (暫定的) */
    /* 「〜はもちろん」の扱いで話が変ってっきた？？？
    if (key_pos + 1 == jend_pos && max_pos != key_pos) {
	max_pos = i;
	max_score = -100;
	return;
    }
    */
    /*
      閾値を越えて，まだstatusが x なら n に
      閾値を越えて，statusが n なら スコア比較
      閾値を越えなくても，参考のためスコアを記憶
    */
    flag = 0 as libc::c_int;
    if sim_threshold <= pure_score &&
        (*ptr).status as libc::c_int == 'x' as i32 {
        (*ptr).status = 'n' as i32 as libc::c_char;
        flag = (0 as libc::c_int == 0) as libc::c_int
    } else if sim_threshold <= pure_score &&
        (*ptr).status as libc::c_int == 'n' as i32 &&
        (*ptr).max_score < max_score {
        flag = (0 as libc::c_int == 0) as libc::c_int
    } else if (*ptr).status as libc::c_int == 'x' as i32 &&
        (*ptr).max_score < max_score {
        flag = (0 as libc::c_int == 0) as libc::c_int
    }
    if flag == (0 as libc::c_int == 0) as libc::c_int {
        (*ptr).max_score = max_score;
        (*ptr).pure_score = pure_score;
        (*ptr).max_path[0 as libc::c_int as usize] = max_pos;
        j = 0 as libc::c_int;
        loop {
            (*ptr).max_path[(j + 1 as libc::c_int) as usize] =
                prepos_matrix[(*ptr).max_path[j as usize] as
                    usize][(j + key_pos + 1 as libc::c_int) as
                    usize];
            if (*ptr).max_path[(j + 1 as libc::c_int) as usize] ==
                -(1 as libc::c_int) {
                (*ptr).jend_pos = j + key_pos + 1 as libc::c_int;
                break;
            } else { j += 1 }
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn detect_para_scope(mut sp: *mut SENTENCE_DATA,
                                           mut para_num: libc::c_int,
                                           mut restrict_p: libc::c_int)
                                           -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut para_ptr: *mut PARA_DATA =
        &mut *(*sp).para_data.offset(para_num as isize) as *mut PARA_DATA;
    let mut key_pos: libc::c_int = (*para_ptr).key_pos;
    /* 
       restrict_p
         TRUE : 前の並列解析の失敗によって特定のキーだけを処理する場合
	 FALSE : はじめにすべてのキーを処理する場合
	 
       restrict_matrix
         括弧による制限と前の並列構造解析による制限(restrict_pの場合)
	 (restrict_p==FALSEの場合ここで初期化)
    */
    (*para_ptr).status = 'x' as i32 as libc::c_char;
    (*para_ptr).max_score = -100.0f64 as libc::c_float;
    (*para_ptr).pure_score = -100.0f64 as libc::c_float;
    (*para_ptr).manager_ptr = 0 as Para_M_ptr;
    if restrict_p == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            j = i + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                (*restrict_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    = 1 as libc::c_int;
                j += 1
            }
            i += 1
        }
    }
    mask_quote_scope(sp, key_pos);
    k = 0 as libc::c_int;
    while k < (*sp).Bnst_num {
        penalty_table[k as usize] =
            if k == key_pos {
                0 as libc::c_int
            } else { calc_static_level_penalty(sp, key_pos, k) };
        k += 1
    }
    if OptInput & 1 as libc::c_int != 0 {
        _detect_para_scope(sp, para_num, para_ptr,
                           (*(*sp).bnst_data.offset(key_pos as
                               isize)).dpnd_head);
    } else {
        j = key_pos + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            _detect_para_scope(sp, para_num, para_ptr, j);
            j += 1
        }
    }
    if !((*para_ptr).status as libc::c_int == 'x' as i32) {
        if (*para_ptr).status as libc::c_int == 'n' as i32 &&
            (*para_ptr).pure_score as libc::c_double >= 3.9f64 {
            (*para_ptr).status = 's' as i32 as libc::c_char
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
    /* 解析結果statusがxでも,一応TRUEを返す */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn detect_all_para_scope(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        detect_para_scope(sp, i, 0 as libc::c_int);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_para_key(mut sp: *mut SENTENCE_DATA)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: [libc::c_char; 16] = [0; 16];
    let mut condition: [libc::c_char; 256] = [0; 256];
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        cp =
            check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe4\xb8\xa6\xe3\x82\xad\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() &&
            (Language != 2 as libc::c_int ||
                Language == 2 as libc::c_int &&
                    (!check_feature((*(*sp).bnst_data.offset((i +
                        1 as
                            libc::c_int)
                        as
                        isize)).f,
                                    b"CC\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() ||
                        !check_feature((*(*sp).bnst_data.offset((i +
                            1 as
                                libc::c_int)
                            as
                            isize)).f,
                                       b"PU\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null()))
        {
            (*(*sp).bnst_data.offset(i as isize)).para_num = (*sp).Para_num;
            (*(*sp).para_data.offset((*sp).Para_num as isize)).para_char =
                ('a' as i32 + (*sp).Para_num) as libc::c_char;
            (*(*sp).para_data.offset((*sp).Para_num as isize)).key_pos = i;
            type_0[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            condition[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            sscanf(cp,
                   b"%*[^:]:%[^:]:%s\x00" as *const u8 as *const libc::c_char,
                   type_0.as_mut_ptr(), condition.as_mut_ptr());
            if strncmp(type_0.as_mut_ptr(),
                       b"\xe5\x90\x8d\x00" as *const u8 as
                           *const libc::c_char,
                       strlen(b"\xe5\x90\x8d\x00" as *const u8 as
                           *const libc::c_char)) == 0 {
                (*(*sp).bnst_data.offset(i as isize)).para_key_type =
                    1 as libc::c_int as libc::c_char
            } else if strncmp(type_0.as_mut_ptr(),
                              b"\xe8\xbf\xb0\x00" as *const u8 as
                                  *const libc::c_char,
                              strlen(b"\xe8\xbf\xb0\x00" as *const u8 as
                                  *const libc::c_char)) == 0 {
                (*(*sp).bnst_data.offset(i as isize)).para_key_type =
                    2 as libc::c_int as libc::c_char
            } else if strncmp(type_0.as_mut_ptr(),
                              b"\xef\xbc\x9f\x00" as *const u8 as
                                  *const libc::c_char,
                              strlen(b"\xef\xbc\x9f\x00" as *const u8 as
                                  *const libc::c_char)) == 0 {
                (*(*sp).bnst_data.offset(i as isize)).para_key_type =
                    4 as libc::c_int as libc::c_char
            }
            (*(*sp).para_data.offset((*sp).Para_num as isize)).type_0 =
                (*(*sp).bnst_data.offset(i as isize)).para_key_type as
                    libc::c_int;
            /* 「〜はもちろん」などの場合の"並キ:名-1:...."の処理 */
            if *type_0.as_mut_ptr().offset(3 as libc::c_int as isize) != 0 {
                (*(*sp).para_data.offset((*sp).Para_num as isize)).iend_pos =
                    i +
                        atoi(type_0.as_mut_ptr().offset(3 as libc::c_int as
                            isize))
            } else {
                (*(*sp).para_data.offset((*sp).Para_num as isize)).iend_pos =
                    i
            }
            string2feature_pattern(&mut (*(*sp).para_data.offset((*sp).Para_num
                as
                isize)).f_pattern,
                                   condition.as_mut_ptr());
            (*sp).Para_num += 1;
            if (*sp).Para_num >= 32 as libc::c_int {
                i += 1;
                while i < (*sp).Bnst_num {
                    /* 残りの文節に-1を与える */
                    (*(*sp).bnst_data.offset(i as isize)).para_num =
                        -(1 as libc::c_int);
                    i += 1
                }
                fprintf(stderr,
                        b";; Too many para (%s)!\n\x00" as *const u8 as
                            *const libc::c_char,
                        if !(*sp).Comment.is_null() {
                            (*sp).Comment as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        });
                return -(1 as libc::c_int);
            }
        } else {
            (*(*sp).bnst_data.offset(i as isize)).para_num =
                -(1 as libc::c_int)
        }
        i += 1
    }
    if (*sp).Para_num == 0 as libc::c_int { return 0 as libc::c_int; }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        cp =
            check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe5\x8c\xba\xe5\x88\x87\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                sscanf(cp,
                       b"%*[^:]:%*d-%d\x00" as *const u8 as
                           *const libc::c_char,
                       &mut (*(*sp).bnst_data.offset(i as isize)).sp_level as
                           *mut libc::c_int);
            } else {
                sscanf(cp,
                       b"%*[^:]:%d-%*d\x00" as *const u8 as
                           *const libc::c_char,
                       &mut (*(*sp).bnst_data.offset(i as isize)).sp_level as
                           *mut libc::c_int);
            }
        } else {
            (*(*sp).bnst_data.offset(i as isize)).sp_level = 0 as libc::c_int
        }
        i += 1
    }
    return (*sp).Para_num;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn farthest_child(mut sp: *mut SENTENCE_DATA,
                                        mut b_ptr: *mut BNST_DATA)
                                        -> libc::c_int
/*==================================================================*/
{
    /* 一番遠い子供の文節番号を返す
       (今のところこの関数は使っていない) */
    let mut i: libc::c_int = 0;
    let mut loop_ptr: *mut BNST_DATA = b_ptr;
    while !(*loop_ptr).child[0 as libc::c_int as usize].is_null() {
        i = 0 as libc::c_int;
        while !(*loop_ptr).child[i as usize].is_null() { i += 1 }
        loop_ptr = (*loop_ptr).child[(i - 1 as libc::c_int) as usize]
    }
    return loop_ptr.wrapping_offset_from((*sp).bnst_data) as libc::c_long as
        libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_recovery(mut sp: *mut SENTENCE_DATA)
                                       -> libc::c_int
/*==================================================================*/
{
    /* 並列構造の情報の再現 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ending_bonus_score: libc::c_int = 0 as libc::c_int;
    let mut starting_bonus_score: libc::c_int = 0 as libc::c_int;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 並列構造がfixならば再現する必要がない */
    if OptParaFix != 0 { return (0 as libc::c_int == 0) as libc::c_int; }
    (*sp).Para_num = 0 as libc::c_int;
    (*sp).Para_M_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if (*b_ptr).dpnd_type as libc::c_int == 'P' as i32 {
            if (*sp).Para_num >= 32 as libc::c_int {
                while i < (*sp).Bnst_num {
                    /* 残りの文節に-1を与える */
                    (*b_ptr).para_num = -(1 as libc::c_int); /* 不正確 */
                    i += 1;
                    b_ptr = b_ptr.offset(1)
                }
                fprintf(stderr,
                        b";; Too many para (%s)!\n\x00" as *const u8 as
                            *const libc::c_char,
                        if !(*sp).Comment.is_null() {
                            (*sp).Comment as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        });
                break;
            } else {
                (*b_ptr).para_num = (*sp).Para_num;
                (*(*sp).para_data.offset((*sp).Para_num as isize)).key_pos =
                    i;
                (*(*sp).para_data.offset((*sp).Para_num as isize)).jend_pos =
                    (*b_ptr).dpnd_head;
                (*(*sp).para_data.offset((*sp).Para_num as isize)).iend_pos =
                    i;
                j = i - 1 as libc::c_int;
                while j >= 0 as libc::c_int &&
                    ((*(*sp).bnst_data.offset(j as isize)).dpnd_head < i
                        ||
                        (*(*sp).bnst_data.offset(j as isize)).dpnd_head
                            == i &&
                            (*(*sp).bnst_data.offset(j as
                                isize)).dpnd_type
                                as libc::c_int != 'P' as i32) {
                    j -= 1
                }
                (*(*sp).para_data.offset((*sp).Para_num as
                    isize)).max_path[0 as libc::c_int
                    as usize] =
                    j + 1 as libc::c_int;
                (*(*sp).para_data.offset((*sp).Para_num as isize)).status =
                    'n' as i32 as libc::c_char;
                /* 正解入力のときはスコア計算 */
                if OptInput & 1 as libc::c_int != 0 {
                    cp =
                        check_feature((*b_ptr).f,
                                      b"\xe4\xb8\xa6\xe3\x82\xad\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char);
                    if !cp.is_null() {
                        cp =
                            cp.offset(strlen(b"\xe4\xb8\xa6\xe3\x82\xad:\x00"
                                as *const u8 as
                                *const libc::c_char) as
                                isize);
                        if strncmp(cp,
                                   b"\xe5\x90\x8d\x00" as *const u8 as
                                       *const libc::c_char,
                                   strlen(b"\xe5\x90\x8d\x00" as *const u8 as
                                       *const libc::c_char)) == 0 {
                            (*(*sp).para_data.offset((*sp).Para_num as
                                isize)).type_0 =
                                1 as libc::c_int
                        } else if strncmp(cp,
                                          b"\xe8\xbf\xb0\x00" as *const u8 as
                                              *const libc::c_char,
                                          strlen(b"\xe8\xbf\xb0\x00" as
                                              *const u8 as
                                              *const libc::c_char)) ==
                            0 {
                            (*(*sp).para_data.offset((*sp).Para_num as
                                isize)).type_0 =
                                2 as libc::c_int
                        } else {
                            (*(*sp).para_data.offset((*sp).Para_num as
                                isize)).type_0 =
                                4 as libc::c_int
                        }
                    } else {
                        (*(*sp).para_data.offset((*sp).Para_num as
                            isize)).type_0 =
                            0 as libc::c_int
                    }
                    dp_search_scope(sp,
                                    (*(*sp).para_data.offset((*sp).Para_num as
                                        isize)).key_pos,
                                    (*(*sp).para_data.offset((*sp).Para_num as
                                        isize)).iend_pos,
                                    (*(*sp).para_data.offset((*sp).Para_num as
                                        isize)).jend_pos);
                    ending_bonus_score =
                        calc_ending_bonus_score(sp,
                                                (*(*sp).para_data.offset((*sp).Para_num
                                                    as
                                                    isize)).jend_pos,
                                                &mut *(*sp).para_data.offset((*sp).Para_num
                                                    as
                                                    isize));
                    starting_bonus_score =
                        calc_starting_bonus_score(sp,
                                                  (*(*sp).para_data.offset((*sp).Para_num
                                                      as
                                                      isize)).max_path[0
                                                      as
                                                      libc::c_int
                                                      as
                                                      usize],
                                                  &mut *(*sp).para_data.offset((*sp).Para_num
                                                      as
                                                      isize));
                    (*(*sp).para_data.offset((*sp).Para_num as
                        isize)).max_score =
                        score_matrix[(*(*sp).para_data.offset((*sp).Para_num
                            as
                            isize)).max_path[0
                            as
                            libc::c_int
                            as
                            usize]
                            as
                            usize][((*(*sp).para_data.offset((*sp).Para_num
                            as
                            isize)).key_pos
                            + 1 as libc::c_int) as
                            usize] as libc::c_float /
                            norm[((*(*sp).para_data.offset((*sp).Para_num as
                                isize)).jend_pos
                                -
                                (*(*sp).para_data.offset((*sp).Para_num
                                    as
                                    isize)).max_path[0
                                    as
                                    libc::c_int
                                    as
                                    usize]
                                + 1 as libc::c_int) as usize] +
                            starting_bonus_score as libc::c_float +
                            ending_bonus_score as libc::c_float
                }
                (*sp).Para_num += 1
            }
        } else { (*b_ptr).para_num = -(1 as libc::c_int) }
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    return detect_para_relation(sp);
}
