#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;
use libc::tm;
use juman::getid;

use crate::{Class, Dpnd_matrix, ErrorComment, fflush, fprintf, fputs, free, juman, Mask_matrix, match_matrix, path_matrix, PM_Memo, Quote_matrix, restrict_matrix, SENTENCE_DATA, sprintf, sscanf, strcat, strcmp, strcpy, strlen, strncmp, strstr, WarningComment};
use crate::case_print::{print_case_result, print_pa_structure};
use crate::consts::VERBOSE1;
use crate::ctools::{assign_cfeature, check_feature, CLASS_num, exit, Form, fputc, getenv, katakana2hiragana, Language, localtime, malloc_data, OptAnalysis, OptArticle, OptCKY, OptCopula, OptDisplay, OptEllipsis, OptExpress, OptInput, OptNbest, OptNE, OptPostProcess, OptRecoverPerson, Outfp, PrintNum, realloc_data, stderr, strftime, strncpy, time, Type, VerboseLevel};
use crate::dpnd_analysis::{tag_bnst_postprocess, undo_tag_bnst_postprocess};
use crate::feature::{clear_feature, copy_feature, print_feature, print_feature2, print_some_feature};
use crate::juman::getid::{get_bunrui_id, get_hinsi_id};
use crate::read_rule::case2num;
use crate::structs::{MRPH_DATA, TOTAL_MGR};
use crate::tree_conv::{bnst_to_mrph_tree, bnst_to_tag_tree, init_bnst_tree_property, make_dpnd_tree};
use crate::types::{BNST_DATA, CF_PRED_MGR, FEATURE, PARA_DATA, PARA_MANAGER, size_t, TAG_DATA, time_t};

/*====================================================================

			     出力ルーチン

                                               S.Kurohashi 91. 6.25
                                               S.Kurohashi 93. 5.31

    $Id$
====================================================================*/
#[no_mangle]
pub static mut mrph_buffer: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut Sen_Num: libc::c_int = 1 as libc::c_int;
/* -table のときのみ使用する */
#[no_mangle]
pub static mut Tag_Num: libc::c_int = 1 as libc::c_int;
/* -table のときのみ使用する */
/* for printing Chinese parse tree */
#[no_mangle]
pub static mut bnst_dpnd: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut bnst_level: [libc::c_int; 200] = [0; 200];
#[no_mangle]
pub static mut bnst_word: [*mut libc::c_char; 200] =
    [0 as *const libc::c_char as *mut libc::c_char; 200];
#[no_mangle]
pub static mut bnst_pos: [*mut libc::c_char; 200] =
    [0 as *const libc::c_char as *mut libc::c_char; 200];
#[no_mangle]
pub static mut bnst_tree: [[*mut libc::c_char; 100]; 200] =
    [[0 as *const libc::c_char as *mut libc::c_char; 100]; 200];
#[no_mangle]
pub static mut bnst_inverse_tree: [[*mut libc::c_char; 200]; 100] =
    [[0 as *const libc::c_char as *mut libc::c_char; 200]; 100];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn pp2mrph(mut pp: *mut libc::c_char,
                                 mut pp_len: libc::c_int)
                                 -> *mut libc::c_char
