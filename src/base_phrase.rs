#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]


use crate::{Chi_np_end_matrix, Chi_np_start_matrix, Chi_quote_end_matrix, Chi_quote_start_matrix, ctools, Dpnd_matrix, tools};
use crate::ctools::check_feature;
use crate::structs::CDB_FILE;
use crate::tools::{Chi_root, OptDisplay};
use crate::types::{DBM_FILE, SENTENCE_DATA};


#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut np_matrix: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut pp_matrix: [libc::c_int; 200] = [0; 200];

#[no_mangle]
pub unsafe extern "C" fn init_phrase(mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        np_matrix[i as usize] = -(1 as libc::c_int);
        pp_matrix[i as usize] = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Chi_np_start_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                = -(1 as libc::c_int);
            (*Chi_np_end_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                -(1 as libc::c_int);
            j += 1
        }
        i += 1
    }
    Chi_root = -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn check_phrase(mut sp: *mut SENTENCE_DATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut start_np: libc::c_int = 0;
    let mut end_np: libc::c_int = 0;
    flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        /* assign root feature */
        if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"IS_ROOT\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"CC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"DEG\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"DEC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"DER\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"DEV\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"DT\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"ETC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"FW\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"IJ\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"LC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"MSP\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
            ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"PU\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
            Chi_root = i
        }
        /* assign NP feature */
        if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"NP_B\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            j = i + 1 as libc::c_int;
            while j < (*sp).Bnst_num &&
                ctools::check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                    b"NP_B\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() &&
                ctools::check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                    b"NP_O\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() {
                j += 1
            }
            np_matrix[i as usize] = j - 1 as libc::c_int;
            i = j - 1 as libc::c_int;
            flag = (0 as libc::c_int == 0) as libc::c_int
        }
        i += 1
    }
    /* fix baseNP scope error */
    start_np = -(1 as libc::c_int);
    end_np = -(1 as libc::c_int);
    flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if np_matrix[i as usize] != -(1 as libc::c_int) {
            /* if the last word of baseNP is not noun, then reduce baseNP scope */
            j = np_matrix[i as usize];
            while j >= i {
                if !ctools::check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                  b"NN\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                       !ctools::check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                      b"NR\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                       !ctools::check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                      b"NT\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                    np_matrix[i as usize] = j;
                    break ;
                } else { j -= 1 }
            }
            /* if the first/last word of baseNP is PU, but the last/first word of baseNP is not PU, then delete this NP */
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"PU\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*(*sp).bnst_data.offset(np_matrix[i as
                                                                        usize]
                                                              as isize)).f,
                                 b"PU\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() ||
                   check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                 b"PU\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() &&
                       !check_feature((*(*sp).bnst_data.offset(np_matrix[i as
                                                                             usize]
                                                                   as
                                                                   isize)).f,
                                      b"PU\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                np_matrix[i as usize] = -(1 as libc::c_int);
                break ;
            } else {
                if flag == 0 {
                    start_np = i;
                    end_np = np_matrix[i as usize]
                } else { end_np = np_matrix[start_np as usize] }
                if OptDisplay == 3 as libc::c_int {
                    printf(b"NP (%d-%d)\n\x00" as *const u8 as
                               *const libc::c_char, start_np, end_np);
                }
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"PP_B\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            j = i + 1 as libc::c_int;
            while j < (*sp).Bnst_num &&
                      check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                    b"PP_B\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() &&
                      check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                    b"PP_O\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() {
                j += 1
            }
            pp_matrix[i as usize] = j - 1 as libc::c_int;
            if OptDisplay == 3 as libc::c_int {
                printf(b"PP (%d-%d)\n\x00" as *const u8 as
                           *const libc::c_char, i, j - 1 as libc::c_int);
            }
            i = j - 1 as libc::c_int;
            flag = (0 as libc::c_int == 0) as libc::c_int
        }
        i += 1
    }
    return flag;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_matrix_for_phrase(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !(np_matrix[i as usize] == -(1 as libc::c_int) &&
                 pp_matrix[i as usize] == -(1 as libc::c_int)) {
            /* the head of np must be the last word */
            if np_matrix[i as usize] != -(1 as libc::c_int) {
                head = np_matrix[i as usize];
                j = np_matrix[i as usize];
                while j >= i {
                    if !check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                      b"NN\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                           !check_feature((*(*sp).bnst_data.offset(j as
                                                                       isize)).f,
                                          b"NR\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                        head = j;
                        break ;
                    } else { j -= 1 }
                }
                /* mask upper dpnd */
                j = 0 as libc::c_int;
                while j < i {
                    k = i;
                    while k <= np_matrix[i as usize] {
                        if k != head {
                            (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                  isize))[k as
                                                                              usize]
                                = 0 as libc::c_int
                        }
                        k += 1
                    }
                    j += 1
                }
                /* mask right dpnd */
                j = i;
                while j <= np_matrix[i as usize] {
                    k = np_matrix[i as usize] + 1 as libc::c_int;
                    while k < (*sp).Bnst_num {
                        if j != head {
                            (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                  isize))[k as
                                                                              usize]
                                = 0 as libc::c_int
                        }
                        k += 1
                    }
                    j += 1
                }
                /* mask inside dpnd */
                j = i;
                while j <= np_matrix[i as usize] {
                    if j != head &&
                           check_feature((*(*sp).bnst_data.offset(j as
                                                                      isize)).f,
                                         b"JJ\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() {
                        k = j + 1 as libc::c_int;
                        while k <= np_matrix[i as usize] {
                            if k != head {
                                (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                      isize))[k
                                                                                  as
                                                                                  usize]
                                    = 0 as libc::c_int
                            }
                            k += 1
                        }
                    }
                    j += 1
                }
            }
            /* the head of pp must be the first word */
            if pp_matrix[i as usize] != -(1 as libc::c_int) {
                /* mask upper dpnd */
                j = 0 as libc::c_int;
                while j < i {
                    k = i + 1 as libc::c_int;
                    while k <= pp_matrix[i as usize] {
                        (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                              isize))[k as
                                                                          usize]
                            = 0 as libc::c_int;
                        k += 1
                    }
                    j += 1
                }
                /* mask right dpnd */
                j = i + 1 as libc::c_int;
                while j <= pp_matrix[i as usize] {
                    k = pp_matrix[i as usize];
                    while k < (*sp).Bnst_num {
                        (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                              isize))[k as
                                                                          usize]
                            = 0 as libc::c_int;
                        k += 1
                    }
                    j += 1
                }
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_matrix_for_fragment(mut sp:
                                                        *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut head_pos: libc::c_int = -(1 as libc::c_int);
    /* get the head position of this fragment, i.e. the last non-pu word */
    i = (*sp).Bnst_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                         b"PU\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() {
            head_pos = i;
            break ;
        } else { i -= 1 }
    }
    /* change dpnd matrix, make all the words depend on the head word */
    if head_pos != -(1 as libc::c_int) {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            j = i + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                if i != head_pos && j != head_pos {
                    (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                        = 0 as libc::c_int
                } else if i != head_pos && j == head_pos {
                    (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                        = 'R' as i32
                } else if i == head_pos && j != head_pos {
                    (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                        = 'L' as i32
                }
                j += 1
            }
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_np_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut start: libc::c_int = -(1 as libc::c_int);
    let mut end: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if np_matrix[i as usize] == -(1 as libc::c_int) {
            j = i;
            while j < (*sp).Bnst_num {
                (*Chi_np_start_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                    = -(1 as libc::c_int);
                (*Chi_np_end_matrix.as_mut_ptr().offset(i as
                                                            isize))[j as
                                                                        usize]
                    = -(1 as libc::c_int);
                j += 1
            }
        } else {
            start = i;
            end = np_matrix[i as usize];
            // check if this np conflict with detected quote, if conflict, delete this np
            if (*Chi_quote_start_matrix.as_mut_ptr().offset(start as
                                                                isize))[start
                                                                            as
                                                                            usize]
                   != -(1 as libc::c_int) &&
                   (*Chi_quote_start_matrix.as_mut_ptr().offset(start as
                                                                    isize))[start
                                                                                as
                                                                                usize]
                       < start &&
                   (*Chi_quote_end_matrix.as_mut_ptr().offset(start as
                                                                  isize))[start
                                                                              as
                                                                              usize]
                       >= start &&
                   (*Chi_quote_end_matrix.as_mut_ptr().offset(start as
                                                                  isize))[start
                                                                              as
                                                                              usize]
                       <= end ||
                   (*Chi_quote_start_matrix.as_mut_ptr().offset(end as
                                                                    isize))[end
                                                                                as
                                                                                usize]
                       != -(1 as libc::c_int) &&
                       (*Chi_quote_start_matrix.as_mut_ptr().offset(end as
                                                                        isize))[end
                                                                                    as
                                                                                    usize]
                           >= start &&
                       (*Chi_quote_start_matrix.as_mut_ptr().offset(end as
                                                                        isize))[end
                                                                                    as
                                                                                    usize]
                           <= end &&
                       (*Chi_quote_end_matrix.as_mut_ptr().offset(end as
                                                                      isize))[end
                                                                                  as
                                                                                  usize]
                           > end {
                i = end
            } else {
                j = start;
                while j <= end {
                    k = j;
                    while k <= end {
                        (*Chi_np_start_matrix.as_mut_ptr().offset(j as
                                                                      isize))[k
                                                                                  as
                                                                                  usize]
                            = start;
                        (*Chi_np_end_matrix.as_mut_ptr().offset(j as
                                                                    isize))[k
                                                                                as
                                                                                usize]
                            = end;
                        k += 1
                    }
                    j += 1
                }
                (*Chi_np_start_matrix.as_mut_ptr().offset(start as
                                                              isize))[end as
                                                                          usize]
                    = -(1 as libc::c_int);
                (*Chi_np_end_matrix.as_mut_ptr().offset(start as
                                                            isize))[end as
                                                                        usize]
                    = -(1 as libc::c_int);
                i = end
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn base_phrase(mut sp: *mut SENTENCE_DATA,
                                     mut is_frag: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    let mut flag: libc::c_int = 0;
    init_phrase(sp);
    if is_frag != 0 { return 0 as libc::c_int }
    /* 呼応のチェック */
    flag =
        if check_phrase(sp) == (0 as libc::c_int == 0) as libc::c_int {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int };
    /* 行列の書き換え */
    change_matrix_for_phrase(sp);
    assign_np_matrix(sp);
    return flag;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn fragment(mut sp: *mut SENTENCE_DATA) -> libc::c_int 
 /*==================================================================*/
 {
    // deal with np phrase, i.e. with only nouns
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                         b"NN\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"NR\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"NT\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"PN\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
               check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                             b"PU\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
            flag = 0 as libc::c_int;
            break ;
        } else { i += 1 }
    }
    if flag != 0 { change_matrix_for_fragment(sp); }
    return flag;
}
/*====================================================================
                               END
====================================================================*/
