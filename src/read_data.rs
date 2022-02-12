#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//! 形態素解析列の読み込み，文節へのまとめ
use libc;

use crate::{Class, ErrorComment, fflush, fgets, fprintf, free, memset, PM_Memo, regexp, sentence_data, sprintf, sscanf, strcat, strchr, strcmp, strcpy, strdup, strlen, strncmp, strstr};
use crate::anaphora::clear_context;
use crate::case_analysis::{pp_code_to_kstr, pp_kstr_to_code};
use crate::context::ClearSentences;
use crate::ctools::{assign_cfeature, check_feature, exit, fgetc, Form, Language, log, malloc_data, OptAnalysis, OptChiPos, Outfp, stderr, string_length, strncat, strncpy, strrchr, strtok};
use crate::feature::{append_feature, assign_feature, clear_feature, copy_feature, delete_alt_feature, delete_cfeature, delete_cfeature_from_mrphs};
use crate::juman::getid::{get_bunrui_id, get_form_id, get_hinsi_id, get_type_id};
use crate::lib_sm::sm2feature;
use crate::proper::clear_ne_cache;
use crate::read_rule::{CurHomoRuleSize, GeneralRuleArray, GeneralRuleNum, HomoRuleArray};
use crate::regexp::{regexpbnstrule_match, regexpmrph_match, regexpmrphrule_match, regexptagrule_match};
use crate::structs::{_FEATURE, BnstRule, HomoRule, MRPH_DATA, MrphRule, sentence, tnode_b};
use crate::thesaurus::get_bnst_code_all;
use crate::tools::{ArticleID, Bnst_start, Chi_word_type, Input_bnst_feature, Input_tag_feature, OptAnaphora, OptDisplay, OptEllipsis, OptIgnoreChar, OptInput, OptKatakanaNormalize, OptMode, OptNE, OptNElearn, OptReadFeature, preArticleID, realloc_data, Tag_dpnd, Tag_start, Tag_type, total_sen_num};
use crate::types::{BNST_DATA, CF_PRED_MGR, CPM_ptr, FEATURE, FEATUREptr, FILE, SENTENCE_DATA, size_t, TAG_DATA};

