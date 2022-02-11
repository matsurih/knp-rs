#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]


use crate::case_analysis::{make_print_string, pp_code_to_kstr};
use crate::case_ipal::CFSimExist;
use crate::case_match::count_pat_element;
use crate::context::OptUseSmfix;
use crate::ctools::{check_feature, malloc_data, Outfp};
use crate::lib_print::{_print_bnst, print_eos};
use crate::structs::{_sort_kv, CDB_FILE, CF_MATCH_MGR};
use crate::tools::{OptCaseFlag, OptEllipsis, OptExpress};
use crate::TOTAL_MGR;
use crate::types::{CF_PRED_MGR, DBM_FILE, SENTENCE_DATA, size_t};

#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut sm_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut sm2code_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut smp2smg_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
/*====================================================================

			   格構造解析: 表示

                                               S.Kurohashi 93. 5.31

    $Id$
====================================================================*/
#[no_mangle]
pub static mut EX_PRINT_NUM: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub static mut PrintFrequency: libc::c_int = 0 as libc::c_int;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_depend_type(mut cpm_ptr: *mut CF_PRED_MGR,
                                           mut num: libc::c_int,
                                           mut flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    /* 係タイプの出力
       flag == FALSE : 対応可能な格を出力しない
    */
    /* 省略のとき */
    if (*cpm_ptr).elem_b_num[num as usize] == -(2 as libc::c_int) {
        fprintf(Outfp,
                b"\xe3\x80\x8a\xe7\x9c\x81\xe3\x80\x8b\x00" as *const u8 as
                    *const libc::c_char);
        return
    } else {
        /* 照応のとき */
        if (*cpm_ptr).elem_b_num[num as usize] == -(3 as libc::c_int) {
            fprintf(Outfp,
                    b"\xe3\x80\x8a\xe7\x85\xa7\xe3\x80\x8b\x00" as *const u8
                        as *const libc::c_char);
            return
        } else {
            if flag == 0 as libc::c_int &&
                   (*cpm_ptr).elem_b_num[num as usize] == -(1 as libc::c_int)
               {
                fprintf(Outfp,
                        b"\xe3\x80\x8a--\xe3\x80\x8b\x00" as *const u8 as
                            *const libc::c_char);
                return
            }
        }
    }
    fprintf(Outfp, b"\xe3\x80\x8a\x00" as *const u8 as *const libc::c_char);
    if (*cpm_ptr).cf.type_0 == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while (*cpm_ptr).cf.pp[num as usize][i as usize] !=
                  -(10 as libc::c_int) {
            if i != 0 { fputc('/' as i32, Outfp); }
            if (*cpm_ptr).cf.pp[num as usize][i as usize] < 0 as libc::c_int {
                fputs(b"--\x00" as *const u8 as *const libc::c_char, Outfp);
            } else {
                fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                        pp_code_to_kstr((*cpm_ptr).cf.pp[num as
                                                             usize][i as
                                                                        usize]));
            }
            i += 1
        }
    } else { fputs(b"--\x00" as *const u8 as *const libc::c_char, Outfp); }
    fprintf(Outfp, b"\xe3\x80\x8b\x00" as *const u8 as *const libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_data_cframe(mut cpm_ptr: *mut CF_PRED_MGR,
                                           mut cmm_ptr: *mut CF_MATCH_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut tmp: libc::c_char = 0;
    fprintf(Outfp, b"\xe3\x80\x90\x00" as *const u8 as *const libc::c_char);
    if !(*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).entry.is_null() {
        fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                (*(*cpm_ptr).cmm[0 as libc::c_int as usize].cf_ptr).entry);
    } else {
        fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                (*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr());
        /* 用言表記 */
    }
    if (*cpm_ptr).cf.voice == 1 as libc::c_int {
        fprintf(Outfp,
                b"(\xe4\xbd\xbf\xe5\xbd\xb9)\xe3\x80\x91\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*cpm_ptr).cf.voice == 2 as libc::c_int {
        fprintf(Outfp,
                b"(\xe5\x8f\x97\xe8\xba\xab)\xe3\x80\x91\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*cpm_ptr).cf.voice == 4 as libc::c_int {
        fprintf(Outfp,
                b"(\xe4\xbd\xbf\xe5\xbd\xb9&\xe5\x8f\x97\xe8\xba\xab)\xe3\x80\x91\x00"
                    as *const u8 as *const libc::c_char);
    } else if (*cpm_ptr).cf.voice == 8 as libc::c_int {
        fprintf(Outfp,
                b"(\xe3\x82\x82\xe3\x82\x89\xe3\x81\x86)\xe3\x80\x91\x00" as
                    *const u8 as *const libc::c_char);
    } else if (*cpm_ptr).cf.voice == 16 as libc::c_int {
        fprintf(Outfp,
                b"(\xe3\x81\xbb\xe3\x81\x97\xe3\x81\x84)\xe3\x80\x91\x00" as
                    *const u8 as *const libc::c_char);
    } else {
        fprintf(Outfp,
                b"\xe3\x80\x91\x00" as *const u8 as *const libc::c_char);
    }
    fprintf(Outfp, b" %s \x00" as *const u8 as *const libc::c_char,
            (*cpm_ptr).cf.pred_type.as_mut_ptr());
    if OptUseSmfix == (0 as libc::c_int == 0) as libc::c_int &&
           CFSimExist == (0 as libc::c_int == 0) as libc::c_int &&
           (*(*cpm_ptr).pred_b_ptr).cf_num !=
               (*(*cpm_ptr).pred_b_ptr).e_cf_num {
        fprintf(Outfp, b"[%d/%d]\x00" as *const u8 as *const libc::c_char,
                (*(*cpm_ptr).pred_b_ptr).e_cf_num,
                if (*(*cpm_ptr).pred_b_ptr).cf_num > 1 as libc::c_int {
                    ((*(*cpm_ptr).pred_b_ptr).cf_num) - 1 as libc::c_int
                } else { 1 as libc::c_int });
    } else {
        fprintf(Outfp, b"[%d]\x00" as *const u8 as *const libc::c_char,
                if (*(*cpm_ptr).pred_b_ptr).cf_num > 1 as libc::c_int {
                    ((*(*cpm_ptr).pred_b_ptr).cf_num) - 1 as libc::c_int
                } else { 1 as libc::c_int });
    }
    /* 格フレームを決定した方法 */
    if (*cpm_ptr).decided == 2 as libc::c_int {
        fputs(b" D\x00" as *const u8 as *const libc::c_char, Outfp);
    } else if (*cpm_ptr).decided == 1 as libc::c_int {
        fputs(b" C\x00" as *const u8 as *const libc::c_char, Outfp);
    } else { fputs(b" U\x00" as *const u8 as *const libc::c_char, Outfp); }
    i = 0 as libc::c_int;
    while i < (*cpm_ptr).cf.element_num {
        fputc(' ' as i32, Outfp);
        _print_bnst((*cpm_ptr).elem_b_ptr[i as usize]);
        /* 係タイプの出力 */
        print_depend_type(cpm_ptr, i, (0 as libc::c_int == 0) as libc::c_int);
        /* 任意格の要素をマーク */
        if (*cpm_ptr).cf.oblig[i as usize] == 0 as libc::c_int {
            fputc('*' as i32, Outfp);
        }
        i += 1
    }
    if !check_feature((*(*cpm_ptr).pred_b_ptr).f,
                      b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        printf(b" <\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97>\x00"
                   as *const u8 as *const libc::c_char);
    }
    if OptExpress == 16 as libc::c_int {
        fprintf(Outfp, b"<BR>\x00" as *const u8 as *const libc::c_char);
    }
    fputc('\n' as i32, Outfp);
}
/*==================================================================*/
unsafe extern "C" fn number_compare(mut i: *const libc::c_void,
                                    mut j: *const libc::c_void)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* sort function */
    return (*(i as *const _sort_kv)).value - (*(j as *const _sort_kv)).value;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_crrspnd(mut cpm_ptr: *mut CF_PRED_MGR,
                                       mut cmm_ptr: *mut CF_MATCH_MGR) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut print_num: libc::c_int = 0;
    let mut elist: [_sort_kv; 24] = [_sort_kv{key: 0, value: 0,}; 24];
    if (*(*cmm_ptr).cf_ptr).cf_address ==
           -(1 as libc::c_int) as libc::c_ulonglong {
        /* 格フレームがない場合 */
        return
    }
    /* 得点, 意味の表示 */
    fprintf(Outfp,
            b"\xe2\x98\x85%6.3f\xe7\x82\xb9 \x00" as *const u8 as
                *const libc::c_char, (*cmm_ptr).score);
    if OptCaseFlag & 16 as libc::c_int == 0 {
        fprintf(Outfp, b"(%d/%.3f) \x00" as *const u8 as *const libc::c_char,
                (*cmm_ptr).pure_score[0 as libc::c_int as usize] as
                    libc::c_int,
                sqrt(count_pat_element((*cmm_ptr).cf_ptr,
                                       &mut *(*cmm_ptr).result_lists_p.as_mut_ptr().offset(0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize))
                         as libc::c_double));
    }
    fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
            (*(*cmm_ptr).cf_ptr).cf_id.as_mut_ptr());
    /* 格フレーム類似度 */
    if OptUseSmfix == (0 as libc::c_int == 0) as libc::c_int &&
           CFSimExist == (0 as libc::c_int == 0) as libc::c_int {
        fprintf(Outfp, b"(%.2f) \x00" as *const u8 as *const libc::c_char,
                (*(*cmm_ptr).cf_ptr).cf_similarity as libc::c_double);
    }
    if !(*(*cmm_ptr).cf_ptr).feature.is_null() {
        fprintf(Outfp, b"%s \x00" as *const u8 as *const libc::c_char,
                (*(*cmm_ptr).cf_ptr).feature);
    }
    if (*(*cmm_ptr).cf_ptr).voice == 2 as libc::c_int {
        fprintf(Outfp,
                b"(\xe9\x96\x93\xe5\x8f\x97)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 3 as libc::c_int {
        fprintf(Outfp,
                b"(\xe7\x9b\xb4\xe5\x8f\x971)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 4 as libc::c_int {
        fprintf(Outfp,
                b"(\xe7\x9b\xb4\xe5\x8f\x972)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 5 as libc::c_int ||
                  (*(*cmm_ptr).cf_ptr).voice == 6 as libc::c_int ||
                  (*(*cmm_ptr).cf_ptr).voice == 7 as libc::c_int {
        fprintf(Outfp,
                b"(\xe4\xbd\xbf\xe5\xbd\xb9)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 8 as libc::c_int {
        fprintf(Outfp,
                b"(\xe4\xbd\xbf\xe5\xbd\xb9&\xe5\x8f\x97\xe8\xba\xab)\x00" as
                    *const u8 as *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 9 as libc::c_int {
        fprintf(Outfp,
                b"(\xe5\x8f\xaf\xe8\x83\xbd)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 10 as libc::c_int {
        fprintf(Outfp,
                b"(\xe5\xb0\x8a\xe6\x95\xac)\x00" as *const u8 as
                    *const libc::c_char);
    } else if (*(*cmm_ptr).cf_ptr).voice == 11 as libc::c_int {
        fprintf(Outfp,
                b"(\xe8\x87\xaa\xe7\x99\xba)\x00" as *const u8 as
                    *const libc::c_char);
    }
    /* fprintf(Outfp, "%s\n", i_ptr->DATA + i_ptr->imi); */
    if OptExpress == 16 as libc::c_int {
        fprintf(Outfp, b"<BR>\n\x00" as *const u8 as *const libc::c_char);
    } else {
        fputs(b"-----------------------------------\n\x00" as *const u8 as
                  *const libc::c_char, Outfp);
    }
    /* 格要素対応の表示 */
    k = 0 as libc::c_int;
    while k < (*cmm_ptr).result_num {
        if k != 0 as libc::c_int {
            fputs(b"---\n\x00" as *const u8 as *const libc::c_char, Outfp);
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"<BR>\x00" as *const u8 as *const libc::c_char);
            }
        }
        /* 格をソートして出力 */
        i = 0 as libc::c_int;
        while i < (*(*cmm_ptr).cf_ptr).element_num {
            elist[i as usize].key = i;
            elist[i as usize].value =
                (*(*cmm_ptr).cf_ptr).pp[i as
                                            usize][0 as libc::c_int as usize];
            i += 1
        }
        qsort(elist.as_mut_ptr() as *mut libc::c_void,
              (*(*cmm_ptr).cf_ptr).element_num as size_t,
              ::std::mem::size_of::<_sort_kv>() as libc::c_ulong,
              Some(number_compare as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
        l = 0 as libc::c_int;
        while l < (*(*cmm_ptr).cf_ptr).element_num {
            i = elist[l as usize].key;
            num = (*cmm_ptr).result_lists_p[k as usize].flag[i as usize];
            if (*(*cmm_ptr).cf_ptr).adjacent[i as usize] ==
                   (0 as libc::c_int == 0) as libc::c_int {
                fprintf(Outfp,
                        b" \xe2\x97\x8e \x00" as *const u8 as
                            *const libc::c_char);
            } else {
                fprintf(Outfp,
                        b" \xe2\x97\x8f \x00" as *const u8 as
                            *const libc::c_char);
            }
            if num == -(1 as libc::c_int) ||
                   (*cmm_ptr).score == -(2 as libc::c_int) as libc::c_double {
                /* -2は全体で不一致 */
                fputs(b"--\x00" as *const u8 as *const libc::c_char, Outfp);
                if OptCaseFlag & 16 as libc::c_int != 0 {
                    fprintf(Outfp,
                            b" \xef\xbc\xbb%.3f\xef\xbc\xbd\x00" as *const u8
                                as *const libc::c_char,
                            (*cmm_ptr).result_lists_p[k as
                                                          usize].score[i as
                                                                           usize]);
                }
            } else {
                _print_bnst((*cpm_ptr).elem_b_ptr[num as usize]);
                /* 係タイプの出力 */
                print_depend_type(cpm_ptr, num, 0 as libc::c_int);
                if num != -(1 as libc::c_int) &&
                       (*cpm_ptr).cf.oblig[num as usize] == 0 as libc::c_int {
                    fputc('*' as i32, Outfp);
                }
                /* 格ごとのスコアを表示 */
                if OptCaseFlag & 16 as libc::c_int != 0 {
                    fprintf(Outfp,
                            b"\xef\xbc\xbb%.3f\xef\xbc\xbd\x00" as *const u8
                                as *const libc::c_char,
                            (*cmm_ptr).result_lists_p[k as
                                                          usize].score[i as
                                                                           usize]);
                } else if (*cmm_ptr).result_lists_p[k as
                                                        usize].score[i as
                                                                         usize]
                              >= 0 as libc::c_int as libc::c_double {
                    fprintf(Outfp,
                            b"\xef\xbc\xbb%2d\xe7\x82\xb9\xef\xbc\xbd\x00" as
                                *const u8 as *const libc::c_char,
                            (*cmm_ptr).result_lists_p[k as
                                                          usize].score[i as
                                                                           usize]
                                as libc::c_int);
                }
            }
            fprintf(Outfp,
                    b" : \xe3\x80\x8a\x00" as *const u8 as
                        *const libc::c_char);
            if !(*(*cmm_ptr).cf_ptr).pp_str[i as usize].is_null() {
                fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*(*cmm_ptr).cf_ptr).pp_str[i as usize]);
            } else {
                j = 0 as libc::c_int;
                while (*(*cmm_ptr).cf_ptr).pp[i as usize][j as usize] !=
                          -(10 as libc::c_int) {
                    if j != 0 as libc::c_int { fputc('/' as i32, Outfp); }
                    fprintf(Outfp,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            pp_code_to_kstr((*(*cmm_ptr).cf_ptr).pp[i as
                                                                        usize][j
                                                                                   as
                                                                                   usize]));
                    j += 1
                }
            }
            fprintf(Outfp,
                    b"\xe3\x80\x8b\x00" as *const u8 as *const libc::c_char);
            /* 用例の出力 */
            if !(*(*cmm_ptr).cf_ptr).ex_list[i as usize].is_null() {
                print_num =
                    if EX_PRINT_NUM < 0 as libc::c_int {
                        (*(*cmm_ptr).cf_ptr).ex_num[i as usize]
                    } else if (*(*cmm_ptr).cf_ptr).ex_num[i as usize] >
                                  EX_PRINT_NUM {
                        EX_PRINT_NUM
                    } else { (*(*cmm_ptr).cf_ptr).ex_num[i as usize] };
                fputc('(' as i32, Outfp);
                j = 0 as libc::c_int;
                while j < print_num {
                    if j != 0 as libc::c_int { fputc('|' as i32, Outfp); }
                    if j ==
                           (*cmm_ptr).result_lists_p[k as
                                                         usize].pos[i as
                                                                        usize]
                       {
                        fprintf(Outfp,
                                b"\xe3\x80\x90\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    if PrintFrequency != 0 {
                        fprintf(Outfp,
                                b"%s:%d\x00" as *const u8 as
                                    *const libc::c_char,
                                *(*(*cmm_ptr).cf_ptr).ex_list[i as
                                                                  usize].offset(j
                                                                                    as
                                                                                    isize),
                                *(*(*cmm_ptr).cf_ptr).ex_freq[i as
                                                                  usize].offset(j
                                                                                    as
                                                                                    isize));
                    } else {
                        fprintf(Outfp,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                *(*(*cmm_ptr).cf_ptr).ex_list[i as
                                                                  usize].offset(j
                                                                                    as
                                                                                    isize));
                    }
                    if j ==
                           (*cmm_ptr).result_lists_p[k as
                                                         usize].pos[i as
                                                                        usize]
                       {
                        fprintf(Outfp,
                                b"\xe3\x80\x91\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    j += 1
                }
                if (*cmm_ptr).result_lists_p[k as usize].pos[i as usize] >=
                       print_num {
                    fprintf(Outfp,
                            b"/\xe3\x80\x90\x00" as *const u8 as
                                *const libc::c_char);
                    if PrintFrequency != 0 {
                        fprintf(Outfp,
                                b"%s:%d\x00" as *const u8 as
                                    *const libc::c_char,
                                *(*(*cmm_ptr).cf_ptr).ex_list[i as
                                                                  usize].offset((*cmm_ptr).result_lists_p[k
                                                                                                              as
                                                                                                              usize].pos[i
                                                                                                                             as
                                                                                                                             usize]
                                                                                    as
                                                                                    isize),
                                *(*(*cmm_ptr).cf_ptr).ex_freq[i as
                                                                  usize].offset((*cmm_ptr).result_lists_p[k
                                                                                                              as
                                                                                                              usize].pos[i
                                                                                                                             as
                                                                                                                             usize]
                                                                                    as
                                                                                    isize));
                    } else {
                        fprintf(Outfp,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                *(*(*cmm_ptr).cf_ptr).ex_list[i as
                                                                  usize].offset((*cmm_ptr).result_lists_p[k
                                                                                                              as
                                                                                                              usize].pos[i
                                                                                                                             as
                                                                                                                             usize]
                                                                                    as
                                                                                    isize));
                    }
                    fprintf(Outfp,
                            b"\xe3\x80\x91\x00" as *const u8 as
                                *const libc::c_char);
                }
                if print_num != (*(*cmm_ptr).cf_ptr).ex_num[i as usize] {
                    fputs(b"...\x00" as *const u8 as *const libc::c_char,
                          Outfp);
                }
                fputc(')' as i32, Outfp);
            }
            if (*(*cmm_ptr).cf_ptr).oblig[i as usize] == 0 as libc::c_int {
                fputc('*' as i32, Outfp);
            }
            /* 意味素の出力 */
            if !(*(*cmm_ptr).cf_ptr).semantics[i as usize].is_null() {
                fprintf(Outfp,
                        b"[%s]\x00" as *const u8 as *const libc::c_char,
                        (*(*cmm_ptr).cf_ptr).semantics[i as usize]);
            }
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"<BR>\x00" as *const u8 as *const libc::c_char);
            }
            fputc('\n' as i32, Outfp);
            l += 1
        }
        k += 1
    }
    if OptExpress == 16 as libc::c_int {
        fprintf(Outfp, b"<BR>\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_good_crrspnds(mut cpm_ptr: *mut CF_PRED_MGR,
                                             mut cmm_ptr: *mut CF_MATCH_MGR,
                                             mut ipal_num: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* case_analysis では -1 の時がある */
    let mut j: libc::c_int = 0;
    let mut check: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut max_num: libc::c_int = 0;
    let mut max_score: libc::c_int = 0;
    let mut max_counts: libc::c_int = 0;
    let mut all_max_score: libc::c_int = 0 as libc::c_int;
    check =
        malloc_data((::std::mem::size_of::<libc::c_int>() as
                         libc::c_ulong).wrapping_mul(ipal_num as
                                                         libc::c_ulong),
                    b"print_good_crrspnds\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < ipal_num {
        *check.offset(i as isize) = 1 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < ipal_num {
        max_num = -(1 as libc::c_int);
        max_score = -(10 as libc::c_int);
        j = 0 as libc::c_int;
        while j < ipal_num {
            if *check.offset(j as isize) != 0 &&
                   (*cmm_ptr.offset(j as isize)).score >
                       max_score as libc::c_double {
                max_score =
                    (*cmm_ptr.offset(j as isize)).score as libc::c_int;
                max_num = j;
                max_counts = 1 as libc::c_int
            } else if *check.offset(j as isize) != 0 &&
                          (*cmm_ptr.offset(j as isize)).score ==
                              max_score as libc::c_double {
                max_counts += 1
            }
            j += 1
        }
        if i == 0 as libc::c_int { all_max_score = max_score }
        /* 表示の停止条件
	if (OptDisplay == OPT_NORMAL || OptDisplay == OPT_DETAIL) {
	    if (max_score != all_max_score && i >= 3) 
		break;
	}
	*/
        print_crrspnd(cpm_ptr, cmm_ptr.offset(max_num as isize));
        *check.offset(max_num as isize) = 0 as libc::c_int;
        i += 1
    }
    free(check as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_case_result(mut sp: *mut SENTENCE_DATA,
                                           mut Sen_Num: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tag_num: libc::c_int = 0;
    let mut tm: *mut TOTAL_MGR = (*sp).Best_mgr;
    if OptExpress != 16 as libc::c_int {
        fputs(b"<Case Structure Analysis Data>\n\x00" as *const u8 as
                  *const libc::c_char, Outfp);
        fprintf(Outfp,
                b"\xe2\x96\xa0 %d Score:%.3f, Dflt:%d, Possibility:%d/%d \xe2\x96\xa0\n\x00"
                    as *const u8 as *const libc::c_char, (*sp).Sen_num,
                (*tm).score, (*tm).dflt, (*tm).pssb + 1 as libc::c_int,
                1 as libc::c_int);
    }
    /* 上記出力の最後の引数(依存構造の数)は1にしている．
       ちゃんと扱ってない */
    i = (*tm).pred_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !(*tm).cpm[i as usize].pred_b_ptr.is_null() {
            if i != (*tm).pred_num - 1 as libc::c_int &&
                   OptExpress != 16 as libc::c_int {
                fputc('\n' as i32, Outfp);
            }
            tag_num = (*(*tm).cpm[i as usize].pred_b_ptr).num;
            if OptExpress == 16 as libc::c_int {
                fprintf(Outfp,
                        b"%%%% LABEL=%d_%dd\n\x00" as *const u8 as
                            *const libc::c_char, Sen_Num - 1 as libc::c_int,
                        tag_num + 1 as libc::c_int);
            }
            print_data_cframe(&mut *(*tm).cpm.as_mut_ptr().offset(i as isize),
                              &mut *(*(*tm).cpm.as_mut_ptr().offset(i as
                                                                        isize)).cmm.as_mut_ptr().offset(0
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            isize));
            j = 0 as libc::c_int;
            while j < (*tm).cpm[i as usize].result_num {
                if OptEllipsis != 0 {
                    print_crrspnd(if !(*tm).cpm[i as
                                                    usize].cmm[j as
                                                                   usize].cpm.is_null()
                                     {
                                      (*tm).cpm[i as
                                                    usize].cmm[j as usize].cpm
                                  } else {
                                      &mut *(*tm).cpm.as_mut_ptr().offset(i as
                                                                              isize)
                                  },
                                  &mut *(*(*tm).cpm.as_mut_ptr().offset(i as
                                                                            isize)).cmm.as_mut_ptr().offset(j
                                                                                                                as
                                                                                                                isize));
                    free((*tm).cpm[i as usize].cmm[j as usize].cpm as
                             *mut libc::c_void);
                } else {
                    print_crrspnd(&mut *(*tm).cpm.as_mut_ptr().offset(i as
                                                                          isize),
                                  &mut *(*(*tm).cpm.as_mut_ptr().offset(i as
                                                                            isize)).cmm.as_mut_ptr().offset(j
                                                                                                                as
                                                                                                                isize));
                }
                j += 1
            }
        }
        /* 述語ではないと判断したものはスキップ */
        i -= 1
    }
    if OptExpress != 16 as libc::c_int {
        fputs(b"</Case Structure Analysis Data>\n\x00" as *const u8 as
                  *const libc::c_char, Outfp);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_pa_structure(mut sp: *mut SENTENCE_DATA,
                                            mut eos_flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    let mut relation: [libc::c_char; 5120] = [0; 5120];
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 前から順番に、Predicate-Argument Structure を出力 */
    p = (*(*sp).Best_mgr).pred_num - 1 as libc::c_int;
    while p >= 0 as libc::c_int {
        cpm_ptr =
            &mut *(*(*sp).Best_mgr).cpm.as_mut_ptr().offset(p as isize) as
                *mut CF_PRED_MGR;
        fprintf(Outfp, b"%2d %s\x00" as *const u8 as *const libc::c_char,
                (*(*sp).Best_mgr).pred_num - 1 as libc::c_int - p,
                (*(*(*cpm_ptr).pred_b_ptr).head_ptr).Goi.as_mut_ptr());
        let mut current_block_14: u64;
        /* 入力側の各格要素の記述 */
        i = 0 as libc::c_int;
        while i < (*cpm_ptr).cf.element_num {
            /* 指示詞の解析をする場合は、指示詞を除く */
            if !(OptEllipsis & 2 as libc::c_int != 0 &&
                     !check_feature((*(*cpm_ptr).elem_b_ptr[i as usize]).f,
                                    b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe5\xaf\xbe\xe8\xb1\xa1\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char).is_null()) {
                num =
                    (*cpm_ptr).cmm[0 as libc::c_int as
                                       usize].result_lists_d[0 as libc::c_int
                                                                 as
                                                                 usize].flag[i
                                                                                 as
                                                                                 usize];
                /* 割り当てなし */
                if num == -(2 as libc::c_int) {
                    /* 割り当てなしだが、入力側の格が明示されている場合はそれを表示
		   (格の可能性はひとつしかなく、未格以外) */
		/* ★「へ」も扱いたい (現在は「へ/ニ」となっている) */
                    if (*cpm_ptr).cf.pp[i as usize][1 as libc::c_int as usize]
                           == -(10 as libc::c_int) &&
                           (*cpm_ptr).cf.pp[i as
                                                usize][0 as libc::c_int as
                                                           usize] >=
                               0 as libc::c_int {
                        strcpy(relation.as_mut_ptr(),
                               pp_code_to_kstr((*cpm_ptr).cf.pp[i as
                                                                    usize][0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]));
                        current_block_14 = 8457315219000651999;
                    } else { current_block_14 = 15619007995458559411; }
                } else {
                    /* 割り当てられている格 */
                    if num >= 0 as libc::c_int {
                        strcpy(relation.as_mut_ptr(),
                               pp_code_to_kstr((*(*cpm_ptr).cmm[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].cf_ptr).pp[num
                                                                                          as
                                                                                          usize][0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     usize]));
                    }
                    current_block_14 = 8457315219000651999;
                }
                match current_block_14 {
                    15619007995458559411 => { }
                    _ => {
                        word =
                            make_print_string((*cpm_ptr).elem_b_ptr[i as
                                                                        usize],
                                              0 as libc::c_int);
                        if !word.is_null() {
                            /* 省略の場合は * を付与 */
                            fprintf(Outfp,
                                    b" %s:%s%s\x00" as *const u8 as
                                        *const libc::c_char, word,
                                    relation.as_mut_ptr(),
                                    if (*cpm_ptr).elem_b_num[i as usize] <=
                                           -(2 as libc::c_int) {
                                        b"*\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"\x00" as *const u8 as
                                            *const libc::c_char
                                    });
                            free(word as *mut libc::c_void);
                        }
                    }
                }
            }
            i += 1
        }
        fputc('\n' as i32, Outfp);
        p -= 1
    }
    print_eos(eos_flag);
}
/*====================================================================
                               END
====================================================================*/
