#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]


use crate::ctools::Outfp;
use crate::lib_print::print_matrix;
use crate::{restrict_matrix, Revised_para_num};
use crate::para_dpnd::{_check_para_d_struct, D_found_array};
use crate::structs::CDB_FILE;
use crate::tools::OptDisplay;
use crate::types::{DBM_FILE, PARA_DATA, SENTENCE_DATA};

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
static mut judge_matrix: [[libc::c_int; 4]; 4] =
    [[1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
      0 as libc::c_int]];
static mut judge_matrix_pos_str: [[libc::c_int; 4]; 4] =
    [[0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
      0 as libc::c_int]];
static mut judge_matrix_pre_str: [[libc::c_int; 4]; 4] =
    [[0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
     [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
     [1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
      0 as libc::c_int]];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_restrict_matrix(mut sp: *mut SENTENCE_DATA, mut key_pos: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(Outfp, b"<< restrict matrix >>\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i <= key_pos {
        j = key_pos + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            fprintf(Outfp, b"%3d\x00" as *const u8 as *const libc::c_char, (*restrict_matrix.as_mut_ptr().offset(i as isize))[j as usize]);
            j += 1
        }
        fputc('\n' as i32, Outfp);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_restrict_matrix(mut sp: *mut SENTENCE_DATA, mut a1: libc::c_int, mut a2: libc::c_int, mut a3: libc::c_int, mut b1: libc::c_int, mut b2: libc::c_int, mut b3: libc::c_int, mut flag: libc::c_int) {
    /* 制限行列の処理 */
    let mut rel_pre: libc::c_int = 0;
    let mut rel_pos: libc::c_int = 0;
    match flag {
        1 | 3 => {
            a1 = 0 as libc::c_int;
            while a1 <= a2 {
                if (a2 + 1 as libc::c_int) < b1 {
                    rel_pre = 0 as libc::c_int
                } else if a2 + 1 as libc::c_int == b1 {
                    rel_pre = 1 as libc::c_int
                } else if a1 < b1 {
                    rel_pre = 2 as libc::c_int
                } else {
                    rel_pre = 3 as libc::c_int
                }
                a3 = a2 + 1 as libc::c_int;
                while a3 < (*sp).Bnst_num {
                    if a3 < b1 {
                        /* 重複しない */
                        (*restrict_matrix.as_mut_ptr().offset(a1 as isize))[a3 as usize] = 1 as libc::c_int
                    } else {
                        if a3 < b2 {
                            rel_pos = 0 as libc::c_int
                        } else if a3 == b2 {
                            rel_pos = 1 as libc::c_int
                        } else if a3 < b3 {
                            rel_pos = 2 as libc::c_int
                        } else { rel_pos = 3 as libc::c_int }
                        if flag == 1 as libc::c_int {
                            (*restrict_matrix.as_mut_ptr().offset(a1 as isize))[a3 as usize] = judge_matrix_pos_str[rel_pre as usize][rel_pos as usize]
                        } else {
                            (*restrict_matrix.as_mut_ptr().offset(a1 as isize))[a3 as usize] = judge_matrix[rel_pre as usize][rel_pos as usize]
                        }
                    }
                    a3 += 1
                }
                a1 += 1
            }
        }
        2 | 4 => {
            b1 = 0 as libc::c_int;
            while b1 <= b2 {
                if a3 < b1 {
                    /* 重複しない */
                    b3 = b2 + 1 as libc::c_int;
                    while b3 < (*sp).Bnst_num {
                        (*restrict_matrix.as_mut_ptr().offset(b1 as isize))[b3 as usize] = 1 as libc::c_int;
                        b3 += 1
                    }
                } else {
                    if (a2 + 1 as libc::c_int) < b1 {
                        rel_pre = 0 as libc::c_int
                    } else if a2 + 1 as libc::c_int == b1 {
                        rel_pre = 1 as libc::c_int
                    } else if a1 < b1 {
                        rel_pre = 2 as libc::c_int
                    } else { rel_pre = 3 as libc::c_int }
                    b3 = b2 + 1 as libc::c_int;
                    while b3 < (*sp).Bnst_num {
                        if a3 < b2 {
                            rel_pos = 0 as libc::c_int
                        } else if a3 == b2 {
                            rel_pos = 1 as libc::c_int
                        } else if a3 < b3 {
                            rel_pos = 2 as libc::c_int
                        } else {
                            rel_pos = 3 as libc::c_int
                        }
                        if flag == 2 as libc::c_int {
                            (*restrict_matrix.as_mut_ptr().offset(b1 as isize))[b3 as usize] = judge_matrix_pre_str[rel_pre as usize][rel_pos as usize]
                        } else {
                            (*restrict_matrix.as_mut_ptr().offset(b1 as isize))[b3 as usize] = judge_matrix[rel_pre as usize][rel_pos as usize]
                        }
                        b3 += 1
                    }
                }
                b1 += 1
            }
        }
        _ => { }
    }
    /* 出力 */
    if OptDisplay == 3 as libc::c_int {
        if flag == 1 as libc::c_int || flag == 3 as libc::c_int {
            print_matrix(sp, 4 as libc::c_int, a2);
        } else if flag == 2 as libc::c_int || flag == 4 as libc::c_int {
            print_matrix(sp, 4 as libc::c_int, b2);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn revise_para_rel(mut sp: *mut SENTENCE_DATA, mut pre: libc::c_int, mut pos: libc::c_int) {
    /* 並列構造間の関係による修正 */
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    let mut a3: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut b3: libc::c_int = 0;
    let mut ptr1: *mut PARA_DATA = 0 as *mut PARA_DATA;
    let mut ptr2: *mut PARA_DATA = 0 as *mut PARA_DATA;
    ptr1 = &mut *(*sp).para_data.offset(pre as isize) as *mut PARA_DATA;
    ptr2 = &mut *(*sp).para_data.offset(pos as isize) as *mut PARA_DATA;
    a1 = (*ptr1).max_path[0 as libc::c_int as usize];
    a2 = (*ptr1).key_pos;
    a3 = (*ptr1).jend_pos;
    b1 = (*ptr2).max_path[0 as libc::c_int as usize];
    b2 = (*ptr2).key_pos;
    b3 = (*ptr2).jend_pos;
    /* 後だけ強並列 -> 前を修正 */
    if (*ptr1).status as libc::c_int != 's' as i32 &&
           (*ptr2).status as libc::c_int == 's' as i32 {
        set_restrict_matrix(sp, a1, a2, a3, b1, b2, b3, 1 as libc::c_int);
        Revised_para_num = pre
    } else if (*ptr1).status as libc::c_int == 's' as i32 &&
                  (*ptr2).status as libc::c_int != 's' as i32 {
        set_restrict_matrix(sp, a1, a2, a3, b1, b2, b3, 2 as libc::c_int);
        Revised_para_num = pos
    } else if (*ptr1).max_score <= (*ptr2).max_score {
        set_restrict_matrix(sp, a1, a2, a3, b1, b2, b3, 3 as libc::c_int);
        Revised_para_num = pre
    } else {
        /* 前だけ強並列 -> 後を修正 */
        /* スコア比較 -> 前を修正 */
        /* スコア比較 -> 後を修正 */
        set_restrict_matrix(sp, a1, a2, a3, b1, b2, b3, 4 as libc::c_int);
        Revised_para_num = pos
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn revise_para_kakari(mut sp: *mut SENTENCE_DATA, mut num: libc::c_int, mut array: *mut libc::c_int) {
    /* 係り受け誤りによる修正 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: *mut PARA_DATA = (*sp).para_data.offset(num as isize);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*restrict_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                1 as libc::c_int;
            j += 1
        }
        i += 1
    }
    /* 前部の制限 */
    if _check_para_d_struct(sp, 0 as libc::c_int, (*ptr).key_pos,
                            0 as libc::c_int, 0 as libc::c_int,
                            0 as *mut libc::c_int) == 0 as libc::c_int {
        k = (*ptr).key_pos;
        while D_found_array[k as usize] == (0 as libc::c_int == 0) as libc::c_int {
            k -= 1
        }
        i = 0 as libc::c_int;
        while i <= k {
            j = (*ptr).key_pos + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                (*restrict_matrix.as_mut_ptr().offset(i as isize))[j as usize] = 0 as libc::c_int;
                j += 1
            }
            i += 1
        }
    }
    /* 後部の制限 */
    j = (*ptr).key_pos + 2 as libc::c_int;
    while j < (*sp).Bnst_num {
        if _check_para_d_struct(sp, (*ptr).key_pos + 1 as libc::c_int, j,
                                0 as libc::c_int, 0 as libc::c_int,
                                0 as *mut libc::c_int) == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i <= (*ptr).key_pos {
                (*restrict_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    = 0 as libc::c_int;
                i += 1
            }
        }
        j += 1
    }
    Revised_para_num = num;
    if OptDisplay == 3 as libc::c_int {
        print_matrix(sp, 5 as libc::c_int, (*ptr).key_pos);
    };
}
/*====================================================================
                               END
====================================================================*/
