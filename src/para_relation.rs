#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//! 並列構造間の関係

use libc;

use crate::{fprintf, sprintf};
use crate::ctools::{assign_cfeature, fputc, Outfp, stderr};
use crate::lib_print::print_bnst;
use crate::para_revision::revise_para_rel;
use crate::structs::CDB_FILE;
use crate::tools::OptDisplay;
use crate::types::{DBM_FILE, PARA_DATA, Para_M_ptr, PARA_MANAGER, SENTENCE_DATA};

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
pub static mut para_rel_matrix: [[libc::c_int; 32]; 32] = [[0; 32]; 32];
static mut RESULT: [*mut libc::c_char; 9] =
    [b"\xe9\x87\x8d\xe3\x81\xaa\xe3\x82\x8a\xe3\x81\xaa\xe3\x81\x97\x00" as
        *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xb0\x91\xe3\x81\x97\xe9\x87\x8d\xe3\x81\xaa\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x89\x8d\xe3\x81\xa7\xe9\x87\x8d\xe3\x81\xaa\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\xbe\x8c\xe3\x81\xa7\xe9\x87\x8d\xe3\x81\xaa\xe3\x82\x8b\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe9\x87\x8d\xe8\xa4\x87\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char,
        b"\xe5\x89\x8d\xe9\x83\xa8\xe3\x81\xae\xe4\xbf\xae\xe6\xad\xa3\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x90\xab\xe3\x81\xbe\xe3\x82\x8c\xe3\x82\x8b\xe5\x89\x8d\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe5\x90\xab\xe3\x81\xbe\xe3\x82\x8c\xe3\x82\x8b\xe5\xbe\x8c\x00" as
            *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xe8\xaa\xa4\xe3\x82\x8a\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char];
static mut rel_matrix_normal: [[libc::c_int; 4]; 4] =
    [[1 as libc::c_int, 3 as libc::c_int, 8 as libc::c_int, 7 as libc::c_int],
        [2 as libc::c_int, 4 as libc::c_int, 8 as libc::c_int, 7 as libc::c_int],
        [5 as libc::c_int, 5 as libc::c_int, 8 as libc::c_int, 8 as libc::c_int],
        [6 as libc::c_int, 6 as libc::c_int, 8 as libc::c_int,
            8 as libc::c_int]];
static mut rel_matrix_strong: [[libc::c_int; 4]; 4] =
    [[8 as libc::c_int, 3 as libc::c_int, 8 as libc::c_int, 7 as libc::c_int],
        [2 as libc::c_int, 4 as libc::c_int, 8 as libc::c_int, 7 as libc::c_int],
        [5 as libc::c_int, 5 as libc::c_int, 8 as libc::c_int, 8 as libc::c_int],
        [6 as libc::c_int, 6 as libc::c_int, 8 as libc::c_int,
            8 as libc::c_int]];