/// 4文字以上でカタカナ末尾の長音符を吸収
#[no_mangle]
pub unsafe extern "C" fn selected_imi2feature(mut str: *mut libc::c_char, mut m_ptr: *mut MRPH_DATA) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut imip: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(str, b"NIL\x00" as *const u8 as *const libc::c_char) == 0 {
        return;
    }
    buf = strdup(str);
    /* 通常 "" で括られている */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\"' as i32 {
        imip =
            &mut *buf.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        cp = strchr(imip, '\"' as i32);
        if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
    } else { imip = buf }
    token = strtok(imip, b" \x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        /* 以下のもの以外を付与 */
        if strncmp(token,
                   b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00" as
                       *const u8 as *const libc::c_char,
                   strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                       as *const u8 as *const libc::c_char)) != 0 &&
            strncmp(token,
                    b"\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8b\x95\xe8\xa9\x9e\x00"
                        as *const u8 as *const libc::c_char,
                    strlen(b"\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8b\x95\xe8\xa9\x9e\x00"
                        as *const u8 as *const libc::c_char)) != 0
            &&
            strncmp(token,
                    b"\xe6\xbc\xa2\xe5\xad\x97\xe8\xaa\xad\xe3\x81\xbf\x00"
                        as *const u8 as *const libc::c_char,
                    strlen(b"\xe6\xbc\xa2\xe5\xad\x97\xe8\xaa\xad\xe3\x81\xbf\x00"
                        as *const u8 as *const libc::c_char)) != 0
            &&
            strncmp(token,
                    b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                        as *const u8 as *const libc::c_char,
                    strlen(b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00"
                        as *const u8 as *const libc::c_char)) != 0
            &&
            strncmp(token,
                    b"\xe3\x83\x89\xe3\x83\xa1\xe3\x82\xa4\xe3\x83\xb3\x00"
                        as *const u8 as *const libc::c_char,
                    strlen(b"\xe3\x83\x89\xe3\x83\xa1\xe3\x82\xa4\xe3\x83\xb3\x00"
                        as *const u8 as *const libc::c_char)) != 0 {
            assign_cfeature(&mut (*m_ptr).f, token, 0 as libc::c_int);
        }
        token =
            strtok(0 as *mut libc::c_char,
                   b" \x00" as *const u8 as *const libc::c_char)
    }
    free(buf as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn assign_feature_alt_mrph(mut fpp: *mut *mut FEATURE, mut m_ptr: *mut MRPH_DATA) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = malloc_data(
        strlen(
            (*m_ptr).Goi2.as_mut_ptr()
        ).wrapping_add(
            strlen((*m_ptr).Yomi.as_mut_ptr())
        ).wrapping_add(
            strlen((*m_ptr).Goi.as_mut_ptr())
        ).wrapping_add(
            strlen((*m_ptr).Imi.as_mut_ptr())
        ).wrapping_add(
            (10 as libc::c_int * 3 as libc::c_int) as libc::c_ulong
        ),
        b"assign_feature_alt_mrph\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char;
    sprintf(buf,
            b"ALT-%s-%s-%s-%d-%d-%d-%d-%s\x00" as *const u8 as *const libc::c_char,
            (*m_ptr).Goi2.as_mut_ptr(),
            (*m_ptr).Yomi.as_mut_ptr(),
            (*m_ptr).Goi.as_mut_ptr(),
            (*m_ptr).Hinshi,
            (*m_ptr).Bunrui,
            (*m_ptr).Katuyou_Kata,
            (*m_ptr).Katuyou_Kei,
            (*m_ptr).Imi.as_mut_ptr(),
    );
    assign_cfeature(fpp, buf, 0 as libc::c_int);
    free(buf as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn get_mrph_rep(mut m_ptr: *mut MRPH_DATA) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strstr(
        (*m_ptr).Imi.as_mut_ptr(),
        b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as *const u8 as *const libc::c_char,
    );
    if !cp.is_null() {
        return cp.offset(
            strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as *const u8 as *const libc::c_char) as isize
        );
    }
    return 0 as *mut libc::c_char;
}

/// flagが立っていてかつ、代表表記が変更されている場合は変更前の代表表記を返す
#[no_mangle]
pub unsafe extern "C" fn get_mrph_rep_from_f(mut m_ptr: *mut MRPH_DATA, mut flag: libc::c_int) -> *mut libc::c_char
{
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if flag != 0 && {
        cp = check_feature(
            (*m_ptr).f,
            b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        !cp.is_null()
    } {
        return cp.offset(
            strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char) as isize
        );
    }
    cp = check_feature(
        (*m_ptr).f,
        b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !cp.is_null() {
        return cp.offset(
            strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as *const u8 as *const libc::c_char) as isize
        );
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn get_mrph_rep_length(mut rep_strt: *mut libc::c_char) -> libc::c_int {
    let mut rep_end: *mut libc::c_char = 0 as *mut libc::c_char;
    if rep_strt.is_null() {
        return 0 as libc::c_int;
    }
    rep_end = strchr(rep_strt, ' ' as i32);
    if rep_end.is_null() {
        rep_end = strchr(rep_strt, '\"' as i32)
    }
    return rep_end.wrapping_offset_from(rep_strt) as libc::c_long as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_bnst_head_canonical_rep(mut ptr: *mut BNST_DATA, mut compound_flag: libc::c_int) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if compound_flag != 0 {
        /* 主辞+α */
        cp = check_feature((*ptr).f,
                           b"\xe4\xb8\xbb\xe8\xbe\x9e\xe2\x80\x99\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if !cp.is_null() {
            return cp.offset(strlen(b"\xe4\xb8\xbb\xe8\xbe\x9e\xe2\x80\x99\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                as *const u8 as *const libc::c_char)
                as isize);
        }
    }
    cp = check_feature((*ptr).f,
                       b"\xe4\xb8\xbb\xe8\xbe\x9e\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    return if !cp.is_null() {
        cp.offset(strlen(b"\xe4\xb8\xbb\xe8\xbe\x9e\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
            as *const u8 as *const libc::c_char) as
            isize)
    } else { 0 as *mut libc::c_char };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_rep_f_from_imi(mut m_ptr: *mut MRPH_DATA)
                                               -> libc::c_int
/*==================================================================*/
{
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut length: libc::c_int = 0;
    cp =
        strstr((*m_ptr).Imi.as_mut_ptr(),
               b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as
                   *const u8 as *const libc::c_char);
    if !cp.is_null() {
        length = get_mrph_rep_length(cp);
        strncpy(buf.as_mut_ptr(), cp, length as libc::c_ulong);
        buf[length as usize] = '\u{0}' as i32 as libc::c_char;
        assign_cfeature(&mut (*m_ptr).f, buf.as_mut_ptr(), 0 as libc::c_int);
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_mrph_rn(mut m_ptr: *mut MRPH_DATA)
                                      -> *mut libc::c_char
/*==================================================================*/
{
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut goi_length: libc::c_int =
        strlen((*m_ptr).Goi.as_mut_ptr()) as libc::c_int;
    let mut yomi_length: libc::c_int =
        strlen((*m_ptr).Yomi.as_mut_ptr()) as libc::c_int;
    /* (代表表記がないときに)代表表記を作る */
    /* 基本形の方が長いかもしれないので余分に確保 */
    buf =
        malloc_data((goi_length + yomi_length + 128 as libc::c_int) as size_t,
                    b"make_mrph_rn\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut libc::c_char;
    /* 指定された文字以上で長音符で終わるカタカナの代表表記は長音符を削除 */
    if OptKatakanaNormalize != 0 &&
        (strcmp(Class[(*m_ptr).Hinshi as
            usize][0 as libc::c_int as usize].id as
                    *const libc::c_char,
                b"\xe6\x9c\xaa\xe5\xae\x9a\xe7\xbe\xa9\xe8\xaa\x9e\x00" as
                    *const u8 as *const libc::c_char) == 0 &&
            strcmp(Class[(*m_ptr).Hinshi as
                usize][(*m_ptr).Bunrui as usize].id as
                       *const libc::c_char,
                   b"\xe3\x82\xab\xe3\x82\xbf\xe3\x82\xab\xe3\x83\x8a\x00"
                       as *const u8 as *const libc::c_char) == 0 ||
            !strstr((*m_ptr).Imi.as_mut_ptr(),
                    b"\xe8\x87\xaa\xe5\x8b\x95\xe7\x8d\xb2\xe5\xbe\x97\x00"
                        as *const u8 as *const libc::c_char).is_null()) &&
        goi_length >= 4 as libc::c_int * 3 as libc::c_int &&
        strcmp((*m_ptr).Goi.as_mut_ptr().offset(goi_length as
            isize).offset(-(3 as
            libc::c_int
            as
            isize)),
               b"\xe3\x83\xbc\x00" as *const u8 as *const libc::c_char) ==
            0 &&
        strcmp((*m_ptr).Goi.as_mut_ptr().offset(goi_length as
            isize).offset(-(3 as
            libc::c_int
            as
            isize)).offset(-(3
            as
            libc::c_int
            as
            isize)),
               b"\xe3\x83\xbc\xe3\x83\xbc\x00" as *const u8 as
                   *const libc::c_char) != 0 {
        /* 長音符は一つだけ */
        sprintf(buf, b"%.*s/%.*s\x00" as *const u8 as *const libc::c_char,
                goi_length - 3 as libc::c_int, (*m_ptr).Goi.as_mut_ptr(),
                yomi_length - 3 as libc::c_int, (*m_ptr).Yomi.as_mut_ptr());
    } else {
        sprintf(buf, b"%s/%s\x00" as *const u8 as *const libc::c_char,
                (*m_ptr).Goi.as_mut_ptr(), (*m_ptr).Yomi.as_mut_ptr());
    }
    if (*m_ptr).Katuyou_Kata > 0 as libc::c_int &&
        (*m_ptr).Katuyou_Kei > 0 as libc::c_int {
        /* 活用語 */
        if (*m_ptr).Katuyou_Kata < 128 as libc::c_int &&
            (*m_ptr).Katuyou_Kei < 128 as libc::c_int &&
            !Form[(*m_ptr).Katuyou_Kata as
                usize][(*m_ptr).Katuyou_Kei as usize].gobi.is_null()
        {
            if *Form[(*m_ptr).Katuyou_Kata as
                usize][(*m_ptr).Katuyou_Kei as
                usize].gobi.offset(0 as libc::c_int as
                isize) as
                libc::c_int == '-' as i32 {
                /* エ基本形: 語幹が得られない */
                /* 原型の読みが不明なので、両方とも原型にしておく */
                sprintf(buf, b"%s/%s\x00" as *const u8 as *const libc::c_char,
                        (*m_ptr).Goi.as_mut_ptr(),
                        (*m_ptr).Goi.as_mut_ptr()); /* 読みを語幹にする */
            } else {
                *buf.offset(strlen(buf).wrapping_sub(strlen(Form[(*m_ptr).Katuyou_Kata
                    as
                    usize][(*m_ptr).Katuyou_Kei
                    as
                    usize].gobi
                    as
                    *const libc::c_char))
                    as isize) = '\u{0}' as i32 as libc::c_char;
                strcat(buf,
                       Form[
                           (*m_ptr).Katuyou_Kata as usize
                           ][
                           get_form_id(b"\xe5\x9f\xba\xe6\x9c\xac\xe5\xbd\xa2\x00" as *const u8 as *mut libc::c_uchar, (*m_ptr).Katuyou_Kata) as usize
                           ].gobi as *const libc::c_char);
                /* 基本形をつける */
            }
        } else {
            fprintf(stderr,
                    b";; Invalid morpheme ID: kata(%d) kei(%d)\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*m_ptr).Katuyou_Kata, (*m_ptr).Katuyou_Kei);
        }
    }
    return buf;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn rn2canonical_rn(mut m_ptr: *mut MRPH_DATA)
/*==================================================================*/
{
    let mut rn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 代表表記をそのまま正規化代表表記に */
    rn = get_mrph_rep_from_f(m_ptr, 0 as libc::c_int);
    if !rn.is_null() {
        buf =
            malloc_data(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                as *const u8 as
                *const libc::c_char).wrapping_add(strlen(rn)).wrapping_add(1
                as
                libc::c_int
                as
                libc::c_ulong),
                        b"rn2canonical_rn\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut libc::c_char;
        strcpy(buf,
               b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                   as *const u8 as *const libc::c_char);
        strcat(buf, rn);
        assign_cfeature(&mut (*m_ptr).f, buf, 0 as libc::c_int);
        free(buf as *mut libc::c_void);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_cc_feature_to_bp(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut merged_rep_size: libc::c_int = 5120 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merged_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    /* <内容語>形態素の正規化代表表記から、基本句の正規化代表表記を作成 */
    merged_rep =
        malloc_data(merged_rep_size as size_t,
                    b"assign_cc_feature_to_bp\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        /* すべての基本句に付与 */
        *merged_rep = '\u{0}' as i32 as libc::c_char;
        j = 0 as libc::c_int;
        while j < (*(*sp).tag_data.offset(i as isize)).mrph_num {
            if (!check_feature((*(*(*sp).tag_data.offset(i as
                isize)).mrph_ptr.offset(j
                as
                isize)).f,
                               b"\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*sp).tag_data.offset(i as
                    isize)).mrph_ptr.offset(j
                    as
                    isize)).f,
                               b"\xe6\xba\x96\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) &&
                check_feature((*(*(*sp).tag_data.offset(i as
                    isize)).mrph_ptr.offset(j
                    as
                    isize)).f,
                              b"\xe7\x89\xb9\xe6\xae\x8a\xe9\x9d\x9e\xe8\xa6\x8b\xe5\x87\xba\xe8\xaa\x9e\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                {
                    cp =
                        check_feature((*(*(*sp).tag_data.offset(i as
                            isize)).mrph_ptr.offset(j
                            as
                            isize)).f,
                                      b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                          as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                    !cp.is_null()
                } {
                if *merged_rep != 0 {
                    if strlen(merged_rep).wrapping_add(strlen(cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                        as
                        *const u8
                        as
                        *const libc::c_char)
                        as
                        isize))).wrapping_add(2
                        as
                        libc::c_int
                        as
                        libc::c_ulong)
                        > merged_rep_size as libc::c_ulong {
                        merged_rep_size *= 2 as libc::c_int;
                        merged_rep =
                            realloc_data(merged_rep as *mut libc::c_void,
                                         merged_rep_size as size_t,
                                         b"assign_cc_feature_to_bp\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char) as
                                *mut libc::c_char
                    }
                    strcat(merged_rep,
                           b"+\x00" as *const u8 as *const libc::c_char);
                    strcat(merged_rep,
                           cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                               as *const u8 as
                               *const libc::c_char) as
                               isize));
                } else { strcpy(merged_rep, cp); }
            }
            j += 1
        }
        if *merged_rep != 0 {
            assign_cfeature(&mut (*(*sp).tag_data.offset(i as isize)).f,
                            merged_rep, 0 as libc::c_int);
            /* 連結代表表記 */
        }
        i += 1
    }
    free(merged_rep as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_cc_feature_to_bnst(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut merged_rep_size: libc::c_int = 5120 as libc::c_int;
    let mut error_flag: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merged_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 基本句の正規化代表表記から文節の正規化代表表記を作成 */
    merged_rep =
        malloc_data(merged_rep_size as size_t,
                    b"assign_cc_feature_to_bnst\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        /* すべての文節に付与 */
        *merged_rep = '\u{0}' as i32 as libc::c_char;
        last_rep = 0 as *mut libc::c_char;
        j = 0 as libc::c_int;
        while j < (*(*sp).bnst_data.offset(i as isize)).tag_num {
            cp =
                check_feature((*(*(*sp).bnst_data.offset(i as
                    isize)).tag_ptr.offset(j
                    as
                    isize)).f,
                              b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                if *merged_rep != 0 {
                    if strlen(merged_rep).wrapping_add(strlen(cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                        as
                        *const u8
                        as
                        *const libc::c_char)
                        as
                        isize))).wrapping_add(2
                        as
                        libc::c_int
                        as
                        libc::c_ulong)
                        > merged_rep_size as libc::c_ulong {
                        merged_rep_size *= 2 as libc::c_int;
                        merged_rep =
                            realloc_data(merged_rep as *mut libc::c_void,
                                         merged_rep_size as size_t,
                                         b"assign_cc_feature_to_bnst\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char) as
                                *mut libc::c_char
                    }
                    strcat(merged_rep,
                           b"+\x00" as *const u8 as *const libc::c_char);
                    strcat(merged_rep,
                           cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                               as *const u8 as
                               *const libc::c_char) as
                               isize));
                } else { strcpy(merged_rep, cp); }
                last_rep = cp
            }
            j += 1
        }
        if *merged_rep != 0 {
            assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            merged_rep, 0 as libc::c_int);
            /* 連結した代表表記 */
        } /* 主辞代表表記 */
        if !last_rep.is_null() {
            strncpy(last_rep.offset(strlen(b"\xe6\xad\xa3\x00" as *const u8 as
                *const libc::c_char) as isize),
                    b"\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as
                        *const libc::c_char,
                    strlen(b"\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as
                        *const libc::c_char)); /* 末尾の基本句にも付与 */
            assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                            last_rep.offset(strlen(b"\xe6\xad\xa3\x00" as
                                *const u8 as
                                *const libc::c_char) as
                                isize), 0 as libc::c_int);
            assign_cfeature(&mut (*(*(*sp).bnst_data.offset(i as
                isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                as
                isize)).tag_num
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).f,
                            last_rep.offset(strlen(b"\xe6\xad\xa3\x00" as
                                *const u8 as
                                *const libc::c_char) as
                                isize), 0 as libc::c_int);
            strncpy(last_rep.offset(strlen(b"\xe6\xad\xa3\x00" as *const u8 as
                *const libc::c_char) as isize),
                    b"\xe8\xa6\x8f\xe5\x8c\x96\x00" as *const u8 as
                        *const libc::c_char,
                    strlen(b"\xe8\xa6\x8f\xe5\x8c\x96\x00" as *const u8 as
                        *const libc::c_char));
        }
        /* 末尾が一文字漢字のときは、主辞’代表表記を出力 */
        if (*(*sp).bnst_data.offset(i as isize)).tag_num > 1 as libc::c_int &&
            !check_feature((*(*(*sp).bnst_data.offset(i as
                isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                as
                isize)).tag_num
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).f,
                           b"\xe4\xb8\x80\xe6\x96\x87\xe5\xad\x97\xe6\xbc\xa2\xe5\xad\x97\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() {
            *merged_rep =
                '\u{0}' as i32 as libc::c_char; /* 主辞’代表表記 */
            error_flag =
                0 as libc::c_int; /* 末尾の基本句にも付与 */
            j =
                (*(*sp).bnst_data.offset(i as isize)).tag_num -
                    2 as libc::c_int;
            while j < (*(*sp).bnst_data.offset(i as isize)).tag_num {
                cp =
                    check_feature((*(*(*sp).bnst_data.offset(i as
                        isize)).tag_ptr.offset(j
                        as
                        isize)).f,
                                  b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char);
                if !cp.is_null() {
                    if *merged_rep != 0 {
                        strcat(merged_rep,
                               b"+\x00" as *const u8 as *const libc::c_char);
                        strcat(merged_rep,
                               cp.offset(strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                   as *const u8 as
                                   *const libc::c_char) as
                                   isize));
                    } else { strcpy(merged_rep, cp); }
                    j += 1
                } else {
                    error_flag = 1 as libc::c_int;
                    break;
                }
            }
            if error_flag == 0 {
                strncpy(merged_rep,
                        b"\xe4\xb8\xbb\xe8\xbe\x9e\xe2\x80\x99\x00" as
                            *const u8 as *const libc::c_char,
                        strlen(b"\xe4\xb8\xbb\xe8\xbe\x9e\xe2\x80\x99\x00" as
                            *const u8 as *const libc::c_char));
                assign_cfeature(&mut (*(*sp).bnst_data.offset(i as isize)).f,
                                merged_rep, 0 as libc::c_int);
                assign_cfeature(&mut (*(*(*sp).bnst_data.offset(i as
                    isize)).tag_ptr.offset((*(*sp).bnst_data.offset(i
                    as
                    isize)).tag_num
                    as
                    isize).offset(-(1
                    as
                    libc::c_int
                    as
                    isize))).f,
                                merged_rep, 0 as libc::c_int);
                strncpy(merged_rep,
                        b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\x00" as
                            *const u8 as *const libc::c_char,
                        strlen(b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\x00" as
                            *const u8 as *const libc::c_char));
            }
        }
        i += 1
    }
    free(merged_rep as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_canonical_rep_to_mrph(mut sp:
                                                      *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut m: MRPH_DATA =
        MRPH_DATA {
            type_0: 0,
            num: 0,
            parent: 0 as *mut tnode_b,
            child: [0 as *mut tnode_b; 32],
            length: 0,
            space: 0,
            dpnd_head: 0,
            dpnd_type: 0,
            dpnd_dflt: 0,
            para_num: 0,
            para_key_type: 0,
            para_top_p: 0,
            para_type: 0,
            to_para_p: 0,
            tnum: 0,
            inum: 0,
            out_head_flag: 0,
            Goi: [0; 129],
            Yomi: [0; 129],
            Goi2: [0; 129],
            Hinshi: 0,
            Bunrui: 0,
            Katuyou_Kata: 0,
            Katuyou_Kei: 0,
            Imi: [0; 1024],
            f: 0 as *mut _FEATURE,
            Num: 0,
            SM: 0 as *mut libc::c_char,
            Pos: [0; 4],
            Type: [0; 9],
        };
    let mut m_ptr: *mut MRPH_DATA = (*sp).mrph_data;
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_strt2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merged_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut rep_length: libc::c_int = 0;
    let mut rep_length2: libc::c_int = 0;
    let mut merged_rep_size: libc::c_int = 5120 as libc::c_int;
    merged_rep =
        malloc_data(merged_rep_size as size_t,
                    b"assign_canonical_rep_to_mrph\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char) as
            *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        /* 採用されている形態素の代表表記 */
        rep_strt = get_mrph_rep(m_ptr);
        rep_length = get_mrph_rep_length(rep_strt);
        if !(rep_length < 1 as libc::c_int) {
            strcpy(merged_rep,
                   b"\xe6\xad\xa3\xe8\xa6\x8f\xe5\x8c\x96\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                       as *const u8 as *const libc::c_char);
            strncat(merged_rep, rep_strt, rep_length as libc::c_ulong);
            fp = (*m_ptr).f;
            while !fp.is_null() {
                if strncmp((*fp).cp,
                           b"ALT-\x00" as *const u8 as *const libc::c_char,
                           4 as libc::c_int as libc::c_ulong) == 0 {
                    sscanf((*fp).cp.offset(4 as libc::c_int as isize),
                           b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00" as
                               *const u8 as *const libc::c_char,
                           m.Goi2.as_mut_ptr(), m.Yomi.as_mut_ptr(),
                           m.Goi.as_mut_ptr(),
                           &mut m.Hinshi as *mut libc::c_int,
                           &mut m.Bunrui as *mut libc::c_int,
                           &mut m.Katuyou_Kata as *mut libc::c_int,
                           &mut m.Katuyou_Kei as *mut libc::c_int,
                           m.Imi.as_mut_ptr());
                    rep_strt2 = get_mrph_rep(&mut m);
                    rep_length2 = get_mrph_rep_length(rep_strt2);
                    if rep_length2 > 0 as libc::c_int &&
                        (rep_length != rep_length2 ||
                            strncmp(rep_strt, rep_strt2,
                                    rep_length as libc::c_ulong) != 0) {
                        /* 正規化代表表記に"?"で連結 */
                        if strlen(merged_rep).wrapping_add(rep_length2 as
                            libc::c_ulong).wrapping_add(2
                            as
                            libc::c_int
                            as
                            libc::c_ulong)
                            > merged_rep_size as libc::c_ulong {
                            merged_rep_size *= 2 as libc::c_int;
                            merged_rep =
                                realloc_data(merged_rep as *mut libc::c_void,
                                             merged_rep_size as size_t,
                                             b"assign_canonical_rep_to_mrph\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char) as
                                    *mut libc::c_char
                        }
                        strcat(merged_rep,
                               b"?\x00" as *const u8 as *const libc::c_char);
                        strncat(merged_rep, rep_strt2,
                                rep_length2 as libc::c_ulong);
                    }
                }
                fp = (*fp).next
            }
            /* 正規化代表表記を付与 */
            assign_cfeature(&mut (*m_ptr).f, merged_rep, 0 as libc::c_int);
        }
        i += 1;
        m_ptr = m_ptr.offset(1)
    }
    free(merged_rep as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn lexical_disambiguation(mut sp: *mut SENTENCE_DATA,
                                                mut m_ptr: *mut MRPH_DATA,
                                                mut homo_num: libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0; /* 実質的同形異義語なら 1 */
    let mut j: libc::c_int =
        0; /* いずれかの形態素とマッチした
					   ルール内形態素パターンに 1 */
    let mut k: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut orig_amb_flag: libc::c_int = 0;
    let mut pref_mrph: libc::c_int = 0;
    let mut pref_rule: libc::c_int = 0;
    let mut bw_length: libc::c_int = 0;
    let mut real_homo_num: libc::c_int = 0;
    let mut uniq_flag: [libc::c_int; 30] = [0; 30];
    let mut matched_flag: [libc::c_int; 10] = [0; 10];
    let mut rep_length: libc::c_int = 0;
    let mut rep_length2: libc::c_int = 0;
    // let mut merged_rep_size: libc::c_int = 5120 as libc::c_int;
    let mut r_ptr: *mut HomoRule = 0 as *mut HomoRule;
    let mut loop_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    // let mut loop_ptr2: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut fname: [libc::c_char; 256] = [0; 256];
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_strt2: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 処理する最大数を越えていれば、最大数個だけチェックする */
    if homo_num > 30 as libc::c_int { homo_num = 30 as libc::c_int }
    /* 品詞(細分類)が異なる形態素だけを残し，uniq_flagを1にする
       => すべて残すように変更 (2006/10/16) */
    uniq_flag[0 as libc::c_int as usize] = 1 as libc::c_int;
    real_homo_num = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < homo_num {
        uniq_flag[i as usize] = 1 as libc::c_int;
        if uniq_flag[i as usize] != 0 { real_homo_num += 1 }
        i += 1
    }
    /* 実質的同形異義語がなければ何も処理はしない */
    if real_homo_num == 1 as libc::c_int { return; }
    /* ルール (mrph_homo.rule)に従って優先する形態素を選択
       ※ 同形異義語数とルール中の形態素数が同じことが条件
          各同形異義語がルール中の形態素のいずれかにマッチすればよい
	  ルールの最初の形態素にマッチしたものを優先(pref_mrph が記憶)
    */
    flag = 0 as libc::c_int;
    pref_mrph = 0 as libc::c_int;
    pref_rule = 0 as libc::c_int;
    i = 0 as libc::c_int;
    r_ptr = HomoRuleArray.as_mut_ptr();
    while i < CurHomoRuleSize {
        if (*(*r_ptr).pattern).mrphsize as libc::c_int > 10 as libc::c_int {
            fprintf(stderr,
                    b";; The number of Rule morphs is too large in HomoRule.\n\x00"
                        as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        /* そこまでの形態素列をチェック */
        bw_length =
            m_ptr.wrapping_offset_from((*sp).mrph_data) as libc::c_long as
                libc::c_int;
        if !((*r_ptr).pre_pattern.is_null() && bw_length != 0 as libc::c_int
            ||
            !(*r_ptr).pre_pattern.is_null() &&
                regexp::regexpmrphs_match((*(*r_ptr).pre_pattern).mrph.offset((*(*r_ptr).pre_pattern).mrphsize
                    as
                    libc::c_int
                    as
                    isize).offset(-(1
                    as
                    libc::c_int
                    as
                    isize)),
                                          (*(*r_ptr).pre_pattern).mrphsize as
                                              libc::c_int,
                                          m_ptr.offset(-(1 as libc::c_int as
                                              isize)), bw_length,
                                          1 as libc::c_int, 0 as libc::c_int,
                                          0 as libc::c_int) ==
                    -(1 as libc::c_int)) {
            pref_mrph = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < (*(*r_ptr).pattern).mrphsize as libc::c_int {
                matched_flag[k as usize] = 0 as libc::c_int;
                k += 1
            }
            j = 0 as libc::c_int;
            loop_ptr = m_ptr;
            while j < homo_num {
                if !(uniq_flag[j as usize] == 0 as libc::c_int) {
                    flag = 0 as libc::c_int;
                    k = 0 as libc::c_int;
                    while k < (*(*r_ptr).pattern).mrphsize as libc::c_int {
                        if !(matched_flag[k as usize] != 0 &&
                            (*(*(*r_ptr).pattern).mrph.offset(k as
                                isize)).ast_flag
                                as libc::c_int != '*' as i32) {
                            if regexpmrph_match((*(*r_ptr).pattern).mrph.offset(k
                                as
                                isize),
                                                loop_ptr) ==
                                (0 as libc::c_int == 0) as libc::c_int {
                                flag = (0 as libc::c_int == 0) as libc::c_int;
                                if k == 0 as libc::c_int { pref_mrph = j }
                                matched_flag[k as usize] =
                                    (0 as libc::c_int == 0) as libc::c_int;
                                break;
                            }
                        }
                        k += 1
                    }
                    if flag == 0 as libc::c_int { break; }
                }
                j += 1;
                loop_ptr = loop_ptr.offset(1)
            }
            if flag == (0 as libc::c_int == 0) as libc::c_int {
                k = 0 as libc::c_int;
                while k < (*(*r_ptr).pattern).mrphsize as libc::c_int {
                    if matched_flag[k as usize] == 0 as libc::c_int {
                        flag = 0 as libc::c_int;
                        break;
                    } else { k += 1 }
                }
                if flag == (0 as libc::c_int == 0) as libc::c_int {
                    pref_rule = i;
                    break;
                }
            }
        }
        i += 1;
        r_ptr = r_ptr.offset(1)
    }
    /* 多義性をマークするfeatureを与える */
    assign_cfeature(&mut (*m_ptr.offset(pref_mrph as isize)).f,
                    b"\xe5\x93\x81\xe6\x9b\x96\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int);
    if flag == (0 as libc::c_int == 0) as libc::c_int {
        /* ルールにマッチ */
        /* ルールに記述されているfeatureを与える (「品曖」を削除するルールもある) */
        assign_feature(&mut (*m_ptr.offset(pref_mrph as isize)).f,
                       &mut (*HomoRuleArray.as_mut_ptr().offset(pref_rule as
                           isize)).f,
                       m_ptr as *mut libc::c_void, 0 as libc::c_int,
                       1 as libc::c_int, 0 as libc::c_int);
        if 0 as libc::c_int != 0 && OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Lexical Disambiguation (%dth mrph -> %dth homo by %dth rule : %s :\x00"
                        as *const u8 as *const libc::c_char,
                    m_ptr.wrapping_offset_from((*sp).mrph_data) as
                        libc::c_long, pref_mrph, pref_rule,
                    (*m_ptr.offset(pref_mrph as isize)).Goi2.as_mut_ptr());
            i = 0 as libc::c_int;
            loop_ptr = m_ptr;
            while i < homo_num {
                if uniq_flag[i as usize] != 0 {
                    fprintf(Outfp,
                            b" %s\x00" as *const u8 as *const libc::c_char,
                            Class[(*loop_ptr).Hinshi as
                                usize][(*loop_ptr).Bunrui as usize].id);
                }
                i += 1;
                loop_ptr = loop_ptr.offset(1)
            }
            fprintf(Outfp, b")\n\x00" as *const u8 as *const libc::c_char);
        }
    } else if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b";; Cannot disambiguate lexical ambiguities by rules (%dth mrph : %s ?\x00"
                    as *const u8 as *const libc::c_char,
                m_ptr.wrapping_offset_from((*sp).mrph_data) as libc::c_long,
                (*m_ptr.offset(pref_mrph as isize)).Goi2.as_mut_ptr());
        i = 0 as libc::c_int;
        loop_ptr = m_ptr;
        while i < homo_num {
            if uniq_flag[i as usize] != 0 {
                fprintf(Outfp, b" %s\x00" as *const u8 as *const libc::c_char,
                        Class[(*loop_ptr).Hinshi as
                            usize][(*loop_ptr).Bunrui as usize].id);
            }
            i += 1;
            loop_ptr = loop_ptr.offset(1)
        }
        fprintf(Outfp, b")\n\x00" as *const u8 as *const libc::c_char);
    }
    /* pref_mrph以外の形態素情報をALTで保存する
       また、それらの意味情報の一部をpref_mrphのfeatureに付与 */
    orig_amb_flag = 0 as libc::c_int;
    if !check_feature((*m_ptr.offset(pref_mrph as isize)).f,
                      b"\xe5\x93\x81\xe6\x9b\x96\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
    {
        /* pref_mrphの代表表記 */
        rep_strt = get_mrph_rep(m_ptr.offset(pref_mrph as isize));
        rep_length = get_mrph_rep_length(rep_strt);
        i = 0 as libc::c_int;
        while i < homo_num {
            if i != pref_mrph {
                /* 代表表記がpref_mrphと異なる場合、orig_amb_flagを1にする */
                rep_strt2 = get_mrph_rep(m_ptr.offset(i as isize));
                rep_length2 = get_mrph_rep_length(rep_strt2);
                if rep_length > 0 as libc::c_int &&
                    (rep_length != rep_length2 ||
                        strncmp(rep_strt, rep_strt2,
                                rep_length as libc::c_ulong) != 0) {
                    orig_amb_flag = 1 as libc::c_int
                }
                /* 形態素情報をfeature(<ALT-...>)として保存 */
                assign_feature_alt_mrph(&mut (*m_ptr.offset(pref_mrph as
                    isize)).f,
                                        m_ptr.offset(i as isize));
                /* pref_mrph以外の形態素がもつ意味情報をすべて付与しておく */
                selected_imi2feature((*m_ptr.offset(i as
                    isize)).Imi.as_mut_ptr(),
                                     m_ptr.offset(pref_mrph as isize));
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while i < homo_num {
            if !(uniq_flag[i as usize] == 0 as libc::c_int) {
                sprintf(fname.as_mut_ptr(),
                        b"\xe5\x93\x81\xe6\x9b\x96-%s\x00" as *const u8 as
                            *const libc::c_char,
                        Class[(*m_ptr.offset(i as isize)).Hinshi as
                            usize][(*m_ptr.offset(i as isize)).Bunrui as
                            usize].id);
                assign_cfeature(&mut (*m_ptr.offset(pref_mrph as isize)).f,
                                fname.as_mut_ptr(), 0 as libc::c_int);
            }
            i += 1
        }
    }
    /* 代表表記が曖昧なときはマークしておく */
    if orig_amb_flag != 0 {
        assign_cfeature(&mut (*m_ptr.offset(pref_mrph as isize)).f,
                        b"\xe5\x8e\x9f\xe5\xbd\xa2\xe6\x9b\x96\xe6\x98\xa7\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char, 0 as libc::c_int);
    }
    /* pref_mrph番目のデータをコピー */
    if pref_mrph != 0 as libc::c_int {
        strcpy((*m_ptr).Goi2.as_mut_ptr(),
               (*m_ptr.offset(pref_mrph as isize)).Goi2.as_mut_ptr());
        strcpy((*m_ptr).Yomi.as_mut_ptr(),
               (*m_ptr.offset(pref_mrph as isize)).Yomi.as_mut_ptr());
        strcpy((*m_ptr).Goi.as_mut_ptr(),
               (*m_ptr.offset(pref_mrph as isize)).Goi.as_mut_ptr());
        (*m_ptr).Hinshi = (*m_ptr.offset(pref_mrph as isize)).Hinshi;
        (*m_ptr).Bunrui = (*m_ptr.offset(pref_mrph as isize)).Bunrui;
        (*m_ptr).Katuyou_Kata =
            (*m_ptr.offset(pref_mrph as isize)).Katuyou_Kata;
        (*m_ptr).Katuyou_Kei =
            (*m_ptr.offset(pref_mrph as isize)).Katuyou_Kei;
        strcpy((*m_ptr).Imi.as_mut_ptr(),
               (*m_ptr.offset(pref_mrph as isize)).Imi.as_mut_ptr());
        clear_feature(&mut (*m_ptr).f);
        (*m_ptr).f = (*m_ptr.offset(pref_mrph as isize)).f;
        let ref mut fresh0 = (*m_ptr.offset(pref_mrph as isize)).f;
        *fresh0 = 0 as FEATUREptr;
        (*m_ptr).length = (*m_ptr.offset(pref_mrph as isize)).length
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn readtoeos(mut fp: *mut FILE) -> libc::c_int
/*==================================================================*/
{
    let mut input_buffer: [libc::c_uchar; 5120] = [0; 5120];
    loop {
        if fgets(input_buffer.as_mut_ptr() as *mut libc::c_char,
                 5120 as libc::c_int, fp).is_null() {
            return -(1 as libc::c_int);
        }
        if strcmp(input_buffer.as_mut_ptr() as *const libc::c_char,
                  b"EOS\n\x00" as *const u8 as *const libc::c_char) == 0 {
            return 0 as libc::c_int;
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn readtonl(mut fp: *mut FILE) -> libc::c_int
/*==================================================================*/
{
    let mut input_buffer: libc::c_int = 0;
    loop {
        input_buffer = fgetc(fp);
        if input_buffer == -(1 as libc::c_int) { return -(1 as libc::c_int); }
        if input_buffer == '\n' as i32 { return 0 as libc::c_int; }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_mrph_file(mut fp: *mut FILE,
                                        mut buffer: *mut libc::c_uchar)
                                        -> libc::c_int
/*==================================================================*/
{
    let mut len: libc::c_int = 0;
    if fgets(buffer as *mut libc::c_char, 5120 as libc::c_int, fp).is_null() {
        return -(1 as libc::c_int);
    }
    /* Server モードの場合は 注意 \r\n になる*/
    if OptMode == 1 as libc::c_int {
        len = strlen(buffer as *const libc::c_char) as libc::c_int;
        if len > 2 as libc::c_int &&
            *buffer.offset((len - 1 as libc::c_int) as isize) as
                libc::c_int == '\n' as i32 &&
            *buffer.offset((len - 2 as libc::c_int) as isize) as
                libc::c_int == '\r' as i32 {
            *buffer.offset((len - 2 as libc::c_int) as isize) =
                '\n' as i32 as libc::c_uchar;
            *buffer.offset((len - 1 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_uchar
        }
        if *buffer.offset(0 as libc::c_int as isize) as libc::c_int ==
            0xb as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn imi2feature(mut str: *mut libc::c_char,
                                     mut m_ptr: *mut MRPH_DATA)
                                     -> libc::c_int
/*==================================================================*/
{
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    token = strtok(str, b" \x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        assign_cfeature(&mut (*m_ptr).f, token, 0 as libc::c_int);
        token =
            strtok(0 as *mut libc::c_char,
                   b" \x00" as *const u8 as *const libc::c_char)
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_existing_features(mut m_ptr: *mut MRPH_DATA)
/*==================================================================*/
{
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe3\x83\x89\xe3\x83\xa1\xe3\x82\xa4\xe3\x83\xb3\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe6\xbc\xa2\xe5\xad\x97\xe8\xaa\xad\xe3\x81\xbf\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe6\xb3\xa8\xe9\x87\x88\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe8\xac\x99\xe8\xad\xb2\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe5\xb0\x8a\xe6\x95\xac\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe4\xb8\x81\xe5\xaf\xa7\xe5\x8b\x95\xe8\xa9\x9e\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe6\xa8\x99\xe6\xba\x96\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
    delete_cfeature(&mut (*m_ptr).f,
                    b"\xe7\x9c\x81\xe7\x95\xa5\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn copy_mrph(mut dst: *mut MRPH_DATA,
                                   mut src: *mut MRPH_DATA,
                                   mut imi2feature_flag: libc::c_int)
/*==================================================================*/
{
    let mut imip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy((*dst).Goi.as_mut_ptr(), (*src).Goi.as_mut_ptr());
    strcpy((*dst).Yomi.as_mut_ptr(), (*src).Yomi.as_mut_ptr());
    strcpy((*dst).Goi2.as_mut_ptr(), (*src).Goi2.as_mut_ptr());
    (*dst).Hinshi = (*src).Hinshi;
    (*dst).Bunrui = (*src).Bunrui;
    (*dst).Katuyou_Kata = (*src).Katuyou_Kata;
    (*dst).Katuyou_Kei = (*src).Katuyou_Kei;
    strcpy((*dst).Imi.as_mut_ptr(), (*src).Imi.as_mut_ptr());
    /* 意味情報をfeatureへ */
    if imi2feature_flag != 0 {
        if (*src).Imi[0 as libc::c_int as usize] as libc::c_int == '\"' as i32
        {
            /* 通常 "" で括られている */
            imip =
                &mut *(*src).Imi.as_mut_ptr().offset(1 as libc::c_int as
                    isize) as
                    *mut libc::c_char;
            cp = strchr(imip, '\"' as i32);
            if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
        } else { imip = (*src).Imi.as_mut_ptr() }
        imi2feature(imip, dst);
    } else {
        (*dst).f = (*src).f;
        (*src).f = 0 as FEATUREptr
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn feature_string2f(mut str: *mut libc::c_char,
                                          mut f: *mut *mut FEATURE)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    token = strtok(str, b"><\x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        assign_cfeature(f, token, 0 as libc::c_int);
        token =
            strtok(0 as *mut libc::c_char,
                   b"><\x00" as *const u8 as *const libc::c_char)
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn store_one_annotation(mut sp: *mut SENTENCE_DATA,
                                              mut tp: *mut TAG_DATA,
                                              mut token: *mut libc::c_char)
                                              -> libc::c_int
/*==================================================================*/
{
    let mut flag: libc::c_char = 0;
    let mut rel: [libc::c_char; 128] = [0; 128];
    let mut word: [libc::c_char; 256] = [0; 256];
    let mut tag_n: libc::c_int = 0;
    let mut sent_n: libc::c_int = 0;
    sscanf(token,
           b"%[^/]/%c/%[^/]/%d/%d/%*[^;]\x00" as *const u8 as
               *const libc::c_char, rel.as_mut_ptr(),
           &mut flag as *mut libc::c_char, word.as_mut_ptr(),
           &mut tag_n as *mut libc::c_int, &mut sent_n as *mut libc::c_int);
    (*(*tp).c_cpm_ptr).cf.pp[(*(*tp).c_cpm_ptr).cf.element_num as
        usize][0 as libc::c_int as usize] =
        pp_kstr_to_code(rel.as_mut_ptr());
    (*(*tp).c_cpm_ptr).cf.pp[(*(*tp).c_cpm_ptr).cf.element_num as
        usize][1 as libc::c_int as usize] =
        -(10 as libc::c_int);
    if (*(*tp).c_cpm_ptr).cf.pp[(*(*tp).c_cpm_ptr).cf.element_num as
        usize][0 as libc::c_int as usize] ==
        -(10 as libc::c_int) {
        if OptDisplay == 3 as libc::c_int {
            fprintf(stderr,
                    b";; Unknown case <%s>\n\x00" as *const u8 as
                        *const libc::c_char, rel.as_mut_ptr());
        }
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if flag as libc::c_int == 'E' as i32 || flag as libc::c_int == 'U' as i32
    {
        /* 不特定、または、割り当てなし(OptReadFeature用) */
        (*(*tp).c_cpm_ptr).elem_b_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] = 0 as *mut TAG_DATA;
        (*(*tp).c_cpm_ptr).elem_s_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] = 0 as *mut sentence
    } else if sent_n > 0 as libc::c_int {
        /* 異常なタグ単位が指定されているかチェック */
        if (*sp).Sen_num - sent_n < 1 as libc::c_int ||
            tag_n >=
                (*sentence_data.as_mut_ptr().offset((*sp).Sen_num as
                    isize).offset(-(1
                    as
                    libc::c_int
                    as
                    isize)).offset(-(sent_n
                    as
                    isize))).Tag_num
        {
            fprintf(stderr,
                    b";; discarded inappropriate annotation: %s/%c/%s/%d/%d\n\x00"
                        as *const u8 as *const libc::c_char, rel.as_mut_ptr(),
                    flag as libc::c_int, word.as_mut_ptr(), tag_n, sent_n);
            return 0 as libc::c_int;
        }
        (*(*tp).c_cpm_ptr).elem_b_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] =
            (*sentence_data.as_mut_ptr().offset((*sp).Sen_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize)).offset(-(sent_n
                as
                isize))).tag_data.offset(tag_n
                as
                isize);
        (*(*tp).c_cpm_ptr).elem_s_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] =
            sentence_data.as_mut_ptr().offset((*sp).Sen_num as
                isize).offset(-(1 as
                libc::c_int
                as
                isize)).offset(-(sent_n
                as
                isize))
    } else {
        /* 現在の対象文 (この文はまだsentence_dataに入っていないため、上のようには扱えない)
   	   異常なタグ単位が指定されているかのチェックはcheck_annotation()で行う */
        (*(*tp).c_cpm_ptr).elem_b_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] =
            (*sp).tag_data.offset(tag_n as isize);
        (*(*tp).c_cpm_ptr).elem_s_ptr[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] = sp
    }
    if flag as libc::c_int == 'C' as i32 {
        (*(*tp).c_cpm_ptr).elem_b_num[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] =
            (*(*tp).c_cpm_ptr).cf.element_num
    } else if flag as libc::c_int == 'N' as i32 {
        (*(*tp).c_cpm_ptr).elem_b_num[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] = -(1 as libc::c_int)
    } else {
        (*(*tp).c_cpm_ptr).elem_b_num[(*(*tp).c_cpm_ptr).cf.element_num as
            usize] = -(2 as libc::c_int)
    }
    (*(*tp).c_cpm_ptr).cf.element_num += 1;
    if (*(*tp).c_cpm_ptr).cf.element_num >= 24 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_annotation(mut sp: *mut SENTENCE_DATA,
                                         mut tp: *mut TAG_DATA)
                                         -> libc::c_int
/*==================================================================*/
{
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* featureから格解析結果を取得 */
    cp =
        check_feature((*tp).f,
                      b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() {
        (*tp).c_cpm_ptr =
            malloc_data(::std::mem::size_of::<CF_PRED_MGR>() as libc::c_ulong,
                        b"read_annotation\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                *mut CF_PRED_MGR;
        memset((*tp).c_cpm_ptr as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<CF_PRED_MGR>() as libc::c_ulong);
        cp =
            cp.offset(strlen(b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        cp = strchr(cp, ':' as i32).offset(1 as libc::c_int as isize);
        if OptAnaphora != 0 {
            cp = strchr(cp, ':' as i32).offset(1 as libc::c_int as isize)
        }
        start_cp = cp;
        while *cp != 0 {
            if *cp as libc::c_int == ';' as i32 {
                if store_one_annotation(sp, tp, start_cp) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                start_cp = cp.offset(1 as libc::c_int as isize)
            }
            cp = cp.offset(1)
        }
        if store_one_annotation(sp, tp, start_cp) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_annotation(mut sp: *mut SENTENCE_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut check: [libc::c_int; 24] = [0; 24];
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        tp = (*sp).tag_data.offset(i as isize);
        if !(*tp).c_cpm_ptr.is_null() {
            j = 0 as libc::c_int;
            while j < (*(*tp).c_cpm_ptr).cf.element_num {
                /* 対象文の場合に、異常なタグ単位が指定されているかチェック */
                if sp == (*(*tp).c_cpm_ptr).elem_s_ptr[j as usize] &&
                    (*(*tp).c_cpm_ptr).elem_b_ptr[j as
                        usize].wrapping_offset_from((*sp).tag_data)
                        as libc::c_long >= (*sp).Tag_num as libc::c_long {
                    if !(*(*(*tp).c_cpm_ptr).elem_b_ptr[j as
                        usize]).head_ptr.is_null()
                    {
                        fprintf(stderr,
                                b";; discarded inappropriate annotation: %s/?/%s/%d/0\n\x00"
                                    as *const u8 as *const libc::c_char,
                                pp_code_to_kstr((*(*tp).c_cpm_ptr).cf.pp[j as
                                    usize][0
                                    as
                                    libc::c_int
                                    as
                                    usize]),
                                (*(*(*(*tp).c_cpm_ptr).elem_b_ptr[j as
                                    usize]).head_ptr).Goi.as_mut_ptr(),
                                (*(*(*tp).c_cpm_ptr).elem_b_ptr[j as
                                    usize]).num);
                        check[j as usize] = 0 as libc::c_int
                    }
                } else {
                    check[j as usize] = (0 as libc::c_int == 0) as libc::c_int
                }
                j += 1
            }
            /* ずらす */
            k = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*(*tp).c_cpm_ptr).cf.element_num {
                if check[j as usize] == (0 as libc::c_int == 0) as libc::c_int
                {
                    if k != j {
                        (*(*tp).c_cpm_ptr).cf.pp[k as
                            usize][0 as libc::c_int
                            as usize] =
                            (*(*tp).c_cpm_ptr).cf.pp[j as
                                usize][0 as
                                libc::c_int
                                as usize];
                        (*(*tp).c_cpm_ptr).elem_b_ptr[k as usize] =
                            (*(*tp).c_cpm_ptr).elem_b_ptr[j as usize];
                        (*(*tp).c_cpm_ptr).elem_s_ptr[k as usize] =
                            (*(*tp).c_cpm_ptr).elem_s_ptr[j as usize];
                        (*(*tp).c_cpm_ptr).elem_b_num[k as usize] =
                            (*(*tp).c_cpm_ptr).elem_b_num[j as usize]
                    }
                    k += 1
                }
                j += 1
            }
            if k != 0 {
                (*(*tp).c_cpm_ptr).cf.element_num = k
            } else {
                /* 1つもなくなったらfree */
                free((*tp).c_cpm_ptr as *mut libc::c_void);
                (*tp).c_cpm_ptr = 0 as CPM_ptr
            }
        }
        i += 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_mrph(mut sp: *mut SENTENCE_DATA,
                                   mut fp: *mut FILE) -> libc::c_int
/*==================================================================*/
{
    let mut input_buffer: [libc::c_uchar; 5120] = [0; 5120];
    // let mut rev_ibuffer: [libc::c_uchar; 5120] = [0; 5120];
    let mut rest_buffer: [libc::c_uchar; 5120] = [0; 5120];
    let mut Hinshi_str: [libc::c_uchar; 5120] = [0; 5120];
    let mut Bunrui_str: [libc::c_uchar; 5120] = [0; 5120];
    let mut ne_buffer: [libc::c_uchar; 5120] = [0; 5120];
    let mut Katuyou_Kata_str: [libc::c_uchar; 5120] = [0; 5120];
    let mut Katuyou_Kei_str: [libc::c_uchar; 5120] = [0; 5120];
    let mut m_ptr: *mut MRPH_DATA = (*sp).mrph_data;
    let mut homo_num: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut mrph_item: libc::c_int = 0;
    let mut bnst_item: libc::c_int = 0;
    let mut tag_item: libc::c_int = 0;
    let mut ne_flag: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut homo_flag: libc::c_int = 0;
    (*sp).Mrph_num = 0 as libc::c_int;
    homo_num = 0 as libc::c_int;
    ErrorComment = 0 as *mut libc::c_char;
    *PM_Memo.as_mut_ptr().offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    input_buffer[(5120 as libc::c_int - 1 as libc::c_int) as usize] =
        '\n' as i32 as libc::c_uchar;
    /* 文カウント (S-IDがないときの出力用; 形態素が一つもなくても数える) */
    total_sen_num += 1;
    loop {
        if read_mrph_file(fp, input_buffer.as_mut_ptr()) ==
            -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if input_buffer[(5120 as libc::c_int - 1 as libc::c_int) as usize] as
            libc::c_int != '\n' as i32 {
            input_buffer[(5120 as libc::c_int - 1 as libc::c_int) as usize] =
                '\u{0}' as i32 as libc::c_uchar;
            fprintf(stderr,
                    b";; Too long mrph <%s> !\n\x00" as *const u8 as
                        *const libc::c_char, input_buffer.as_mut_ptr());
            return readtonl(fp);
        } else {
            if input_buffer[strlen(input_buffer.as_mut_ptr() as
                *const libc::c_char).wrapping_sub(1 as
                libc::c_int
                as
                libc::c_ulong)
                as usize] as libc::c_int != '\n' as i32 {
                fprintf(stderr,
                        b";; Too long mrph <%s> !\n\x00" as *const u8 as
                            *const libc::c_char, input_buffer.as_mut_ptr());
                return 0 as libc::c_int;
            }
        }
        /* -i によるコメント行 */
        if OptIgnoreChar as libc::c_int != 0 &&
            *input_buffer.as_mut_ptr() as libc::c_int ==
                OptIgnoreChar as libc::c_int {
            fprintf(Outfp, b"%s\x00" as *const u8 as *const libc::c_char,
                    input_buffer.as_mut_ptr());
            fflush(Outfp);
        } else if input_buffer[0 as libc::c_int as usize] as libc::c_int ==
            '#' as i32 {
            let mut match_num: libc::c_int = 0;
            input_buffer[strlen(input_buffer.as_mut_ptr() as
                *const libc::c_char).wrapping_sub(1 as
                libc::c_int
                as
                libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_uchar;
            (*sp).Comment =
                malloc_data(strlen(input_buffer.as_mut_ptr() as
                    *const libc::c_char),
                            b"read_mrph\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            (*sp).KNPSID =
                malloc_data(strlen(input_buffer.as_mut_ptr() as
                    *const libc::c_char).wrapping_add(3 as
                    libc::c_int
                    as
                    libc::c_ulong),
                            b"read_mrph\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) as
                    *mut libc::c_char;
            match_num =
                sscanf(input_buffer.as_mut_ptr() as *const libc::c_char,
                       b"# %s %[^\n]\x00" as *const u8 as *const libc::c_char,
                       (*sp).KNPSID, (*sp).Comment);
            if match_num < 2 as libc::c_int {
                /* # による正規のコメント行 */
                /* コメント行にスペースが含まれない場合 */
                *(*sp).Comment.offset(0 as libc::c_int as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
            /* 文章が変わったら固有名詞スタック, 前文データをクリア */
            if strncmp((*sp).KNPSID,
                       b"S-ID:\x00" as *const u8 as *const libc::c_char,
                       5 as libc::c_int as libc::c_ulong) == 0 &&
                !strchr((*sp).KNPSID.offset(5 as libc::c_int as isize),
                        '-' as i32).is_null() &&
                strlen((*sp).KNPSID) <
                    (::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                        as libc::c_ulong)
            {
                /* 「記事ID-文ID」という形式ならば */
                /* 末尾の'-'より前をArticleIDとみなす */
                strcpy(ArticleID.as_mut_ptr(),
                       (*sp).KNPSID.offset(5 as libc::c_int as isize));
                *strrchr(ArticleID.as_mut_ptr(), '-' as i32) =
                    '\u{0}' as i32 as libc::c_char;
                if strcmp(ArticleID.as_mut_ptr(), preArticleID.as_mut_ptr())
                    != 0 {
                    if OptDisplay == 3 as libc::c_int {
                        fprintf(stderr,
                                b";; New Article %s\n\x00" as *const u8 as
                                    *const libc::c_char,
                                input_buffer.as_mut_ptr());
                    }
                    if OptAnaphora != 0 {
                        clear_context(sp,
                                      (0 as libc::c_int == 0) as libc::c_int);
                    } else if OptEllipsis != 0 { ClearSentences(sp); }
                    if OptNE != 0 { clear_ne_cache(); }
                }
                strcpy(preArticleID.as_mut_ptr(), ArticleID.as_mut_ptr());
            }
        } else if input_buffer[0 as libc::c_int as usize] as libc::c_int ==
            '*' as i32 {
            let mut dpnd_head: libc::c_int = 0;
            let mut dpnd_type: libc::c_char = 0;
            /* 解析済みの場合 */
            /* 文節行 */
            /* 文節行を読む
         * input_buffer -> bnst_item
         * "*" -> EOF
         * "* " -> EOF
         * "* hoge" -> 0
         * "* 1" -> 1
         * "* 1D" -> 2
         * "* 1D <...>" -> 3
         */
            bnst_item =
                sscanf(input_buffer.as_mut_ptr() as *const libc::c_char,
                       b"* %d%c %[^\n]\x00" as *const u8 as
                           *const libc::c_char,
                       &mut dpnd_head as *mut libc::c_int,
                       &mut dpnd_type as *mut libc::c_char,
                       rest_buffer.as_mut_ptr()); /* 係り受け付与済み */
            match bnst_item {
                2 | 3 => { OptInput |= 1 as libc::c_int }
                -1 => {}
                _ => {
                    fprintf(stderr,
                            b";; Invalid input <%s> !\n\x00" as *const u8 as
                                *const libc::c_char,
                            input_buffer.as_mut_ptr()); /* 文節分割済み */
                    OptInput = 0 as libc::c_int;
                    return readtoeos(fp);
                }
            }
            OptInput |= 4 as libc::c_int;
            if (*sp).Mrph_num == 0 as libc::c_int {
                if OptEllipsis != 0 { OptAnalysis = 6 as libc::c_int }
                (*sp).Bnst_num = 0 as libc::c_int;
                (*sp).Tag_num = 0 as libc::c_int;
                memset(Bnst_start.as_mut_ptr() as *mut libc::c_void,
                       0 as libc::c_int,
                       (::std::mem::size_of::<libc::c_int>() as
                           libc::c_ulong).wrapping_mul(200 as libc::c_int as
                           libc::c_ulong));
                memset(Tag_start.as_mut_ptr() as *mut libc::c_void,
                       0 as libc::c_int,
                       (::std::mem::size_of::<libc::c_int>() as
                           libc::c_ulong).wrapping_mul(200 as libc::c_int as
                           libc::c_ulong));
                if OptReadFeature != 0 {
                    memset(Input_bnst_feature.as_mut_ptr() as
                               *mut libc::c_void, 0 as libc::c_int,
                           (::std::mem::size_of::<*mut FEATURE>() as
                               libc::c_ulong).wrapping_mul(200 as libc::c_int
                               as
                               libc::c_ulong));
                    memset(Input_tag_feature.as_mut_ptr() as
                               *mut libc::c_void, 0 as libc::c_int,
                           (::std::mem::size_of::<*mut FEATURE>() as
                               libc::c_ulong).wrapping_mul(200 as libc::c_int
                               as
                               libc::c_ulong));
                }
            }
            (*(*sp).Best_mgr).dpnd.head[(*sp).Bnst_num as usize] = dpnd_head;
            (*(*sp).Best_mgr).dpnd.type_0[(*sp).Bnst_num as usize] =
                dpnd_type;
            /* 文節の入力されたfeatureを使う */
            if bnst_item == 3 as libc::c_int && OptReadFeature != 0 {
                /* featureを<>でsplitしてfに変換 */
                feature_string2f(rest_buffer.as_mut_ptr() as
                                     *mut libc::c_char,
                                 &mut *Input_bnst_feature.as_mut_ptr().offset((*sp).Bnst_num
                                     as
                                     isize));
            }
            Bnst_start[((*sp).Mrph_num - homo_num) as usize] =
                1 as libc::c_int;
            (*sp).Bnst_num += 1
        } else if input_buffer[0 as libc::c_int as usize] as libc::c_int ==
            '+' as i32 {
            if OptInput == 0 as libc::c_int {
                fprintf(stderr,
                        b";; Invalid input <%s> !\n\x00" as *const u8 as
                            *const libc::c_char, input_buffer.as_mut_ptr());
                return readtoeos(fp);
            }
            /* タグ単位行 */
            /* タグ単位行を読む
         * input_buffer -> tag_item
         * "+" -> EOF
         * "+ " -> EOF
         * "+ hoge" -> 0
         * "+ 1" -> 1
         * "+ 1D" -> 2
         * "+ 1D <...>" -> 3
         */
            tag_item =
                sscanf(input_buffer.as_mut_ptr() as *const libc::c_char,
                       b"+ %d%c %[^\n]\x00" as *const u8 as
                           *const libc::c_char,
                       &mut *Tag_dpnd.as_mut_ptr().offset((*sp).Tag_num as
                           isize) as
                           *mut libc::c_int,
                       &mut *Tag_type.as_mut_ptr().offset((*sp).Tag_num as
                           isize) as
                           *mut libc::c_int,
                       rest_buffer.as_mut_ptr()); /* 係り受け付与済み */
            match tag_item {
                2 | 3 => { OptInput |= 1 as libc::c_int }
                -1 => {}
                _ => {
                    fprintf(stderr,
                            b";; Invalid input <%s> !\n\x00" as *const u8 as
                                *const libc::c_char,
                            input_buffer.as_mut_ptr()); /* タグ分割済み */
                    OptInput = 0 as libc::c_int;
                    return readtoeos(fp);
                }
            }
            OptInput |= 4 as libc::c_int;
            /* タグ単位の入力されたfeatureを使う */
            if tag_item == 3 as libc::c_int && OptReadFeature != 0 {
                /* featureを<>でsplitしてfに変換 */
                feature_string2f(rest_buffer.as_mut_ptr() as
                                     *mut libc::c_char,
                                 &mut *Input_tag_feature.as_mut_ptr().offset((*sp).Tag_num
                                     as
                                     isize));
            }
            Tag_start[((*sp).Mrph_num - homo_num) as usize] =
                1 as libc::c_int;
            (*sp).Tag_num += 1
        } else if strcmp(input_buffer.as_mut_ptr() as *const libc::c_char,
                         b"EOS\n\x00" as *const u8 as *const libc::c_char) ==
            0 {
            /* 文末 */
            /* 形態素が一つもないとき */
            if (*sp).Mrph_num == 0 as libc::c_int { return 0 as libc::c_int; }
            /* タグ単位のない解析済の場合 */
            if OptInput & 1 as libc::c_int != 0 &&
                (*sp).Tag_num == 0 as libc::c_int {
                OptInput |= 2 as libc::c_int
            }
            if homo_num != 0 {
                /* 前に同形異義語セットがあれば処理する */
                lexical_disambiguation(sp,
                                       m_ptr.offset(-(homo_num as
                                           isize)).offset(-(1
                                           as
                                           libc::c_int
                                           as
                                           isize)),
                                       homo_num + 1 as libc::c_int);
                (*sp).Mrph_num -= homo_num;
                m_ptr = m_ptr.offset(-(homo_num as isize));
                i = 0 as libc::c_int;
                while i < homo_num {
                    clear_feature(&mut (*m_ptr.offset(i as isize)).f);
                    i += 1
                }
                homo_num = 0 as libc::c_int
            } else if (*sp).Mrph_num > 0 as libc::c_int {
                /* 同形異義語がないときに正規化代表表記を付与 */
                rn2canonical_rn(m_ptr.offset(-(1 as libc::c_int as isize)));
            }
            /* KNPSIDがないとき(# S-ID行がないとき)は付与 */
            if (*sp).KNPSID.is_null() {
                /* "S-ID:"(5バイト), log(文数)/log(10) + 1バイト, 括弧ID(3バイト), +1バイト */
                (*sp).KNPSID =
                    malloc_data((log(total_sen_num as libc::c_double) /
                        log(10 as libc::c_int as libc::c_double)
                        + 10 as libc::c_int as libc::c_double) as
                                    size_t,
                                b"read_mrph\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
                        as *mut libc::c_char;
                sprintf((*sp).KNPSID,
                        b"S-ID:%d\x00" as *const u8 as *const libc::c_char,
                        total_sen_num);
            }
            return (0 as libc::c_int == 0) as libc::c_int;
        } else {
            /* 通常の形態素 */
            /* 同形異義語かどうか */
            if input_buffer[0 as libc::c_int as usize] as libc::c_int ==
                '@' as i32 &&
                input_buffer[1 as libc::c_int as usize] as libc::c_int ==
                    ' ' as i32 &&
                input_buffer[2 as libc::c_int as usize] as libc::c_int !=
                    '@' as i32 {
                homo_flag = 1 as libc::c_int
            } else { homo_flag = 0 as libc::c_int }
            if homo_flag == 0 as libc::c_int && homo_num != 0 {
                /* 同形異義語マークがなく，前に同形異義語セットがあれば
	           lexical_disambiguationを呼んで処理 */
                lexical_disambiguation(sp,
                                       m_ptr.offset(-(homo_num as
                                           isize)).offset(-(1
                                           as
                                           libc::c_int
                                           as
                                           isize)),
                                       homo_num + 1 as libc::c_int);
                (*sp).Mrph_num -= homo_num;
                m_ptr = m_ptr.offset(-(homo_num as isize));
                i = 0 as libc::c_int;
                while i < homo_num {
                    clear_feature(&mut (*m_ptr.offset(i as isize)).f);
                    i += 1
                }
                homo_num = 0 as libc::c_int
            } else if (*sp).Mrph_num > 0 as libc::c_int {
                /* 同形異義語がないときに正規化代表表記を付与 */
                rn2canonical_rn(m_ptr.offset(-(1 as libc::c_int as isize)));
            }
            if (*sp).Mrph_num >= 200 as libc::c_int {
                fprintf(stderr,
                        b";; Too many mrph (%s %s%s...)!\n\x00" as *const u8
                            as *const libc::c_char,
                        if !(*sp).Comment.is_null() {
                            (*sp).Comment as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, (*sp).mrph_data,
                        (*sp).mrph_data.offset(1 as libc::c_int as isize));
                return readtoeos(fp);
            }
            offset =
                if homo_flag != 0 {
                    2 as libc::c_int
                } else { 0 as libc::c_int };
            mrph_item =
                sscanf(input_buffer.as_mut_ptr().offset(offset as isize) as
                           *const libc::c_char,
                       b"%s %s %s %s %d %s %d %s %d %s %d %[^\n]\x00" as
                           *const u8 as *const libc::c_char,
                       (*m_ptr).Goi2.as_mut_ptr(), (*m_ptr).Yomi.as_mut_ptr(),
                       (*m_ptr).Goi.as_mut_ptr(), Hinshi_str.as_mut_ptr(),
                       &mut (*m_ptr).Hinshi as *mut libc::c_int,
                       Bunrui_str.as_mut_ptr(),
                       &mut (*m_ptr).Bunrui as *mut libc::c_int,
                       Katuyou_Kata_str.as_mut_ptr(),
                       &mut (*m_ptr).Katuyou_Kata as *mut libc::c_int,
                       Katuyou_Kei_str.as_mut_ptr(),
                       &mut (*m_ptr).Katuyou_Kei as *mut libc::c_int,
                       rest_buffer.as_mut_ptr());
            (*m_ptr).type_0 = 4 as libc::c_int;
            (*m_ptr).num = (*sp).Mrph_num;
            (*m_ptr).length =
                strlen((*m_ptr).Goi2.as_mut_ptr()) as libc::c_int;
            if Language == 2 as libc::c_int {
                /* 最大数を越えないようにチェック */
                /* 形態素情報 :
	       語彙(活用形) 読み 語彙(原型) 
	       品詞(+番号) 細分類(+番号) 活用型(+番号) 活用形(+番号) 
	       意味情報
	    */
                /* transfer POS to word features for Chinese */
                assign_cfeature(&mut (*m_ptr).f,
                                Hinshi_str.as_mut_ptr() as *mut libc::c_char,
                                0 as libc::c_int);
                if OptChiPos == 0 {
                    strcpy((*m_ptr).Pos.as_mut_ptr(),
                           Hinshi_str.as_mut_ptr() as *const libc::c_char);
                    // treat different punc as different type
                    if strcmp(*Chi_word_type.as_mut_ptr().offset((*m_ptr).Hinshi
                        as
                        isize),
                              b"punc\x00" as *const u8 as *const libc::c_char)
                        == 0 {
                        if strcmp((*m_ptr).Goi.as_mut_ptr(),
                                  b",\x00" as *const u8 as
                                      *const libc::c_char) == 0 ||
                            strcmp((*m_ptr).Goi.as_mut_ptr(),
                                   b"\xef\xbc\x8c\x00" as *const u8 as
                                       *const libc::c_char) == 0 {
                            strcpy((*m_ptr).Type.as_mut_ptr(),
                                   b"punc\x00" as *const u8 as
                                       *const libc::c_char);
                        } else if strcmp((*m_ptr).Goi.as_mut_ptr(),
                                         b"\xef\xbc\x9a\x00" as *const u8 as
                                             *const libc::c_char) == 0 ||
                            strcmp((*m_ptr).Goi.as_mut_ptr(),
                                   b":\x00" as *const u8 as
                                       *const libc::c_char) == 0 {
                            strcpy((*m_ptr).Type.as_mut_ptr(),
                                   b"punc\x00" as *const u8 as
                                       *const libc::c_char);
                        } else if strcmp((*m_ptr).Goi.as_mut_ptr(),
                                         b"\xe3\x80\x81\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            strcpy((*m_ptr).Type.as_mut_ptr(),
                                   b"punc\x00" as *const u8 as
                                       *const libc::c_char);
                        } else if strcmp((*m_ptr).Goi.as_mut_ptr(),
                                         b"\xef\xbc\x9b\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            strcpy((*m_ptr).Type.as_mut_ptr(),
                                   b"punc\x00" as *const u8 as
                                       *const libc::c_char);
                        } else {
                            strcpy((*m_ptr).Type.as_mut_ptr(),
                                   b"\x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else {
                        strcpy((*m_ptr).Type.as_mut_ptr(),
                               *Chi_word_type.as_mut_ptr().offset((*m_ptr).Hinshi
                                   as
                                   isize));
                    }
                }
            }
            if mrph_item == 12 as libc::c_int {
                let mut imip: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut rep_buf: *mut libc::c_char = 0 as *mut libc::c_char;
                /* "<NE:...>"形式で与えられた固有表現タグを意味情報に追加 */
                ne_flag = 0 as libc::c_int;
                if OptNElearn != 0 &&
                    {
                        cp =
                            strstr(rest_buffer.as_mut_ptr() as
                                       *const libc::c_char,
                                   b"<NE:\x00" as *const u8 as
                                       *const libc::c_char);
                        !cp.is_null()
                    } {
                    ne_flag =
                        sscanf(cp,
                               b"<NE:%[^>]>\x00" as *const u8 as
                                   *const libc::c_char,
                               ne_buffer.as_mut_ptr());
                    if ne_flag != 0 &&
                        strncmp(rest_buffer.as_mut_ptr() as
                                    *const libc::c_char,
                                b"NIL\x00" as *const u8 as
                                    *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong) == 0 {
                        sprintf(rest_buffer.as_mut_ptr() as *mut libc::c_char,
                                b"\"NE:%s\"\x00" as *const u8 as
                                    *const libc::c_char,
                                ne_buffer.as_mut_ptr());
                    } else if ne_flag != 0 &&
                        rest_buffer[0 as libc::c_int as usize] as
                            libc::c_int == '\"' as i32 &&
                        strlen(rest_buffer.as_mut_ptr() as
                            *const libc::c_char).wrapping_add(strlen(ne_buffer.as_mut_ptr()
                            as
                            *const libc::c_char)).wrapping_add(4
                            as
                            libc::c_int
                            as
                            libc::c_ulong)
                            < 5120 as libc::c_int as libc::c_ulong
                        &&
                        {
                            imip =
                                strchr(rest_buffer.as_mut_ptr().offset(1
                                    as
                                    libc::c_int
                                    as
                                    isize)
                                           as *const libc::c_char,
                                       '\"' as i32);
                            !imip.is_null()
                        } {
                        *imip = '\u{0}' as i32 as libc::c_char;
                        strcat(imip,
                               b" NE:\x00" as *const u8 as
                                   *const libc::c_char);
                        strcat(imip,
                               ne_buffer.as_mut_ptr() as *const libc::c_char);
                        strcat(imip,
                               b"\"\x00" as *const u8 as *const libc::c_char);
                    }
                }
                /* 意味情報をfeatureへ */
                if strncmp(rest_buffer.as_mut_ptr() as *const libc::c_char,
                           b"NIL\x00" as *const u8 as *const libc::c_char,
                           3 as libc::c_int as libc::c_ulong) != 0 {
                    /* 通常 "" で括られている */
                    if rest_buffer[0 as libc::c_int as usize] as libc::c_int
                        == '\"' as i32 {
                        imip =
                            &mut *rest_buffer.as_mut_ptr().offset(1 as
                                libc::c_int
                                as
                                isize)
                                as *mut libc::c_uchar as *mut libc::c_char;
                        cp = strchr(imip, '\"' as i32);
                        if !cp.is_null() {
                            *cp = '\u{0}' as i32 as libc::c_char
                        }
                        /* 疑似代表表記を追加する */
                        if strcmp(Hinshi_str.as_mut_ptr() as
                                      *const libc::c_char,
                                  b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8
                                      as *const libc::c_char) != 0 &&
                            strcmp(Hinshi_str.as_mut_ptr() as
                                       *const libc::c_char,
                                   b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00"
                                       as *const u8 as *const libc::c_char)
                                != 0 &&
                            strcmp(Hinshi_str.as_mut_ptr() as
                                       *const libc::c_char,
                                   b"\xe5\x8a\xa9\xe5\x8b\x95\xe8\xa9\x9e\x00"
                                       as *const u8 as *const libc::c_char)
                                != 0 &&
                            strcmp(Hinshi_str.as_mut_ptr() as
                                       *const libc::c_char,
                                   b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as
                                       *const u8 as *const libc::c_char) !=
                                0 &&
                            strstr(imip,
                                   b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                       as *const u8 as
                                       *const libc::c_char).is_null() {
                            sprintf((*m_ptr).Imi.as_mut_ptr(),
                                    b"\"%s\"\x00" as *const u8 as
                                        *const libc::c_char,
                                    imip); /* make_mrph_rn()における参照用 */
                            rep_buf = make_mrph_rn(m_ptr);
                            if strlen(imip).wrapping_add(strlen(b" \xe7\x96\x91\xe4\xbc\xbc\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98 \xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                as
                                *const u8
                                as
                                *const libc::c_char)).wrapping_add(strlen(rep_buf)).wrapping_add(2
                                as
                                libc::c_int
                                as
                                libc::c_ulong)
                                < 5120 as libc::c_int as libc::c_ulong {
                                strcat(imip,
                                       b" \xe7\x96\x91\xe4\xbc\xbc\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98 \xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                                strcat(imip, rep_buf);
                            }
                            free(rep_buf as *mut libc::c_void);
                        }
                        sprintf((*m_ptr).Imi.as_mut_ptr(),
                                b"\"%s\"\x00" as *const u8 as
                                    *const libc::c_char, imip);
                    } else {
                        imip = rest_buffer.as_mut_ptr() as *mut libc::c_char;
                        cp = strchr(imip, ' ' as i32);
                        if !cp.is_null() {
                            *cp = '\u{0}' as i32 as libc::c_char
                        }
                        strcpy((*m_ptr).Imi.as_mut_ptr(), imip);
                    }
                    imi2feature(imip, m_ptr);
                } else {
                    /* 意味情報がNILのとき */
                    /* 疑似代表表記を追加する */
                    rep_buf = make_mrph_rn(m_ptr);
                    if strcmp(Hinshi_str.as_mut_ptr() as *const libc::c_char,
                              b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                                  *const libc::c_char) != 0 &&
                        strcmp(Hinshi_str.as_mut_ptr() as
                                   *const libc::c_char,
                               b"\xe5\x88\xa4\xe5\xae\x9a\xe8\xa9\x9e\x00"
                                   as *const u8 as *const libc::c_char) !=
                            0 &&
                        strcmp(Hinshi_str.as_mut_ptr() as
                                   *const libc::c_char,
                               b"\xe5\x8a\xa9\xe5\x8b\x95\xe8\xa9\x9e\x00"
                                   as *const u8 as *const libc::c_char) !=
                            0 &&
                        strcmp(Hinshi_str.as_mut_ptr() as
                                   *const libc::c_char,
                               b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8
                                   as *const libc::c_char) != 0 &&
                        strlen(b" \xe7\x96\x91\xe4\xbc\xbc\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98 \xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                            as *const u8 as
                            *const libc::c_char).wrapping_add(strlen(rep_buf)).wrapping_add(1
                            as
                            libc::c_int
                            as
                            libc::c_ulong)
                            < 5120 as libc::c_int as libc::c_ulong {
                        imip = rest_buffer.as_mut_ptr() as *mut libc::c_char;
                        *imip = '\u{0}' as i32 as libc::c_char;
                        strcat(imip,
                               b"\xe7\x96\x91\xe4\xbc\xbc\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98 \xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                                   as *const u8 as *const libc::c_char);
                        strcat(imip, rep_buf);
                        sprintf((*m_ptr).Imi.as_mut_ptr(),
                                b"\"%s\"\x00" as *const u8 as
                                    *const libc::c_char, imip);
                        imi2feature(imip, m_ptr);
                    } else {
                        strcpy((*m_ptr).Imi.as_mut_ptr(),
                               b"NIL\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    free(rep_buf as *mut libc::c_void);
                }
            } else if mrph_item == 11 as libc::c_int {
                strcpy((*m_ptr).Imi.as_mut_ptr(),
                       b"NIL\x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(stderr,
                        b";; Invalid input (%d items)<%s> !\n\x00" as
                            *const u8 as *const libc::c_char, mrph_item,
                        input_buffer.as_mut_ptr());
                if !(*sp).Comment.is_null() {
                    fprintf(stderr,
                            b"(%s)\n\x00" as *const u8 as *const libc::c_char,
                            (*sp).Comment);
                }
                return readtoeos(fp);
            }
            if OptInput & 1 as libc::c_int != 0 {
                (*m_ptr).Hinshi = get_hinsi_id(Hinshi_str.as_mut_ptr());
                (*m_ptr).Bunrui = get_bunrui_id(Bunrui_str.as_mut_ptr(), (*m_ptr).Hinshi);
                (*m_ptr).Katuyou_Kata = get_type_id(Katuyou_Kata_str.as_mut_ptr());
                (*m_ptr).Katuyou_Kei = get_form_id(Katuyou_Kei_str.as_mut_ptr(), (*m_ptr).Katuyou_Kata);
            }
            if homo_flag != 0 { homo_num += 1 }
            (*sp).Mrph_num += 1;
            m_ptr = m_ptr.offset(1)
        }
    };
}
/* clear_feature(&(m_ptr->f)); 
	       mainの文ごとのループの先頭で処理に移動 */
/* 同形異義語は一旦 sp->mrph_data にいれる */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_one_mrph_imi(mut m_ptr: *mut MRPH_DATA)
/*==================================================================*/
{
    let mut org_buffer: [libc::c_char; 5120] = [0; 5120];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* もとの形態素情報を意味情報およびfeatureとして保存 */
    sprintf(org_buffer.as_mut_ptr(),
            b"\xe5\x93\x81\xe8\xa9\x9e\xe5\xa4\x89\xe6\x9b\xb4:%s-%s-%s-%d-%d-%d-%d-%s\x00"
                as *const u8 as *const libc::c_char,
            (*m_ptr).Goi2.as_mut_ptr(), (*m_ptr).Yomi.as_mut_ptr(),
            (*m_ptr).Goi.as_mut_ptr(), (*m_ptr).Hinshi, (*m_ptr).Bunrui,
            (*m_ptr).Katuyou_Kata, (*m_ptr).Katuyou_Kei,
            (*m_ptr).Imi.as_mut_ptr()); /* featureへ */
    assign_cfeature(&mut (*m_ptr).f, org_buffer.as_mut_ptr(),
                    0 as libc::c_int);
    cp = strrchr((*m_ptr).Imi.as_mut_ptr(), '\"' as i32);
    if !cp.is_null() {
        *cp = '\u{0}' as i32 as libc::c_char;
        sprintf(org_buffer.as_mut_ptr(),
                b" \xe5\x93\x81\xe8\xa9\x9e\xe5\xa4\x89\xe6\x9b\xb4:%s-%s-%s-%d-%d-%d-%d\"\x00"
                    as *const u8 as *const libc::c_char,
                (*m_ptr).Goi2.as_mut_ptr(), (*m_ptr).Yomi.as_mut_ptr(),
                (*m_ptr).Goi.as_mut_ptr(), (*m_ptr).Hinshi, (*m_ptr).Bunrui,
                (*m_ptr).Katuyou_Kata, (*m_ptr).Katuyou_Kei);
        strcat((*m_ptr).Imi.as_mut_ptr(), org_buffer.as_mut_ptr());
        /* Imiへ */
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_one_mrph_rep(mut m_ptr: *mut MRPH_DATA,
                                             mut modify_feature_flag:
                                             libc::c_int,
                                             mut suffix_char: libc::c_char)
/*==================================================================*/
{
    // let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut pre: [libc::c_char; 1024] = [0; 1024];
    let mut str1: [libc::c_char; 1024] = [0; 1024];
    let mut str2: [libc::c_char; 1024] = [0; 1024];
    let mut post: [libc::c_char; 1024] = [0; 1024];
    let mut orig_rep: [libc::c_char; 1024] = [0; 1024];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 「代表表記:動く/うごく」->「代表表記:動き/うごきv」 */
    /* 活用する品詞ではない場合、または、すでに一度代表表記が変更されている場合 */
    if (*m_ptr).Katuyou_Kata == 0 as libc::c_int ||
        (*m_ptr).Katuyou_Kei == 0 as libc::c_int ||
        !check_feature((*m_ptr).f,
                       b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        return;
    }
    cp =
        strstr((*m_ptr).Imi.as_mut_ptr(),
               b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as
                   *const u8 as *const libc::c_char);
    if !cp.is_null() {
        cp =
            cp.offset(strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        sscanf(cp, b"%[^/]\x00" as *const u8 as *const libc::c_char,
               str1.as_mut_ptr());
        pre[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        strncat(pre.as_mut_ptr(), (*m_ptr).Imi.as_mut_ptr(),
                cp.wrapping_offset_from((*m_ptr).Imi.as_mut_ptr()) as
                    libc::c_long as libc::c_ulong);
        offset =
            strlen(str1.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                libc::c_ulong) as
                libc::c_int;
        sscanf(cp.offset(offset as isize),
               b"%[^ \"]\x00" as *const u8 as *const libc::c_char,
               str2.as_mut_ptr());
        post[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        offset =
            (offset as libc::c_ulong).wrapping_add(strlen(str2.as_mut_ptr()))
                as libc::c_int as libc::c_int;
        strcat(post.as_mut_ptr(), cp.offset(offset as isize));
        strcpy(orig_rep.as_mut_ptr(),
               b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00"
                   as *const u8 as *const libc::c_char);
        strncat(orig_rep.as_mut_ptr(), cp, offset as libc::c_ulong);
        /* もとの代表表記を保持 */
    } else { return; }
    /* 語幹にする */
    str1[strlen(str1.as_mut_ptr()).wrapping_sub(strlen(Form[
        (*m_ptr).Katuyou_Kata as usize
        ][
        get_form_id(b"\xe5\x9f\xba\xe6\x9c\xac\xe5\xbd\xa2\x00" as *const u8 as *mut libc::c_uchar,
                    (*m_ptr).Katuyou_Kata)
        as
        usize].gobi
        as
        *const libc::c_char))
        as usize] = '\u{0}' as i32 as libc::c_char;
    str2[strlen(str2.as_mut_ptr()).wrapping_sub(strlen(Form[
        (*m_ptr).Katuyou_Kata as usize
        ][
        get_form_id(b"\xe5\x9f\xba\xe6\x9c\xac\xe5\xbd\xa2\x00" as *const u8 as *mut libc::c_uchar,
                    (*m_ptr).Katuyou_Kata)
        as
        usize].gobi
        as
        *const libc::c_char))
        as usize] = '\u{0}' as i32 as libc::c_char;
    /* 活用形をつける */
    strcat(str1.as_mut_ptr(),
           Form[(*m_ptr).Katuyou_Kata as
               usize][(*m_ptr).Katuyou_Kei as usize].gobi as
               *const libc::c_char);
    strcat(str2.as_mut_ptr(),
           Form[(*m_ptr).Katuyou_Kata as
               usize][(*m_ptr).Katuyou_Kei as usize].gobi as
               *const libc::c_char);
    /* 意味情報の修正: 修正した代表表記ともとの代表表記 */
    if strlen(pre.as_mut_ptr()).wrapping_add(strlen(str1.as_mut_ptr())).wrapping_add(strlen(str2.as_mut_ptr())).wrapping_add(strlen(orig_rep.as_mut_ptr())).wrapping_add(strlen(post.as_mut_ptr())).wrapping_add(4
        as
        libc::c_int
        as
        libc::c_ulong)
        <= 1024 as libc::c_int as libc::c_ulong {
        sprintf((*m_ptr).Imi.as_mut_ptr(),
                b"%s%s/%s%c %s%s\x00" as *const u8 as *const libc::c_char,
                pre.as_mut_ptr(), str1.as_mut_ptr(), str2.as_mut_ptr(),
                suffix_char as libc::c_int, orig_rep.as_mut_ptr(),
                post.as_mut_ptr());
    }
    /* featureの修正 */
    if modify_feature_flag != 0 {
        cp =
            check_feature((*m_ptr).f,
                          b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            /* もとの代表表記をfeatureに保存 */
            cp =
                cp.offset(strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                    as *const u8 as *const libc::c_char) as
                    isize); /* 新しい代表表記をfeatureへ */
            sprintf(pre.as_mut_ptr(),
                    b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:%s\x00"
                        as *const u8 as *const libc::c_char, cp);
            assign_cfeature(&mut (*m_ptr).f, pre.as_mut_ptr(),
                            0 as libc::c_int);
        }
        sprintf(pre.as_mut_ptr(),
                b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:%s/%s%c\x00"
                    as *const u8 as *const libc::c_char, str1.as_mut_ptr(),
                str2.as_mut_ptr(), suffix_char as libc::c_int);
        assign_cfeature(&mut (*m_ptr).f, pre.as_mut_ptr(), 0 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_one_mrph(mut m_ptr: *mut MRPH_DATA,
                                         mut f: *mut FEATURE)
/*==================================================================*/
{
    let mut h_buffer: [libc::c_char; 62] = [0; 62];
    let mut b_buffer: [libc::c_char; 62] = [0; 62];
    let mut kata_buffer: [libc::c_char; 62] = [0; 62];
    let mut kei_buffer: [libc::c_char; 62] = [0; 62];
    let mut num: libc::c_int = 0;
    (*m_ptr).Hinshi = 0 as libc::c_int;
    (*m_ptr).Bunrui = 0 as libc::c_int;
    (*m_ptr).Katuyou_Kata = 0 as libc::c_int;
    (*m_ptr).Katuyou_Kei = 0 as libc::c_int;
    num = sscanf(
        (*f).cp,
        b"%*[^:]:%[^:]:%[^:]:%[^:]:%[^:]\x00" as *const u8 as *const libc::c_char, h_buffer.as_mut_ptr(),
        b_buffer.as_mut_ptr(), kata_buffer.as_mut_ptr(),
        kei_buffer.as_mut_ptr(),
    );
    (*m_ptr).Hinshi = get_hinsi_id(h_buffer.as_mut_ptr() as *mut libc::c_uchar);
    if num >= 2 as libc::c_int {
        if strcmp(b_buffer.as_mut_ptr(), b"*\x00" as *const u8 as *const libc::c_char) == 0 {
            (*m_ptr).Bunrui = 0 as libc::c_int
        } else {
            (*m_ptr).Bunrui = get_bunrui_id(b_buffer.as_mut_ptr() as *mut libc::c_uchar, (*m_ptr).Hinshi)
        }
    }
    if num >= 3 as libc::c_int {
        (*m_ptr).Katuyou_Kata = get_type_id(kata_buffer.as_mut_ptr() as *mut libc::c_uchar);
        (*m_ptr).Katuyou_Kei = get_form_id(kei_buffer.as_mut_ptr() as *mut libc::c_uchar, (*m_ptr).Katuyou_Kata)
    }
    /* 品詞変更が活用なしの場合は原型も変更する */
    /* ▼ 逆(活用なし→活用あり)は扱っていない */
    if (*m_ptr).Katuyou_Kata == 0 as libc::c_int {
        strcpy((*m_ptr).Goi.as_mut_ptr(), (*m_ptr).Goi2.as_mut_ptr());
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_alt_mrph(mut m_ptr: *mut MRPH_DATA,
                                         mut f: *mut FEATURE)
/*==================================================================*/
{
    let mut fpp: *mut *mut FEATURE = &mut (*m_ptr).f;
    let mut ret_fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut m: MRPH_DATA =
        MRPH_DATA {
            type_0: 0,
            num: 0,
            parent: 0 as *mut tnode_b,
            child: [0 as *mut tnode_b; 32],
            length: 0,
            space: 0,
            dpnd_head: 0,
            dpnd_type: 0,
            dpnd_dflt: 0,
            para_num: 0,
            para_key_type: 0,
            para_top_p: 0,
            para_type: 0,
            to_para_p: 0,
            tnum: 0,
            inum: 0,
            out_head_flag: 0,
            Goi: [0; 129],
            Yomi: [0; 129],
            Goi2: [0; 129],
            Hinshi: 0,
            Bunrui: 0,
            Katuyou_Kata: 0,
            Katuyou_Kei: 0,
            Imi: [0; 1024],
            f: 0 as *mut _FEATURE,
            Num: 0,
            SM: 0 as *mut libc::c_char,
            Pos: [0; 4],
            Type: [0; 9],
        };
    /* ALT中の「代表表記:動く/うごく」->「代表表記:動き/うごきv」 */
    m.f = 0 as FEATUREptr; /* 古いALTは削除 */
    while !(*fpp).is_null() {
        if strncmp((**fpp).cp,
                   b"ALT-\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 {
            sscanf((**fpp).cp.offset(4 as libc::c_int as isize),
                   b"%[^-]-%[^-]-%[^-]-%d-%d-%d-%d-%[^\n]\x00" as *const u8 as
                       *const libc::c_char, m.Goi2.as_mut_ptr(),
                   m.Yomi.as_mut_ptr(), m.Goi.as_mut_ptr(),
                   &mut m.Hinshi as *mut libc::c_int,
                   &mut m.Bunrui as *mut libc::c_int,
                   &mut m.Katuyou_Kata as *mut libc::c_int,
                   &mut m.Katuyou_Kei as *mut libc::c_int,
                   m.Imi.as_mut_ptr());
            change_one_mrph_imi(&mut m);
            change_one_mrph_rep(&mut m, 0 as libc::c_int,
                                'v' as i32 as libc::c_char);
            change_one_mrph(&mut m, f);
            assign_feature_alt_mrph(&mut ret_fp, &mut m);
            free((**fpp).cp as *mut libc::c_void);
            *fpp = (**fpp).next
        } else { fpp = &mut (**fpp).next }
    }
    /* 新しいALT */
    if !ret_fp.is_null() { append_feature(&mut (*m_ptr).f, ret_fp); };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn change_mrph(mut m_ptr: *mut MRPH_DATA,
                                     mut f: *mut FEATURE)
/*==================================================================*/
{
    change_one_mrph_imi(m_ptr); /* 意味情報、featureを修正 */
    change_one_mrph_rep(m_ptr, 1 as libc::c_int,
                        'v' as i32 as
                            libc::c_char); /* 代表表記を修正 */
    change_one_mrph(m_ptr, f); /* 品詞などを修正 */
    change_alt_mrph(m_ptr, f);
    /* ALTの中も修正 */
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_Bunrui(mut cp: *mut libc::c_char) -> libc::c_int
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while !Class[6 as libc::c_int as usize][j as usize].id.is_null() {
        if strcmp(Class[6 as libc::c_int as usize][j as usize].id as
                      *const libc::c_char, cp) == 0 {
            return j;
        }
        j += 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn break_feature(mut fp: *mut FEATURE) -> libc::c_int
/*==================================================================*/
{
    while !fp.is_null() {
        if strcmp((*fp).cp,
                  b"&break:normal\x00" as *const u8 as *const libc::c_char) ==
            0 {
            return 1 as libc::c_int;
        } else {
            if strcmp((*fp).cp,
                      b"&break:jump\x00" as *const u8 as *const libc::c_char)
                == 0 {
                return 2 as libc::c_int;
            } else {
                if strncmp((*fp).cp,
                           b"&break\x00" as *const u8 as *const libc::c_char,
                           strlen(b"&break\x00" as *const u8 as
                               *const libc::c_char)) == 0 {
                    return 1 as libc::c_int;
                }
            }
        }
        fp = (*fp).next
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_mrph_feature(mut s_r_ptr: *mut MrphRule,
                                             mut r_size: libc::c_int,
                                             mut s_m_ptr: *mut MRPH_DATA,
                                             mut m_length: libc::c_int,
                                             mut mode: libc::c_int,
                                             mut break_mode: libc::c_int,
                                             mut direction: libc::c_int,
                                             mut also_assign_flag: libc::c_int,
                                             mut temp_assign_flag: libc::c_int)
/*==================================================================*/
{
    /* ある範囲(文全体,文節内など)に対して形態素のマッチングを行う */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut match_length: libc::c_int = 0;
    let mut feature_break_mode: libc::c_int = 0;
    let mut r_ptr: *mut MrphRule = 0 as *mut MrphRule;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    /* 逆方向に適用する場合はデータのおしりをさしておく必要がある */
    if direction == -(1 as libc::c_int) {
        s_m_ptr = s_m_ptr.offset((m_length - 1 as libc::c_int) as isize)
    }
    /* MRM
       	1.self_patternの先頭の形態素位置
	  2.ルール
	    3.self_patternの末尾の形態素位置
	の順にループが回る (3のループはregexpmrphrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL
	    2のレベルでbreakする
	break_mode == RLOOP_BREAK_JUMP
	    2のレベルでbreakし，self_pattern長だけ1のループを進める
     */
    if mode == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < m_length {
            r_ptr = s_r_ptr;
            m_ptr = s_m_ptr.offset((i * direction) as isize);
            j = 0 as libc::c_int;
            while j < r_size {
                match_length =
                    regexpmrphrule_match(r_ptr, m_ptr,
                                         (if direction == 1 as libc::c_int {
                                             i
                                         } else {
                                             (m_length - i) -
                                                 1 as libc::c_int
                                         }),
                                         (if direction == 1 as libc::c_int {
                                             (m_length) - i
                                         } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_m_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_m_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        k += 1
                    }
                    feature_break_mode = break_feature((*r_ptr).f);
                    if break_mode == 1 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int {
                        break;
                    }
                    if break_mode == 2 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        i += match_length - 1 as libc::c_int;
                        break;
                    }
                }
                j += 1;
                r_ptr = r_ptr.offset(1)
            }
            i += 1
        }
    } else if mode == 1 as libc::c_int {
        r_ptr = s_r_ptr;
        j = 0 as libc::c_int;
        while j < r_size {
            feature_break_mode = break_feature((*r_ptr).f);
            i = 0 as libc::c_int;
            while i < m_length {
                m_ptr = s_m_ptr.offset((i * direction) as isize);
                match_length =
                    regexpmrphrule_match(r_ptr, m_ptr,
                                         (if direction == 1 as libc::c_int {
                                             i
                                         } else {
                                             (m_length - i) -
                                                 1 as libc::c_int
                                         }),
                                         (if direction == 1 as libc::c_int {
                                             (m_length) - i
                                         } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_m_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_m_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        k += 1
                    }
                    if break_mode == 1 as libc::c_int ||
                        break_mode == 2 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        break;
                    }
                }
                i += 1
            }
            j += 1;
            r_ptr = r_ptr.offset(1)
        }
    };
}
/* RMM
       	1.ルール
	  2.self_patternの先頭の形態素位置
	    3.self_patternの末尾の形態素位置
	の順にループが回る (3のループはregexpmrphrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL||RLOOP_BREAK_JUMP
	    2のレベルでbreakする (※この使い方は考えにくいが)
    */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_tag_feature(mut s_r_ptr: *mut BnstRule,
                                            mut r_size: libc::c_int,
                                            mut s_b_ptr: *mut TAG_DATA,
                                            mut b_length: libc::c_int,
                                            mut mode: libc::c_int,
                                            mut break_mode: libc::c_int,
                                            mut direction: libc::c_int,
                                            mut also_assign_flag: libc::c_int,
                                            mut temp_assign_flag:
                                            libc::c_int)
/*==================================================================*/
{
    /* ある範囲(文全体,文節内など)に対してタグ単位のマッチングを行う */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut match_length: libc::c_int = 0;
    let mut feature_break_mode: libc::c_int = 0;
    let mut r_ptr: *mut BnstRule = 0 as *mut BnstRule;
    let mut b_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* 逆方向に適用する場合はデータのおしりをさしておく必要がある */
    if direction == -(1 as libc::c_int) {
        s_b_ptr = s_b_ptr.offset((b_length - 1 as libc::c_int) as isize)
    }
    /* MRM
       	1.self_patternの先頭の文節位置
	  2.ルール
	    3.self_patternの末尾の文節位置
	の順にループが回る (3のループはregexpbnstrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL
	    2のレベルでbreakする
	break_mode == RLOOP_BREAK_JUMP
	    2のレベルでbreakし，self_pattern長だけ1のループを進める
     */
    if mode == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < b_length {
            r_ptr = s_r_ptr;
            b_ptr = s_b_ptr.offset((i * direction) as isize);
            j = 0 as libc::c_int;
            while j < r_size {
                match_length =
                    regexptagrule_match(r_ptr, b_ptr,
                                        (if direction == 1 as libc::c_int {
                                            i
                                        } else {
                                            (b_length - i) - 1 as libc::c_int
                                        }),
                                        (if direction == 1 as libc::c_int {
                                            (b_length) - i
                                        } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_b_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_b_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        if also_assign_flag != 0 {
                            /* 属する文節にも付与する場合 */
                            assign_feature(&mut (*(*s_b_ptr.offset((i *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).b_ptr).f,
                                           &mut (*r_ptr).f,
                                           s_b_ptr.offset((i * direction) as
                                               isize) as
                                               *mut libc::c_void, k,
                                           match_length - k,
                                           temp_assign_flag);
                        }
                        k += 1
                    }
                    feature_break_mode = break_feature((*r_ptr).f);
                    if break_mode == 1 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int {
                        break;
                    }
                    if break_mode == 2 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        i += match_length - 1 as libc::c_int;
                        break;
                    }
                }
                j += 1;
                r_ptr = r_ptr.offset(1)
            }
            i += 1
        }
    } else if mode == 1 as libc::c_int {
        r_ptr = s_r_ptr;
        j = 0 as libc::c_int;
        while j < r_size {
            feature_break_mode = break_feature((*r_ptr).f);
            i = 0 as libc::c_int;
            while i < b_length {
                b_ptr = s_b_ptr.offset((i * direction) as isize);
                match_length =
                    regexptagrule_match(r_ptr, b_ptr,
                                        (if direction == 1 as libc::c_int {
                                            i
                                        } else {
                                            (b_length - i) - 1 as libc::c_int
                                        }),
                                        (if direction == 1 as libc::c_int {
                                            (b_length) - i
                                        } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_b_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_b_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        if also_assign_flag != 0 {
                            /* RMM
       	1.ルール
	  2.self_patternの先頭の文節位置
	    3.self_patternの末尾の文節位置
	の順にループが回る (3のループはregexpbnstrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL||RLOOP_BREAK_JUMP
	    2のレベルでbreakする (※この使い方は考えにくいが)
    */
                            /* 属する文節にも付与する場合 */
                            assign_feature(&mut (*(*s_b_ptr.offset((i *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).b_ptr).f,
                                           &mut (*r_ptr).f,
                                           s_b_ptr.offset((i * direction) as
                                               isize) as
                                               *mut libc::c_void, k,
                                           match_length - k,
                                           temp_assign_flag);
                        }
                        k += 1
                    }
                    if break_mode == 1 as libc::c_int ||
                        break_mode == 2 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        break;
                    }
                }
                i += 1
            }
            j += 1;
            r_ptr = r_ptr.offset(1)
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_bnst_feature(mut s_r_ptr: *mut BnstRule,
                                             mut r_size: libc::c_int,
                                             mut s_b_ptr: *mut BNST_DATA,
                                             mut b_length: libc::c_int,
                                             mut mode: libc::c_int,
                                             mut break_mode: libc::c_int,
                                             mut direction: libc::c_int,
                                             mut also_assign_flag:
                                             libc::c_int,
                                             mut temp_assign_flag:
                                             libc::c_int)
/*==================================================================*/
{
    /* ある範囲(文全体,文節内など)に対して文節のマッチングを行う */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut match_length: libc::c_int = 0;
    let mut feature_break_mode: libc::c_int = 0;
    let mut r_ptr: *mut BnstRule = 0 as *mut BnstRule;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    /* 逆方向に適用する場合はデータのおしりをさしておく必要がある */
    if direction == -(1 as libc::c_int) {
        s_b_ptr = s_b_ptr.offset((b_length - 1 as libc::c_int) as isize)
    }
    /* MRM
       	1.self_patternの先頭の文節位置
	  2.ルール
	    3.self_patternの末尾の文節位置
	の順にループが回る (3のループはregexpbnstrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL
	    2のレベルでbreakする
	break_mode == RLOOP_BREAK_JUMP
	    2のレベルでbreakし，self_pattern長だけ1のループを進める
     */
    if mode == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < b_length {
            r_ptr = s_r_ptr;
            b_ptr = s_b_ptr.offset((i * direction) as isize);
            j = 0 as libc::c_int;
            while j < r_size {
                match_length =
                    regexpbnstrule_match(r_ptr, b_ptr,
                                         (if direction == 1 as libc::c_int {
                                             i
                                         } else {
                                             (b_length - i) -
                                                 1 as libc::c_int
                                         }),
                                         (if direction == 1 as libc::c_int {
                                             (b_length) - i
                                         } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_b_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_b_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        if also_assign_flag != 0 {
                            /* headのタグ単位にも付与する場合 */
                            assign_feature(&mut (*(*s_b_ptr.offset((i *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).tag_ptr.offset((*s_b_ptr.offset((i
                                *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).tag_num
                                as
                                isize).offset(-(1
                                as
                                libc::c_int
                                as
                                isize))).f,
                                           &mut (*r_ptr).f,
                                           s_b_ptr.offset((i * direction) as
                                               isize) as
                                               *mut libc::c_void, k,
                                           match_length - k,
                                           temp_assign_flag);
                        }
                        k += 1
                    }
                    feature_break_mode = break_feature((*r_ptr).f);
                    if break_mode == 1 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int {
                        break;
                    }
                    if break_mode == 2 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        i += match_length - 1 as libc::c_int;
                        break;
                    }
                }
                j += 1;
                r_ptr = r_ptr.offset(1)
            }
            i += 1
        }
    } else if mode == 1 as libc::c_int {
        r_ptr = s_r_ptr;
        j = 0 as libc::c_int;
        while j < r_size {
            feature_break_mode = break_feature((*r_ptr).f);
            i = 0 as libc::c_int;
            while i < b_length {
                b_ptr = s_b_ptr.offset((i * direction) as isize);
                match_length =
                    regexpbnstrule_match(r_ptr, b_ptr,
                                         (if direction == 1 as libc::c_int {
                                             i
                                         } else {
                                             (b_length - i) -
                                                 1 as libc::c_int
                                         }),
                                         (if direction == 1 as libc::c_int {
                                             (b_length) - i
                                         } else { (i) + 1 as libc::c_int }));
                if match_length != -(1 as libc::c_int) {
                    k = 0 as libc::c_int;
                    while k < match_length {
                        assign_feature(&mut (*s_b_ptr.offset((i * direction)
                            as
                            isize).offset(k
                            as
                            isize)).f,
                                       &mut (*r_ptr).f,
                                       s_b_ptr.offset((i * direction) as
                                           isize) as
                                           *mut libc::c_void, k,
                                       match_length - k, temp_assign_flag);
                        if also_assign_flag != 0 {
                            /* RMM
       	1.ルール
	  2.self_patternの先頭の文節位置
	    3.self_patternの末尾の文節位置
	の順にループが回る (3のループはregexpbnstrule_matchの中)
	
	break_mode == RLOOP_BREAK_NORMAL||RLOOP_BREAK_JUMP
	    2のレベルでbreakする (※この使い方は考えにくいが)
    */
                            /* headのタグ単位にも付与する場合 */
                            assign_feature(&mut (*(*s_b_ptr.offset((i *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).tag_ptr.offset((*s_b_ptr.offset((i
                                *
                                direction)
                                as
                                isize).offset(k
                                as
                                isize)).tag_num
                                as
                                isize).offset(-(1
                                as
                                libc::c_int
                                as
                                isize))).f,
                                           &mut (*r_ptr).f,
                                           s_b_ptr.offset((i * direction) as
                                               isize) as
                                               *mut libc::c_void, k,
                                           match_length - k,
                                           temp_assign_flag);
                        }
                        k += 1
                    }
                    if break_mode == 1 as libc::c_int ||
                        break_mode == 2 as libc::c_int ||
                        feature_break_mode == 1 as libc::c_int ||
                        feature_break_mode == 2 as libc::c_int {
                        break;
                    }
                }
                i += 1
            }
            j += 1;
            r_ptr = r_ptr.offset(1)
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_general_feature(mut data: *mut libc::c_void,
                                                mut size: libc::c_int,
                                                mut flag: libc::c_int,
                                                mut also_assign_flag:
                                                libc::c_int,
                                                mut temp_assign_flag:
                                                libc::c_int)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut assign_function: Option<unsafe extern "C" fn() -> ()> = None;
    /* 形態素, タグ単位, 文節の場合分け */
    if flag == 1 as libc::c_int || flag == 16 as libc::c_int ||
        flag == 6 as libc::c_int {
        assign_function =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut MrphRule,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                *mut MRPH_DATA,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int)
                                                                -> ()>,
                Option<unsafe extern "C" fn()
                    ->
                    ()>>(Some(assign_mrph_feature
                as
                unsafe extern "C" fn(_:
                                     *mut MrphRule,
                                     _:
                                     libc::c_int,
                                     _:
                                     *mut MRPH_DATA,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int)
                                     -> ()))
    } else if flag == 11 as libc::c_int || flag == 13 as libc::c_int ||
        flag == 14 as libc::c_int {
        assign_function =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut BnstRule,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                *mut TAG_DATA,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int)
                                                                -> ()>,
                Option<unsafe extern "C" fn()
                    ->
                    ()>>(Some(assign_tag_feature
                as
                unsafe extern "C" fn(_:
                                     *mut BnstRule,
                                     _:
                                     libc::c_int,
                                     _:
                                     *mut TAG_DATA,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int)
                                     -> ()))
    } else if flag == 2 as libc::c_int || flag == 12 as libc::c_int {
        assign_function =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut BnstRule,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                *mut BNST_DATA,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int,
                                                                _:
                                                                libc::c_int)
                                                                -> ()>,
                Option<unsafe extern "C" fn()
                    ->
                    ()>>(Some(assign_bnst_feature
                as
                unsafe extern "C" fn(_:
                                     *mut BnstRule,
                                     _:
                                     libc::c_int,
                                     _:
                                     *mut BNST_DATA,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int,
                                     _:
                                     libc::c_int)
                                     -> ()))
    }
    i = 0 as libc::c_int;
    while i < GeneralRuleNum {
        if (*GeneralRuleArray.offset(i as isize)).type_0 == flag {
            ::std::mem::transmute::<_,
                fn(_: _, _: _, _: _, _: _, _: _, _: _,
                   _: _, _: _,
                   _:
                   _)>(assign_function.expect("non-null function pointer"))((*GeneralRuleArray.offset(i
                as
                isize)).RuleArray,
                                                                            (*GeneralRuleArray.offset(i
                                                                                as
                                                                                isize)).CurRuleSize,
                                                                            data,
                                                                            size,
                                                                            (*GeneralRuleArray.offset(i
                                                                                as
                                                                                isize)).mode,
                                                                            (*GeneralRuleArray.offset(i
                                                                                as
                                                                                isize)).breakmode,
                                                                            (*GeneralRuleArray.offset(i
                                                                                as
                                                                                isize)).direction,
                                                                            also_assign_flag,
                                                                            temp_assign_flag);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_bnst(mut sp: *mut SENTENCE_DATA,
                                   mut m_ptr: *mut MRPH_DATA)
                                   -> *mut BNST_DATA
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    b_ptr = (*sp).bnst_data.offset((*sp).Bnst_num as isize);
    (*b_ptr).type_0 = 1 as libc::c_int;
    (*b_ptr).num = (*sp).Bnst_num;
    (*sp).Bnst_num += 1;
    if (*sp).Bnst_num > 200 as libc::c_int {
        fprintf(stderr,
                b";; Too many bnst (%s %s%s...)!\n\x00" as *const u8 as
                    *const libc::c_char,
                if !(*sp).Comment.is_null() {
                    (*sp).Comment as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*sp).mrph_data,
                (*sp).mrph_data.offset(1 as libc::c_int as isize));
        (*sp).Bnst_num = 0 as libc::c_int;
        return 0 as *mut BNST_DATA;
    }
    (*b_ptr).mrph_ptr = m_ptr;
    (*b_ptr).mrph_num = 0 as libc::c_int;
    (*b_ptr).BGH_num = 0 as libc::c_int;
    (*b_ptr).SM_num = 0 as libc::c_int;
    (*b_ptr).para_key_type = 0 as libc::c_int as libc::c_char;
    (*b_ptr).para_top_p = 0 as libc::c_int as libc::c_char;
    (*b_ptr).para_type = 0 as libc::c_int as libc::c_char;
    (*b_ptr).to_para_p = 0 as libc::c_int as libc::c_char;
    (*b_ptr).cpm_ptr = 0 as CPM_ptr;
    (*b_ptr).voice = 0 as libc::c_int;
    (*b_ptr).space = 0 as libc::c_int;
    (*b_ptr).pred_b_ptr = 0 as *mut tnode_b;
    i = 0 as libc::c_int;
    cp = (*b_ptr).SCASE_code.as_mut_ptr();
    while i < 11 as libc::c_int {
        *cp = 0 as libc::c_int as libc::c_char;
        i += 1;
        cp = cp.offset(1)
    }
    /* clear_feature(&(b_ptr->f));
       mainの文ごとのループの先頭で処理に移動 */
    return b_ptr;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_Jiritu_Go(mut sp: *mut SENTENCE_DATA, mut ptr: *mut BNST_DATA) {
    let mut mp: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    (*ptr).Jiritu_Go[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* 主辞より前の部分で接頭辞以外を自立語としておいておく */
    mp = (*ptr).mrph_ptr;
    while mp <= (*ptr).head_ptr {
        if check_feature((*mp).f,
                         b"\xe6\x8e\xa5\xe9\xa0\xad\x00" as *const u8 as
                             *const libc::c_char as
                             *mut libc::c_char).is_null() {
            if strlen((*ptr).Jiritu_Go.as_mut_ptr()).wrapping_add(strlen((*mp).Goi.as_mut_ptr())).wrapping_add(2 as libc::c_int as libc::c_ulong) > 256 as libc::c_int as libc::c_ulong {
                if OptDisplay == 3 as libc::c_int {
                    /* warning */
                    fprintf(stderr,
                            b";; Too big bunsetsu (%s %s...)!\n\x00" as
                                *const u8 as *const libc::c_char,
                            if !(*sp).Comment.is_null() {
                                (*sp).Comment as *const libc::c_char
                            } else {
                                b"\x00" as *const u8 as *const libc::c_char
                            }, (*ptr).Jiritu_Go.as_mut_ptr());
                }
                return;
            }
            strcat((*ptr).Jiritu_Go.as_mut_ptr(), (*mp).Goi.as_mut_ptr());
        }
        mp = mp.offset(1)
    };
}

#[no_mangle]
pub unsafe extern "C" fn decide_head_ptr(mut ptr: *mut BNST_DATA) {
    let mut i: libc::c_int = 0;
    if (*ptr).type_0 == 2 as libc::c_int {
        i = (*ptr).mrph_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                              b"\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
                !check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                               b"\xe6\xba\x96\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() {
                (*ptr).head_ptr = (*ptr).mrph_ptr.offset(i as isize);
                return;
            }
            i -= 1
        }
    } else {
        /* 文節のときは形式名詞「の」をheadとしない */
        i = (*ptr).mrph_num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                             b"\xe7\x89\xb9\xe6\xae\x8a\xe9\x9d\x9e\xe8\xa6\x8b\xe5\x87\xba\xe8\xaa\x9e\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                (!check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                                b"\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char).is_null() ||
                    !check_feature((*(*ptr).mrph_ptr.offset(i as
                        isize)).f,
                                   b"\xe6\xba\x96\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null()) {
                (*ptr).head_ptr = (*ptr).mrph_ptr.offset(i as isize);
                assign_cfeature(&mut (*(*ptr).head_ptr).f,
                                b"\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
                return;
            }
            i -= 1
        }
    }
    /* 付属語しかない場合 */
    (*ptr).head_ptr = (*ptr).mrph_ptr;
}

#[no_mangle]
pub unsafe extern "C" fn calc_bnst_length(mut sp: *mut SENTENCE_DATA,
                                          mut b_ptr: *mut BNST_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    let mut current_length: libc::c_int = 0;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    (*b_ptr).length = 0 as libc::c_int;
    j = 0 as libc::c_int;
    m_ptr = (*b_ptr).mrph_ptr;
    while j < (*b_ptr).mrph_num {
        current_length =
            string_length((*m_ptr).Goi2.as_mut_ptr()) * 2 as libc::c_int;
        if (*b_ptr).length + current_length >= 256 as libc::c_int {
            if OptDisplay == 3 as libc::c_int {
                /* warning */
                fprintf(stderr,
                        b";; Too big bunsetsu (%s ...%s...)!\n\x00" as
                            *const u8 as *const libc::c_char,
                        if !(*sp).Comment.is_null() {
                            (*sp).Comment as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, (*m_ptr).Goi2.as_mut_ptr());
            }
            break;
        } else {
            (*b_ptr).length += current_length;
            j += 1;
            m_ptr = m_ptr.offset(1)
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_bunsetsu(mut sp: *mut SENTENCE_DATA)
                                       -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    // let mut j: libc::c_int = 0;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    (*sp).Bnst_num = 0 as libc::c_int;
    (*sp).Max_New_Bnst_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    m_ptr = (*sp).mrph_data;
    while i < (*sp).Mrph_num {
        if !check_feature((*m_ptr).f,
                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            b_ptr = init_bnst(sp, m_ptr);
            if b_ptr.is_null() { return 0 as libc::c_int; }
        }
        (*b_ptr).mrph_num += 1;
        i += 1;
        m_ptr = m_ptr.offset(1)
    }
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        /* initialization for -assignf option */
        (*b_ptr).dpnd_head = 0 as libc::c_int;
        (*b_ptr).dpnd_type = 'D' as i32 as libc::c_char;
        if calc_bnst_length(sp, b_ptr) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_bunsetsu_pm(mut sp: *mut SENTENCE_DATA)
                                          -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut m_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut b_ptr: *mut BNST_DATA = (*sp).bnst_data;
    i = 0 as libc::c_int;
    m_ptr = (*sp).mrph_data;
    while i < (*sp).Mrph_num {
        if Bnst_start[i as usize] != 0 {
            if i != 0 as libc::c_int { b_ptr = b_ptr.offset(1) }
            (*b_ptr).type_0 = 1 as libc::c_int;
            (*b_ptr).num =
                b_ptr.wrapping_offset_from((*sp).bnst_data) as libc::c_long as
                    libc::c_int;
            (*b_ptr).mrph_ptr = m_ptr;
            (*b_ptr).mrph_num = 1 as libc::c_int;
            (*b_ptr).cpm_ptr = 0 as CPM_ptr;
            (*b_ptr).voice = 0 as libc::c_int;
            (*b_ptr).pred_b_ptr = 0 as *mut tnode_b;
            j = 0 as libc::c_int;
            cp = (*b_ptr).SCASE_code.as_mut_ptr();
            while j < 11 as libc::c_int {
                *cp = 0 as libc::c_int as libc::c_char;
                j += 1;
                cp = cp.offset(1)
            }
            /* clear_feature(&(b_ptr->f));
	       mainの文ごとのループの先頭で処理に移動 */
        } else { (*b_ptr).mrph_num += 1 }
        i += 1;
        m_ptr = m_ptr.offset(1)
    }
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if OptReadFeature != 0 { (*b_ptr).f = Input_bnst_feature[i as usize] }
        assign_cfeature(&mut (*b_ptr).f,
                        b"\xe8\xa7\xa3\xe6\x9e\x90\xe6\xb8\x88\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char, 0 as libc::c_int);
        if calc_bnst_length(sp, b_ptr) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn push_tag_units(mut tp: *mut TAG_DATA,
                                        mut mp: *mut MRPH_DATA)
/*==================================================================*/
{
    if !check_feature((*mp).f,
                      b"\xe9\x9d\x9e\xe7\x8b\xac\xe7\xab\x8b\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        if (*tp).settou_num == 0 as libc::c_int { (*tp).settou_ptr = mp }
        (*tp).settou_num += 1
    } else if !check_feature((*mp).f,
                             b"\xe8\x87\xaa\xe7\xab\x8b\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() ||
        !check_feature((*mp).f,
                       b"\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        if (*tp).jiritu_num == 0 as libc::c_int { (*tp).jiritu_ptr = mp }
        (*tp).jiritu_num += 1
    } else {
        if (*tp).fuzoku_num == 0 as libc::c_int { (*tp).fuzoku_ptr = mp }
        (*tp).fuzoku_num += 1
    }
    (*tp).mrph_num += 1;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn after_make_tag_units(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        tp = (*sp).tag_data.offset(i as isize);
        (*tp).type_0 = 2 as libc::c_int;
        decide_head_ptr(tp as *mut BNST_DATA);
        /* initialization for -assignf option */
        (*tp).dpnd_head = 0 as libc::c_int;
        (*tp).dpnd_type = 'D' as i32 as libc::c_char;
        if OptReadFeature != 0 {
            (*tp).f = Input_tag_feature[i as usize];
            read_annotation(sp, tp);
        } else { (*tp).c_cpm_ptr = 0 as CPM_ptr }
        /* BNST_DATAにcastしている tricky? */
        get_bnst_code_all(tp as
            *mut BNST_DATA); /* case_analysis.rule で使っている */
        if (*tp).inum != 0 as libc::c_int {
            assign_cfeature(&mut (*tp).f,
                            b"\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
            assign_cfeature(&mut (*tp).f,
                            b"\xe4\xbf\x82:\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 0 as libc::c_int);
        } else {
            /* headのときは文節のfeatureをコピー */
            /* <文頭>, <文末>もつくが、文頭の文節が2タグ単位以上もつ場合は、
               <文頭>のつく位置が間違っているので下で修正する */
            copy_feature(&mut (*tp).f,
                         (*(*tp).b_ptr).f); /* <サ変>は文節とタグ単位では異なる */
            delete_cfeature(&mut (*tp).f,
                            b"\xe3\x82\xb5\xe5\xa4\x89\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
            /* 形式名詞「の」に用言がコピーされるので削除 */
            if !check_feature((*(*tp).head_ptr).f,
                              b"\xe7\x89\xb9\xe6\xae\x8a\xe9\x9d\x9e\xe8\xa6\x8b\xe5\x87\xba\xe8\xaa\x9e\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                delete_cfeature(&mut (*tp).f,
                                b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
            }
        }
        /* 各タグ単位の長さを計算しておく */
        calc_bnst_length(sp, tp as *mut BNST_DATA);
        i += 1
    }
    /* <文頭>の修正 */
    if (*(*sp).bnst_data).tag_num > 1 as libc::c_int {
        delete_cfeature(&mut (*(*(*sp).bnst_data).tag_ptr.offset((*(*sp).bnst_data).tag_num
            as
            isize).offset(-(1
            as
            libc::c_int
            as
            isize))).f,
                        b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
        assign_cfeature(&mut (*(*sp).tag_data).f,
                        b"\xe6\x96\x87\xe9\xa0\xad\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
    }
    /* タグ単位ルールを適用する */
    assign_general_feature((*sp).tag_data as *mut libc::c_void, (*sp).Tag_num,
                           11 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
    /* NTTコードをfeatureに表示 */
    sm2feature(sp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_mrph_set_inum(mut sp: *mut SENTENCE_DATA,
                                            mut num: libc::c_int)
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    j = num - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        let fresh1 = count;
        count = count + 1;
        (*(*sp).mrph_data.offset(j as isize)).inum = fresh1;
        if (*(*sp).mrph_data.offset(j as isize)).tnum >= 0 as libc::c_int {
            break;
        }
        j -= 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_tag_unit_set_inum(mut sp: *mut SENTENCE_DATA,
                                                mut num: libc::c_int)
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    j = num - 2 as libc::c_int;
    while j >= 0 as libc::c_int {
        count += 1;
        (*(*sp).tag_data.offset(j as isize)).inum = count;
        if (*(*sp).tag_data.offset(j as isize)).bnum >= 0 as libc::c_int {
            break;
        }
        j -= 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_tag_units(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mp: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut tp: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut bp: *mut BNST_DATA = (*sp).bnst_data;
    let mut pre_bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    (*sp).Tag_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        mp = (*sp).mrph_data.offset(i as isize);
        flag =
            check_feature((*mp).f,
                          b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\xa7\x8b\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        /* 文節始まりの形態素だけど<タグ単位始>がついていない場合も許す */
        if !flag.is_null() || !bp.is_null() && (*bp).mrph_ptr == mp {
            tp = (*sp).tag_data.offset((*sp).Tag_num as isize);
            if flag.is_null() {
                assign_cfeature(&mut (*mp).f,
                                b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\xa7\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
            }
            memset(tp as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<TAG_DATA>() as libc::c_ulong);
            (*tp).num = (*sp).Tag_num;
            (*tp).mrph_ptr = mp;
            (*mp).tnum = (*tp).num;
            make_mrph_set_inum(sp, i);
            /* 文節区切りと一致するとき */
            if !bp.is_null() && (*bp).mrph_ptr == (*tp).mrph_ptr {
                /* 遡ってinumを付与 */
                if (*sp).Tag_num > 0 as libc::c_int &&
                    (*tp.offset(-(1 as libc::c_int as isize))).bnum <
                        0 as libc::c_int {
                    make_tag_unit_set_inum(sp,
                                           (*sp).Tag_num); /* タグ単位から文節へマーク */
                } /* 文節からタグ単位へマーク */
                (*tp).bnum = (*bp).num;
                (*tp).b_ptr = bp;
                (*bp).tag_ptr = tp;
                (*bp).tag_num = 1 as libc::c_int;
                pre_bp = bp;
                if (*bp).num < (*sp).Bnst_num - 1 as libc::c_int {
                    bp = bp.offset(1)
                } else {
                    /* 最後の文節が終わった */
                    bp = 0 as *mut BNST_DATA
                }
            } else {
                (*tp).bnum = -(1 as libc::c_int);
                (*tp).b_ptr = pre_bp;
                (*pre_bp).tag_num += 1
            }
            (*sp).Tag_num += 1
        } else { (*mp).tnum = -(1 as libc::c_int) }
        push_tag_units(tp, mp);
        i += 1
    }
    if (*(*sp).tag_data.offset((*sp).Tag_num as
        isize).offset(-(1 as libc::c_int as
        isize))).bnum <
        0 as libc::c_int {
        make_tag_unit_set_inum(sp, (*sp).Tag_num);
    }
    make_mrph_set_inum(sp, (*sp).Mrph_num);
    after_make_tag_units(sp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_tag_units_pm(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut mp: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut tp: *mut TAG_DATA = (*sp).tag_data;
    let mut bp: *mut BNST_DATA = (*sp).bnst_data;
    let mut pre_bp: *mut BNST_DATA = 0 as *mut BNST_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        mp = (*sp).mrph_data.offset(i as isize);
        if Tag_start[i as usize] != 0 {
            if i != 0 as libc::c_int { tp = tp.offset(1) }
            if check_feature((*mp).f,
                             b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\xa7\x8b\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
                assign_cfeature(&mut (*mp).f,
                                b"\xe3\x82\xbf\xe3\x82\xb0\xe5\x8d\x98\xe4\xbd\x8d\xe5\xa7\x8b\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char, 0 as libc::c_int);
            }
            memset(tp as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<TAG_DATA>() as libc::c_ulong);
            (*tp).num =
                tp.wrapping_offset_from((*sp).tag_data) as libc::c_long as
                    libc::c_int;
            (*tp).mrph_ptr = mp;
            (*mp).tnum = (*tp).num;
            make_mrph_set_inum(sp, i);
            /* 文節区切りと一致するとき */
            if !bp.is_null() && (*bp).mrph_ptr == (*tp).mrph_ptr {
                /* 遡ってinumを付与 */
                if (*tp).num > 0 as libc::c_int &&
                    (*tp.offset(-(1 as libc::c_int as isize))).bnum <
                        0 as libc::c_int {
                    make_tag_unit_set_inum(sp,
                                           (*tp).num); /* タグ単位から文節へマーク */
                } /* 文節からタグ単位へマーク */
                (*tp).bnum = (*bp).num;
                (*tp).b_ptr = bp;
                (*bp).tag_ptr = tp;
                (*bp).tag_num = 1 as libc::c_int;
                pre_bp = bp;
                if (*bp).num < (*sp).Bnst_num - 1 as libc::c_int {
                    bp = bp.offset(1)
                } else {
                    /* 最後の文節が終わった */
                    bp = 0 as *mut BNST_DATA
                }
            } else {
                (*tp).bnum = -(1 as libc::c_int);
                (*tp).b_ptr = pre_bp;
                (*pre_bp).tag_num += 1
            }
        } else { (*mp).tnum = -(1 as libc::c_int) }
        push_tag_units(tp, mp);
        i += 1
    }
    if (*(*sp).tag_data.offset((*sp).Tag_num as
        isize).offset(-(1 as libc::c_int as
        isize))).bnum <
        0 as libc::c_int {
        make_tag_unit_set_inum(sp, (*sp).Tag_num);
    }
    make_mrph_set_inum(sp, (*sp).Mrph_num);
    after_make_tag_units(sp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dpnd_info_to_tag_pm(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    /* 係り受けに関する種々の情報を DPND から TAG_DATA にコピー (解析済版) */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        (*(*sp).tag_data.offset(i as isize)).dpnd_head = Tag_dpnd[i as usize];
        (*(*sp).tag_data.offset(i as isize)).dpnd_type =
            Tag_type[i as usize] as libc::c_char;
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn reset_mrph(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut delete_count: libc::c_int = 0 as libc::c_int;
    let mut move_table: [libc::c_int; 200] = [0; 200];
    i = 1 as libc::c_int;
    while i < (*sp).Mrph_num {
        if (*(*sp).mrph_data.offset(i as
            isize)).Goi[0 as libc::c_int as usize]
            as libc::c_int == '\u{0}' as i32 {
            /* マージされてなくなった形態素 */
            move_table[i as usize] = 0 as libc::c_int;
            delete_count += 1
        } else {
            move_table[i as usize] = delete_count
            /* 何個前に移動させるか */
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i < (*sp).Mrph_num {
        if move_table[i as usize] > 0 as libc::c_int {
            /* 移動させるべき形態素 */
            copy_mrph((*sp).mrph_data.offset(i as
                isize).offset(-(move_table[i
                as
                usize]
                as
                isize)),
                      (*sp).mrph_data.offset(i as isize), 0 as libc::c_int);
            /* featureはコピー */
        }
        i += 1
    }
    (*sp).Mrph_num -= delete_count;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn merge_mrph_rep(mut dst: *mut MRPH_DATA,
                                        mut src: *mut MRPH_DATA)
/*==================================================================*/
{
    let mut offset: libc::c_int = 0;
    let mut src_str1: [libc::c_char; 1024] = [0; 1024];
    let mut src_str2: [libc::c_char; 1024] = [0; 1024];
    let mut dst_pre: [libc::c_char; 1024] = [0; 1024];
    let mut dst_str1: [libc::c_char; 1024] = [0; 1024];
    let mut dst_str2: [libc::c_char; 1024] = [0; 1024];
    let mut dst_post: [libc::c_char; 1024] = [0; 1024];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp =
        strstr((*src).Imi.as_mut_ptr(),
               b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as
                   *const u8 as *const libc::c_char);
    if !cp.is_null() {
        /* マージするもの */
        cp =
            cp.offset(strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        sscanf(cp, b"%[^/]\x00" as *const u8 as *const libc::c_char,
               src_str1.as_mut_ptr());
        sscanf(cp.offset(strlen(src_str1.as_mut_ptr()) as
            isize).offset(1 as libc::c_int as isize),
               b"%[^ \"]\x00" as *const u8 as *const libc::c_char,
               src_str2.as_mut_ptr());
    } else { return; }
    cp =
        strstr((*dst).Imi.as_mut_ptr(),
               b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00" as
                   *const u8 as *const libc::c_char);
    if !cp.is_null() {
        /* マージ先 */
        cp =
            cp.offset(strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98:\x00"
                as *const u8 as *const libc::c_char) as
                isize); /* 漢字部分のマージ */
        sscanf(cp, b"%[^/]\x00" as *const u8 as *const libc::c_char,
               dst_str1.as_mut_ptr());
        dst_pre[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        strncat(dst_pre.as_mut_ptr(), (*dst).Imi.as_mut_ptr(),
                cp.wrapping_offset_from((*dst).Imi.as_mut_ptr()) as
                    libc::c_long as libc::c_ulong);
        offset =
            strlen(dst_str1.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                libc::c_ulong) as
                libc::c_int;
        sscanf(cp.offset(offset as isize),
               b"%[^ \"]\x00" as *const u8 as *const libc::c_char,
               dst_str2.as_mut_ptr());
        dst_post[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        offset =
            (offset as
                libc::c_ulong).wrapping_add(strlen(dst_str2.as_mut_ptr())) as
                libc::c_int as libc::c_int;
        strcat(dst_post.as_mut_ptr(), cp.offset(offset as isize));
    } else { return; }
    if strlen(dst_str1.as_mut_ptr()).wrapping_add(strlen(src_str1.as_mut_ptr()))
        < 1024 as libc::c_int as libc::c_ulong &&
        strlen(dst_str2.as_mut_ptr()).wrapping_add(strlen(src_str2.as_mut_ptr()))
            < 1024 as libc::c_int as libc::c_ulong {
        strcat(dst_str1.as_mut_ptr(), src_str1.as_mut_ptr());
        strcat(dst_str2.as_mut_ptr(), src_str2.as_mut_ptr());
        /* 読み部分のマージ */
    } else { return; }
    /* 意味情報の修正 */
    if strlen(dst_pre.as_mut_ptr()).wrapping_add(strlen(dst_str1.as_mut_ptr())).wrapping_add(strlen(dst_str2.as_mut_ptr())).wrapping_add(strlen(dst_post.as_mut_ptr())).wrapping_add(2
        as
        libc::c_int
        as
        libc::c_ulong)
        <= 1024 as libc::c_int as libc::c_ulong {
        sprintf((*dst).Imi.as_mut_ptr(),
                b"%s%s/%s%s\x00" as *const u8 as *const libc::c_char,
                dst_pre.as_mut_ptr(), dst_str1.as_mut_ptr(),
                dst_str2.as_mut_ptr(), dst_post.as_mut_ptr());
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn merge_mrph(mut sp: *mut SENTENCE_DATA,
                                    mut start_num: libc::c_int,
                                    mut length: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut goi_length: libc::c_int = 0 as libc::c_int;
    let mut yomi_length: libc::c_int = 0 as libc::c_int;
    let mut goi2_length: libc::c_int = 0 as libc::c_int;
    /* 先頭の形態素にマージ */
    /* まず、マージ後の長さをチェック */
    i = 0 as libc::c_int;
    while i < length {
        goi_length =
            (goi_length as
                libc::c_ulong).wrapping_add(strlen((*(*sp).mrph_data.offset(start_num
                as
                isize).offset(i
                as
                isize)).Goi.as_mut_ptr()))
                as libc::c_int as libc::c_int;
        yomi_length =
            (yomi_length as
                libc::c_ulong).wrapping_add(strlen((*(*sp).mrph_data.offset(start_num
                as
                isize).offset(i
                as
                isize)).Yomi.as_mut_ptr()))
                as libc::c_int as libc::c_int;
        goi2_length =
            (goi2_length as
                libc::c_ulong).wrapping_add(strlen((*(*sp).mrph_data.offset(start_num
                as
                isize).offset(i
                as
                isize)).Goi2.as_mut_ptr()))
                as libc::c_int as libc::c_int;
        i += 1
    }
    if goi_length > 128 as libc::c_int || yomi_length > 128 as libc::c_int ||
        goi2_length > 128 as libc::c_int {
        return 0 as libc::c_int;
        /* 長すぎるなら、そのようなマージは不適当なので、棄却する */
    }
    i = 1 as libc::c_int;
    while i < length {
        strcat((*(*sp).mrph_data.offset(start_num as isize)).Goi.as_mut_ptr(),
               (*(*sp).mrph_data.offset(start_num as
                   isize).offset(i as
                   isize)).Goi.as_mut_ptr());
        strcat((*(*sp).mrph_data.offset(start_num as
            isize)).Yomi.as_mut_ptr(),
               (*(*sp).mrph_data.offset(start_num as
                   isize).offset(i as
                   isize)).Yomi.as_mut_ptr());
        strcat((*(*sp).mrph_data.offset(start_num as
            isize)).Goi2.as_mut_ptr(),
               (*(*sp).mrph_data.offset(start_num as
                   isize).offset(i as
                   isize)).Goi2.as_mut_ptr());
        /* feature削除 */
        merge_mrph_rep((*sp).mrph_data.offset(start_num as isize),
                       (*sp).mrph_data.offset(start_num as
                           isize).offset(i as
                           isize)); /* Imi領域の代表表記をマージ */
        (*(*sp).mrph_data.offset(start_num as
            isize).offset(i as
            isize)).Goi[0 as
            libc::c_int
            as
            usize]
            = '\u{0}' as i32 as libc::c_char; /* マージ済みの印 */
        clear_feature(&mut (*(*sp).mrph_data.offset(start_num as
            isize).offset(i as
            isize)).f); /* 旧ALT情報を削除 */
        i += 1
    } /* Imi領域の代表表記をfeatureへ */
    delete_alt_feature(&mut (*(*sp).mrph_data.offset(start_num as isize)).f);
    assign_rep_f_from_imi((*sp).mrph_data.offset(start_num as isize));
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn preprocess_mrph(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut start_num: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merge_type: [libc::c_char; 128] = [0; 128];
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    assign_general_feature((*sp).mrph_data as *mut libc::c_void,
                           (*sp).Mrph_num, 16 as libc::c_int,
                           0 as libc::c_int, 0 as libc::c_int);
    /* 正解入力のときは形態素連結をしない */
    if OptInput & 1 as libc::c_int != 0 { return; }
    /* 形態素連結の処理 */
    merge_type[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        cp = 0 as *mut libc::c_char;
        fp = (*(*sp).mrph_data.offset(i as isize)).f;
        while !fp.is_null() {
            if strncmp((*fp).cp,
                       b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe9\x80\xa3\xe7\xb5\x90-\x00"
                           as *const u8 as *const libc::c_char,
                       strlen(b"\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe9\x80\xa3\xe7\xb5\x90-\x00"
                           as *const u8 as *const libc::c_char)) == 0 {
                if !cp.is_null() {
                    fprintf(stderr,
                            b";; Both %s and %s are assigned to %s\n\x00" as
                                *const u8 as *const libc::c_char, cp,
                            (*fp).cp,
                            (*(*sp).mrph_data.offset(i as
                                isize)).Goi.as_mut_ptr());
                } else { cp = (*fp).cp }
            }
            fp = (*fp).next
        }
        if !cp.is_null() {
            /* 形態素連結があった場合 */
            if merge_type[0 as libc::c_int as usize] == 0 {
                /* 開始 */
                start_num = i;
                strcpy(merge_type.as_mut_ptr(), cp);
            } else if strcmp(merge_type.as_mut_ptr(), cp) != 0 {
                /* 直前までとタイプが異なる場合 */
                if merge_mrph(sp, start_num, i - start_num) ==
                    0 as libc::c_int {
                    delete_cfeature_from_mrphs((*sp).mrph_data.offset(start_num
                        as
                        isize),
                                               i - start_num,
                                               merge_type.as_mut_ptr());
                }
                start_num = i;
                strcpy(merge_type.as_mut_ptr(), cp);
            }
        } else if merge_type[0 as libc::c_int as usize] != 0 {
            /* 直前までの形態素連結を処理 */
            if merge_mrph(sp, start_num, i - start_num) == 0 as libc::c_int {
                delete_cfeature_from_mrphs((*sp).mrph_data.offset(start_num as
                    isize),
                                           i - start_num,
                                           merge_type.as_mut_ptr());
            }
            merge_type[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        }
        i += 1
    }
    if merge_type[0 as libc::c_int as usize] != 0 {
        if merge_mrph(sp, start_num, i - start_num) == 0 as libc::c_int {
            delete_cfeature_from_mrphs((*sp).mrph_data.offset(start_num as
                isize),
                                       i - start_num,
                                       merge_type.as_mut_ptr());
        }
    }
    reset_mrph(sp);
}
