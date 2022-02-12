#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//! 木構造処理

use libc;

use crate::{atoi, BNST_DATA, Class, FEATUREptr, fprintf, Mask_matrix, MRPH_DATA, PARA_DATA, PARA_MANAGER, SENTENCE_DATA, strchr, strcmp, TAG_DATA, tnode_t};
use crate::ctools::{check_feature, exit, Language, stderr};
use crate::OptExpandP;
use crate::types::Treeptr_B;

#[no_mangle]
pub unsafe extern "C" fn init_bnst_tree_property(mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh0 = (*(*sp).bnst_data.offset(i as isize)).parent;
        *fresh0 = 0 as Treeptr_B;
        let ref mut fresh1 =
            (*(*sp).bnst_data.offset(i as isize)).child[0 as libc::c_int as usize];
        *fresh1 = 0 as Treeptr_B;
        (*(*sp).bnst_data.offset(i as isize)).para_top_p =
            0 as libc::c_int as libc::c_char;
        (*(*sp).bnst_data.offset(i as isize)).para_type =
            0 as libc::c_int as libc::c_char;
        (*(*sp).bnst_data.offset(i as isize)).to_para_p =
            0 as libc::c_int as libc::c_char;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_tag_tree_property(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh2 = (*(*sp).tag_data.offset(i as isize)).parent;
        *fresh2 = 0 as *mut tnode_t;
        let ref mut fresh3 =
            (*(*sp).tag_data.offset(i as
                isize)).child[0 as libc::c_int as
                usize];
        *fresh3 = 0 as *mut tnode_t;
        (*(*sp).tag_data.offset(i as isize)).para_top_p =
            0 as libc::c_int as libc::c_char;
        (*(*sp).tag_data.offset(i as isize)).para_type =
            0 as libc::c_int as libc::c_char;
        (*(*sp).tag_data.offset(i as isize)).to_para_p =
            0 as libc::c_int as libc::c_char;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_mrph_tree_property(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let ref mut fresh4 = (*(*sp).mrph_data.offset(i as isize)).parent;
        *fresh4 = 0 as Treeptr_B;
        let ref mut fresh5 =
            (*(*sp).mrph_data.offset(i as
                isize)).child[0 as libc::c_int as
                usize];
        *fresh5 = 0 as Treeptr_B;
        (*(*sp).mrph_data.offset(i as isize)).para_top_p =
            0 as libc::c_int as libc::c_char;
        (*(*sp).mrph_data.offset(i as isize)).para_type =
            0 as libc::c_int as libc::c_char;
        (*(*sp).mrph_data.offset(i as isize)).to_para_p =
            0 as libc::c_int as libc::c_char;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn t_add_node(mut parent: *mut BNST_DATA,
                                    mut child: *mut BNST_DATA,
                                    mut pos: libc::c_int) -> *mut BNST_DATA
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut child_num: libc::c_int = 0;
    child_num = 0 as libc::c_int;
    while !(*parent).child[child_num as usize].is_null() { child_num += 1 }
    if pos == -(1 as libc::c_int) {
        (*parent).child[child_num as usize] = child;
        (*parent).child[(child_num + 1 as libc::c_int) as usize] =
            0 as Treeptr_B
    } else {
        i = child_num;
        while i >= pos {
            (*parent).child[(i + 1 as libc::c_int) as usize] =
                (*parent).child[i as usize];
            i -= 1
        }
        (*parent).child[pos as usize] = child
    }
    return parent;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn t_attach_node(mut parent: *mut BNST_DATA,
                                       mut child: *mut BNST_DATA,
                                       mut pos: libc::c_int)
                                       -> *mut BNST_DATA
/*==================================================================*/
{
    (*child).parent = parent;
    return t_add_node(parent, child, pos);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn t_del_node(mut parent: *mut BNST_DATA,
                                    mut child: *mut BNST_DATA)
                                    -> *mut BNST_DATA
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*parent).child[i as usize].is_null() {
        if (*parent).child[i as usize] == child {
            j = i;
            while !(*parent).child[j as usize].is_null() {
                (*parent).child[j as usize] =
                    (*parent).child[(j + 1 as libc::c_int) as usize];
                j += 1
            }
            break;
        } else { i += 1 }
    }
    (*child).parent = 0 as Treeptr_B;
    return child;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_simple_tree(mut sp: *mut SENTENCE_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut child_num: libc::c_int = 0;
    let mut pre_node_child_num: libc::c_int = 0;
    let mut buffer: [libc::c_int; 200] = [0; 200];
    let mut tmp_b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* dpnd.head[i]をbuffer[i]にコピーし，3つ以上からなる並列構造では
       係先を隣のheadから末尾のheadに変更する．
       また部分並列の係り先を末尾のheadに変更する．*/
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num - 1 as libc::c_int {
        buffer[i as usize] = (*(*sp).bnst_data.offset(i as isize)).dpnd_head;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        j = 0 as libc::c_int;
        while j < (*(*sp).para_manager.offset(i as isize)).part_num - 1 as libc::c_int {
            buffer[(*(*sp).para_manager.offset(i as isize)).end[j as usize] as usize] = (*(*sp).para_manager.offset(i as isize)).end[((*(*sp).para_manager.offset(i as isize)).part_num - 1 as libc::c_int) as usize];

            k = (*(*sp).para_manager.offset(i as isize)).start[j as usize];
            while k <=
                (*(*sp).para_manager.offset(i as isize)).end[j as usize]
            {
                if (*Mask_matrix.as_mut_ptr().offset(k as isize))[(*(*sp).para_manager.offset(i as isize)).end[j as usize] as usize] == 3 as libc::c_int {
                    buffer[k as usize] = (*(*sp).para_manager.offset(i as isize)).end[((*(*sp).para_manager.offset(i as isize)).part_num - 1 as libc::c_int) as usize]
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    /* 依存構造木構造リンク付け */
    pre_node_child_num = 0 as libc::c_int;
    j = (*sp).Bnst_num - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        /* 受け側 */
        if pre_node_child_num != 0 as libc::c_int {
            child_num = pre_node_child_num;
            pre_node_child_num = 0 as libc::c_int
        } else { child_num = 0 as libc::c_int }
        let mut current_block_31: u64;
        i = j - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            /* 係り側 */
            if !((*(*sp).bnst_data.offset(i as isize)).num ==
                -(1 as libc::c_int)) {
                if buffer[i as usize] == j {
                    /* i -> j */
                    if (*(*sp).bnst_data.offset(j as isize)).num ==
                        -(1 as libc::c_int) {
                        /* 後処理でマージされたノード */
                        if j - i == 1 as libc::c_int {
                            current_block_31 = 15768484401365413375;
                        } else {
                            /* マージされたノードに係るノード (直前以外) */
                            let fresh6 =
                                pre_node_child_num; /* 後処理でマージされたノードならば -1 */
                            pre_node_child_num = pre_node_child_num + 1;
                            let ref mut fresh7 =
                                (*(*sp).bnst_data.offset((j -
                                    1 as
                                        libc::c_int)
                                    as
                                    isize)).child[fresh6
                                    as
                                    usize];
                            *fresh7 = (*sp).bnst_data.offset(i as isize);
                            current_block_31 = 7828949454673616476;
                        }
                    } else {
                        let fresh8 = child_num;
                        child_num = child_num + 1;
                        let ref mut fresh9 =
                            (*(*sp).bnst_data.offset(j as
                                isize)).child[fresh8
                                as
                                usize];
                        *fresh9 = (*sp).bnst_data.offset(i as isize);
                        current_block_31 = 7828949454673616476;
                    }
                    match current_block_31 {
                        15768484401365413375 => {}
                        _ => {
                            if child_num >= 32 as libc::c_int {
                                child_num =
                                    32 as libc::c_int - 1 as libc::c_int;
                                break;
                            } else {
                                let ref mut fresh10 =
                                    (*(*sp).bnst_data.offset(i as
                                        isize)).parent;
                                *fresh10 =
                                    if (*(*sp).bnst_data.offset(j as
                                        isize)).num
                                        == -(1 as libc::c_int) {
                                        (*sp).bnst_data.offset(j as
                                            isize).offset(-(1
                                            as
                                            libc::c_int
                                            as
                                            isize))
                                    } else {
                                        (*sp).bnst_data.offset(j as isize)
                                    };
                                if (*Mask_matrix.as_mut_ptr().offset(i as
                                    isize))[j
                                    as
                                    usize]
                                    == 3 as libc::c_int {
                                    (*(*sp).bnst_data.offset(i as
                                        isize)).para_type
                                        = 2 as libc::c_int as libc::c_char
                                }
                            }
                        }
                    }
                    /* PARA_NORMALは展開時にセット */
                }
            }
            /* マージ側 -> マージされた側: スキップ */
            i -= 1
        }
        let ref mut fresh11 =
            (*(*sp).bnst_data.offset(j as isize)).child[child_num as usize];
        *fresh11 = 0 as Treeptr_B;
        j -= 1
    }
    /* 子供をsort */
    j = (*sp).Bnst_num - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        child_num = 0 as libc::c_int;
        while !(*(*sp).bnst_data.offset(j as
            isize)).child[child_num as
            usize].is_null()
        {
            child_num += 1
        }
        if !(child_num < 2 as libc::c_int) {
            i = 0 as libc::c_int;
            while i < child_num - 1 as libc::c_int {
                k = i + 1 as libc::c_int;
                while k < child_num {
                    if (*(*(*sp).bnst_data.offset(j as
                        isize)).child[i as
                        usize]).num
                        <
                        (*(*(*sp).bnst_data.offset(j as
                            isize)).child[k as
                            usize]).num
                    {
                        tmp_b_ptr =
                            (*(*sp).bnst_data.offset(j as
                                isize)).child[i as
                                usize];
                        let ref mut fresh12 =
                            (*(*sp).bnst_data.offset(j as
                                isize)).child[i as
                                usize];
                        *fresh12 =
                            (*(*sp).bnst_data.offset(j as
                                isize)).child[k as
                                usize];
                        let ref mut fresh13 =
                            (*(*sp).bnst_data.offset(j as
                                isize)).child[k as
                                usize];
                        *fresh13 = tmp_b_ptr
                    }
                    k += 1
                }
                i += 1
            }
        }
        /* 2個以上のみ */
        j -= 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn strong_corr_node(mut sp: *mut SENTENCE_DATA,
                                          mut p_ptr: *mut PARA_DATA,
                                          mut b_ptr: *mut BNST_DATA)
                                          -> *mut BNST_DATA
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = (*p_ptr).jend_pos - (*p_ptr).key_pos - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*sp).bnst_data.offset((*p_ptr).max_path[i as usize] as isize) ==
            b_ptr {
            return (*sp).bnst_data.offset((*p_ptr).key_pos as
                isize).offset(i as
                isize).offset(1
                as
                libc::c_int
                as
                isize);
        }
        i -= 1
    }
    return 0 as *mut BNST_DATA;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn strong_para_expand(mut sp: *mut SENTENCE_DATA,
                                            mut m_ptr: *mut PARA_MANAGER)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p_ptr: *mut PARA_DATA = 0 as *mut PARA_DATA;
    let mut pp_ptr: *mut PARA_DATA = 0 as *mut PARA_DATA;
    let mut start_b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut bb_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* 強並列内に係る文節を展開 : コピー無 */
    i = 0 as libc::c_int;
    while i < (*m_ptr).child_num {
        strong_para_expand(sp, (*m_ptr).child[i as usize]);
        i += 1
    }
    p_ptr =
        (*sp).para_data.offset((*m_ptr).para_data_num[0 as libc::c_int as
            usize] as isize);
    if (*p_ptr).status as libc::c_int == 's' as i32 {
        start_b_ptr =
            (*sp).bnst_data.offset((*m_ptr).start[0 as libc::c_int as usize]
                as isize);
        i = (*m_ptr).start[0 as libc::c_int as usize];
        b_ptr = start_b_ptr;
        while i < (*m_ptr).end[0 as libc::c_int as usize] {
            j = 0 as libc::c_int;
            while !(*b_ptr).child[j as usize].is_null() {
                if (*b_ptr).child[j as usize] < start_b_ptr {
                    (*(*b_ptr).child[j as usize]).to_para_p =
                        (0 as libc::c_int == 0) as libc::c_int as
                            libc::c_char;
                    bb_ptr = b_ptr;
                    k = 0 as libc::c_int;
                    pp_ptr = p_ptr;
                    while k < (*m_ptr).para_num {
                        bb_ptr = strong_corr_node(sp, pp_ptr, bb_ptr);
                        if !bb_ptr.is_null() {
                            t_add_node(bb_ptr, (*b_ptr).child[j as usize],
                                       -(1 as libc::c_int));
                        }
                        k += 1;
                        pp_ptr = pp_ptr.offset(1)
                    }
                }
                j += 1
            }
            i += 1;
            b_ptr = b_ptr.offset(1)
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_correct_postprocessed_bnst_num(mut sp:
                                                            *mut SENTENCE_DATA,
                                                            mut num:
                                                            libc::c_int)
                                                            -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = num;
    while i >= 0 as libc::c_int {
        if (*(*sp).bnst_data.offset(i as isize)).num != -(1 as libc::c_int) {
            return i;
        }
        i -= 1
    }
    return i;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_top_expand(mut sp: *mut SENTENCE_DATA,
                                         mut m_ptr: *mut PARA_MANAGER)
                                         -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut new_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut end_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut pre_end_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* 並列をまとめるノードの挿入

       B  B<P> B  B<P>(end_ptr) B  B ｜ .... B(new_ptr)
                ↑                  (↑ここまでが通常の文節)
                ｜
                B(new_ptr) をここに挿入し B<P>(end_ptr)の内容をコピー
		B<P>(end_ptr) は PARA(並列をまとめるノード)となる
    */
    i = 0 as libc::c_int; /* コピー */
    while i < (*m_ptr).child_num {
        if para_top_expand(sp, (*m_ptr).child[i as usize]) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1
    }
    end_ptr =
        (*sp).bnst_data.offset(get_correct_postprocessed_bnst_num(sp,
                                                                  (*m_ptr).end[((*m_ptr).part_num
                                                                      -
                                                                      1
                                                                          as
                                                                          libc::c_int)
                                                                      as
                                                                      usize])
            as isize);
    pre_end_ptr =
        (*sp).bnst_data.offset(get_correct_postprocessed_bnst_num(sp,
                                                                  (*m_ptr).end[((*m_ptr).part_num
                                                                      -
                                                                      2
                                                                          as
                                                                          libc::c_int)
                                                                      as
                                                                      usize])
            as isize);
    new_ptr =
        (*sp).bnst_data.offset((*sp).Bnst_num as
            isize).offset((*sp).New_Bnst_num as isize);
    (*sp).New_Bnst_num += 1;
    if (*sp).Bnst_num + (*sp).New_Bnst_num > 200 as libc::c_int {
        fprintf(stderr, b";; Too many nodes in expanding para top .\n\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if (*sp).Max_New_Bnst_num < (*sp).New_Bnst_num {
        (*sp).Max_New_Bnst_num = (*sp).New_Bnst_num
    }
    *new_ptr = *end_ptr;
    /*
      new_ptr に end_ptr をコピーすると双方のf(featureへのポインタ)から
      fの実体がポイントされ，freeする際に問題となる．当初は,

      	end_ptr->f = NULL;
	
      として対処していたがこれでは並列末尾の文節がその後の用言の格解析で
      格要素とみなされない，また何度かmake_treeを行うfがなくなってしまう
      などの問題があったので，clear_featureを文ごとの解析のループの
      先頭で行うように修正した (98/02/07)
    */
    /* 子ノードの整理 */
    (*new_ptr).child[0 as libc::c_int as usize] = 0 as Treeptr_B;
    t_attach_node(end_ptr, new_ptr, 0 as libc::c_int);
    while pre_end_ptr < (*end_ptr).child[1 as libc::c_int as usize] &&
        (*(*end_ptr).child[1 as libc::c_int as usize]).para_type as
            libc::c_int != 2 as libc::c_int {
        t_attach_node(new_ptr,
                      t_del_node(end_ptr,
                                 (*end_ptr).child[1 as libc::c_int as usize]),
                      -(1 as libc::c_int));
    }
    /* フラグ(PARA,<P>)の整理 */
    (*end_ptr).para_type = 0 as libc::c_int as libc::c_char;
    (*end_ptr).para_top_p =
        (0 as libc::c_int == 0) as libc::c_int as libc::c_char;
    (*new_ptr).para_type = 1 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*m_ptr).part_num - 1 as libc::c_int {
        (*(*sp).bnst_data.offset(get_correct_postprocessed_bnst_num(sp,
                                                                    (*m_ptr).end[i
                                                                        as
                                                                        usize])
            as isize)).para_type =
            1 as libc::c_int as libc::c_char;
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_modifier_expand(mut b_ptr: *mut BNST_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* PARA に係っているノードを <P> に係ける : コピー無 */
    if (*b_ptr).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int {
        i = 0 as libc::c_int;
        while !(*b_ptr).child[i as usize].is_null() {
            if (*(*b_ptr).child[i as usize]).para_type as libc::c_int ==
                0 as libc::c_int &&
                check_feature((*(*b_ptr).child[i as usize]).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                /* b_ptr->child[i] 修飾文節 */
                (*(*b_ptr).child[i as usize]).to_para_p =
                    (0 as libc::c_int == 0) as libc::c_int as libc::c_char;
                j = 0 as libc::c_int;
                while !(*b_ptr).child[j as usize].is_null() {
                    if (*(*b_ptr).child[j as usize]).para_type as libc::c_int
                        == 1 as libc::c_int {
                        /* b_ptr->child[j] <P>文節 */
                        k = 0 as libc::c_int;
                        while !(*(*b_ptr).child[j as
                            usize]).child[k as
                            usize].is_null()
                        {
                            k += 1
                        }
                        (*(*b_ptr).child[j as usize]).child[k as usize] =
                            (*b_ptr).child[i as usize];
                        (*(*b_ptr).child[j as
                            usize]).child[(k +
                            1 as
                                libc::c_int)
                            as usize] =
                            0 as Treeptr_B
                    }
                    j += 1
                }
                (*b_ptr).child[i as usize] = 0 as Treeptr_B
            }
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while !(*b_ptr).child[i as usize].is_null() {
        para_modifier_expand((*b_ptr).child[i as usize]);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn incomplete_para_expand(mut sp: *mut SENTENCE_DATA,
                                                mut b_ptr: *mut BNST_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut para_pos: libc::c_int = 0;
    let mut new_num: libc::c_int = 0;
    let mut child_num: libc::c_int = 0;
    let mut para_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut new_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut pre_childs: [*mut BNST_DATA; 10] = [0 as *mut BNST_DATA; 10];
    let mut pos_childs: [*mut BNST_DATA; 10] = [0 as *mut BNST_DATA; 10];
    /* 部分並列の展開 : コピー有(述語が新データ，もとの述語はPARAに) */
    para_pos = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while !(*b_ptr).child[i as usize].is_null() {
        if (*(*b_ptr).child[i as usize]).para_top_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            j = 0 as libc::c_int;
            while !(*(*b_ptr).child[i as usize]).child[j as usize].is_null() {
                if (*(*(*b_ptr).child[i as
                    usize]).child[j as usize]).para_type
                    as libc::c_int == 2 as libc::c_int {
                    para_pos = i;
                    break;
                } else { j += 1 }
            }
        }
        i += 1
    }
    if para_pos != -(1 as libc::c_int) {
        /* もとの修飾要素をストック */
        i = 0 as libc::c_int; /* コピー */
        while !(*b_ptr).child[i as usize].is_null() && i < para_pos {
            pre_childs[i as usize] =
                (*b_ptr).child[i as
                    usize]; /* 注意！！ こうしないと後でSF */
            i += 1
        } /* 新ノードの親(自分自身) */
        pre_childs[i as usize] =
            0 as *mut BNST_DATA; /* 元ノードは PARA */
        i = para_pos + 1 as libc::c_int;
        j = 0 as libc::c_int;
        while !(*b_ptr).child[i as usize].is_null() {
            pos_childs[j as usize] = (*b_ptr).child[i as usize];
            i += 1;
            j += 1
        }
        pos_childs[j as usize] = 0 as *mut BNST_DATA;
        para_ptr = (*b_ptr).child[para_pos as usize];
        new_num = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while !(*para_ptr).child[i as usize].is_null() {
            if (*(*para_ptr).child[i as usize]).para_type as libc::c_int ==
                1 as libc::c_int {
                new_ptr =
                    (*sp).bnst_data.offset((*sp).Bnst_num as
                        isize).offset((*sp).New_Bnst_num
                        as isize);
                (*sp).New_Bnst_num += 1;
                if (*sp).Bnst_num + (*sp).New_Bnst_num > 200 as libc::c_int {
                    fprintf(stderr,
                            b";; Too many nodes in expanding incomplete para .\n\x00"
                                as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                *new_ptr = *b_ptr;
                (*new_ptr).f = 0 as FEATUREptr;
                (*new_ptr).parent = b_ptr;
                (*b_ptr).child[new_num as usize] = new_ptr;
                /* 新しいノードの子を設定
		   (後ろの修飾ノード，<P>，<I>, 前の修飾ノード) */
                child_num = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while !pre_childs[j as usize].is_null() {
                    let fresh14 = child_num;
                    child_num = child_num + 1;
                    (*new_ptr).child[fresh14 as usize] =
                        pre_childs[j as usize];
                    j += 1
                }
                let fresh15 = child_num;
                child_num = child_num + 1;
                (*new_ptr).child[fresh15 as usize] =
                    (*para_ptr).child[i as usize];
                while !(*para_ptr).child[(i + 1 as libc::c_int) as
                    usize].is_null() &&
                    (*(*para_ptr).child[(i + 1 as libc::c_int) as
                        usize]).para_type as
                        libc::c_int == 2 as libc::c_int {
                    let fresh16 = child_num;
                    child_num = child_num + 1;
                    (*new_ptr).child[fresh16 as usize] =
                        (*para_ptr).child[(i + 1 as libc::c_int) as usize];
                    i += 1
                }
                j = 0 as libc::c_int;
                while !pos_childs[j as usize].is_null() {
                    let fresh17 = child_num;
                    child_num = child_num + 1;
                    (*new_ptr).child[fresh17 as usize] =
                        pos_childs[j as usize];
                    j += 1
                }
                let fresh18 = child_num;
                child_num = child_num + 1;
                (*new_ptr).child[fresh18 as usize] = 0 as Treeptr_B;
                new_ptr = new_ptr.offset(1);
                new_num += 1
            }
            i += 1
        }
        (*b_ptr).child[new_num as usize] = 0 as Treeptr_B;
        (*b_ptr).para_top_p =
            (0 as libc::c_int == 0) as libc::c_int as libc::c_char
    }
    i = 0 as libc::c_int;
    while !(*b_ptr).child[i as usize].is_null() {
        incomplete_para_expand(sp, (*b_ptr).child[i as usize]);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_dpnd_tree(mut sp: *mut SENTENCE_DATA)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0; /* 初期化 */
    init_bnst_tree_property(sp);
    (*sp).New_Bnst_num = 0 as libc::c_int;
    if make_simple_tree(sp) == 0 as libc::c_int {
        /* リンク付け */
        return 0 as libc::c_int;
    }
    if OptExpandP == (0 as libc::c_int == 0) as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Para_M_num {
            /* 強並列の展開 */
            if (*(*sp).para_manager.offset(i as isize)).parent.is_null() {
                strong_para_expand(sp, (*sp).para_manager.offset(i as isize));
            }
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        /* PARAの展開 */
        if (*(*sp).para_manager.offset(i as isize)).parent.is_null() {
            if para_top_expand(sp, (*sp).para_manager.offset(i as isize)) ==
                0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        i += 1
    }
    if OptExpandP == (0 as libc::c_int == 0) as libc::c_int {
        para_modifier_expand((*sp).bnst_data.offset((*sp).Bnst_num as
            isize).offset(-(1 as
            libc::c_int
            as
            isize)));
        /* PARA修飾の展開 */
    }
    /*
    incomplete_para_expand(sp->bnst_data + sp->Bnst_num - 1);*/
    /* 部分並列の展開 */
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_info_to_tag(mut bp: *mut BNST_DATA,
                                          mut tp: *mut TAG_DATA)
/*==================================================================*/
{
    (*tp).para_num = (*bp).para_num;
    (*tp).para_key_type = (*bp).para_key_type;
    (*tp).para_top_p = (*bp).para_top_p;
    (*tp).para_type = (*bp).para_type;
    (*tp).to_para_p = (*bp).to_para_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn para_info_to_mrph(mut bp: *mut BNST_DATA,
                                           mut mp: *mut MRPH_DATA)
/*==================================================================*/
{
    (*mp).para_num = (*bp).para_num;
    (*mp).para_key_type = (*bp).para_key_type;
    (*mp).para_top_p = (*bp).para_top_p;
    (*mp).para_type = (*bp).para_type;
    (*mp).to_para_p = (*bp).to_para_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_head_tag_from_bnst(mut bp: *mut BNST_DATA,
                                                 mut target_offset:
                                                 libc::c_int)
                                                 -> libc::c_int
/*==================================================================*/
{
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut gov: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    cp =
        check_feature((*bp).f,
                      b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() ||
        {
            cp =
                check_feature((*bp).f,
                              b"\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            !cp.is_null()
        } {
        cp2 = strchr(cp, ':' as i32);
        if !cp2.is_null() {
            offset = atoi(cp2.offset(1 as libc::c_int as isize));
            if offset > 0 as libc::c_int ||
                (*bp).tag_num <= -(1 as libc::c_int) * offset {
                offset = 0 as libc::c_int
            }
        }
    }
    gov = (*bp).tag_num - 1 as libc::c_int + offset;
    while gov >= 0 as libc::c_int {
        if (*(*bp).tag_ptr.offset(gov as isize)).num != -(1 as libc::c_int) {
            if target_offset <= 0 as libc::c_int { break; }
            target_offset -= 1
        }
        gov -= 1
    }
    return gov;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_head_tag_from_dpnd_bnst(mut bp: *mut BNST_DATA)
                                                      -> libc::c_int
/*==================================================================*/
{
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut gov: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 「タグ単位受無視」のときは係り先を最後のタグ単位とする */
    if check_feature((*bp).f,
                     b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\xe7\x84\xa1\xe8\xa6\x96\x00"
                         as *const u8 as *const libc::c_char as
                         *mut libc::c_char).is_null() &&
        {
            cp =
                check_feature((*(*bp).parent).f,
                              b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            (!cp.is_null()) ||
                {
                    cp =
                        check_feature((*(*bp).parent).f,
                                      b"\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                    !cp.is_null()
                }
        } {
        cp2 = strchr(cp, ':' as i32);
        if !cp2.is_null() {
            offset = atoi(cp2.offset(1 as libc::c_int as isize));
            if offset > 0 as libc::c_int ||
                (*(*bp).parent).tag_num <= -(1 as libc::c_int) * offset {
                offset = 0 as libc::c_int
            }
        }
    }
    gov = (*(*bp).parent).tag_num - 1 as libc::c_int + offset;
    while gov >= 0 as libc::c_int {
        if (*(*(*bp).parent).tag_ptr.offset(gov as isize)).num !=
            -(1 as libc::c_int) {
            break;
        }
        gov -= 1
    }
    return gov;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn find_head_mrph_from_dpnd_bnst(mut dep_ptr:
                                                       *mut BNST_DATA,
                                                       mut gov_ptr:
                                                       *mut BNST_DATA)
                                                       -> *mut MRPH_DATA
/*==================================================================*/
{
    // let mut bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* 係り先に判定詞があり、係り元が連用なら、係り先形態素を主辞名詞ではなく判定詞にする */
    return if !dep_ptr.is_null() &&
        (*gov_ptr).head_ptr.offset(1 as libc::c_int as isize) <=
            (*gov_ptr).mrph_ptr.offset((*gov_ptr).mrph_num as
                isize).offset(-(1 as libc::c_int
                as isize))
        &&
        !check_feature((*gov_ptr).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        strcmp(Class[(*(*gov_ptr).head_ptr.offset(1 as libc::c_int as
            isize)).Hinshi as
            usize][0 as libc::c_int as usize].id as
                   *const libc::c_char,
               b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as *const u8 as
                   *const libc::c_char) == 0 &&
        !(!check_feature((*dep_ptr).f,
                         b"\xe9\x80\xa3\xe4\xbd\x93\xe4\xbf\xae\xe9\xa3\xbe\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() ||
            !check_feature((*dep_ptr).f,
                           b"\xe4\xbf\x82:\xe9\x9a\xa3\x00" as *const u8
                               as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            !check_feature((*dep_ptr).f,
                           b"\xe4\xbf\x82:\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
            (*dep_ptr).dpnd_type as libc::c_int == 'P' as i32 &&
                !check_feature((*dep_ptr).f,
                               b"\xe4\xb8\xa6\xe3\x82\xad:\xe5\x90\x8d\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
        //	!(dep_ptr->para_type == PARA_NIL || /* 並列のときは最後から2番目の要素のみ修正 */
//	  ((bp = (BNST_DATA *)search_nearest_para_child((TAG_DATA *)dep_ptr->parent)) && dep_ptr->num == bp->num))) {
        (*gov_ptr).head_ptr.offset(1 as libc::c_int as isize)
    } else if !dep_ptr.is_null() &&
        (*gov_ptr).head_ptr.offset(-(1 as libc::c_int as isize)) >=
            (*gov_ptr).mrph_ptr &&
        !check_feature((*dep_ptr).f,
                       b"\xe4\xbf\x82:\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        !check_feature((*dep_ptr).f,
                       b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        check_feature((*dep_ptr).f,
                      b"\xe3\x82\xab\xe3\x82\xa6\xe3\x83\xb3\xe3\x82\xbf\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        !check_feature((*gov_ptr).f,
                       b"\xe3\x82\xab\xe3\x82\xa6\xe3\x83\xb3\xe3\x82\xbf\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() &&
        !check_feature((*(*gov_ptr).head_ptr.offset(-(1 as
            libc::c_int
            as
            isize))).f,
                       b"\xe6\x95\xb0\xe5\xad\x97\x00" as *const u8
                           as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        (*gov_ptr).head_ptr.offset(-(1 as libc::c_int as isize))
    } else { (*gov_ptr).head_ptr };
}
/* 係り元が裸の数量で、係り先にカウンタがあるなら、係り先形態素を主辞名詞ではなくその前の数詞にする 
     「１〜３個」など */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn bnst_to_tag_tree(mut sp: *mut SENTENCE_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut last_b_flag: libc::c_int = 1 as libc::c_int;
    let mut gov: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut gov_head: libc::c_int = 0;
    let mut pre_bp_num: libc::c_int = 0;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* 文節の木構造からタグ単位の木構造へ変換 */
    init_tag_tree_property(sp);
    (*sp).New_Tag_num = 0 as libc::c_int;
    /* new bnst -> tag */
    i = (*sp).New_Bnst_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        /* <PARA>(1)-<PARA>(2) のときのために後からする */
        bp =
            (*sp).bnst_data.offset((*sp).Bnst_num as
                isize).offset(i as isize);
        /* new領域にcopy */
        head = find_head_tag_from_bnst(bp, 0 as libc::c_int);
        if head < 0 as libc::c_int {
            /* 主辞基本句 */
            head = (*bp).tag_num - 1 as libc::c_int
        } /* New領域にコピーした主辞基本句へのポインタ */
        *(*sp).tag_data.offset((*sp).Tag_num as
            isize).offset((*sp).New_Tag_num as isize) =
            *(*bp).tag_ptr.offset(head as isize);
        (*sp).New_Tag_num += 1;
        tp =
            (*sp).tag_data.offset((*sp).Tag_num as
                isize).offset((*sp).New_Tag_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize));
        para_info_to_tag(bp, tp);
        (*tp).child[0 as libc::c_int as usize] = 0 as *mut tnode_t;
        /* <PARA>のときはheadのみ */
        if (*bp).para_top_p as libc::c_int == 0 as libc::c_int {
            /* 文節内の主辞基本句より前側 */
            if head > 0 as libc::c_int &&
                {
                    pre_bp_num =
                        find_head_tag_from_bnst(bp, 1 as libc::c_int);
                    (pre_bp_num) >= 0 as libc::c_int
                } {
                /* 文節内タグ単位の親が <P>(-<PARA>) のとき */
                let ref mut fresh19 =
                    (*(*bp).tag_ptr.offset(pre_bp_num as
                        isize)).parent; /* 主辞のひとつ前 -> 主辞 */
                *fresh19 = tp;
                t_add_node(tp as *mut BNST_DATA,
                           (*bp).tag_ptr.offset(pre_bp_num as isize) as
                               *mut BNST_DATA, -(1 as libc::c_int));
                /* 主辞基本句は bp->tag_ptr からはたどれない (Newの方) */
                j = 0 as libc::c_int;
                while j < pre_bp_num {
                    gov = j + 1 as libc::c_int;
                    while gov <= pre_bp_num {
                        if (*(*bp).tag_ptr.offset(gov as isize)).num !=
                            -(1 as libc::c_int) {
                            break;
                        }
                        gov += 1
                    }
                    if !(gov > pre_bp_num ||
                        (*(*bp).tag_ptr.offset(j as isize)).num ==
                            -(1 as libc::c_int)) {
                        let ref mut fresh20 =
                            (*(*bp).tag_ptr.offset(j as isize)).parent;
                        *fresh20 = (*bp).tag_ptr.offset(gov as isize);
                        t_add_node((*bp).tag_ptr.offset(gov as isize) as
                                       *mut BNST_DATA,
                                   (*bp).tag_ptr.offset(j as isize) as
                                       *mut BNST_DATA, -(1 as libc::c_int));
                    }
                    /* 文節内 */
                    /* 後処理でマージされた基本句 */
                    j += 1
                }
            }
        }
        /* 親と子のリンクつけ (new) */
        gov_head =
            find_head_tag_from_dpnd_bnst(bp); /* 係り先の主辞基本句 */
        (*tp).parent =
            (*(*bp).parent).tag_ptr.offset(gov_head as isize); /* PARAへ */
        t_add_node((*(*bp).parent).tag_ptr.offset(gov_head as isize) as
                       *mut BNST_DATA, tp as *mut BNST_DATA,
                   -(1 as libc::c_int));
        /* 文節内の主辞基本句より後 (PARAから残りの基本句へ) */
        if (*bp).parent < (*sp).bnst_data.offset((*sp).Bnst_num as isize) {
            /* 親がNewのときはすでに設定している */
            tp = (*(*bp).parent).tag_ptr.offset(gov_head as isize);
            j = head + 1 as libc::c_int;
            while j < (*bp).tag_num {
                if !((*(*bp).tag_ptr.offset(j as isize)).num ==
                    -(1 as libc::c_int)) {
                    (*tp).parent = (*bp).tag_ptr.offset(j as isize);
                    t_add_node((*bp).tag_ptr.offset(j as isize) as
                                   *mut BNST_DATA, tp as *mut BNST_DATA,
                               -(1 as libc::c_int));
                    tp = (*bp).tag_ptr.offset(j as isize)
                }
                j += 1
            }
            (*tp).parent = 0 as *mut tnode_t
            /* 係り先未定のマーク */
        }
        /* PARAまたは基本句1つのときは、tag_ptrをNew側にしておく */
        if 1 as libc::c_int != 0 ||
            (*bp).para_top_p as libc::c_int ==
                (0 as libc::c_int == 0) as libc::c_int ||
            (*bp).tag_num == 1 as libc::c_int {
            (*bp).tag_ptr =
                (*sp).tag_data.offset((*sp).Tag_num as
                    isize).offset((*sp).New_Tag_num as
                    isize).offset(-(1
                    as
                    libc::c_int
                    as
                    isize));
            (*bp).tag_num = 1 as libc::c_int
        }
        i -= 1
    }
    /* orig */
    i = (*sp).Bnst_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        bp = (*sp).bnst_data.offset(i as isize);
        if !((*bp).num == -(1 as libc::c_int)) {
            head = find_head_tag_from_bnst(bp, 0 as libc::c_int);
            if head < 0 as libc::c_int {
                /* 主辞基本句 */
                head = (*bp).tag_num - 1 as libc::c_int
            }
            para_info_to_tag(bp, (*bp).tag_ptr.offset(head as isize));
            /* <PARA>のときはheadのみだが、tag_ptr, tag_numの変更はしない */
            if (*bp).para_top_p as libc::c_int == 0 as libc::c_int {
                /* 文節内 */
                j = 0 as libc::c_int;
                while j < (*bp).tag_num - 1 as libc::c_int {
                    gov = j + 1 as libc::c_int;
                    while gov < (*bp).tag_num {
                        if (*(*bp).tag_ptr.offset(gov as isize)).num !=
                            -(1 as libc::c_int) {
                            break;
                        }
                        gov += 1
                    }
                    if !(gov >= (*bp).tag_num ||
                        (*(*bp).tag_ptr.offset(j as isize)).num ==
                            -(1 as libc::c_int)) {
                        let ref mut fresh21 =
                            (*(*bp).tag_ptr.offset(j as isize)).parent;
                        *fresh21 = (*bp).tag_ptr.offset(gov as isize);
                        t_add_node((*bp).tag_ptr.offset(gov as isize) as
                                       *mut BNST_DATA,
                                   (*bp).tag_ptr.offset(j as isize) as
                                       *mut BNST_DATA, -(1 as libc::c_int));
                    }
                    /* 後処理でマージされた基本句 */
                    j += 1
                }
            }
            if last_b_flag != 0 {
                /* 最後の文節 (後処理があるので i == Bnst_num - 1 とは限らない) */
                last_b_flag = 0 as libc::c_int
            } else if !(*bp).parent.is_null() {
                head = (*bp).tag_num - 1 as libc::c_int;
                while head >= 0 as libc::c_int {
                    /* 親と子 */
                    /* 最後の基本句をさがす */
                    if (*(*bp).tag_ptr.offset(head as isize)).num !=
                        -(1 as libc::c_int) {
                        break;
                    }
                    head -= 1
                }
                tp = (*bp).tag_ptr.offset(head as isize);
                if (*bp).para_top_p as libc::c_int ==
                    (0 as libc::c_int == 0) as libc::c_int {
                    /* PARAの場合はnewの方で少し処理している場合がある */
                    while !(*tp).parent.is_null() { tp = (*tp).parent }
                } /* タグ単位内の係り先をルールから得る */
                offset = find_head_tag_from_dpnd_bnst(bp);
                (*tp).parent =
                    (*(*bp).parent).tag_ptr.offset(offset as isize);
                t_add_node((*(*bp).parent).tag_ptr.offset(offset as isize) as
                               *mut BNST_DATA, tp as *mut BNST_DATA,
                           -(1 as libc::c_int));
            } else if Language != 2 as libc::c_int {
                fprintf(stderr, b";; %s(%d)\'s parent doesn\'t exist!\n\x00" as *const u8 as *const libc::c_char, (*bp).Jiritu_Go.as_mut_ptr(), i);
            }
        }
        /* 後処理でマージされた文節 */
        i -= 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn bnst_to_mrph_tree(mut sp: *mut SENTENCE_DATA) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    // let mut offset: libc::c_int = 0;
    let mut last_b_flag: libc::c_int = 1 as libc::c_int;
    // let mut gov: libc::c_int = 0;
    // let mut head: libc::c_int = 0;
    // let mut gov_head: libc::c_int = 0;
    // let mut pre_bp_num: libc::c_int = 0;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut mp: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut tmp_mp: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut head_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    /* 文節の木構造から形態素の木構造へ変換 */
    init_mrph_tree_property(sp);
    (*sp).New_Mrph_num = 0 as libc::c_int;
    /* new bnst -> tag */
    i = (*sp).New_Bnst_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        /* <PARA>(1)-<PARA>(2) のときのために後からする */
        bp =
            (*sp).bnst_data.offset((*sp).Bnst_num as
                isize).offset(i as isize);
        // head_ptr = bp->mrph_ptr + bp->mrph_num - 1; // bp->head_ptr; /* ★主辞形態素★ */
        head_ptr =
            find_head_mrph_from_dpnd_bnst(0 as *mut BNST_DATA,
                                          bp); /* 主辞形態素 */
        /* new領域にcopy */
        *(*sp).mrph_data.offset((*sp).Mrph_num as
            isize).offset((*sp).New_Mrph_num as isize)
            = *head_ptr; /* 主辞形態素 */
        (*sp).New_Mrph_num +=
            1; /* New領域にコピーした主辞形態素へのポインタ */
        mp =
            (*sp).mrph_data.offset((*sp).Mrph_num as isize).offset((*sp).New_Mrph_num as isize).offset(-(1 as libc::c_int as isize));
        para_info_to_mrph(bp, mp);
        (*mp).child[0 as libc::c_int as usize] = 0 as Treeptr_B;
        /* <PARA>のときはheadのみ */
        if (*bp).para_top_p as libc::c_int == 0 as libc::c_int {
            /* 文節内の主辞形態素より前側 */
            if head_ptr > (*bp).mrph_ptr {
                /* 文節内形態素の親が <P>(-<PARA>) のとき */
                let ref mut fresh22 =
                    (*head_ptr.offset(-(1 as libc::c_int as
                        isize))).parent; /* 主辞のひとつ前 -> 主辞 */
                *fresh22 = mp as *mut BNST_DATA;
                t_add_node(mp as *mut BNST_DATA,
                           head_ptr.offset(-(1 as libc::c_int as isize)) as
                               *mut BNST_DATA, -(1 as libc::c_int));
                /* 文節内 */
                tmp_mp = head_ptr.offset(-(2 as libc::c_int as isize));
                while tmp_mp >= (*bp).mrph_ptr {
                    (*tmp_mp).parent =
                        tmp_mp.offset(1 as libc::c_int as isize) as
                            *mut BNST_DATA;
                    t_add_node(tmp_mp.offset(1 as libc::c_int as isize) as
                                   *mut BNST_DATA, tmp_mp as *mut BNST_DATA,
                               -(1 as libc::c_int));
                    tmp_mp = tmp_mp.offset(-1)
                }
            }
        }
        /* 親と子のリンクつけ (new) */
        (*mp).parent =
            find_head_mrph_from_dpnd_bnst(bp, (*bp).parent) as
                *mut BNST_DATA; /* 係り先の主辞形態素 (PARAへ) */
        t_add_node((*mp).parent as *mut BNST_DATA, mp as *mut BNST_DATA,
                   -(1 as libc::c_int));
        /* 文節内の主辞形態素より後 (PARAから残りの基本句へ) */
        if (*bp).parent < (*sp).bnst_data.offset((*sp).Bnst_num as isize) {
            /* 親がNewのときはすでに設定している */
            mp = (*mp).parent as *mut MRPH_DATA;
            tmp_mp = head_ptr.offset(1 as libc::c_int as isize);
            while tmp_mp < (*bp).mrph_ptr.offset((*bp).mrph_num as isize) {
                (*mp).parent = tmp_mp as *mut BNST_DATA; /* PARA */
                t_add_node(tmp_mp as *mut BNST_DATA, mp as *mut BNST_DATA,
                           -(1 as libc::c_int));
                mp = tmp_mp;
                tmp_mp = tmp_mp.offset(1)
            }
            (*mp).parent = 0 as Treeptr_B
            /* 係り先未定のマーク */
        }
        /* mrph_ptrをNew側にしておく */
        (*bp).mrph_ptr =
            (*sp).mrph_data.offset((*sp).Mrph_num as
                isize).offset((*sp).New_Mrph_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize));
        (*bp).head_ptr = (*bp).mrph_ptr;
        (*bp).mrph_num = 1 as libc::c_int;
        i -= 1
    }
    /* orig */
    i = (*sp).Bnst_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        bp = (*sp).bnst_data.offset(i as isize);
        if !((*bp).num == -(1 as libc::c_int)) {
            if (*bp).para_type as libc::c_int != 0 as libc::c_int {
                head_ptr =
                    (*bp).mrph_ptr.offset((*bp).mrph_num as
                        isize).offset(-(1 as libc::c_int
                        as isize))
            } else {
                head_ptr =
                    find_head_mrph_from_dpnd_bnst(0 as *mut BNST_DATA, bp)
                /* 主辞形態素 */
            }
            para_info_to_mrph(bp, head_ptr);
            /* <PARA>のときはheadのみだが、tag_ptr, tag_numの変更はしない */
            if (*bp).para_top_p as libc::c_int == 0 as libc::c_int {
                /* 文節内 */
                tmp_mp =
                    (*bp).mrph_ptr.offset((*bp).mrph_num as
                        isize).offset(-(2 as libc::c_int
                        as isize));
                while tmp_mp >= (*bp).mrph_ptr {
                    /* 最終形態素の1つ前以前 */
                    (*tmp_mp).parent =
                        tmp_mp.offset(1 as libc::c_int as isize) as
                            *mut BNST_DATA;
                    t_add_node(tmp_mp.offset(1 as libc::c_int as isize) as
                                   *mut BNST_DATA, tmp_mp as *mut BNST_DATA,
                               -(1 as libc::c_int));
                    tmp_mp = tmp_mp.offset(-1)
                }
            }
            if last_b_flag != 0 {
                /* 最後の文節 (後処理があるので i == Bnst_num - 1 とは限らない) */
                last_b_flag = 0 as libc::c_int
            } else if !(*bp).parent.is_null() {
                /* 親と子 */
                mp =
                    (*bp).mrph_ptr.offset((*bp).mrph_num as
                        isize).offset(-(1 as libc::c_int
                        as
                        isize)); /* 係り元: 最終形態素 */
                if (*bp).para_top_p as libc::c_int ==
                    (0 as libc::c_int == 0) as libc::c_int {
                    /* PARAの場合はnewの方で少し処理している場合がある */
                    while !(*mp).parent.is_null() {
                        mp = (*mp).parent as *mut MRPH_DATA
                    }
                } /* タグ単位内の係り先をルールから得る */
                (*mp).parent =
                    find_head_mrph_from_dpnd_bnst(bp, (*bp).parent) as
                        *mut BNST_DATA;
                t_add_node((*mp).parent as *mut BNST_DATA,
                           mp as *mut BNST_DATA, -(1 as libc::c_int));
            } else if Language != 2 as libc::c_int {
                fprintf(stderr,
                        b";; %s(%d)\'s parent doesn\'t exist!\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*bp).Jiritu_Go.as_mut_ptr(), i);
            }
        }
        /* 後処理でマージされた文節 */
        i -= 1
    }
    panic!("Reached end of non-void function without returning");
}
