#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;
use libc::{c_char, c_int};

use crate::{BNST_DATA, bnst_data, case_analysis, case_data, Chi_dpnd_matrix, Chi_pa_matrix, Chi_pos_matrix, ctools, dpnd_analysis, Dpnd_matrix, ErrorComment, feature, fprintf, free, Mask_matrix, para_dpnd, Para_matrix, printf, Quote_matrix, sprintf, sscanf, strcmp, strcpy, strdup, TAG_DATA, tnode_t, TOTAL_MGR, tree_conv, types};
use crate::case_analysis::{copy_cpm, find_best_cf, get_closest_case_component, init_case_frame, noun_lexical_disambiguation_by_case_analysis, pp_hstr_to_code, record_all_case_analisys, record_case_analysis, verb_lexical_disambiguation_by_case_analysis, Work_mgr};
use crate::case_data::{_make_data_cframe_ex, _make_data_cframe_pp, _make_data_cframe_sm, make_data_cframe_child};
use crate::case_ipal::{calc_adv_modifying_num_probability, calc_adv_modifying_probability, calc_vp_modifying_num_probability, calc_vp_modifying_probability, get_noun_co_ex_probability, get_noun_co_num_probability, get_para_ex_probability, get_para_exist_probability};
use crate::case_print::{print_crrspnd, print_data_cframe};
use crate::consts::CHINESE;
use crate::ctools::{abs, assign_cfeature, check_feature, Chi_np_end_matrix, Chi_np_start_matrix, Chi_quote_end_matrix, Chi_quote_start_matrix, Language, log, malloc, malloc_data, OptAnalysis, OptChiPos, stderr};
use crate::dpnd_analysis::{dpnd_info_to_bnst, dpnd_info_to_mrph, dpnd_info_to_tag, get_case_prob, get_case_prob_wpos, when_no_dpnd_struct};
use crate::feature::{delete_cfeature, feature_pattern_match};
use crate::lib_print::{do_postprocess, print_kakari, print_result};
use crate::para_analysis::para_recovery;
use crate::read_data::assign_general_feature;
use crate::read_rule::case2num;
use crate::tools::{Chi_root, Chi_root_prob_matrix, Chi_word_pos, Chi_word_type, cky_matrix, cky_table, left_arg, OptBeam, OptCaseFlag, OptChiGenerative, OptDisplay, OptExpress, OptNbest, OptParaFix, OptParaNoFixFlag, OptPostProcess, right_arg};
use crate::tree_conv::make_dpnd_tree;
use crate::types::{CASE_FRAME, CF_PRED_MGR, CKY, CKYptr, ELLIPSIS_MGR, SENTENCE_DATA};

#[no_mangle]
pub static mut cpm_allocated_cky_num: libc::c_int = -(1 as libc::c_int);

#[no_mangle]
pub unsafe extern "C" fn make_data_cframe_rentai_simple(mut pre_cpm_ptr: *mut CF_PRED_MGR,
                                                        mut d_ptr: *mut TAG_DATA,
                                                        mut t_ptr: *mut TAG_DATA) {
    if check_feature((*t_ptr).f,
                     b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                         as *const u8 as *const libc::c_char as
                         *mut libc::c_char).is_null() ||
        !check_feature((*d_ptr).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        _make_data_cframe_pp(pre_cpm_ptr, d_ptr, 0 as libc::c_int);
    } else {
        (*pre_cpm_ptr).cf.pp[(*pre_cpm_ptr).cf.element_num as
            usize][0 as libc::c_int as usize] =
            pp_hstr_to_code(b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                as *const u8 as *const libc::c_char as
                *mut libc::c_char);
        (*pre_cpm_ptr).cf.pp[(*pre_cpm_ptr).cf.element_num as
            usize][1 as libc::c_int as usize] =
            -(10 as libc::c_int);
        (*pre_cpm_ptr).cf.oblig[(*pre_cpm_ptr).cf.element_num as usize] =
            0 as libc::c_int
    }
    _make_data_cframe_sm(pre_cpm_ptr, t_ptr);
    _make_data_cframe_ex(pre_cpm_ptr, t_ptr);
    (*pre_cpm_ptr).elem_b_ptr[(*pre_cpm_ptr).cf.element_num as usize] = t_ptr;
    (*(*pre_cpm_ptr).elem_b_ptr[(*pre_cpm_ptr).cf.element_num as usize]).next
        = 0 as *mut tnode_t;
    (*pre_cpm_ptr).elem_b_num[(*pre_cpm_ptr).cf.element_num as usize] =
        -(1 as libc::c_int);
    (*pre_cpm_ptr).cf.weight[(*pre_cpm_ptr).cf.element_num as usize] =
        0 as libc::c_int;
    (*pre_cpm_ptr).cf.adjacent[(*pre_cpm_ptr).cf.element_num as usize] =
        0 as libc::c_int;
    (*pre_cpm_ptr).cf.element_num += 1;
}

#[no_mangle]
pub unsafe extern "C" fn recover_para_ptr_to_cpm(mut cpm_ptr:
                                                 *mut CF_PRED_MGR) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        if !(*cpm_ptr).para_b_ptr[i as usize].is_null() {
            (*(*cpm_ptr).elem_b_ptr[i as usize]).next =
                (*cpm_ptr).para_b_ptr[i as usize]
        } else {
            (*(*cpm_ptr).elem_b_ptr[i as usize]).next = 0 as *mut tnode_t
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn add_coordinated_phrases(mut cky_ptr: *mut CKY,
                                                 mut next: *mut *mut TAG_DATA)
                                                 -> *mut *mut TAG_DATA {
    while !cky_ptr.is_null() {
        if (*cky_ptr).para_flag != 0 ||
            (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
            break;
        }
        cky_ptr = (*cky_ptr).right
    }
    return if cky_ptr.is_null() {
        0 as *mut *mut TAG_DATA
    } else if (*cky_ptr).para_flag != 0 {
        add_coordinated_phrases((*cky_ptr).left,
                                add_coordinated_phrases((*cky_ptr).right,
                                                        next))
    } else if (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
        let mut next_pp: *mut *mut TAG_DATA = 0 as *mut *mut TAG_DATA;
        *next =
            (*(*(*cky_ptr).left).b_ptr).tag_ptr.offset((*(*(*cky_ptr).left).b_ptr).tag_num
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize));
        (**next).next = 0 as *mut tnode_t;
        next_pp =
            add_coordinated_phrases((*cky_ptr).right, &mut (**next).next);
        if !next_pp.is_null() {
            next_pp
        } else { &mut (**next).next }
    } else { 0 as *mut *mut TAG_DATA };
}

#[no_mangle]
pub unsafe extern "C" fn check_dpnd_possibility(mut sp: *mut SENTENCE_DATA,
                                                mut dep: libc::c_int,
                                                mut gov: libc::c_int,
                                                mut begin: libc::c_int,
                                                mut relax_flag: libc::c_int)
                                                -> libc::c_char {
    if OptParaFix == 0 as libc::c_int && begin >= 0 as libc::c_int &&
        (*(*sp).bnst_data.offset(dep as isize)).para_num !=
            -(1 as libc::c_int) &&
        (*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(dep as
            isize)).para_num
            as
            isize))[begin as
            usize][gov as
            usize]
            >= 0 as libc::c_int as libc::c_double ||
        OptParaFix == 1 as libc::c_int &&
            (*Mask_matrix.as_mut_ptr().offset(dep as isize))[gov as usize]
                == 2 as libc::c_int {
        return 'P' as i32 as libc::c_char;
    } else {
        if OptParaFix == 1 as libc::c_int &&
            (*Mask_matrix.as_mut_ptr().offset(dep as isize))[gov as usize]
                == 3 as libc::c_int {
            return 'I' as i32 as libc::c_char;
        } else {
            if (*Dpnd_matrix.as_mut_ptr().offset(dep as isize))[gov as usize]
                != 0 &&
                (*Quote_matrix.as_mut_ptr().offset(dep as
                    isize))[gov as
                    usize]
                    != 0 &&
                (Language != 2 as libc::c_int &&
                    (OptParaFix == 0 as libc::c_int ||
                        (*Mask_matrix.as_mut_ptr().offset(dep as
                            isize))[gov
                            as
                            usize]
                            == 1 as libc::c_int) ||
                    Language == 2 as libc::c_int &&
                        (*Mask_matrix.as_mut_ptr().offset(dep as
                            isize))[gov
                            as
                            usize]
                            != 0 as libc::c_int) {
                return (*Dpnd_matrix.as_mut_ptr().offset(dep as
                    isize))[gov as
                    usize]
                    as libc::c_char;
            } else {
                if ((*Dpnd_matrix.as_mut_ptr().offset(dep as
                    isize))[gov as
                    usize]
                    == 'R' as i32 || relax_flag != 0) &&
                    Language != 2 as libc::c_int {
                    return 'R' as i32 as libc::c_char;
                }
            }
        }
    }
    return '\u{0}' as i32 as libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn make_work_mgr_dpnd_check(mut sp: *mut SENTENCE_DATA,
                                                  mut cky_ptr: *mut CKY,
                                                  mut d_ptr: *mut BNST_DATA) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0;
    let mut tmp_d_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut tmp_t_ptr: *mut TAG_DATA =
        (*d_ptr).tag_ptr.offset((*d_ptr).tag_num as
            isize).offset(-(1 as libc::c_int as
            isize));
    while !tmp_t_ptr.is_null() {
        tmp_d_ptr = (*tmp_t_ptr).b_ptr;
        if !(*cky_ptr).right.is_null() &&
            (*(*cky_ptr).right).dpnd_type as libc::c_int == 'P' as i32 &&
            (*(*cky_ptr).right).j < (*tmp_d_ptr).num + 3 as libc::c_int {
            start = (*(*cky_ptr).right).j
        } else { start = (*tmp_d_ptr).num + 1 as libc::c_int }
        i = start;
        while i < (*sp).Bnst_num {
            if check_dpnd_possibility(sp, (*tmp_d_ptr).num, i,
                                      -(1 as libc::c_int),
                                      if i ==
                                          (*sp).Bnst_num - 1 as libc::c_int
                                          && count == 0 as libc::c_int {
                                          (0 as libc::c_int == 0) as
                                              libc::c_int
                                      } else { 0 as libc::c_int }) != 0 {
                Work_mgr.dpnd.check[(*tmp_d_ptr).num as
                    usize].pos[count as usize] = i;
                count += 1
            }
            i += 1
        }
        Work_mgr.dpnd.check[(*tmp_d_ptr).num as usize].num = count;
        tmp_t_ptr = (*tmp_t_ptr).next
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_work_mgr_dpnd_check_for_noun(mut sp:
                                                           *mut SENTENCE_DATA,
                                                           mut d_ptr:
                                                           *mut BNST_DATA) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = (*d_ptr).num + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            Work_mgr.dpnd.check[(*d_ptr).num as usize].pos[count as usize] =
                i;
            count += 1
        }
        i += 1
    }
    Work_mgr.dpnd.check[(*d_ptr).num as usize].num = count;
}