/* 
   strongの(0,1)はBADにしていたが，POSでよい例文があったので修正した．

   例文) ラジさん自身は英国籍を持つが、インド系の二万五千人のうち約五、
   六千人はインド国籍も英国籍もなく、香港住民としての資格しかない。
*/
#[no_mangle]
pub unsafe extern "C" fn print_two_para_relation(mut sp: *mut SENTENCE_DATA, mut p_num1: libc::c_int, mut p_num2: libc::c_int) {
    /* 並列構造間の関係の表示 */
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    let mut a3: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut b3: libc::c_int = 0;
    let mut ptr1: *mut PARA_DATA = 0 as *mut PARA_DATA;
    let mut ptr2: *mut PARA_DATA = 0 as *mut PARA_DATA;
    ptr1 = &mut *(*sp).para_data.offset(p_num1 as isize) as *mut PARA_DATA;
    a1 = (*ptr1).max_path[0 as libc::c_int as usize];
    a2 = (*ptr1).key_pos;
    a3 = (*ptr1).jend_pos;
    ptr2 = &mut *(*sp).para_data.offset(p_num2 as isize) as *mut PARA_DATA;
    b1 = (*ptr2).max_path[0 as libc::c_int as usize];
    b2 = (*ptr2).key_pos;
    b3 = (*ptr2).jend_pos;
    fprintf(Outfp, b"%-10s ==> \x00" as *const u8 as *const libc::c_char,
            RESULT[para_rel_matrix[p_num1 as usize][p_num2 as usize] as
                usize]);
    if a1 != a2 {
        print_bnst(&mut *(*sp).bnst_data.offset(a1 as isize), 0 as *mut libc::c_char);
    }
    fputc('(' as i32, Outfp);
    print_bnst(&mut *(*sp).bnst_data.offset(a2 as isize), 0 as *mut libc::c_char);
    fputc(')' as i32, Outfp);
    print_bnst(&mut *(*sp).bnst_data.offset(a3 as isize),
               0 as *mut libc::c_char);
    fprintf(Outfp, b" <=> \x00" as *const u8 as *const libc::c_char);
    if b1 != b2 {
        print_bnst(&mut *(*sp).bnst_data.offset(b1 as isize),
                   0 as *mut libc::c_char);
    }
    fputc('(' as i32, Outfp);
    print_bnst(&mut *(*sp).bnst_data.offset(b2 as isize),
               0 as *mut libc::c_char);
    fputc(')' as i32, Outfp);
    print_bnst(&mut *(*sp).bnst_data.offset(b3 as isize),
               0 as *mut libc::c_char);
    fputc('\n' as i32, Outfp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_para_manager(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    (*sp).Para_M_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        (*(*sp).para_manager.offset(i as isize)).para_num = 0 as libc::c_int;
        (*(*sp).para_manager.offset(i as isize)).part_num = 0 as libc::c_int;
        let ref mut fresh0 = (*(*sp).para_manager.offset(i as isize)).parent;
        *fresh0 = 0 as Para_M_ptr;
        (*(*sp).para_manager.offset(i as isize)).child_num = 0 as libc::c_int;
        let ref mut fresh1 =
            (*(*sp).para_data.offset(i as isize)).manager_ptr;
        *fresh1 = 0 as Para_M_ptr;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_location(mut sp: *mut SENTENCE_DATA,
                                       mut pre_num: libc::c_int,
                                       mut pos_num: libc::c_int)
                                       -> libc::c_int
/*==================================================================*/
{
    /* 並列構造間の関係の決定 */
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    let mut a3: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut b3: libc::c_int = 0;
    let mut rel_pre: libc::c_int = 0;
    let mut rel_pos: libc::c_int = 0;
    a1 =
        (*(*sp).para_data.offset(pre_num as
            isize)).max_path[0 as libc::c_int as
            usize];
    a2 = (*(*sp).para_data.offset(pre_num as isize)).key_pos;
    a3 = (*(*sp).para_data.offset(pre_num as isize)).jend_pos;
    b1 =
        (*(*sp).para_data.offset(pos_num as
            isize)).max_path[0 as libc::c_int as
            usize];
    b2 = (*(*sp).para_data.offset(pos_num as isize)).key_pos;
    b3 = (*(*sp).para_data.offset(pos_num as isize)).jend_pos;
    if a3 < b1 { return 0 as libc::c_int; }
    if (a2 + 1 as libc::c_int) < b1 {
        rel_pre = 0 as libc::c_int
    } else if a2 + 1 as libc::c_int == b1 {
        rel_pre = 1 as libc::c_int
    } else if a1 < b1 {
        rel_pre = 2 as libc::c_int
    } else { rel_pre = 3 as libc::c_int }
    if a3 < b2 {
        rel_pos = 0 as libc::c_int
    } else if a3 == b2 {
        rel_pos = 1 as libc::c_int
    } else if a3 < b3 {
        rel_pos = 2 as libc::c_int
    } else { rel_pos = 3 as libc::c_int }
    return if (*(*sp).para_data.offset(pos_num as isize)).status as libc::c_int ==
        's' as i32 {
        rel_matrix_strong[rel_pre as usize][rel_pos as usize]
    } else { rel_matrix_normal[rel_pre as usize][rel_pos as usize] };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_brother_p(mut sp: *mut SENTENCE_DATA,
                                        mut pre_num: libc::c_int,
                                        mut pos_num: libc::c_int)
                                        -> libc::c_int
/*==================================================================*/
{
    /* REL_POS -> REL_PAR に変換する条件
       前の並列構造のpost-conjunctと後の並列構造のpre-conjunctの
       大きさがそれほどかわらない（４：３以下）
       */
    let mut pre_length: libc::c_int = 0;
    let mut pos_length: libc::c_int = 0;
    pre_length =
        (*(*sp).para_data.offset(pre_num as isize)).jend_pos -
            (*(*sp).para_data.offset(pre_num as isize)).key_pos;
    pos_length =
        (*(*sp).para_data.offset(pos_num as isize)).key_pos -
            (*(*sp).para_data.offset(pos_num as
                isize)).max_path[0 as libc::c_int as
                usize] +
            1 as libc::c_int;
    return if pre_length * 3 as libc::c_int <= pos_length * 4 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_child(mut parent_ptr: *mut PARA_MANAGER,
                                      mut child_ptr: *mut PARA_MANAGER)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*parent_ptr).child_num {
        if (*parent_ptr).child[i as usize] == child_ptr {
            j = i;
            while j < (*parent_ptr).child_num - 1 as libc::c_int {
                (*parent_ptr).child[j as usize] =
                    (*parent_ptr).child[(j + 1 as libc::c_int) as usize];
                j += 1
            }
            (*parent_ptr).child_num -= 1 as libc::c_int;
            break;
        } else { i += 1 }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn set_parent(mut parent_ptr: *mut PARA_MANAGER,
                                    mut child_ptr: *mut PARA_MANAGER)
                                    -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut i_num: libc::c_int = 0;
    let mut j_num: libc::c_int = 0;
    if !(*child_ptr).parent.is_null() {
        if (*child_ptr).parent == parent_ptr {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < (*(*child_ptr).parent).para_num {
            i_num = (*(*child_ptr).parent).para_data_num[i as usize];
            j = 0 as libc::c_int;
            while j < (*parent_ptr).para_num {
                j_num = (*parent_ptr).para_data_num[j as usize];
                /* 元の親が直接の親 */
                if i_num < j_num &&
                    (para_rel_matrix[i_num as usize][j_num as usize] ==
                        1 as libc::c_int ||
                        para_rel_matrix[i_num as usize][j_num as usize] ==
                            2 as libc::c_int ||
                        para_rel_matrix[i_num as usize][j_num as usize] ==
                            5 as libc::c_int ||
                        para_rel_matrix[i_num as usize][j_num as usize] ==
                            6 as libc::c_int) ||
                    j_num < i_num &&
                        (para_rel_matrix[j_num as usize][i_num as usize] ==
                            3 as libc::c_int ||
                            para_rel_matrix[j_num as
                                usize][i_num as usize] ==
                                7 as libc::c_int) {
                    return (0 as libc::c_int == 0) as libc::c_int;
                } else {
                    /* 新しい親が直接の親 */
                    if i_num < j_num &&
                        (para_rel_matrix[i_num as usize][j_num as usize] ==
                            3 as libc::c_int ||
                            para_rel_matrix[i_num as
                                usize][j_num as usize] ==
                                7 as libc::c_int) ||
                        j_num < i_num &&
                            (para_rel_matrix[j_num as
                                usize][i_num as usize] ==
                                1 as libc::c_int ||
                                para_rel_matrix[j_num as
                                    usize][i_num as usize]
                                    == 2 as libc::c_int ||
                                para_rel_matrix[j_num as
                                    usize][i_num as usize]
                                    == 5 as libc::c_int ||
                                para_rel_matrix[j_num as
                                    usize][i_num as usize]
                                    == 6 as libc::c_int) {
                        delete_child((*child_ptr).parent, child_ptr);
                        (*child_ptr).parent = parent_ptr;
                        let fresh2 = (*parent_ptr).child_num;
                        (*parent_ptr).child_num = (*parent_ptr).child_num + 1;
                        (*parent_ptr).child[fresh2 as usize] = child_ptr;
                        if (*parent_ptr).child_num >= 32 as libc::c_int {
                            fprintf(stderr,
                                    b";; Too many para!\n\x00" as *const u8 as
                                        *const libc::c_char);
                            return 0 as libc::c_int;
                        }
                        return (0 as libc::c_int == 0) as libc::c_int;
                    }
                }
                j += 1
            }
            i += 1
        }
    } else {
        (*child_ptr).parent = parent_ptr;
        let fresh3 = (*parent_ptr).child_num;
        (*parent_ptr).child_num = (*parent_ptr).child_num + 1;
        (*parent_ptr).child[fresh3 as usize] = child_ptr;
        if (*parent_ptr).child_num >= 32 as libc::c_int {
            fprintf(stderr,
                    b";; Too many para!\n\x00" as *const u8 as
                        *const libc::c_char);
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_revise_scope(mut ptr: *mut PARA_MANAGER)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut child_ptr: *mut PARA_MANAGER = 0 as *mut PARA_MANAGER;
    if (*ptr).child_num != 0 {
        /* 子供の処理 */
        i = 0 as libc::c_int;
        while i < (*ptr).child_num {
            para_revise_scope((*ptr).child[i as usize]);
            i += 1
        }
        /* 左側の修正 */
        if (*(*ptr).child[0 as libc::c_int as
            usize]).start[0 as libc::c_int as usize] <
            (*ptr).start[0 as libc::c_int as usize] {
            (*ptr).start[0 as libc::c_int as usize] =
                (*(*ptr).child[0 as libc::c_int as
                    usize]).start[0 as libc::c_int as usize]
        }
        /* 右側の修正 */
        child_ptr =
            (*ptr).child[((*ptr).child_num - 1 as libc::c_int) as usize];
        if (*ptr).end[((*ptr).part_num - 1 as libc::c_int) as usize] <
            (*child_ptr).end[((*child_ptr).part_num - 1 as libc::c_int) as
                usize] {
            (*ptr).end[((*ptr).part_num - 1 as libc::c_int) as usize] =
                (*child_ptr).end[((*child_ptr).part_num - 1 as libc::c_int) as
                    usize]
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn detect_para_relation(mut sp: *mut SENTENCE_DATA)
                                              -> libc::c_int
/*==================================================================*/
{
    /* 並列構造間の関係の整理 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut m_ptr: *mut PARA_MANAGER = 0 as *mut PARA_MANAGER;
    let mut m_ptr1: *mut PARA_MANAGER = 0 as *mut PARA_MANAGER;
    let mut m_ptr2: *mut PARA_MANAGER = 0 as *mut PARA_MANAGER;
    let mut buffer1: [libc::c_char; 128] = [0; 128];
    let mut buffer2: [libc::c_char; 128] = [0; 128];
    /* 位置関係の決定，誤りの修正 */
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if !((*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32) {
            j = i + 1 as libc::c_int;
            while j < (*sp).Para_num {
                if !((*(*sp).para_data.offset(j as isize)).status as
                    libc::c_int == 'x' as i32) {
                    para_rel_matrix[i as usize][j as usize] =
                        para_location(sp, i, j);
                    if para_rel_matrix[i as usize][j as usize] ==
                        8 as libc::c_int {
                        if OptDisplay == 3 as libc::c_int {
                            print_two_para_relation(sp, i, j);
                        }
                        revise_para_rel(sp, i, j);
                        return 0 as libc::c_int;
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    init_para_manager(sp);
    /* REL_POSで重なりの割合が大きい場合REL_PARに変更 */
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if !((*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32) {
            j = 0 as libc::c_int;
            while j < (*sp).Para_num {
                if !((*(*sp).para_data.offset(j as isize)).status as
                    libc::c_int == 'x' as i32) {
                    if para_rel_matrix[i as usize][j as usize] ==
                        3 as libc::c_int &&
                        para_brother_p(sp, i, j) ==
                            (0 as libc::c_int == 0) as libc::c_int {
                        para_rel_matrix[i as usize][j as usize] =
                            4 as libc::c_int
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    /* 左にREL_POS，右にREL_PREの場合，その間をREL_REVに変更 */
    i = 1 as libc::c_int;
    while i < (*sp).Para_num - 1 as libc::c_int {
        if !((*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32) {
            j = 0 as libc::c_int;
            while j < i {
                if !((*(*sp).para_data.offset(j as isize)).status as
                    libc::c_int == 'x' as i32) {
                    if para_rel_matrix[j as usize][i as usize] ==
                        3 as libc::c_int {
                        k = i + 1 as libc::c_int;
                        while k < (*sp).Para_num {
                            if !((*(*sp).para_data.offset(k as isize)).status
                                as libc::c_int == 'x' as i32) {
                                if para_rel_matrix[i as usize][k as usize] ==
                                    2 as libc::c_int {
                                    para_rel_matrix[j as usize][k as usize] =
                                        5 as libc::c_int
                                }
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
    /* 兄弟関係のまとめ，MANAGERによる管理 */
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if !((*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32) {
            if !(*(*sp).para_data.offset(i as isize)).manager_ptr.is_null() {
                m_ptr = (*(*sp).para_data.offset(i as isize)).manager_ptr
            } else {
                let fresh4 = (*sp).Para_M_num;
                (*sp).Para_M_num = (*sp).Para_M_num + 1;
                m_ptr =
                    &mut *(*sp).para_manager.offset(fresh4 as isize) as
                        *mut PARA_MANAGER;
                let ref mut fresh5 =
                    (*(*sp).para_data.offset(i as isize)).manager_ptr;
                *fresh5 = m_ptr;
                let fresh6 = (*m_ptr).para_num;
                (*m_ptr).para_num = (*m_ptr).para_num + 1;
                (*m_ptr).para_data_num[fresh6 as usize] = i;
                if (*m_ptr).para_num >= 32 as libc::c_int {
                    fprintf(stderr,
                            b";; Too many para (%s)!\n\x00" as *const u8 as
                                *const libc::c_char,
                            if !(*sp).Comment.is_null() {
                                (*sp).Comment as *const libc::c_char
                            } else {
                                b"\x00" as *const u8 as *const libc::c_char
                            });
                    return 0 as libc::c_int;
                }
                (*m_ptr).start[(*m_ptr).part_num as usize] =
                    (*(*sp).para_data.offset(i as
                        isize)).max_path[0 as
                        libc::c_int
                        as
                        usize];
                let fresh7 = (*m_ptr).part_num;
                (*m_ptr).part_num = (*m_ptr).part_num + 1;
                (*m_ptr).end[fresh7 as usize] =
                    (*(*sp).para_data.offset(i as isize)).key_pos;
                (*m_ptr).start[(*m_ptr).part_num as usize] =
                    (*(*sp).para_data.offset(i as isize)).key_pos +
                        1 as libc::c_int;
                let fresh8 = (*m_ptr).part_num;
                (*m_ptr).part_num = (*m_ptr).part_num + 1;
                (*m_ptr).end[fresh8 as usize] =
                    (*(*sp).para_data.offset(i as isize)).jend_pos
            }
            j = i + 1 as libc::c_int;
            while j < (*sp).Para_num {
                if !((*(*sp).para_data.offset(j as isize)).status as
                    libc::c_int == 'x' as i32) {
                    match para_rel_matrix[i as usize][j as usize] {
                        4 => {
                            let ref mut fresh9 =
                                (*(*sp).para_data.offset(j as
                                    isize)).manager_ptr;
                            *fresh9 = m_ptr;
                            let fresh10 = (*m_ptr).para_num;
                            (*m_ptr).para_num = (*m_ptr).para_num + 1;
                            (*m_ptr).para_data_num[fresh10 as usize] = j;
                            if (*m_ptr).para_num >= 32 as libc::c_int {
                                fprintf(stderr,
                                        b";; Too many para (%s)!\n\x00" as
                                            *const u8 as *const libc::c_char,
                                        if !(*sp).Comment.is_null() {
                                            (*sp).Comment as
                                                *const libc::c_char
                                        } else {
                                            b"\x00" as *const u8 as
                                                *const libc::c_char
                                        });
                                return 0 as libc::c_int;
                            }
                            (*m_ptr).start[(*m_ptr).part_num as usize] =
                                (*(*sp).para_data.offset(j as isize)).key_pos
                                    + 1 as libc::c_int;
                            let fresh11 = (*m_ptr).part_num;
                            (*m_ptr).part_num = (*m_ptr).part_num + 1;
                            (*m_ptr).end[fresh11 as usize] =
                                (*(*sp).para_data.offset(j as isize)).jend_pos
                        }
                        _ => {}
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    /* 親子関係のまとめ m_ptr1が子，m_ptr2が親の時に処理 */
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if !((*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32) {
            m_ptr1 = (*(*sp).para_data.offset(i as isize)).manager_ptr;
            j = 0 as libc::c_int;
            while j < (*sp).Para_num {
                if !((*(*sp).para_data.offset(j as isize)).status as
                    libc::c_int == 'x' as i32) {
                    m_ptr2 =
                        (*(*sp).para_data.offset(j as isize)).manager_ptr;
                    if i < j &&
                        (para_rel_matrix[i as usize][j as usize] ==
                            1 as libc::c_int ||
                            para_rel_matrix[i as usize][j as usize] ==
                                2 as libc::c_int ||
                            para_rel_matrix[i as usize][j as usize] ==
                                5 as libc::c_int ||
                            para_rel_matrix[i as usize][j as usize] ==
                                6 as libc::c_int) ||
                        j < i &&
                            (para_rel_matrix[j as usize][i as usize] ==
                                3 as libc::c_int ||
                                para_rel_matrix[j as usize][i as usize] ==
                                    7 as libc::c_int) {
                        if set_parent(m_ptr2, m_ptr1) == 0 as libc::c_int {
                            return 0 as libc::c_int;
                        }
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    /* 範囲の修正 */
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        if (*(*sp).para_manager.offset(i as isize)).parent.is_null() {
            para_revise_scope(&mut *(*sp).para_manager.offset(i as isize));
        }
        i += 1
    }
    /* 強並列のマーク */
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        flag = (0 as libc::c_int == 0) as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*(*sp).para_manager.offset(i as isize)).para_num {
            if (*(*sp).para_data.offset((*(*sp).para_manager.offset(i as
                isize)).para_data_num[j
                as
                usize]
                as isize)).status as libc::c_int
                != 's' as i32 {
                flag = 0 as libc::c_int;
                break;
            } else { j += 1 }
        }
        (*(*sp).para_manager.offset(i as isize)).status =
            if flag == (0 as libc::c_int == 0) as libc::c_int {
                's' as i32
            } else { 'w' as i32 } as libc::c_char;
        i += 1
    }
    /* 並列解析結果をfeatureに */
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        j = 0 as libc::c_int;
        while j <
            (*(*sp).para_manager.offset(i as isize)).part_num -
                1 as libc::c_int {
            sprintf(buffer1.as_mut_ptr(),
                    b"\xe4\xb8\xa6\xe7\xb5\x90\xe5\x8f\xa5\xe6\x95\xb0:%d\x00"
                        as *const u8 as *const libc::c_char,
                    (*(*sp).para_manager.offset(i as isize)).part_num);
            sprintf(buffer2.as_mut_ptr(),
                    b"\xe4\xb8\xa6\xe7\xb5\x90\xe6\x96\x87\xe7\xaf\x80\xe6\x95\xb0:%d\x00"
                        as *const u8 as *const libc::c_char,
                    (*(*sp).para_manager.offset(i as
                        isize)).end[1 as
                        libc::c_int
                        as usize]
                        -
                        (*(*sp).para_manager.offset(i as
                            isize)).start[1 as
                            libc::c_int
                            as
                            usize]
                        + 1 as libc::c_int);
            assign_cfeature(&mut (*(*sp).bnst_data.offset(*(*(*sp).para_manager.offset(i
                as
                isize)).end.as_mut_ptr().offset(j
                as
                isize)
                as isize)).f,
                            buffer1.as_mut_ptr(), 0 as libc::c_int);
            assign_cfeature(&mut (*(*sp).bnst_data.offset(*(*(*sp).para_manager.offset(i
                as
                isize)).end.as_mut_ptr().offset(j
                as
                isize)
                as isize)).f,
                            buffer2.as_mut_ptr(), 0 as libc::c_int);
            j += 1
        }
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*====================================================================
                               END
====================================================================*/
