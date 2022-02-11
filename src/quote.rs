#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
//! 鈎括弧の処理
use crate::{Chi_quote_end_matrix, Chi_quote_start_matrix, ctools, Quote_matrix, tools};
use crate::ctools::OptChiPos;
use crate::tools::{OptChiGenerative, OptDisplay};

#[no_mangle]
pub static mut quote_data: tools::QUOTE_DATA = tools::QUOTE_DATA{in_num: [0; 40], out_num: [0; 40],};

#[no_mangle]
pub unsafe extern "C" fn init_quote(mut sp: *mut tools::SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        quote_data.in_num[i as usize] = -(1 as libc::c_int);
        quote_data.out_num[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*Quote_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                1 as libc::c_int;
            (*Chi_quote_start_matrix.as_mut_ptr().offset(i as
                                                             isize))[j as
                                                                         usize]
                = -(1 as libc::c_int);
            (*Chi_quote_end_matrix.as_mut_ptr().offset(i as
                                                           isize))[j as usize]
                = -(1 as libc::c_int);
            j += 1
        }
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_quote() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while quote_data.in_num[i as usize] >= 0 as libc::c_int {
        fprintf(ctools::Outfp,
                b"Quote_num %d in %d out %d \n\x00" as *const u8 as
                    *const libc::c_char, i, quote_data.in_num[i as usize],
                quote_data.out_num[i as usize]);
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn check_quote(mut sp: *mut tools::SENTENCE_DATA) -> libc::c_int {
    /*
      "．．「○○」．．．「××」．．"  
      "．．「○○「×××」○○」．．" 
      "．．「○○○○○○○○○○○○"  
      "××××××××××××」．．"  などのパターンに対処
    */
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut stack: [libc::c_int; 40] = [0; 40];
    let mut s_num: libc::c_int = 0;
    let mut quote_p: libc::c_int = 0 as libc::c_int;
    k = 0 as libc::c_int;
    s_num = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            /* 最大数を越えないかチェック(最後の要素が番人なので、それを変えては
	       いけない) */
            if k >= 40 as libc::c_int - 1 as libc::c_int {
                fprintf(ctools::stderr,
                        b";; Too many quote (%s) ...\n\x00" as *const u8 as
                            *const libc::c_char,
                        if !(*sp).Comment.is_null() {
                            (*sp).Comment as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        });
                return -(1 as libc::c_int)
            }
            s_num += 1;
            stack[s_num as usize] = k;
            quote_data.in_num[k as usize] = i;
            k += 1;
            /* 「『‥ を扱うため上のことを繰り返す */
            if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\xef\xbc\x92\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                if k >= 40 as libc::c_int - 1 as libc::c_int {
                    fprintf(ctools::stderr,
                            b";; Too many quote (%s) ...\n\x00" as *const u8
                                as *const libc::c_char,
                            if !(*sp).Comment.is_null() {
                                (*sp).Comment as *const libc::c_char
                            } else {
                                b"\x00" as *const u8 as *const libc::c_char
                            }); /* 括弧終が多い場合 */
                    return -(1 as libc::c_int)
                }
                s_num += 1;
                stack[s_num as usize] = k;
                quote_data.in_num[k as usize] = i;
                k += 1
            }
        }
        if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if s_num == -(1 as libc::c_int) {
                if k >= 40 as libc::c_int - 1 as libc::c_int {
                    fprintf(ctools::stderr,
                            b";; Too many quote (%s) ...\n\x00" as *const u8
                                as *const libc::c_char,
                            if !(*sp).Comment.is_null() {
                                (*sp).Comment as *const libc::c_char
                            } else {
                                b"\x00" as *const u8 as *const libc::c_char
                            });
                    return -(1 as libc::c_int)
                }
                quote_data.out_num[k as usize] = i;
                k += 1
            } else {
                quote_data.out_num[stack[s_num as usize] as usize] = i;
                s_num -= 1
            }
            /* ‥』」 を扱うため上のことを繰り返す */
            if !ctools::check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\xef\xbc\x92\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                if s_num == -(1 as libc::c_int) {
                    if k >= 40 as libc::c_int - 1 as libc::c_int {
                        fprintf(ctools::stderr,
                                b";; Too many quote (%s) ...\n\x00" as
                                    *const u8 as *const libc::c_char,
                                if !(*sp).Comment.is_null() {
                                    (*sp).Comment as *const libc::c_char
                                } else {
                                    b"\x00" as *const u8 as
                                        *const libc::c_char
                                }); /* 括弧終が多い場合 */
                        return -(1 as libc::c_int)
                    }
                    quote_data.out_num[k as usize] = i;
                    k += 1
                } else {
                    quote_data.out_num[stack[s_num as usize] as usize] = i;
                    s_num -= 1
                }
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < k {
        /* 括弧が閉じていない場合は, 文頭または文末を境界に */
        if quote_data.in_num[i as usize] == -(1 as libc::c_int) {
            quote_data.in_num[i as usize] = 0 as libc::c_int
        }
        if quote_data.out_num[i as usize] == -(1 as libc::c_int) {
            quote_data.out_num[i as usize] = (*sp).Bnst_num - 1 as libc::c_int
        }
        /* 一文節の括弧を考慮しない場合
	if (quote_data.in_num[i] != quote_data.out_num[i])
	quote_p = TRUE;
	*/
        quote_p = (0 as libc::c_int == 0) as libc::c_int;
        i += 1
    }
    return quote_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mask_quote(mut sp: *mut tools::SENTENCE_DATA)
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* １文節だけの括弧は無視 */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    k = 0 as libc::c_int;
    while quote_data.in_num[k as usize] >= 0 as libc::c_int {
        start = quote_data.in_num[k as usize];
        end = quote_data.out_num[k as usize];
        if !(start == end) {
            /* 括弧の上のマスク */
            i = 0 as libc::c_int;
            while i < start {
                j = start;
                while j < end {
                    (*Quote_matrix.as_mut_ptr().offset(i as
                                                           isize))[j as usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                /* 
	       括弧内末尾の文節には連格,連体,ノ格,同格連体,括弧並列のみ
	       係れるとする．
	       		例) 「私の「本当の気持ち」は…」

	       用言に連用が係ることも稀にはあるが，それを許すと通常の場合の
	       解析誤りが大量に生まれるので無視する．
	       		例) 「彼が「東京にいった」ことは…」
	    */
                if !((*Quote_matrix.as_mut_ptr().offset(i as
                                                            isize))[end as
                                                                        usize]
                         != 0 &&
                         (!ctools::check_feature((*(*sp).bnst_data.offset(i as
                                                                      isize)).f,
                                         b"\xe4\xbf\x82:\xe9\x80\xa3\xe6\xa0\xbc\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() ||
                              !ctools::check_feature((*(*sp).bnst_data.offset(i as
                                                                          isize)).f,
                                             b"\xe4\xbf\x82:\xe9\x80\xa3\xe4\xbd\x93\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                              ||
                              !ctools::check_feature((*(*sp).bnst_data.offset(i as
                                                                          isize)).f,
                                             b"\xe4\xbf\x82:\xe3\x83\x8e\xe6\xa0\xbc\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                              ||
                              !ctools::check_feature((*(*sp).bnst_data.offset(i as
                                                                          isize)).f,
                                             b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe9\x80\xa3\xe4\xbd\x93\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                              ||
                              !ctools::check_feature((*(*sp).bnst_data.offset(i as
                                                                          isize)).f,
                                             b"\xe4\xbf\x82:\xe6\x8b\xac\xe5\xbc\xa7\xe4\xb8\xa6\xe5\x88\x97\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()))
                   {
                    (*Quote_matrix.as_mut_ptr().offset(i as
                                                           isize))[end as
                                                                       usize]
                        = 0 as libc::c_int
                }
                i += 1
            }
            /* 括弧の右のマスク */
            i = start;
            while i < end {
                j = end + 1 as libc::c_int;
                while j < (*sp).Bnst_num {
                    (*Quote_matrix.as_mut_ptr().offset(i as
                                                           isize))[j as usize]
                        = 0 as libc::c_int;
                    j += 1
                }
                i += 1
            }
            /* 括弧内の句点の右上のマスク 
	   (句点の右は開けておく --> 次の文末とPになる) */
            l = start;
            while l < end {
                if !ctools::check_feature((*(*sp).bnst_data.offset(l as isize)).f,
                                  b"\xe4\xbf\x82:\xe6\x96\x87\xe6\x9c\xab\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    i = start;
                    while i < l {
                        j = l + 1 as libc::c_int;
                        while j <= end {
                            (*Quote_matrix.as_mut_ptr().offset(i as
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
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn mask_quote_for_chi(mut sp: *mut tools::SENTENCE_DATA)
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* １文節だけの括弧は無視 */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    k = 0 as libc::c_int;
    while quote_data.in_num[k as usize] >= 0 as libc::c_int {
        start = quote_data.in_num[k as usize];
        end = quote_data.out_num[k as usize];
        if !(start == end) {
            if ctools::OptChiPos == 0 &&
                   !ctools::check_feature((*(*sp).bnst_data.offset(start as isize)).f,
                                  b"PU\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                   !ctools::check_feature((*(*sp).bnst_data.offset(end as isize)).f,
                                  b"PU\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                   OptChiPos != 0 {
                /* 括弧の上のマスク */
                i = 0 as libc::c_int;
                while i < start {
                    (*Quote_matrix.as_mut_ptr().offset(i as
                                                           isize))[start as
                                                                       usize]
                        = 0 as libc::c_int;
                    (*Quote_matrix.as_mut_ptr().offset(i as
                                                           isize))[end as
                                                                       usize]
                        = 0 as libc::c_int;
                    i += 1
                }
                /* 括弧の右のマスク */
                i = end + 1 as libc::c_int;
                while i < (*sp).Bnst_num {
                    (*Quote_matrix.as_mut_ptr().offset(start as
                                                           isize))[i as usize]
                        = 0 as libc::c_int;
                    (*Quote_matrix.as_mut_ptr().offset(end as
                                                           isize))[i as usize]
                        = 0 as libc::c_int;
                    i += 1
                }
                (*Quote_matrix.as_mut_ptr().offset(start as
                                                       isize))[end as usize] =
                    0 as libc::c_int;
                j = start;
                while j <= end {
                    i = j;
                    while i <= end {
                        (*Chi_quote_start_matrix.as_mut_ptr().offset(j as isize))[i as usize] = start;
                        (*Chi_quote_end_matrix.as_mut_ptr().offset(j as isize))[i as usize] = end;
                        i += 1
                    }
                    j += 1
                }
                (*Chi_quote_start_matrix.as_mut_ptr().offset(start as isize))[end as usize]  = -(1 as libc::c_int);
                (*Chi_quote_end_matrix.as_mut_ptr().offset(start as isize))[end as usize]  = -(1 as libc::c_int)
            }
        }
        k += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn quote(mut sp: *mut tools::SENTENCE_DATA) -> libc::c_int {
    let mut quote_p: libc::c_int = 0 as libc::c_int;
    init_quote(sp);
    if ctools::Language != 2 as libc::c_int ||
        ctools::Language == 2 as libc::c_int && OptChiGenerative == 0 {
        quote_p = check_quote(sp);
        if quote_p != 0 {
            /* 鈎括弧の検出 */
            if quote_p == -(1 as libc::c_int) { return quote_p }
            if OptDisplay == 3 as libc::c_int && ctools::Language != 2 as libc::c_int {
                print_quote();
            }
            if ctool::Language != 2 as libc::c_int {
                mask_quote(sp);
                /* 行列の書き換え */
            } else {
                mask_quote_for_chi(sp);
                // mask quote for Chinese
            }
        }
    }
    return quote_p;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn add_comment(mut sp: *mut tools::SENTENCE_DATA, mut add_string: *mut libc::c_char) {
    if !(*sp).Comment.is_null() {
        /* 既存のコメントと結合 */
        let mut orig_comment: *mut libc::c_char = (*sp).Comment;
        (*sp).Comment =
            ctools::malloc_data(
                strlen((*sp).Comment).wrapping_add(strlen(add_string)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                b"add_comment\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) as *mut libc::c_char;
        sprintf((*sp).Comment, b"%s %s\x00" as *const u8 as *const libc::c_char, orig_comment, add_string);
        free(orig_comment as *mut libc::c_void);
    } else {
        /* 新たなコメント */
        (*sp).Comment = strdup(add_string)
    };
}

#[no_mangle]
pub unsafe extern "C" fn process_input_paren(mut sp: *mut tools::SENTENCE_DATA, mut paren_spp: *mut *mut tools::SENTENCE_DATA) -> libc::c_int {
    let mut i: libc::c_int = 0; /* initialization */
    let mut j: libc::c_int = 0;
    let mut paren_mrph_num: libc::c_int = 0 as libc::c_int;
    let mut paren_level: libc::c_int = 0 as libc::c_int;
    let mut paren_start: libc::c_int = 0;
    let mut paren_table: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut paren_num: libc::c_int = 0 as libc::c_int;
    let mut m_ptr: *mut tools::MRPH_DATA = (*sp).mrph_data;
    // let mut next_sentence_data: tools::SENTENCE_DATA = tools::SENTENCE_DATA{
    //     Sen_num: 0,
    //     available: 0,
    //     Mrph_num: 0,
    //     New_Mrph_num: 0,
    //     Bnst_num: 0,
    //     New_Bnst_num: 0,
    //     Max_New_Bnst_num: 0,
    //     Tag_num: 0,
    //     New_Tag_num: 0,
    //     Para_M_num: 0,
    //     Para_num: 0,
    //     frame_num_max: 0,
    //     mrph_data: 0 as *mut tools::MRPH_DATA,
    //     bnst_data: 0 as *mut tools::BNST_DATA,
    //     tag_data: 0 as *mut tools::TAG_DATA,
    //     para_data: 0 as *mut tools::PARA_DATA,
    //     para_manager: 0 as *mut tools::PARA_MANAGER,
    //     cpm: 0 as *mut tools::CF_PRED_MGR,
    //     cf: 0 as *mut tools::CASE_FRAME,
    //     Best_mgr: 0 as *mut tools::TOTAL_MGR,
    //     KNPSID: 0 as *mut libc::c_char,
    //     Comment: 0 as *mut libc::c_char,
    //     score: 0.,
    // };
    paren_table =
        ctools::malloc_data((::std::mem::size_of::<libc::c_int>() as
                         libc::c_ulong).wrapping_mul((*sp).Mrph_num as
                                                         libc::c_ulong),
                    b"process_input_paren\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_int;
    memset(paren_table as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul((*sp).Mrph_num as libc::c_ulong));
    /* 括弧チェック */
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        if strcmp((*m_ptr.offset(i as isize)).Goi.as_mut_ptr(),
                  b"\xef\xbc\x88\x00" as *const u8 as *const libc::c_char) ==
               0 {
            /* beginning of parenthesis */
            if paren_level == 0 as libc::c_int { paren_start = i }
            paren_level += 1
        } else if strcmp((*m_ptr.offset(i as isize)).Goi.as_mut_ptr(),
                         b"\xef\xbc\x89\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            /* end of parenthesis */
            paren_level -= 1;
            if paren_level == 0 as libc::c_int &&
                   i != paren_start + 1 as libc::c_int {
                /* （）のような中身がない場合は除く */
                /* 数詞は対象外にする? */
                *paren_table.offset(paren_start as isize) =
                    'B' as i32; /* beginning */
                /* 括弧数 */
                *paren_table.offset(i as isize) = 'E' as i32; /* end */
                paren_mrph_num += 2 as libc::c_int; /* intermediate */
                j = paren_start + 1 as libc::c_int;
                while j < i {
                    *paren_table.offset(j as isize) = 'I' as i32;
                    paren_mrph_num += 1;
                    j += 1
                    /* 括弧部分の形態素数 */
                }
                paren_num += 1
            }
        }
        i += 1
    }
    return if paren_num == 0 as libc::c_int || paren_num >= 100 as libc::c_int ||
        *paren_table == 'B' as i32 &&
            *paren_table.offset((*sp).Mrph_num as
                isize).offset(-(1 as libc::c_int as
                isize)) ==
                'E' as i32 {
        /* 全体が括弧の時は対象外 */
        0 as libc::c_int
    } else {
        let mut paren_count: libc::c_int = -(1 as libc::c_int);
        let mut char_pos: libc::c_int = 0 as libc::c_int;
        *paren_spp =
            ctools::malloc_data((::std::mem::size_of::<tools::SENTENCE_DATA>() as
                libc::c_ulong).wrapping_mul(paren_num as
                libc::c_ulong),
                        b"process_input_paren\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut tools::SENTENCE_DATA;
        /* 各括弧文 */
        i = 0 as libc::c_int; /* 括弧文のIDは-02から */
        while i < paren_num {
            (*(*paren_spp).offset(i as isize)).Mrph_num =
                0 as libc::c_int; /* 本文のIDに-01をつける */
            let ref mut fresh0 =
                (*(*paren_spp).offset(i as
                    isize)).mrph_data; /* 本文のコメント行に */
            *fresh0 =
                ctools::malloc_data((::std::mem::size_of::<tools::MRPH_DATA>() as
                    libc::c_ulong).wrapping_mul(paren_mrph_num as
                    libc::c_ulong),
                            b"process_input_paren\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut tools::MRPH_DATA;
            let ref mut fresh1 = (*(*paren_spp).offset(i as isize)).KNPSID;
            *fresh1 =
                ctools::malloc_data(strlen((*sp).KNPSID).wrapping_add(4 as libc::c_int
                    as
                    libc::c_ulong),
                            b"process_input_paren\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            sprintf((*(*paren_spp).offset(i as isize)).KNPSID,
                    b"%s-%02d\x00" as *const u8 as *const libc::c_char,
                    (*sp).KNPSID, i + 2 as libc::c_int);
            let ref mut fresh2 = (*(*paren_spp).offset(i as isize)).Comment;
            *fresh2 =
                ctools::malloc_data(strlen(b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b:\xef\xbc\x88 \xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82:\xef\xbc\x89 \xe6\x8b\xac\xe5\xbc\xa7\xe4\xbd\x8d\xe7\xbd\xae:\x00"
                    as *const u8 as
                    *const libc::c_char).wrapping_add(4 as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"process_input_paren\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            sprintf((*(*paren_spp).offset(i as isize)).Comment,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b:\xef\xbc\x88 \xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82:\xef\xbc\x89 \xe6\x8b\xac\xe5\xbc\xa7\xe4\xbd\x8d\xe7\xbd\xae:\x00"
                        as *const u8 as *const libc::c_char);
            i += 1
        }
        strcat((*sp).KNPSID, b"-01\x00" as *const u8 as *const libc::c_char);
        add_comment(sp,
                    b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\x89\x8a\xe9\x99\xa4\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
        /* 本文と括弧文を分離 */
        j = 0 as libc::c_int;
        i = j;
        while i < (*sp).Mrph_num {
            if *paren_table.offset(i as isize) == 0 as libc::c_int {
                /* 括弧ではない部分 */
                if i != j {
                    *m_ptr.offset(j as isize) = *m_ptr.offset(i as isize);
                    (*m_ptr.offset(j as isize)).num = j;
                    let ref mut fresh3 = (*m_ptr.offset(i as isize)).f;
                    *fresh3 = 0 as tools::FEATUREptr
                }
                j += 1
            } else {
                /* 括弧部分 */
                if *paren_table.offset(i as isize) == 'B' as i32 {
                    /* 括弧始 */
                    paren_count += 1;
                    sprintf((*(*paren_spp).offset(paren_count as
                        isize)).Comment,
                            b"%s%d\x00" as *const u8 as *const libc::c_char,
                            (*(*paren_spp).offset(paren_count as
                                isize)).Comment,
                            char_pos);
                }
                if *paren_table.offset(i as isize) == 'I' as i32 {
                    /* 括弧内部 */
                    *(*(*paren_spp).offset(paren_count as
                        isize)).mrph_data.offset((*(*paren_spp).offset(paren_count
                        as
                        isize)).Mrph_num
                        as
                        isize)
                        = *m_ptr.offset(i as isize);
                    (*(*(*paren_spp).offset(paren_count as
                        isize)).mrph_data.offset((*(*paren_spp).offset(paren_count
                        as
                        isize)).Mrph_num
                        as
                        isize)).num
                        =
                        (*(*paren_spp).offset(paren_count as isize)).Mrph_num;
                    let ref mut fresh4 =
                        (*(*paren_spp).offset(paren_count as isize)).Mrph_num;
                    *fresh4 += 1;
                    let ref mut fresh5 = (*m_ptr.offset(i as isize)).f;
                    *fresh5 = 0 as tools::FEATUREptr
                }
            }
            char_pos +=
                ctools::string_length((*m_ptr.offset(i as isize)).Goi2.as_mut_ptr());
            i += 1
        }
        (*sp).Mrph_num -= paren_mrph_num;
        free(paren_table as *mut libc::c_void);
        paren_num
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn prepare_paren_sentence(mut sp: *mut tools::SENTENCE_DATA,
                                                mut paren_sp:
                                                    *mut tools::SENTENCE_DATA)
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    (*sp).KNPSID = (*paren_sp).KNPSID;
    (*sp).Comment = (*paren_sp).Comment;
    (*sp).Mrph_num = (*paren_sp).Mrph_num;
    i = 0 as libc::c_int;
    while i < (*paren_sp).Mrph_num {
        *(*sp).mrph_data.offset(i as isize) =
            *(*paren_sp).mrph_data.offset(i as isize);
        i += 1
    };
}
/*====================================================================
                               END
====================================================================*/