#[no_mangle]
pub unsafe extern "C" fn analyze_deverbative_noun_in_bunsetsu(mut sp: *mut SENTENCE_DATA, mut bp: *mut BNST_DATA) {
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut i: libc::c_int = 0;
    let mut child_num: libc::c_int = 0;
    let mut score: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < (*bp).tag_num - 1 as libc::c_int {
        tp = (*bp).tag_ptr.offset(i as isize);
        if !check_feature((*tp).f,
                          b"\xe9\x9d\x9e\xe7\x94\xa8\xe8\xa8\x80\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (*tp).cf_num > 0 as libc::c_int {
            cpm_ptr =
                malloc_data(::std::mem::size_of::<CF_PRED_MGR>() as
                                libc::c_ulong,
                            b"analyze_deverbative_noun_in_bunsetsu\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char) as *mut CF_PRED_MGR;
            init_case_frame(&mut (*cpm_ptr).cf);
            (*tp).cpm_ptr = cpm_ptr;
            (*cpm_ptr).pred_b_ptr = tp;
            (*cpm_ptr).score = -(1 as libc::c_int) as libc::c_double;
            (*cpm_ptr).result_num = 0 as libc::c_int;
            (*cpm_ptr).tie_num = 0 as libc::c_int;
            (*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr =
                0 as *mut CASE_FRAME;
            (*cpm_ptr).decided = 0 as libc::c_int;
            (*cpm_ptr).cf.pred_b_ptr = tp;
            (*cpm_ptr).cf.element_num = 0 as libc::c_int;
            case_data::set_data_cf_type(cpm_ptr);
            child_num = 0 as libc::c_int;
            if i > 0 as libc::c_int &&
                make_data_cframe_child(sp, cpm_ptr,
                                       tp.offset(-(1 as libc::c_int as
                                           isize)), child_num,
                                       0 as libc::c_int) != 0 {
                child_num += 1
            }
            make_data_cframe_rentai_simple(cpm_ptr, tp,
                                           tp.offset(1 as libc::c_int as
                                               isize));
            child_num += 1;
            score =
                find_best_cf(sp, cpm_ptr,
                             get_closest_case_component(sp, cpm_ptr),
                             0 as libc::c_int, 0 as *mut CF_PRED_MGR);
            record_case_analysis(sp, cpm_ptr, 0 as *mut ELLIPSIS_MGR,
                                 0 as libc::c_int);
            free(cpm_ptr as *mut libc::c_void);
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_dpnd(mut sp: *mut SENTENCE_DATA,
                                         mut Best_mgr: *mut TOTAL_MGR,
                                         mut cky_ptr: *mut CKY)
                                         -> libc::c_int {
    // let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptAnalysis == 1 as libc::c_int {
        if !(*(*cky_ptr).cpm_ptr).pred_b_ptr.is_null() &&
            (*Best_mgr).cpm[(*(*(*cky_ptr).cpm_ptr).pred_b_ptr).pred_num as
                usize].pred_b_ptr.is_null() {
            copy_cpm(&mut *(*Best_mgr).cpm.as_mut_ptr().offset((*(*(*cky_ptr).cpm_ptr).pred_b_ptr).pred_num
                as isize),
                     (*cky_ptr).cpm_ptr, 0 as libc::c_int);
            (*(*(*cky_ptr).cpm_ptr).pred_b_ptr).cpm_ptr =
                &mut *(*Best_mgr).cpm.as_mut_ptr().offset((*(*(*cky_ptr).cpm_ptr).pred_b_ptr).pred_num
                    as isize) as
                    *mut CF_PRED_MGR
        }
        if !(*cky_ptr).left.is_null() && !(*cky_ptr).right.is_null() &&
            !(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr.is_null() &&
            (!check_feature((*(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr).f,
                            b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr).f,
                               b"\xe5\xbc\xb7\xe8\xaa\xbf\xe6\xa7\x8b\xe6\x96\x87\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                    !check_feature((*(*(*cky_ptr).right).b_ptr).f,
                                   b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) {
            let mut cpm_ptr: *mut CF_PRED_MGR =
                &mut *(*Best_mgr).cpm.as_mut_ptr().offset((*(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr).pred_num
                    as isize) as
                    *mut CF_PRED_MGR;
            if (*cpm_ptr).pred_b_ptr.is_null() {
                copy_cpm(cpm_ptr, (*(*cky_ptr).left).cpm_ptr,
                         0 as libc::c_int);
                (*(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr).cpm_ptr = cpm_ptr
            }
            make_work_mgr_dpnd_check(sp, (*cky_ptr).left,
                                     (*(*cky_ptr).right).b_ptr);
            recover_para_ptr_to_cpm(cpm_ptr);
            make_data_cframe_rentai_simple(cpm_ptr, (*cpm_ptr).pred_b_ptr,
                                           (*(*(*cky_ptr).right).b_ptr).tag_ptr.offset((*(*(*cky_ptr).right).b_ptr).tag_num
                                               as
                                               isize).offset(-(1
                                               as
                                               libc::c_int
                                               as
                                               isize)));
            find_best_cf(sp, cpm_ptr, get_closest_case_component(sp, cpm_ptr),
                         (0 as libc::c_int == 0) as libc::c_int,
                         0 as *mut CF_PRED_MGR);
        }
        if OptCaseFlag & 524288 as libc::c_int != 0 &&
            !(*cky_ptr).left.is_null() &&
            (*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr.is_null() {
            analyze_deverbative_noun_in_bunsetsu(sp,
                                                 (*(*cky_ptr).left).b_ptr);
        }
    }
    if !(*cky_ptr).left.is_null() && !(*cky_ptr).right.is_null() {
        if OptDisplay == 3 as libc::c_int {
            printf(b"(%d, %d): (%d, %d) (%d, %d)\n\x00" as *const u8 as
                       *const libc::c_char, (*cky_ptr).i, (*cky_ptr).j,
                   (*(*cky_ptr).left).i, (*(*cky_ptr).left).j,
                   (*(*cky_ptr).right).i, (*(*cky_ptr).right).j);
        }
        if (*cky_ptr).para_flag == 0 as libc::c_int {
            if (*cky_ptr).dpnd_type as libc::c_int != 'P' as i32 &&
                {
                    cp =
                        check_feature((*(*(*cky_ptr).left).b_ptr).f,
                                      b"\xe4\xbf\x82:\xe7\x84\xa1\xe6\xa0\xbc\xe5\xbe\x93\xe5\xb1\x9e\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                    !cp.is_null()
                } {
                sscanf(cp,
                       b"%*[^:]:%*[^:]:%d\x00" as *const u8 as
                           *const libc::c_char,
                       &mut *(*Best_mgr).dpnd.head.as_mut_ptr().offset((*(*(*cky_ptr).left).b_ptr).num
                           as
                           isize)
                           as *mut libc::c_int);
                (*Best_mgr).dpnd.type_0[(*(*(*cky_ptr).left).b_ptr).num as
                    usize] =
                    'D' as i32 as libc::c_char
            } else if (*cky_ptr).direction == -(1 as libc::c_int) {
                (*Best_mgr).dpnd.head[(*(*(*cky_ptr).right).b_ptr).num as
                    usize] =
                    (*(*(*cky_ptr).left).b_ptr).num;
                (*Best_mgr).dpnd.type_0[(*(*(*cky_ptr).right).b_ptr).num as
                    usize] = (*cky_ptr).dpnd_type
            } else {
                (*Best_mgr).dpnd.head[(*(*(*cky_ptr).left).b_ptr).num as
                    usize] =
                    (*(*(*cky_ptr).right).b_ptr).num;
                (*Best_mgr).dpnd.type_0[(*(*(*cky_ptr).left).b_ptr).num as
                    usize] = (*cky_ptr).dpnd_type
            }
            if Language == 2 as libc::c_int &&
                (*cky_ptr).dpnd_type as libc::c_int != 'P' as i32 {
                if (*cky_ptr).para_score > 0 as libc::c_int as libc::c_double
                {
                    if (*cky_ptr).direction == -(1 as libc::c_int) {
                        (*Best_mgr).dpnd.head[(*(*(*cky_ptr).right).b_ptr).num
                            as usize] =
                            (*(*(*cky_ptr).left).b_ptr).num;
                        (*Best_mgr).dpnd.type_0[(*(*(*cky_ptr).right).b_ptr).num
                            as usize] =
                            (*cky_ptr).dpnd_type;
                        (*(*sp).bnst_data.offset((*(*(*cky_ptr).right).b_ptr).num
                            as isize)).is_para =
                            1 as libc::c_int;
                        (*(*sp).bnst_data.offset((*(*(*cky_ptr).left).b_ptr).num
                            as isize)).is_para =
                            2 as libc::c_int
                    } else {
                        (*Best_mgr).dpnd.head[(*(*(*cky_ptr).left).b_ptr).num
                            as usize] =
                            (*(*(*cky_ptr).right).b_ptr).num;
                        (*Best_mgr).dpnd.type_0[(*(*(*cky_ptr).left).b_ptr).num
                            as usize] =
                            (*cky_ptr).dpnd_type;
                        (*(*sp).bnst_data.offset((*(*(*cky_ptr).left).b_ptr).num
                            as isize)).is_para =
                            1 as libc::c_int;
                        (*(*sp).bnst_data.offset((*(*(*cky_ptr).right).b_ptr).num
                            as isize)).is_para =
                            2 as libc::c_int
                    }
                }
            }
        }
        convert_to_dpnd(sp, Best_mgr, (*cky_ptr).left);
        convert_to_dpnd(sp, Best_mgr, (*cky_ptr).right);
    } else if OptDisplay == 3 as libc::c_int {
        printf(b"(%d, %d)\n\x00" as *const u8 as *const libc::c_char,
               (*cky_ptr).i, (*cky_ptr).j);
    }
    dpnd_analysis::call_count_dpnd_candidates(sp, &mut (*Best_mgr).dpnd);
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn check_scase(mut g_ptr: *mut BNST_DATA,
                                     mut scase_check: *mut libc::c_int,
                                     mut rentai: libc::c_int,
                                     mut un_count: libc::c_int)
                                     -> libc::c_int {
    let mut vacant_slot_num: libc::c_int = 0 as libc::c_int;
    if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
        *const u8 as *const libc::c_char as
        *mut libc::c_char) as usize] as
        libc::c_int -
        *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char
            as *mut libc::c_char) as isize) ==
        1 as libc::c_int {
        vacant_slot_num += 1
    }
    if (*g_ptr).SCASE_code[case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
        *const u8 as *const libc::c_char as
        *mut libc::c_char) as usize] as
        libc::c_int -
        *scase_check.offset(case2num(b"\xe3\x83\xb2\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char
            as *mut libc::c_char) as isize) ==
        1 as libc::c_int {
        vacant_slot_num += 1
    }
    if (*g_ptr).SCASE_code[case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
        *const u8 as *const libc::c_char as
        *mut libc::c_char) as usize] as
        libc::c_int -
        *scase_check.offset(case2num(b"\xe3\x83\x8b\xe6\xa0\xbc\x00" as
            *const u8 as *const libc::c_char
            as *mut libc::c_char) as isize) ==
        1 as libc::c_int && rentai == 1 as libc::c_int &&
        !check_feature((*g_ptr).f,
                       b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x8b\x95\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        vacant_slot_num += 1
    }
    if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00" as
        *const u8 as *const libc::c_char as
        *mut libc::c_char) as usize] as
        libc::c_int -
        *scase_check.offset(case2num(b"\xe3\x82\xac\xef\xbc\x92\x00" as
            *const u8 as *const libc::c_char
            as *mut libc::c_char) as isize) ==
        1 as libc::c_int {
        vacant_slot_num += 1
    }
    return if rentai + un_count <= vacant_slot_num {
        (rentai + un_count) * 10 as libc::c_int
    } else { vacant_slot_num * 10 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn calc_score(mut sp: *mut SENTENCE_DATA,
                                    mut cky_ptr: *mut CKY) -> libc::c_double {
    // let mut right_ptr: *mut CKY = (*cky_ptr).right;
    let mut tmp_cky_ptr: *mut CKY = cky_ptr;
    // let mut tmp_child_ptr: *mut CKY = 0 as *mut CKY;
    let mut g_ptr: *mut BNST_DATA = (*cky_ptr).b_ptr;
    let mut d_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pred_p: libc::c_int = 0 as libc::c_int;
    let mut topic_score: libc::c_int = 0 as libc::c_int;
    let mut ha_check: libc::c_int = 0 as libc::c_int;
    let mut un_count: *mut libc::c_int = 0 as *mut libc::c_int;
    // let mut rentai: libc::c_int = 0;
    // let mut vacant_slot_num: libc::c_int = 0;
    let mut scase_check: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut default_pos: libc::c_int = 0;
    let mut verb: libc::c_int = 0;
    let mut comma: libc::c_int = 0;
    let mut one_score: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut prob: libc::c_double = 0.;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chi_pa_thre: libc::c_double = 0.;
    let mut weight_dpnd: libc::c_double = 0.;
    let mut weight_pos: libc::c_double = 0.;
    let mut weight_comma: libc::c_double = 0.;
    let mut weight_root: libc::c_double = 0.;
    let mut weight_pa: libc::c_double = 0.;
    let mut pos_prob_thre_high: libc::c_double = 0.;
    let mut pos_prob_thre_low: libc::c_double = 0.;
    let mut pos_occur_thre_high: libc::c_int = 0;
    let mut pos_occur_thre_low: libc::c_int = 0;
    let mut left_arg_num: libc::c_int = 0;
    let mut right_arg_num: libc::c_int = 0;
    let mut ptr_num: libc::c_int = 0;
    let mut chicase_prob: libc::c_double = 0.;
    let mut pre_pos_index: libc::c_int = 0;
    chi_pa_thre = 0.00005f64;
    weight_dpnd = 1.0f64;
    weight_pos = 0.5f64;
    weight_comma = 1.0f64;
    weight_root = 0.5f64;
    weight_pa = 1.0f64;
    pos_prob_thre_high = 0.95f64;
    pos_prob_thre_low = 0.05f64;
    pos_occur_thre_high = 100 as libc::c_int;
    pos_occur_thre_low = 50 as libc::c_int;
    while !tmp_cky_ptr.is_null() {
        if !if (*tmp_cky_ptr).direction == 1 as libc::c_int {
            (*tmp_cky_ptr).left
        } else { (*tmp_cky_ptr).right }.is_null() {
            one_score +=
                if (*tmp_cky_ptr).direction == 1 as libc::c_int {
                    (*(*tmp_cky_ptr).left).score
                } else { (*(*tmp_cky_ptr).right).score }
        }
        tmp_cky_ptr =
            if (*tmp_cky_ptr).direction == 1 as libc::c_int {
                (*tmp_cky_ptr).right
            } else { (*tmp_cky_ptr).left }
    }
    if OptDisplay == 3 as libc::c_int {
        if Language == 2 as libc::c_int {
            printf(b"%.6f=>\x00" as *const u8 as *const libc::c_char,
                   one_score);
        } else {
            printf(b"%.3f=>\x00" as *const u8 as *const libc::c_char,
                   one_score);
        }
    }
    if !check_feature((*g_ptr).f,
                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
        ||
        !check_feature((*g_ptr).f,
                       b"\xe6\xba\x96\xe7\x94\xa8\xe8\xa8\x80\x00" as
                           *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        pred_p = 1 as libc::c_int;
        k = 0 as libc::c_int;
        while k < 11 as libc::c_int {
            (*cky_ptr).scase_check[k as usize] = 0 as libc::c_int;
            k += 1
        }
        scase_check =
            &mut *(*cky_ptr).scase_check.as_mut_ptr().offset(0 as libc::c_int
                as isize) as
                *mut libc::c_int;
        (*cky_ptr).un_count = 0 as libc::c_int;
        un_count = &mut (*cky_ptr).un_count
    }
    while !cky_ptr.is_null() {
        if !if (*cky_ptr).direction == 1 as libc::c_int {
            (*cky_ptr).left
        } else { (*cky_ptr).right }.is_null() {
            d_ptr =
                if (*cky_ptr).direction == 1 as libc::c_int {
                    (*(*cky_ptr).left).b_ptr
                } else { (*(*cky_ptr).right).b_ptr };
            if !((*d_ptr).num < (*g_ptr).num &&
                ((*Mask_matrix.as_mut_ptr().offset((*d_ptr).num as
                    isize))[(*g_ptr).num
                    as
                    usize]
                    == 2 as libc::c_int ||
                    (*Mask_matrix.as_mut_ptr().offset((*d_ptr).num as
                        isize))[(*g_ptr).num
                        as
                        usize]
                        == 3 as libc::c_int) ||
                (*g_ptr).num < (*d_ptr).num &&
                    ((*Mask_matrix.as_mut_ptr().offset((*g_ptr).num as
                        isize))[(*d_ptr).num
                        as
                        usize]
                        == 2 as libc::c_int ||
                        (*Mask_matrix.as_mut_ptr().offset((*g_ptr).num
                            as
                            isize))[(*d_ptr).num
                            as
                            usize]
                            == 3 as libc::c_int)) {
                count = 0 as libc::c_int;
                pos = 0 as libc::c_int;
                verb = 0 as libc::c_int;
                comma = 0 as libc::c_int;
                if (*d_ptr).num < (*g_ptr).num {
                    i = (*d_ptr).num + 1 as libc::c_int;
                    while i < (*sp).Bnst_num {
                        if check_dpnd_possibility(sp, (*d_ptr).num, i,
                                                  (*cky_ptr).i,
                                                  if i ==
                                                      (*sp).Bnst_num -
                                                          1 as libc::c_int
                                                      &&
                                                      count ==
                                                          0 as libc::c_int
                                                  {
                                                      (0 as libc::c_int == 0)
                                                          as libc::c_int
                                                  } else { 0 as libc::c_int })
                            != 0 {
                            if i == (*g_ptr).num { pos = count }
                            count += 1
                        }
                        if !(i >= (*g_ptr).num) {
                            if Language == 2 as libc::c_int && OptChiPos == 0
                                &&
                                (!check_feature((*(*sp).bnst_data.offset(i
                                    as
                                    isize)).f,
                                                b"VV\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VA\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VC\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VE\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null())
                            {
                                verb += 1
                            }
                            if Language == 2 as libc::c_int && OptChiPos == 0 &&
                                !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"PU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() &&
                                (strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b",\x00" as *const u8 as *const libc::c_char) == 0 ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9a\x00" as *const u8 as *const libc::c_char) == 0 ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char) == 0 ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9b\x00" as *const u8 as *const libc::c_char) == 0 ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x8c\x00" as *const u8 as *const libc::c_char) == 0) {
                                comma += 1
                            }
                        }
                        i += 1
                    }
                } else {
                    i = (*d_ptr).num - 1 as libc::c_int;
                    while i >= 0 as libc::c_int {
                        if check_dpnd_possibility(sp, i, (*d_ptr).num,
                                                  (*cky_ptr).i,
                                                  0 as libc::c_int) != 0 {
                            if i == (*g_ptr).num { pos = count }
                            count += 1
                        }
                        if !(i <= (*g_ptr).num) {
                            if Language == 2 as libc::c_int && OptChiPos == 0
                                &&
                                (!check_feature((*(*sp).bnst_data.offset(i
                                    as
                                    isize)).f,
                                                b"VV\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VA\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VC\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                    ||
                                    !check_feature((*(*sp).bnst_data.offset(i
                                        as
                                        isize)).f,
                                                   b"VE\x00" as *const u8
                                                       as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null())
                            {
                                verb += 1
                            }
                            if Language == 2 as libc::c_int && OptChiPos == 0
                                &&
                                !check_feature((*(*sp).bnst_data.offset(i
                                    as
                                    isize)).f,
                                               b"PU\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null()
                                &&
                                (strcmp((*(*(*sp).bnst_data.offset(i as
                                    isize)).head_ptr).Goi.as_mut_ptr(),
                                        b",\x00" as *const u8 as
                                            *const libc::c_char) == 0 ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as
                                        isize)).head_ptr).Goi.as_mut_ptr(),
                                           b"\xef\xbc\x9a\x00" as
                                               *const u8 as
                                               *const libc::c_char) == 0
                                    ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as
                                        isize)).head_ptr).Goi.as_mut_ptr(),
                                           b":\x00" as *const u8 as
                                               *const libc::c_char) == 0
                                    ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as
                                        isize)).head_ptr).Goi.as_mut_ptr(),
                                           b"\xef\xbc\x9b\x00" as
                                               *const u8 as
                                               *const libc::c_char) == 0
                                    ||
                                    strcmp((*(*(*sp).bnst_data.offset(i as
                                        isize)).head_ptr).Goi.as_mut_ptr(),
                                           b"\xef\xbc\x8c\x00" as
                                               *const u8 as
                                               *const libc::c_char) == 0)
                            {
                                comma += 1
                            }
                        }
                        i -= 1
                    }
                }
                default_pos =
                    if (*(*d_ptr).dpnd_rule).preference == -(1 as libc::c_int)
                    {
                        count
                    } else { (*(*d_ptr).dpnd_rule).preference };
                if !check_feature((*d_ptr).f,
                                  b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    one_score -=
                        abs(default_pos - 1 as libc::c_int - pos) as
                            libc::c_double
                } else if Language != 2 as libc::c_int {
                    one_score -=
                        (abs(default_pos - 1 as libc::c_int - pos) *
                            2 as libc::c_int) as libc::c_double
                }
                if (*d_ptr).num + 1 as libc::c_int == (*g_ptr).num &&
                    abs(default_pos - 1 as libc::c_int - pos) >
                        0 as libc::c_int &&
                    !check_feature((*d_ptr).f,
                                   b"\xe8\xaa\xad\xe7\x82\xb9\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() {
                    one_score -= 5 as libc::c_int as libc::c_double
                }
            }
            if pred_p != 0 &&
                {
                    cp =
                        check_feature((*d_ptr).f,
                                      b"\xe4\xbf\x82\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                    !cp.is_null()
                } {
                if !check_feature((*d_ptr).f,
                                  b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                    strcmp(cp,
                           b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as
                               *const u8 as *const libc::c_char) == 0 {
                    cp2 =
                        check_feature((*g_ptr).f,
                                      b"\xe6\x8f\x90\xe9\xa1\x8c\xe5\x8f\x97\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char);
                    if !cp2.is_null() {
                        sscanf(cp2,
                               b"%*[^:]:%d\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut topic_score as *mut libc::c_int);
                        one_score += topic_score as libc::c_double
                    }
                    if !check_feature((*d_ptr).f,
                                      b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                        !check_feature((*d_ptr).f,
                                       b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                           *const u8 as *const libc::c_char
                                           as *mut libc::c_char).is_null()
                    {
                        one_score += 10 as libc::c_int as libc::c_double
                    } else if ha_check == 0 as libc::c_int {
                        one_score += 10 as libc::c_int as libc::c_double;
                        ha_check = 1 as libc::c_int
                    }
                }
                k =
                    case2num(cp.offset(3 as libc::c_int as
                        isize).offset(1 as libc::c_int as
                        isize));
                if strcmp(cp,
                          b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as
                              *const u8 as *const libc::c_char) == 0 {
                    if !check_feature((*d_ptr).f,
                                      b"\xe6\x99\x82\xe9\x96\x93\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() ||
                        !check_feature((*d_ptr).f,
                                       b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                           *const u8 as *const libc::c_char
                                           as *mut libc::c_char).is_null()
                    {
                        one_score += 10 as libc::c_int as libc::c_double
                    } else { *un_count += 1 }
                } else if strcmp(cp,
                                 b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    if check_feature((*g_ptr).f,
                                     b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                        if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                            as *const u8 as
                            *const libc::c_char
                            as
                            *mut libc::c_char)
                            as usize] as libc::c_int !=
                            0 &&
                            *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                as *const u8
                                as
                                *const libc::c_char
                                as
                                *mut libc::c_char)
                                as isize) ==
                                0 as libc::c_int {
                            one_score += 10 as libc::c_int as libc::c_double;
                            *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                                as *const u8 as
                                *const libc::c_char
                                as
                                *mut libc::c_char)
                                as isize) =
                                1 as libc::c_int
                        }
                    }
                } else if strcmp(cp,
                                 b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                     as *const u8 as *const libc::c_char) == 0
                {
                    if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                        as *const u8 as
                        *const libc::c_char as
                        *mut libc::c_char) as
                        usize] as libc::c_int != 0 &&
                        *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                            as *const u8 as
                            *const libc::c_char
                            as
                            *mut libc::c_char)
                            as isize) ==
                            0 as libc::c_int {
                        one_score += 10 as libc::c_int as libc::c_double;
                        *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                            as *const u8 as
                            *const libc::c_char
                            as *mut libc::c_char)
                            as isize) = 1 as libc::c_int
                    } else if (*g_ptr).SCASE_code[case2num(b"\xe3\x82\xac\xef\xbc\x92\x00"
                        as *const u8 as
                        *const libc::c_char
                        as
                        *mut libc::c_char)
                        as usize] as libc::c_int
                        != 0 &&
                        *scase_check.offset(case2num(b"\xe3\x82\xac\xef\xbc\x92\x00"
                            as
                            *const u8
                            as
                            *const libc::c_char
                            as
                            *mut libc::c_char)
                            as isize) ==
                            0 as libc::c_int {
                        one_score += 10 as libc::c_int as libc::c_double;
                        *scase_check.offset(case2num(b"\xe3\x82\xac\xe6\xa0\xbc\x00"
                            as *const u8 as
                            *const libc::c_char
                            as *mut libc::c_char)
                            as isize) = 1 as libc::c_int
                    }
                } else if k != -(1 as libc::c_int) {
                    if *scase_check.offset(k as isize) == 0 as libc::c_int {
                        *scase_check.offset(k as isize) = 1 as libc::c_int;
                        one_score += 10 as libc::c_int as libc::c_double
                    }
                }
                if !check_feature((*d_ptr).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                    (!check_feature((*d_ptr).f,
                                    b"\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                                        as *const u8 as *const libc::c_char
                                        as *mut libc::c_char).is_null() ||
                        !check_feature((*d_ptr).f,
                                       b"\xe4\xbf\x82:\xe3\x82\xac\xe6\xa0\xbc\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null())
                    &&
                    !check_feature((*g_ptr).f,
                                   b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null() {
                    one_score += 3 as libc::c_int as libc::c_double
                }
            }
            if !check_feature((*d_ptr).f,
                              b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                if !check_feature((*g_ptr).f,
                                  b"\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                    !check_feature((*g_ptr).f,
                                   b"\xe3\x83\xab\xe3\x83\xbc\xe3\x83\xab\xe5\xa4\x96\xe3\x81\xae\xe9\x96\xa2\xe4\xbf\x82\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null() {
                    one_score += 10 as libc::c_int as libc::c_double
                } else {
                    one_score +=
                        (check_scase(d_ptr,
                                     &mut *(*(*cky_ptr).left).scase_check.as_mut_ptr().offset(0
                                         as
                                         libc::c_int
                                         as
                                         isize),
                                     1 as libc::c_int,
                                     (*(*cky_ptr).left).un_count) -
                            check_scase(d_ptr,
                                        &mut *(*(*cky_ptr).left).scase_check.as_mut_ptr().offset(0
                                            as
                                            libc::c_int
                                            as
                                            isize),
                                        0 as libc::c_int,
                                        (*(*cky_ptr).left).un_count)) as
                            libc::c_double
                }
            }
            if Language == 2 as libc::c_int {
                if OptChiGenerative != 0 {
                    let mut det_head: *mut libc::c_char =
                        malloc(12 as libc::c_int as libc::c_ulong) as
                            *mut libc::c_char;
                    prob = 0 as libc::c_int as libc::c_double;
                    chicase_prob = 0 as libc::c_int as libc::c_double;
                    i = 0 as libc::c_int;
                    while i < 30 as libc::c_int + 1 as libc::c_int {
                        *left_arg.as_mut_ptr().offset(i as isize) =
                            -(1 as libc::c_int);
                        *right_arg.as_mut_ptr().offset(i as isize) =
                            -(1 as libc::c_int);
                        i += 1
                    }
                    ptr_num = -(1 as libc::c_int);
                    left_arg_num = 0 as libc::c_int;
                    right_arg_num = 0 as libc::c_int;
                    if (*cky_ptr).i == 0 as libc::c_int &&
                        (*cky_ptr).j == (*sp).Bnst_num {
                        sprintf(det_head,
                                b"DETHEAD_-1\x00" as *const u8 as
                                    *const libc::c_char);
                    } else {
                        sprintf(det_head,
                                b"DETHEAD_%i\x00" as *const u8 as
                                    *const libc::c_char, (*g_ptr).num);
                    }
                    if check_feature((*d_ptr).f, det_head).is_null() {
                        prob += log(0.3f64)
                    }
                    if !det_head.is_null() {
                        free(det_head as *mut libc::c_void);
                    }
                    if OptChiPos == 0 {
                        if (*cky_ptr).direction == 1 as libc::c_int {
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_LtoR[0
                                    as
                                    libc::c_int
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_dis_comma_LtoR[0
                                    as
                                    libc::c_int
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d prob:%f dis_comma:%f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*d_ptr).num,
                                       (*g_ptr).num,
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                           as
                                           isize))[(*g_ptr).num
                                           as
                                           usize].prob_LtoR[0
                                           as
                                           libc::c_int
                                           as
                                           usize],
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                           as
                                           isize))[(*g_ptr).num
                                           as
                                           usize].prob_dis_comma_LtoR[0
                                           as
                                           libc::c_int
                                           as
                                           usize],
                                       prob);
                            }
                            if (*cky_ptr).chicase_score +
                                1 as libc::c_int as libc::c_double >
                                -0.0000000000000001f64 &&
                                ((*cky_ptr).chicase_score +
                                    1 as libc::c_int as libc::c_double) <
                                    0.0000000000000001f64 {
                                if !(*cky_ptr).left.is_null() {
                                    ptr_num = (*(*(*cky_ptr).left).b_ptr).num;
                                    if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                        as
                                        isize)).head_ptr).Type.as_mut_ptr(),
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char) !=
                                        0 as libc::c_int {
                                        if strcmp((*(*(*sp).bnst_data.offset((*g_ptr).num
                                            as
                                            isize)).head_ptr).Type.as_mut_ptr(),
                                                  b"verb\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                            0 as libc::c_int ||
                                            left_arg_num <=
                                                0 as libc::c_int ||
                                            strcmp((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset((left_arg_num
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                isize)
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                   b"adv\x00" as *const u8
                                                       as
                                                       *const libc::c_char)
                                                != 0 as libc::c_int ||
                                            strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                   b"adv\x00" as *const u8
                                                       as
                                                       *const libc::c_char)
                                                != 0 as libc::c_int {
                                            *left_arg.as_mut_ptr().offset(left_arg_num
                                                as
                                                isize)
                                                = ptr_num;
                                            left_arg_num += 1;
                                            if left_arg_num >
                                                30 as libc::c_int {
                                                fprintf(stderr,
                                                        b";; number of arguments exceeded maximum\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                                return -999.0f64;
                                            }
                                        }
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).right;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        -(1 as libc::c_int) {
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    } else {
                                        if !(*tmp_cky_ptr).left.is_null() {
                                            ptr_num =
                                                (*(*(*tmp_cky_ptr).left).b_ptr).num;
                                            if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                    as
                                                    isize)).head_ptr).Type.as_mut_ptr(),
                                                          b"\x00" as *const u8
                                                              as
                                                              *const libc::c_char)
                                                    != 0 as libc::c_int {
                                                    if strcmp((*(*(*sp).bnst_data.offset((*g_ptr).num
                                                        as
                                                        isize)).head_ptr).Type.as_mut_ptr(),
                                                              b"verb\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char)
                                                        != 0 as libc::c_int
                                                        ||
                                                        left_arg_num <=
                                                            0 as
                                                                libc::c_int
                                                        ||
                                                        strcmp((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset((left_arg_num
                                                            -
                                                            1
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize)
                                                            as
                                                            isize)).head_ptr).Type.as_mut_ptr(),
                                                               b"adv\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char)
                                                            !=
                                                            0 as
                                                                libc::c_int
                                                        ||
                                                        strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                            as
                                                            isize)).head_ptr).Type.as_mut_ptr(),
                                                               b"adv\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char)
                                                            !=
                                                            0 as
                                                                libc::c_int
                                                    {
                                                        *left_arg.as_mut_ptr().offset(left_arg_num
                                                            as
                                                            isize)
                                                            = ptr_num;
                                                        left_arg_num += 1;
                                                        if left_arg_num >
                                                            30 as
                                                                libc::c_int
                                                        {
                                                            fprintf(stderr,
                                                                    b";; number of arguments exceeded maximum\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                            return -999.0f64;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    }
                                }
                                tmp_cky_ptr = cky_ptr;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        -(1 as libc::c_int) {
                                        if !(*tmp_cky_ptr).right.is_null() {
                                            ptr_num =
                                                (*(*(*tmp_cky_ptr).right).b_ptr).num;
                                            if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                *right_arg.as_mut_ptr().offset(right_arg_num
                                                    as
                                                    isize)
                                                    = ptr_num;
                                                right_arg_num += 1;
                                                if right_arg_num >
                                                    30 as libc::c_int {
                                                    fprintf(stderr,
                                                            b";; number of arguments exceeded maximum\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                    return -999.0f64;
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    } else {
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    }
                                }
                                if left_arg_num > 0 as libc::c_int ||
                                    right_arg_num > 0 as libc::c_int {
                                    (*cky_ptr).chicase_score =
                                        get_case_prob(sp, (*g_ptr).num,
                                                      left_arg_num,
                                                      right_arg_num)
                                } else { (*cky_ptr).chicase_score = 1.0f64 }
                            }
                            if (*cky_ptr).chicase_score >
                                0.0000000000000001f64 {
                                prob += log((*cky_ptr).chicase_score)
                            } else { prob += -999.0f64 }
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d chicase:%.6f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num, (*cky_ptr).chicase_score,
                                       prob);
                            }
                        } else if (*cky_ptr).direction == -(1 as libc::c_int)
                        {
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_RtoL[0
                                    as
                                    libc::c_int
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_dis_comma_RtoL[0
                                    as
                                    libc::c_int
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d prob:%f dis_comma:%f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num,
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                           as
                                           isize))[(*d_ptr).num
                                           as
                                           usize].prob_RtoL[0
                                           as
                                           libc::c_int
                                           as
                                           usize],
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                           as
                                           isize))[(*d_ptr).num
                                           as
                                           usize].prob_dis_comma_RtoL[0
                                           as
                                           libc::c_int
                                           as
                                           usize],
                                       prob);
                            }
                            if (*cky_ptr).chicase_score +
                                1 as libc::c_int as libc::c_double >
                                -0.0000000000000001f64 &&
                                ((*cky_ptr).chicase_score +
                                    1 as libc::c_int as libc::c_double) <
                                    0.0000000000000001f64 {
                                if !(*cky_ptr).right.is_null() {
                                    ptr_num =
                                        (*(*(*cky_ptr).right).b_ptr).num;
                                    if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                        as
                                        isize)).head_ptr).Type.as_mut_ptr(),
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char) !=
                                        0 as libc::c_int {
                                        *right_arg.as_mut_ptr().offset(right_arg_num
                                            as
                                            isize)
                                            = ptr_num;
                                        right_arg_num += 1;
                                        if right_arg_num > 30 as libc::c_int {
                                            fprintf(stderr,
                                                    b";; number of arguments exceeded maximum\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                            return -999.0f64;
                                        }
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).left;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        1 as libc::c_int {
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    } else {
                                        if !(*tmp_cky_ptr).right.is_null() {
                                            ptr_num =
                                                (*(*(*tmp_cky_ptr).right).b_ptr).num;
                                            if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                *right_arg.as_mut_ptr().offset(right_arg_num
                                                    as
                                                    isize)
                                                    = ptr_num;
                                                right_arg_num += 1;
                                                if right_arg_num >
                                                    30 as libc::c_int {
                                                    fprintf(stderr,
                                                            b";; number of arguments exceeded maximum\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                    return -999.0f64;
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).left;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        1 as libc::c_int {
                                        if !(*tmp_cky_ptr).left.is_null() {
                                            ptr_num =
                                                (*(*(*tmp_cky_ptr).left).b_ptr).num;
                                            if strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                as
                                                isize)).head_ptr).Type.as_mut_ptr(),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                if strcmp((*(*(*sp).bnst_data.offset((*g_ptr).num
                                                    as
                                                    isize)).head_ptr).Type.as_mut_ptr(),
                                                          b"verb\x00" as
                                                              *const u8 as
                                                              *const libc::c_char)
                                                    != 0 as libc::c_int ||
                                                    left_arg_num <=
                                                        0 as libc::c_int ||
                                                    strcmp((*(*(*sp).bnst_data.offset(*left_arg.as_mut_ptr().offset((left_arg_num
                                                        -
                                                        1
                                                            as
                                                            libc::c_int)
                                                        as
                                                        isize)
                                                        as
                                                        isize)).head_ptr).Type.as_mut_ptr(),
                                                           b"adv\x00" as
                                                               *const u8 as
                                                               *const libc::c_char)
                                                        != 0 as libc::c_int
                                                    ||
                                                    strcmp((*(*(*sp).bnst_data.offset(ptr_num
                                                        as
                                                        isize)).head_ptr).Type.as_mut_ptr(),
                                                           b"adv\x00" as
                                                               *const u8 as
                                                               *const libc::c_char)
                                                        != 0 as libc::c_int
                                                {
                                                    *left_arg.as_mut_ptr().offset(left_arg_num
                                                        as
                                                        isize)
                                                        = ptr_num;
                                                    left_arg_num += 1;
                                                    if left_arg_num >
                                                        30 as libc::c_int {
                                                        fprintf(stderr,
                                                                b";; number of arguments exceeded maximum\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                        return -999.0f64;
                                                    }
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    } else {
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    }
                                }
                                if left_arg_num > 0 as libc::c_int ||
                                    right_arg_num > 0 as libc::c_int {
                                    (*cky_ptr).chicase_score =
                                        get_case_prob(sp, (*g_ptr).num,
                                                      left_arg_num,
                                                      right_arg_num)
                                } else { (*cky_ptr).chicase_score = 1.0f64 }
                            }
                            if (*cky_ptr).chicase_score >
                                0.0000000000000001f64 {
                                prob += log((*cky_ptr).chicase_score)
                            } else { prob += -999.0f64 }
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d chicase:%.6f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num, (*cky_ptr).chicase_score,
                                       prob);
                            }
                        }
                        if (*cky_ptr).i == 0 as libc::c_int &&
                            (*cky_ptr).j ==
                                (*sp).Bnst_num - 1 as libc::c_int {
                            prob +=
                                log((*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize)).prob[0
                                    as
                                    libc::c_int
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(root:%.16f)%.16f=>\x00" as *const u8
                                           as *const libc::c_char,
                                       (*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                           as
                                           isize)).prob[0
                                           as
                                           libc::c_int
                                           as
                                           usize],
                                       prob);
                            }
                        }
                        one_score += prob
                    } else {
                        if (*cky_ptr).direction == 1 as libc::c_int {
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_LtoR[(*cky_ptr).index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_dis_comma_LtoR[(*cky_ptr).index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).left_pos_index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).right_pos_index
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d (%s,%s) prob:%f dis_comma:%f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*d_ptr).num,
                                       (*g_ptr).num,
                                       *Chi_word_pos.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                           as
                                           isize),
                                       *Chi_word_pos.as_mut_ptr().offset((*cky_ptr).right_pos_index
                                           as
                                           isize),
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                           as
                                           isize))[(*g_ptr).num
                                           as
                                           usize].prob_LtoR[(*cky_ptr).index
                                           as
                                           usize],
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                           as
                                           isize))[(*g_ptr).num
                                           as
                                           usize].prob_dis_comma_LtoR[(*cky_ptr).index
                                           as
                                           usize],
                                       prob);
                            }
                            if (*cky_ptr).chicase_score +
                                1 as libc::c_int as libc::c_double >
                                -0.0000000000000001f64 &&
                                ((*cky_ptr).chicase_score +
                                    1 as libc::c_int as libc::c_double) <
                                    0.0000000000000001f64 {
                                if !(*cky_ptr).left.is_null() {
                                    ptr_num = (*cky_ptr).left_pos_index;
                                    if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                        as
                                        isize),
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char) !=
                                        0 as libc::c_int {
                                        if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                            as
                                            isize),
                                                  b"verb\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                            0 as libc::c_int ||
                                            left_arg_num <=
                                                0 as libc::c_int ||
                                            strcmp(*Chi_word_type.as_mut_ptr().offset(pre_pos_index
                                                as
                                                isize),
                                                   b"adv\x00" as *const u8
                                                       as
                                                       *const libc::c_char)
                                                != 0 as libc::c_int ||
                                            strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                                as
                                                isize),
                                                   b"adv\x00" as *const u8
                                                       as
                                                       *const libc::c_char)
                                                != 0 as libc::c_int {
                                            *left_arg.as_mut_ptr().offset(left_arg_num
                                                as
                                                isize)
                                                = ptr_num;
                                            pre_pos_index =
                                                (*cky_ptr).left_pos_index;
                                            left_arg_num += 1;
                                            if left_arg_num >
                                                30 as libc::c_int {
                                                fprintf(stderr,
                                                        b";; number of arguments exceeded maximum\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                                return -999.0f64;
                                            }
                                        }
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).right;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        -(1 as libc::c_int) {
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    } else {
                                        if !(*tmp_cky_ptr).left.is_null() {
                                            ptr_num =
                                                (*tmp_cky_ptr).left_pos_index;
                                            if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                                as
                                                isize),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                                    as
                                                    isize),
                                                          b"\x00" as *const u8
                                                              as
                                                              *const libc::c_char)
                                                    != 0 as libc::c_int {
                                                    if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                                        as
                                                        isize),
                                                              b"verb\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char)
                                                        != 0 as libc::c_int
                                                        ||
                                                        left_arg_num <=
                                                            0 as
                                                                libc::c_int
                                                        ||
                                                        strcmp(*Chi_word_type.as_mut_ptr().offset(pre_pos_index
                                                            as
                                                            isize),
                                                               b"adv\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char)
                                                            !=
                                                            0 as
                                                                libc::c_int
                                                        ||
                                                        strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                                            as
                                                            isize),
                                                               b"adv\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char)
                                                            !=
                                                            0 as
                                                                libc::c_int
                                                    {
                                                        *left_arg.as_mut_ptr().offset(left_arg_num
                                                            as
                                                            isize)
                                                            = ptr_num;
                                                        pre_pos_index =
                                                            (*cky_ptr).left_pos_index;
                                                        left_arg_num += 1;
                                                        if left_arg_num >
                                                            30 as
                                                                libc::c_int
                                                        {
                                                            fprintf(stderr,
                                                                    b";; number of arguments exceeded maximum\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                            return -999.0f64;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    }
                                }
                                tmp_cky_ptr = cky_ptr;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        -(1 as libc::c_int) {
                                        if !(*tmp_cky_ptr).right.is_null() {
                                            ptr_num =
                                                (*tmp_cky_ptr).right_pos_index;
                                            if strcmp(*Chi_word_type.as_mut_ptr().offset((*tmp_cky_ptr).right_pos_index
                                                as
                                                isize),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                *right_arg.as_mut_ptr().offset(right_arg_num
                                                    as
                                                    isize)
                                                    = ptr_num;
                                                right_arg_num += 1;
                                                if right_arg_num >
                                                    30 as libc::c_int {
                                                    fprintf(stderr,
                                                            b";; number of arguments exceeded maximum\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                    return -999.0f64;
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    } else {
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    }
                                }
                                if left_arg_num > 0 as libc::c_int ||
                                    right_arg_num > 0 as libc::c_int {
                                    (*cky_ptr).chicase_score =
                                        get_case_prob_wpos(sp, (*g_ptr).num,
                                                           left_arg_num,
                                                           right_arg_num,
                                                           (*cky_ptr).right_pos_index)
                                } else { (*cky_ptr).chicase_score = 1.0f64 }
                            }
                            if (*cky_ptr).chicase_score >
                                0.0000000000000001f64 {
                                prob += log((*cky_ptr).chicase_score)
                            } else { prob += -999.0f64 }
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).right_pos_index
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d chicase:%.6f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num, (*cky_ptr).chicase_score,
                                       prob);
                            }
                            if (*cky_ptr).i == 0 as libc::c_int &&
                                (*cky_ptr).j ==
                                    (*sp).Bnst_num - 1 as libc::c_int {
                                prob +=
                                    log((*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                        as
                                        isize)).prob[(*cky_ptr).index
                                        as
                                        usize]);
                                prob +=
                                    log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                        as
                                        isize)).prob_pos_index[(*cky_ptr).right_pos_index
                                        as
                                        usize]);
                                if OptDisplay == 3 as libc::c_int {
                                    printf(b"(root:%.16f)%.16f=>\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           (*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                               as
                                               isize)).prob[(*cky_ptr).index
                                               as
                                               usize],
                                           prob);
                                }
                            }
                        } else if (*cky_ptr).direction == -(1 as libc::c_int)
                        {
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_RtoL[(*cky_ptr).index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_dis_comma_RtoL[(*cky_ptr).index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).right_pos_index
                                    as
                                    usize]);
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).left_pos_index
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d (%s,%s) prob:%f dis_comma:%f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num,
                                       *Chi_word_pos.as_mut_ptr().offset((*cky_ptr).left_pos_index
                                           as
                                           isize),
                                       *Chi_word_pos.as_mut_ptr().offset((*cky_ptr).right_pos_index
                                           as
                                           isize),
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                           as
                                           isize))[(*d_ptr).num
                                           as
                                           usize].prob_RtoL[(*cky_ptr).index
                                           as
                                           usize],
                                       (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                           as
                                           isize))[(*d_ptr).num
                                           as
                                           usize].prob_dis_comma_RtoL[(*cky_ptr).index
                                           as
                                           usize],
                                       prob);
                            }
                            if (*cky_ptr).chicase_score +
                                1 as libc::c_int as libc::c_double >
                                -0.0000000000000001f64 &&
                                ((*cky_ptr).chicase_score +
                                    1 as libc::c_int as libc::c_double) <
                                    0.0000000000000001f64 {
                                if !(*cky_ptr).right.is_null() {
                                    ptr_num = (*cky_ptr).right_pos_index;
                                    if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).right_pos_index
                                        as
                                        isize),
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char) !=
                                        0 as libc::c_int {
                                        *right_arg.as_mut_ptr().offset(right_arg_num
                                            as
                                            isize)
                                            = ptr_num;
                                        right_arg_num += 1;
                                        if right_arg_num > 30 as libc::c_int {
                                            fprintf(stderr,
                                                    b";; number of arguments exceeded maximum\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                            return -999.0f64;
                                        }
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).left;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        1 as libc::c_int {
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    } else {
                                        if !(*tmp_cky_ptr).right.is_null() {
                                            ptr_num =
                                                (*tmp_cky_ptr).right_pos_index;
                                            if strcmp(*Chi_word_type.as_mut_ptr().offset((*tmp_cky_ptr).left_pos_index
                                                as
                                                isize),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                *right_arg.as_mut_ptr().offset(right_arg_num
                                                    as
                                                    isize)
                                                    = ptr_num;
                                                right_arg_num += 1;
                                                if right_arg_num >
                                                    30 as libc::c_int {
                                                    fprintf(stderr,
                                                            b";; number of arguments exceeded maximum\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                    return -999.0f64;
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    }
                                }
                                tmp_cky_ptr = (*cky_ptr).left;
                                while !tmp_cky_ptr.is_null() {
                                    if (*tmp_cky_ptr).direction ==
                                        1 as libc::c_int {
                                        if !(*tmp_cky_ptr).left.is_null() {
                                            ptr_num =
                                                (*tmp_cky_ptr).left_pos_index;
                                            if strcmp(*Chi_word_type.as_mut_ptr().offset((*tmp_cky_ptr).left_pos_index
                                                as
                                                isize),
                                                      b"\x00" as *const u8 as
                                                          *const libc::c_char)
                                                != 0 as libc::c_int {
                                                if strcmp(*Chi_word_type.as_mut_ptr().offset((*cky_ptr).right_pos_index
                                                    as
                                                    isize),
                                                          b"verb\x00" as
                                                              *const u8 as
                                                              *const libc::c_char)
                                                    != 0 as libc::c_int ||
                                                    left_arg_num <=
                                                        0 as libc::c_int ||
                                                    strcmp(*Chi_word_type.as_mut_ptr().offset(pre_pos_index
                                                        as
                                                        isize),
                                                           b"adv\x00" as
                                                               *const u8 as
                                                               *const libc::c_char)
                                                        != 0 as libc::c_int
                                                    ||
                                                    strcmp(*Chi_word_type.as_mut_ptr().offset((*tmp_cky_ptr).left_pos_index
                                                        as
                                                        isize),
                                                           b"adv\x00" as
                                                               *const u8 as
                                                               *const libc::c_char)
                                                        != 0 as libc::c_int
                                                {
                                                    *left_arg.as_mut_ptr().offset(left_arg_num
                                                        as
                                                        isize)
                                                        = ptr_num;
                                                    pre_pos_index =
                                                        (*tmp_cky_ptr).left_pos_index;
                                                    left_arg_num += 1;
                                                    if left_arg_num >
                                                        30 as libc::c_int {
                                                        fprintf(stderr,
                                                                b";; number of arguments exceeded maximum\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                        return -999.0f64;
                                                    }
                                                }
                                            }
                                        }
                                        tmp_cky_ptr = (*tmp_cky_ptr).right
                                    } else {
                                        tmp_cky_ptr = (*tmp_cky_ptr).left
                                    }
                                }
                                if left_arg_num > 0 as libc::c_int ||
                                    right_arg_num > 0 as libc::c_int {
                                    (*cky_ptr).chicase_score =
                                        get_case_prob_wpos(sp, (*g_ptr).num,
                                                           left_arg_num,
                                                           right_arg_num,
                                                           (*cky_ptr).left_pos_index)
                                } else { (*cky_ptr).chicase_score = 1.0f64 }
                            }
                            if (*cky_ptr).chicase_score >
                                0.0000000000000001f64 {
                                prob += log((*cky_ptr).chicase_score)
                            } else { prob += -999.0f64 }
                            prob +=
                                log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize)).prob_pos_index[(*cky_ptr).left_pos_index
                                    as
                                    usize]);
                            if OptDisplay == 3 as libc::c_int {
                                printf(b"(dpnd:%d,%d chicase:%.6f)%.6f=>\x00"
                                           as *const u8 as
                                           *const libc::c_char, (*g_ptr).num,
                                       (*d_ptr).num, (*cky_ptr).chicase_score,
                                       prob);
                            }
                            if (*cky_ptr).i == 0 as libc::c_int &&
                                (*cky_ptr).j ==
                                    (*sp).Bnst_num - 1 as libc::c_int {
                                prob +=
                                    log((*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                        as
                                        isize)).prob[(*cky_ptr).index
                                        as
                                        usize]);
                                prob +=
                                    log((*Chi_pos_matrix.as_mut_ptr().offset((*g_ptr).num
                                        as
                                        isize)).prob_pos_index[(*cky_ptr).left_pos_index
                                        as
                                        usize]);
                                if OptDisplay == 3 as libc::c_int {
                                    printf(b"(root:%.16f)%.16f=>\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           (*Chi_root_prob_matrix.as_mut_ptr().offset((*g_ptr).num
                                               as
                                               isize)).prob[(*cky_ptr).index
                                               as
                                               usize],
                                           prob);
                                }
                            }
                        }
                        one_score += prob
                    }
                } else if (*cky_ptr).direction == 1 as libc::c_int {
                    one_score +=
                        weight_dpnd *
                            (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                as
                                isize))[(*g_ptr).num
                                as
                                usize].prob_LtoR[(*cky_ptr).index
                                as
                                usize];
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(dpnd:%d,%d %f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, (*d_ptr).num,
                               (*g_ptr).num,
                               (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                   as
                                   isize))[(*g_ptr).num
                                   as
                                   usize].prob_LtoR[(*cky_ptr).index
                                   as
                                   usize],
                               one_score);
                    }
                    if (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num as
                        isize))[(*g_ptr).num
                        as
                        usize].prob_pos_LtoR
                        >= pos_prob_thre_high &&
                        (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                            as
                            isize))[(*g_ptr).num
                            as
                            usize].occur_pos
                            >= pos_occur_thre_high as libc::c_double {
                        one_score +=
                            weight_pos *
                                (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_pos_LtoR
                    } else if (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                        as
                        isize))[(*g_ptr).num
                        as
                        usize].occur_pos
                        <= pos_occur_thre_low as libc::c_double {
                        one_score -=
                            weight_pos *
                                (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize].prob_pos_LtoR
                    } else if (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                        as
                        isize))[(*g_ptr).num
                        as
                        usize].prob_pos_LtoR
                        <= pos_prob_thre_low {
                        one_score -=
                            weight_pos *
                                (1 as libc::c_int as libc::c_double -
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                        as
                                        isize))[(*g_ptr).num
                                        as
                                        usize].prob_pos_LtoR)
                    }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(pos:%f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char,
                               (*Chi_dpnd_matrix.as_mut_ptr().offset((*d_ptr).num
                                   as
                                   isize))[(*g_ptr).num
                                   as
                                   usize].prob_pos_LtoR,
                               one_score);
                    }
                    one_score -=
                        weight_comma * (1.0f64 * comma as libc::c_double) /
                            (comma + 1 as libc::c_int) as libc::c_double;
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(comma:%d)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, comma, one_score);
                    }
                    if (*d_ptr).num == Chi_root { one_score -= weight_root }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(root)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, one_score);
                    }
                    if (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num as
                        isize))[(*g_ptr).num
                        as
                        usize]
                        >= chi_pa_thre {
                        one_score +=
                            weight_pa *
                                (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize]
                    }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(pa:%f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char,
                               (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num
                                   as
                                   isize))[(*g_ptr).num
                                   as
                                   usize],
                               one_score);
                    }
                } else if (*cky_ptr).direction == -(1 as libc::c_int) {
                    one_score +=
                        weight_dpnd *
                            (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                as
                                isize))[(*d_ptr).num
                                as
                                usize].prob_RtoL[(*cky_ptr).index
                                as
                                usize];
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(dpnd:%d,%d %f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, (*d_ptr).num,
                               (*g_ptr).num,
                               (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                   as
                                   isize))[(*d_ptr).num
                                   as
                                   usize].prob_RtoL[(*cky_ptr).index
                                   as
                                   usize],
                               one_score);
                    }
                    if (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num as
                        isize))[(*d_ptr).num
                        as
                        usize].prob_pos_RtoL
                        >= pos_prob_thre_high &&
                        (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                            as
                            isize))[(*d_ptr).num
                            as
                            usize].occur_pos
                            >= pos_occur_thre_high as libc::c_double {
                        one_score +=
                            weight_pos *
                                (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_pos_RtoL
                    } else if (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                        as
                        isize))[(*d_ptr).num
                        as
                        usize].occur_pos
                        <= pos_occur_thre_low as libc::c_double {
                        one_score -=
                            weight_pos *
                                (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                    as
                                    isize))[(*d_ptr).num
                                    as
                                    usize].prob_pos_RtoL
                    } else if (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                        as
                        isize))[(*d_ptr).num
                        as
                        usize].prob_pos_RtoL
                        <= pos_prob_thre_low {
                        one_score -=
                            weight_pos *
                                (1 as libc::c_int as libc::c_double -
                                    (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                        as
                                        isize))[(*d_ptr).num
                                        as
                                        usize].prob_pos_RtoL)
                    }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(pos: %f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char,
                               (*Chi_dpnd_matrix.as_mut_ptr().offset((*g_ptr).num
                                   as
                                   isize))[(*d_ptr).num
                                   as
                                   usize].prob_pos_RtoL,
                               one_score);
                    }
                    one_score -=
                        weight_comma * (1.0f64 * comma as libc::c_double) /
                            (comma + 1 as libc::c_int) as libc::c_double;
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(comma: %d)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, comma, one_score);
                    }
                    if (*d_ptr).num == Chi_root { one_score -= weight_root }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(root)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char, one_score);
                    }
                    if (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num as
                        isize))[(*g_ptr).num
                        as
                        usize]
                        >= chi_pa_thre {
                        one_score +=
                            weight_pa *
                                (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num
                                    as
                                    isize))[(*g_ptr).num
                                    as
                                    usize]
                    }
                    if OptDisplay == 3 as libc::c_int {
                        printf(b"(pa:%f)%.6f=>\x00" as *const u8 as
                                   *const libc::c_char,
                               (*Chi_pa_matrix.as_mut_ptr().offset((*d_ptr).num
                                   as
                                   isize))[(*g_ptr).num
                                   as
                                   usize],
                               one_score);
                    }
                }
            }
        }
        cky_ptr =
            if (*cky_ptr).direction == 1 as libc::c_int {
                (*cky_ptr).right
            } else { (*cky_ptr).left }
    }
    if pred_p != 0 {
        one_score +=
            check_scase(g_ptr, scase_check, 0 as libc::c_int, *un_count) as
                libc::c_double
    }
    if OptDisplay == 3 as libc::c_int {
        if Language == 2 as libc::c_int {
            printf(b"%.6f\n\x00" as *const u8 as *const libc::c_char,
                   one_score);
        } else {
            printf(b"%.3f\n\x00" as *const u8 as *const libc::c_char,
                   one_score);
        }
    }
    return one_score;
}

