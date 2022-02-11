#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{Chi_np_end_matrix, Chi_np_start_matrix, Chi_quote_end_matrix, Chi_quote_start_matrix, Dpnd_matrix, Mask_matrix, Para_matrix, Quote_matrix};
use crate::ctools::{check_feature, Language, stderr};
use crate::lib_print::print_bnst;
use crate::para_revision::revise_para_kakari;
use crate::structs::CDB_FILE;
use crate::tools::{OptDisplay, OptParaFix};
use crate::types::{DBM_FILE, PARA_MANAGER, SENTENCE_DATA};


#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
/*====================================================================

       並列構造内部の依存構造のチェック，依存可能性行列のマスク

                                               S.Kurohashi 93. 5.25
                                               S.Kurohashi 93. 5.31

    $Id$

====================================================================*/
#[no_mangle]
pub static mut D_check_array: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut D_found_array: [libc::c_int; 200] = [0; 200];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_stop_extend(mut sp: *mut SENTENCE_DATA,
                                           mut num: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    return if !check_feature((*(*sp).bnst_data.offset(num as isize)).f,
                             b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char).is_null()
        &&
        check_feature((*(*sp).bnst_data.offset(num as isize)).f,
                      b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe9\x80\xa3\xe4\xbd\x93\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() ||
        !check_feature((*(*sp).bnst_data.offset(num as isize)).f,
                       b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8 as
                           *const libc::c_char as
                           *mut libc::c_char).is_null() ||
        !check_feature((*(*sp).bnst_data.offset(num as isize)).f,
                       b"\xe4\xbf\x82:\xe3\x83\x87\xe6\xa0\xbc\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
            !check_feature((*(*sp).bnst_data.offset(num as isize)).f,
                           b"\xe3\x83\x8f\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null() {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_extend_p(mut sp: *mut SENTENCE_DATA,
                                       mut m_ptr: *mut PARA_MANAGER)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 並列構造前部を延長する基準 : 強並列 or 用言を含む */
    let mut i: libc::c_int = 0;
    if (*(*sp).para_data.offset((*m_ptr).para_data_num[0 as libc::c_int as
                                                           usize] as
                                    isize)).status as libc::c_int ==
           's' as i32 {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    i = (*m_ptr).start[0 as libc::c_int as usize];
    while i <= (*m_ptr).end[0 as libc::c_int as usize] {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            return (0 as libc::c_int == 0) as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn parent_range(mut m_ptr: *mut PARA_MANAGER)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 親の並列構造がある場合の範囲延長の制限
       		先頭部分に含まれる場合 : 制限なし
		それ以外に含まれる場合 : 直前のキーの位置 */
    let mut i: libc::c_int = 0;
    if (*m_ptr).parent.is_null() { return 0 as libc::c_int }
    i = (*(*m_ptr).parent).part_num - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        if (*(*m_ptr).parent).start[i as usize] <=
               (*m_ptr).start[0 as libc::c_int as usize] {
            return (*(*m_ptr).parent).start[i as usize]
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _check_para_d_struct(mut sp: *mut SENTENCE_DATA,
                                              mut str: libc::c_int,
                                              mut end: libc::c_int,
                                              mut extend_p: libc::c_int,
                                              mut limit: libc::c_int,
                                              mut s_p: *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* Iマークを埋める時の便宜 */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut success_p: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut hikousa_array: [libc::c_int; 200] = [0; 200];
    D_found_array[end as usize] = (0 as libc::c_int == 0) as libc::c_int;
    k = 0 as libc::c_int;
    while k <= end { hikousa_array[k as usize] = 1 as libc::c_int; k += 1 }
    /* 延長も調べるので,この初期化はstrからでなく0から */
    /* 並列構造内部の依存構造を調べる
       (各文節がもっとも近い文節にかかると仮定) */
    i = end - 1 as libc::c_int;
    while i >= str {
        if D_check_array[i as usize] == (0 as libc::c_int == 0) as libc::c_int
           {
            D_found_array[i as usize] = (0 as libc::c_int == 0) as libc::c_int
        } else {
            found = 0 as libc::c_int;
            j = i + 1 as libc::c_int;
            while j <= end {
                if (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                       != 0 &&
                       (*Quote_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                           != 0 &&
                       (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                             isize))[j as
                                                                         usize]
                           != 0 && hikousa_array[j as usize] != 0 {
                    D_found_array[i as usize] =
                        (0 as libc::c_int == 0) as libc::c_int;
                    k = i + 1 as libc::c_int;
                    while k < j {
                        hikousa_array[k as usize] = 0 as libc::c_int;
                        k += 1
                    }
                    found = (0 as libc::c_int == 0) as libc::c_int;
                    break ;
                } else { j += 1 }
            }
            if found == 0 as libc::c_int {
                D_found_array[i as usize] = 0 as libc::c_int;
                /* revise_para_kakariからの呼出(s_p == NULL)は表示なし */
                if OptDisplay == 3 as libc::c_int && !s_p.is_null() {
                    fprintf(stderr,
                            b";; Cannot find a head for bunsetsu <\x00" as
                                *const u8 as *const libc::c_char);
                    print_bnst((*sp).bnst_data.offset(i as isize),
                               0 as *mut libc::c_char);
                    fprintf(stderr,
                            b">.\n\x00" as *const u8 as *const libc::c_char);
                }
                success_p = 0 as libc::c_int;
                k = i + 1 as libc::c_int;
                while k <= end {
                    hikousa_array[k as usize] = 0 as libc::c_int;
                    k += 1
                }
            }
        }
        i -= 1
    }
    /* 並列構造前部の延長可能範囲を調べる */
    if extend_p == (0 as libc::c_int == 0) as libc::c_int &&
           success_p == (0 as libc::c_int == 0) as libc::c_int {
        i = str - 1 as libc::c_int;
        loop  {
            if i < limit ||
                   check_stop_extend(sp, i) ==
                       (0 as libc::c_int == 0) as libc::c_int {
                *s_p = i + 1 as libc::c_int;
                break ;
            } else {
                if D_check_array[i as usize] ==
                       (0 as libc::c_int == 0) as libc::c_int {
                    D_found_array[i as usize] =
                        (0 as libc::c_int == 0) as libc::c_int
                } else {
                    found = 0 as libc::c_int;
                    j = i + 1 as libc::c_int;
                    while j <= end {
                        /* 
		       '< end' か '<= end' かで, 並列末尾が延長する文節の
		       係り先となり得るかどうかが変わる．

		       実験の結果,'<= end'とする方が全体の精度はよい．

		       具体例) 950101071-030, 950101169-002, 950101074-019
		    */
                        if (*Mask_matrix.as_mut_ptr().offset(i as
                                                                 isize))[j as
                                                                             usize]
                               != 0 &&
                               (*Quote_matrix.as_mut_ptr().offset(i as
                                                                      isize))[j
                                                                                  as
                                                                                  usize]
                                   != 0 &&
                               (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                     isize))[j
                                                                                 as
                                                                                 usize]
                                   != 0 && hikousa_array[j as usize] != 0 {
                            D_found_array[i as usize] =
                                (0 as libc::c_int == 0) as libc::c_int;
                            k = i + 1 as libc::c_int;
                            while k < j {
                                hikousa_array[k as usize] = 0 as libc::c_int;
                                k += 1
                            }
                            found = (0 as libc::c_int == 0) as libc::c_int;
                            break ;
                            /* 96/01/22までなかなった?? */
                        } else { j += 1 }
                    }
                    if found == 0 as libc::c_int {
                        *s_p = i + 1 as libc::c_int;
                        break ;
                    }
                }
                i -= 1
            }
        }
    }
    return success_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_error_state(mut sp: *mut SENTENCE_DATA,
                                           mut m_ptr: *mut PARA_MANAGER,
                                           mut error: *mut libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* エラー状態のチェック : 

           修正可能な場合 : 2つのだけの部分からなる並列構造
			    3つ以上の部分からなる並列構造で誤りが先頭
			    3つ以上の部分からなる並列構造で誤りが末尾

	   それ以外の場合は修正断念 (return -1) */
    let mut i: libc::c_int = 0;
    return if (*m_ptr).part_num == 2 as libc::c_int {
        (*m_ptr).para_data_num[0 as libc::c_int as usize]
    } else if *error.offset(0 as libc::c_int as isize) ==
        (0 as libc::c_int == 0) as libc::c_int {
        i = 1 as libc::c_int;
        while i < (*m_ptr).part_num {
            if *error.offset(i as isize) ==
                (0 as libc::c_int == 0) as libc::c_int {
                if OptDisplay == 3 as libc::c_int {
                    fprintf(stderr,
                            b";; Cannot revise invalid kakari struct in para!!\n\x00"
                                as *const u8 as *const libc::c_char);
                }
                return -(1 as libc::c_int)
            }
            i += 1
        }
        (*m_ptr).para_data_num[0 as libc::c_int as usize]
    } else if *error.offset(((*m_ptr).part_num - 1 as libc::c_int) as isize)
        == (0 as libc::c_int == 0) as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*m_ptr).part_num - 1 as libc::c_int {
            if *error.offset(i as isize) ==
                (0 as libc::c_int == 0) as libc::c_int {
                if OptDisplay == 3 as libc::c_int {
                    fprintf(stderr,
                            b";; Cannot revise invalid kakari struct in para!!\n\x00"
                                as *const u8 as *const libc::c_char);
                }
                return -(1 as libc::c_int)
            }
            i += 1
        }
        (*m_ptr).para_data_num[((*m_ptr).para_num - 1 as libc::c_int)
            as usize]
    } else { -(1 as libc::c_int) };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_para_d_struct(mut sp: *mut SENTENCE_DATA,
                                             mut m_ptr: *mut PARA_MANAGER)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut noun_flag: libc::c_int = 0;
    let mut start_pos: libc::c_int = 0;
    let mut invalid_flag: libc::c_int = 0 as libc::c_int;
    let mut no_more_error: libc::c_int = 0;
    let mut no_more_error_here: libc::c_int = 0;
    let mut error_check: [libc::c_int; 32] = [0; 32];
    let mut error_pos: [libc::c_int; 32] = [0; 32];
    let mut revised_p_num: libc::c_int = 0;
    // let mut head_pos: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*m_ptr).child_num {
        /* 子供の再帰処理 */
        if check_para_d_struct(sp, (*m_ptr).child[i as usize]) ==
               0 as libc::c_int {
            if Language == 2 as libc::c_int {
                if i == (*m_ptr).child_num - 1 as libc::c_int {
                    return 0 as libc::c_int
                }
            } else { return 0 as libc::c_int }
        }
        i += 1
    }
    if Language == 2 as libc::c_int {
        /* if the first word in a noun coordination is verb, then change it to be verb coordination */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            k = 0 as libc::c_int;
            while k < (*m_ptr).para_num {
                if check_feature((*(*sp).bnst_data.offset((*m_ptr).start[k as
                                                                             usize]
                                                              as isize)).f,
                                 b"VV\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset((*m_ptr).start[k
                                                                                 as
                                                                                 usize]
                                                                  as
                                                                  isize)).f,
                                     b"VA\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset((*m_ptr).start[k
                                                                                 as
                                                                                 usize]
                                                                  as
                                                                  isize)).f,
                                     b"VC\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset((*m_ptr).start[k
                                                                                 as
                                                                                 usize]
                                                                  as
                                                                  isize)).f,
                                     b"VE\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                    (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as
                                                              usize] as
                                                 isize)).para_key_type =
                        1 as libc::c_int as libc::c_char;
                    break ;
                } else {
                    (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as
                                                              usize] as
                                                 isize)).para_key_type =
                        2 as libc::c_int as libc::c_char;
                    k += 1
                }
            }
        }
        /* if the last word in a noun coordination is not noun, then reduce the scope */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            k = (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int) as usize];
            while k >=
                      (*m_ptr).start[((*m_ptr).part_num - 1 as libc::c_int) as
                                         usize] {
                if !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                  b"NN\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                       !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                      b"NR\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                       !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                      b"NT\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                       !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                      b"M\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                       !check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                      b"PN\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                    (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int) as
                                     usize] = k;
                    break ;
                } else { k -= 1 }
            }
        }
        /* if the first word in a noun coordination is verb or DEC, then reduce the scope */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            k = (*m_ptr).start[0 as libc::c_int as usize];
            while k <= (*m_ptr).end[0 as libc::c_int as usize] {
                if check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                 b"VV\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                     b"VC\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                     b"VE\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                     b"VA\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                     b"P\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       check_feature((*(*sp).bnst_data.offset(k as isize)).f,
                                     b"DEC\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                    (*m_ptr).start[0 as libc::c_int as usize] = k;
                    break ;
                } else { k += 1 }
            }
        }
        /* if there is a verb in noun coordination, and there is no DEC, then reduce the coordination scope */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num {
                if exist_chi(sp, (*m_ptr).start[k as usize],
                             (*m_ptr).end[k as usize],
                             b"verb\x00" as *const u8 as *const libc::c_char)
                       != -(1 as libc::c_int) &&
                       exist_chi(sp, (*m_ptr).start[k as usize],
                                 (*m_ptr).end[k as usize],
                                 b"DEC\x00" as *const u8 as
                                     *const libc::c_char) ==
                           -(1 as libc::c_int) {
                    if k == 0 as libc::c_int {
                        (*m_ptr).start[k as usize] =
                            exist_chi(sp, (*m_ptr).start[k as usize],
                                      (*m_ptr).end[k as usize],
                                      b"verb\x00" as *const u8 as
                                          *const libc::c_char) +
                                1 as libc::c_int
                    }
                }
                k += 1
            }
        }
        /* if the first word in a noun coordination is DEG, then enlarge the scope */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            if !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                           as isize)).f,
                              b"DEG\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char).is_null() {
                if (*Chi_np_start_matrix.as_mut_ptr().offset(((*m_ptr).start[0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                  -
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 isize))[((*m_ptr).start[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                              -
                                                                              1
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             usize]
                       != -(1 as libc::c_int) {
                    (*m_ptr).start[0 as libc::c_int as usize] =
                        (*Chi_np_start_matrix.as_mut_ptr().offset(((*m_ptr).start[0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize]
                                                                       -
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      isize))[((*m_ptr).start[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize]
                                                                                   -
                                                                                   1
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  usize]
                } else if !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize]
                                                                      as
                                                                      isize).offset(-(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize))).f,
                                         b"NN\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() ||
                              !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize]
                                                                          as
                                                                          isize).offset(-(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize))).f,
                                             b"NR\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                              ||
                              !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize]
                                                                          as
                                                                          isize).offset(-(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize))).f,
                                             b"PN\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                 {
                    (*m_ptr).start[0 as libc::c_int as usize] -= 1
                } else { (*m_ptr).start[0 as libc::c_int as usize] += 1 }
            }
        }
        k = 0 as libc::c_int;
        while k < (*m_ptr).part_num {
            /* enlarge the coordination scope if there is a NR before the first NN of this coordination */
            if !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                           as isize)).f,
                              b"NN\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   (*m_ptr).start[0 as libc::c_int as usize] !=
                       0 as libc::c_int &&
                   !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                               as
                                                               isize).offset(-(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize))).f,
                                  b"NR\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                (*m_ptr).start[0 as libc::c_int as usize] -= 1
            }
            // check if the scope of coordination conflict with the scope of baseNP
            if (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).start[k as
                                                                            usize]
                                                             as
                                                             isize))[(*m_ptr).start[k
                                                                                        as
                                                                                        usize]
                                                                         as
                                                                         usize]
                   != -(1 as libc::c_int) &&
                   (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                as
                                                                                usize]
                                                                 as
                                                                 isize))[(*m_ptr).start[k
                                                                                            as
                                                                                            usize]
                                                                             as
                                                                             usize]
                       < (*m_ptr).start[k as usize] &&
                   ((*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                               as
                                                                               usize]
                                                                as
                                                                isize))[(*m_ptr).start[k
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            usize]
                        != -(1 as libc::c_int) &&
                        (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                            >= (*m_ptr).start[k as usize] &&
                        (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                            <= (*m_ptr).end[k as usize]) {
                /* enlarge the coordination scope if it is the first conjunctive structure */
                if k == 0 as libc::c_int {
                    (*m_ptr).start[k as usize] =
                        (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                     as
                                                                                     usize]
                                                                      as
                                                                      isize))[(*m_ptr).start[k
                                                                                                 as
                                                                                                 usize]
                                                                                  as
                                                                                  usize]
                }
            } else if (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                 as
                                                                                 usize]
                                                                    as
                                                                    isize))[(*m_ptr).end[k
                                                                                             as
                                                                                             usize]
                                                                                as
                                                                                usize]
                          != -(1 as libc::c_int) &&
                          (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                     as
                                                                                     usize]
                                                                        as
                                                                        isize))[(*m_ptr).end[k
                                                                                                 as
                                                                                                 usize]
                                                                                    as
                                                                                    usize]
                              >= (*m_ptr).start[k as usize] &&
                          ((*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                    as
                                                                                    usize]
                                                                       as
                                                                       isize))[(*m_ptr).end[k
                                                                                                as
                                                                                                usize]
                                                                                   as
                                                                                   usize]
                               != -(1 as libc::c_int) &&
                               (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                        as
                                                                                        usize]
                                                                           as
                                                                           isize))[(*m_ptr).end[k
                                                                                                    as
                                                                                                    usize]
                                                                                       as
                                                                                       usize]
                                   > (*m_ptr).end[k as usize]) {
                /* enlarge the coordination scope if it is the last conjunctive structure */
                if k == (*m_ptr).part_num - 1 as libc::c_int {
                    (*m_ptr).end[k as usize] =
                        (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                 as
                                                                                 usize]
                                                                    as
                                                                    isize))[(*m_ptr).end[k
                                                                                             as
                                                                                             usize]
                                                                                as
                                                                                usize]
                }
            }
            // check if the scope of coordination conflict with the scope of quote
            if (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                               as
                                                                               usize]
                                                                as
                                                                isize))[(*m_ptr).start[k
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            usize]
                   != -(1 as libc::c_int) &&
                   (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                       < (*m_ptr).start[k as usize] &&
                   ((*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                  as
                                                                                  usize]
                                                                   as
                                                                   isize))[(*m_ptr).start[k
                                                                                              as
                                                                                              usize]
                                                                               as
                                                                               usize]
                        != -(1 as libc::c_int) &&
                        (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                      as
                                                                                      usize]
                                                                       as
                                                                       isize))[(*m_ptr).start[k
                                                                                                  as
                                                                                                  usize]
                                                                                   as
                                                                                   usize]
                            >= (*m_ptr).start[k as usize] &&
                        (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                      as
                                                                                      usize]
                                                                       as
                                                                       isize))[(*m_ptr).start[k
                                                                                                  as
                                                                                                  usize]
                                                                                   as
                                                                                   usize]
                            <= (*m_ptr).end[k as usize]) {
                /* enlarge the coordination scope if it is the first conjunctive structure */
                if k == 0 as libc::c_int {
                    (*m_ptr).start[k as usize] =
                        (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                        as
                                                                                        usize]
                                                                         as
                                                                         isize))[(*m_ptr).start[k
                                                                                                    as
                                                                                                    usize]
                                                                                     as
                                                                                     usize]
                }
            } else if (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                    as
                                                                                    usize]
                                                                       as
                                                                       isize))[(*m_ptr).end[k
                                                                                                as
                                                                                                usize]
                                                                                   as
                                                                                   usize]
                          != -(1 as libc::c_int) &&
                          (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                        as
                                                                                        usize]
                                                                           as
                                                                           isize))[(*m_ptr).end[k
                                                                                                    as
                                                                                                    usize]
                                                                                       as
                                                                                       usize]
                              >= (*m_ptr).start[k as usize] &&
                          ((*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                       as
                                                                                       usize]
                                                                          as
                                                                          isize))[(*m_ptr).end[k
                                                                                                   as
                                                                                                   usize]
                                                                                      as
                                                                                      usize]
                               != -(1 as libc::c_int) &&
                               (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                           as
                                                                                           usize]
                                                                              as
                                                                              isize))[(*m_ptr).end[k
                                                                                                       as
                                                                                                       usize]
                                                                                          as
                                                                                          usize]
                                   > (*m_ptr).end[k as usize]) {
                /* enlarge the coordination scope if it is the last conjunctive structure */
                if k == (*m_ptr).part_num - 1 as libc::c_int {
                    (*m_ptr).end[k as usize] =
                        (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                    as
                                                                                    usize]
                                                                       as
                                                                       isize))[(*m_ptr).end[k
                                                                                                as
                                                                                                usize]
                                                                                   as
                                                                                   usize]
                }
            }
            k += 1
        }
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 2 as libc::c_int {
            /* change the scope of verb coordination for Chinese, i.e. change [V][CC V] to be [V CC][V] */
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num - 1 as libc::c_int {
                (*(*sp).bnst_data.offset((*m_ptr).start[(k + 1 as libc::c_int)
                                                            as usize] as
                                             isize)).para_num =
                    (*(*sp).bnst_data.offset((*m_ptr).end[k as usize] as
                                                 isize)).para_num;
                (*(*sp).bnst_data.offset((*m_ptr).end[k as usize] as
                                             isize)).para_num =
                    -(1 as libc::c_int);
                (*m_ptr).end[k as usize] =
                    (*m_ptr).start[(k + 1 as libc::c_int) as usize];
                (*m_ptr).start[(k + 1 as libc::c_int) as usize] += 1;
                k += 1
            }
        }
    }
    /* 体言文節の働きが曖昧なものが，並列構造解析で明確になる場合の処理
       ----------------------------------------------------------------
       例) 「風間さんは雑誌の編集者、恵美子さんは車メーカーのコンパニオン。」
       		「編集者」が判定詞省略であることがわかる

       例) 「発電能力で約四十倍、電力量では約六十倍も増やし、…」
           「今年は四月に統一地方選挙、七月に参院選挙があります。」
       		「約四十倍」,「統一地方選挙」がサ変省略でないことがわかる
       
       → 「a1 a2 a3, b1 b2 b3」において，すべて体言の場合，(a1 a3),(a2 a3)
       の係り受けを(a1 b3),(a2 b3)の係り受けで上書きする．
       ただし，隣にだけ係るような係り受けの場合に副作用があるので，(a2 a3)
       の上書きは，(a2 a3)と(a2 b3)が異なる場合のみとする．

       例) 「映画は二月クランクイン、十月公開の予定。」


       ※ この方法では前のconjunctの係り受けを修正することしかできず，
       次の例は正しく扱えない．

       例) 「前回は７戦だったが、今回は九番勝負で先に５勝した…」

       本来的には,新たに正しいfeatureを与えて係り受けの解析をやり直す
       必要がある．その場合には，
	・片方が判定詞 → もう一方も判定詞
       	・片方がサ変 → サ変を取り消す
       という処理を行えばよいだろう．そうすれば上の例も正しく解析される
       ようになる．
    */
    if (*m_ptr).status as libc::c_int == 's' as i32 {
        noun_flag = 1 as libc::c_int;
        k = 0 as libc::c_int;
        while k < (*m_ptr).part_num {
            if check_feature((*(*sp).bnst_data.offset((*m_ptr).end[k as usize]
                                                          as isize)).f,
                             b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() {
                noun_flag = 0 as libc::c_int
            }
            k += 1
        }
        if noun_flag != 0 {
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num - 1 as libc::c_int {
                i = (*m_ptr).start[k as usize];
                while i < (*m_ptr).end[k as usize] {
                    if !(i == (*m_ptr).end[k as usize] - 1 as libc::c_int &&
                             (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                   isize))[(*m_ptr).end[k
                                                                                            as
                                                                                            usize]
                                                                               as
                                                                               usize]
                                 ==
                                 (*Dpnd_matrix.as_mut_ptr().offset(((*m_ptr).end[((*m_ptr).part_num
                                                                                      -
                                                                                      1
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     usize]
                                                                        -
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       isize))[(*m_ptr).end[((*m_ptr).part_num
                                                                                                 -
                                                                                                 1
                                                                                                     as
                                                                                                     libc::c_int)
                                                                                                as
                                                                                                usize]
                                                                                   as
                                                                                   usize])
                       {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[(*m_ptr).end[k
                                                                                       as
                                                                                       usize]
                                                                          as
                                                                          usize]
                            =
                            (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                  isize))[(*m_ptr).end[((*m_ptr).part_num
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           usize]
                                                                              as
                                                                              usize]
                    }
                    i += 1
                }
                k += 1
            }
        }
    }
    /* 依存構造解析可能性のチェック */
    start_pos = (*m_ptr).start[0 as libc::c_int as usize];
    k = 0 as libc::c_int;
    while k < (*m_ptr).part_num {
        if _check_para_d_struct(sp, (*m_ptr).start[k as usize],
                                (*m_ptr).end[k as usize],
                                (if k == 0 as libc::c_int {
                                     para_extend_p(sp, m_ptr)
                                 } else { 0 as libc::c_int }),
                                (if k == 0 as libc::c_int {
                                     parent_range(m_ptr)
                                 } else { 0 as libc::c_int }), &mut start_pos)
               == 0 as libc::c_int {
            invalid_flag = (0 as libc::c_int == 0) as libc::c_int;
            error_check[k as usize] = (0 as libc::c_int == 0) as libc::c_int
        } else {
            error_check[k as usize] = 0 as libc::c_int
            /* 初期化 */
        }
        k += 1
    }
    /* 依存構造解析に失敗した場合

       「彼は東京で八百屋を,彼女は大阪で主婦を,私は京都で学生をしている」
       のように述語の省略された含む並列構造を検出する．

       各部分の係り先のない文節の数が同じで,強並列であれば
       上記のタイプの並列構造と考える．
       (係り先のない文節のタイプ(ガ格など)は,厳密に対応するとは限らない
       ので制限しない)

       アルゴリズム : 
       先頭部分の各係り先のない文節について
	       	各部分に係先のないそれと同じタイプの文節があるかどうか調べる
       */
    if invalid_flag == (0 as libc::c_int == 0) as libc::c_int {
        /* 要検討 */
        /* if (sp->para_data[m_ptr->para_data_num[0]].pure_score < PARA_INCOMP_TH) { */
        if (*m_ptr).status as libc::c_int != 's' as i32 {
            revised_p_num =
                check_error_state(sp, m_ptr, error_check.as_mut_ptr());
            if revised_p_num != -(1 as libc::c_int) {
                revise_para_kakari(sp, revised_p_num,
                                   D_found_array.as_mut_ptr());
                return 0 as libc::c_int
            }
        } else {
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num {
                error_pos[k as usize] = (*m_ptr).end[k as usize];
                k += 1
            }
            's_661:
                loop  {
                    no_more_error = 0 as libc::c_int;
                    no_more_error_here = 0 as libc::c_int;
                    i =
                        error_pos[0 as libc::c_int as usize] -
                            1 as libc::c_int;
                    while D_found_array[i as usize] ==
                              (0 as libc::c_int == 0) as libc::c_int &&
                              start_pos <= i {
                        i -= 1
                    }
                    error_pos[0 as libc::c_int as usize] = i;
                    if i == start_pos - 1 as libc::c_int {
                        no_more_error = (0 as libc::c_int == 0) as libc::c_int
                    }
                    k = 1 as libc::c_int;
                    while k < (*m_ptr).part_num {
                        i = error_pos[k as usize] - 1 as libc::c_int;
                        while D_found_array[i as usize] ==
                                  (0 as libc::c_int == 0) as libc::c_int &&
                                  (*m_ptr).start[k as usize] <= i {
                            i -= 1
                        }
                        error_pos[k as usize] = i;
                        if i == (*m_ptr).start[k as usize] - 1 as libc::c_int
                           {
                            no_more_error_here =
                                (0 as libc::c_int == 0) as libc::c_int
                        }
                        /* エラーの対応がつかない(部分並列でない) */
                        if no_more_error != no_more_error_here {
                            revised_p_num =
                                check_error_state(sp, m_ptr,
                                                  error_check.as_mut_ptr());
                            if !(revised_p_num != -(1 as libc::c_int)) {
                                break 's_661 ;
                            }
                            revise_para_kakari(sp, revised_p_num,
                                               D_found_array.as_mut_ptr());
                            return 0 as libc::c_int
                        } else { k += 1 }
                    }
                    if no_more_error == (0 as libc::c_int == 0) as libc::c_int
                       {
                        break ;
                    }
                }
        }
    }
    /* チェック済みの印 */
    k = start_pos;
    while k < (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int) as usize] {
        D_check_array[k as usize] = (0 as libc::c_int == 0) as libc::c_int;
        k += 1
    }
    if Language != 2 as libc::c_int {
        /* 先頭のconjunctのマスク */
        k = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < start_pos {
            /* < start_pos */
            j = (*m_ptr).start[k as usize];
            while j <= (*m_ptr).end[k as usize] {
                (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int;
                j += 1
            }
            i += 1
        }
        /* ★★ 実験 endの上のカバーしない
	   for (i = start_pos; i < m_ptr->start[k]; i++)       end の上
	   Mask_matrix[i][m_ptr->end[k]] = 0;
	*/
        i = (*m_ptr).start[k as usize];
        while i <= (*m_ptr).end[k as usize] {
            j = (*m_ptr).end[k as usize] + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int;
                j += 1
            }
            i += 1
        }
        if (*(*sp).para_data.offset((*m_ptr).para_data_num[0 as libc::c_int as
                                                               usize] as
                                        isize)).status as libc::c_int ==
               's' as i32 {
            /* 強並列 ??? */
            i = 0 as libc::c_int;
            while i < (*m_ptr).start[0 as libc::c_int as usize] {
                (*Mask_matrix.as_mut_ptr().offset(i as
                                                      isize))[(*m_ptr).end[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                                  as usize] =
                    0 as libc::c_int;
                i += 1
            }
        }
        /* 内部のconjunctのマスク */
        k = 1 as libc::c_int;
        while k < (*m_ptr).part_num - 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*m_ptr).start[k as usize] {
                j = (*m_ptr).start[k as usize];
                while j <= (*m_ptr).end[k as usize] {
                    (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                i += 1
            }
            i = (*m_ptr).start[k as usize];
            while i <= (*m_ptr).end[k as usize] {
                j = (*m_ptr).end[k as usize] + 1 as libc::c_int;
                while j < (*sp).Bnst_num {
                    (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                i += 1
            }
            k += 1
        }
        /* 末尾のconjunctのマスク */
        k = (*m_ptr).part_num - 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*m_ptr).start[k as usize] {
            j = (*m_ptr).start[k as usize];
            while j < (*m_ptr).end[k as usize] {
                /* < end */
                (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int;
                j += 1
            }
            i += 1
        }
        i = (*m_ptr).start[k as usize];
        while i < (*m_ptr).end[k as usize] {
            /* < end */
            j = (*m_ptr).end[k as usize] + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int;
                j += 1
            }
            i += 1
        }
        /* 並列の係り先 */
        k = 0 as libc::c_int;
        while k < (*m_ptr).part_num - 1 as libc::c_int {
            (*Mask_matrix.as_mut_ptr().offset((*m_ptr).end[k as usize] as
                                                  isize))[(*m_ptr).end[(k +
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           usize]
                                                              as usize] =
                2 as libc::c_int;
            k += 1
            /*
	      Mask_matrix[m_ptr->end[k]][m_ptr->end[m_ptr->part_num - 1]] = 2;
	    */
        }
        if invalid_flag == (0 as libc::c_int == 0) as libc::c_int {
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num {
                i = (*m_ptr).start[k as usize];
                while i <= (*m_ptr).end[k as usize] {
                    if D_found_array[i as usize] == 0 as libc::c_int {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[(*m_ptr).end[k
                                                                                       as
                                                                                       usize]
                                                                          as
                                                                          usize]
                            = 3 as libc::c_int;
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[(*m_ptr).end[((*m_ptr).part_num
                                                                                        -
                                                                                        1
                                                                                            as
                                                                                            libc::c_int)
                                                                                       as
                                                                                       usize]
                                                                          as
                                                                          usize]
                            = 3 as libc::c_int
                    }
                    i += 1
                }
                k += 1
            }
        }
    }
    /* 部分並列の場合,Mask_matrixは最初のheadと最後のheadを3にしておく．
       最初のheadはdpnd.headをつくるとき，最後のheadはtreeを作る時に使う */
    if Language == 2 as libc::c_int {
        /* if the first word in a noun coordination is P, and the following word is not noun, then remove this coordination */
        if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as usize] as
                                        isize)).para_key_type as libc::c_int
               == 1 as libc::c_int {
            if !check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                           as isize)).f,
                              b"P\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                              as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).f,
                                 b"NN\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                   check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                              as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).f,
                                 b"NR\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                   check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                              as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).f,
                                 b"NT\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                   check_feature((*(*sp).bnst_data.offset((*m_ptr).start[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                              as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).f,
                                 b"PN\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() {
                return 0 as libc::c_int
            }
        }
        k = 0 as libc::c_int;
        while k < (*m_ptr).part_num {
            // check if the scope of coordination conflict with the scope of baseNP
            if (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).start[k as
                                                                            usize]
                                                             as
                                                             isize))[(*m_ptr).start[k
                                                                                        as
                                                                                        usize]
                                                                         as
                                                                         usize]
                   != -(1 as libc::c_int) &&
                   (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                as
                                                                                usize]
                                                                 as
                                                                 isize))[(*m_ptr).start[k
                                                                                            as
                                                                                            usize]
                                                                             as
                                                                             usize]
                       < (*m_ptr).start[k as usize] &&
                   ((*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                               as
                                                                               usize]
                                                                as
                                                                isize))[(*m_ptr).start[k
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            usize]
                        != -(1 as libc::c_int) &&
                        (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                            >= (*m_ptr).start[k as usize] &&
                        (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                            <= (*m_ptr).end[k as usize]) {
                return 0 as libc::c_int
            } else {
                if (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).end[k as
                                                                              usize]
                                                                 as
                                                                 isize))[(*m_ptr).end[k
                                                                                          as
                                                                                          usize]
                                                                             as
                                                                             usize]
                       != -(1 as libc::c_int) &&
                       (*Chi_np_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                  as
                                                                                  usize]
                                                                     as
                                                                     isize))[(*m_ptr).end[k
                                                                                              as
                                                                                              usize]
                                                                                 as
                                                                                 usize]
                           >= (*m_ptr).start[k as usize] &&
                       ((*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                 as
                                                                                 usize]
                                                                    as
                                                                    isize))[(*m_ptr).end[k
                                                                                             as
                                                                                             usize]
                                                                                as
                                                                                usize]
                            != -(1 as libc::c_int) &&
                            (*Chi_np_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                     as
                                                                                     usize]
                                                                        as
                                                                        isize))[(*m_ptr).end[k
                                                                                                 as
                                                                                                 usize]
                                                                                    as
                                                                                    usize]
                                > (*m_ptr).end[k as usize]) {
                    return 0 as libc::c_int
                }
            }
            // check if the scope of coordination conflict with the scope of quote
            if (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                               as
                                                                               usize]
                                                                as
                                                                isize))[(*m_ptr).start[k
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            usize]
                   != -(1 as libc::c_int) &&
                   (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                   as
                                                                                   usize]
                                                                    as
                                                                    isize))[(*m_ptr).start[k
                                                                                               as
                                                                                               usize]
                                                                                as
                                                                                usize]
                       < (*m_ptr).start[k as usize] &&
                   ((*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                  as
                                                                                  usize]
                                                                   as
                                                                   isize))[(*m_ptr).start[k
                                                                                              as
                                                                                              usize]
                                                                               as
                                                                               usize]
                        != -(1 as libc::c_int) &&
                        (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                      as
                                                                                      usize]
                                                                       as
                                                                       isize))[(*m_ptr).start[k
                                                                                                  as
                                                                                                  usize]
                                                                                   as
                                                                                   usize]
                            >= (*m_ptr).start[k as usize] &&
                        (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).start[k
                                                                                      as
                                                                                      usize]
                                                                       as
                                                                       isize))[(*m_ptr).start[k
                                                                                                  as
                                                                                                  usize]
                                                                                   as
                                                                                   usize]
                            <= (*m_ptr).end[k as usize]) {
                return 0 as libc::c_int
            } else {
                if (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                 as
                                                                                 usize]
                                                                    as
                                                                    isize))[(*m_ptr).end[k
                                                                                             as
                                                                                             usize]
                                                                                as
                                                                                usize]
                       != -(1 as libc::c_int) &&
                       (*Chi_quote_start_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                     as
                                                                                     usize]
                                                                        as
                                                                        isize))[(*m_ptr).end[k
                                                                                                 as
                                                                                                 usize]
                                                                                    as
                                                                                    usize]
                           >= (*m_ptr).start[k as usize] &&
                       ((*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                    as
                                                                                    usize]
                                                                       as
                                                                       isize))[(*m_ptr).end[k
                                                                                                as
                                                                                                usize]
                                                                                   as
                                                                                   usize]
                            != -(1 as libc::c_int) &&
                            (*Chi_quote_end_matrix.as_mut_ptr().offset((*m_ptr).end[k
                                                                                        as
                                                                                        usize]
                                                                           as
                                                                           isize))[(*m_ptr).end[k
                                                                                                    as
                                                                                                    usize]
                                                                                       as
                                                                                       usize]
                                > (*m_ptr).end[k as usize]) {
                    return 0 as libc::c_int
                }
            }
            k += 1
        }
        if (*(*sp).bnst_data.offset(((*m_ptr).end[0 as libc::c_int as usize] -
                                         1 as libc::c_int) as
                                        isize)).para_key_type as libc::c_int
               == 2 as libc::c_int {
            if OptParaFix != 0 {
                // for verb coordination, mask the outside area for the non-first conjunction
                i = 0 as libc::c_int; // verb coordination
                while i < (*m_ptr).start[0 as libc::c_int as usize] {
                    j =
                        (*m_ptr).start[1 as libc::c_int as usize] +
                            1 as libc::c_int;
                    while j <=
                              (*m_ptr).end[((*m_ptr).part_num -
                                                1 as libc::c_int) as usize] {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 0 as libc::c_int;
                        j += 1
                    }
                    i += 1
                }
                i =
                    (*m_ptr).start[1 as libc::c_int as usize] +
                        1 as libc::c_int;
                while i <=
                          (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int)
                                           as usize] {
                    j =
                        (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int) as
                                         usize] + 1 as libc::c_int;
                    while j < (*sp).Bnst_num {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 0 as libc::c_int;
                        j += 1
                    }
                    i += 1
                }
            }
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num {
                (*Mask_matrix.as_mut_ptr().offset((*m_ptr).start[k as usize]
                                                      as
                                                      isize))[(*m_ptr).end[k
                                                                               as
                                                                               usize]
                                                                  as usize] =
                    'V' as i32;
                if k < (*m_ptr).part_num - 1 as libc::c_int {
                    i = (*m_ptr).start[k as usize] + 1 as libc::c_int;
                    while i <
                              (*m_ptr).end[((*m_ptr).part_num -
                                                2 as libc::c_int) as usize] {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[(*m_ptr).end[k
                                                                                       as
                                                                                       usize]
                                                                          as
                                                                          usize]
                            = 0 as libc::c_int;
                        i += 1
                    }
                }
                k += 1
            }
        } else if (*(*sp).bnst_data.offset((*m_ptr).end[0 as libc::c_int as
                                                            usize] as
                                               isize)).para_key_type as
                      libc::c_int == 1 as libc::c_int {
            if OptParaFix != 0 {
                // for noun coordination, mask the outside area for the non-last conjunction
                i = 0 as libc::c_int; // noun coordination
                while i < (*m_ptr).start[0 as libc::c_int as usize] {
                    j = (*m_ptr).start[0 as libc::c_int as usize];
                    while j <
                              (*m_ptr).start[((*m_ptr).part_num -
                                                  1 as libc::c_int) as usize]
                          {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 0 as libc::c_int;
                        j += 1
                    }
                    i += 1
                }
                i = (*m_ptr).start[0 as libc::c_int as usize];
                while i <
                          (*m_ptr).start[((*m_ptr).part_num -
                                              1 as libc::c_int) as usize] {
                    j =
                        (*m_ptr).end[((*m_ptr).part_num - 1 as libc::c_int) as
                                         usize] + 1 as libc::c_int;
                    while j < (*sp).Bnst_num {
                        (*Mask_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 0 as libc::c_int;
                        j += 1
                    }
                    i += 1
                }
            }
            k = 0 as libc::c_int;
            while k < (*m_ptr).part_num {
                (*Mask_matrix.as_mut_ptr().offset((*m_ptr).start[k as usize]
                                                      as
                                                      isize))[(*m_ptr).end[k
                                                                               as
                                                                               usize]
                                                                  as usize] =
                    'N' as i32;
                if k > 0 as libc::c_int {
                    i = (*m_ptr).start[k as usize] + 1 as libc::c_int;
                    while i <
                              (*m_ptr).end[((*m_ptr).part_num -
                                                1 as libc::c_int) as usize] {
                        (*Mask_matrix.as_mut_ptr().offset((*m_ptr).start[k as
                                                                             usize]
                                                              as
                                                              isize))[i as
                                                                          usize]
                            = 0 as libc::c_int;
                        i += 1
                    }
                }
                k += 1
            }
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_mask_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                1 as libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_para_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < (*sp).Para_num {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            j = 0 as libc::c_int;
            while j < (*sp).Bnst_num {
                (*Para_matrix.as_mut_ptr().offset(k as
                                                      isize))[i as
                                                                  usize][j as
                                                                             usize]
                    = -(1 as libc::c_int) as libc::c_double;
                j += 1
            }
            i += 1
        }
        k += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_dpnd_in_para(mut sp: *mut SENTENCE_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    /* 初期化 */
    init_mask_matrix(sp);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        D_check_array[i as usize] = 0 as libc::c_int;
        i += 1
    }
    /* 並列構造内の係受けチェック，マスク */
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        if (*(*sp).para_manager.offset(i as isize)).parent.is_null() {
            if check_para_d_struct(sp,
                                   &mut *(*sp).para_manager.offset(i as
                                                                       isize))
                   == 0 as libc::c_int {
                if Language == 2 as libc::c_int {
                    if i == (*sp).Para_M_num - 1 as libc::c_int {
                        return 0 as libc::c_int
                    }
                } else { return 0 as libc::c_int }
            }
        }
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*====================================================================
                               END
====================================================================*/