/*==================================================================*/
{
    let mut hira_pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hinsi_id: libc::c_int = 0;
    if pp_len as libc::c_ulong ==
        strlen(b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8 as
            *const libc::c_char) &&
        strncmp(pp,
                b"\xe3\x82\xac\xef\xbc\x92\x00" as *const u8 as
                    *const libc::c_char, pp_len as libc::c_ulong) == 0 {
        pp_len =
            (pp_len as
                libc::c_ulong).wrapping_sub(strlen(b"\xef\xbc\x92\x00" as
                *const u8 as
                *const libc::c_char))
                as libc::c_int as libc::c_int
        /* ガ２ -> ガ */
    }
    sprintf(mrph_buffer.as_mut_ptr(), b"%.*s\x00" as *const u8 as *const libc::c_char, pp_len, pp);
    hira_pp = katakana2hiragana(mrph_buffer.as_mut_ptr() as *mut libc::c_uchar) as *mut libc::c_char;
    hinsi_id = get_hinsi_id(b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as *mut libc::c_uchar);
    sprintf(mrph_buffer.as_mut_ptr(), b"%s %s %s \xe5\x8a\xa9\xe8\xa9\x9e %d \xe6\xa0\xbc\xe5\x8a\xa9\xe8\xa9\x9e %d * 0 * 0 NIL\x00" as *const u8 as *const libc::c_char, hira_pp, hira_pp,
            hira_pp,
            hinsi_id,
            get_bunrui_id(b"\xe6\xa0\xbc\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as *mut libc::c_uchar, hinsi_id)
    );
    free(hira_pp as *mut libc::c_void);
    return mrph_buffer.as_mut_ptr();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn pos2symbol(mut hinshi: *mut libc::c_char,
                                    mut bunrui: *mut libc::c_char)
                                    -> libc::c_char
/*==================================================================*/
{
    if strcmp(hinshi,
              b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return ' ' as i32 as libc::c_char;
    } else {
        if strcmp(hinshi,
                  b"\xe5\x8b\x95\xe8\xa9\x9e\x00" as *const u8 as
                      *const libc::c_char) == 0 {
            return 'v' as i32 as libc::c_char;
        } else {
            if strcmp(hinshi,
                      b"\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\x00" as *const u8
                          as *const libc::c_char) == 0 {
                return 'j' as i32 as libc::c_char;
            } else {
                if strcmp(hinshi,
                          b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00" as
                              *const u8 as *const libc::c_char) == 0 {
                    return 'c' as i32 as libc::c_char;
                } else {
                    if strcmp(hinshi,
                              b"\xe5\x8a\xa9\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                                  *const u8 as *const libc::c_char) == 0 {
                        return 'x' as i32 as libc::c_char;
                    } else {
                        if strcmp(hinshi,
                                  b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as *const u8
                                      as *const libc::c_char) == 0 &&
                            strcmp(bunrui,
                                   b"\xe5\x9b\xba\xe6\x9c\x89\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                       as *const u8 as *const libc::c_char)
                                == 0 {
                            return 'N' as i32 as libc::c_char;
                        } else {
                            if strcmp(hinshi,
                                      b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as
                                          *const u8 as *const libc::c_char) ==
                                0 &&
                                strcmp(bunrui,
                                       b"\xe4\xba\xba\xe5\x90\x8d\x00" as
                                           *const u8 as
                                           *const libc::c_char) == 0 {
                                return 'J' as i32 as libc::c_char;
                            } else {
                                if strcmp(hinshi,
                                          b"\xe5\x90\x8d\xe8\xa9\x9e\x00" as
                                              *const u8 as
                                              *const libc::c_char) == 0 &&
                                    strcmp(bunrui,
                                           b"\xe5\x9c\xb0\xe5\x90\x8d\x00"
                                               as *const u8 as
                                               *const libc::c_char) == 0 {
                                    return 'C' as i32 as libc::c_char;
                                } else {
                                    if strcmp(hinshi,
                                              b"\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                                  as *const u8 as
                                                  *const libc::c_char) == 0 {
                                        return 'n' as i32 as libc::c_char;
                                    } else {
                                        if strcmp(hinshi,
                                                  b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) ==
                                            0 {
                                            return 'd' as i32 as libc::c_char;
                                        } else {
                                            if strcmp(hinshi,
                                                      b"\xe5\x89\xaf\xe8\xa9\x9e\x00"
                                                          as *const u8 as
                                                          *const libc::c_char)
                                                == 0 {
                                                return 'a' as i32 as
                                                    libc::c_char;
                                            } else {
                                                if strcmp(hinshi,
                                                          b"\xe5\x8a\xa9\xe8\xa9\x9e\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                                    == 0 {
                                                    return 'p' as i32 as
                                                        libc::c_char;
                                                } else {
                                                    if strcmp(hinshi,
                                                              b"\xe6\x8e\xa5\xe7\xb6\x9a\xe8\xa9\x9e\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char)
                                                        == 0 {
                                                        return 'c' as i32 as
                                                            libc::c_char;
                                                    } else {
                                                        if strcmp(hinshi,
                                                                  b"\xe9\x80\xa3\xe4\xbd\x93\xe8\xa9\x9e\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                                            == 0 {
                                                            return 'm' as i32
                                                                as
                                                                libc::c_char;
                                                        } else {
                                                            if strcmp(hinshi,
                                                                      b"\xe6\x84\x9f\xe5\x8b\x95\xe8\xa9\x9e\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char)
                                                                == 0 {
                                                                return '!' as
                                                                    i32
                                                                    as
                                                                    libc::c_char;
                                                            } else {
                                                                if strcmp(hinshi,
                                                                          b"\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char)
                                                                    == 0 {
                                                                    return 'p'
                                                                        as
                                                                        i32
                                                                        as
                                                                        libc::c_char;
                                                                } else {
                                                                    if strcmp(hinshi,
                                                                              b"\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char)
                                                                        ==
                                                                        0 {
                                                                        return 's'
                                                                            as
                                                                            i32
                                                                            as
                                                                            libc::c_char;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return '?' as i32 as libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_mrph(mut m_ptr: *mut MRPH_DATA)
/*==================================================================*/
{
    fprintf(Outfp, b"%s %s %s \x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Goi2.as_mut_ptr(), (*m_ptr).Yomi.as_mut_ptr(),
            (*m_ptr).Goi.as_mut_ptr());
    if Language == 1 as libc::c_int {
        if (*m_ptr).Hinshi >= CLASS_num {
            fputc('\n' as i32, Outfp);
            fprintf(stderr,
                    b";; Hinshi number is invalid. (%d)\n\x00" as *const u8 as
                        *const libc::c_char, (*m_ptr).Hinshi);
            exit(1 as libc::c_int);
        }
        fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
                Class[(*m_ptr).Hinshi as
                    usize][0 as libc::c_int as usize].id);
    } else { fprintf(Outfp, b"* \x00" as *const u8 as *const libc::c_char); }
    fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Hinshi);
    if Language == 1 as libc::c_int && (*m_ptr).Bunrui != 0 {
        fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
                Class[(*m_ptr).Hinshi as usize][(*m_ptr).Bunrui as usize].id);
    } else { fprintf(Outfp, b"* \x00" as *const u8 as *const libc::c_char); }
    fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Bunrui);
    if Language == 1 as libc::c_int && (*m_ptr).Katuyou_Kata != 0 {
        fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
                Type[(*m_ptr).Katuyou_Kata as usize].name);
    } else { fprintf(Outfp, b"* \x00" as *const u8 as *const libc::c_char); }
    fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Katuyou_Kata);
    if Language == 1 as libc::c_int && (*m_ptr).Katuyou_Kei != 0 {
        fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
                Form[(*m_ptr).Katuyou_Kata as
                    usize][(*m_ptr).Katuyou_Kei as usize].name);
    } else { fprintf(Outfp, b"* \x00" as *const u8 as *const libc::c_char); }
    fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Katuyou_Kei);
    fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Imi.as_mut_ptr());
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_mrph_f(mut m_ptr: *mut MRPH_DATA)
/*==================================================================*/
{
    let mut yomi_buffer: [libc::c_char; 128] = [0; 128];
    sprintf(yomi_buffer.as_mut_ptr(),
            b"(%s)\x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Yomi.as_mut_ptr());
    fprintf(Outfp,
            b"%-16.16s%-18.18s %-14.14s\x00" as *const u8 as
                *const libc::c_char, (*m_ptr).Goi2.as_mut_ptr(),
            yomi_buffer.as_mut_ptr(),
            Class[(*m_ptr).Hinshi as usize][(*m_ptr).Bunrui as usize].id);
    if (*m_ptr).Katuyou_Kata != 0 {
        fprintf(Outfp,
                b" %-14.14s %-12.12s\x00" as *const u8 as *const libc::c_char,
                Type[(*m_ptr).Katuyou_Kata as usize].name,
                Form[(*m_ptr).Katuyou_Kata as
                    usize][(*m_ptr).Katuyou_Kei as usize].name);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn search_nearest_para_child(mut bp: *mut TAG_DATA)
                                                   -> *mut TAG_DATA
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* 並列のときに、最後から2番目の要素をかえす */
    if (*bp).para_top_p != 0 {
        i = 1 as libc::c_int;
        while !(*bp).child[i as usize].is_null() {
            /* 0は最後の要素 */
            if (*(*bp).child[i as usize]).para_type as libc::c_int !=
                0 as libc::c_int {
                return (*bp).child[i as usize];
            }
            i += 1
        }
    }
    return 0 as *mut TAG_DATA;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_eos(mut eos_flag: libc::c_int)
/*==================================================================*/
{
    if eos_flag != 0 {
        fputs(b"EOS\n\x00" as *const u8 as *const libc::c_char, Outfp);
    } else {
        fputs(b"EOP\n\x00" as *const u8 as *const libc::c_char, Outfp);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_tags(mut sp: *mut SENTENCE_DATA,
                                    mut flag: libc::c_int,
                                    mut eos_flag: libc::c_int)
/*==================================================================*/
{
    /* 現在は常に flag == 1 (0は旧形式出力) */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut b_count: libc::c_int = 0 as libc::c_int;
    let mut case_len: libc::c_int = 0;
    let mut bp_independent_offset: libc::c_int = 0 as libc::c_int;
    let mut dpnd_head: libc::c_int = 0;
    let mut t_table: [libc::c_int; 200] = [0; 200];
    let mut b_table: [libc::c_int; 200] = [0; 200];
    let mut t_proj_table: [libc::c_int; 200] = [0; 200];
    let mut t_copula_table: [libc::c_int; 200] = [0; 200];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut pre_bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut bp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* ノードの挿入を考慮しながら、基本句、文節の変換テーブルを作成 */
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        if !((*t_ptr).num == -(1 as libc::c_int)) {
            /* 追加ノード */
            if OptRecoverPerson != 0 && pre_bp != (*t_ptr).b_ptr {
                /* 文節の切れ目ごとにチェック */
                fp =
                    (*(*(*t_ptr).b_ptr).tag_ptr.offset((*(*t_ptr).b_ptr).tag_num
                        as
                        isize).offset(-(1
                        as
                        libc::c_int
                        as
                        isize))).f; /* headの基本句 */
                while !fp.is_null() {
                    /* featureのloop: featureをチェック */
                    if strncmp((*fp).cp,
                               b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00" as
                                   *const u8 as *const libc::c_char,
                               strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                   as *const u8 as
                                   *const libc::c_char)) == 0 &&
                        !strstr((*fp).cp,
                                b":\xef\xbc\x83\x00" as *const u8 as
                                    *const libc::c_char).is_null() {
                        /* tag_after_dpnd_and_case.ruleで使われている */
                        t_copula_table[count as usize] = 0 as libc::c_int;
                        count += 1;
                        b_count += 1
                    }
                    fp = (*fp).next
                }
                pre_bp = (*t_ptr).b_ptr
            }
            /* 判定詞(-copula)の基本句を分解するとき */
            if !check_feature((*t_ptr).f,
                              b"\xef\xbc\xb4\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe5\x88\x86\xe8\xa7\xa3\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                t_copula_table[count as usize] = 0 as libc::c_int;
                count += 1;
                t_copula_table[count as usize] = 1 as libc::c_int
                /* 新しいテーブルにおける基本句分解の位置を記録 */
            } else {
                t_copula_table[count as usize] = 0 as libc::c_int
            } /* numを更新しているので使える */
            let fresh0 = count;
            count = count + 1;
            t_table[(*t_ptr).num as usize] = fresh0;
            if (*t_ptr).bnum >= 0 as libc::c_int {
                /* 文節行 (bnumを更新しているので使える) */
                let fresh1 = b_count;
                b_count = b_count + 1;
                b_table[(*t_ptr).bnum as usize] = fresh1
            }
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
        /* 後処理でマージされたタグ */
    }
    i = 0 as libc::c_int;
    while i < count {
        /* 非交差条件チェック用 */
        t_proj_table[i as usize] = 0 as libc::c_int;
        i += 1
    }
    count = 0 as libc::c_int;
    pre_bp = 0 as *mut BNST_DATA;
    i = 0 as libc::c_int;
    t_ptr = (*sp).tag_data;
    while i < (*sp).Tag_num {
        if !((*t_ptr).num == -(1 as libc::c_int)) {
            if flag == 1 as libc::c_int {
                bp_independent_offset = 0 as libc::c_int;
                if OptExpress == 16 as libc::c_int {
                    fprintf(Outfp,
                            b"%%%% LABEL=%d_%db\n\x00" as *const u8 as
                                *const libc::c_char,
                            Sen_Num - 1 as libc::c_int, i + 1 as libc::c_int);
                }
                /* 追加ノード */
                if OptRecoverPerson != 0 && pre_bp != (*t_ptr).b_ptr {
                    fp =
                        (*(*(*t_ptr).b_ptr).tag_ptr.offset((*(*t_ptr).b_ptr).tag_num
                            as
                            isize).offset(-(1
                            as
                            libc::c_int
                            as
                            isize))).f; /* headの基本句 */
                    while !fp.is_null() {
                        /* featureのloop: featureをチェック */
                        if strncmp((*fp).cp,
                                   b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                       as *const u8 as *const libc::c_char,
                                   strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                       as *const u8 as
                                       *const libc::c_char)) == 0 &&
                            {
                                cp =
                                    strstr((*fp).cp,
                                           b":\xef\xbc\x83\x00" as
                                               *const u8 as
                                               *const libc::c_char); /* 格の部分の長さ */
                                !cp.is_null()
                            } {
                            case_len =
                                (cp.wrapping_offset_from((*fp).cp) as
                                    libc::c_long as
                                    libc::c_ulong).wrapping_sub(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                    as
                                    *const u8
                                    as
                                    *const libc::c_char))
                                    as libc::c_int; /* ＃の頭 */
                            cp = cp.offset(1); /* 係り先はhead */
                            dpnd_head =
                                t_table[(*(*(*t_ptr).b_ptr).tag_ptr.offset((*(*t_ptr).b_ptr).tag_num
                                    as
                                    isize).offset(-(1
                                    as
                                    libc::c_int
                                    as
                                    isize))).num
                                    as usize];
                            if t_proj_table[count as usize] != 0 &&
                                dpnd_head > t_proj_table[count as usize] {
                                /* 非交差条件 */
                                dpnd_head = t_proj_table[count as usize]
                            }
                            if strncmp((*fp).cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                as *const u8
                                as
                                *const libc::c_char)
                                as isize),
                                       b"\xef\xbc\x83\x00" as *const u8 as
                                           *const libc::c_char,
                                       case_len as libc::c_ulong) == 0 {
                                fprintf(Outfp,
                                        b"* %dD <\xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x89\xe6\x8c\xbf\xe5\x85\xa5>\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b_table[(*(*t_ptr).b_ptr).num as
                                            usize]);
                                fprintf(Outfp,
                                        b"+ %dD <\xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x89\xe6\x8c\xbf\xe5\x85\xa5>\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, dpnd_head);
                                fprintf(Outfp,
                                        b"%s %s %s \xe5\x90\x8d\xe8\xa9\x9e 6 \xe6\x99\xae\xe9\x80\x9a\xe5\x90\x8d\xe8\xa9\x9e 1 * 0 * 0 NIL\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, cp, cp, cp);
                            } else {
                                fprintf(Outfp,
                                        b"* %dD <\xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x89\xe6\x8c\xbf\xe5\x85\xa5><\xe4\xbf\x82:%.*s\xe6\xa0\xbc>\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b_table[(*(*t_ptr).b_ptr).num as
                                            usize], case_len,
                                        (*fp).cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char)
                                            as isize));
                                fprintf(Outfp,
                                        b"+ %dD <\xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x89\xe6\x8c\xbf\xe5\x85\xa5><\xe4\xbf\x82:%.*s\xe6\xa0\xbc><\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc:%.*s>\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, dpnd_head,
                                        case_len,
                                        (*fp).cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char)
                                            as isize),
                                        case_len,
                                        (*fp).cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char)
                                            as isize));
                                fprintf(Outfp,
                                        b"%s %s %s \xe5\x90\x8d\xe8\xa9\x9e 6 \xe6\x99\xae\xe9\x80\x9a\xe5\x90\x8d\xe8\xa9\x9e 1 * 0 * 0 NIL\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, cp, cp, cp);
                                fprintf(Outfp,
                                        b"%s\n\x00" as *const u8 as
                                            *const libc::c_char,
                                        pp2mrph((*fp).cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa6\x81\xe7\xb4\xa0-\x00"
                                            as
                                            *const u8
                                            as
                                            *const libc::c_char)
                                            as isize),
                                                case_len));
                            }
                            count += 1
                        }
                        fp = (*fp).next
                    }
                    pre_bp = (*t_ptr).b_ptr
                }
                /* 判定詞(-copula)の基本句を分解するとき */
                if !check_feature((*t_ptr).f,
                                  b"\xef\xbc\xb4\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe5\x88\x86\xe8\xa7\xa3\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    if (*t_ptr).bnum >= 0 as libc::c_int {
                        /* 文節行 */
                        fprintf(Outfp,
                                b"* %d%c\x00" as *const u8 as
                                    *const libc::c_char,
                                if (*(*t_ptr).b_ptr).dpnd_head ==
                                    -(1 as libc::c_int) {
                                    -(1 as libc::c_int)
                                } else {
                                    b_table[(*(*t_ptr).b_ptr).dpnd_head as
                                        usize]
                                },
                                (*(*t_ptr).b_ptr).dpnd_type as libc::c_int);
                        if !(*(*t_ptr).b_ptr).f.is_null() {
                            fputc(' ' as i32, Outfp);
                            print_feature((*(*t_ptr).b_ptr).f, Outfp);
                        }
                        fputc('\n' as i32, Outfp);
                    }
                    fprintf(Outfp,
                            b"+ %dD <\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe5\x88\x86\xe8\xa7\xa3><\xe4\xbf\x82:\xe9\x9a\xa3>\n\x00"
                                as *const u8 as *const libc::c_char,
                            t_table[(*t_ptr).num as usize]);
                    j = 0 as libc::c_int;
                    m_ptr = (*t_ptr).mrph_ptr;
                    while j < (*t_ptr).mrph_num {
                        if !check_feature((*m_ptr).f,
                                          b"\xe5\xbe\x8c\xe5\x87\xa6\xe7\x90\x86-\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe5\xa7\x8b\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() {
                            break;
                        }
                        print_mrph(m_ptr);
                        if !(*m_ptr).f.is_null() {
                            fputc(' ' as i32, Outfp);
                            print_feature((*m_ptr).f, Outfp);
                        }
                        fputc('\n' as i32, Outfp);
                        bp_independent_offset += 1;
                        j += 1;
                        m_ptr = m_ptr.offset(1)
                    }
                    count += 1
                }
                /* 文節行 */
                if bp_independent_offset == 0 as libc::c_int &&
                    (*t_ptr).bnum >= 0 as libc::c_int {
                    if PrintNum != 0 {
                        fprintf(Outfp,
                                b"* %d %d%c\x00" as *const u8 as
                                    *const libc::c_char, (*t_ptr).bnum,
                                if (*(*t_ptr).b_ptr).dpnd_head ==
                                    -(1 as libc::c_int) {
                                    -(1 as libc::c_int)
                                } else {
                                    b_table[(*(*t_ptr).b_ptr).dpnd_head as
                                        usize]
                                },
                                (*(*t_ptr).b_ptr).dpnd_type as libc::c_int);
                    } else {
                        fprintf(Outfp,
                                b"* %d%c\x00" as *const u8 as
                                    *const libc::c_char,
                                if (*(*t_ptr).b_ptr).dpnd_head ==
                                    -(1 as libc::c_int) {
                                    -(1 as libc::c_int)
                                } else {
                                    b_table[(*(*t_ptr).b_ptr).dpnd_head as
                                        usize]
                                },
                                (*(*t_ptr).b_ptr).dpnd_type as libc::c_int);
                    }
                    if !(*(*t_ptr).b_ptr).f.is_null() {
                        fputc(' ' as i32, Outfp);
                        print_feature((*(*t_ptr).b_ptr).f, Outfp);
                    }
                    if OptExpress == 16 as libc::c_int {
                        fprintf(Outfp,
                                b"<BR><BR>\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    fputc('\n' as i32, Outfp);
                }
                /* 判定詞分解時: 連体修飾は判定詞の前の名詞に係るように修正 */
                dpnd_head =
                    if (*t_ptr).dpnd_head == -(1 as libc::c_int) {
                        -(1 as libc::c_int)
                    } else { t_table[(*t_ptr).dpnd_head as usize] };
                if OptCopula != 0 && dpnd_head != -(1 as libc::c_int) &&
                    t_copula_table[dpnd_head as usize] != 0 {
                    /* 係り先が判定詞分解 */
                    if t_table[(*t_ptr).num as usize] <
                        dpnd_head - 1 as libc::c_int &&
                        ((!check_feature((*t_ptr).f,
                                         b"\xe9\x80\xa3\xe4\xbd\x93\xe4\xbf\xae\xe9\xa3\xbe\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null()
                            ||
                            !check_feature((*t_ptr).f,
                                           b"\xe4\xbf\x82:\xe9\x9a\xa3\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null()
                            ||
                            !check_feature((*t_ptr).f,
                                           b"\xe4\xbf\x82:\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null())
                            &&
                            ((*t_ptr).para_type as libc::c_int ==
                                0 as libc::c_int ||
                                {
                                    bp =
                                        search_nearest_para_child((*t_ptr).parent);
                                    (!bp.is_null()) &&
                                        (*t_ptr).num == (*bp).num
                                }) ||
                            t_proj_table[t_table[(*t_ptr).num as usize] as
                                usize] != 0 &&
                                dpnd_head >
                                    t_proj_table[t_table[(*t_ptr).num as
                                        usize] as
                                        usize]) {
                        /* 非交差条件 */
                        dpnd_head -= 1
                    }
                }
                if PrintNum != 0 {
                    fprintf(Outfp,
                            b"+ %d %d%c\x00" as *const u8 as
                                *const libc::c_char, (*t_ptr).num, dpnd_head,
                            (*t_ptr).dpnd_type as libc::c_int);
                } else {
                    fprintf(Outfp,
                            b"+ %d%c\x00" as *const u8 as *const libc::c_char,
                            dpnd_head, (*t_ptr).dpnd_type as libc::c_int);
                }
                if !(*t_ptr).f.is_null() {
                    fputc(' ' as i32, Outfp);
                    print_feature((*t_ptr).f, Outfp);
                }
                if OptExpress == 16 as libc::c_int {
                    fprintf(Outfp,
                            b"<BR><BR>\x00" as *const u8 as
                                *const libc::c_char);
                }
                fputc('\n' as i32, Outfp);
                j = t_table[(*t_ptr).num as usize];
                while j < dpnd_head {
                    if t_proj_table[j as usize] == 0 ||
                        t_proj_table[j as usize] > dpnd_head {
                        t_proj_table[j as usize] = dpnd_head
                    }
                    j += 1
                }
            } else {
                fprintf(Outfp,
                        b"%c\n\x00" as *const u8 as *const libc::c_char,
                        if (*t_ptr).bnum < 0 as libc::c_int {
                            '+' as i32
                        } else { '*' as i32 });
            }
            j = bp_independent_offset;
            m_ptr = (*t_ptr).mrph_ptr.offset(bp_independent_offset as isize);
            while j < (*t_ptr).mrph_num {
                print_mrph(m_ptr);
                if !(*m_ptr).f.is_null() {
                    fputc(' ' as i32, Outfp);
                    print_feature((*m_ptr).f, Outfp);
                }
                if OptExpress == 16 as libc::c_int {
                    fprintf(Outfp,
                            b"<BR><BR>\x00" as *const u8 as
                                *const libc::c_char);
                }
                fputc('\n' as i32, Outfp);
                j += 1;
                m_ptr = m_ptr.offset(1)
            }
            count += 1
        }
        i += 1;
        t_ptr = t_ptr.offset(1)
        /* 後処理でマージされたタグ */
    }
    print_eos(eos_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_mrphs(mut sp: *mut SENTENCE_DATA,
                                     mut eos_flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut bp_f: *mut FEATURE = 0 as *mut FEATURE;
    let mut bp_copied_f: *mut FEATURE = 0 as *mut FEATURE;
    i = 0 as libc::c_int;
    m_ptr = (*sp).mrph_data;
    while i < (*sp).Mrph_num {
        /* 基本句行 */
        if (*m_ptr).tnum >= 0 as libc::c_int {
            t_ptr = (*sp).tag_data.offset((*m_ptr).tnum as isize);
            /* 文節行 */
            if (*t_ptr).bnum >= 0 as libc::c_int {
                if PrintNum != 0 {
                    fprintf(Outfp,
                            b"* %d %d%c\x00" as *const u8 as
                                *const libc::c_char, (*t_ptr).bnum,
                            if (*(*t_ptr).b_ptr).dpnd_head ==
                                -(1 as libc::c_int) {
                                -(1 as libc::c_int)
                            } else { (*(*t_ptr).b_ptr).dpnd_head },
                            (*(*t_ptr).b_ptr).dpnd_type as libc::c_int);
                } else {
                    fprintf(Outfp,
                            b"* %d%c\x00" as *const u8 as *const libc::c_char,
                            if (*(*t_ptr).b_ptr).dpnd_head ==
                                -(1 as libc::c_int) {
                                -(1 as libc::c_int)
                            } else { (*(*t_ptr).b_ptr).dpnd_head },
                            (*(*t_ptr).b_ptr).dpnd_type as libc::c_int);
                }
                if !(*(*t_ptr).b_ptr).f.is_null() {
                    fputc(' ' as i32, Outfp);
                    print_feature((*(*t_ptr).b_ptr).f, Outfp);
                }
                fputc('\n' as i32, Outfp);
            }
            if PrintNum != 0 {
                fprintf(Outfp,
                        b"+ %d %d%c\x00" as *const u8 as *const libc::c_char,
                        (*m_ptr).tnum, (*t_ptr).dpnd_head,
                        (*t_ptr).dpnd_type as libc::c_int);
            } else {
                fprintf(Outfp,
                        b"+ %d%c\x00" as *const u8 as *const libc::c_char,
                        (*t_ptr).dpnd_head,
                        (*t_ptr).dpnd_type as libc::c_int);
            }
            if !(*t_ptr).f.is_null() {
                fputc(' ' as i32, Outfp);
                print_feature((*t_ptr).f, Outfp);
                bp_f = (*t_ptr).f
            }
            fputc('\n' as i32, Outfp);
        }
        /* 形態素係り受け行 */
        if PrintNum != 0 {
            fprintf(Outfp,
                    b"- %d %d%c\x00" as *const u8 as *const libc::c_char,
                    (*m_ptr).num, (*m_ptr).dpnd_head,
                    (*m_ptr).dpnd_type as libc::c_int);
        } else {
            fprintf(Outfp, b"- %d%c\x00" as *const u8 as *const libc::c_char,
                    (*m_ptr).dpnd_head, (*m_ptr).dpnd_type as libc::c_int);
        }
        /* 形態素featureは以下の形態素行に出力するので省略 */
        fputc('\n' as i32, Outfp);
        /* 形態素情報 */
        print_mrph(m_ptr);
        /* 基本句headの形態素に基本句のfeatureを付与 */
        if (*m_ptr).out_head_flag != 0 {
            if !bp_f.is_null() {
                fputc(' ' as i32, Outfp);
                if !(*m_ptr).f.is_null() {
                    /* 形態素自身のfeature -> bp_fとマージ */
                    bp_copied_f =
                        0 as
                            *mut FEATURE; /* bp_f中の正規化代表表記などを上書き */
                    copy_feature(&mut bp_copied_f, bp_f);
                    copy_feature(&mut bp_copied_f, (*m_ptr).f);
                    print_feature(bp_copied_f, Outfp);
                    clear_feature(&mut bp_copied_f);
                } else { print_feature(bp_f, Outfp); }
                bp_f = 0 as *mut FEATURE
            }
        } else {
            fputs(b" <\xe4\xbf\x82:\xe5\x9f\xba\xe6\x9c\xac\xe5\x8f\xa5\xe5\x86\x85>\x00"
                      as *const u8 as *const libc::c_char, Outfp);
            if !(*m_ptr).f.is_null() {
                /* 形態素自身のfeature */
                print_feature((*m_ptr).f, Outfp);
            }
        }
        fputc('\n' as i32, Outfp);
        i += 1;
        m_ptr = m_ptr.offset(1)
    }
    print_eos(eos_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_mrphs_only(mut sp: *mut SENTENCE_DATA,
                                          mut eos_flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        print_mrph((*sp).mrph_data.offset(i as isize));
        if !(*(*sp).mrph_data.offset(i as isize)).f.is_null() {
            fprintf(Outfp, b" \x00" as *const u8 as *const libc::c_char);
            print_feature((*(*sp).mrph_data.offset(i as isize)).f, Outfp);
        }
        fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    print_eos(eos_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_bnst_with_mrphs(mut sp: *mut SENTENCE_DATA,
                                               mut have_dpnd_flag:
                                               libc::c_int,
                                               mut eos_flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if !((*b_ptr).num == -(1 as libc::c_int)) {
            if have_dpnd_flag == 1 as libc::c_int {
                if Language == 2 as libc::c_int &&
                    ((*b_ptr).is_para == 1 as libc::c_int ||
                        (*b_ptr).is_para == 2 as libc::c_int) {
                    fprintf(Outfp,
                            b"* %dP\x00" as *const u8 as *const libc::c_char,
                            (*b_ptr).dpnd_head);
                } else {
                    fprintf(Outfp,
                            b"* %d%c\x00" as *const u8 as *const libc::c_char,
                            (*b_ptr).dpnd_head,
                            (*b_ptr).dpnd_type as libc::c_int);
                }
                if !(*b_ptr).f.is_null() {
                    fprintf(Outfp,
                            b" \x00" as *const u8 as *const libc::c_char);
                    print_feature((*b_ptr).f, Outfp);
                }
                fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(Outfp,
                        b"*\n\x00" as *const u8 as *const libc::c_char);
            }
            j = 0 as libc::c_int;
            m_ptr = (*b_ptr).mrph_ptr;
            while j < (*b_ptr).mrph_num {
                print_mrph(m_ptr);
                if !(*m_ptr).f.is_null() {
                    fprintf(Outfp,
                            b" \x00" as *const u8 as *const libc::c_char);
                    print_feature((*m_ptr).f, Outfp);
                }
                /* print_mrph_f(m_ptr); */
                fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
                j += 1;
                m_ptr = m_ptr.offset(1)
            }
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
        /* 後処理でマージされた文節 */
    }
    print_eos(eos_flag);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_all_result(mut sp: *mut SENTENCE_DATA,
                                          mut eos_flag: libc::c_int)
/*==================================================================*/
{
    if OptAnalysis == 5 as libc::c_int {
        print_mrphs_only(sp, eos_flag);
    } else if OptAnalysis == 3 as libc::c_int {
        print_bnst_with_mrphs(sp, 0 as libc::c_int, eos_flag);
    } else if OptNbest == 0 as libc::c_int &&
        !(OptArticle != 0 && OptEllipsis != 0) {
        print_result(sp, 1 as libc::c_int, eos_flag);
    }
    if Language == 2 as libc::c_int { print_tree_for_chinese(sp); }
    fflush(Outfp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn _print_bnst(mut ptr: *mut TAG_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if !ptr.is_null() {
        i = 0 as libc::c_int;
        while i < (*ptr).mrph_num {
            fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                    (*(*ptr).mrph_ptr.offset(i as isize)).Goi2.as_mut_ptr());
            i += 1
        }
    } else {
        fprintf(Outfp,
                b"\xe4\xb8\x8d\xe7\x89\xb9\xe5\xae\x9a:\xe4\xba\xba\x00" as
                    *const u8 as *const libc::c_char);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_mrph_with_para(mut ptr: *mut MRPH_DATA,
                                              mut cp: *mut libc::c_char)
/*==================================================================*/
{
    // let mut i: libc::c_int = 0;
    if !cp.is_null() && !ptr.is_null() {
        if OptExpress == 16 as libc::c_int {
            if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
                strcpy(cp,
                       b"\xef\xbc\x9cP\xef\xbc\x9e\x00" as *const u8 as
                           *const libc::c_char);
            } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
                strcpy(cp,
                       b"&lt;I&gt;\x00" as *const u8 as *const libc::c_char);
            } else {
                *cp.offset(0 as libc::c_int as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
        } else if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
            strcpy(cp, b"<P>\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
            strcpy(cp, b"<I>\x00" as *const u8 as *const libc::c_char);
        } else {
            *cp.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        if (*ptr).para_top_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            strcat(cp, b"PARA\x00" as *const u8 as *const libc::c_char);
        } else { strcpy(cp, (*ptr).Goi2.as_mut_ptr()); }
    } else if cp.is_null() && !ptr.is_null() {
        if (*ptr).para_top_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            fprintf(Outfp, b"PARA\x00" as *const u8 as *const libc::c_char);
        } else {
            if OptExpress == 16 as libc::c_int {
                let fresh2 = Tag_Num;
                Tag_Num = Tag_Num + 1;
                fprintf(Outfp,
                        b"%%%% %d %d 1 LABEL=%d_%db align=right style=white-space:nowrap\n\x00"
                            as *const u8 as *const libc::c_char, Sen_Num,
                        fresh2, Sen_Num, Tag_Num - 1 as libc::c_int);
            }
            fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                    (*ptr).Goi2.as_mut_ptr());
            if Language == 1 as libc::c_int && OptDisplay != 1 as libc::c_int
                && OptDisplay != 5 as libc::c_int {
                fprintf(Outfp, b"%c\x00" as *const u8 as *const libc::c_char,
                        pos2symbol(Class[(*ptr).Hinshi as
                            usize][0 as libc::c_int as
                            usize].id as
                                       *mut libc::c_char,
                                   Class[(*ptr).Hinshi as
                                       usize][(*ptr).Bunrui as usize].id
                                       as *mut libc::c_char) as libc::c_int);
            }
        }
        if OptExpress == 16 as libc::c_int {
            if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
                fprintf(Outfp,
                        b"\xef\xbc\x9cP\xef\xbc\x9e\x00" as *const u8 as
                            *const libc::c_char);
            } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
                fprintf(Outfp,
                        b"&lt;I&gt;\x00" as *const u8 as *const libc::c_char);
            }
        } else if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
            fprintf(Outfp, b"<P>\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
            fprintf(Outfp, b"<I>\x00" as *const u8 as *const libc::c_char);
        }
        if (*ptr).to_para_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            fprintf(Outfp, b"(D)\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_bnst(mut ptr: *mut BNST_DATA,
                                    mut cp: *mut libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if !cp.is_null() && !ptr.is_null() {
        if OptExpress == 16 as libc::c_int {
            if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
                strcpy(cp,
                       b"\xef\xbc\x9cP\xef\xbc\x9e\x00" as *const u8 as
                           *const libc::c_char);
            } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
                strcpy(cp,
                       b"&lt;I&gt;\x00" as *const u8 as *const libc::c_char);
            } else {
                *cp.offset(0 as libc::c_int as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
        } else if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
            strcpy(cp, b"<P>\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
            strcpy(cp, b"<I>\x00" as *const u8 as *const libc::c_char);
        } else {
            *cp.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        if (*ptr).para_top_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            strcat(cp, b"PARA\x00" as *const u8 as *const libc::c_char);
        } else {
            strcpy(cp, (*(*ptr).mrph_ptr).Goi2.as_mut_ptr());
            i = 1 as libc::c_int;
            while i < (*ptr).mrph_num {
                strcat(cp,
                       (*(*ptr).mrph_ptr.offset(i as
                           isize)).Goi2.as_mut_ptr());
                i += 1
            }
        }
    } else if cp.is_null() && !ptr.is_null() {
        if (*ptr).para_top_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            fprintf(Outfp, b"PARA\x00" as *const u8 as *const libc::c_char);
        } else {
            if OptExpress == 16 as libc::c_int {
                let fresh3 = Tag_Num;
                Tag_Num = Tag_Num + 1;
                fprintf(Outfp,
                        b"%%%% %d %d 1 LABEL=%d_%db align=right style=white-space:nowrap\n\x00"
                            as *const u8 as *const libc::c_char, Sen_Num,
                        fresh3, Sen_Num, Tag_Num - 1 as libc::c_int);
            }
            i = 0 as libc::c_int;
            while i < (*ptr).mrph_num {
                fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*ptr).mrph_ptr.offset(i as
                            isize)).Goi2.as_mut_ptr());
                if Language == 1 as libc::c_int &&
                    OptDisplay != 1 as libc::c_int &&
                    OptDisplay != 5 as libc::c_int {
                    fprintf(Outfp,
                            b"%c\x00" as *const u8 as *const libc::c_char,
                            pos2symbol(Class[(*(*ptr).mrph_ptr.offset(i as
                                isize)).Hinshi
                                as
                                usize][0 as libc::c_int as
                                usize].id as
                                           *mut libc::c_char,
                                       Class[(*(*ptr).mrph_ptr.offset(i as
                                           isize)).Hinshi
                                           as
                                           usize][(*(*ptr).mrph_ptr.offset(i
                                           as
                                           isize)).Bunrui
                                           as usize].id as
                                           *mut libc::c_char) as libc::c_int);
                }
                i += 1
            }
        }
        if OptExpress == 16 as libc::c_int {
            if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
                fprintf(Outfp,
                        b"\xef\xbc\x9cP\xef\xbc\x9e\x00" as *const u8 as
                            *const libc::c_char);
            } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
                fprintf(Outfp,
                        b"&lt;I&gt;\x00" as *const u8 as *const libc::c_char);
            }
        } else if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
            fprintf(Outfp, b"<P>\x00" as *const u8 as *const libc::c_char);
        } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
            fprintf(Outfp, b"<I>\x00" as *const u8 as *const libc::c_char);
        }
        if (*ptr).to_para_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
            fprintf(Outfp, b"(D)\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_data2ipal_corr(mut b_ptr: *mut BNST_DATA,
                                              mut cpm_ptr: *mut CF_PRED_MGR)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut elem_num: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    match (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).voice {
        2 | 5 | 6 | 7 => { offset = 0 as libc::c_int }
        _ => { offset = 1 as libc::c_int }
    }
    flag = 0 as libc::c_int;
    if (*cpm_ptr).elem_b_num[0 as libc::c_int as usize] == -(1 as libc::c_int)
    {
        elem_num = 0 as libc::c_int;
        flag = (0 as libc::c_int == 0) as libc::c_int
    }
    if flag == (0 as libc::c_int == 0) as libc::c_int {
        flag = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j <
            (*(*cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).element_num {
            if (*cpm_ptr).cmm[0 as libc::c_int as
                usize].result_lists_p[0 as libc::c_int as
                usize].flag[j as
                usize]
                == elem_num {
                fprintf(Outfp,
                        b" N%d\x00" as *const u8 as *const libc::c_char,
                        offset + j);
                flag = (0 as libc::c_int == 0) as libc::c_int
            }
            j += 1
        }
    }
    if flag == 0 as libc::c_int {
        fprintf(Outfp, b" *\x00" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while !(*b_ptr).child[i as usize].is_null() {
        flag = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*cpm_ptr).cf.element_num {
            if (*cpm_ptr).elem_b_num[j as usize] == i {
                elem_num = j;
                flag = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else { j += 1 }
        }
        if flag == (0 as libc::c_int == 0) as libc::c_int {
            flag = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j <
                (*(*cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).element_num {
                if (*cpm_ptr).cmm[0 as libc::c_int as
                    usize].result_lists_p[0 as libc::c_int
                    as
                    usize].flag[j
                    as
                    usize]
                    == elem_num {
                    fprintf(Outfp,
                            b" N%d\x00" as *const u8 as *const libc::c_char,
                            offset + j);
                    flag = (0 as libc::c_int == 0) as libc::c_int
                }
                j += 1
            }
        }
        if flag == 0 as libc::c_int {
            fprintf(Outfp, b" *\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_bnst_detail(mut ptr: *mut BNST_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0; /* 文節始り */
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    fputc('(' as i32, Outfp);
    if (*ptr).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int {
        if !(*ptr).child[1 as libc::c_int as usize].is_null() &&
            (*(*ptr).child[1 as libc::c_int as usize]).para_key_type as
                libc::c_int == 1 as libc::c_int {
            fprintf(Outfp,
                    b"noun_para\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp,
                    b"pred_para\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
                (*ptr).num);
        /* 係り受け情報の表示 (追加:97/10/29) */
        fprintf(Outfp, b"(type:%c) \x00" as *const u8 as *const libc::c_char,
                (*ptr).dpnd_type as libc::c_int);
        fputc('(' as i32, Outfp);
        i = 0 as libc::c_int;
        m_ptr = (*ptr).mrph_ptr;
        while i < (*ptr).mrph_num {
            fputc('(' as i32, Outfp);
            print_mrph(m_ptr);
            fprintf(Outfp, b" \x00" as *const u8 as *const libc::c_char);
            print_feature2((*m_ptr).f, Outfp);
            fputc(')' as i32, Outfp);
            i += 1;
            m_ptr = m_ptr.offset(1)
        }
        fputc(')' as i32, Outfp);
        fprintf(Outfp, b" \x00" as *const u8 as *const libc::c_char);
        print_feature2((*ptr).f, Outfp);
        if OptAnalysis == 2 as libc::c_int ||
            check_feature((*ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            (*ptr).cpm_ptr.is_null() {
            /* 解析前 */
            fprintf(Outfp, b" NIL\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp, b" (\x00" as *const u8 as *const libc::c_char);
            if (*(*ptr).cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr.is_null() {
                /* ------------変更:述語素, 格形式を出力-----------------
	    if (ptr->cpm_ptr != NULL &&
		ptr->cpm_ptr->cmm[0].cf_ptr != NULL &&
		(ptr->cpm_ptr->cmm[0].cf_ptr)->cf_address != -1) {
		get_ipal_frame(i_ptr, 
			       (ptr->cpm_ptr->cmm[0].cf_ptr)->cf_address);
		if (i_ptr->DATA[i_ptr->jyutugoso]) {
		    fprintf(Outfp, " 述語素 %s", 
			    i_ptr->DATA+i_ptr->jyutugoso);
		} else {
		    fprintf(Outfp, " 述語素 nil");
		}
		fprintf(Outfp, " 格形式 (");
		for (j=0; *((i_ptr->DATA)+(i_ptr->kaku_keishiki[j])) 
			       != NULL; j++){
		    fprintf(Outfp, " %s", 
			    i_ptr->DATA+i_ptr->kaku_keishiki[j]);
		}
		fprintf(Outfp, ")");
	    }
	    ------------------------------------------------------- */
                fprintf(Outfp,
                        b"-2\x00" as *const u8 as
                            *const libc::c_char); /* 格フレームにENTRYなし */
            } else if (*(*(*ptr).cpm_ptr).cmm[0 as libc::c_int as
                usize].cf_ptr).cf_address ==
                -(1 as libc::c_int) as libc::c_ulonglong {
                fprintf(Outfp,
                        b"-1\x00" as *const u8 as
                            *const libc::c_char); /* 格要素なし */
            } else {
                fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*(*ptr).cpm_ptr).cmm[0 as libc::c_int as
                            usize].cf_ptr).cf_id.as_mut_ptr());
                match (*(*(*ptr).cpm_ptr).cmm[0 as libc::c_int as
                    usize].cf_ptr).voice {
                    1 => {
                        fprintf(Outfp,
                                b" \xe8\x83\xbd\xe5\x8b\x95\x00" as *const u8
                                    as *const libc::c_char);
                    }
                    2 => {
                        fprintf(Outfp,
                                b" \xe9\x96\x93\xe5\x8f\x97\x00" as *const u8
                                    as *const libc::c_char);
                    }
                    3 => {
                        fprintf(Outfp,
                                b" \xe7\x9b\xb4\xe5\x8f\x97\xef\xbc\x91\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    4 => {
                        fprintf(Outfp,
                                b" \xe7\x9b\xb4\xe5\x8f\x97\xef\xbc\x92\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    5 => {
                        fprintf(Outfp,
                                b" \xe4\xbd\xbf\xe5\xbd\xb9\xe3\x83\xb2\xe3\x83\x8b\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    6 => {
                        fprintf(Outfp,
                                b" \xe4\xbd\xbf\xe5\xbd\xb9\xe3\x83\xb2\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    7 => {
                        fprintf(Outfp,
                                b" \xe4\xbd\xbf\xe5\xbd\xb9\xe3\x83\x8b\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    8 => {
                        fprintf(Outfp,
                                b" \xe4\xbd\xbf\xe5\xbd\xb9&\xe5\x8f\x97\xe8\xba\xab\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    9 => {
                        fprintf(Outfp,
                                b" \xe5\x8f\xaf\xe8\x83\xbd\x00" as *const u8
                                    as *const libc::c_char);
                    }
                    10 => {
                        fprintf(Outfp,
                                b" \xe5\xb0\x8a\xe6\x95\xac\x00" as *const u8
                                    as *const libc::c_char);
                    }
                    11 => {
                        fprintf(Outfp,
                                b" \xe8\x87\xaa\xe7\x99\xba\x00" as *const u8
                                    as *const libc::c_char);
                    }
                    _ => {}
                }
                fprintf(Outfp, b" (\x00" as *const u8 as *const libc::c_char);
                print_data2ipal_corr(ptr, (*ptr).cpm_ptr);
                fprintf(Outfp, b")\x00" as *const u8 as *const libc::c_char);
            }
            fprintf(Outfp, b")\x00" as *const u8 as *const libc::c_char);
        }
    }
    fputc(')' as i32, Outfp);
    /* 文節終わり */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_sentence_slim(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    init_bnst_tree_property(sp);
    fputc('(' as i32, Outfp);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        print_bnst(&mut *(*sp).bnst_data.offset(i as isize),
                   0 as *mut libc::c_char);
        i += 1
    }
    fputc(')' as i32, Outfp);
    fputc('\n' as i32, Outfp);
}
/*====================================================================
			       行列表示
====================================================================*/
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_M_bnst(mut sp: *mut SENTENCE_DATA,
                                      mut b_num: libc::c_int,
                                      mut max_length: libc::c_int,
                                      mut para_char: *mut libc::c_int)
/*==================================================================*/
{
    let mut ptr: *mut BNST_DATA =
        &mut *(*sp).bnst_data.offset(b_num as isize) as *mut BNST_DATA;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut space: libc::c_int = 0;
    let mut comma_p: libc::c_int = 0;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut cp: *mut libc::c_char = tmp.as_mut_ptr();
    if (*ptr).mrph_num == 1 as libc::c_int {
        strcpy(tmp.as_mut_ptr(), (*(*ptr).mrph_ptr).Goi2.as_mut_ptr());
        comma_p = 0 as libc::c_int
    } else {
        strcpy(tmp.as_mut_ptr(), (*(*ptr).mrph_ptr).Goi2.as_mut_ptr());
        i = 1 as libc::c_int;
        while i < (*ptr).mrph_num - 1 as libc::c_int {
            strcat(tmp.as_mut_ptr(),
                   (*(*ptr).mrph_ptr.offset(i as isize)).Goi2.as_mut_ptr());
            i += 1
        }
        if strcmp(Class[(*(*ptr).mrph_ptr.offset((*ptr).mrph_num as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).Hinshi
            as usize][0 as libc::c_int as usize].id as
                      *const libc::c_char,
                  b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                      *const libc::c_char) == 0 &&
            strcmp(Class[(*(*ptr).mrph_ptr.offset((*ptr).mrph_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).Hinshi
                as
                usize][(*(*ptr).mrph_ptr.offset((*ptr).mrph_num
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).Bunrui
                as usize].id as
                       *const libc::c_char,
                   b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                       *const libc::c_char) == 0 {
            strcat(tmp.as_mut_ptr(),
                   b",\x00" as *const u8 as *const libc::c_char);
            comma_p = (0 as libc::c_int == 0) as libc::c_int
        } else {
            strcat(tmp.as_mut_ptr(),
                   (*(*ptr).mrph_ptr.offset((*ptr).mrph_num as
                       isize).offset(-(1 as
                       libc::c_int
                       as
                       isize))).Goi2.as_mut_ptr());
            comma_p = 0 as libc::c_int
        }
    }
    space =
        if (*ptr).para_key_type as libc::c_int != 0 {
            (max_length -
                ((*sp).Bnst_num - b_num - 1 as libc::c_int) *
                    3 as libc::c_int) - 2 as libc::c_int
        } else {
            (max_length) -
                ((*sp).Bnst_num - b_num - 1 as libc::c_int) * 3 as libc::c_int
        };
    len =
        if comma_p != 0 {
            ((*ptr).length) - 1 as libc::c_int
        } else { (*ptr).length };
    if len > space {
        if space % 2 as libc::c_int != len % 2 as libc::c_int {
            cp = cp.offset((len + 1 as libc::c_int - space) as isize);
            fputc(' ' as i32, Outfp);
        } else { cp = cp.offset((len - space) as isize) }
    } else {
        i = 0 as libc::c_int;
        while i < space - len {
            fputc(' ' as i32, Outfp);
            i += 1
        }
    }
    if (*ptr).para_key_type != 0 {
        fprintf(Outfp, b"%c>\x00" as *const u8 as *const libc::c_char,
                'a' as i32 + *para_char);
        *para_char += 1
    }
    fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char, cp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_line(mut length: libc::c_int,
                                    mut flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length - 1 as libc::c_int {
        fputc('-' as i32, Outfp);
        i += 1
    }
    if flag != 0 {
        fputc(')' as i32, Outfp);
    } else { fputc('-' as i32, Outfp); };
    fputc('\n' as i32, Outfp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_matrix(mut sp: *mut SENTENCE_DATA,
                                      mut type_0: libc::c_int,
                                      mut key_pos: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0; /* para_key の表示用 */
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut over_flag: libc::c_int = 0 as libc::c_int;
    let mut max_length: libc::c_int = 0 as libc::c_int;
    let mut para_char: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut PARA_DATA = 0 as *mut PARA_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*path_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int;
            j += 1
        }
        i += 1
    }
    /* パスのマーク付け(PARA) */
    if type_0 == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Para_num {
            ptr = &mut *(*sp).para_data.offset(i as isize) as *mut PARA_DATA;
            if !(((*ptr).max_score as libc::c_double) < 0.0f64) {
                /* statusがxでもスコアがあれば参考のため表示 */
                j = (*ptr).key_pos + 1 as libc::c_int;
                while j <= (*ptr).jend_pos {
                    if Language != 2 as libc::c_int {
                        (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j -
                            (*ptr).key_pos
                            -
                            1
                                as
                                libc::c_int)
                            as
                            usize]
                            as
                            isize))[j as
                            usize]
                            =
                            if (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j
                                -
                                (*ptr).key_pos
                                -
                                1
                                    as
                                    libc::c_int)
                                as
                                usize]
                                as
                                isize))[j
                                as
                                usize]
                                != 0 {
                                -(1 as libc::c_int)
                            } else { ('a' as i32) + i }
                    } else if !check_feature((*(*sp).bnst_data.offset(j as
                        isize)).f,
                                             b"CC\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                        ||
                        !check_feature((*(*sp).bnst_data.offset(j as
                            isize)).f,
                                       b"PU\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null()
                    {
                        (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j -
                            (*ptr).key_pos)
                            as
                            usize]
                            as
                            isize))[j as
                            usize]
                            =
                            if (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j
                                -
                                (*ptr).key_pos)
                                as
                                usize]
                                as
                                isize))[j
                                as
                                usize]
                                != 0 {
                                -(1 as libc::c_int)
                            } else { ('a' as i32) + i }
                    } else {
                        (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j -
                            (*ptr).key_pos
                            -
                            1
                                as
                                libc::c_int)
                            as
                            usize]
                            as
                            isize))[j as
                            usize]
                            =
                            if (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j
                                -
                                (*ptr).key_pos
                                -
                                1
                                    as
                                    libc::c_int)
                                as
                                usize]
                                as
                                isize))[j
                                as
                                usize]
                                != 0 {
                                -(1 as libc::c_int)
                            } else { ('a' as i32) + i }
                    }
                    j += 1
                }
            }
            i += 1
        }
    }
    /* 長さの計算 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        length =
            (*(*sp).bnst_data.offset(i as isize)).length +
                ((*sp).Bnst_num - i - 1 as libc::c_int) * 3 as libc::c_int;
        if (*(*sp).bnst_data.offset(i as isize)).para_key_type != 0 {
            length += 2 as libc::c_int
        }
        if max_length < length { max_length = length }
        i += 1
    }
    /* 印刷用の処理 */
    if type_0 == 0 as libc::c_int {
        fprintf(Outfp,
                b"<< PARA MATRIX >>\n\x00" as *const u8 as
                    *const libc::c_char);
    } else if type_0 == 1 as libc::c_int {
        fprintf(Outfp,
                b"<< DPND MATRIX >>\n\x00" as *const u8 as
                    *const libc::c_char);
    } else if type_0 == 2 as libc::c_int {
        fprintf(Outfp,
                b"<< MASK MATRIX >>\n\x00" as *const u8 as
                    *const libc::c_char);
    } else if type_0 == 3 as libc::c_int {
        fprintf(Outfp,
                b"<< QUOTE MATRIX >>\n\x00" as *const u8 as
                    *const libc::c_char);
    } else if type_0 == 4 as libc::c_int {
        fprintf(Outfp,
                b"<< RESTRICT MATRIX for PARA RELATION>>\n\x00" as *const u8
                    as *const libc::c_char);
    } else if type_0 == 5 as libc::c_int {
        fprintf(Outfp,
                b"<< RESTRICT MATRIX for DEPENDENCY STRUCTURE>>\n\x00" as
                    *const u8 as *const libc::c_char);
    } else if type_0 == 6 as libc::c_int {
        fprintf(Outfp,
                b"<< RESTRICT MATRIX for QUOTE SCOPE>>\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    print_line(max_length, over_flag);
    i = 0 as libc::c_int;
    while i < max_length - (*sp).Bnst_num * 3 as libc::c_int {
        fputc(' ' as i32, Outfp);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        fprintf(Outfp, b"%2d \x00" as *const u8 as *const libc::c_char, i);
        i += 1
    }
    fputc('\n' as i32, Outfp);
    print_line(max_length, over_flag);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        print_M_bnst(sp, i, max_length, &mut para_char);
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            if type_0 == 0 as libc::c_int {
                fprintf(Outfp, b"%2d\x00" as *const u8 as *const libc::c_char,
                        (*match_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]);
            } else if type_0 == 1 as libc::c_int {
                if (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    == 0 as libc::c_int {
                    fprintf(Outfp,
                            b" -\x00" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(Outfp,
                            b" %c\x00" as *const u8 as *const libc::c_char,
                            (*Dpnd_matrix.as_mut_ptr().offset(i as
                                isize))[j as
                                usize]
                                as libc::c_char as libc::c_int);
                }
            } else if type_0 == 2 as libc::c_int {
                fprintf(Outfp, b"%2d\x00" as *const u8 as *const libc::c_char,
                        (*Mask_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]);
            } else if type_0 == 3 as libc::c_int {
                fprintf(Outfp, b"%2d\x00" as *const u8 as *const libc::c_char,
                        (*Quote_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]);
            } else if type_0 == 4 as libc::c_int || type_0 == 5 as libc::c_int
                || type_0 == 6 as libc::c_int {
                if j <= key_pos {
                    fprintf(Outfp,
                            b"--\x00" as *const u8 as *const libc::c_char);
                } else if key_pos < i {
                    fprintf(Outfp,
                            b" |\x00" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(Outfp,
                            b"%2d\x00" as *const u8 as *const libc::c_char,
                            (*restrict_matrix.as_mut_ptr().offset(i as
                                isize))[j
                                as
                                usize]);
                }
            }
            match (*path_matrix.as_mut_ptr().offset(i as isize))[j as usize] {
                0 => { fputc(' ' as i32, Outfp); }
                -1 => { fputc('*' as i32, Outfp); }
                _ => {
                    fputc((*path_matrix.as_mut_ptr().offset(i as
                        isize))[j as
                        usize],
                          Outfp);
                }
            }
            j += 1
        }
        fputc('\n' as i32, Outfp);
        i += 1
    }
    print_line(max_length, over_flag);
    if type_0 == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Para_num {
            fprintf(Outfp,
                    b"%c(%c):%4.1f(%4.1f) \x00" as *const u8 as
                        *const libc::c_char,
                    (*(*sp).para_data.offset(i as isize)).para_char as
                        libc::c_int,
                    (*(*sp).para_data.offset(i as isize)).status as
                        libc::c_int,
                    (*(*sp).para_data.offset(i as isize)).max_score as
                        libc::c_double,
                    (*(*sp).para_data.offset(i as isize)).pure_score as
                        libc::c_double);
            i += 1
        }
        fputc('\n' as i32, Outfp);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_para_similarity_feature(mut sp:
                                                        *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut buffer: [libc::c_char; 5120] = [0; 5120];
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        sprintf(buffer.as_mut_ptr(),
                b"\xe4\xb8\xa6\xe5\x88\x97\xe9\xa1\x9e\xe4\xbc\xbc\xe5\xba\xa6:%.3f\x00"
                    as *const u8 as *const libc::c_char,
                (*(*sp).para_data.offset(i as isize)).max_score as
                    libc::c_double);
        assign_cfeature(&mut (*(*sp).bnst_data.offset((*(*sp).para_data.offset(i
            as
            isize)).key_pos
            as isize)).f,
                        buffer.as_mut_ptr(), 0 as libc::c_int);
        i += 1
    };
}
/*====================================================================
	                並列構造間の関係表示
====================================================================*/
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_para_manager(mut sp: *mut SENTENCE_DATA,
                                            mut m_ptr: *mut PARA_MANAGER,
                                            mut level: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < level * 5 as libc::c_int {
        fputc(' ' as i32, Outfp);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*m_ptr).para_num {
        fprintf(Outfp, b" %c\x00" as *const u8 as *const libc::c_char,
                (*(*sp).para_data.offset((*m_ptr).para_data_num[i as usize] as
                    isize)).para_char as
                    libc::c_int);
        i += 1
    }
    fputc(':' as i32, Outfp);
    i = 0 as libc::c_int;
    while i < (*m_ptr).part_num {
        if (*m_ptr).start[i as usize] == (*m_ptr).end[i as usize] {
            fputc('(' as i32, Outfp);
            print_bnst(&mut *(*sp).bnst_data.offset(*(*m_ptr).start.as_mut_ptr().offset(i
                as
                isize)
                as isize),
                       0 as *mut libc::c_char);
            fputc(')' as i32, Outfp);
        } else {
            fputc('(' as i32, Outfp);
            print_bnst(&mut *(*sp).bnst_data.offset(*(*m_ptr).start.as_mut_ptr().offset(i
                as
                isize)
                as isize),
                       0 as *mut libc::c_char);
            fputc('-' as i32, Outfp);
            print_bnst(&mut *(*sp).bnst_data.offset(*(*m_ptr).end.as_mut_ptr().offset(i
                as
                isize)
                as isize),
                       0 as *mut libc::c_char);
            fputc(')' as i32, Outfp);
        }
        i += 1
    }
    fputc('\n' as i32, Outfp);
    i = 0 as libc::c_int;
    while i < (*m_ptr).child_num {
        print_para_manager(sp, (*m_ptr).child[i as usize],
                           level + 1 as libc::c_int);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_para_relation(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Para_M_num {
        if (*(*sp).para_manager.offset(i as isize)).parent.is_null() {
            print_para_manager(sp,
                               &mut *(*sp).para_manager.offset(i as isize),
                               0 as libc::c_int);
        }
        i += 1
    };
}
/*====================================================================
	                木構造表示(from JK)
====================================================================*/
static mut max_width: libc::c_int = 0;
/* 木の最大幅 */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mylog(mut n: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        num = num * 2 as libc::c_int;
        i += 1
    }
    return num;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_self_space(mut ptr: *mut BNST_DATA,
                                         mut depth2: libc::c_int)
/*==================================================================*/
{
    if (*ptr).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int {
        (*ptr).space = 4 as libc::c_int
    } else if OptDisplay == 1 as libc::c_int || OptDisplay == 5 as libc::c_int
    {
        (*ptr).space = (*ptr).length
    } else if (*ptr).type_0 == 4 as libc::c_int {
        (*ptr).space = (*ptr).length + 1 as libc::c_int
    } else { (*ptr).space = (*ptr).length + (*ptr).mrph_num } /* *4 */
    if (*ptr).para_type as libc::c_int == 1 as libc::c_int ||
        (*ptr).para_type as libc::c_int == 2 as libc::c_int ||
        (*ptr).to_para_p as libc::c_int ==
            (0 as libc::c_int == 0) as libc::c_int {
        (*ptr).space += 1 as libc::c_int
    }
    (*ptr).space += (depth2 - 1 as libc::c_int) * 8 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_tree_width(mut ptr: *mut BNST_DATA,
                                         mut depth2: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    calc_self_space(ptr, depth2);
    if (*ptr).space > max_width { max_width = (*ptr).space }
    if !(*ptr).child[0 as libc::c_int as usize].is_null() {
        i = 0 as libc::c_int;
        while !(*ptr).child[i as usize].is_null() {
            calc_tree_width((*ptr).child[i as usize],
                            depth2 + 1 as libc::c_int);
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn show_link(mut depth: libc::c_int,
                                   mut ans_flag: *mut libc::c_char,
                                   mut para_type: libc::c_char,
                                   mut to_para_p: libc::c_char)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    if depth != 1 as libc::c_int {
        /* 親への枝 (兄弟を考慮) */
        if para_type as libc::c_int == 1 as libc::c_int ||
            para_type as libc::c_int == 2 as libc::c_int ||
            to_para_p as libc::c_int ==
                (0 as libc::c_int == 0) as libc::c_int {
            if OptExpress != 16 as libc::c_int {
                fprintf(Outfp,
                        b"\xe2\x94\x80\x00" as *const u8 as
                            *const libc::c_char);
            }
        } else if OptExpress == 16 as libc::c_int {
            fprintf(Outfp,
                    b"\xe2\x94\x80\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp,
                    b"\xe2\x94\x80\xe2\x94\x80\x00" as *const u8 as
                        *const libc::c_char);
        }
        if *ans_flag.offset((depth - 1 as libc::c_int) as isize) as
            libc::c_int == '1' as i32 {
            fprintf(Outfp,
                    b"\xe2\x94\xa4\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp,
                    b"\xe2\x94\x90\x00" as *const u8 as *const libc::c_char);
        }
        if OptExpress == 16 as libc::c_int {
            fprintf(Outfp,
                    b"&nbsp;&nbsp;\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp,
                    b"\xe3\x80\x80\x00" as *const u8 as *const libc::c_char);
        }
        /* 祖先の兄弟の枝 */
        i = depth - 1 as libc::c_int;
        while i > 1 as libc::c_int {
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"&nbsp;&nbsp;&nbsp;&nbsp;\x00" as *const u8 as
                            *const libc::c_char);
            } else {
                fprintf(Outfp,
                        b"\xe3\x80\x80\xe3\x80\x80\x00" as *const u8 as
                            *const libc::c_char);
            }
            if *ans_flag.offset((i - 1 as libc::c_int) as isize) as
                libc::c_int == '1' as i32 {
                fprintf(Outfp,
                        b"\xe2\x94\x82\x00" as *const u8 as
                            *const libc::c_char);
            } else if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"&nbsp;&nbsp;\x00" as *const u8 as
                            *const libc::c_char);
            } else {
                fprintf(Outfp,
                        b"\xe3\x80\x80\x00" as *const u8 as
                            *const libc::c_char);
            }
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"&nbsp;&nbsp;\x00" as *const u8 as
                            *const libc::c_char);
            } else {
                fprintf(Outfp,
                        b"\xe3\x80\x80\x00" as *const u8 as
                            *const libc::c_char);
            }
            i -= 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn show_self(mut ptr: *mut BNST_DATA,
                                   mut depth: libc::c_int,
                                   mut ans_flag_p: *mut libc::c_char,
                                   mut flag: libc::c_int)
/*==================================================================*/
{
    /* 
       depth は自分の深さ(根が1)

       ans_flag は自分と祖先が最後の子かどうかの履歴
       深さnの祖先(または自分)が最後の子であれば ans_flag[n-1] が '0'
       そうでなければ '1'(この場合枝の描画が必要)
    */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ans_flag: [libc::c_char; 200] = [0; 200];
    if !ans_flag_p.is_null() {
        strncpy(ans_flag.as_mut_ptr(), ans_flag_p,
                200 as libc::c_int as libc::c_ulong);
    } else {
        ans_flag[0 as libc::c_int as usize] = '0' as i32 as libc::c_char
        /* 最初に呼ばれるとき */
    }
    if !(*ptr).child[0 as libc::c_int as usize].is_null() {
        i = 0 as libc::c_int;
        while !(*ptr).child[i as usize].is_null() { i += 1 }
        /* 最後の子は ans_flag を 0 に */
        ans_flag[depth as usize] = '0' as i32 as libc::c_char;
        show_self((*ptr).child[(i - 1 as libc::c_int) as usize],
                  depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                  0 as libc::c_int);
        if i > 1 as libc::c_int {
            /* 他の子は ans_flag を 1 に */
            ans_flag[depth as usize] = '1' as i32 as libc::c_char;
            j = i - 2 as libc::c_int;
            while j > 0 as libc::c_int {
                show_self((*ptr).child[j as usize], depth + 1 as libc::c_int,
                          ans_flag.as_mut_ptr(), 0 as libc::c_int);
                j -= 1
            }
            /* flag: 1: ─PARA 2: -<P>PARA */
            if (*ptr).para_top_p as libc::c_int ==
                (0 as libc::c_int == 0) as libc::c_int &&
                (*ptr).para_type as libc::c_int == 0 as libc::c_int &&
                (*ptr).to_para_p as libc::c_int == 0 as libc::c_int {
                show_self((*ptr).child[0 as libc::c_int as usize],
                          depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                          1 as libc::c_int);
            } else if (*ptr).para_top_p as libc::c_int ==
                (0 as libc::c_int == 0) as libc::c_int {
                show_self((*ptr).child[0 as libc::c_int as usize],
                          depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                          2 as libc::c_int);
            } else {
                show_self((*ptr).child[0 as libc::c_int as usize],
                          depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                          0 as libc::c_int);
            }
        }
    }
    calc_self_space(ptr, depth);
    if OptExpress != 16 as libc::c_int {
        if (*ptr).para_top_p as libc::c_int !=
            (0 as libc::c_int == 0) as libc::c_int {
            i = 0 as libc::c_int;
            while i < max_width - (*ptr).space {
                fputc(' ' as i32, Outfp);
                i += 1
            }
        }
    }
    if OptExpress & 4 as libc::c_int != 0 {
        print_mrph_with_para(ptr as *mut MRPH_DATA, 0 as *mut libc::c_char);
    } else { print_bnst(ptr, 0 as *mut libc::c_char); }
    if flag == 0 as libc::c_int {
        show_link(depth, ans_flag.as_mut_ptr(), (*ptr).para_type,
                  (*ptr).to_para_p);
        if OptExpress == 65 as libc::c_int {
            print_some_feature((*ptr).f, Outfp);
        }
        fputc('\n' as i32, Outfp);
    } else if flag == 1 as libc::c_int {
        if OptExpress != 16 as libc::c_int {
            fprintf(Outfp,
                    b"\xe2\x94\x80\x00" as *const u8 as *const libc::c_char);
        }
    } else if flag == 2 as libc::c_int {
        if OptExpress != 16 as libc::c_int {
            fprintf(Outfp, b"-\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn show_sexp(mut ptr: *mut BNST_DATA,
                                   mut depth: libc::c_int,
                                   mut pars: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < depth {
        fputc(' ' as i32, Outfp);
        i += 1
    }
    fprintf(Outfp, b"(\x00" as *const u8 as *const libc::c_char);
    if (*ptr).para_top_p as libc::c_int ==
        (0 as libc::c_int == 0) as libc::c_int {
        if !(*ptr).child[1 as libc::c_int as usize].is_null() &&
            (*(*ptr).child[1 as libc::c_int as usize]).para_key_type as
                libc::c_int == 1 as libc::c_int {
            fprintf(Outfp,
                    b"(noun_para\x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(Outfp,
                    b"(pred_para\x00" as *const u8 as *const libc::c_char);
        }
        if !(*ptr).child[0 as libc::c_int as usize].is_null() {
            fputc('\n' as i32, Outfp);
            i = 0 as libc::c_int;
            while !(*ptr).child[(i + 1 as libc::c_int) as usize].is_null() &&
                (*(*ptr).child[(i + 1 as libc::c_int) as
                    usize]).para_type as libc::c_int !=
                    0 as libc::c_int {
                /* <P>の最後以外 */
                /* UCHI fputc(',', Outfp); */
                show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                          0 as libc::c_int);
                i += 1
            }
            if !(*ptr).child[(i + 1 as libc::c_int) as usize].is_null() {
                /* その他がある場合 */
                /* <P>の最後 */
                /* UCHI fputc(',', Outfp); */
                show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                          1 as libc::c_int);
                i += 1;
                /* その他の最後以外 */
                while !(*ptr).child[(i + 1 as libc::c_int) as usize].is_null()
                {
                    /* UCHI fputc(',', Outfp); */
                    show_sexp((*ptr).child[i as usize],
                              depth + 3 as libc::c_int, 0 as libc::c_int);
                    i += 1
                }
                /* その他の最後 */
                /* UCHI fputc(',', Outfp); */
                show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                          pars + 1 as libc::c_int);
            } else {
                /* <P>の最後 */
                /* UCHI fputc(',', Outfp); */
                show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                          pars + 1 as libc::c_int + 1 as libc::c_int);
            }
        }
    } else {
        print_bnst_detail(ptr);
        if !(*ptr).child[0 as libc::c_int as usize].is_null() {
            fputc('\n' as i32, Outfp);
            i = 0 as libc::c_int;
            while !(*ptr).child[(i + 1 as libc::c_int) as usize].is_null() {
                /* UCHI fputc(',', Outfp); */
                show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                          0 as libc::c_int);
                i += 1
            }
            /* UCHI fputc(',', Outfp); */
            show_sexp((*ptr).child[i as usize], depth + 3 as libc::c_int,
                      pars + 1 as libc::c_int);
        } else {
            i = 0 as libc::c_int;
            while i < pars + 1 as libc::c_int {
                fputc(')' as i32, Outfp);
                i += 1
            }
            fputc('\n' as i32, Outfp);
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_kakari(mut sp: *mut SENTENCE_DATA,
                                      mut type_0: libc::c_int,
                                      mut eos_flag: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut last_b_offset: libc::c_int = 1 as libc::c_int;
    let mut last_t_offset: libc::c_int = 1 as libc::c_int;
    /* 最後の文節、基本句がマージされている場合があるので、
       本当の最後の文節、基本句を探す */
    if OptPostProcess != 0 {
        i = (*sp).Bnst_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if (*(*sp).bnst_data.offset(i as isize)).num !=
                -(1 as libc::c_int) {
                last_b_offset = (*sp).Bnst_num - i;
                break;
            } else { i -= 1 }
        }
        i = (*sp).Tag_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if (*(*sp).tag_data.offset(i as isize)).num != -(1 as libc::c_int)
            {
                last_t_offset = (*sp).Tag_num - i;
                break;
            } else { i -= 1 }
        }
    }
    /* 依存構造木の表示 */
    if type_0 == 8 as libc::c_int {
        show_sexp((*sp).bnst_data.offset((*sp).Bnst_num as
            isize).offset(-(last_b_offset as
            isize)),
                  0 as libc::c_int, 0 as libc::c_int);
    } else if type_0 & 2 as libc::c_int != 0 {
        max_width = 0 as libc::c_int;
        calc_tree_width((*sp).bnst_data.offset((*sp).Bnst_num as
            isize).offset(-(last_b_offset
            as
            isize)),
                        1 as libc::c_int);
        show_self((*sp).bnst_data.offset((*sp).Bnst_num as
            isize).offset(-(last_b_offset as
            isize)),
                  1 as libc::c_int, 0 as *mut libc::c_char, 0 as libc::c_int);
    } else if type_0 & 4 as libc::c_int != 0 {
        max_width = 0 as libc::c_int;
        calc_tree_width((*sp).mrph_data.offset((*sp).Mrph_num as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))
                            as *mut BNST_DATA, 1 as libc::c_int);
        show_self((*sp).mrph_data.offset((*sp).Mrph_num as
            isize).offset(-(1 as libc::c_int
            as isize)) as
                      *mut BNST_DATA, 1 as libc::c_int,
                  0 as *mut libc::c_char, 0 as libc::c_int);
    } else {
        /* 文節のtreeを描くとき */
        /* 形態素のtreeを描くとき */
        /* tag単位のtreeを描くとき */
        max_width = 0 as libc::c_int;
        calc_tree_width((*sp).tag_data.offset((*sp).Tag_num as
            isize).offset(-(last_t_offset
            as
            isize))
                            as *mut BNST_DATA, 1 as libc::c_int);
        show_self((*sp).tag_data.offset((*sp).Tag_num as
            isize).offset(-(last_t_offset as
            isize)) as
                      *mut BNST_DATA, 1 as libc::c_int,
                  0 as *mut libc::c_char, 0 as libc::c_int);
    }
    if OptExpress == 16 as libc::c_int {
        Tag_Num = 1 as libc::c_int;
        Sen_Num += 1
    } else { print_eos(eos_flag); };
}
/*====================================================================
			      チェック用
====================================================================*/
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_bnst(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut b_buffer: [libc::c_char; 256] = [0; 256];
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        ptr = &mut *(*sp).bnst_data.offset(i as isize) as *mut BNST_DATA;
        b_buffer[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        j = 0 as libc::c_int;
        while j < (*ptr).mrph_num {
            /* buffer overflow */
            if strlen(b_buffer.as_mut_ptr()).wrapping_add(strlen((*(*ptr).mrph_ptr.offset(j
                as
                isize)).Goi2.as_mut_ptr())).wrapping_add(4
                as
                libc::c_int
                as
                libc::c_ulong)
                > 256 as libc::c_int as libc::c_ulong {
                break;
            }
            if (*ptr).mrph_ptr.offset(j as isize) == (*ptr).head_ptr {
                strcat(b_buffer.as_mut_ptr(),
                       b"[\x00" as *const u8 as *const libc::c_char);
                strcat(b_buffer.as_mut_ptr(),
                       (*(*ptr).mrph_ptr.offset(j as
                           isize)).Goi2.as_mut_ptr());
                strcat(b_buffer.as_mut_ptr(),
                       b"]\x00" as *const u8 as *const libc::c_char);
            } else {
                strcat(b_buffer.as_mut_ptr(),
                       (*(*ptr).mrph_ptr.offset(j as
                           isize)).Goi2.as_mut_ptr());
            }
            strcat(b_buffer.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
            j += 1
        }
        fprintf(Outfp, b"%-20s\x00" as *const u8 as *const libc::c_char,
                b_buffer.as_mut_ptr());
        print_feature((*ptr).f, Outfp);
        if !check_feature((*ptr).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() ||
            !check_feature((*ptr).f,
                           b"\xe6\xba\x96\xe7\x94\xa8\xe8\xa8\x80\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            fprintf(Outfp,
                    b" <\xe8\xa1\xa8\xe5\xb1\xa4\xe6\xa0\xbc:\x00" as
                        *const u8 as *const libc::c_char);
            if (*ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x82\xac,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\xb2,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\x8b,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x87\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\x87,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x82\xab\xe3\x83\xa9\xe6\xa0\xbc\x00"
                as *const u8 as
                *const libc::c_char as
                *mut libc::c_char) as usize] !=
                0 {
                fprintf(Outfp,
                        b"\xe3\x82\xab\xe3\x83\xa9,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x88\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\x88,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\xa8\xe3\x83\xaa\xe6\xa0\xbc\x00"
                as *const u8 as
                *const libc::c_char as
                *mut libc::c_char) as usize] !=
                0 {
                fprintf(Outfp,
                        b"\xe3\x83\xa8\xe3\x83\xaa,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x98\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\x98,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x9e\xe3\x83\x87\xe6\xa0\xbc\x00"
                as *const u8 as
                *const libc::c_char as
                *mut libc::c_char) as usize] !=
                0 {
                fprintf(Outfp,
                        b"\xe3\x83\x9e\xe3\x83\x87,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x83\x8e\xe6\xa0\xbc\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x83\x8e,\x00" as *const u8 as
                            *const libc::c_char);
            }
            if (*ptr).SCASE_code[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00" as
                *const u8 as *const libc::c_char
                as *mut libc::c_char) as usize]
                != 0 {
                fprintf(Outfp,
                        b"\xe3\x82\xac\xef\xbc\x92,\x00" as *const u8 as
                            *const libc::c_char);
            }
            fprintf(Outfp, b">\x00" as *const u8 as *const libc::c_char);
        }
        fputc('\n' as i32, Outfp);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_case_for_table(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf1: [libc::c_char; 256] = [0; 256];
    let mut buf2: [libc::c_char; 256] = [0; 256];
    let mut buf3: [libc::c_char; 256] = [0; 256];
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        cp =
            check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            /* OPT_TABLE */
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"%%%% %d %d 2 LABEL=%d_%dd style=white-space:nowrap\n\x00"
                            as *const u8 as *const libc::c_char,
                        Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int,
                        Sen_Num - 1 as libc::c_int, i + 1 as libc::c_int);
                fprintf(Outfp,
                        b"*\n\x00" as *const u8 as *const libc::c_char);
            }
            /* O */
            cp =
                check_feature((*(*sp).tag_data.offset(i as isize)).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            loop {
                next =
                    strstr(cp,
                           b"/O/\x00" as *const u8 as *const libc::c_char);
                if next.is_null() { break; }
                cp = next;
                while *cp.offset(0 as libc::c_int as isize) as libc::c_int !=
                    ';' as i32 &&
                    *cp.offset(0 as libc::c_int as isize) as libc::c_int
                        != ':' as i32 {
                    cp = cp.offset(-1)
                }
                if sscanf(cp,
                          b"%*[:;]%[^/]%*[/]%[^/]%*[/]%[^/]%*[/]\x00" as
                              *const u8 as *const libc::c_char,
                          buf1.as_mut_ptr(), buf2.as_mut_ptr(),
                          buf3.as_mut_ptr()) != 0 {
                    fprintf(Outfp,
                            b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                                *const u8 as *const libc::c_char,
                            Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int);
                    fprintf(Outfp,
                            b"&nbsp;%s:%s&nbsp;\n\x00" as *const u8 as
                                *const libc::c_char, buf1.as_mut_ptr(),
                            buf3.as_mut_ptr());
                    cp =
                        strstr(cp,
                               buf2.as_mut_ptr()).offset(1 as libc::c_int as
                            isize)
                }
            }
            /* C */
            cp =
                check_feature((*(*sp).tag_data.offset(i as isize)).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            loop {
                next =
                    strstr(cp,
                           b"/C/\x00" as *const u8 as *const libc::c_char);
                if next.is_null() { break; }
                cp = next;
                while *cp.offset(0 as libc::c_int as isize) as libc::c_int !=
                    ';' as i32 &&
                    *cp.offset(0 as libc::c_int as isize) as libc::c_int
                        != ':' as i32 {
                    cp = cp.offset(-1)
                }
                if sscanf(cp,
                          b"%*[:;]%[^/]%*[/]%[^/]%*[/]%[^/]%*[/]\x00" as
                              *const u8 as *const libc::c_char,
                          buf1.as_mut_ptr(), buf2.as_mut_ptr(),
                          buf3.as_mut_ptr()) != 0 {
                    fprintf(Outfp,
                            b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                                *const u8 as *const libc::c_char,
                            Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int);
                    fprintf(Outfp,
                            b"&nbsp;[%s:%s]&nbsp;\n\x00" as *const u8 as
                                *const libc::c_char, buf1.as_mut_ptr(),
                            buf3.as_mut_ptr());
                    cp =
                        strstr(cp,
                               buf2.as_mut_ptr()).offset(1 as libc::c_int as
                            isize)
                }
            }
            /* N */
            cp =
                check_feature((*(*sp).tag_data.offset(i as isize)).f,
                              b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            loop {
                next =
                    strstr(cp,
                           b"/N/\x00" as *const u8 as *const libc::c_char);
                if next.is_null() { break; }
                cp = next;
                while *cp.offset(0 as libc::c_int as isize) as libc::c_int !=
                    ';' as i32 &&
                    *cp.offset(0 as libc::c_int as isize) as libc::c_int
                        != ':' as i32 {
                    cp = cp.offset(-1)
                }
                if sscanf(cp,
                          b"%*[:;]%[^/]%*[/]%[^/]%*[/]%[^/]%*[/]\x00" as
                              *const u8 as *const libc::c_char,
                          buf1.as_mut_ptr(), buf2.as_mut_ptr(),
                          buf3.as_mut_ptr()) != 0 {
                    fprintf(Outfp,
                            b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                                *const u8 as *const libc::c_char,
                            Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int);
                    fprintf(Outfp,
                            b"&nbsp;[%s:%s]&nbsp;\n\x00" as *const u8 as
                                *const libc::c_char, buf1.as_mut_ptr(),
                            buf3.as_mut_ptr());
                    cp =
                        strstr(cp,
                               buf2.as_mut_ptr()).offset(1 as libc::c_int as
                            isize)
                }
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_corefer_for_table(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut s_num: libc::c_int = 0;
    let mut t_num: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        cp =
            check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"COREFER_ID\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            fprintf(Outfp,
                    b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                        *const u8 as *const libc::c_char,
                    Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int);
            fprintf(Outfp,
                    b"&nbsp;ID=%s&nbsp;\n\x00" as *const u8 as
                        *const libc::c_char,
                    cp.offset(11 as libc::c_int as isize));
            if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                              b"REFERRED\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                sscanf(check_feature((*(*sp).tag_data.offset(i as isize)).f,
                                     b"REFERRED\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"REFERRED:%d-%d\x00" as *const u8 as
                           *const libc::c_char,
                       &mut s_num as *mut libc::c_int,
                       &mut t_num as *mut libc::c_int);
                fprintf(Outfp,
                        b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                            *const u8 as *const libc::c_char,
                        Sen_Num - s_num - 1 as libc::c_int,
                        t_num + 2 as libc::c_int);
                fprintf(Outfp,
                        b"&nbsp;ID=%s&nbsp;\n\x00" as *const u8 as
                            *const libc::c_char,
                        cp.offset(11 as libc::c_int as isize));
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_ne_for_table(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        cp =
            check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"NE\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            fprintf(Outfp,
                    b"%%%% %d %d 2 style=white-space:nowrap\n\x00" as
                        *const u8 as *const libc::c_char,
                    Sen_Num - 1 as libc::c_int, i + 2 as libc::c_int);
            fprintf(Outfp,
                    b"&nbsp;%s&nbsp;\n\x00" as *const u8 as
                        *const libc::c_char,
                    cp.offset(3 as libc::c_int as isize));
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_result(mut sp: *mut SENTENCE_DATA,
                                      mut case_print_flag: libc::c_int,
                                      mut eos_flag: libc::c_int)
/*==================================================================*/
{
    /* case_print_flag: 格解析結果を出力 */
    let mut date_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut time_string: [libc::c_char; 64] = [0; 64];
    let mut t: time_t = 0;
    let mut tms: *mut tm = 0 as *mut tm;
    let mut tm: *mut TOTAL_MGR = (*sp).Best_mgr;
    /* 時間の取得 */
    t = time(0 as *mut time_t);
    tms = localtime(&mut t);
    if strftime(time_string.as_mut_ptr(), 64 as libc::c_int as size_t,
                b"%Y/%m/%d\x00" as *const u8 as *const libc::c_char, tms) == 0
    {
        time_string[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    /* PS出力の場合
       dpnd_info_to_bnst(&(tm->dpnd));
       make_dpnd_tree();
       print_kakari2ps();
       return;
    */
    /* 既解析へのパターンマッチで, マッチがなければ出力しない
       if (OptAnalysis == OPT_AssignF && !PM_Memo[0]) return;
    */
    /* ヘッダの出力 */
    if OptExpress == 16 as libc::c_int {
        if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int
            || OptNE != 0 {
            fprintf(Outfp,
                    b"%%%% %d %d 2\n\x00" as *const u8 as *const libc::c_char,
                    Sen_Num, Tag_Num);
            fprintf(Outfp,
                    b"\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\n\x00"
                        as *const u8 as *const libc::c_char);
        }
        let fresh4 = Tag_Num;
        Tag_Num = Tag_Num + 1;
        fprintf(Outfp,
                b"%%%% %d %d 1 style=white-space:nowrap\n\x00" as *const u8 as
                    *const libc::c_char, Sen_Num, fresh4);
    }
    /* S-ID */
    if !(*sp).KNPSID.is_null() {
        fprintf(Outfp, b"# %s\x00" as *const u8 as *const libc::c_char,
                (*sp).KNPSID);
    } else {
        fprintf(Outfp, b"# S-ID:%d\x00" as *const u8 as *const libc::c_char,
                (*sp).Sen_num);
    }
    /* コメント */
    if !(*sp).Comment.is_null() {
        fprintf(Outfp, b" %s\x00" as *const u8 as *const libc::c_char,
                (*sp).Comment);
    }
    if OptInput == 0 as libc::c_int {
        fprintf(Outfp, b" KNP:%s-%s\x00" as *const u8 as *const libc::c_char,
                b"5.0\x00" as *const u8 as *const libc::c_char,
                b"eb641498\x00" as *const u8 as *const libc::c_char);
        date_p = getenv(b"DATE\x00" as *const u8 as *const libc::c_char);
        if !date_p.is_null() {
            fprintf(Outfp,
                    b" DATE:%s\x00" as *const u8 as *const libc::c_char,
                    date_p);
        } else if time_string[0 as libc::c_int as usize] != 0 {
            fprintf(Outfp,
                    b" DATE:%s\x00" as *const u8 as *const libc::c_char,
                    time_string.as_mut_ptr());
        }
    }
    /* スコアを出力 (CKY時、通常入力時) */
    if OptCKY != 0 && OptInput & 1 as libc::c_int == 0 {
        fprintf(Outfp, b" SCORE:%.5f\x00" as *const u8 as *const libc::c_char,
                (*sp).score);
    }
    /* エラーがあれば、エラーの内容 */
    if !ErrorComment.is_null() {
        fprintf(Outfp, b" ERROR:%s\x00" as *const u8 as *const libc::c_char,
                ErrorComment);
        free(ErrorComment as *mut libc::c_void);
        ErrorComment = 0 as *mut libc::c_char
    }
    /* 警告があれば、警告の内容 */
    if !WarningComment.is_null() {
        fprintf(Outfp, b" WARNING:%s\x00" as *const u8 as *const libc::c_char,
                WarningComment);
        free(WarningComment as *mut libc::c_void);
        WarningComment = 0 as *mut libc::c_char
    }
    if *PM_Memo.as_mut_ptr().offset(0 as libc::c_int as isize) != 0 {
        if !(*sp).Comment.is_null() &&
            !strstr((*sp).Comment,
                    b"MEMO\x00" as *const u8 as
                        *const libc::c_char).is_null() {
            fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                    PM_Memo.as_mut_ptr());
        } else {
            fprintf(Outfp,
                    b" MEMO:%s\x00" as *const u8 as *const libc::c_char,
                    PM_Memo.as_mut_ptr());
        }
    }
    fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
    /* 解析結果のメインの出力 */
    if OptExpress == 4 as libc::c_int {
        print_mrphs(sp, eos_flag);
    } else if OptExpress == 0 as libc::c_int {
        print_tags(sp, 1 as libc::c_int, eos_flag);
    } else if OptExpress == 2 as libc::c_int {
        print_bnst_with_mrphs(sp, 1 as libc::c_int, eos_flag);
    } else if OptExpress == 32 as libc::c_int {
        /* FIXME: 格解析結果の整合性をとる必要がある */
        print_pa_structure(sp, eos_flag);
    } else if OptExpress == 3 as libc::c_int {
        /* 文節のtree出力 */
        if make_dpnd_tree(sp) != 0 {
            print_kakari(sp, OptExpress, eos_flag);
        } else { print_eos(eos_flag); }
    } else if OptExpress == 5 as libc::c_int {
        /* 形態素のtree出力 */
        if make_dpnd_tree(sp) != 0 {
            bnst_to_mrph_tree(sp); /* 形態素の木へ */
            print_kakari(sp, OptExpress, eos_flag);
        } else { print_eos(eos_flag); }
    } else if make_dpnd_tree(sp) != 0 {
        /* タグ単位のtree出力 */
        bnst_to_tag_tree(sp);
        print_kakari(sp, OptExpress, eos_flag); /* タグ単位の木へ */
        /* OPT_TREE */
    } else { print_eos(eos_flag); }
    if OptExpress == 16 as libc::c_int {
        print_tags(sp, 1 as libc::c_int, eos_flag);
        if OptAnalysis == 1 as libc::c_int || OptAnalysis == 6 as libc::c_int
        {
            print_case_for_table(sp);
        }
        if OptNE != 0 { print_ne_for_table(sp); }
        if OptEllipsis & 8 as libc::c_int != 0 {
            print_corefer_for_table(sp);
        }
    }
    /* nbestオプションなどではこの関数が複数回呼ばれるので後処理を元に戻しておく */
    if OptPostProcess != 0 { undo_tag_bnst_postprocess(sp); }
    /* 格解析を行なった場合の出力 */
    if case_print_flag != 0 && OptArticle == 0 &&
        ((OptAnalysis == 1 as libc::c_int ||
            OptAnalysis == 6 as libc::c_int) &&
            (OptDisplay == 2 as libc::c_int ||
                OptDisplay == 3 as libc::c_int ||
                OptExpress == 16 as libc::c_int) ||
            OptEllipsis != 0 &&
                VerboseLevel as libc::c_uint >=
                    VERBOSE1 as libc::c_int as libc::c_uint) {
        print_case_result(sp, Sen_Num);
        /* 次の解析のために初期化しておく */
        (*tm).pred_num = 0 as libc::c_int
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn do_postprocess(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    /* 後処理 */
    if make_dpnd_tree(sp) != 0 {
        bnst_to_tag_tree(sp); /* タグ単位の木へ */
        if OptExpress == 0 as libc::c_int || OptExpress == 2 as libc::c_int {
            tag_bnst_postprocess(sp, 1 as libc::c_int);
        } else {
            tag_bnst_postprocess(sp, 0 as libc::c_int);
            /* 木構造出力のため、num, dpnd_head の番号の付け替えはしない */
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn push_entity(mut list: *mut *mut *mut libc::c_char,
                                     mut key: *mut libc::c_char,
                                     mut count: libc::c_int,
                                     mut max: *mut libc::c_int)
/*==================================================================*/
{
    if *max == 0 as libc::c_int {
        *max = 1024 as libc::c_int;
        *list =
            malloc_data((::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul(*max as
                libc::c_ulong),
                        b"push_entity\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut *mut libc::c_char
    } else if *max <= count {
        *max <<= 1 as libc::c_int;
        *list =
            realloc_data(*list as *mut libc::c_void,
                         (::std::mem::size_of::<*mut libc::c_char>() as
                             libc::c_ulong).wrapping_mul(*max as
                             libc::c_ulong),
                         b"push_entity\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char) as
                *mut *mut libc::c_char
    }
    let ref mut fresh5 = *(*list).offset(count as isize);
    *fresh5 = key;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn prepare_entity(mut bp: *mut BNST_DATA)
/*==================================================================*/
{
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    /* モダリティ */
    flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*bp).mrph_num {
        if flag & 0x1 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe4\xbe\x9d\xe9\xa0\xbc\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh6 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh6, &mut max);
            flag |= 0x1 as libc::c_int
        }
        if flag & 0x2 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe6\x84\x8f\xe5\xbf\x97\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh7 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh7, &mut max);
            flag |= 0x2 as libc::c_int
        }
        if flag & 0x4 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe5\x8b\xa7\xe8\xaa\x98\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh8 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh8, &mut max);
            flag |= 0x4 as libc::c_int
        }
        if flag & 0x8 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe9\xa1\x98\xe6\x9c\x9b\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh9 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh9, &mut max);
            flag |= 0x8 as libc::c_int
        }
        if flag & 0x10 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe7\xa6\x81\xe6\xad\xa2\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh10 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh10, &mut max);
            flag |= 0x10 as libc::c_int
        }
        if flag & 0x20 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe4\xb8\x89\xe4\xba\xba\xe7\xa7\xb0\xe6\x84\x8f\xe5\xbf\x97\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh11 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh11, &mut max);
            flag |= 0x20 as libc::c_int
        }
        if flag & 0x40 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe7\x94\xb3\xe3\x81\x97\xe5\x87\xba\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh12 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh12, &mut max);
            flag |= 0x40 as libc::c_int
        }
        if flag & 0x80 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe6\x8e\xa8\xe9\x87\x8f\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh13 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh13, &mut max);
            flag |= 0x80 as libc::c_int
        }
        if flag & 0x100 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe6\x84\x8f\xe6\x80\x9d-\xe5\x91\xbd\xe4\xbb\xa4\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh14 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh14, &mut max);
            flag |= 0x100 as libc::c_int
        }
        if flag & 0x200 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\xbd\x93\xe7\x82\xba\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh15 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh15, &mut max);
            flag |= 0x200 as libc::c_int
        }
        if flag & 0x400 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\xbd\x93\xe7\x82\xba-\xe8\xa8\xb1\xe5\x8f\xaf\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh16 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh16, &mut max);
            flag |= 0x400 as libc::c_int
        }
        if flag & 0x800 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\x88\xa4\xe6\x96\xad-\xe5\x8f\xaf\xe8\x83\xbd\xe6\x80\xa7\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh17 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh17, &mut max);
            flag |= 0x800 as libc::c_int
        }
        if flag & 0x1000 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\x88\xa4\xe6\x96\xad-\xe5\x8f\xaf\xe8\x83\xbd\xe6\x80\xa7-\xe4\xb8\x8d\xe5\x8f\xaf\xe8\x83\xbd\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh18 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh18, &mut max);
            flag |= 0x1000 as libc::c_int
        }
        if flag & 0x2000 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\x88\xa4\xe6\x96\xad-\xe6\x8e\xa8\xe9\x87\x8f\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh19 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh19, &mut max);
            flag |= 0x2000 as libc::c_int
        }
        if flag & 0x4000 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\x88\xa4\xe6\x96\xad-\xe4\xbc\x9d\xe8\x81\x9e\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh20 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh20, &mut max);
            flag |= 0x4000 as libc::c_int
        }
        if flag & 0x8000 as libc::c_int == 0 &&
            {
                cp =
                    check_feature((*(*bp).mrph_ptr.offset(i as isize)).f,
                                  b"Modality-\xe5\x88\xa4\xe6\x96\xad-\xe6\xa7\x98\xe6\x85\x8b\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                !cp.is_null()
            } {
            let fresh21 = count;
            count = count + 1;
            push_entity(&mut list, cp, fresh21, &mut max);
            flag |= 0x8000 as libc::c_int
        }
        i += 1
    }
    /* 出力するfeatureがあれば出力 */
    if count != 0 {
        let mut i_0: libc::c_int = 0;
        let mut len: libc::c_int = 0 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < count {
            len =
                (len as
                    libc::c_ulong).wrapping_add(strlen(*list.offset(i_0 as
                    isize)).wrapping_add(1
                    as
                    libc::c_int
                    as
                    libc::c_ulong))
                    as libc::c_int as libc::c_int;
            i_0 += 1
        }
        str =
            malloc_data((::std::mem::size_of::<libc::c_char>() as
                libc::c_ulong).wrapping_mul((len +
                2 as
                    libc::c_int)
                as
                libc::c_ulong),
                        b"print_entity\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        strcpy(str, b"C:\x00" as *const u8 as *const libc::c_char);
        i_0 = 0 as libc::c_int;
        while i_0 < count {
            if i_0 != 0 as libc::c_int {
                strcat(str, b" \x00" as *const u8 as *const libc::c_char);
            }
            strcat(str, *list.offset(i_0 as isize));
            i_0 += 1
        }
        assign_cfeature(&mut (*bp).f, str, 0 as libc::c_int);
        free(str as *mut libc::c_void);
        free(list as *mut libc::c_void);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn prepare_all_entity(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        prepare_entity((*sp).bnst_data.offset(i as isize));
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_tree_for_chinese(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max_len: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut max_inverse_len: libc::c_int = 0;
    let mut up_corner: *mut libc::c_char =
        b"\xe2\x94\x8c\xe2\x94\x80\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut down_corner: *mut libc::c_char =
        b"\xe2\x94\x94\xe2\x94\x80\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut middle_corner: *mut libc::c_char =
        b"\xe2\x94\x9c\xe2\x94\x80\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut link: *mut libc::c_char =
        b"\xe2\x94\x82\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut para: *mut libc::c_char =
        b"<P>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut para_head: *mut libc::c_char =
        b"<PARA><P>\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* initialization */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < 100 as libc::c_int {
            bnst_tree[i as usize][j as usize] =
                b"\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            j += 1
        }
        i += 1
    }
    /* read data */
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        bnst_word[i as usize] = (*(*b_ptr).head_ptr).Goi.as_mut_ptr();
        bnst_pos[i as usize] = (*(*b_ptr).head_ptr).Pos.as_mut_ptr();
        bnst_dpnd[i as usize] = (*b_ptr).dpnd_head;
        bnst_level[i as usize] = -(1 as libc::c_int);
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    /* get root level */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        while bnst_level[i as usize] == -(1 as libc::c_int) {
            j = i;
            while bnst_dpnd[j as usize] != -(1 as libc::c_int) &&
                bnst_level[bnst_dpnd[j as usize] as usize] ==
                    -(1 as libc::c_int) {
                j = bnst_dpnd[j as usize]
            }
            if bnst_dpnd[j as usize] == -(1 as libc::c_int) {
                bnst_level[j as usize] = 0 as libc::c_int
            } else {
                bnst_level[j as usize] =
                    bnst_level[bnst_dpnd[j as usize] as usize] +
                        1 as libc::c_int
            }
        }
        i += 1
    }
    /* get print tree */
    max_len = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        len = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < bnst_level[i as usize] * 4 as libc::c_int {
            if bnst_dpnd[i as usize] != -(1 as libc::c_int) &&
                j <
                    bnst_level[bnst_dpnd[i as usize] as usize] *
                        4 as libc::c_int {
                if len >= 100 as libc::c_int {
                    fprintf(Outfp,
                            b">>>tree width exceeds maximum length\n\x00" as
                                *const u8 as *const libc::c_char);
                    return;
                }
                bnst_tree[i as usize][len as usize] =
                    b" \x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                len += 1
            } else if bnst_dpnd[i as usize] != -(1 as libc::c_int) &&
                j ==
                    bnst_level[bnst_dpnd[i as usize] as usize] *
                        4 as libc::c_int {
                if bnst_dpnd[i as usize] != -(1 as libc::c_int) &&
                    bnst_dpnd[i as usize] < i {
                    if len >= 100 as libc::c_int {
                        fprintf(Outfp,
                                b">>>tree width exceeds maximum length\n\x00"
                                    as *const u8 as *const libc::c_char);
                        return;
                    }
                    bnst_tree[i as usize][len as usize] = down_corner;
                    len += 1
                } else if bnst_dpnd[i as usize] > i {
                    if len >= 100 as libc::c_int {
                        fprintf(Outfp,
                                b">>>tree width exceeds maximum length\n\x00"
                                    as *const u8 as *const libc::c_char);
                        return;
                    }
                    bnst_tree[i as usize][len as usize] = up_corner;
                    len += 1
                }
            }
            j += 1
        }
        if len >= 100 as libc::c_int {
            fprintf(Outfp,
                    b">>>tree width exceeds maximum length\n\x00" as *const u8
                        as *const libc::c_char);
            return;
        }
        if (*(*sp).bnst_data.offset(i as isize)).is_para == 1 as libc::c_int {
            bnst_tree[i as usize][len as usize] = para;
            len += 1
        } else if (*(*sp).bnst_data.offset(i as isize)).is_para ==
            2 as libc::c_int {
            bnst_tree[i as usize][len as usize] = para_head;
            len += 1
        }
        bnst_tree[i as usize][len as usize] = bnst_word[i as usize];
        len += 1;
        bnst_tree[i as usize][len as usize] =
            b"/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        len += 1;
        bnst_tree[i as usize][len as usize] = bnst_pos[i as usize];
        len += 1;
        if len > max_len { max_len = len }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < max_len {
            if bnst_tree[i as usize][j as usize] ==
                b"\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char {
                bnst_tree[i as usize][j as usize] =
                    b"***\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            j += 1
        }
        i += 1
    }
    /* inverse the tree */
    max_inverse_len = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < max_len {
        len = 0 as libc::c_int;
        j = (*sp).Bnst_num - 1 as libc::c_int;
        while j > -(1 as libc::c_int) {
            bnst_inverse_tree[i as usize][len as usize] =
                bnst_tree[j as usize][i as usize];
            len += 1;
            j -= 1
        }
        if len > max_inverse_len { max_inverse_len = len }
        i += 1
    }
    /* change bnst_inverse_tree */
    i = 0 as libc::c_int;
    while i < max_len {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            if bnst_inverse_tree[i as usize][j as usize] == down_corner {
                k = j + 1 as libc::c_int;
                while k < (*sp).Bnst_num {
                    if bnst_inverse_tree[i as usize][k as usize] ==
                        down_corner {
                        bnst_inverse_tree[i as usize][k as usize] =
                            middle_corner
                    } else {
                        if !(bnst_inverse_tree[i as usize][k as usize] ==
                            b" \x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char) {
                            break;
                        }
                        bnst_inverse_tree[i as usize][k as usize] = link
                    }
                    k += 1
                }
            } else if bnst_inverse_tree[i as usize][j as usize] == up_corner {
                k = j - 1 as libc::c_int;
                while k > -(1 as libc::c_int) {
                    if bnst_inverse_tree[i as usize][k as usize] == up_corner
                    {
                        bnst_inverse_tree[i as usize][k as usize] =
                            middle_corner
                    } else {
                        if !(bnst_inverse_tree[i as usize][k as usize] ==
                            b" \x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char) {
                            break;
                        }
                        bnst_inverse_tree[i as usize][k as usize] = link
                    }
                    k -= 1
                }
            }
            j += 1
        }
        i += 1
    }
    /* inverse tree again and print */
    i = max_inverse_len - 1 as libc::c_int;
    while i > -(1 as libc::c_int) {
        if max_inverse_len - 1 as libc::c_int - i < 10 as libc::c_int {
            fprintf(Outfp, b"%d   \x00" as *const u8 as *const libc::c_char,
                    max_inverse_len - 1 as libc::c_int - i);
        } else if max_inverse_len - 1 as libc::c_int - i < 100 as libc::c_int
        {
            fprintf(Outfp, b"%d  \x00" as *const u8 as *const libc::c_char,
                    max_inverse_len - 1 as libc::c_int - i);
        } else {
            fprintf(Outfp, b"%d \x00" as *const u8 as *const libc::c_char,
                    max_inverse_len - 1 as libc::c_int - i);
        }
        j = 0 as libc::c_int;
        while j < max_len {
            if bnst_inverse_tree[j as usize][i as usize] !=
                b"***\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char {
                fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                        bnst_inverse_tree[j as usize][i as usize]);
            } else {
                fprintf(Outfp, b" \x00" as *const u8 as *const libc::c_char);
            }
            j += 1
        }
        fprintf(Outfp, b"\n\x00" as *const u8 as *const libc::c_char);
        i -= 1
    };
}
/*====================================================================
                               END
====================================================================*/
