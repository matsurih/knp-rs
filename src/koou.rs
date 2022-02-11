#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]


use crate::ctools::{check_feature, Language, Outfp, stderr};
use crate::Dpnd_matrix;
use crate::read_rule::{CurKoouRuleSize, KoouRuleArray};
use crate::regexp::_regexpbnst_match;
use crate::structs::{CDB_FILE, KoouRule};
use crate::tools::OptDisplay;
use crate::types::{BNST_DATA, DBM_FILE, SENTENCE_DATA};

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



/*====================================================================

			      呼応の処理

                                               S.Kurohashi 1995. 7. 4
                                               S.Ozaki     1995. 2. 8

    $Id$

====================================================================*/
#[no_mangle]
pub static mut Koou_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut Koou_dpnd_matrix: [[libc::c_int; 200]; 200] = [[0; 200]; 200];
#[no_mangle]
pub static mut koou_m_p: [libc::c_int; 200] = [0; 200];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_koou(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        koou_m_p[i as usize] = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            Koou_matrix[i as usize][j as usize] = 0 as libc::c_int;
            Koou_dpnd_matrix[i as usize][j as usize] = 0 as libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_koou(mut sp: *mut SENTENCE_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* 今のところ記述できない */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut pu_flag: libc::c_int = 0;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut c_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut r_ptr: *mut KoouRule = 0 as *mut KoouRule;
    flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        r_ptr = KoouRuleArray.as_mut_ptr();
        while j < CurKoouRuleSize {
            if _regexpbnst_match((*r_ptr).start_pattern, b_ptr) !=
                   -(1 as libc::c_int) &&
                   (Language != 2 as libc::c_int ||
                        Language == 2 as libc::c_int &&
                            !check_feature((*(*sp).bnst_data.offset(i as
                                                                        isize)).f,
                                           b"P\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null()) {
                if OptDisplay == 3 as libc::c_int {
                    fprintf(stderr,
                            b"Start (%d) %d\n\x00" as *const u8 as
                                *const libc::c_char, j, i);
                }
                k = i;
                c_ptr = b_ptr;
                while k < (*sp).Bnst_num {
                    if Language == 2 as libc::c_int {
                        pu_flag = 0 as libc::c_int;
                        l = i;
                        while l < k {
                            if !check_feature((*(*sp).bnst_data.offset(l as
                                                                           isize)).f,
                                              b"PU\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                               {
                                pu_flag = 1 as libc::c_int;
                                break ;
                            } else { l += 1 }
                        }
                    } else { pu_flag = 0 as libc::c_int }
                    if _regexpbnst_match((*r_ptr).end_pattern, c_ptr) !=
                           -(1 as libc::c_int) &&
                           (Language != 2 as libc::c_int ||
                                Language == 2 as libc::c_int && pu_flag == 0
                                    &&
                                    (!check_feature((*(*sp).bnst_data.offset(k
                                                                                 as
                                                                                 isize)).f,
                                                    b"LC\x00" as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char).is_null()
                                         ||
                                         !check_feature((*(*sp).bnst_data.offset(k
                                                                                     as
                                                                                     isize)).f,
                                                        b"NN\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char).is_null()))
                       {
                        koou_m_p[i as usize] =
                            (0 as libc::c_int == 0) as libc::c_int;
                        flag = (0 as libc::c_int == 0) as libc::c_int;
                        Koou_matrix[i as usize][k as usize] =
                            1 as libc::c_int;
                        Koou_dpnd_matrix[i as usize][k as usize] =
                            (*r_ptr).dpnd_type as libc::c_int;
                        if OptDisplay == 3 as libc::c_int {
                            fprintf(Outfp,
                                    b"  End %d\n\x00" as *const u8 as
                                        *const libc::c_char, k);
                        }
                    } else if !(*r_ptr).uke_pattern.is_null() &&
                                  _regexpbnst_match((*r_ptr).uke_pattern,
                                                    c_ptr) !=
                                      -(1 as libc::c_int) &&
                                  (Language != 2 as libc::c_int ||
                                       Language == 2 as libc::c_int &&
                                           pu_flag == 0 &&
                                           !check_feature((*(*sp).bnst_data.offset(k
                                                                                       as
                                                                                       isize)).f,
                                                          b"LC\x00" as
                                                              *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char).is_null())
                     {
                        Koou_matrix[i as usize][k as usize] =
                            2 as libc::c_int;
                        Koou_dpnd_matrix[i as usize][k as usize] =
                            (*r_ptr).dpnd_type as libc::c_int;
                        if OptDisplay == 3 as libc::c_int {
                            fprintf(Outfp,
                                    b"  Uke %d\n\x00" as *const u8 as
                                        *const libc::c_char, k);
                        }
                    }
                    k += 1;
                    c_ptr = c_ptr.offset(1)
                }
            }
            j += 1;
            r_ptr = r_ptr.offset(1)
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    return flag;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mask_for(mut sp: *mut SENTENCE_DATA,
                                  mut si: libc::c_int, mut start: libc::c_int,
                                  mut end: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = si + 1 as libc::c_int;
    while i < start {
        j = end + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mask_back(mut si: libc::c_int,
                                   mut start: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < si {
        j = si + 1 as libc::c_int;
        while j < start {
            (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_matrix(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut f_start: libc::c_int = 0;
    let mut f_end: libc::c_int = 0;
    if Language == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            if koou_m_p[i as usize] == (0 as libc::c_int == 0) as libc::c_int
               {
                f_start = -(1 as libc::c_int);
                f_end = -(1 as libc::c_int);
                j = i;
                while j < (*sp).Bnst_num {
                    if Koou_matrix[i as usize][j as usize] > 0 as libc::c_int
                       {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = Koou_dpnd_matrix[i as usize][j as usize];
                        if Koou_matrix[i as usize][j as usize] ==
                               1 as libc::c_int {
                            f_end = j;
                            if f_start < 0 as libc::c_int { f_start = j }
                        }
                        if Koou_dpnd_matrix[i as usize][j as usize] ==
                               'L' as i32 {
                            k = j + 1 as libc::c_int;
                            while k < (*sp).Bnst_num {
                                (*Dpnd_matrix.as_mut_ptr().offset(j as
                                                                      isize))[k
                                                                                  as
                                                                                  usize]
                                    = 0 as libc::c_int;
                                k += 1
                            }
                            k = 0 as libc::c_int;
                            while k < i {
                                (*Dpnd_matrix.as_mut_ptr().offset(k as
                                                                      isize))[j
                                                                                  as
                                                                                  usize]
                                    = 0 as libc::c_int;
                                k += 1
                            }
                            k = i + 1 as libc::c_int;
                            while k < j {
                                l = 0 as libc::c_int;
                                while l < i {
                                    (*Dpnd_matrix.as_mut_ptr().offset(l as
                                                                          isize))[k
                                                                                      as
                                                                                      usize]
                                        = 0 as libc::c_int;
                                    l += 1
                                }
                                l = j + 1 as libc::c_int;
                                while l < (*sp).Bnst_num {
                                    (*Dpnd_matrix.as_mut_ptr().offset(k as
                                                                          isize))[l
                                                                                      as
                                                                                      usize]
                                        = 0 as libc::c_int;
                                    l += 1
                                }
                                k += 1
                            }
                        }
                        if Koou_dpnd_matrix[i as usize][j as usize] ==
                               'R' as i32 {
                            k = i + 1 as libc::c_int;
                            while k < (*sp).Bnst_num {
                                if k != j {
                                    (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                          isize))[k
                                                                                      as
                                                                                      usize]
                                        = 0 as libc::c_int
                                }
                                k += 1
                            }
                            k = 0 as libc::c_int;
                            while k < i {
                                (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                                      isize))[k
                                                                                  as
                                                                                  usize]
                                    = 0 as libc::c_int;
                                k += 1
                            }
                            k = i + 1 as libc::c_int;
                            while k < j {
                                l = 0 as libc::c_int;
                                while l < i {
                                    (*Dpnd_matrix.as_mut_ptr().offset(l as
                                                                          isize))[k
                                                                                      as
                                                                                      usize]
                                        = 0 as libc::c_int;
                                    l += 1
                                }
                                l = j + 1 as libc::c_int;
                                while l < (*sp).Bnst_num {
                                    (*Dpnd_matrix.as_mut_ptr().offset(k as
                                                                          isize))[l
                                                                                      as
                                                                                      usize]
                                        = 0 as libc::c_int;
                                    l += 1
                                }
                                k += 1
                            }
                        }
                    }
                    j += 1
                }
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num {
            if koou_m_p[i as usize] == (0 as libc::c_int == 0) as libc::c_int
               {
                /* i -> f_start .. f_end という呼応 */
                f_start = -(1 as libc::c_int);
                f_end = -(1 as libc::c_int);
                j = i;
                while j < (*sp).Bnst_num {
                    if Koou_matrix[i as usize][j as usize] > 0 as libc::c_int
                       {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = Koou_dpnd_matrix[i as usize][j as usize];
                        if Koou_matrix[i as usize][j as usize] ==
                               1 as libc::c_int {
                            /* 前部[0,i)が(i,f_start)に係るのをマスク */
                            /* end_pattern */
                            f_end = j;
                            if f_start < 0 as libc::c_int {
                                f_start = j
                            } /* (複数ある)呼応内で係りうる最後の文節 */
                            /* (複数ある)呼応内で係りうる最初の文節 */
                        }
                    } else {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            = 0 as libc::c_int
                    } /* 内部(i,f_start)がf_end以降に係るのをマスク */
                    j += 1
                }
                mask_for(sp, i, f_start, f_end);
                mask_back(i, f_start);
            }
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn koou(mut sp: *mut SENTENCE_DATA) -> libc::c_int 
 /*==================================================================*/
 {
    let mut flag: libc::c_int = 0;
    init_koou(sp);
    flag =
        if check_koou(sp) == (0 as libc::c_int == 0) as libc::c_int {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int };
    /* 行列の書き換え */
    change_matrix(sp);
    return flag;
}
/*====================================================================
                               END
====================================================================*/