#[no_mangle]
pub unsafe extern "C" fn count_distance(mut sp: *mut SENTENCE_DATA,
                                        mut cky_ptr: *mut CKY,
                                        mut g_ptr: *mut BNST_DATA,
                                        mut pos: *mut libc::c_int)
                                        -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    *pos = 0 as libc::c_int;
    i = (*(*(*cky_ptr).left).b_ptr).num + 1 as libc::c_int;
    while i < (*sp).Bnst_num {
        if check_dpnd_possibility(sp, (*(*(*cky_ptr).left).b_ptr).num, i,
                                  (*cky_ptr).i,
                                  if i == (*sp).Bnst_num - 1 as libc::c_int &&
                                      count == 0 as libc::c_int {
                                      (0 as libc::c_int == 0) as libc::c_int
                                  } else { 0 as libc::c_int }) != 0 {
            if i == (*g_ptr).num { *pos = count }
            count += 1
        }
        i += 1
    }
    return count;
}

#[no_mangle]
pub unsafe extern "C" fn add_internal_bunsetsu_child(mut sp:
                                                     *mut SENTENCE_DATA,
                                                     mut cpm_ptr:
                                                     *mut CF_PRED_MGR,
                                                     mut t_ptr: *mut TAG_DATA,
                                                     mut g_ptr:
                                                     *mut BNST_DATA,
                                                     mut child_num:
                                                     libc::c_int)
                                                     -> libc::c_int {
    return if !(*cpm_ptr).pred_b_ptr.is_null() &&
        t_ptr.offset(-(1 as libc::c_int as isize)) > (*g_ptr).tag_ptr &&
        make_data_cframe_child(sp, cpm_ptr,
                               t_ptr.offset(-(1 as libc::c_int as isize)),
                               child_num, 0 as libc::c_int) != 0 {
        1 as libc::c_int
    } else { 0 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn calc_case_probability(mut sp: *mut SENTENCE_DATA,
                                               mut cky_ptr: *mut CKY,
                                               mut Best_mgr: *mut TOTAL_MGR)
                                               -> libc::c_double {
    // let mut right_ptr: *mut CKY = (*cky_ptr).right;
    let mut orig_cky_ptr: *mut CKY = cky_ptr;
    let mut para_cky_ptr: *mut CKY = 0 as *mut CKY;
    let mut g_ptr: *mut BNST_DATA = (*cky_ptr).b_ptr;
    let mut d_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    let mut t_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut dt_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut pre_cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    // let mut i: libc::c_int = 0;
    let mut pred_p: libc::c_int = 0 as libc::c_int;
    let mut child_num: libc::c_int = 0 as libc::c_int;
    let mut wo_ni_overwritten_flag: libc::c_int = 0 as libc::c_int;
    let mut renyou_modifying_num: libc::c_int = 0 as libc::c_int;
    let mut adverb_modifying_num: libc::c_int = 0 as libc::c_int;
    let mut noun_modifying_num: libc::c_int = 0 as libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut one_score: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut orig_score: libc::c_double = 0.;
    let mut case_analysis_score: libc::c_double = 0.;
    let mut para_key: *mut libc::c_char = 0 as *mut libc::c_char;
    while !cky_ptr.is_null() {
        if !(*cky_ptr).left.is_null() {
            one_score += (*(*cky_ptr).left).score
        }
        cky_ptr = (*cky_ptr).right
    }
    if OptDisplay == 3 as libc::c_int {
        printf(b"%.3f=>\x00" as *const u8 as *const libc::c_char, one_score);
    }
    cky_ptr = orig_cky_ptr;
    if !check_feature((*g_ptr).f,
                      b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97:-1\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() &&
        (*g_ptr).tag_num > 1 as libc::c_int {
        t_ptr =
            (*g_ptr).tag_ptr.offset((*g_ptr).tag_num as
                isize).offset(-(2 as libc::c_int as
                isize))
    } else {
        t_ptr =
            (*g_ptr).tag_ptr.offset((*g_ptr).tag_num as
                isize).offset(-(1 as libc::c_int as
                isize))
    }
    if (*t_ptr).cf_num > 0 as libc::c_int {
        (*(*cky_ptr).cpm_ptr).pred_b_ptr = t_ptr;
        case_data::set_data_cf_type((*cky_ptr).cpm_ptr);
        if ((*(*cky_ptr).cpm_ptr).cf.type_0 == 1 as libc::c_int ||
            OptCaseFlag & 4096 as libc::c_int != 0) &&
            !((*cky_ptr).i == (*cky_ptr).j &&
                !check_feature((*g_ptr).f,
                               b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
            pred_p = 1 as libc::c_int;
            cpm_ptr = (*cky_ptr).cpm_ptr;
            (*cpm_ptr).score = -(1 as libc::c_int) as libc::c_double;
            (*cpm_ptr).result_num = 0 as libc::c_int;
            (*cpm_ptr).tie_num = 0 as libc::c_int;
            (*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr =
                0 as *mut CASE_FRAME;
            (*cpm_ptr).decided = 0 as libc::c_int;
            (*cpm_ptr).cf.pred_b_ptr = t_ptr;
            (*t_ptr).cpm_ptr = cpm_ptr;
            (*cpm_ptr).cf.element_num = 0 as libc::c_int
        } else {
            (*(*cky_ptr).cpm_ptr).pred_b_ptr = 0 as *mut TAG_DATA;
            (*(*cky_ptr).cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr =
                0 as *mut CASE_FRAME
        }
    } else {
        (*(*cky_ptr).cpm_ptr).pred_b_ptr = 0 as *mut TAG_DATA;
        (*(*cky_ptr).cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr =
            0 as *mut CASE_FRAME
    }
    while !cky_ptr.is_null() {
        if !(*cky_ptr).left.is_null() &&
            (*cky_ptr).para_flag == 0 as libc::c_int {
            d_ptr = (*(*cky_ptr).left).b_ptr;
            dt_ptr =
                (*d_ptr).tag_ptr.offset((*d_ptr).tag_num as
                    isize).offset(-(1 as libc::c_int
                    as isize));
            flag = 0 as libc::c_int;
            if (*(*cky_ptr).left).i == (*(*cky_ptr).left).j &&
                !check_feature((*d_ptr).f,
                               b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                assign_cfeature(&mut (*d_ptr).f,
                                b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                assign_cfeature(&mut (*dt_ptr).f,
                                b"\xe4\xbf\x82:\xe3\x83\x8b\xe6\xa0\xbc\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                assign_cfeature(&mut (*dt_ptr).f,
                                b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\xe3\x83\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                delete_cfeature(&mut (*dt_ptr).f,
                                b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                wo_ni_overwritten_flag = 1 as libc::c_int
            } else { wo_ni_overwritten_flag = 0 as libc::c_int }
            if (*cky_ptr).dpnd_type as libc::c_int == 'R' as i32 {
                one_score += -(1000 as libc::c_int) as libc::c_double
            }
            if OptParaFix == 0 as libc::c_int {
                if (*d_ptr).para_num != -(1 as libc::c_int) &&
                    {
                        para_key =
                            check_feature((*d_ptr).f,
                                          b"\xe4\xb8\xa6\xe3\x82\xad\x00"
                                              as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                        !para_key.is_null()
                    } {
                    make_work_mgr_dpnd_check_for_noun(sp, d_ptr);
                    if (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
                        one_score +=
                            get_para_exist_probability(para_key,
                                                       (*cky_ptr).para_score,
                                                       (0 as libc::c_int == 0)
                                                           as libc::c_int,
                                                       (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                           as
                                                           isize).offset(-(1
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)),
                                                       t_ptr);
                        if OptParaNoFixFlag & 1 as libc::c_int != 0 ||
                            !check_feature((*d_ptr).f,
                                           b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null()
                        {
                            one_score +=
                                get_para_ex_probability(para_key,
                                                        (*cky_ptr).para_score,
                                                        (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                            as
                                                            isize).offset(-(1
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize)),
                                                        t_ptr)
                        } else if OptParaNoFixFlag & 8 as libc::c_int != 0 {
                            one_score +=
                                get_para_ex_probability(para_key,
                                                        (*cky_ptr).para_score,
                                                        (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                            as
                                                            isize).offset(-(1
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize)),
                                                        t_ptr) /
                                    2 as libc::c_int as libc::c_double
                        }
                        flag += 1
                    } else {
                        one_score +=
                            get_para_exist_probability(para_key,
                                                       (*(*sp).para_data.offset((*d_ptr).para_num
                                                           as
                                                           isize)).max_score
                                                           as libc::c_double,
                                                       0 as libc::c_int,
                                                       (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                           as
                                                           isize).offset(-(1
                                                           as
                                                           libc::c_int
                                                           as
                                                           isize)),
                                                       t_ptr)
                    }
                }
            }
            if (*cky_ptr).dpnd_type as libc::c_int != 'P' as i32 &&
                pred_p != 0 {
                make_work_mgr_dpnd_check(sp, cky_ptr, d_ptr);
                if make_data_cframe_child(sp, cpm_ptr,
                                          (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                              as
                                              isize).offset(-(1
                                              as
                                              libc::c_int
                                              as
                                              isize)),
                                          child_num,
                                          if (*t_ptr).num ==
                                              (*d_ptr).num +
                                                  1 as libc::c_int {
                                              (0 as libc::c_int == 0) as
                                                  libc::c_int
                                          } else { 0 as libc::c_int }) != 0 {
                    add_coordinated_phrases((*cky_ptr).left,
                                            &mut (**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(((*cpm_ptr).cf.element_num
                                                -
                                                1
                                                    as
                                                    libc::c_int)
                                                as
                                                isize)).next);
                    (*cpm_ptr).para_b_ptr[((*cpm_ptr).cf.element_num -
                        1 as libc::c_int) as usize] =
                        (*(*cpm_ptr).elem_b_ptr[((*cpm_ptr).cf.element_num -
                            1 as libc::c_int) as
                            usize]).next;
                    child_num += 1;
                    flag += 1
                }
                if !check_feature((*d_ptr).f,
                                  b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                    (check_feature((*d_ptr).f,
                                   b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() ||
                        check_feature((*d_ptr).f,
                                      b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null()) ||
                    !check_feature((*d_ptr).f,
                                   b"\xe4\xbf\xae\xe9\xa3\xbe\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() {
                    flag += 1
                }
            } else if (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
                para_cky_ptr = (*cky_ptr).left
            }
            if (!check_feature((*d_ptr).f,
                               b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*d_ptr).f,
                               b"\xe5\xbc\xb7\xe8\xaa\xbf\xe6\xa7\x8b\xe6\x96\x87\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                    !check_feature((*g_ptr).f,
                                   b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) &&
                !(*(*(*cky_ptr).left).cpm_ptr).pred_b_ptr.is_null() {
                pre_cpm_ptr = (*(*cky_ptr).left).cpm_ptr;
                (*(*pre_cpm_ptr).pred_b_ptr).cpm_ptr = pre_cpm_ptr;
                make_work_mgr_dpnd_check(sp, cky_ptr, d_ptr);
                recover_para_ptr_to_cpm(pre_cpm_ptr);
                make_data_cframe_rentai_simple(pre_cpm_ptr,
                                               (*pre_cpm_ptr).pred_b_ptr,
                                               t_ptr);
                add_coordinated_phrases((*cky_ptr).right,
                                        &mut (**(*pre_cpm_ptr).elem_b_ptr.as_mut_ptr().offset(((*pre_cpm_ptr).cf.element_num
                                            -
                                            1
                                                as
                                                libc::c_int)
                                            as
                                            isize)).next);
                orig_score = (*pre_cpm_ptr).score;
                one_score -= orig_score;
                one_score +=
                    find_best_cf(sp, pre_cpm_ptr,
                                 get_closest_case_component(sp, pre_cpm_ptr),
                                 0 as libc::c_int, 0 as *mut CF_PRED_MGR);
                (*pre_cpm_ptr).score = orig_score;
                (*pre_cpm_ptr).cf.element_num -= 1;
                flag += 1
            }
            if OptParaFix == 0 as libc::c_int && flag == 0 as libc::c_int &&
                !check_feature((*g_ptr).f,
                               b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char).is_null() &&
                !check_feature((*d_ptr).f,
                               b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                let ref mut fresh0 =
                    (*(*d_ptr).tag_ptr.offset((*d_ptr).tag_num as
                        isize).offset(-(1 as
                        libc::c_int
                        as
                        isize))).next;
                *fresh0 = 0 as *mut tnode_t;
                (*t_ptr).next = 0 as *mut tnode_t;
                add_coordinated_phrases((*cky_ptr).left,
                                        &mut (*(*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                            as
                                            isize).offset(-(1
                                            as
                                            libc::c_int
                                            as
                                            isize))).next);
                add_coordinated_phrases((*cky_ptr).right, &mut (*t_ptr).next);
                make_work_mgr_dpnd_check(sp, cky_ptr, d_ptr);
                one_score +=
                    get_noun_co_ex_probability((*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                        as
                        isize).offset(-(1
                        as
                        libc::c_int
                        as
                        isize)),
                                               t_ptr);
                noun_modifying_num += 1
            }
            if wo_ni_overwritten_flag != 0 {
                assign_cfeature(&mut (*d_ptr).f,
                                b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                assign_cfeature(&mut (*dt_ptr).f,
                                b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                assign_cfeature(&mut (*dt_ptr).f,
                                b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                delete_cfeature(&mut (*dt_ptr).f,
                                b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\xe3\x83\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
            }
        }
        cky_ptr = (*cky_ptr).right
    }
    if pred_p != 0 {
        (*t_ptr).cpm_ptr = cpm_ptr;
        if OptCaseFlag & 524288 as libc::c_int != 0 {
            child_num +=
                add_internal_bunsetsu_child(sp, cpm_ptr, t_ptr, g_ptr,
                                            child_num)
        }
        if !check_feature((*t_ptr).f,
                          b"\xef\xbc\xb4\xe7\x94\xa8\xe8\xa8\x80\xe5\x90\x8c\xe6\x96\x87\xe7\xaf\x80\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if !_make_data_cframe_pp(cpm_ptr, t_ptr,
                                     (0 as libc::c_int == 0) as
                                         libc::c_int).is_null() {
                _make_data_cframe_sm(cpm_ptr, t_ptr);
                _make_data_cframe_ex(cpm_ptr, t_ptr);
                (*cpm_ptr).elem_b_ptr[(*cpm_ptr).cf.element_num as usize] =
                    t_ptr;
                (*cpm_ptr).elem_b_num[(*cpm_ptr).cf.element_num as usize] =
                    child_num;
                (*cpm_ptr).cf.weight[(*cpm_ptr).cf.element_num as usize] =
                    0 as libc::c_int;
                (*cpm_ptr).cf.adjacent[(*cpm_ptr).cf.element_num as usize] =
                    (0 as libc::c_int == 0) as libc::c_int;
                (*cpm_ptr).cf.element_num += 1
            }
        }
        case_analysis_score =
            find_best_cf(sp, cpm_ptr, get_closest_case_component(sp, cpm_ptr),
                         0 as libc::c_int,
                         if !para_cky_ptr.is_null() {
                             (*para_cky_ptr).cpm_ptr
                         } else { 0 as *mut CF_PRED_MGR });
        one_score += case_analysis_score;
        cky_ptr = orig_cky_ptr;
        while !cky_ptr.is_null() {
            if !(*cky_ptr).left.is_null() {
                d_ptr = (*(*cky_ptr).left).b_ptr;
                if (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 &&
                    !check_feature((*g_ptr).f,
                                   b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null() &&
                    check_feature((*d_ptr).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    one_score += case_analysis_score
                }
                if (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 &&
                    !check_feature((*g_ptr).f,
                                   b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() &&
                    check_feature((*g_ptr).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                      as *const u8 as *const libc::c_char
                                      as *mut libc::c_char).is_null() &&
                    check_feature((*d_ptr).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    one_score -= 1000 as libc::c_int as libc::c_double
                }
                if (*cky_ptr).dpnd_type as libc::c_int != 'P' as i32 &&
                    !((*(*cky_ptr).left).i == (*(*cky_ptr).left).j &&
                        !check_feature((*d_ptr).f,
                                       b"ID:\xef\xbc\x88\xe3\x80\x9c\xe3\x82\x92\xef\xbc\x89\xe3\x80\x9c\xe3\x81\xab\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null())
                {
                    if !check_feature((*d_ptr).f,
                                      b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() &&
                        !check_feature((*d_ptr).f,
                                       b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                           *const u8 as *const libc::c_char
                                           as *mut libc::c_char).is_null()
                        &&
                        check_feature((*d_ptr).f,
                                      b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null() {
                        make_work_mgr_dpnd_check(sp, cky_ptr, d_ptr);
                        one_score +=
                            calc_vp_modifying_probability(t_ptr,
                                                          (*cpm_ptr).cmm[0 as
                                                              libc::c_int
                                                              as
                                                              usize].cf_ptr,
                                                          (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                              as
                                                              isize).offset(-(1
                                                              as
                                                              libc::c_int
                                                              as
                                                              isize)),
                                                          (*(*(*cky_ptr).left).cpm_ptr).cmm[0
                                                              as
                                                              libc::c_int
                                                              as
                                                              usize].cf_ptr);
                        renyou_modifying_num += 1
                    } else if !check_feature((*d_ptr).f,
                                             b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                        &&
                        check_feature((*d_ptr).f,
                                      b"\xe7\x94\xa8\xe8\xa8\x80\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char).is_null()
                        ||
                        !check_feature((*d_ptr).f,
                                       b"\xe4\xbf\xae\xe9\xa3\xbe\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null()
                    {
                        make_work_mgr_dpnd_check(sp, cky_ptr, d_ptr);
                        one_score +=
                            calc_adv_modifying_probability(t_ptr,
                                                           (*cpm_ptr).cmm[0 as
                                                               libc::c_int
                                                               as
                                                               usize].cf_ptr,
                                                           (*d_ptr).tag_ptr.offset((*d_ptr).tag_num
                                                               as
                                                               isize).offset(-(1
                                                               as
                                                               libc::c_int
                                                               as
                                                               isize)));
                        adverb_modifying_num += 1
                    }
                }
            }
            cky_ptr = (*cky_ptr).right
        }
        one_score +=
            calc_vp_modifying_num_probability(t_ptr,
                                              (*cpm_ptr).cmm[0 as libc::c_int
                                                  as
                                                  usize].cf_ptr,
                                              renyou_modifying_num);
        one_score +=
            calc_adv_modifying_num_probability(t_ptr,
                                               (*cpm_ptr).cmm[0 as libc::c_int
                                                   as
                                                   usize].cf_ptr,
                                               adverb_modifying_num)
    }
    if OptParaFix == 0 as libc::c_int &&
        (pred_p == 0 ||
            !check_feature((*t_ptr).f,
                           b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null()) {
        one_score +=
            get_noun_co_num_probability(t_ptr, noun_modifying_num,
                                        para_cky_ptr)
    }
    if OptDisplay == 3 as libc::c_int {
        printf(b"%.3f\n\x00" as *const u8 as *const libc::c_char, one_score);
    }
    return one_score;
}

#[no_mangle]
pub unsafe extern "C" fn relax_barrier_for_P(mut cky_ptr: *mut CKY,
                                             mut dep: libc::c_int,
                                             mut gov: libc::c_int,
                                             mut dep_check: *mut libc::c_int)
                                             -> libc::c_int {
    while !cky_ptr.is_null() {
        if !(*cky_ptr).left.is_null() &&
            (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
            if *dep_check.offset(dep as isize) >= (*(*cky_ptr).left).j {
                return (0 as libc::c_int == 0) as libc::c_int;
            } else {
                if (*cky_ptr).para_flag != 0 {
                    if relax_barrier_for_P((*cky_ptr).left, dep, gov,
                                           dep_check) != 0 {
                        return (0 as libc::c_int == 0) as libc::c_int;
                    }
                }
            }
        }
        cky_ptr = (*cky_ptr).right
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn relax_dpnd_for_P(mut cky_ptr: *mut CKY,
                                          mut dep: libc::c_int,
                                          mut gov: libc::c_int)
                                          -> libc::c_int {
    let mut i: libc::c_int = 0;
    while !cky_ptr.is_null() {
        if !(*cky_ptr).left.is_null() &&
            (*cky_ptr).dpnd_type as libc::c_int == 'P' as i32 {
            i = (*(*cky_ptr).left).i;
            while i <= (*(*cky_ptr).left).j {
                if (*Dpnd_matrix.as_mut_ptr().offset(dep as
                    isize))[i as usize]
                    != 0 &&
                    (*Quote_matrix.as_mut_ptr().offset(dep as
                        isize))[i as
                        usize]
                        != 0 {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
                i += 1
            }
        }
        cky_ptr = (*cky_ptr).right
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fix_predicate_coordination(mut sp:
                                                    *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if (*(*sp).para_data.offset(i as isize)).type_0 == 2 as libc::c_int {
            if (*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
                'x' as i32 {
                (*(*sp).para_data.offset(i as isize)).max_score =
                    -(1 as libc::c_int) as libc::c_float
            }
            j = 0 as libc::c_int;
            while j < (*sp).Bnst_num {
                k = 0 as libc::c_int;
                while k < (*sp).Bnst_num {
                    if (*(*sp).para_data.offset(i as isize)).status as
                        libc::c_int == 'x' as i32 ||
                        j !=
                            (*(*sp).para_data.offset(i as
                                isize)).max_path[0
                                as
                                libc::c_int
                                as
                                usize]
                        ||
                        k != (*(*sp).para_data.offset(i as isize)).jend_pos
                    {
                        (*Para_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize][k
                            as
                            usize]
                            = -(1 as libc::c_int) as libc::c_double
                    }
                    k += 1
                }
                j += 1
            }
            if (*(*sp).para_data.offset(i as isize)).status as libc::c_int !=
                'x' as i32 {
                j =
                    (*(*sp).para_data.offset(i as isize)).key_pos +
                        1 as libc::c_int;
                while j < (*sp).Bnst_num {
                    if j == (*(*sp).para_data.offset(i as isize)).jend_pos {
                        (*Dpnd_matrix.as_mut_ptr().offset((*(*sp).para_data.offset(i
                            as
                            isize)).key_pos
                            as
                            isize))[j as
                            usize]
                            = 'R' as i32
                    } else {
                        (*Dpnd_matrix.as_mut_ptr().offset((*(*sp).para_data.offset(i
                            as
                            isize)).key_pos
                            as
                            isize))[j as
                            usize]
                            = 0 as libc::c_int
                    }
                    j += 1
                }
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn restrict_parenthetic_coordination(mut sp:
                                                           *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num - 1 as libc::c_int {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe4\xbf\x82:\xe6\x8b\xac\xe5\xbc\xa7\xe4\xb8\xa6\xe5\x88\x97\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            count = 0 as libc::c_int;
            j = i + 1 as libc::c_int;
            while j < (*sp).Bnst_num {
                if (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    != 0 {
                    if count > 0 as libc::c_int {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]
                            = 0 as libc::c_int
                    }
                    count += 1
                }
                j += 1
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn restrict_end_prefer_dependency(mut sp:
                                                        *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num - 1 as libc::c_int {
        if (*(*(*sp).bnst_data.offset(i as isize)).dpnd_rule).preference ==
            -(1 as libc::c_int) {
            count = 0 as libc::c_int;
            j = (*sp).Bnst_num - 1 as libc::c_int;
            while j > i {
                if (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    != 0 {
                    if count > 0 as libc::c_int {
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize]
                            = 0 as libc::c_int
                    }
                    count += 1
                }
                j -= 1
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn discard_bad_coordination(mut sp:
                                                  *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if (*(*sp).para_data.offset(i as isize)).status as libc::c_int ==
            'x' as i32 {
            j = 0 as libc::c_int;
            while j < (*sp).Bnst_num {
                k = 0 as libc::c_int;
                while k < (*sp).Bnst_num {
                    (*Para_matrix.as_mut_ptr().offset(i as
                        isize))[j as
                        usize][k
                        as
                        usize]
                        = -(1 as libc::c_int) as libc::c_double;
                    k += 1
                }
                j += 1
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn handle_incomplete_coordination(mut sp:
                                                        *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            if (*Mask_matrix.as_mut_ptr().offset(i as isize))[j as usize] ==
                3 as libc::c_int &&
                (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize]
                    == 0 as libc::c_int {
                (*Dpnd_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    'I' as i32
            }
            j += 1
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn extend_para_matrix(mut sp: *mut SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut max_pos: libc::c_int = 0;
    let mut max_score: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < (*sp).Para_num {
        if (*(*sp).para_data.offset(i as isize)).max_score >=
            0 as libc::c_int as libc::c_float {
            if (*(*sp).para_data.offset(i as isize)).type_0 ==
                2 as libc::c_int {
                offset = 0 as libc::c_int
            } else { offset = 1 as libc::c_int }
            l =
                (*(*sp).para_data.offset(i as isize)).key_pos +
                    1 as libc::c_int;
            while l < (*sp).Bnst_num {
                max_score = -(2147483647 as libc::c_int) as libc::c_double;
                j = (*(*sp).para_data.offset(i as isize)).iend_pos;
                while j >= 0 as libc::c_int {
                    if max_score <
                        (*Para_matrix.as_mut_ptr().offset(i as
                            isize))[j as
                            usize][l
                            as
                            usize]
                    {
                        max_score =
                            (*Para_matrix.as_mut_ptr().offset(i as
                                isize))[j as
                                usize][l
                                as
                                usize];
                        max_pos = j
                    }
                    j -= 1
                }
                if max_score >= 0 as libc::c_int as libc::c_double {
                    j = max_pos - 1 as libc::c_int;
                    while j >= 0 as libc::c_int {
                        if para_dpnd::check_stop_extend(sp, j) != 0 { break; }
                        flag = 0 as libc::c_int;
                        k = j + 1 as libc::c_int;
                        while k <=
                            (*(*sp).para_data.offset(i as isize)).key_pos
                                - offset {
                            if (*Dpnd_matrix.as_mut_ptr().offset(j as isize))[k as usize]
                                != 0 &&
                                (*Quote_matrix.as_mut_ptr().offset(j as isize))[k as usize]
                                    != 0 {
                                (*Para_matrix.as_mut_ptr().offset(i as isize))[j as usize][l as usize]
                                    = max_score;
                                flag = 1 as libc::c_int;
                                if OptDisplay == 3 as libc::c_int {
                                    printf(b"Para Extension (%s-%s-%s) -> %s\n\x00" as *const u8 as *const libc::c_char,
                                           (*(*(*sp).bnst_data.offset(max_pos as isize)).head_ptr).Goi.as_mut_ptr(),
                                           (*(*(*sp).bnst_data.offset((*(*sp).para_data.offset(i as isize)).key_pos as isize)).head_ptr).Goi.as_mut_ptr(),
                                           (*(*(*sp).bnst_data.offset(l as isize)).head_ptr).Goi.as_mut_ptr(),
                                           (*(*(*sp).bnst_data.offset(j as isize)).head_ptr).Goi.as_mut_ptr());
                                }
                                break;
                            } else { k += 1 }
                        }
                        if flag == 0 as libc::c_int { break; }
                        j -= 1
                    }
                }
                l += 1
            }
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_cky(mut sp: *mut SENTENCE_DATA,
                                 mut cky_ptr: *mut CKY,
                                 mut left_ptr: *mut CKY,
                                 mut right_ptr: *mut CKY, mut i: libc::c_int,
                                 mut j: libc::c_int, mut k: libc::c_int,
                                 mut dpnd_type: libc::c_char,
                                 mut direction: libc::c_int,
                                 mut index: libc::c_int) {
    let mut l: libc::c_int = 0;
    (*cky_ptr).index = index;
    (*cky_ptr).i = i;
    (*cky_ptr).j = j;
    (*cky_ptr).next = 0 as CKYptr;
    (*cky_ptr).left = left_ptr;
    (*cky_ptr).right = right_ptr;
    (*cky_ptr).direction = direction;
    (*cky_ptr).dpnd_type = dpnd_type;
    (*cky_ptr).cp = ('a' as i32 + j) as libc::c_char;
    if (*cky_ptr).direction == -(1 as libc::c_int) {
        (*cky_ptr).b_ptr = (*(*cky_ptr).left).b_ptr
    } else {
        (*cky_ptr).b_ptr =
            if !(*cky_ptr).right.is_null() {
                (*(*cky_ptr).right).b_ptr
            } else { (*sp).bnst_data.offset(j as isize) }
    }
    (*cky_ptr).un_count = 0 as libc::c_int;
    l = 0 as libc::c_int;
    while l < 11 as libc::c_int {
        (*cky_ptr).scase_check[l as usize] = 0 as libc::c_int;
        l += 1
    }
    (*cky_ptr).para_flag = 0 as libc::c_int;
    (*cky_ptr).para_score = -(1 as libc::c_int) as libc::c_double;
    (*cky_ptr).chicase_score = -(1 as libc::c_int) as libc::c_double;
    (*cky_ptr).chicase_lex_score = -(1 as libc::c_int) as libc::c_double;
    (*cky_ptr).score = 0 as libc::c_int as libc::c_double;
    if Language == 2 as libc::c_int {
        if !left_ptr.is_null() && !right_ptr.is_null() {
            (*cky_ptr).left_pos_index =
                (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                    as
                    isize))[(*(*right_ptr).b_ptr).num
                    as
                    usize].left_pos_index[index
                    as
                    usize];
            (*cky_ptr).right_pos_index =
                (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                    as
                    isize))[(*(*right_ptr).b_ptr).num
                    as
                    usize].right_pos_index[index
                    as
                    usize]
        } else if (*cky_ptr).i == (*cky_ptr).j &&
            (*cky_ptr).i != -(1 as libc::c_int) {
            (*cky_ptr).left_pos_index =
                (*Chi_dpnd_matrix.as_mut_ptr().offset((*cky_ptr).i as
                    isize))[(*cky_ptr).j
                    as
                    usize].left_pos_index[0
                    as
                    libc::c_int
                    as
                    usize];
            (*cky_ptr).right_pos_index =
                (*Chi_dpnd_matrix.as_mut_ptr().offset((*cky_ptr).i as
                    isize))[(*cky_ptr).j
                    as
                    usize].right_pos_index[0
                    as
                    libc::c_int
                    as
                    usize]
        } else {
            (*cky_ptr).left_pos_index = -(1 as libc::c_int);
            (*cky_ptr).right_pos_index = -(1 as libc::c_int)
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn new_cky_data(mut cky_table_num: *mut libc::c_int)
                                      -> *mut CKY {
    let mut cky_ptr: *mut CKY = 0 as *mut CKY;
    cky_ptr =
        &mut *cky_table.as_mut_ptr().offset(*cky_table_num as isize) as
            *mut CKY;
    if OptAnalysis == 1 as libc::c_int &&
        *cky_table_num > cpm_allocated_cky_num {
        (*cky_ptr).cpm_ptr =
            malloc_data(::std::mem::size_of::<CF_PRED_MGR>() as libc::c_ulong,
                        b"new_cky_data\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut CF_PRED_MGR;
        init_case_frame(&mut (*(*cky_ptr).cpm_ptr).cf);
        (*(*cky_ptr).cpm_ptr).cf.type_0 = 0 as libc::c_int;
        cpm_allocated_cky_num = *cky_table_num
    }
    *cky_table_num += 1;
    if OptCaseFlag & 65536 as libc::c_int != 0 &&
        OptAnalysis == 1 as libc::c_int &&
        *cky_table_num >= 2048 as libc::c_int ||
        *cky_table_num >= 1000000 as libc::c_int {
        if OptDisplay == 3 as libc::c_int {
            fprintf(stderr,
                    b";; cky_table_num exceeded maximum (%d)\n\x00" as
                        *const u8 as *const libc::c_char, *cky_table_num);
        }
        return 0 as *mut CKY;
    }
    return cky_ptr;
}

#[no_mangle]
pub unsafe extern "C" fn copy_cky_data(mut dest: *mut CKY,
                                       mut src: *mut CKY) {
    let mut l: libc::c_int = 0;
    if dest == src { return; }
    (*dest).index = (*src).index;
    (*dest).i = (*src).i;
    (*dest).j = (*src).j;
    (*dest).next = (*src).next;
    (*dest).left = (*src).left;
    (*dest).right = (*src).right;
    (*dest).direction = (*src).direction;
    (*dest).dpnd_type = (*src).dpnd_type;
    (*dest).cp = (*src).cp;
    (*dest).direction = (*src).direction;
    (*dest).b_ptr = (*src).b_ptr;
    (*dest).un_count = (*src).un_count;
    l = 0 as libc::c_int;
    while l < 11 as libc::c_int {
        (*dest).scase_check[l as usize] = (*src).scase_check[l as usize];
        l += 1
    }
    (*dest).para_flag = (*src).para_flag;
    (*dest).para_score = (*src).para_score;
    (*dest).score = (*src).score;
    (*dest).cpm_ptr = (*src).cpm_ptr;
}

#[no_mangle]
pub unsafe extern "C" fn after_cky(mut sp: *mut SENTENCE_DATA,
                                   mut Best_mgr: *mut TOTAL_MGR,
                                   mut cky_ptr: *mut CKY,
                                   mut return_flag: libc::c_int,
                                   mut eos_flag: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp_cky_ptr: *mut CKY = 0 as *mut CKY;
    let mut node_stack: [*mut CKY; 200] = [0 as *mut CKY; 200];
    let mut tail_index: libc::c_int = 0;
    if return_flag == 0 as libc::c_int {
        if OptNbest == (0 as libc::c_int == 0) as libc::c_int {
            (*sp).available = 0 as libc::c_int;
            ErrorComment =
                strdup(b"Cannot detect dependency structure\x00" as *const u8
                    as *const libc::c_char);
            when_no_dpnd_struct(sp);
            dpnd_info_to_bnst(sp, &mut (*Best_mgr).dpnd);
            if OptExpress & 2 as libc::c_int == 0 {
                dpnd_info_to_tag(sp, &mut (*Best_mgr).dpnd);
                if OptExpress & 4 as libc::c_int != 0 {
                    dpnd_info_to_mrph(sp);
                }
            }
            if OptPostProcess != 0 { do_postprocess(sp); }
            print_result(sp, 0 as libc::c_int, eos_flag);
        }
        return return_flag;
    }
    (*Best_mgr).pred_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        if (*(*sp).tag_data.offset(i as isize)).cf_num > 0 as libc::c_int &&
            !(*(*sp).tag_data.offset(i as isize)).cpm_ptr.is_null() &&
            !(*(*(*sp).tag_data.offset(i as
                isize)).cpm_ptr).pred_b_ptr.is_null()
            &&
            ((*(*sp).tag_data.offset(i as isize)).inum == 0 as libc::c_int
                &&
                check_feature((*(*(*sp).tag_data.offset(i as
                    isize)).b_ptr).f,
                              b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97:-1\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
                (*(*sp).tag_data.offset(i as isize)).inum ==
                    1 as libc::c_int &&
                    !check_feature((*(*(*sp).tag_data.offset(i as
                        isize)).b_ptr).f,
                                   b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\x8f\x97:-1\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null()) {
            (*(*sp).tag_data.offset(i as isize)).pred_num =
                (*Best_mgr).pred_num;
            (*Best_mgr).pred_num += 1
        }
        i += 1
    }
    if Language == 2 as libc::c_int && OptChiPos != 0 {
        tail_index = 0 as libc::c_int;
        node_stack[tail_index as usize] = cky_ptr;
        while tail_index >= 0 as libc::c_int {
            tmp_cky_ptr = node_stack[tail_index as usize];
            tail_index -= 1;
            if !tmp_cky_ptr.is_null() {
                if (*tmp_cky_ptr).direction == 1 as libc::c_int {
                    strcpy((*(*(*sp).bnst_data.offset((*(*tmp_cky_ptr).b_ptr).num
                        as
                        isize)).head_ptr).Pos.as_mut_ptr(),
                           *Chi_word_pos.as_mut_ptr().offset((*tmp_cky_ptr).right_pos_index
                               as isize));
                } else if (*tmp_cky_ptr).direction == -(1 as libc::c_int) {
                    strcpy((*(*(*sp).bnst_data.offset((*(*tmp_cky_ptr).b_ptr).num
                        as
                        isize)).head_ptr).Pos.as_mut_ptr(),
                           *Chi_word_pos.as_mut_ptr().offset((*tmp_cky_ptr).left_pos_index
                               as isize));
                }
                if !(*tmp_cky_ptr).right.is_null() {
                    tail_index += 1;
                    node_stack[tail_index as usize] = (*tmp_cky_ptr).right
                }
                if !(*tmp_cky_ptr).left.is_null() {
                    tail_index += 1;
                    node_stack[tail_index as usize] = (*tmp_cky_ptr).left
                }
            }
        }
    }
    while !cky_ptr.is_null() {
        i = 0 as libc::c_int;
        while i < (*Best_mgr).pred_num {
            (*Best_mgr).cpm[i as usize].pred_b_ptr = 0 as *mut TAG_DATA;
            i += 1
        }
        if OptDisplay == 3 as libc::c_int {
            printf(b"---------------------\n\x00" as *const u8 as
                *const libc::c_char);
            printf(b"score=%.3f\n\x00" as *const u8 as *const libc::c_char,
                   (*cky_ptr).score);
        }
        (*Best_mgr).dpnd.head[(*(*cky_ptr).b_ptr).num as usize] =
            -(1 as libc::c_int);
        (*Best_mgr).score = (*cky_ptr).score;
        (*sp).score = (*Best_mgr).score;
        convert_to_dpnd(sp, Best_mgr, cky_ptr);
        i = 0 as libc::c_int;
        while i < (*sp).Bnst_num - 1 as libc::c_int {
            if (*Best_mgr).dpnd.head[i as usize] < 0 as libc::c_int {
                if i >=
                    (*Best_mgr).dpnd.head[(i +
                        (*Best_mgr).dpnd.head[i as
                            usize])
                        as usize] {
                    if Language != 2 as libc::c_int {
                        (*Best_mgr).dpnd.head[i as usize] =
                            (*sp).Bnst_num - 1 as libc::c_int
                    }
                } else {
                    (*Best_mgr).dpnd.head[i as usize] =
                        (*Best_mgr).dpnd.head[(i +
                            (*Best_mgr).dpnd.head[i as
                                usize])
                            as usize]
                }
            }
            i += 1
        }
        if OptAnalysis == 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*(*sp).Best_mgr).pred_num {
                if !(*Best_mgr).cpm[i as usize].pred_b_ptr.is_null() {
                    case_analysis::assign_nil_assigned_components(sp, &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as isize));
                    j = 0 as libc::c_int;
                    while j < (*(*(*sp).Best_mgr).cpm[i as usize].cmm[0 as libc::c_int as usize].cf_ptr).element_num {
                        case_analysis::append_cf_feature(
                            &mut (*(*(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as isize)).pred_b_ptr).f,
                            &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i as isize), (*(*sp).Best_mgr).cpm[i as usize].cmm[0 as libc::c_int as usize].cf_ptr,
                            j,
                        );
                        j += 1
                    }
                }
                i += 1
            }
        }
        dpnd_info_to_bnst(sp, &mut (*Best_mgr).dpnd);
        para_recovery(sp);
        if OptExpress & 2 as libc::c_int == 0 {
            dpnd_info_to_tag(sp, &mut (*Best_mgr).dpnd);
            if OptExpress & 4 as libc::c_int != 0 { dpnd_info_to_mrph(sp); }
        }
        if make_dpnd_tree(sp) != 0 {
            tree_conv::bnst_to_tag_tree(sp);
            if OptAnalysis == 1 as libc::c_int {
                i = 0 as libc::c_int;
                while i < (*Best_mgr).pred_num {
                    if !(*Best_mgr).cpm[i as usize].pred_b_ptr.is_null() {
                        if (*Best_mgr).cpm[i as usize].result_num !=
                            0 as libc::c_int &&
                            (*(*Best_mgr).cpm[i as
                                usize].cmm[0 as
                                libc::c_int
                                as
                                usize].cf_ptr).cf_address
                                != -(1 as libc::c_int) as libc::c_ulonglong
                            &&
                            (*Best_mgr).cpm[i as
                                usize].cmm[0 as libc::c_int
                                as
                                usize].score
                                != -(1001 as libc::c_int) as libc::c_double
                        {
                            verb_lexical_disambiguation_by_case_analysis(&mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                as
                                isize));
                            noun_lexical_disambiguation_by_case_analysis(&mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(i
                                as
                                isize));
                        }
                    }
                    i += 1
                }
            }
            if OptNbest == (0 as libc::c_int == 0) as libc::c_int {
                assign_general_feature((*sp).bnst_data as *mut libc::c_void,
                                       (*sp).Bnst_num, 12 as libc::c_int,
                                       0 as libc::c_int,
                                       (0 as libc::c_int == 0) as
                                           libc::c_int);
                assign_general_feature((*sp).tag_data as *mut libc::c_void,
                                       (*sp).Tag_num, 13 as libc::c_int,
                                       0 as libc::c_int,
                                       (0 as libc::c_int == 0) as
                                           libc::c_int);
                if OptPostProcess != 0 { do_postprocess(sp); }
                if OptAnalysis == 1 as libc::c_int {
                    record_all_case_analisys(sp,
                                             (0 as libc::c_int == 0) as
                                                 libc::c_int);
                }
                print_result(sp, 0 as libc::c_int,
                             if eos_flag != 0 && (*cky_ptr).next.is_null() {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int });
                if OptAnalysis == 1 as libc::c_int &&
                    OptDisplay == 3 as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < (*Best_mgr).pred_num {
                        if !(*Best_mgr).cpm[i as usize].pred_b_ptr.is_null() {
                            print_data_cframe(&mut *(*Best_mgr).cpm.as_mut_ptr().offset(i
                                as
                                isize),
                                              &mut *(*(*Best_mgr).cpm.as_mut_ptr().offset(i
                                                  as
                                                  isize)).cmm.as_mut_ptr().offset(0
                                                  as
                                                  libc::c_int
                                                  as
                                                  isize));
                            j = 0 as libc::c_int;
                            while j < (*Best_mgr).cpm[i as usize].result_num {
                                print_crrspnd(&mut *(*Best_mgr).cpm.as_mut_ptr().offset(i
                                    as
                                    isize),
                                              &mut *(*(*Best_mgr).cpm.as_mut_ptr().offset(i
                                                  as
                                                  isize)).cmm.as_mut_ptr().offset(j
                                                  as
                                                  isize));
                                j += 1
                            }
                        }
                        i += 1
                    }
                }
                i = 0 as libc::c_int;
                while i < (*sp).Tag_num {
                    feature::delete_temp_feature(&mut (*(*sp).tag_data.offset(i as
                        isize)).f);
                    i += 1
                }
            } else if OptDisplay == 3 as libc::c_int {
                print_kakari(sp,
                             if OptExpress & 2 as libc::c_int != 0 {
                                 3 as libc::c_int
                             } else { 1 as libc::c_int }, 1 as libc::c_int);
            }
        }
        cky_ptr = (*cky_ptr).next
    }
    return return_flag;
}

#[no_mangle]
pub unsafe extern "C" fn sort_cky_ptrs(mut orig_cky_ptr_ptr: *mut *mut CKY,
                                       mut beam: libc::c_int) {
    let mut cky_ptr: *mut CKY = *orig_cky_ptr_ptr;
    let mut start_cky_ptr_ptr: *mut *mut CKY = orig_cky_ptr_ptr;
    let mut pre_ptr: *mut CKY = 0 as *mut CKY;
    let mut best_ptr: *mut CKY = 0 as *mut CKY;
    let mut best_pre_ptr: *mut CKY = 0 as *mut CKY;
    let mut best_score: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < beam && !cky_ptr.is_null() {
        best_score = -(2147483647 as libc::c_int) as libc::c_double;
        best_pre_ptr = 0 as *mut CKY;
        pre_ptr = 0 as *mut CKY;
        while !cky_ptr.is_null() {
            if (*cky_ptr).score > best_score {
                best_score = (*cky_ptr).score;
                best_ptr = cky_ptr;
                best_pre_ptr = pre_ptr
            }
            pre_ptr = cky_ptr;
            cky_ptr = (*cky_ptr).next
        }
        if !best_pre_ptr.is_null() {
            (*best_pre_ptr).next = (*best_ptr).next;
            (*best_ptr).next = *start_cky_ptr_ptr;
            *start_cky_ptr_ptr = best_ptr
        }
        start_cky_ptr_ptr = &mut (*best_ptr).next;
        cky_ptr = (*best_ptr).next;
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn cky(mut sp: *mut SENTENCE_DATA,
                             mut Best_mgr: *mut TOTAL_MGR,
                             mut eos_flag: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut sort_flag: libc::c_int = 0;
    // let mut sen_len: libc::c_int = 0;
    let mut cky_table_num: libc::c_int = 0;
    let mut pre_cky_table_num: libc::c_int = 0;
    let mut dep_check: [libc::c_int; 200] = [0; 200];
    let mut best_score: libc::c_double = 0.;
    let mut para_score: libc::c_double = 0.;
    let mut dpnd_type: libc::c_char = 0;
    let mut cky_ptr: *mut CKY = 0 as *mut CKY;
    let mut left_ptr: *mut CKY = 0 as *mut CKY;
    let mut right_ptr: *mut CKY = 0 as *mut CKY;
    let mut best_ptr: *mut CKY = 0 as *mut CKY;
    let mut pre_ptr: *mut CKY = 0 as *mut CKY;
    let mut best_pre_ptr: *mut CKY = 0 as *mut CKY;
    let mut start_ptr: *mut CKY = 0 as *mut CKY;
    let mut sort_pre_ptr: *mut CKY = 0 as *mut CKY;
    let mut tmp_ptr: *mut CKY = 0 as *mut CKY;
    let mut next_pp: *mut *mut CKY = 0 as *mut *mut CKY;
    let mut next_pp_for_ij: *mut *mut CKY = 0 as *mut *mut CKY;
    cky_table_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        dep_check[i as usize] = -(1 as libc::c_int);
        (*Best_mgr).dpnd.head[i as usize] = -(1 as libc::c_int);
        (*Best_mgr).dpnd.type_0[i as usize] = 'D' as i32 as libc::c_char;
        i += 1
    }
    restrict_parenthetic_coordination(sp);
    restrict_end_prefer_dependency(sp);
    if OptParaFix == 0 as libc::c_int {
        discard_bad_coordination(sp);
        handle_incomplete_coordination(sp);
    }
    j = 0 as libc::c_int;
    while j < (*sp).Bnst_num {
        i = j;
        while i >= 0 as libc::c_int {
            if OptDisplay == 3 as libc::c_int {
                printf(b"(%d,%d)\n\x00" as *const u8 as *const libc::c_char,
                       i, j);
            }
            cky_matrix[i as usize][j as usize] = 0 as *mut CKY;
            if i == j {
                cky_ptr = new_cky_data(&mut cky_table_num);
                if cky_ptr.is_null() { return 0 as libc::c_int; }
                cky_matrix[i as usize][j as usize] = cky_ptr;
                set_cky(
                    sp,
                    cky_ptr,
                    0 as *mut CKY,
                    0 as *mut CKY,
                    i,
                    j,
                    -(1 as libc::c_int),
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int,
                    -(1 as libc::c_int),
                );
                (*cky_ptr).score =
                    if OptAnalysis == 1 as libc::c_int {
                        calc_case_probability(sp, cky_ptr, Best_mgr)
                    } else { calc_score(sp, cky_ptr) }
            } else {
                next_pp_for_ij = 0 as *mut *mut CKY;
                pre_cky_table_num = cky_table_num;
                k = 0 as libc::c_int;
                while k < j - i {
                    para_score =
                        if (*(*sp).bnst_data.offset(i as
                            isize).offset(k as
                            isize)).para_num
                            == -(1 as libc::c_int) {
                            -(1 as libc::c_int) as libc::c_double
                        } else {
                            (*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                as
                                isize).offset(k
                                as
                                isize)).para_num
                                as
                                isize))[i as
                                usize][j
                                as
                                usize]
                        };
                    next_pp = 0 as *mut *mut CKY;
                    left_ptr = cky_matrix[i as usize][(i + k) as usize];
                    while !left_ptr.is_null() {
                        right_ptr =
                            cky_matrix[(i + k + 1 as libc::c_int) as
                                usize][j as usize];
                        while !right_ptr.is_null() {
                            dpnd_type =
                                check_dpnd_possibility(sp,
                                                       (*(*left_ptr).b_ptr).num,
                                                       (*(*right_ptr).b_ptr).num,
                                                       i,
                                                       (if j ==
                                                           (*sp).Bnst_num
                                                               -
                                                               1 as
                                                                   libc::c_int
                                                           &&
                                                           dep_check[(i +
                                                               k)
                                                               as
                                                               usize]
                                                               ==
                                                               -(1 as
                                                                   libc::c_int)
                                                       {
                                                           (0 as libc::c_int
                                                               == 0) as
                                                               libc::c_int
                                                       } else {
                                                           0 as libc::c_int
                                                       }));
                            if dpnd_type as libc::c_int != 0 &&
                                (dpnd_type as libc::c_int == 'P' as i32 &&
                                    (OptParaFix ==
                                        (0 as libc::c_int == 0) as
                                            libc::c_int ||
                                        (*right_ptr).dpnd_type as
                                            libc::c_int != 'P' as i32) ||
                                    dpnd_type as libc::c_int != 'P' as i32
                                        &&
                                        (dep_check[(i + k) as usize] <=
                                            0 as libc::c_int ||
                                            dep_check[(i + k) as usize]
                                                >= j ||
                                            OptParaFix ==
                                                0 as libc::c_int &&
                                                relax_barrier_for_P(right_ptr,
                                                                    i +
                                                                        k,
                                                                    j,
                                                                    dep_check.as_mut_ptr())
                                                    != 0)) {
                                if Language == 2 as libc::c_int {
                                    let mut current_block_152: u64;
                                    l = 0 as libc::c_int;
                                    while l <
                                        (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                            as
                                            isize))[(*(*right_ptr).b_ptr).num
                                            as
                                            usize].count
                                    {
                                        if (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                            as
                                            isize))[(*(*right_ptr).b_ptr).num
                                            as
                                            usize].direction[l
                                            as
                                            usize]
                                            as libc::c_int == 'B' as i32 {
                                            if check_chi_dpnd_possibility(i,
                                                                          j,
                                                                          k,
                                                                          left_ptr,
                                                                          right_ptr,
                                                                          sp,
                                                                          'R'
                                                                              as
                                                                              i32,
                                                                          l)
                                                != 0 {
                                                if i == 0 as libc::c_int &&
                                                    j ==
                                                        (*sp).Bnst_num -
                                                            1 as
                                                                libc::c_int
                                                    &&
                                                    (*Chi_root_prob_matrix.as_mut_ptr().offset((*(*right_ptr).b_ptr).num
                                                        as
                                                        isize)).prob[0
                                                        as
                                                        libc::c_int
                                                        as
                                                        usize]
                                                        <=
                                                        0.0000000000000001f64
                                                {
                                                    current_block_152 =
                                                        6450597802325118133;
                                                } else {
                                                    cky_ptr =
                                                        new_cky_data(&mut cky_table_num);
                                                    if cky_ptr.is_null() {
                                                        return 0 as
                                                            libc::c_int;
                                                    }
                                                    if next_pp.is_null() {
                                                        start_ptr = cky_ptr
                                                    } else {
                                                        *next_pp = cky_ptr
                                                    }
                                                    set_cky(sp, cky_ptr,
                                                            left_ptr,
                                                            right_ptr, i, j,
                                                            k,
                                                            'R' as i32 as
                                                                libc::c_char,
                                                            1 as libc::c_int,
                                                            l);
                                                    next_pp =
                                                        &mut (*cky_ptr).next;
                                                    if OptDisplay ==
                                                        3 as libc::c_int {
                                                        printf(b"   (%d,%d), (%d,%d) b=%d [%s%s%s], %c(para=%.3f), score=\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               i, i + k,
                                                               i + k +
                                                                   1 as
                                                                       libc::c_int,
                                                               j,
                                                               dep_check[(i +
                                                                   k)
                                                                   as
                                                                   usize],
                                                               (*(*(*left_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                               b"->\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (*(*(*right_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                               'R' as i32,
                                                               para_score);
                                                    }
                                                    (*cky_ptr).para_score =
                                                        para_score;
                                                    (*cky_ptr).score =
                                                        if OptAnalysis ==
                                                            1 as
                                                                libc::c_int
                                                        {
                                                            calc_case_probability(sp,
                                                                                  cky_ptr,
                                                                                  Best_mgr)
                                                        } else {
                                                            calc_score(sp,
                                                                       cky_ptr)
                                                        };
                                                    if OptParaFix != 0 {
                                                        if (*Mask_matrix.as_mut_ptr().offset(i
                                                            as
                                                            isize))[(i
                                                            +
                                                            k)
                                                            as
                                                            usize]
                                                            == 'N' as i32
                                                            &&
                                                            (*Mask_matrix.as_mut_ptr().offset((i
                                                                +
                                                                k
                                                                +
                                                                1
                                                                    as
                                                                    libc::c_int)
                                                                as
                                                                isize))[j
                                                                as
                                                                usize]
                                                                ==
                                                                'N' as i32
                                                        {
                                                            (*cky_ptr).score
                                                                +=
                                                                50 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_double;
                                                            if OptDisplay ==
                                                                3 as
                                                                    libc::c_int
                                                            {
                                                                printf(b"=>%.3f\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       (*cky_ptr).score);
                                                            }
                                                        }
                                                    }
                                                    if OptParaFix == 0 &&
                                                        OptChiPos == 0 {
                                                        if (*cky_ptr).para_score
                                                            >
                                                            0 as
                                                                libc::c_int
                                                                as
                                                                libc::c_double
                                                            &&
                                                            ((*Mask_matrix.as_mut_ptr().offset(i
                                                                as
                                                                isize))[(i
                                                                +
                                                                k)
                                                                as
                                                                usize]
                                                                ==
                                                                'N' as i32
                                                                ||
                                                                (*Mask_matrix.as_mut_ptr().offset((i
                                                                    +
                                                                    k
                                                                    +
                                                                    1
                                                                        as
                                                                        libc::c_int)
                                                                    as
                                                                    isize))[j
                                                                    as
                                                                    usize]
                                                                    ==
                                                                    'N' as
                                                                        i32)
                                                        {
                                                            (*cky_ptr).score
                                                                +=
                                                                log((*cky_ptr).para_score
                                                                    +
                                                                    1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_double)
                                                        } else if (*(*sp).bnst_data.offset(i
                                                            as
                                                            isize).offset(k
                                                            as
                                                            isize)).para_num
                                                            !=
                                                            -(1 as
                                                                libc::c_int)
                                                            &&
                                                            !(*cky_ptr).right.is_null()
                                                            &&
                                                            (*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                                as
                                                                isize).offset(k
                                                                as
                                                                isize)).para_num
                                                                as
                                                                isize))[i
                                                                as
                                                                usize][(*(*(*cky_ptr).right).b_ptr).num
                                                                as
                                                                usize]
                                                                >
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_double
                                                            &&
                                                            exist_chi(sp,
                                                                      (*(*(*cky_ptr).right).b_ptr).num
                                                                          +
                                                                          1
                                                                              as
                                                                              libc::c_int,
                                                                      j,
                                                                      b"pu\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char)
                                                                ==
                                                                -(1
                                                                    as
                                                                    libc::c_int)
                                                            &&
                                                            ((*Mask_matrix.as_mut_ptr().offset(i
                                                                as
                                                                isize))[(i
                                                                +
                                                                k)
                                                                as
                                                                usize]
                                                                ==
                                                                'V'
                                                                    as
                                                                    i32
                                                                ||
                                                                (*Mask_matrix.as_mut_ptr().offset((i
                                                                    +
                                                                    k
                                                                    +
                                                                    1
                                                                        as
                                                                        libc::c_int)
                                                                    as
                                                                    isize))[(*(*(*cky_ptr).right).b_ptr).num
                                                                    as
                                                                    usize]
                                                                    ==
                                                                    'V'
                                                                        as
                                                                        i32)
                                                        {
                                                            (*cky_ptr).score
                                                                +=
                                                                log((*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                                    as
                                                                    isize).offset(k
                                                                    as
                                                                    isize)).para_num
                                                                    as
                                                                    isize))[i
                                                                    as
                                                                    usize][(*(*(*cky_ptr).right).b_ptr).num
                                                                    as
                                                                    usize]
                                                                    +
                                                                    1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_double)
                                                        }
                                                        if OptDisplay ==
                                                            3 as
                                                                libc::c_int
                                                        {
                                                            printf(b"(para)=>%.3f\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (*cky_ptr).score);
                                                        }
                                                    }
                                                    current_block_152 =
                                                        10435735846551762309;
                                                }
                                            } else {
                                                current_block_152 =
                                                    10435735846551762309;
                                            }
                                            match current_block_152 {
                                                6450597802325118133 => {}
                                                _ => {
                                                    if check_chi_dpnd_possibility(i,
                                                                                  j,
                                                                                  k,
                                                                                  left_ptr,
                                                                                  right_ptr,
                                                                                  sp,
                                                                                  'L'
                                                                                      as
                                                                                      i32,
                                                                                  l)
                                                        != 0 {
                                                        if !(i ==
                                                            0 as
                                                                libc::c_int
                                                            &&
                                                            j ==
                                                                (*sp).Bnst_num
                                                                    -
                                                                    1 as
                                                                        libc::c_int
                                                            &&
                                                            (*Chi_root_prob_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                                as
                                                                isize)).prob[0
                                                                as
                                                                libc::c_int
                                                                as
                                                                usize]
                                                                <=
                                                                0.0000000000000001f64)
                                                        {
                                                            cky_ptr =
                                                                new_cky_data(&mut cky_table_num);
                                                            if cky_ptr.is_null()
                                                            {
                                                                return 0 as
                                                                    libc::c_int;
                                                            }
                                                            if next_pp.is_null()
                                                            {
                                                                start_ptr =
                                                                    cky_ptr
                                                            } else {
                                                                *next_pp =
                                                                    cky_ptr
                                                            }
                                                            set_cky(sp,
                                                                    cky_ptr,
                                                                    left_ptr,
                                                                    right_ptr,
                                                                    i, j, k,
                                                                    'L' as i32
                                                                        as
                                                                        libc::c_char,
                                                                    -(1 as
                                                                        libc::c_int),
                                                                    l);
                                                            next_pp =
                                                                &mut (*cky_ptr).next;
                                                            if OptDisplay ==
                                                                3 as
                                                                    libc::c_int
                                                            {
                                                                printf(b"   (%d,%d), (%d,%d) b=%d [%s%s%s], %c(para=%.3f), score=\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       i,
                                                                       i + k,
                                                                       i + k +
                                                                           1
                                                                               as
                                                                               libc::c_int,
                                                                       j,
                                                                       dep_check[(i
                                                                           +
                                                                           k)
                                                                           as
                                                                           usize],
                                                                       (*(*(*left_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                                       b"<-\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       (*(*(*right_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                                       'L' as
                                                                           i32,
                                                                       para_score);
                                                            }
                                                            (*cky_ptr).para_score
                                                                = para_score;
                                                            (*cky_ptr).score =
                                                                if OptAnalysis
                                                                    ==
                                                                    1 as
                                                                        libc::c_int
                                                                {
                                                                    calc_case_probability(sp,
                                                                                          cky_ptr,
                                                                                          Best_mgr)
                                                                } else {
                                                                    calc_score(sp,
                                                                               cky_ptr)
                                                                };
                                                            if OptParaFix != 0
                                                            {
                                                                if (*Mask_matrix.as_mut_ptr().offset(i
                                                                    as
                                                                    isize))[(i
                                                                    +
                                                                    k)
                                                                    as
                                                                    usize]
                                                                    ==
                                                                    'V' as
                                                                        i32
                                                                    &&
                                                                    (*Mask_matrix.as_mut_ptr().offset((i
                                                                        +
                                                                        k
                                                                        +
                                                                        1
                                                                            as
                                                                            libc::c_int)
                                                                        as
                                                                        isize))[j
                                                                        as
                                                                        usize]
                                                                        ==
                                                                        'V'
                                                                            as
                                                                            i32
                                                                {
                                                                    (*cky_ptr).score
                                                                        +=
                                                                        50 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_double;
                                                                    if OptDisplay
                                                                        ==
                                                                        3
                                                                            as
                                                                            libc::c_int
                                                                    {
                                                                        printf(b"=>%.3f\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               (*cky_ptr).score);
                                                                    }
                                                                }
                                                            }
                                                            if OptParaFix == 0
                                                                &&
                                                                OptChiPos
                                                                    == 0 {
                                                                if (*cky_ptr).para_score
                                                                    >
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_double
                                                                    &&
                                                                    ((*Mask_matrix.as_mut_ptr().offset(i
                                                                        as
                                                                        isize))[(i
                                                                        +
                                                                        k)
                                                                        as
                                                                        usize]
                                                                        ==
                                                                        'N'
                                                                            as
                                                                            i32
                                                                        &&
                                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                                            +
                                                                            k
                                                                            +
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                            as
                                                                            isize))[j
                                                                            as
                                                                            usize]
                                                                            ==
                                                                            'N'
                                                                                as
                                                                                i32)
                                                                {
                                                                    (*cky_ptr).score
                                                                        +=
                                                                        log((*cky_ptr).para_score
                                                                            +
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_double)
                                                                } else if (*(*sp).bnst_data.offset(i
                                                                    as
                                                                    isize).offset(k
                                                                    as
                                                                    isize)).para_num
                                                                    !=
                                                                    -(1
                                                                        as
                                                                        libc::c_int)
                                                                    &&
                                                                    !(*cky_ptr).right.is_null()
                                                                    &&
                                                                    (*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                                        as
                                                                        isize).offset(k
                                                                        as
                                                                        isize)).para_num
                                                                        as
                                                                        isize))[i
                                                                        as
                                                                        usize][(*(*(*cky_ptr).right).b_ptr).num
                                                                        as
                                                                        usize]
                                                                        >
                                                                        0
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_double
                                                                    &&
                                                                    exist_chi(sp,
                                                                              (*(*(*cky_ptr).right).b_ptr).num
                                                                                  +
                                                                                  1
                                                                                      as
                                                                                      libc::c_int,
                                                                              j,
                                                                              b"pu\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char)
                                                                        ==
                                                                        -(1
                                                                            as
                                                                            libc::c_int)
                                                                    &&
                                                                    ((*Mask_matrix.as_mut_ptr().offset(i
                                                                        as
                                                                        isize))[(i
                                                                        +
                                                                        k)
                                                                        as
                                                                        usize]
                                                                        ==
                                                                        'V'
                                                                            as
                                                                            i32
                                                                        &&
                                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                                            +
                                                                            k
                                                                            +
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                            as
                                                                            isize))[(*(*(*cky_ptr).right).b_ptr).num
                                                                            as
                                                                            usize]
                                                                            ==
                                                                            'V'
                                                                                as
                                                                                i32)
                                                                {
                                                                    (*cky_ptr).score
                                                                        +=
                                                                        log((*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                                            as
                                                                            isize).offset(k
                                                                            as
                                                                            isize)).para_num
                                                                            as
                                                                            isize))[i
                                                                            as
                                                                            usize][(*(*(*cky_ptr).right).b_ptr).num
                                                                            as
                                                                            usize]
                                                                            +
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_double)
                                                                }
                                                                if OptDisplay
                                                                    ==
                                                                    3 as
                                                                        libc::c_int
                                                                {
                                                                    printf(b"(para)=>%.3f\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           (*cky_ptr).score);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else if !(l != 0) {
                                            if !(i == 0 as libc::c_int &&
                                                j ==
                                                    (*sp).Bnst_num -
                                                        1 as libc::c_int
                                                &&
                                                (if (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                    as
                                                    isize))[(*(*right_ptr).b_ptr).num
                                                    as
                                                    usize].direction[l
                                                    as
                                                    usize]
                                                    as libc::c_int ==
                                                    'R' as i32 {
                                                    (*Chi_root_prob_matrix.as_mut_ptr().offset((*(*right_ptr).b_ptr).num
                                                        as
                                                        isize)).prob[0
                                                        as
                                                        libc::c_int
                                                        as
                                                        usize]
                                                } else {
                                                    (*Chi_root_prob_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                        as
                                                        isize)).prob[0
                                                        as
                                                        libc::c_int
                                                        as
                                                        usize]
                                                }) <=
                                                    0.0000000000000001f64)
                                            {
                                                cky_ptr =
                                                    new_cky_data(&mut cky_table_num);
                                                if cky_ptr.is_null() {
                                                    return 0 as libc::c_int;
                                                }
                                                if next_pp.is_null() {
                                                    start_ptr = cky_ptr
                                                } else { *next_pp = cky_ptr }
                                                if OptParaFix != 0 {
                                                    if (*Mask_matrix.as_mut_ptr().offset(i
                                                        as
                                                        isize))[(i
                                                        +
                                                        k)
                                                        as
                                                        usize]
                                                        == 'N' as i32 &&
                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                            +
                                                            k
                                                            +
                                                            1
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize))[j
                                                            as
                                                            usize]
                                                            == 'N' as i32 {
                                                        set_cky(sp, cky_ptr,
                                                                left_ptr,
                                                                right_ptr, i,
                                                                j, k,
                                                                'R' as i32 as
                                                                    libc::c_char,
                                                                1 as
                                                                    libc::c_int,
                                                                l);
                                                    } else if (*Mask_matrix.as_mut_ptr().offset(i
                                                        as
                                                        isize))[(i
                                                        +
                                                        k)
                                                        as
                                                        usize]
                                                        ==
                                                        'V' as i32
                                                        &&
                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                            +
                                                            k
                                                            +
                                                            1
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize))[j
                                                            as
                                                            usize]
                                                            ==
                                                            'V' as
                                                                i32
                                                    {
                                                        set_cky(sp, cky_ptr,
                                                                left_ptr,
                                                                right_ptr, i,
                                                                j, k,
                                                                'L' as i32 as
                                                                    libc::c_char,
                                                                -(1 as
                                                                    libc::c_int),
                                                                l);
                                                    } else {
                                                        set_cky(sp, cky_ptr,
                                                                left_ptr,
                                                                right_ptr, i,
                                                                j, k,
                                                                (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                                    as
                                                                    isize))[(*(*right_ptr).b_ptr).num
                                                                    as
                                                                    usize].direction[l
                                                                    as
                                                                    usize],
                                                                if (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                                    as
                                                                    isize))[(*(*right_ptr).b_ptr).num
                                                                    as
                                                                    usize].direction[l
                                                                    as
                                                                    usize]
                                                                    as
                                                                    libc::c_int
                                                                    ==
                                                                    'L' as
                                                                        i32
                                                                {
                                                                    -(1 as
                                                                        libc::c_int)
                                                                } else {
                                                                    1 as
                                                                        libc::c_int
                                                                }, l);
                                                    }
                                                } else {
                                                    set_cky(sp, cky_ptr,
                                                            left_ptr,
                                                            right_ptr, i, j,
                                                            k,
                                                            (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                                as
                                                                isize))[(*(*right_ptr).b_ptr).num
                                                                as
                                                                usize].direction[l
                                                                as
                                                                usize],
                                                            if (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                                as
                                                                isize))[(*(*right_ptr).b_ptr).num
                                                                as
                                                                usize].direction[l
                                                                as
                                                                usize]
                                                                as
                                                                libc::c_int
                                                                ==
                                                                'L' as i32
                                                            {
                                                                -(1 as
                                                                    libc::c_int)
                                                            } else {
                                                                1 as
                                                                    libc::c_int
                                                            }, l);
                                                }
                                                next_pp =
                                                    &mut (*cky_ptr).next;
                                                if OptDisplay ==
                                                    3 as libc::c_int {
                                                    printf(b"   (%d,%d), (%d,%d) b=%d [%s%s%s], %c(para=%.3f), score=\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           i, i + k,
                                                           i + k +
                                                               1 as
                                                                   libc::c_int,
                                                           j,
                                                           dep_check[(i + k)
                                                               as
                                                               usize],
                                                           (*(*(*left_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                           if (*cky_ptr).direction
                                                               ==
                                                               -(1 as
                                                                   libc::c_int)
                                                           {
                                                               b"<-\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } else {
                                                               b"->\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           },
                                                           (*(*(*right_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                                           (*Chi_dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                               as
                                                               isize))[(*(*right_ptr).b_ptr).num
                                                               as
                                                               usize].direction[l
                                                               as
                                                               usize]
                                                               as libc::c_int,
                                                           para_score);
                                                }
                                                (*cky_ptr).para_score =
                                                    para_score;
                                                (*cky_ptr).score =
                                                    if OptAnalysis ==
                                                        1 as libc::c_int {
                                                        calc_case_probability(sp,
                                                                              cky_ptr,
                                                                              Best_mgr)
                                                    } else {
                                                        calc_score(sp,
                                                                   cky_ptr)
                                                    };
                                                if OptParaFix != 0 {
                                                    if (*Mask_matrix.as_mut_ptr().offset(i
                                                        as
                                                        isize))[(i
                                                        +
                                                        k)
                                                        as
                                                        usize]
                                                        == 'N' as i32 &&
                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                            +
                                                            k
                                                            +
                                                            1
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize))[j
                                                            as
                                                            usize]
                                                            == 'N' as i32 {
                                                        (*cky_ptr).score +=
                                                            50 as libc::c_int
                                                                as
                                                                libc::c_double;
                                                        if OptDisplay ==
                                                            3 as
                                                                libc::c_int
                                                        {
                                                            printf(b"=>%.3f\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (*cky_ptr).score);
                                                        }
                                                    } else if (*Mask_matrix.as_mut_ptr().offset(i
                                                        as
                                                        isize))[(i
                                                        +
                                                        k)
                                                        as
                                                        usize]
                                                        ==
                                                        'V' as i32
                                                        &&
                                                        (*Mask_matrix.as_mut_ptr().offset((i
                                                            +
                                                            k
                                                            +
                                                            1
                                                                as
                                                                libc::c_int)
                                                            as
                                                            isize))[j
                                                            as
                                                            usize]
                                                            ==
                                                            'V' as
                                                                i32
                                                    {
                                                        (*cky_ptr).score +=
                                                            50 as libc::c_int
                                                                as
                                                                libc::c_double;
                                                        if OptDisplay ==
                                                            3 as
                                                                libc::c_int
                                                        {
                                                            printf(b"=>%.3f\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (*cky_ptr).score);
                                                        }
                                                    }
                                                }
                                                if OptParaFix == 0 &&
                                                    OptChiPos == 0 {
                                                    if (*cky_ptr).para_score >
                                                        0 as libc::c_int as
                                                            libc::c_double
                                                        &&
                                                        ((*Mask_matrix.as_mut_ptr().offset(i
                                                            as
                                                            isize))[(i
                                                            +
                                                            k)
                                                            as
                                                            usize]
                                                            == 'N' as i32
                                                            ||
                                                            (*Mask_matrix.as_mut_ptr().offset((i
                                                                +
                                                                k
                                                                +
                                                                1
                                                                    as
                                                                    libc::c_int)
                                                                as
                                                                isize))[j
                                                                as
                                                                usize]
                                                                ==
                                                                'N' as
                                                                    i32) {
                                                        (*cky_ptr).score +=
                                                            log((*cky_ptr).para_score
                                                                +
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_double)
                                                    } else if (*(*sp).bnst_data.offset(i
                                                        as
                                                        isize).offset(k
                                                        as
                                                        isize)).para_num
                                                        !=
                                                        -(1 as
                                                            libc::c_int)
                                                        &&
                                                        !(*cky_ptr).right.is_null()
                                                        &&
                                                        (*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                            as
                                                            isize).offset(k
                                                            as
                                                            isize)).para_num
                                                            as
                                                            isize))[i
                                                            as
                                                            usize][(*(*(*cky_ptr).right).b_ptr).num
                                                            as
                                                            usize]
                                                            >
                                                            0 as
                                                                libc::c_int
                                                                as
                                                                libc::c_double
                                                        &&
                                                        exist_chi(sp,
                                                                  (*(*(*cky_ptr).right).b_ptr).num
                                                                      +
                                                                      1
                                                                          as
                                                                          libc::c_int,
                                                                  j,
                                                                  b"pu\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                                            ==
                                                            -(1 as
                                                                libc::c_int)
                                                        &&
                                                        ((*Mask_matrix.as_mut_ptr().offset(i
                                                            as
                                                            isize))[(i
                                                            +
                                                            k)
                                                            as
                                                            usize]
                                                            ==
                                                            'V' as
                                                                i32
                                                            ||
                                                            (*Mask_matrix.as_mut_ptr().offset((i
                                                                +
                                                                k
                                                                +
                                                                1
                                                                    as
                                                                    libc::c_int)
                                                                as
                                                                isize))[(*(*(*cky_ptr).right).b_ptr).num
                                                                as
                                                                usize]
                                                                ==
                                                                'V'
                                                                    as
                                                                    i32)
                                                    {
                                                        (*cky_ptr).score +=
                                                            log((*Para_matrix.as_mut_ptr().offset((*(*sp).bnst_data.offset(i
                                                                as
                                                                isize).offset(k
                                                                as
                                                                isize)).para_num
                                                                as
                                                                isize))[i
                                                                as
                                                                usize][(*(*(*cky_ptr).right).b_ptr).num
                                                                as
                                                                usize]
                                                                +
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_double)
                                                    }
                                                    if OptDisplay ==
                                                        3 as libc::c_int {
                                                        printf(b"(para)=>%.3f\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (*cky_ptr).score);
                                                    }
                                                }
                                            }
                                        }
                                        l += 1
                                    }
                                } else {
                                    cky_ptr =
                                        new_cky_data(&mut cky_table_num);
                                    if cky_ptr.is_null() {
                                        return 0 as libc::c_int;
                                    }
                                    if next_pp.is_null() {
                                        start_ptr = cky_ptr
                                    } else { *next_pp = cky_ptr }
                                    set_cky(sp, cky_ptr, left_ptr, right_ptr,
                                            i, j, k, dpnd_type,
                                            if (*Dpnd_matrix.as_mut_ptr().offset((*(*left_ptr).b_ptr).num
                                                as
                                                isize))[(*(*right_ptr).b_ptr).num
                                                as
                                                usize]
                                                == 'L' as i32 {
                                                -(1 as libc::c_int)
                                            } else { 1 as libc::c_int },
                                            -(1 as libc::c_int));
                                    next_pp = &mut (*cky_ptr).next;
                                    if OptDisplay == 3 as libc::c_int {
                                        printf(b"   (%d,%d), (%d,%d) b=%d [%s%s%s], %c(para=%.3f), score=\x00"
                                                   as *const u8 as
                                                   *const libc::c_char, i,
                                               i + k,
                                               i + k + 1 as libc::c_int, j,
                                               dep_check[(i + k) as usize],
                                               (*(*(*left_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                               if (*cky_ptr).direction ==
                                                   -(1 as libc::c_int) {
                                                   b"<-\x00" as *const u8 as
                                                       *const libc::c_char
                                               } else {
                                                   b"->\x00" as *const u8 as
                                                       *const libc::c_char
                                               },
                                               (*(*(*right_ptr).b_ptr).head_ptr).Goi.as_mut_ptr(),
                                               dpnd_type as libc::c_int,
                                               para_score);
                                    }
                                    (*cky_ptr).para_score = para_score;
                                    (*cky_ptr).score =
                                        if OptAnalysis == 1 as libc::c_int {
                                            calc_case_probability(sp, cky_ptr,
                                                                  Best_mgr)
                                        } else { calc_score(sp, cky_ptr) }
                                }
                            }
                            if Language != 2 as libc::c_int &&
                                OptNbest == 0 as libc::c_int &&
                                OptParaFix != 0 &&
                                check_feature((*(*right_ptr).b_ptr).f,
                                              b"\xe7\x94\xa8\xe8\xa8\x80\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                            {
                                break;
                            }
                            right_ptr = (*right_ptr).next
                        }
                        if Language != 2 as libc::c_int &&
                            OptNbest == 0 as libc::c_int && OptParaFix != 0
                            &&
                            (check_feature((*(*left_ptr).b_ptr).f,
                                           b"\xe7\x94\xa8\xe8\xa8\x80\x00"
                                               as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char).is_null()
                                ||
                                !check_feature((*(*left_ptr).b_ptr).f,
                                               b"\xe4\xbf\x82:\xe9\x80\xa3\xe7\x94\xa8\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null())
                        {
                            break;
                        }
                        left_ptr = (*left_ptr).next
                    }
                    if !next_pp.is_null() {
                        if next_pp_for_ij.is_null() {
                            cky_matrix[i as usize][j as usize] = start_ptr
                        } else { *next_pp_for_ij = start_ptr }
                        next_pp_for_ij = next_pp;
                        if j != (*sp).Bnst_num - 1 as libc::c_int {
                            if (OptParaFix != 0 ||
                                (*Dpnd_matrix.as_mut_ptr().offset((i + k)
                                    as
                                    isize))[j
                                    as
                                    usize]
                                    != 0) &&
                                !(*(*(*sp).bnst_data.offset(i as
                                    isize).offset(k
                                    as
                                    isize)).dpnd_rule).barrier.fp[0
                                    as
                                    libc::c_int
                                    as
                                    usize].is_null()
                                &&
                                feature_pattern_match(&mut (*(*(*sp).bnst_data.offset(i
                                    as
                                    isize).offset(k
                                    as
                                    isize)).dpnd_rule).barrier,
                                                      (*(*sp).bnst_data.offset(j
                                                          as
                                                          isize)).f,
                                                      (*sp).bnst_data.offset(i
                                                          as
                                                          isize).offset(k
                                                          as
                                                          isize)
                                                          as
                                                          *mut libc::c_void,
                                                      (*sp).bnst_data.offset(j
                                                          as
                                                          isize)
                                                          as
                                                          *mut libc::c_void)
                                    ==
                                    (0 as libc::c_int == 0) as libc::c_int
                            {
                                dep_check[(i + k) as usize] = j
                            } else if dep_check[(i + k) as usize] ==
                                -(1 as libc::c_int) {
                                if Language != 2 as libc::c_int &&
                                    (OptParaFix != 0 ||
                                        (*Dpnd_matrix.as_mut_ptr().offset((i
                                            +
                                            k)
                                            as
                                            isize))[j
                                            as
                                            usize]
                                            != 0) &&
                                    (*(*(*sp).bnst_data.offset(i as
                                        isize).offset(k
                                        as
                                        isize)).dpnd_rule).preference
                                        != -(1 as libc::c_int) &&
                                    (*(*(*sp).bnst_data.offset(i as
                                        isize).offset(k
                                        as
                                        isize)).dpnd_rule).barrier.fp[0
                                        as
                                        libc::c_int
                                        as
                                        usize].is_null()
                                {
                                    dep_check[(i + k) as usize] = j
                                } else {
                                    dep_check[(i + k) as usize] =
                                        0 as libc::c_int
                                }
                            }
                        }
                    }
                    k += 1
                }
                if OptParaFix == 0 as libc::c_int {
                    next_pp = 0 as *mut *mut CKY;
                    k = 0 as libc::c_int;
                    while k < j - i - 1 as libc::c_int {
                        right_ptr =
                            cky_matrix[(i + k + 1 as libc::c_int) as
                                usize][j as usize];
                        while !right_ptr.is_null() {
                            left_ptr = right_ptr;
                            while !left_ptr.is_null() &&
                                ((*left_ptr).dpnd_type as libc::c_int ==
                                    'P' as i32 ||
                                    (*left_ptr).para_flag != 0) {
                                left_ptr = (*left_ptr).left
                            }
                            if !left_ptr.is_null() && left_ptr != right_ptr {
                                left_ptr =
                                    cky_matrix[i as
                                        usize][(*left_ptr).j as
                                        usize];
                                while !left_ptr.is_null() {
                                    if (*left_ptr).dpnd_type as libc::c_int ==
                                        'P' as i32 {
                                        cky_ptr =
                                            new_cky_data(&mut cky_table_num);
                                        if cky_ptr.is_null() {
                                            return 0 as libc::c_int;
                                        }
                                        if next_pp.is_null() {
                                            start_ptr = cky_ptr
                                        } else { *next_pp = cky_ptr }
                                        set_cky(sp, cky_ptr, left_ptr,
                                                right_ptr, i, j, k,
                                                'P' as i32 as libc::c_char,
                                                1 as libc::c_int,
                                                -(1 as libc::c_int));
                                        next_pp = &mut (*cky_ptr).next;
                                        if OptDisplay == 3 as libc::c_int {
                                            printf(b"** (%d,%d), (%d,%d) b=%d [%s--%s], P(para=--), score=\x00"
                                                       as *const u8 as
                                                       *const libc::c_char, i,
                                                   (*left_ptr).j,
                                                   i + k + 1 as libc::c_int,
                                                   j, dep_check[i as usize],
                                                   (*(*(*sp).bnst_data.offset(i
                                                       as
                                                       isize)).head_ptr).Goi.as_mut_ptr(),
                                                   (*(*(*sp).bnst_data.offset((*left_ptr).j
                                                       as
                                                       isize)).head_ptr).Goi.as_mut_ptr());
                                        }
                                        (*cky_ptr).para_flag =
                                            1 as libc::c_int;
                                        (*cky_ptr).para_score =
                                            (*(*cky_ptr).left).para_score +
                                                (*(*cky_ptr).right).para_score;
                                        (*cky_ptr).score =
                                            if OptAnalysis == 1 as libc::c_int
                                            {
                                                calc_case_probability(sp,
                                                                      cky_ptr,
                                                                      Best_mgr)
                                            } else { calc_score(sp, cky_ptr) }
                                    }
                                    left_ptr = (*left_ptr).next
                                }
                            }
                            right_ptr = (*right_ptr).next
                        }
                        k += 1
                    }
                    if !next_pp.is_null() {
                        if next_pp_for_ij.is_null() {
                            cky_matrix[i as usize][j as usize] = start_ptr
                        } else { *next_pp_for_ij = start_ptr }
                        next_pp_for_ij = next_pp
                    }
                }
                if !next_pp_for_ij.is_null() {
                    if OptBeam != 0 &&
                        !(i == 0 as libc::c_int &&
                            j == (*sp).Bnst_num - 1 as libc::c_int) {
                        sort_cky_ptrs(&mut *(*cky_matrix.as_mut_ptr().offset(i
                            as
                            isize)).as_mut_ptr().offset(j
                            as
                            isize),
                                      OptBeam);
                        next_pp =
                            &mut *(*cky_matrix.as_mut_ptr().offset(i as
                                isize)).as_mut_ptr().offset(j
                                as
                                isize)
                                as *mut *mut CKY;
                        l = 0 as libc::c_int;
                        while l < OptBeam && !(*next_pp).is_null() {
                            next_pp = &mut (**next_pp).next;
                            l += 1
                        }
                        *next_pp = 0 as *mut CKY
                    } else {
                        sort_cky_ptrs(&mut *(*cky_matrix.as_mut_ptr().offset(i
                            as
                            isize)).as_mut_ptr().offset(j
                            as
                            isize),
                                      1 as libc::c_int);
                    }
                    if Language == 2 as libc::c_int &&
                        !(*cky_matrix[i as
                            usize][j as
                            usize]).next.is_null()
                        &&
                        !(*(*cky_matrix[i as
                            usize][j as
                            usize]).next).next.is_null()
                    {
                        m = 1 as libc::c_int;
                        sort_flag = 1 as libc::c_int;
                        while m < 10 as libc::c_int && sort_flag != 0 {
                            cky_ptr = cky_matrix[i as usize][j as usize];
                            sort_pre_ptr = 0 as *mut CKY;
                            l = 0 as libc::c_int;
                            while l < m {
                                if (*cky_ptr).next.is_null() { break; }
                                sort_pre_ptr = cky_ptr;
                                cky_ptr = (*cky_ptr).next;
                                l += 1
                            }
                            if !(*cky_ptr).next.is_null() {
                                sort_flag = 1 as libc::c_int;
                                best_score =
                                    -(2147483647 as libc::c_int) as
                                        libc::c_double;
                                pre_ptr = 0 as *mut CKY;
                                best_pre_ptr = 0 as *mut CKY;
                                while !cky_ptr.is_null() {
                                    if (*cky_ptr).score > best_score {
                                        best_score = (*cky_ptr).score;
                                        best_ptr = cky_ptr;
                                        best_pre_ptr = pre_ptr
                                    }
                                    pre_ptr = cky_ptr;
                                    cky_ptr = (*cky_ptr).next
                                }
                                if !best_pre_ptr.is_null() {
                                    (*best_pre_ptr).next = (*best_ptr).next;
                                    (*best_ptr).next = (*sort_pre_ptr).next;
                                    (*sort_pre_ptr).next = best_ptr
                                }
                                tmp_ptr = cky_matrix[i as usize][j as usize];
                                while !tmp_ptr.is_null() &&
                                    tmp_ptr != (*sort_pre_ptr).next &&
                                    !sort_pre_ptr.is_null() &&
                                    !best_ptr.is_null() {
                                    if (*tmp_ptr).score - (*best_ptr).score <
                                        0.0000000000000001f64 &&
                                        (*tmp_ptr).score -
                                            (*best_ptr).score >
                                            -0.0000000000000001f64 &&
                                        (*tmp_ptr).direction ==
                                            (*best_ptr).direction &&
                                        (*tmp_ptr).b_ptr ==
                                            (*best_ptr).b_ptr &&
                                        (*(*tmp_ptr).left).b_ptr ==
                                            (*(*best_ptr).left).b_ptr &&
                                        (*(*tmp_ptr).right).b_ptr ==
                                            (*(*best_ptr).right).b_ptr {
                                        (*sort_pre_ptr).next =
                                            (*best_ptr).next;
                                        best_ptr = 0 as *mut CKY;
                                        m -= 1;
                                        break;
                                    } else { tmp_ptr = (*tmp_ptr).next }
                                }
                            } else { sort_flag = 0 as libc::c_int }
                            m += 1
                        }
                        cky_ptr = cky_matrix[i as usize][j as usize];
                        l = 0 as libc::c_int;
                        while l < m {
                            if !(*cky_ptr).next.is_null() {
                                cky_ptr = (*cky_ptr).next
                            }
                            l += 1
                        }
                        (*cky_ptr).next = 0 as CKYptr
                    }
                }
            }
            i -= 1
        }
        j += 1
    }
    if OptDisplay == 3 as libc::c_int {
        printf(b">>> n=%d\n\x00" as *const u8 as *const libc::c_char,
               cky_table_num);
    }
    if cky_matrix[0 as libc::c_int as
        usize][((*sp).Bnst_num - 1 as libc::c_int) as
        usize].is_null() {
        return after_cky(sp, Best_mgr, 0 as *mut CKY, 0 as libc::c_int,
                         eos_flag);
    }
    cky_ptr =
        cky_matrix[0 as libc::c_int as
            usize][((*sp).Bnst_num - 1 as libc::c_int) as usize];
    if OptCaseFlag & 256 as libc::c_int != 0 {
        while !cky_ptr.is_null() {
            (*cky_ptr).score +=
                calc_vp_modifying_probability(0 as *mut TAG_DATA,
                                              0 as *mut CASE_FRAME,
                                              (*(*cky_ptr).b_ptr).tag_ptr.offset((*(*cky_ptr).b_ptr).tag_num
                                                  as
                                                  isize).offset(-(1
                                                  as
                                                  libc::c_int
                                                  as
                                                  isize)),
                                              (*(*cky_ptr).cpm_ptr).cmm[0 as
                                                  libc::c_int
                                                  as
                                                  usize].cf_ptr);
            cky_ptr = (*cky_ptr).next
        }
        cky_ptr =
            cky_matrix[0 as libc::c_int as
                usize][((*sp).Bnst_num - 1 as libc::c_int) as
                usize]
    }
    if !(*cky_ptr).next.is_null() {
        best_score = -(2147483647 as libc::c_int) as libc::c_double;
        pre_ptr = 0 as *mut CKY;
        while !cky_ptr.is_null() {
            if (*cky_ptr).score > best_score {
                best_score = (*cky_ptr).score;
                best_ptr = cky_ptr;
                best_pre_ptr = pre_ptr
            }
            pre_ptr = cky_ptr;
            cky_ptr = (*cky_ptr).next
        }
        if pre_ptr != best_ptr {
            if !best_pre_ptr.is_null() {
                (*best_pre_ptr).next = (*best_ptr).next
            } else {
                cky_matrix[0 as libc::c_int as
                    usize][((*sp).Bnst_num - 1 as libc::c_int) as
                    usize] =
                    (*cky_matrix[0 as libc::c_int as
                        usize][((*sp).Bnst_num -
                        1 as libc::c_int) as
                        usize]).next
            }
            (*pre_ptr).next = best_ptr;
            (*best_ptr).next = 0 as CKYptr
        }
        if OptNbest == (0 as libc::c_int == 0) as libc::c_int {
            cky_ptr =
                cky_matrix[0 as libc::c_int as
                    usize][((*sp).Bnst_num - 1 as libc::c_int) as
                    usize]
        } else { cky_ptr = best_ptr }
    }
    return after_cky(sp, Best_mgr, cky_ptr,
                     (0 as libc::c_int == 0) as libc::c_int, eos_flag);
}

pub unsafe extern "C" fn exist_chi(mut sp: *mut SENTENCE_DATA, mut i: libc::c_int, mut j: libc::c_int, mut type_0: *const c_char) -> libc::c_int {
    if strcmp(type_0, b"noun\x00" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"PU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() &&
                (strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b",\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9a\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9b\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x8c\x00" as *const u8 as *const libc::c_char) == 0){
                break;
            }
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"NN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"NT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"NR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null(){
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"DEC\x00" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"DEC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"CC\x00" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"CC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"pu\x00" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"PU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() &&
                (strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b",\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9a\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x9b\x00" as *const u8 as *const libc::c_char) == 0 ||
                    strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xef\xbc\x8c\x00" as *const u8 as *const libc::c_char) == 0){
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"dunhao" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"PU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() &&
                (strcmp((*(*(*sp).bnst_data.offset(i as isize)).head_ptr).Goi.as_mut_ptr(), b"\xE3\x80\x81\x00" as *const u8 as *const libc::c_char) == 0) {
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"verb" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"VV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"VA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null(){
                return i;
            }
            i += 1
        }
    } else if strcmp(type_0, b"prep" as *const u8 as *const libc::c_char) == 0 {
        while i <= j {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f, b"P\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                return i;
            }
            i += 1
        }
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn check_pos_num_chi(mut sp: *mut SENTENCE_DATA,
                                           mut type_0: *mut libc::c_char)
                                           -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    if strcmp(type_0, b"verb\x00" as *const u8 as *const libc::c_char) == 0 {
        k = 0 as libc::c_int;
        while k < (*sp).Bnst_num {
            if !check_feature((*(*sp).bnst_data.offset(k as isize)).f, b"VV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(k as isize)).f, b"VA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(k as isize)).f, b"VC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).bnst_data.offset(k as isize)).f, b"VE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                num += 1
            }
            k += 1
        }
    } else if strcmp(type_0, b"DEC\x00" as *const u8 as *const libc::c_char)
        == 0 {
        k = 0 as libc::c_int;
        while k < (*sp).Bnst_num {
            if !check_feature((*(*sp).bnst_data.offset(k as isize)).f, b"DEC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                num += 1
            }
            k += 1
        }
    }
    return num;
}

#[no_mangle]
pub unsafe extern "C" fn has_child_chi(mut sp: *mut SENTENCE_DATA,
                                       mut cky_ptr: *mut CKY,
                                       mut pos: *mut libc::c_char,
                                       mut direction: libc::c_int)
                                       -> libc::c_int {
    let mut ptr: *mut CKY = cky_ptr;
    if (*ptr).direction == 1 as libc::c_int {
        if direction == 0 as libc::c_int {
            if !(*ptr).left.is_null() &&
                !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).left).b_ptr).num
                    as isize)).f,
                               pos).is_null() {
                return 1 as libc::c_int;
            }
            if !(*ptr).right.is_null() {
                ptr = (*ptr).right;
                while !ptr.is_null() {
                    if (*ptr).direction == 1 as libc::c_int {
                        if !(*ptr).left.is_null() &&
                            !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).left).b_ptr).num
                                as
                                isize)).f,
                                           pos).is_null() {
                            return 1 as libc::c_int;
                        } else { ptr = (*ptr).right }
                    } else { ptr = (*ptr).left }
                }
            }
        } else if !(*ptr).right.is_null() {
            ptr = (*ptr).right;
            while !ptr.is_null() {
                if (*ptr).direction == -(1 as libc::c_int) {
                    if !(*ptr).right.is_null() &&
                        !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).right).b_ptr).num
                            as
                            isize)).f,
                                       pos).is_null() {
                        return 1 as libc::c_int;
                    } else { ptr = (*ptr).left }
                } else { ptr = (*ptr).right }
            }
        }
    } else if direction == 1 as libc::c_int {
        if !(*ptr).right.is_null() &&
            !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).right).b_ptr).num
                as isize)).f,
                           pos).is_null() {
            return 1 as libc::c_int;
        }
        if !(*ptr).left.is_null() {
            ptr = (*ptr).left;
            while !ptr.is_null() {
                if (*ptr).direction == -(1 as libc::c_int) {
                    if !(*ptr).right.is_null() &&
                        !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).right).b_ptr).num
                            as
                            isize)).f,
                                       pos).is_null() {
                        return 1 as libc::c_int;
                    } else { ptr = (*ptr).left }
                } else { ptr = (*ptr).right }
            }
        }
    } else if !(*ptr).left.is_null() {
        ptr = (*ptr).left;
        while !ptr.is_null() {
            if (*ptr).direction == 1 as libc::c_int {
                if !(*ptr).left.is_null() &&
                    !check_feature((*(*sp).bnst_data.offset((*(*(*ptr).left).b_ptr).num
                        as
                        isize)).f,
                                   pos).is_null() {
                    return 1 as libc::c_int;
                } else { ptr = (*ptr).right }
            } else { ptr = (*ptr).left }
        }
    }
    return 0 as libc::c_int;
}

pub unsafe extern "C" fn check_chi_dpnd_possibility (mut i: libc::c_int,
                                                     mut j: libc::c_int,
                                                     mut k: libc::c_int,
                                                     mut left: *mut CKY,
                                                     mut right: *mut CKY,
                                                     mut sp: *mut SENTENCE_DATA,
                                                     mut direction: libc::c_int,
                                                     mut index: libc::c_int) -> libc::c_int {
    let mut left_pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right_pos: *mut libc::c_char = 0 as *mut libc::c_char;

    return if Language != CHINESE {
        1
    } else {
        if Dpnd_matrix[left.b_ptr.num][right.b_ptr.num] > 0 && Dpnd_matrix[left.b_ptr.num][right.b_ptr.num] != 'O' {
            if direction != Dpnd_matrix[left.b_ptr.num][right.b_ptr.num] {
                return 0;
            }
        }

        if !OptChiPos {
            left_pos = (sp.bnst_data + left.b_ptr.num).head_ptr.Pos;
            right_pos = (sp.bnst_data + right.b_ptr.num).head_ptr.Pos;
        } else {
            left_pos = Chi_word_pos[Chi_dpnd_matrix[left.b_ptr.num][right.b_ptr.num].left_pos_index[index]];
            right_pos = Chi_word_pos[Chi_dpnd_matrix[left.b_ptr.num][right.b_ptr.num].right_pos_index[index]];
        }

        /* check if this cky corresponds with the grammar rules for Chinese */
        /* LC cannot depend on noun */
        if !strcmp(left_pos, "LC" as *const libc::c_char) != 0 &&
            (!strcmp(right_pos, "NN" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "NR" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "PN" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "NT" as *const libc::c_char) != 0) &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* sp and main verb */
        if !strcmp(right_pos, "SP" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VV" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VA" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VC" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VE" as *const libc::c_char) != 0 &&
            direction == 'L' as libc::c_int {
            return 0;
        }

        /* verb cannot depend on SP */
        if (!strcmp(right_pos, "SP" as *const libc::c_char) != 0 &&
            (!strcmp(left_pos, "VV" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VA" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VC" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VE" as *const libc::c_char) != 0) &&
            direction == 'R' as libc::c_int) ||
            (!strcmp(left_pos, "SP" as *const libc::c_char) != 0 &&
                (!strcmp(right_pos, "VV" as *const libc::c_char) != 0 ||
                    !strcmp(right_pos, "VA" as *const libc::c_char) != 0 ||
                    !strcmp(right_pos, "VC" as *const libc::c_char) != 0 ||
                    !strcmp(right_pos, "VE" as *const libc::c_char) != 0) &&
                direction == 'L' as libc::c_int) {
            return 0;
        }

        /* adj and verb cannot have dependency relation */
        if (!strcmp(left_pos, "JJ" as *const libc::c_char) != 0 &&
            (!strcmp(right_pos, "VV" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VA" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VC" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VE" as *const libc::c_char) != 0)) ||
            (!strcmp(right_pos, "JJ" as *const libc::c_char) != 0 &&
                (!strcmp(left_pos, "VV" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VA" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VC" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VE" as *const libc::c_char) != 0)) {
            return 0;
        }

        /* only the quote PU can be head */
        if (direction == 'R' as libc::c_int &&
            !strcmp(right_pos, "PU" as *const libc::c_char) != 0 &&
            Chi_quote_end_matrix[right.b_ptr.num][right.b_ptr.num] != right.b_ptr.num) ||
            (direction == 'L' as libc::c_int &&
                !strcmp(left_pos, "PU" as *const libc::c_char) != 0 &&
                Chi_quote_start_matrix[left.b_ptr.num][left.b_ptr.num] != left.b_ptr.num) {
            return 0;
        }

        /* AD cannot be head except for AD */
        if (!strcmp(left_pos, "AD" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "AD" as *const libc::c_char) != 0 &&
            direction == 'L' as libc::c_int) ||
            (!strcmp(right_pos, "AD" as *const libc::c_char) != 0 &&
                strcmp(left_pos, "AD" as *const libc::c_char) != 0 &&
                direction == 'R' as libc::c_int) {
            return 0;
        }

        /* DEC cannot depend on VV before */
        if !strcmp(left_pos, "VV" as *const libc::c_char) != 0 &&
            direction == 'L' as libc::c_int &&
            !strcmp(right_pos, "DEC" as *const libc::c_char) != 0 {
            return 0;
        }

        /* for DEG , DEV, DEC and LC, there should not be two modifiers */
        if (!strcmp(right_pos, "DEG" as *const libc::c_char) != 0 ||
            !strcmp(right_pos, "DEV" as *const libc::c_char) != 0 ||
            !strcmp(right_pos, "LC" as *const libc::c_char) != 0) &&
            right.b_ptr.num - right.i > 0 &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* DEC cannot have two verb modifiers */
        if !strcmp(right_pos, "DEC" as *const libc::c_char) != 0 &&
            direction == 'R' as libc::c_int &&
            (!strcmp(left_pos, "VV" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VA" as *const libc::c_char) != 0) &&
            (has_child_chi(sp, right, "VV" as *mut libc::c_char, 0) ||
                has_child_chi(sp, right, "VA" as *mut libc::c_char, 0)) {
            return 0;
        }

        /* for DEC, if there exists noun between it and previous verb, the noun should depend on verb */
        if !strcmp(right_pos, "DEC" as *const libc::c_char) != 0 &&
            (!strcmp(left_pos, "VV" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VA" as *const libc::c_char) != 0) &&
            exist_chi(sp, right.i, right.b_ptr.num - 1, "noun" as *const libc::c_char) != -1 &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* for DEG, its right head should be noun afterwords */
        if !strcmp(left_pos, "DEG" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "NN" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "NT" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "NR" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "PN" as *const libc::c_char) != 0 &&
            strcmp(right_pos, "M" as *const libc::c_char) != 0 &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* for DEG and DEC, it must have some word before modifying it */
        if ((!strcmp(left_pos, "DEG" as *const libc::c_char) != 0 ||
            !strcmp(left_pos, "DEC" as *const libc::c_char) != 0) &&
            left.i == left.b_ptr.num &&
            direction == 'R' as libc::c_int) ||
            ((!strcmp(right_pos, "DEG" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "DEC" as *const libc::c_char) != 0) &&
                right.i == right.b_ptr.num &&
                direction == 'L' as libc::c_int) {
            return 0;
        }

        /* for DEC, it must have some verb before modifying it */
        if strcmp(left_pos, "VV" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VA" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VC" as *const libc::c_char) != 0 &&
            strcmp(left_pos, "VE" as *const libc::c_char) != 0 &&
            !strcmp(right_pos, "DEC" as *const libc::c_char) != 0 &&
            right.i == right.b_ptr.num &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* LC must have modifier before */
        if !strcmp(left_pos, "LC" as *const libc::c_char) && (left.i == left.b_ptr.num) as libc::c_int {
            return 0;
        }

        /* VC and VE must have modifier behind */
        if (!strcmp(left_pos, "VC" as *const libc::c_char) ||
            !strcmp(left_pos, "VE" as *const libc::c_char)) &&
            (direction == 'R' as libc::c_int &&
                left.j == left.b_ptr.num) {
            return 0;
        }

        /* for verb, there should be only one object afterword */
        if (!strcmp(left_pos, "VV" as *const libc::c_char) ||
            !strcmp(left_pos, "VC" as *const libc::c_char) ||
            !strcmp(left_pos, "VE" as *const libc::c_char) != 0 ||
            !strcmp(left_pos, "P" as *const libc::c_char) != 0 ||
            !strcmp(left_pos, "VA" as *const libc::c_char) != 0) &&
            (!strcmp(right_pos, "NN" as *const libc::c_char) ||
                !strcmp(right_pos, "NR" as *const libc::c_char) ||
                !strcmp(right_pos, "PN" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "M" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "DEG" as *const libc::c_char) != 0) &&
            direction == 'L' as libc::c_int &&
            left.j != left.i &&
            (has_child_chi(sp, left, "NN" as *mut libc::c_char, 1) != 0 ||
                has_child_chi(sp, left, "NR" as *mut libc::c_char, 1) != 0 ||
                has_child_chi(sp, left, "M" as *mut libc::c_char, 1) != 0 ||
                has_child_chi(sp, left, "DEG" as *mut libc::c_char, 1) != 0 ||
                has_child_chi(sp, left, "PN" as *mut libc::c_char, 1) != 0) {
            return 0;
        }

        /* if a verb has object, then between the verb and its object, there should not be another verb depend on the first verb */
        if (!strcmp(left_pos, "VV" as *const libc::c_char) ||
            !strcmp(left_pos, "VC" as *const libc::c_char) ||
            !strcmp(left_pos, "VE" as *const libc::c_char) != 0 ||
            !strcmp(left_pos, "VA" as *const libc::c_char) != 0) &&
            (!strcmp(right_pos, "NN" as *const libc::c_char) ||
                !strcmp(right_pos, "PN" as *const libc::c_char) ||
                !strcmp(right_pos, "NR" as *const libc::c_char) != 0) &&
            (has_child_chi(sp, left, "VV" as *mut libc::c_char, 1) ||
                has_child_chi(sp, left, "VA" as *mut libc::c_char, 1) ||
                has_child_chi(sp, left, "VC" as *mut libc::c_char, 1) != 0 ||
                has_child_chi(sp, left, "VE" as *mut libc::c_char, 1) != 0) {
            return 0;
        }

        /* for verb, there should be only one subject in front of it */
        if (!strcmp(right_pos, "VV" as *const libc::c_char) ||
            !strcmp(right_pos, "VC" as *const libc::c_char) ||
            !strcmp(right_pos, "VE" as *const libc::c_char) != 0 ||
            !strcmp(right_pos, "VA" as *const libc::c_char) != 0) &&
            (!strcmp(left_pos, "NN" as *const libc::c_char) ||
                !strcmp(left_pos, "PN" as *const libc::c_char) ||
                !strcmp(left_pos, "NR" as *const libc::c_char) != 0) &&
            direction == 'R' as libc::c_int &&
            right.j != right.i &&
            (has_child_chi(sp, right, "NN" as *mut libc::c_char, 0) != 0 ||
                has_child_chi(sp, right, "NR" as *mut libc::c_char, 0) != 0 ||
                has_child_chi(sp, right, "PN" as *mut libc::c_char, 0) != 0) {
            return 0;
        }

        /* for preposition, it must have non-pu modifier */
        if (!strcmp(left_pos, "P" as *const libc::c_char) && (direction == ('R' as libc::c_int)) as libc::c_int && left.j - left.i == 0) ||
            (!strcmp(right_pos, "P" as *const libc::c_char) && (direction == ('L' as libc::c_int)) as libc::c_int && right.j - right.i == 0) {
            return 0;
        }

        /* for preposition, it cannot depend on a preposition */
        if !strcmp(right_pos, "P" as *const libc::c_char) && !strcmp(left_pos, "P" as *const libc::c_char) {
            return 0;
        }

        /* for preposition, it cannot depend on a noun before */
        if !strcmp(right_pos, "P" as *const libc::c_char) != 0 &&
            (!strcmp(left_pos, "NN" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "NR" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "PN" as *const libc::c_char) != 0) {
            return 0;
        }

        /* for preposition, it cannot depend on a CD or AD */
        if !strcmp(left_pos, "P" as *const libc::c_char) && !strcmp(right_pos, "CD" as *const libc::c_char) && direction == 'R' as libc::c_int {
            return 0;
        }

        /* a noun cannot depend on its following preposition */
        if !strcmp(right_pos, "P" as *const libc::c_char) != 0 &&
            (!strcmp(left_pos, "NN" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "NR" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "PN" as *const libc::c_char) != 0) &&
            direction == 'R' as libc::c_int {
            return 0;
        }

        /* for preposition, if it depend on verb, it should have modifier */
        if (!strcmp(left_pos, "P" as *const libc::c_char) != 0 &&
            (!strcmp(right_pos, "VA" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VC" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VE" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VV" as *const libc::c_char) != 0) &&
            direction == 'R' as libc::c_int &&
            left.j == left.b_ptr.num) ||
            (!strcmp(right_pos, "P" as *const libc::c_char) != 0 &&
                (!strcmp(left_pos, "VA" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VC" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VE" as *const libc::c_char) != 0 ||
                    !strcmp(left_pos, "VV" as *const libc::c_char) != 0) &&
                direction == 'L' as libc::c_int &&
                right.j == right.b_ptr.num) {
            return 0;
        }

        /* for preposition, it cannot have two modifiers after it */
        if !strcmp(left_pos, "P" as *const libc::c_char) != 0 &&
            left.right as bool &&
            !check_feature((sp.bnst_data + left.right.b_ptr.num).f, "PU" as *mut libc::c_char) as bool &&
            direction == 'L' as libc::c_int {
            return 0;
        }

        /* for preposition, if it depend on verb before, the verb should have object */
        if !strcmp(right_pos, "P" as *const libc::c_char) != 0 &&
            (!strcmp(left_pos, "VC" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VE" as *const libc::c_char) != 0 ||
                !strcmp(left_pos, "VV" as *const libc::c_char) != 0) &&
            direction == 'L' as libc::c_int &&
            right.j == sp.Bnst_num - 1 {
            return 0;
        }

        /* for preposition, if there is LC in the following (no preposibion between them), the words between P and LC should depend on LC */
        if !strcmp(left_pos, "P" as *const libc::c_char) &&
            !strcmp(right_pos, "LC" as *const libc::c_char) &&
            left.j - left.i > 0 &&
            exist_chi(sp, left.b_ptr.num + 1, right.b_ptr.num - 1, "prep" as *const libc::c_char) == -1 {
            return 0;
        }

        /* for preposition, if there is noun between it and following verb, if preposition is head of the verb, all the noun should depend on verb, if verb is head of preposition, all the noun should depend on preposition */
        if !strcmp(left_pos, "P" as *const libc::c_char) != 0 &&
            (!strcmp(right_pos, "VV" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VA" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VC" as *const libc::c_char) != 0 ||
                !strcmp(right_pos, "VE" as *const libc::c_char) != 0) &&
            (direction == 'L' as libc::c_int && /* preposition is head */
                left.j != left.i &&
                (has_child_chi(sp, left, "NN" as *mut libc::c_char, 1) ||
                    has_child_chi(sp, left, "NR" as *mut libc::c_char, 1) ||
                    has_child_chi(sp, left, "PN" as *mut libc::c_char, 1) != 0)) {
            return 0;
        }

        if !OptChiPos {
            /* the word before dunhao cannot have left dependency */
            if direction == 'L' as libc::c_int &&
                check_feature((sp.bnst_data + right.b_ptr.num + 1).f, "PU" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + right.b_ptr.num + 2).f, "VV" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + right.b_ptr.num + 2).f, "VC" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + right.b_ptr.num + 2).f, "VE" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + right.b_ptr.num + 2).f, "VA" as *mut libc::c_char) as bool &&
                !strcmp((sp.bnst_data + right.b_ptr.num + 1).head_ptr.Goi, "、" as *mut libc::c_char) != 0 {
                return 0;
            }

            /* if a SB is followed by VV, then this SB cannot have modifier */
            if (check_feature((sp.bnst_data + left.b_ptr.num).f, "SB" as *mut libc::c_char) &&
                check_feature((sp.bnst_data + left.b_ptr.num + 1).f, "VV" as *mut libc::c_char) &&
                left.i != left.b_ptr.num) ||
                (check_feature((sp.bnst_data + right.b_ptr.num).f, "SB" as *mut libc::c_char) &&
                    check_feature((sp.bnst_data + right.b_ptr.num + 1).f, "VV" as *mut libc::c_char) &&
                    (right.i != right.b_ptr.num || direction == 'R' as libc::c_int)) {
                return 0;
            }

            /* the noun before dunhao should depend on noun after it */
            if direction == 'R' as libc::c_int &&
                (check_feature((sp.bnst_data + left.b_ptr.num).f, "NN" as *mut libc::c_char) ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "NR" as *mut libc::c_char) ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "PN" as *mut libc::c_char) as bool ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "JJ" as *mut libc::c_char) as bool ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "NT" as *mut libc::c_char) as bool ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "M" as *mut libc::c_char) as bool ||
                    check_feature((sp.bnst_data + left.b_ptr.num).f, "DEG" as *mut libc::c_char) as bool) &&
                (!check_feature((sp.bnst_data + right.b_ptr.num).f, "NN" as *mut libc::c_char) &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "NT" as *mut libc::c_char) &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "JJ" as *mut libc::c_char) as bool &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "NR" as *mut libc::c_char) as bool &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "PN" as *mut libc::c_char) as bool &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "DEG" as *mut libc::c_char) as bool &&
                    !check_feature((sp.bnst_data + right.b_ptr.num).f, "M" as *mut libc::c_char) as bool) &&
                check_feature((sp.bnst_data + left.b_ptr.num + 1).f, "PU" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + left.b_ptr.num + 2).f, "VV" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + left.b_ptr.num + 2).f, "VC" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + left.b_ptr.num + 2).f, "VE" as *mut libc::c_char) as bool &&
                !check_feature((sp.bnst_data + left.b_ptr.num + 2).f, "VA" as *mut libc::c_char) as bool &&
                !strcmp((sp.bnst_data + left.b_ptr.num + 1).head_ptr.Goi, "、" as *mut libc::c_char) != 0 {
                return 0;
            }

            /* for preposition, if it has a VV modifier after it, this VV should have object or subject */
            if check_feature((sp.bnst_data + left.b_ptr.num).f, "P" as *mut libc::c_char) &&
                check_feature((sp.bnst_data + right.b_ptr.num).f, "VV" as *mut libc::c_char) &&
                direction == 'L' as libc::c_int &&
                !check_feature((sp.bnst_data + right.b_ptr.num + 1).f, "CC" as *mut libc::c_char) as bool &&
                (!has_child_chi(sp, right, "NN" as *mut libc::c_char, 0) &&
                    !has_child_chi(sp, right, "NR" as *mut libc::c_char, 0) &&
                    !has_child_chi(sp, right, "PN" as *mut libc::c_char, 0) != 0) &&
                (!has_child_chi(sp, right, "NN" as *mut libc::c_char, 1) &&
                    !has_child_chi(sp, right, "NR" as *mut libc::c_char, 1) &&
                    !has_child_chi(sp, right, "PN" as *mut libc::c_char, 1) != 0) {
                return 0;
            }

            /* VC and VE must have modifier before */
            if (check_feature((sp.bnst_data + left.j).f, "VC" as *mut libc::c_char) && (left.j != left.b_ptr.num) as *mut libc::c_char) ||
                (check_feature((sp.bnst_data + right.i).f, "VC" as *mut libc::c_char) && (right.i != right.b_ptr.num) as *mut libc::c_char) {
                return 0;
            }

            /* check if this cky corresponds with the constraint of NP and quote */
            if (Chi_np_end_matrix[i][i + k] != -1 && j > Chi_np_end_matrix[i][i + k]) ||
                (Chi_np_start_matrix[i + k + 1][j] != -1 && i < Chi_np_start_matrix[i + k + 1][j]) {
                return 0;
            }
            if (Chi_quote_end_matrix[i][i + k] != -1 && j > Chi_quote_end_matrix[i][i + k]) ||
                (Chi_quote_start_matrix[i + k + 1][j] != -1 && i < Chi_quote_start_matrix[i + k + 1][j]) {
                return 0;
            }
        }

        1
    }
}