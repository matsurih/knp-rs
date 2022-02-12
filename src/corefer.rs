#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{fprintf, free, memset, MRPH_DATA, sentence_data, sprintf, sscanf, strcat, strchr, strcmp, strcpy, strlen, strncmp, TAG_DATA};
use crate::case_match::sms_match;
use crate::ctools::{assign_cfeature, check_dict_filename, check_feature, Outfp, stderr, strncat, strncpy};
use crate::db::{db_close, db_get, db_read_open};
use crate::feature::{check_category, delete_cfeature};
use crate::lib_sm::sm2code;
use crate::proper::ne_corefer;
use crate::structs::CDB_FILE;
use crate::tools::{OptCorefer, OptDisplay, OptNE};
use crate::types::{BNST_DATA, DBM_FILE, SENTENCE_DATA};

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
/*====================================================================

			     共参照解析

                                               R.SASANO 05. 9.24

    $Id$
====================================================================*/
#[no_mangle]
pub static mut ASCEND_SEN_MAX: libc::c_int = 20 as libc::c_int;
#[no_mangle]
pub static mut corefer_id: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut synonym_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut SynonymFile: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_Synonym_db()
/*==================================================================*/
{
    let mut db_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !SynonymFile.is_null() {
        db_filename =
            check_dict_filename(SynonymFile,
                                (0 as libc::c_int == 0) as libc::c_int)
    } else {
        db_filename =
            check_dict_filename(b"synonym/synonym.db\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                0 as libc::c_int)
    }
    synonym_db = db_read_open(db_filename);
    if synonym_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Opening %s ... failed.\n\x00" as *const u8 as
                        *const libc::c_char, db_filename);
        }
        fprintf(stderr,
                b";; Cannot open Synonym Database <%s>.\n\x00" as *const u8 as
                    *const libc::c_char, db_filename);
    } else if OptDisplay == 3 as libc::c_int {
        fprintf(Outfp,
                b"Opening %s ... done.\n\x00" as *const u8 as
                    *const libc::c_char, db_filename);
    }
    free(db_filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_Synonym_db()
/*==================================================================*/
{
    if SynonymFile.is_null() { return; }
    db_close(synonym_db);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_modify_num(mut tag_ptr: *mut TAG_DATA)
                                        -> libc::c_int
/*==================================================================*/
{
    /* 並列を除いていくつの文節に修飾されているかを返す */
    /* ＡのＢＣとなっている場合はＡがＢに係っているかの判断も行う */
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut b_ptr: *mut BNST_DATA = 0 as *mut BNST_DATA;
    b_ptr = (*tag_ptr).b_ptr;
    /* OptCorefer >= 4の場合は修飾されているかどうかを用いない */
    if OptCorefer >= 4 as libc::c_int { return 0 as libc::c_int; }
    /* 所属する文節が修飾されていない場合 */
    if (*b_ptr).child[0 as libc::c_int as usize].is_null() {
        return 0 as libc::c_int;
    }
    if OptCorefer == 1 as libc::c_int {
        /* "直前タグ受"である場合は主辞以外でも修飾されていると考える */
        if (*tag_ptr).head_ptr != (*b_ptr).head_ptr {
            return if !check_feature((*tag_ptr).f,
                                     b"\xe7\x9b\xb4\xe5\x89\x8d\xe3\x82\xbf\xe3\x82\xb0\xe5\x8f\x97\x00"
                                         as *const u8 as *const libc::c_char as
                                         *mut libc::c_char).is_null() {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        }
    } else if OptCorefer == 3 as libc::c_int {
        /* 文節の主辞でないなら修飾されていないと判断する */
        if (*tag_ptr).head_ptr != (*b_ptr).head_ptr {
            return 0 as libc::c_int;
        }
    }
    /* 所属する文節が修飾されていたらその数を返す */
    if (*(*b_ptr).child[0 as libc::c_int as usize]).para_type != 0 {
        b_ptr = (*b_ptr).child[0 as libc::c_int as usize]
    }
    ret = 0 as libc::c_int;
    i = ret;
    while !(*b_ptr).child[i as usize].is_null() {
        if check_feature((*(*b_ptr).child[i as usize]).f,
                         b"\xe4\xbf\x82:\xe3\x82\xab\xe3\x83\xa9\xe6\xa0\xbc\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            check_feature((*(*b_ptr).child[i as usize]).f,
                          b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe6\x9c\xaa\xe6\xa0\xbc\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            check_feature((*(*b_ptr).child[i as usize]).f,
                          b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe9\x80\xa3\xe4\xbd\x93\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            check_feature((*(*b_ptr).child[i as usize]).f,
                          b"\xe4\xbf\x82:\xe5\x90\x8c\xe6\xa0\xbc\xe9\x80\xa3\xe7\x94\xa8\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            ret += 1
        }
        i += 1
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_anaphor_feature(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    /* 複合名詞に照応詞候補というfeatureを付与する */
    /* 複合名詞 */
    /* 固有表現は基本的にそのまま(LOCATION、DATEは分解) */
    /* それ以外は対象の語から文節先頭までを登録 */
    /* この際、先頭の形態素のみ代表表記を保存して、別に保存した照応詞候補も作成する */
    /* ex. 「立てこもり事件」 → 「立てこもり事件」、「立て籠る/たてこもる+事件」 */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    // let mut l: libc::c_int = 0;
    let mut tag_num: libc::c_int = 0;
    let mut mrph_num: libc::c_int = 0;
    // let mut rep_flag: libc::c_int = 0;
    let mut word: [libc::c_char; 256] = [0; 256];
    let mut word_rep: [libc::c_char; 256] = [0; 256];
    let mut buf: [libc::c_char; 275] = [0; 275];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    /* 文節単位で文の前から処理 */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                          b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            tag_num = (*(*sp).bnst_data.offset(i as isize)).tag_num;
            tag_ptr = (*(*sp).bnst_data.offset(i as isize)).tag_ptr;
            j = tag_num - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                /* 固有表現内である場合 */
                if !check_feature((*tag_ptr.offset(j as isize)).f,
                                  b"NE\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                    !check_feature((*tag_ptr.offset(j as isize)).f,
                                   b"NE\xe5\x86\x85\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char).is_null() {
                    /* 固有表現の主辞には付与 */
                    cp =
                        check_feature((*tag_ptr.offset(j as isize)).f,
                                      b"NE\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char); /* "NE:"を読み飛ばす */
                    if !cp.is_null() {
                        cp = cp.offset(3 as libc::c_int as isize);
                        while *cp as libc::c_int != ':' as i32 {
                            cp = cp.offset(1)
                        }
                        if strlen(cp.offset(1 as libc::c_int as isize)) <
                            (128 as libc::c_int * 2 as libc::c_int) as
                                libc::c_ulong {
                            sprintf(buf.as_mut_ptr(),
                                    b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:%s\x00"
                                        as *const u8 as *const libc::c_char,
                                    cp.offset(1 as libc::c_int as isize));
                            assign_cfeature(&mut (*tag_ptr.offset(j as
                                isize)).f,
                                            buf.as_mut_ptr(),
                                            0 as libc::c_int);
                        }
                    } else {
                        /* 固有表現中である場合(DATEまたはLOCATIONの場合) */
                        mrph_num =
                            (*tag_ptr.offset(j as isize)).mrph_num -
                                1 as
                                    libc::c_int; /* "NE:"を読み飛ばす */
                        if !check_feature((*(*tag_ptr.offset(j as
                            isize)).mrph_ptr.offset(mrph_num
                            as
                            isize)).f,
                                          b"NE:DATE\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char).is_null() &&
                            ((*(*tag_ptr.offset(j as
                                isize)).mrph_ptr.offset(mrph_num
                                as
                                isize)).Hinshi
                                == 6 as libc::c_int &&
                                (*(*tag_ptr.offset(j as
                                    isize)).mrph_ptr.offset(mrph_num
                                    as
                                    isize)).Bunrui
                                    == 10 as libc::c_int ||
                                (*(*tag_ptr.offset(j as
                                    isize)).mrph_ptr.offset(mrph_num
                                    as
                                    isize)).Hinshi
                                    == 14 as libc::c_int &&
                                    (*(*tag_ptr.offset(j as
                                        isize)).mrph_ptr.offset(mrph_num
                                        as
                                        isize)).Bunrui
                                        == 3 as libc::c_int) ||
                            !check_feature((*(*tag_ptr.offset(j as
                                isize)).mrph_ptr.offset(mrph_num
                                as
                                isize)).f,
                                           b"NE:LOCATION\x00" as *const u8
                                               as *const libc::c_char as
                                               *mut libc::c_char).is_null()
                                &&
                                (*(*tag_ptr.offset(j as
                                    isize)).mrph_ptr.offset(mrph_num
                                    as
                                    isize)).Hinshi
                                    == 14 as libc::c_int &&
                                (*(*tag_ptr.offset(j as
                                    isize)).mrph_ptr.offset(mrph_num
                                    as
                                    isize)).Bunrui
                                    == 4 as libc::c_int {
                            k = 0 as libc::c_int;
                            loop {
                                cp =
                                    check_feature((*tag_ptr.offset(j as
                                        isize).offset(k
                                        as
                                        isize)).f,
                                                  b"NE\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                if !cp.is_null() { break; }
                                k += 1
                            }
                            cp = cp.offset(3 as libc::c_int as isize);
                            while *cp as libc::c_int != ':' as i32 {
                                cp = cp.offset(1)
                            }
                            /* cp + 1 は対象の固有表現文字列へのポインタ */
                            k = 0 as libc::c_int;
                            while strncmp(cp.offset(k as
                                isize).offset(1 as
                                libc::c_int
                                as
                                isize),
                                          (*(*tag_ptr.offset(j as
                                              isize)).mrph_ptr.offset(mrph_num
                                              as
                                              isize)).Goi2.as_mut_ptr(),
                                          strlen((*(*tag_ptr.offset(j as
                                              isize)).mrph_ptr.offset(mrph_num
                                              as
                                              isize)).Goi2.as_mut_ptr()))
                                != 0 {
                                k += 1
                            }
                            if k < 128 as libc::c_int * 2 as libc::c_int {
                                strncpy(word.as_mut_ptr(),
                                        cp.offset(1 as libc::c_int as isize),
                                        k as libc::c_ulong);
                                word[k as usize] =
                                    '\u{0}' as i32 as libc::c_char;
                                strcat(word.as_mut_ptr(),
                                       (*(*tag_ptr.offset(j as
                                           isize)).mrph_ptr.offset(mrph_num
                                           as
                                           isize)).Goi2.as_mut_ptr());
                                sprintf(buf.as_mut_ptr(),
                                        b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:%s\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        word.as_mut_ptr());
                                assign_cfeature(&mut (*tag_ptr.offset(j as
                                    isize)).f,
                                                buf.as_mut_ptr(),
                                                0 as libc::c_int);
                            }
                        }
                    }
                } else if !((*(*tag_ptr.offset(j as isize)).head_ptr).Hinshi
                    == 6 as libc::c_int &&
                    (*(*tag_ptr.offset(j as
                        isize)).head_ptr).Bunrui
                        > 7 as libc::c_int &&
                    (*(*tag_ptr.offset(j as
                        isize)).head_ptr).Bunrui
                        < 9 as libc::c_int ||
                    (*(*tag_ptr.offset(j as
                        isize)).head_ptr).Hinshi
                        == 3 as libc::c_int &&
                        !check_feature((*tag_ptr.offset(j as
                            isize)).f,
                                       b"\xe4\xbf\x82:\xe9\x9a\xa3\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null())
                {
                    word_rep[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    word[0 as libc::c_int as usize] =
                        word_rep[0 as libc::c_int as usize];
                    k =
                        (*tag_ptr.offset(j as
                            isize)).head_ptr.wrapping_offset_from((*(*sp).bnst_data.offset(i
                            as
                            isize)).mrph_ptr)
                            as libc::c_long as libc::c_int;
                    while k >= 0 as libc::c_int {
                        /* 固有表現内の語を主辞としない場合 */

                        /* 数詞、形式名詞、および隣に係る形容詞は除外 */
                        /* 先頭の特殊、照応接頭辞は含めない */
                        if !(word[0 as libc::c_int as usize] == 0 &&
                            ((*(*tag_ptr.offset(j as
                                isize)).head_ptr.offset(-(k
                                as
                                isize))).Hinshi
                                == 1 as libc::c_int ||
                                !check_feature((*(*tag_ptr.offset(j as
                                    isize)).head_ptr.offset(-(k
                                    as
                                    isize))).f,
                                               b"\xe7\x85\xa7\xe5\xbf\x9c\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00"
                                                   as *const u8 as
                                                   *const libc::c_char
                                                   as
                                                   *mut libc::c_char).is_null()))
                        {
                            /* 「・」などより前は含めない */
                            if strcmp((*(*tag_ptr.offset(j as
                                isize)).head_ptr.offset(-(k
                                as
                                isize))).Goi2.as_mut_ptr(),
                                      b"\xe3\x83\xbb\x00" as *const u8 as
                                          *const libc::c_char) == 0 ||
                                !check_feature((*(*tag_ptr.offset(j as
                                    isize)).head_ptr.offset(-(k
                                    as
                                    isize))).f,
                                               b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00"
                                                   as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null()
                            {
                                if k > 0 as libc::c_int {
                                    word_rep[0 as libc::c_int as usize] =
                                        '\u{0}' as i32 as libc::c_char;
                                    word[0 as libc::c_int as usize] =
                                        word_rep[0 as libc::c_int as usize]
                                }
                            } else {
                                if OptCorefer == 5 as libc::c_int {
                                    word[0 as libc::c_int as usize] =
                                        '\u{0}' as i32 as libc::c_char
                                }
                                strcat(word.as_mut_ptr(),
                                       (*(*tag_ptr.offset(j as
                                           isize)).head_ptr.offset(-(k
                                           as
                                           isize))).Goi2.as_mut_ptr());
                                if word_rep[0 as libc::c_int as usize] as
                                    libc::c_int == '\u{0}' as i32 {
                                    if k > 0 as libc::c_int &&
                                        {
                                            cp =
                                                check_feature((*(*tag_ptr.offset(j
                                                    as
                                                    isize)).head_ptr.offset(-(k
                                                    as
                                                    isize))).f,
                                                              b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\x00"
                                                                  as
                                                                  *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char);
                                            (!cp.is_null()) ||
                                                {
                                                    cp =
                                                        check_feature((*(*tag_ptr.offset(j
                                                            as
                                                            isize)).head_ptr.offset(-(k
                                                            as
                                                            isize))).f,
                                                                      b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char);
                                                    !cp.is_null()
                                                }
                                        } {
                                        strcat(word_rep.as_mut_ptr(),
                                               strchr(cp,
                                                      ':' as
                                                          i32).offset(1 as
                                                   libc::c_int
                                                   as
                                                   isize));
                                    }
                                } else if strlen(word_rep.as_mut_ptr()).wrapping_add(strlen((*(*tag_ptr.offset(j
                                    as
                                    isize)).head_ptr.offset(-(k
                                    as
                                    isize))).Goi2.as_mut_ptr())).wrapping_add(1
                                    as
                                    libc::c_int
                                    as
                                    libc::c_ulong)
                                    <
                                    (128 as libc::c_int *
                                        2 as libc::c_int) as
                                        libc::c_ulong {
                                    strcat(word_rep.as_mut_ptr(),
                                           b"+\x00" as *const u8 as
                                               *const libc::c_char);
                                    strcat(word_rep.as_mut_ptr(),
                                           (*(*tag_ptr.offset(j as
                                               isize)).head_ptr.offset(-(k
                                               as
                                               isize))).Goi2.as_mut_ptr());
                                }
                            }
                        }
                        k -= 1
                    }
                    if word[0 as libc::c_int as usize] != 0 {
                        sprintf(buf.as_mut_ptr(),
                                b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:%s\x00"
                                    as *const u8 as *const libc::c_char,
                                word.as_mut_ptr());
                        assign_cfeature(&mut (*tag_ptr.offset(j as isize)).f,
                                        buf.as_mut_ptr(), 0 as libc::c_int);
                    }
                    if word_rep[0 as libc::c_int as usize] != 0 {
                        sprintf(buf.as_mut_ptr(),
                                b"\xef\xbc\xb4\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c:%s\x00"
                                    as *const u8 as *const libc::c_char,
                                word_rep.as_mut_ptr());
                        assign_cfeature(&mut (*tag_ptr.offset(j as isize)).f,
                                        buf.as_mut_ptr(), 0 as libc::c_int);
                    }
                }
                j -= 1
            }
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn compare_strings(mut antecedent: *mut libc::c_char,
                                         mut anaphor: *mut libc::c_char,
                                         mut ana_ne: *mut libc::c_char,
                                         mut yomi_flag: libc::c_int,
                                         mut tag_ptr: *mut TAG_DATA,
                                         mut rep: *mut libc::c_char)
                                         -> libc::c_int
/*==================================================================*/
{
    /* 照応詞候補と先行詞候補を比較 */
    /* yomi_flagが立っている場合は漢字と読みの照応 */
    /* repがある場合は先頭形態素を代表表記化して比較する場合(ex.「立てこもる事件 = 立てこもり事件」) */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut ant_ne: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word: [libc::c_char; 512] = [0; 512];
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ant_ne =
        check_feature((*tag_ptr).f,
                      b"NE\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    /* 読み方の場合 */
    if yomi_flag != 0 {
        /* ex. 中島河太郎（なかじま・かわたろう) */
        /* 基準
           とりあえず人名の場合のみ
           前方マッチ文字数×後方マッチ文字数×2 > anaphora文字数
           である場合、読み方を表わしていると判定
           ただしantecedentの直後が<括弧始>の場合(yomi_flag=2)の場合は連続していると考え
           マッチ文字数の少ない方に2文字ボーナス */
        if ant_ne.is_null() ||
            strncmp(ant_ne,
                    b"NE:PERSON\x00" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong) != 0 {
            return 0 as libc::c_int;
        }
        right = 0 as libc::c_int;
        left = right;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < strlen(anaphor) {
            if strncmp(antecedent.offset(i as isize),
                       anaphor.offset(i as isize),
                       3 as libc::c_int as libc::c_ulong) != 0 {
                break;
            }
            left += 1;
            i += 3 as libc::c_int
        }
        j = 0 as libc::c_int;
        while (j as libc::c_ulong) < strlen(anaphor) {
            if strncmp(antecedent.offset(strlen(antecedent) as
                isize).offset(-(j as
                isize)).offset(-(3
                as
                libc::c_int
                as
                isize)),
                       anaphor.offset(strlen(anaphor) as
                           isize).offset(-(j as
                           isize)).offset(-(3
                           as
                           libc::c_int
                           as
                           isize)),
                       3 as libc::c_int as libc::c_ulong) != 0 {
                break;
            }
            right += 1;
            j += 3 as libc::c_int
        }
        if yomi_flag == 2 as libc::c_int {
            if left > right {
                right += 2 as libc::c_int
            } else { left += 2 as libc::c_int };
        }
        if (left * right * 2 as libc::c_int * 3 as libc::c_int) as
            libc::c_ulong > strlen(anaphor) {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    /* 異なる種類の固有表現の場合は不可 */
    if !ana_ne.is_null() && !ant_ne.is_null() &&
        strncmp(ana_ne, ant_ne, 7 as libc::c_int as libc::c_ulong) != 0 {
        return 0 as libc::c_int;
    }
    /* repがある場合は先頭形態素を代表表記化して比較する場合 */
    if !rep.is_null() {
        if strncmp(anaphor, rep, strlen(rep)) == 0 &&
            strncmp(anaphor.offset(strlen(rep) as isize),
                    b"+\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as libc::c_ulong) == 0 &&
            strcmp(anaphor.offset(strlen(rep) as
                isize).offset(1 as libc::c_int as
                isize), antecedent)
                == 0 {
            return 1 as libc::c_int;
        }
    }
    /* 同表記の場合 */
    if strcmp(antecedent, anaphor) == 0 { return 1 as libc::c_int; }
    /* 固有表現が同表記の場合(文節をまたがる固有表現のため) */
    if !ant_ne.is_null() && !ana_ne.is_null() && strcmp(ant_ne, ana_ne) == 0 {
        return 1 as libc::c_int;
    }
    /* 先行詞がPERSONである場合は照応詞候補が先行詞候補の先頭に含まれていればOK */
    /* 先行詞がLOCATIONである場合はさらに照応詞候補が住所末尾1文字だけ短かい場合のみOK */
    /* ex. 村山富市=村山、大分県=大分 */
    if !ant_ne.is_null() && strlen(ant_ne) > strlen(antecedent) &&
        strcmp(ant_ne.offset(strlen(ant_ne) as
            isize).offset(-(strlen(antecedent) as
            isize)), antecedent)
            == 0 &&
        (strncmp(ant_ne,
                 b"NE:PERSON\x00" as *const u8 as *const libc::c_char,
                 7 as libc::c_int as libc::c_ulong) == 0 &&
            !ana_ne.is_null() &&
            strncmp(ana_ne,
                    b"NE:PERSON\x00" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong) == 0 ||
            strncmp(ant_ne,
                    b"NE:LOCATION\x00" as *const u8 as
                        *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong) == 0 &&
                strlen(antecedent).wrapping_sub(strlen(anaphor)) ==
                    3 as libc::c_int as libc::c_ulong &&
                !check_feature((*(*tag_ptr).head_ptr).f,
                               b"\xe4\xbd\x8f\xe6\x89\x80\xe6\x9c\xab\xe5\xb0\xbe\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) &&
        strncmp(antecedent, anaphor, strlen(anaphor)) == 0 {
        return 1 as libc::c_int;
    }
    /* 同義表現辞書が読み込めなかった場合はここで終了 */
    if synonym_db.is_null() { return 0 as libc::c_int; }
    /* そのまま同義表現辞書に登録されている場合 */
    word[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    strcpy(word.as_mut_ptr(), anaphor);
    strcat(word.as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char);
    strcat(word.as_mut_ptr(), antecedent);
    value = db_get(synonym_db, word.as_mut_ptr());
    if !value.is_null() {
        free(value as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    /* 前後から同じ表記の文字を削除して残りの文字列のペアを比較する */
    /* 「金融派生商品-取引」と「デリバティブ-取引」は認識できる */
    /* 「日本銀行」と「日銀」のように同義表現が同じ文字を含む場合は認識できない */
    i = 0 as libc::c_int; /* 公文書公開 公開 のとき */
    while (i as libc::c_ulong) < strlen(anaphor) {
        if strncmp(antecedent.offset(i as isize), anaphor.offset(i as isize),
                   3 as libc::c_int as libc::c_ulong) != 0 {
            break;
        }
        i += 3 as libc::c_int
    }
    j = 0 as libc::c_int;
    while (j as libc::c_ulong) < strlen(anaphor) {
        if strncmp(antecedent.offset(strlen(antecedent) as
            isize).offset(-(j as
            isize)).offset(-(3
            as
            libc::c_int
            as
            isize)),
                   anaphor.offset(strlen(anaphor) as
                       isize).offset(-(j as
                       isize)).offset(-(3
                       as
                       libc::c_int
                       as
                       isize)),
                   3 as libc::c_int as libc::c_ulong) != 0 {
            break;
        }
        j += 3 as libc::c_int
    }
    if strlen(anaphor) < (i + j) as libc::c_ulong { return 0 as libc::c_int; }
    memset(word.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<libc::c_char>() as
               libc::c_ulong).wrapping_mul(128 as libc::c_int as
               libc::c_ulong).wrapping_mul(4
               as
               libc::c_int
               as
               libc::c_ulong));
    strncpy(word.as_mut_ptr(), anaphor.offset(i as isize),
            strlen(anaphor).wrapping_sub(i as
                libc::c_ulong).wrapping_sub(j as
                libc::c_ulong));
    strcat(word.as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char);
    strncat(word.as_mut_ptr(), antecedent.offset(i as isize),
            strlen(antecedent).wrapping_sub(i as
                libc::c_ulong).wrapping_sub(j
                as
                libc::c_ulong));
    strcat(word.as_mut_ptr(),
           b"\x00\x00" as *const u8 as *const libc::c_char);
    value = db_get(synonym_db, word.as_mut_ptr());
    if !value.is_null() {
        free(value as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn search_antecedent(mut sp: *mut SENTENCE_DATA,
                                           mut i: libc::c_int,
                                           mut anaphor: *mut libc::c_char,
                                           mut setubi: *mut libc::c_char,
                                           mut ne: *mut libc::c_char)
                                           -> libc::c_int
/*==================================================================*/
{
    /* 入力されたタグと、共参照関係にあるタグを以前の文から検索する */
    /* setubiが与えられた場合は直後の接尾辞も含めて探索する */
    /* 共参照関係にある語が見つかった場合は結果がfeatureに付与され */
    /* 共参照関係にあるとされた照応詞文字列の先頭のタグの番号 */
    /* 見つからなかった場合は-2を返す */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut yomi_flag: libc::c_int = 0;
    let mut word2_flag: libc::c_int = 0;
    let mut setubi_flag: libc::c_int = 0;
    /* word1:「・」などより前を含めない先行詞候補(先行詞候補1)
       word2:「・」などより前を含める先行詞候補(先行詞候補2)
       yomi2:先行詞候補2の読み方 
       anaphor_rep:照応詞候補の先頭形態素を代表表記化したもの */
    let mut word1: [libc::c_char; 129] = [0; 129];
    let mut word2: [libc::c_char; 129] = [0; 129];
    let mut yomi2: [libc::c_char; 129] = [0; 129];
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut anaphor_rep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut CO: [libc::c_char; 129] = [0; 129];
    let mut sdp: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    cp =
        check_feature((*(*sp).tag_data.offset(i as isize)).f,
                      b"\xef\xbc\xb4\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() {
        anaphor_rep = strchr(cp, ':' as i32).offset(1 as libc::c_int as isize)
    } else { anaphor_rep = 0 as *mut libc::c_char }
    yomi_flag =
        if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe8\xaa\xad\xe3\x81\xbf\xe6\x96\xb9\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    sdp =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    j = 0 as libc::c_int;
    while j as libc::c_long <=
        sdp.wrapping_offset_from(sentence_data.as_mut_ptr()) as
            libc::c_long {
        /* 照応先が何文前か */
        if j >= ASCEND_SEN_MAX {
            break; /* ASCEND_SEN_MAX以上前の文は考慮しない */
        }
        k =
            if j != 0 as libc::c_int {
                ((*sdp.offset(-(j as isize))).Tag_num) - 1 as libc::c_int
            } else { (i) - 1 as libc::c_int };
        while k >= 0 as libc::c_int {
            /* 照応先のタグ */
            tag_ptr =
                (*sdp.offset(-(j as isize))).tag_data.offset(k as isize);
            /* 照応詞候補である場合以外は先行詞候補としない */
            if !check_feature((*tag_ptr).f,
                              b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                /* setubiが与えられた場合、後続の名詞性接尾を比較 */
                if !(!setubi.is_null() &&
                    (*tag_ptr).head_ptr <
                        (*tag_ptr).mrph_ptr.offset((*tag_ptr).mrph_num as
                            isize).offset(-(1
                            as
                            libc::c_int
                            as
                            isize))
                    &&
                    strcmp((*(*tag_ptr).head_ptr.offset(1 as libc::c_int
                        as
                        isize)).Goi2.as_mut_ptr(),
                           setubi) != 0) {
                    /* Ｔ照応可能接尾辞が付与されている場合は接尾辞と照応詞の比較を行い
	       同表記であれば共参照関係にあると決定 */
                    setubi_flag = 0 as libc::c_int;
                    if setubi.is_null() &&
                        !check_feature((*tag_ptr).f,
                                       b"\xef\xbc\xb4\xe7\x85\xa7\xe5\xbf\x9c\xe5\x8f\xaf\xe8\x83\xbd\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                                           as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char).is_null() {
                        l = 1 as libc::c_int;
                        while l <= (*tag_ptr).fuzoku_num {
                            if !(*tag_ptr).head_ptr.offset(l as
                                isize).is_null()
                                &&
                                strcmp((*(*tag_ptr).head_ptr.offset(l as
                                    isize)).Goi2.as_mut_ptr(),
                                       anaphor) == 0 {
                                setubi_flag = l;
                                break;
                            } else { l += 1 }
                        }
                    }
                    let mut current_block_69: u64;
                    l =
                        (*tag_ptr).head_ptr.wrapping_offset_from((*(*tag_ptr).b_ptr).mrph_ptr)
                            as libc::c_long as libc::c_int;
                    while l >= 0 as libc::c_int {
                        yomi2[0 as libc::c_int as usize] =
                            '\u{0}' as i32 as libc::c_char;
                        word2[0 as libc::c_int as usize] =
                            yomi2[0 as libc::c_int as usize];
                        word1[0 as libc::c_int as usize] =
                            word2[0 as libc::c_int as usize];
                        m =
                            if setubi_flag != 0 {
                                0 as libc::c_int
                            } else { l };
                        while m >= 0 as libc::c_int {
                            /* 先頭の特殊、照応接頭辞は含めない */
                            if !(strncmp(word1.as_mut_ptr(),
                                         b"\x00\x00" as *const u8 as
                                             *const libc::c_char,
                                         1 as libc::c_int as libc::c_ulong) ==
                                0 &&
                                ((*(*tag_ptr).head_ptr.offset(-(m as
                                    isize))).Hinshi
                                    == 1 as libc::c_int ||
                                    !check_feature((*(*tag_ptr).head_ptr.offset(-(m
                                        as
                                        isize))).f,
                                                   b"\xe7\x85\xa7\xe5\xbf\x9c\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00"
                                                       as *const u8 as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()))
                            {
                                /* 「・」などより前は含めない(word1) */
                                if strcmp((*(*tag_ptr).head_ptr.offset(-(m as
                                    isize))).Goi2.as_mut_ptr(),
                                          b"\xe3\x83\xbb\x00" as *const u8 as
                                              *const libc::c_char) == 0 ||
                                    strcmp((*(*tag_ptr).head_ptr.offset(-(m
                                        as
                                        isize))).Goi2.as_mut_ptr(),
                                           b"\xef\xbc\x9d\x00" as *const u8
                                               as *const libc::c_char) == 0
                                    ||
                                    !check_feature((*(*tag_ptr).head_ptr.offset(-(m
                                        as
                                        isize))).f,
                                                   b"\xe6\x8b\xac\xe5\xbc\xa7\xe7\xb5\x82\x00"
                                                       as *const u8 as
                                                       *const libc::c_char
                                                       as
                                                       *mut libc::c_char).is_null()
                                {
                                    word1[0 as libc::c_int as usize] =
                                        '\u{0}' as i32 as libc::c_char
                                } else {
                                    if strlen(word1.as_mut_ptr()).wrapping_add(strlen((*(*tag_ptr).head_ptr.offset(-(m
                                        as
                                        isize))).Goi2.as_mut_ptr()))
                                        >
                                        128 as libc::c_int as libc::c_ulong
                                    {
                                        break;
                                        /* 先行詞候補1 */
                                    } /* 先行詞候補2 */
                                    strcat(word1.as_mut_ptr(),
                                           (*(*tag_ptr).head_ptr.offset(-(m as
                                               isize))).Goi2.as_mut_ptr());
                                }
                                if strlen(word2.as_mut_ptr()).wrapping_add(strlen((*(*tag_ptr).head_ptr.offset(-(m
                                    as
                                    isize))).Goi2.as_mut_ptr()))
                                    > 128 as libc::c_int as libc::c_ulong {
                                    break;
                                }
                                strcat(word2.as_mut_ptr(),
                                       (*(*tag_ptr).head_ptr.offset(-(m as
                                           isize))).Goi2.as_mut_ptr());
                                if strlen(yomi2.as_mut_ptr()).wrapping_add(strlen((*(*tag_ptr).head_ptr.offset(-(m
                                    as
                                    isize))).Yomi.as_mut_ptr()))
                                    > 128 as libc::c_int as libc::c_ulong {
                                    break;
                                }
                                strcat(yomi2.as_mut_ptr(),
                                       (*(*tag_ptr).head_ptr.offset(-(m as
                                           isize))).Yomi.as_mut_ptr());
                            }
                            m -= 1
                            /* 先行詞候補2の読み方 */
                        }
                        if setubi_flag != 0 {
                            strcpy(word1.as_mut_ptr(),
                                   (*(*tag_ptr).head_ptr.offset(setubi_flag as
                                       isize)).Goi2.as_mut_ptr());
                        }
                        if !(word1[0 as libc::c_int as usize] == 0) {
                            /* 同一文節で先行詞候補が照応詞に含まれている場合は除外(ex.「日本＝日本国」) n*/
                            if j == 0 as libc::c_int &&
                                (*(*sp).tag_data.offset(i as isize)).b_ptr
                                    ==
                                    (*(*sp).tag_data.offset(k as
                                        isize)).b_ptr
                            {
                                length = 0 as libc::c_int;
                                m = 0 as libc::c_int;
                                while (*(*sp).tag_data.offset(k as
                                    isize).offset(1
                                    as
                                    libc::c_int
                                    as
                                    isize)).head_ptr.offset(m
                                    as
                                    isize)
                                    <=
                                    (*(*sp).tag_data.offset(i as
                                        isize)).head_ptr
                                {
                                    length =
                                        (length as
                                            libc::c_ulong).wrapping_add(strlen((*(*(*sp).tag_data.offset(k
                                            as
                                            isize).offset(1
                                            as
                                            libc::c_int
                                            as
                                            isize)).head_ptr.offset(m
                                            as
                                            isize)).Goi2.as_mut_ptr()))
                                            as libc::c_int as libc::c_int;
                                    m += 1
                                }
                                if (length as libc::c_ulong) < strlen(anaphor)
                                {
                                    current_block_69 = 11057878835866523405;
                                } else {
                                    current_block_69 = 1724319918354933278;
                                }
                            } else { current_block_69 = 1724319918354933278; }
                            match current_block_69 {
                                11057878835866523405 => {}
                                _ => {
                                    word2_flag = 0 as libc::c_int;
                                    if setubi_flag != 0 ||
                                        compare_strings(word1.as_mut_ptr(),
                                                        anaphor, ne,
                                                        0 as libc::c_int,
                                                        tag_ptr,
                                                        0 as
                                                            *mut libc::c_char)
                                            != 0 ||
                                        compare_strings(word2.as_mut_ptr(),
                                                        anaphor, ne,
                                                        0 as libc::c_int,
                                                        tag_ptr,
                                                        0 as
                                                            *mut libc::c_char)
                                            != 0 &&
                                            {
                                                word2_flag =
                                                    1 as libc::c_int;
                                                (word2_flag) != 0
                                            } ||
                                        l as libc::c_long ==
                                            (*tag_ptr).head_ptr.wrapping_offset_from((*(*tag_ptr).b_ptr).mrph_ptr)
                                                as libc::c_long &&
                                            !anaphor_rep.is_null() &&
                                            check_feature((*tag_ptr).f,
                                                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\x86\x85\x00"
                                                              as *const u8
                                                              as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char).is_null()
                                            &&
                                            !(*(*tag_ptr).b_ptr).child[0 as
                                                libc::c_int
                                                as
                                                usize].is_null()
                                            &&
                                            !check_feature((*(*tag_ptr).b_ptr.offset(-(1
                                                as
                                                libc::c_int
                                                as
                                                isize))).f,
                                                           b"\xe9\x80\xa3\xe4\xbd\x93\xe4\xbf\xae\xe9\xa3\xbe\x00"
                                                               as *const u8
                                                               as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char).is_null()
                                            &&
                                            {
                                                cp =
                                                    check_feature((*(*(*tag_ptr).b_ptr.offset(-(1
                                                        as
                                                        libc::c_int
                                                        as
                                                        isize))).head_ptr).f,
                                                                  b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char
                                                                      as
                                                                      *mut libc::c_char);
                                                (!cp.is_null()) ||
                                                    {
                                                        cp =
                                                            check_feature((*(*(*tag_ptr).b_ptr.offset(-(1
                                                                as
                                                                libc::c_int
                                                                as
                                                                isize))).head_ptr).f,
                                                                          b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char
                                                                              as
                                                                              *mut libc::c_char);
                                                        !cp.is_null()
                                                    }
                                            } &&
                                            compare_strings(word1.as_mut_ptr(),
                                                            anaphor_rep,
                                                            ne,
                                                            0 as
                                                                libc::c_int,
                                                            tag_ptr,
                                                            strchr(cp,
                                                                   ':' as
                                                                       i32).offset(1
                                                                as
                                                                libc::c_int
                                                                as
                                                                isize))
                                                != 0 ||
                                        yomi_flag != 0 &&
                                            j == 0 as libc::c_int &&
                                            i - k < 10 as libc::c_int &&
                                            compare_strings(yomi2.as_mut_ptr(),
                                                            anaphor, ne,
                                                            1 as
                                                                libc::c_int,
                                                            tag_ptr,
                                                            0 as
                                                                *mut libc::c_char)
                                                != 0 ||
                                        yomi_flag != 0 &&
                                            j == 0 as libc::c_int &&
                                            i - k < 10 as libc::c_int &&
                                            !check_feature((*tag_ptr.offset(1
                                                as
                                                libc::c_int
                                                as
                                                isize)).f,
                                                           b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00"
                                                               as *const u8
                                                               as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char).is_null()
                                            &&
                                            compare_strings(yomi2.as_mut_ptr(),
                                                            anaphor, ne,
                                                            2 as
                                                                libc::c_int,
                                                            tag_ptr,
                                                            0 as
                                                                *mut libc::c_char)
                                                != 0 ||
                                        !check_feature((*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                       b"\xe4\xba\xba\xe7\xa7\xb0\xe4\xbb\xa3\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char).is_null()
                                            &&
                                            !check_feature((*tag_ptr).f,
                                                           b"NE:PERSON\x00"
                                                               as *const u8
                                                               as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char).is_null()
                                        ||
                                        j == 0 && k == i - 1 as libc::c_int
                                            &&
                                            !check_feature((*tag_ptr).f,
                                                           b"\xef\xbc\xb4\xe8\xa7\xa3\xe6\x9e\x90\xe6\xa0\xbc-\xe3\x82\xac\x00"
                                                               as *const u8
                                                               as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char).is_null()
                                            &&
                                            !check_feature((*(*sp).tag_data.offset(i
                                                as
                                                isize)).f,
                                                           b"\xef\xbc\xb4\xe8\x87\xaa\xe7\xa7\xb0\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                                               as *const u8
                                                               as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char).is_null()
                                            &&
                                            sms_match(sm2code(b"\xe4\xb8\xbb\xe4\xbd\x93\x00"
                                                as
                                                *const u8
                                                as
                                                *const libc::c_char
                                                as
                                                *mut libc::c_char),
                                                      (*tag_ptr).SM_code.as_mut_ptr(),
                                                      1 as libc::c_int) !=
                                                0 {
                                        /* 「・」などより前を含めた場合のみ同義表現があった場合 */
                                        if word2_flag != 0 {
                                            strcpy(word1.as_mut_ptr(),
                                                   word2.as_mut_ptr());
                                        }
                                        /* 同義表現であれば */
                                        if j == 0 as libc::c_int {
                                            sprintf(buf.as_mut_ptr(),
                                                    b"C\xe7\x94\xa8;\xe3\x80\x90%s%s\xe3\x80\x91;=;0;%d;9.99:%s(\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    word1.as_mut_ptr(),
                                                    if !setubi.is_null() {
                                                        setubi as
                                                            *const libc::c_char
                                                    } else {
                                                        b"\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }, k,
                                                    if !(*sp).KNPSID.is_null()
                                                    {
                                                        (*sp).KNPSID.offset(5
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize)
                                                            as
                                                            *const libc::c_char
                                                    } else {
                                                        b"?\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }, k);
                                        } else {
                                            sprintf(buf.as_mut_ptr(),
                                                    b"C\xe7\x94\xa8;\xe3\x80\x90%s%s\xe3\x80\x91;=;%d;%d;9.99:%s(%d\xe6\x96\x87\xe5\x89\x8d):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    word1.as_mut_ptr(),
                                                    if !setubi.is_null() {
                                                        setubi as
                                                            *const libc::c_char
                                                    } else {
                                                        b"\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }, j, k,
                                                    if !(*sdp.offset(-(j as
                                                        isize))).KNPSID.is_null()
                                                    {
                                                        (*sdp.offset(-(j as
                                                            isize))).KNPSID.offset(5
                                                            as
                                                            libc::c_int
                                                            as
                                                            isize)
                                                            as
                                                            *const libc::c_char
                                                    } else {
                                                        b"?\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }, j, k);
                                        }
                                        assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                        buf.as_mut_ptr(),
                                                        0 as libc::c_int);
                                        assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                        b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        0 as libc::c_int);
                                        sprintf(buf.as_mut_ptr(),
                                                b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7:=/O/%s%s/%d/%d/-\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                word1.as_mut_ptr(),
                                                if !setubi.is_null() {
                                                    setubi as
                                                        *const libc::c_char
                                                } else {
                                                    b"\x00" as *const u8 as
                                                        *const libc::c_char
                                                }, k, j);
                                        assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                        buf.as_mut_ptr(),
                                                        0 as libc::c_int);
                                        /* COREFER_IDを付与 */
                                        cp =
                                            check_feature((*tag_ptr).f,
                                                          b"COREFER_ID\x00" as
                                                              *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char);
                                        if !cp.is_null() {
                                            assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                                as
                                                isize)).f,
                                                            cp,
                                                            0 as libc::c_int);
                                        } else {
                                            corefer_id += 1;
                                            sprintf(CO.as_mut_ptr(),
                                                    b"COREFER_ID:%d\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    corefer_id);
                                            assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                                as
                                                isize)).f,
                                                            CO.as_mut_ptr(),
                                                            0 as libc::c_int);
                                            assign_cfeature(&mut (*tag_ptr).f,
                                                            CO.as_mut_ptr(),
                                                            0 as libc::c_int);
                                            if j > 0 as libc::c_int {
                                                sprintf(CO.as_mut_ptr(),
                                                        b"REFERRED:%d-%d\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        j, k);
                                                assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                                    as
                                                    isize)).f,
                                                                CO.as_mut_ptr(),
                                                                0 as
                                                                    libc::c_int);
                                            }
                                        }
                                        /* 固有表現とcoreferの関係にある語を固有表現とみなす */
                                        if OptNE != 0 {
                                            if check_feature((*(*sp).tag_data.offset(i
                                                as
                                                isize)).f,
                                                             b"NE\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut libc::c_char).is_null()
                                                &&
                                                check_feature((*(*sp).tag_data.offset(i
                                                    as
                                                    isize)).f,
                                                              b"NE\xe5\x86\x85\x00"
                                                                  as
                                                                  *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char).is_null()
                                                &&
                                                check_feature((*(*sp).tag_data.offset(i
                                                    as
                                                    isize)).f,
                                                              b"\xe4\xba\xba\xe7\xa7\xb0\xe4\xbb\xa3\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                                                  as
                                                                  *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char).is_null()
                                                &&
                                                check_feature((*(*sp).tag_data.offset(i
                                                    as
                                                    isize)).f,
                                                              b"\xef\xbc\xb4\xe8\x87\xaa\xe7\xa7\xb0\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                                                  as
                                                                  *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char).is_null()
                                                &&
                                                {
                                                    cp =
                                                        check_feature((*tag_ptr).f,
                                                                      b"NE\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char); /* "NE:"を読み飛ばす */
                                                    !cp.is_null()
                                                } && setubi.is_null() ||
                                                yomi_flag != 0 &&
                                                    {
                                                        cp =
                                                            check_feature((*tag_ptr).f,
                                                                          b"NE:PERSON\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char
                                                                              as
                                                                              *mut libc::c_char);
                                                        !cp.is_null()
                                                    } {
                                                cp =
                                                    cp.offset(3 as libc::c_int
                                                        as isize);
                                                while *cp as libc::c_int !=
                                                    ':' as i32 {
                                                    cp = cp.offset(1)
                                                }
                                                if strcmp(cp.offset(1 as
                                                    libc::c_int
                                                    as
                                                    isize),
                                                          word1.as_mut_ptr())
                                                    == 0 {
                                                    ne_corefer(sp, i, anaphor,
                                                               check_feature((*tag_ptr).f,
                                                                             b"NE\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char
                                                                                 as
                                                                                 *mut libc::c_char),
                                                               yomi_flag);
                                                }
                                            }
                                        }
                                        return 1 as libc::c_int;
                                    }
                                }
                            }
                        }
                        l -= 1
                    }
                }
            }
            k -= 1
        }
        j += 1
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn person_post(mut sp: *mut SENTENCE_DATA,
                                     mut cp: *mut libc::c_char,
                                     mut i: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* PERSON + 役職 に"="タグを付与 */
    let mut j: libc::c_int = 0; /* tag_ptrは確実に存在 */
    let mut flag: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut CO: [libc::c_char; 128] = [0; 128];
    let mut mrph_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    tag_ptr = (*sp).tag_data.offset(i as isize);
    mrph_ptr = (*tag_ptr).mrph_ptr;
    /* タグ末尾までNE中である場合のみ対象とする */
    if check_feature((*mrph_ptr.offset(-(1 as libc::c_int as isize))).f,
                     b"NE\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char).is_null() &&
        !(!check_feature((*mrph_ptr.offset(-(2 as libc::c_int as
            isize))).f,
                         b"NE\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            (*mrph_ptr.offset(-(1 as libc::c_int as isize))).Hinshi ==
                1 as libc::c_int &&
            (*mrph_ptr.offset(-(1 as libc::c_int as isize))).Bunrui ==
                5 as libc::c_int) {
        /* 直前が記号である */
        return 0 as libc::c_int;
    }
    flag = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while (mrph_ptr.wrapping_offset_from((*sp).mrph_data) as libc::c_long +
        j as libc::c_long) < (*sp).Mrph_num as libc::c_long {
        if !check_feature((*mrph_ptr.offset(j as isize)).f,
                          b"\xe4\xba\xba\xe5\x90\x8d\xe6\x9c\xab\xe5\xb0\xbe\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            flag = 1 as libc::c_int
        } else if !(!check_feature((*mrph_ptr.offset(j as isize)).f,
                                   b"NE\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char).is_null() ||
            !check_feature((*mrph_ptr.offset(j as isize)).f,
                           b"\xe5\x9b\xba\xe6\x9c\x89\xe4\xbf\xae\xe9\xa3\xbe\x00"
                               as *const u8 as *const libc::c_char
                               as *mut libc::c_char).is_null())
        /* 基本的には、ブッシュ・アメリカ大統領、武部自民党幹事長などを想定 */
        {
            break;
        }
        j += 1
    }
    if flag == 0 { return 0 as libc::c_int; }
    /* 複数のタグにまたがっている場合は次のタグに進む */
    while j > (*tag_ptr).mrph_num {
        j -= (*tag_ptr).mrph_num;
        tag_ptr = tag_ptr.offset(1)
    }
    sprintf(buf.as_mut_ptr(),
            b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;=;0;%d;9.99:%s(\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                as *const u8 as *const libc::c_char, cp, i - 1 as libc::c_int,
            if !(*sp).KNPSID.is_null() {
                (*sp).KNPSID.offset(5 as libc::c_int as isize) as
                    *const libc::c_char
            } else { b"?\x00" as *const u8 as *const libc::c_char },
            i - 1 as libc::c_int);
    assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(), 0 as libc::c_int);
    assign_cfeature(&mut (*tag_ptr).f,
                    b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7(\xe5\xbd\xb9\xe8\x81\xb7)\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 0 as libc::c_int);
    sprintf(buf.as_mut_ptr(),
            b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7:=/O/%s/%d/%d/-\x00"
                as *const u8 as *const libc::c_char, cp, i - 1 as libc::c_int,
            0 as libc::c_int);
    assign_cfeature(&mut (*tag_ptr).f, buf.as_mut_ptr(), 0 as libc::c_int);
    /* COREFER_IDを付与 */
    cp =
        check_feature((*tag_ptr).f,
                      b"COREFER_ID\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    if !cp.is_null() {
        assign_cfeature(&mut (*(*sp).tag_data.offset(i as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).f,
                        cp, 0 as libc::c_int);
    } else {
        cp =
            check_feature((*(*sp).tag_data.offset(i as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).f,
                          b"COREFER_ID\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            assign_cfeature(&mut (*tag_ptr).f, cp, 0 as libc::c_int);
        } else {
            corefer_id += 1;
            sprintf(CO.as_mut_ptr(),
                    b"COREFER_ID:%d\x00" as *const u8 as *const libc::c_char,
                    corefer_id);
            assign_cfeature(&mut (*tag_ptr).f, CO.as_mut_ptr(),
                            0 as libc::c_int);
            assign_cfeature(&mut (*(*sp).tag_data.offset(i as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).f,
                            CO.as_mut_ptr(), 0 as libc::c_int);
        }
    }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn recognize_apposition(mut sp: *mut SENTENCE_DATA,
                                              mut i: libc::c_int)
                                              -> libc::c_int
/*==================================================================*/
{
    /* 「カテゴリ:人 + "、" + PERSON」などの処理(同格) */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut CO: [libc::c_char; 128] = [0; 128];
    let mut head_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut mrph_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    let mut tail_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    /* この段階でi-1番目の基本句は読点、または"・"を伴っている */
    /* 同格と認識する条件 */
    /* i-1番目の基本句の主辞形態素のカテゴリ i番目の基本句の先頭形態素     */
    /* 人                                    PERSON (single or head)       */
    /* 組織・団体                            ORGANIZATION (single or head) */
    /* 場所-施設、場所-自然、場所-その他     LOCATION (single or head)     */
    /* 人工物-乗り物                         ARTIFACT or 未知語            */
    head_ptr =
        (*(*sp).tag_data.offset(i as
            isize).offset(-(1 as libc::c_int as
            isize))).head_ptr; /* i-1番目の主辞形態素 */
    mrph_ptr =
        (*(*sp).tag_data.offset(i as
            isize)).mrph_ptr; /* i番目の先頭形態素 */
    /* head_ptrとmrph_ptrの間は、読点、または"・"のみ可 */
    if mrph_ptr.wrapping_offset_from(head_ptr) as libc::c_long >
        2 as libc::c_int as libc::c_long ||
        mrph_ptr.wrapping_offset_from(head_ptr) as libc::c_long ==
            2 as libc::c_int as libc::c_long &&
            check_feature((*(*sp).tag_data.offset(i as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).f,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            strcmp((*(*(*sp).tag_data.offset(i as
                isize)).mrph_ptr.offset(-(1
                as
                libc::c_int
                as
                isize))).Goi2.as_mut_ptr(),
                   b"\xe3\x83\xbb\x00" as *const u8 as *const libc::c_char)
                != 0 {
        return 0 as libc::c_int;
    }
    if check_category((*head_ptr).f,
                      b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 0 as libc::c_int) != 0 &&
        check_feature((*(*(*(*sp).tag_data.offset(i as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).b_ptr).mrph_ptr).f,
                      b"NE:PERSON\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char).is_null() &&
        (!check_feature((*mrph_ptr).f,
                        b"NE:PERSON:head\x00" as *const u8 as
                            *const libc::c_char as
                            *mut libc::c_char).is_null() ||
            !check_feature((*mrph_ptr).f,
                           b"NE:PERSON:single\x00" as *const u8 as
                               *const libc::c_char as
                               *mut libc::c_char).is_null()) ||
        check_category((*head_ptr).f,
                       b"\xe7\xb5\x84\xe7\xb9\x94\xe3\x83\xbb\xe5\x9b\xa3\xe4\xbd\x93\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char, 0 as libc::c_int) != 0 &&
            check_feature((*(*(*(*sp).tag_data.offset(i as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).b_ptr).mrph_ptr).f,
                          b"NE:ORGANIZATION\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (!check_feature((*mrph_ptr).f,
                            b"NE:ORGANIZATION:head\x00" as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*mrph_ptr).f,
                               b"NE:ORGANIZATION:single\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char).is_null()) ||
        (check_category((*head_ptr).f,
                        b"\xe5\xa0\xb4\xe6\x89\x80-\xe6\x96\xbd\xe8\xa8\xad\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char, 0 as libc::c_int) != 0 ||
            check_category((*head_ptr).f,
                           b"\xe5\xa0\xb4\xe6\x89\x80-\xe8\x87\xaa\xe7\x84\xb6\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char, 0 as libc::c_int) != 0
            ||
            check_category((*head_ptr).f,
                           b"\xe5\xa0\xb4\xe6\x89\x80-\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char, 0 as libc::c_int) != 0)
            &&
            strcmp((*head_ptr).Goi2.as_mut_ptr(),
                   b"\xe3\x81\x82\xe3\x81\xa8\x00" as *const u8 as
                       *const libc::c_char) != 0 &&
            check_feature((*(*(*(*sp).tag_data.offset(i as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).b_ptr).mrph_ptr).f,
                          b"NE:LOCATION\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            (!check_feature((*mrph_ptr).f,
                            b"NE:LOCATION:head\x00" as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*mrph_ptr).f,
                               b"NE:LOCATION:single\x00" as *const u8 as
                                   *const libc::c_char as
                                   *mut libc::c_char).is_null()) ||
        check_category((*head_ptr).f,
                       b"\xe4\xba\xba\xe5\xb7\xa5\xe7\x89\xa9-\xe4\xb9\x97\xe3\x82\x8a\xe7\x89\xa9\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char, 0 as libc::c_int) != 0 &&
            (!check_feature((*mrph_ptr).f,
                            b"NE:ARTIFACT:head\x00" as *const u8 as
                                *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                !check_feature((*mrph_ptr).f,
                               b"NE:ARTIFACT:single\x00" as *const u8 as
                                   *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                check_feature((*mrph_ptr).f,
                              b"NE\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                    !check_feature((*mrph_ptr).f,
                                   b"\xe6\x9c\xaa\xe7\x9f\xa5\xe8\xaa\x9e\x00"
                                       as *const u8 as *const libc::c_char
                                       as *mut libc::c_char).is_null()) {
        /* 固有表現の終了する基本句に解析結果を付与 */
        j = i;
        if !check_feature((*mrph_ptr).f,
                          b"NE\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            while check_feature((*(*sp).tag_data.offset(j as isize)).f,
                                b"NE\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char).is_null() {
                j += 1
            }
        }
        /* A, B, Cなどのような並列構造からの誤検出を防止 */
        /* (以下で、i番目の基本句の直前の形態素は読点、または"・") */
        /* i-1番目の基本句を含む文節直前の形態素が、i番目の基本句の直前の形態素と一致する場合は不可 */
        if (*(*sp).tag_data.offset(i as
            isize).offset(-(1 as libc::c_int as
            isize))).b_ptr !=
            (*sp).bnst_data &&
            strcmp((*(*(*(*sp).tag_data.offset(i as
                isize).offset(-(1 as
                libc::c_int
                as
                isize))).b_ptr).mrph_ptr.offset(-(1
                as
                libc::c_int
                as
                isize))).Goi2.as_mut_ptr(),
                   (*(*(*sp).tag_data.offset(i as
                       isize)).mrph_ptr.offset(-(1
                       as
                       libc::c_int
                       as
                       isize))).Goi2.as_mut_ptr())
                == 0 {
            return 0 as libc::c_int;
        }
        /* 固有表現末を含む文節の最後の形態素が、i番目の基本句の直前の形態素と一致する場合は不可 */
        /* ただし、助詞を含む場合は除く */
        if check_feature((*(*(*sp).tag_data.offset(j as isize)).b_ptr).f,
                         b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                             *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            strcmp((*(*(*(*sp).tag_data.offset(j as
                isize)).b_ptr).mrph_ptr.offset((*(*(*sp).tag_data.offset(j
                as
                isize)).b_ptr).mrph_num
                as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).Goi2.as_mut_ptr(),
                   (*(*(*sp).tag_data.offset(i as
                       isize)).mrph_ptr.offset(-(1
                       as
                       libc::c_int
                       as
                       isize))).Goi2.as_mut_ptr())
                == 0 {
            return 0 as libc::c_int;
        }
        /* 固有表現直後の形態素が、i番目の基本句の直前の形態素と一致する場合は不可 */
        tail_ptr = (*(*sp).tag_data.offset(i as isize)).mrph_ptr;
        k = 0 as libc::c_int;
        while k < (*(*sp).tag_data.offset(j as isize)).mrph_num &&
            !check_feature((*(*(*sp).tag_data.offset(j as
                isize)).mrph_ptr.offset(k
                as
                isize)).f,
                           b"NE\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char).is_null() {
            k += 1
        }
        if k != (*(*sp).tag_data.offset(j as isize)).mrph_num &&
            strcmp((*(*(*sp).tag_data.offset(j as
                isize)).mrph_ptr.offset(k
                as
                isize)).Goi2.as_mut_ptr(),
                   (*(*(*sp).tag_data.offset(i as
                       isize)).mrph_ptr.offset(-(1
                       as
                       libc::c_int
                       as
                       isize))).Goi2.as_mut_ptr())
                == 0 {
            return 0 as libc::c_int;
        }
        /* 固有表現を含む基本句が助詞を伴う、または、文末である
	   または、直後に「PERSON + 人名末尾」である場合以外は不可 */
        if check_feature((*(*sp).tag_data.offset(j as isize)).f,
                         b"\xe5\x8a\xa9\xe8\xa9\x9e\x00" as *const u8 as
                             *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            check_feature((*(*sp).tag_data.offset(j as isize)).f,
                          b"\xe6\x96\x87\xe6\x9c\xab\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() &&
            !(!check_feature((*(*sp).tag_data.offset(j as isize)).f,
                             b"NE:PERSON\x00" as *const u8 as
                                 *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                !check_feature((*(*(*sp).tag_data.offset(j as
                    isize).offset(1
                    as
                    libc::c_int
                    as
                    isize)).mrph_ptr).f,
                               b"\xe4\xba\xba\xe5\x90\x8d\xe6\x9c\xab\xe5\xb0\xbe\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
            return 0 as libc::c_int;
        }
        sprintf(buf.as_mut_ptr(),
                b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;=;0;%d;9.99:%s(\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                    as *const u8 as *const libc::c_char,
                (*head_ptr).Goi2.as_mut_ptr(), i - 1 as libc::c_int,
                if !(*sp).KNPSID.is_null() {
                    (*sp).KNPSID.offset(5 as libc::c_int as isize) as
                        *const libc::c_char
                } else { b"?\x00" as *const u8 as *const libc::c_char },
                i - 1 as libc::c_int);
        assign_cfeature(&mut (*(*sp).tag_data.offset(j as isize)).f,
                        buf.as_mut_ptr(), 0 as libc::c_int);
        assign_cfeature(&mut (*(*sp).tag_data.offset(j as isize)).f,
                        b"\xe5\x90\x8c\xe6\xa0\xbc\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int);
        sprintf(buf.as_mut_ptr(),
                b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7:=/O/null/%d/%d/-\x00"
                    as *const u8 as *const libc::c_char, i - 1 as libc::c_int,
                0 as libc::c_int);
        assign_cfeature(&mut (*(*sp).tag_data.offset(j as isize)).f,
                        buf.as_mut_ptr(), 0 as libc::c_int);
        /* COREFER_IDを付与 */
        cp =
            check_feature((*(*sp).tag_data.offset(j as isize)).f,
                          b"COREFER_ID\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            assign_cfeature(&mut (*(*sp).tag_data.offset(i as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).f,
                            cp, 0 as libc::c_int);
        } else {
            cp =
                check_feature((*(*sp).tag_data.offset(i as
                    isize).offset(-(1 as
                    libc::c_int
                    as
                    isize))).f,
                              b"COREFER_ID\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
            if !cp.is_null() {
                assign_cfeature(&mut (*(*sp).tag_data.offset(j as isize)).f,
                                cp, 0 as libc::c_int);
            } else {
                corefer_id += 1;
                sprintf(CO.as_mut_ptr(),
                        b"COREFER_ID:%d\x00" as *const u8 as
                            *const libc::c_char, corefer_id);
                assign_cfeature(&mut (*(*sp).tag_data.offset(j as isize)).f,
                                CO.as_mut_ptr(), 0 as libc::c_int);
                assign_cfeature(&mut (*(*sp).tag_data.offset(i as
                    isize).offset(-(1
                    as
                    libc::c_int
                    as
                    isize))).f,
                                CO.as_mut_ptr(), 0 as libc::c_int);
            }
        }
        /* 普通名詞側に「省略解析なし」が付与されている場合は除去する */
        if !check_feature((*(*sp).tag_data.offset(i as
            isize).offset(-(1 as
            libc::c_int
            as
            isize))).f,
                          b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            delete_cfeature(&mut (*(*sp).tag_data.offset(i as
                isize).offset(-(1
                as
                libc::c_int
                as
                isize))).f,
                            b"\xe7\x9c\x81\xe7\x95\xa5\xe8\xa7\xa3\xe6\x9e\x90\xe3\x81\xaa\xe3\x81\x97\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn corefer_analysis(mut sp: *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    // let mut person: libc::c_int = 0;
    let mut anaphor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ne: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mrph_ptr: *mut MRPH_DATA = 0 as *mut MRPH_DATA;
    sp =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    let mut current_block_14: u64;
    i = (*sp).Tag_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        /* 解析文のタグ単位:i番目のタグについて */
        /* 共参照解析を行う条件 */
        /* 照応詞候補であり、固有表現中の語、または */
        /* 連体詞形態指示詞以外に修飾されていない語 */
        anaphor =
            check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !anaphor.is_null() &&
            (!check_feature((*(*sp).tag_data.offset(i as isize)).f,
                            b"NE\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                               b"NE\xe5\x86\x85\x00" as *const u8 as
                                   *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                               b"\xe8\xaa\xad\xe3\x81\xbf\xe6\x96\xb9\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                get_modify_num((*sp).tag_data.offset(i as isize)) == 0 ||
                (*(*(*sp).tag_data.offset(i as
                    isize)).mrph_ptr.offset(-(1
                    as
                    libc::c_int
                    as
                    isize))).Hinshi
                    == 1 as libc::c_int &&
                    (*(*(*sp).tag_data.offset(i as
                        isize)).mrph_ptr.offset(-(1
                        as
                        libc::c_int
                        as
                        isize))).Bunrui
                        == 2 as libc::c_int ||
                !check_feature((*(*(*(*sp).tag_data.offset(i as
                    isize)).b_ptr).child[0
                    as
                    libc::c_int
                    as
                    usize]).f,
                               b"\xe9\x80\xa3\xe4\xbd\x93\xe8\xa9\x9e\xe5\xbd\xa2\xe6\x85\x8b\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                !check_feature((*(*(*(*sp).tag_data.offset(i as
                    isize)).b_ptr).child[0
                    as
                    libc::c_int
                    as
                    usize]).f,
                               b"\xe7\x85\xa7\xe5\xbf\x9c\xe6\x8e\xa5\xe9\xa0\xad\xe8\xbe\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
            /* 指示詞の場合 */
            if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                              b"\xe6\x8c\x87\xe7\xa4\xba\xe8\xa9\x9e\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                current_block_14 = 16658872821858055392;
            } else {
                /* 基本句が固有表現を含まず、かつ、基本句主辞の後に形態素がある場合、それをmrph_ptrとする */
                mrph_ptr = 0 as *mut MRPH_DATA;
                ne =
                    check_feature((*(*sp).tag_data.offset(i as isize)).f,
                                  b"NE\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                if ne.is_null() &&
                    (*(*sp).tag_data.offset(i as
                        isize)).mrph_ptr.offset((*(*sp).tag_data.offset(i
                        as
                        isize)).mrph_num
                        as
                        isize).wrapping_offset_from((*(*sp).tag_data.offset(i
                        as
                        isize)).head_ptr)
                        as libc::c_long > 1 as libc::c_int as libc::c_long
                {
                    mrph_ptr =
                        (*(*sp).tag_data.offset(i as
                            isize)).head_ptr.offset(1
                            as
                            libc::c_int
                            as
                            isize)
                }
                /* 先行する表現と共参照関係にあるかをチェック */
                if !mrph_ptr.is_null() &&
                    (*mrph_ptr).Hinshi == 14 as libc::c_int &&
                    (*mrph_ptr).Bunrui < 5 as libc::c_int &&
                    search_antecedent(sp, i,
                                      anaphor.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                          as
                                          *const u8
                                          as
                                          *const libc::c_char)
                                          as
                                          isize).offset(1 as
                                          libc::c_int
                                          as
                                          isize),
                                      (*mrph_ptr).Goi2.as_mut_ptr(),
                                      0 as *mut libc::c_char) != 0 ||
                    search_antecedent(sp, i,
                                      anaphor.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                          as
                                          *const u8
                                          as
                                          *const libc::c_char)
                                          as
                                          isize).offset(1 as
                                          libc::c_int
                                          as
                                          isize),
                                      0 as *mut libc::c_char, ne) != 0 {
                    /* 一般の場合 */
                    /* すでに見つかった共参照関係に含まれる関係は解析しない */
                    /* e.g. 照応詞が「国立大学」なら「国立」は照応詞として考慮しない */
                    if strcmp(anaphor.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                        as *const u8 as
                        *const libc::c_char)
                        as
                        isize).offset(1 as
                        libc::c_int
                        as isize),
                              (*(*(*sp).tag_data.offset(i as
                                  isize)).mrph_ptr).Goi2.as_mut_ptr())
                        == 0 {
                        current_block_14 =
                            16658872821858055392; /* １形態素から成る場合は考慮せず */
                    } else {
                        while i > 0 as libc::c_int {
                            cp =
                                check_feature((*(*sp).tag_data.offset(i as
                                    isize).offset(-(1
                                    as
                                    libc::c_int
                                    as
                                    isize))).f,
                                              b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                            if !(!cp.is_null() &&
                                strncmp(cp, anaphor, strlen(cp)) == 0) {
                                break;
                            }
                            i -= 1;
                            assign_cfeature(&mut (*(*sp).tag_data.offset(i as
                                isize)).f,
                                            b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\xe5\x86\x85\x00"
                                                as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char,
                                            0 as libc::c_int);
                            if strcmp(cp.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                as *const u8 as
                                *const libc::c_char)
                                as
                                isize).offset(1 as
                                libc::c_int
                                as
                                isize),
                                      (*(*(*sp).tag_data.offset(i as
                                          isize)).mrph_ptr).Goi2.as_mut_ptr())
                                == 0 {
                                break;
                            }
                            /* １形態素から成るならそこまで */
                        }
                        current_block_14 = 16658872821858055392;
                    }
                } else { current_block_14 = 224731115979188411; }
            }
        } else { current_block_14 = 224731115979188411; }
        match current_block_14 {
            224731115979188411 => {
                /* PERSON + 人名末尾 の処理(共参照(役職)) */
                if i > 0 as libc::c_int &&
                    {
                        cp =
                            check_feature((*(*sp).tag_data.offset(i as
                                isize).offset(-(1
                                as
                                libc::c_int
                                as
                                isize))).f,
                                          b"NE:PERSON\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                        !cp.is_null()
                    } {
                    person_post(sp, cp.offset(10 as libc::c_int as isize), i);
                }
                /* 「カテゴリ:人 + "、" + PERSON」などの処理(同格) */
                if i > 0 as libc::c_int &&
                    check_feature((*(*sp).tag_data.offset(i as
                        isize).offset(-(1
                        as
                        libc::c_int
                        as
                        isize))).f,
                                  b"NE\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    recognize_apposition(sp, i);
                }
            }
            _ => {}
        }
        i -= 1
        /* ここでは処理をしない */
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn search_antecedent_after_br(mut sp:
                                                    *mut SENTENCE_DATA,
                                                    mut tag_ptr1:
                                                    *mut TAG_DATA,
                                                    mut i: libc::c_int)
                                                    -> libc::c_int
/*==================================================================*/
{
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut sent: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut CO: [libc::c_char; 128] = [0; 128];
    let mut sdp: *mut SENTENCE_DATA = 0 as *mut SENTENCE_DATA;
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    let mut tag_ptr2: *mut TAG_DATA = 0 as *mut TAG_DATA;
    sdp =
        sentence_data.as_mut_ptr().offset((*sp).Sen_num as
            isize).offset(-(1 as libc::c_int
            as isize));
    j = 0 as libc::c_int;
    while j as libc::c_long <=
        sdp.wrapping_offset_from(sentence_data.as_mut_ptr()) as
            libc::c_long {
        /* 照応先が何文前か */
        k =
            if j != 0 {
                ((*sdp.offset(-(j as isize))).Tag_num) - 1 as libc::c_int
            } else { (i) - 1 as libc::c_int };
        while k >= 0 as libc::c_int {
            /* 照応先のタグ */
            tag_ptr =
                (*sdp.offset(-(j as isize))).tag_data.offset(k as isize);
            /* 照応詞候補である場合以外は先行詞候補としない */
            if !check_feature((*tag_ptr).f,
                              b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                /* 照応詞候補と同じ表記のものしか先行詞候補としない */
                if !(strcmp((*(*(*sp).tag_data.offset(i as
                    isize)).head_ptr).Goi2.as_mut_ptr(),
                            (*(*tag_ptr).head_ptr).Goi2.as_mut_ptr()) != 0) {
                    /* 格解析結果がある */
                    sprintf(buf.as_mut_ptr(),
                            b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:%s:\xe5\x90\x8d1\x00"
                                as *const u8 as *const libc::c_char,
                            (*(*tag_ptr).head_ptr).Goi2.as_mut_ptr());
                    cp = check_feature((*tag_ptr).f, buf.as_mut_ptr());
                    if !cp.is_null() {
                        /* <格解析結果:結果:名1:ノ/O/アンケート/0/1/?> */
                        l = 0 as libc::c_int;
                        while l < 3 as libc::c_int {
                            while *cp as libc::c_int != '/' as i32 {
                                cp = cp.offset(1)
                            }
                            cp = cp.offset(1);
                            l += 1
                        }
                        if !(sscanf(cp,
                                    b"%d/%d/\x00" as *const u8 as
                                        *const libc::c_char,
                                    &mut tag as *mut libc::c_int,
                                    &mut sent as *mut libc::c_int) == 0) {
                            /* 指示先のタグへのポインタ */
                            tag_ptr2 =
                                (*sdp.offset(-(j as
                                    isize)).offset(-(sent as
                                    isize))).tag_data.offset(tag
                                    as
                                    isize);
                            /* 指示先のタグが共参照関係にあるかを判定 */
                            if !check_feature((*tag_ptr1).f,
                                              b"COREFER_ID\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                                &&
                                !check_feature((*tag_ptr2).f,
                                               b"COREFER_ID\x00" as
                                                   *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char).is_null()
                                &&
                                strcmp(check_feature((*tag_ptr1).f,
                                                     b"COREFER_ID\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char),
                                       check_feature((*tag_ptr2).f,
                                                     b"COREFER_ID\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char))
                                    == 0 {
                                cp =
                                    check_feature((*tag_ptr).f,
                                                  b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                cp =
                                    cp.offset(strlen(b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                                        as *const u8 as
                                        *const libc::c_char).wrapping_add(1
                                        as
                                        libc::c_int
                                        as
                                        libc::c_ulong)
                                        as isize);
                                if j == 0 as libc::c_int {
                                    sprintf(buf.as_mut_ptr(),
                                            b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;=;0;%d;9.99:%s(\xe5\x90\x8c\xe4\xb8\x80\xe6\x96\x87):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                                                as *const u8 as
                                                *const libc::c_char, cp, k,
                                            if !(*sp).KNPSID.is_null() {
                                                (*sp).KNPSID.offset(5 as
                                                    libc::c_int
                                                    as
                                                    isize)
                                                    as *const libc::c_char
                                            } else {
                                                b"?\x00" as *const u8 as
                                                    *const libc::c_char
                                            }, k);
                                } else {
                                    sprintf(buf.as_mut_ptr(),
                                            b"C\xe7\x94\xa8;\xe3\x80\x90%s\xe3\x80\x91;=;%d;%d;9.99:%s(%d\xe6\x96\x87\xe5\x89\x8d):%d\xe6\x96\x87\xe7\xaf\x80\x00"
                                                as *const u8 as
                                                *const libc::c_char, cp, j, k,
                                            if !(*sdp.offset(-(j as
                                                isize))).KNPSID.is_null()
                                            {
                                                (*sdp.offset(-(j as
                                                    isize))).KNPSID.offset(5
                                                    as
                                                    libc::c_int
                                                    as
                                                    isize)
                                                    as *const libc::c_char
                                            } else {
                                                b"?\x00" as *const u8 as
                                                    *const libc::c_char
                                            }, j, k);
                                }
                                assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                    as
                                    isize)).f,
                                                buf.as_mut_ptr(),
                                                0 as libc::c_int);
                                assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                    as
                                    isize)).f,
                                                b"\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                0 as libc::c_int);
                                sprintf(buf.as_mut_ptr(),
                                        b"\xef\xbc\xb4\xe5\x85\xb1\xe5\x8f\x82\xe7\x85\xa7:=/O/%s/%d/%d/-\x00"
                                            as *const u8 as
                                            *const libc::c_char, cp, k, j);
                                assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                    as
                                    isize)).f,
                                                buf.as_mut_ptr(),
                                                0 as libc::c_int);
                                /* COREFER_IDを付与 */
                                cp =
                                    check_feature((*tag_ptr).f,
                                                  b"COREFER_ID\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                if !cp.is_null() {
                                    assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                        as
                                        isize)).f,
                                                    cp, 0 as libc::c_int);
                                } else {
                                    cp =
                                        check_feature((*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                      b"COREFER_ID\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char);
                                    if !cp.is_null() {
                                        assign_cfeature(&mut (*tag_ptr).f, cp,
                                                        0 as libc::c_int);
                                    } else {
                                        corefer_id += 1;
                                        sprintf(CO.as_mut_ptr(),
                                                b"COREFER_ID:%d\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                corefer_id);
                                        assign_cfeature(&mut (*(*sp).tag_data.offset(i
                                            as
                                            isize)).f,
                                                        CO.as_mut_ptr(),
                                                        0 as libc::c_int);
                                        assign_cfeature(&mut (*tag_ptr).f,
                                                        CO.as_mut_ptr(),
                                                        0 as libc::c_int);
                                    }
                                }
                                return 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
            k -= 1
        }
        j += 1
    }
    panic!("Reached end of non-void function without returning");
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn corefer_analysis_after_br(mut sp:
                                                   *mut SENTENCE_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut sent: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut tag_ptr: *mut TAG_DATA = 0 as *mut TAG_DATA;
    i = 0 as libc::c_int;
    while i < (*sp).Tag_num {
        /* 照応詞候補である場合以外は先行詞候補としない */
        if !check_feature((*(*sp).tag_data.offset(i as isize)).f,
                          b"\xe7\x85\xa7\xe5\xbf\x9c\xe8\xa9\x9e\xe5\x80\x99\xe8\xa3\x9c\x00"
                              as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() {
            /* 名詞に限定(接尾辞は対象外) */
            if !((*(*(*sp).tag_data.offset(i as isize)).head_ptr).Hinshi !=
                6 as libc::c_int) {
                /* 共参照タグがなく、格解析結果がある */
                sprintf(buf.as_mut_ptr(),
                        b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c:%s:\xe5\x90\x8d1\x00"
                            as *const u8 as *const libc::c_char,
                        (*(*(*sp).tag_data.offset(i as
                            isize)).head_ptr).Goi2.as_mut_ptr());
                if check_feature((*(*sp).tag_data.offset(i as isize)).f,
                                 b"COREFER_ID\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char).is_null() &&
                    {
                        cp =
                            check_feature((*(*sp).tag_data.offset(i as
                                isize)).f,
                                          buf.as_mut_ptr());
                        !cp.is_null()
                    } {
                    /* <格解析結果:結果:名1:ノ/O/アンケート/0/1/?> */
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        while *cp as libc::c_int != '/' as i32 {
                            cp = cp.offset(1)
                        }
                        cp = cp.offset(1);
                        j += 1
                    }
                    if sscanf(cp,
                              b"%d/%d/\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut tag as *mut libc::c_int,
                              &mut sent as *mut libc::c_int) != 0 {
                        /* 指示先のタグへのポインタ */
                        tag_ptr =
                            (*sentence_data.as_mut_ptr().offset((*sp).Sen_num
                                as
                                isize).offset(-(1
                                as
                                libc::c_int
                                as
                                isize)).offset(-(sent
                                as
                                isize))).tag_data.offset(tag
                                as
                                isize);
                        search_antecedent_after_br(sp, tag_ptr, i);
                    }
                }
            }
        }
        i += 1
    };
}
/*====================================================================
                               END
====================================================================*/
