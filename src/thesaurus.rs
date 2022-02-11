#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
ptr_wrapping_offset_from, register_tool)]

use crate::{case_match, db, dic, tools, types};
use crate::case_ipal::rep2id;
use crate::ctools::{assign_cfeature, check_dict_filename, check_feature, get_mrph_rep_from_f, Language, malloc_data, Outfp, stderr};
use crate::db::{db_get, db_read_open};
use crate::lib_bgh::{_get_bgh, bgh_code_match_for_case, close_bgh, init_bgh};
use crate::lib_sm::{_get_ntt, check_noun_sm, close_ntt, delete_specified_sm, init_ntt, ntt_code_match};
use crate::read_data::{get_bnst_head_canonical_rep, get_mrph_rep};
use crate::tools::{katakana2hiragana, OptCaseFlag, OptDisplay, OptUseRN, ParaThesaurus, static_buffer1, static_buffer2, Thesaurus};
use crate::types::THESAURUS_FILE;


/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_thesaurus()
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    /* tentative: 新しいシソーラスはNTTと排他的 */
    if Thesaurus != -(1 as libc::c_int) && Thesaurus != 1 as libc::c_int && Thesaurus != 2 as libc::c_int && ParaThesaurus == 2 as libc::c_int {
        ParaThesaurus = Thesaurus;
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp, b"Thesaurus for para analysis is forced to %s.\n\x00" as *const u8 as *const libc::c_char, (*THESAURUS.as_mut_ptr().offset(ParaThesaurus as isize)).name);
        }
    } else if ParaThesaurus != -(1 as libc::c_int) &&
        ParaThesaurus != 1 as libc::c_int &&
        ParaThesaurus != 2 as libc::c_int &&
        Thesaurus == 2 as libc::c_int {
        Thesaurus = ParaThesaurus;
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Thesaurus for case analysis is forced to %s.\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*THESAURUS.as_mut_ptr().offset(Thesaurus as
                        isize)).name);
        }
    }
    if Thesaurus == 1 as libc::c_int || ParaThesaurus == 1 as libc::c_int {
        init_bgh();
    }
    if Thesaurus == 2 as libc::c_int || ParaThesaurus == 2 as libc::c_int {
        init_ntt();
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !(i == 1 as libc::c_int || i == 2 as libc::c_int ||
            (*THESAURUS.as_mut_ptr().offset(i as isize)).path.is_null())
        {
            filename =
                check_dict_filename((*THESAURUS.as_mut_ptr().offset(i as isize)).path, (0 as libc::c_int == 0) as libc::c_int);
            if OptDisplay == 3 as libc::c_int {
                fprintf(Outfp, b"Opening %s ... \x00" as *const u8 as *const libc::c_char, filename);
            }
            let ref mut fresh0 =
                (*THESAURUS.as_mut_ptr().offset(i as isize)).db;
            *fresh0 = db_read_open(filename);
            if (*fresh0).is_null() {
                if OptDisplay == 3 as libc::c_int {
                    fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char, Outfp);
                }
                (*THESAURUS.as_mut_ptr().offset(i as isize)).exist = 0 as libc::c_int
            } else {
                if OptDisplay == 3 as libc::c_int {
                    fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, Outfp);
                }
                (*THESAURUS.as_mut_ptr().offset(i as isize)).exist = (0 as libc::c_int == 0) as libc::c_int
            }
            free(filename as *mut libc::c_void);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_thesaurus() {
    let mut i: libc::c_int = 0;
    if Thesaurus == 1 as libc::c_int || ParaThesaurus == 1 as libc::c_int {
        close_bgh();
    }
    if Thesaurus == 2 as libc::c_int || ParaThesaurus == 2 as libc::c_int {
        close_ntt();
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !(i == 1 as libc::c_int || i == 2 as libc::c_int || (*THESAURUS.as_mut_ptr().offset(i as isize)).exist == 0 as libc::c_int) {
            db::db_close((*THESAURUS.as_mut_ptr().offset(i as isize)).db);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_code(mut cp: *mut libc::c_char,
                                  mut arg: *mut libc::c_char,
                                  mut th: libc::c_int) -> *mut libc::c_char
/*==================================================================*/
{
    if th == 2 as libc::c_int {
        return _get_ntt(cp, arg);
    } else { if th == 1 as libc::c_int { return _get_bgh(cp, arg); } }
    return db_get((*THESAURUS.as_mut_ptr().offset(th as isize)).db, cp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_str_code(mut cp: *mut libc::c_uchar,
                                      mut flag: libc::c_int)
                                      -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut th: libc::c_int = 0;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: libc::c_char = '\u{0}' as i32 as libc::c_char;
    let mut hira: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* 文字列の意味素コードを取得 */
    if flag & 2 as libc::c_int != 0 {
        code = check_noun_sm(cp as *mut libc::c_char);
        if !code.is_null() { return code; }
        th = 2 as libc::c_int;
        if flag & 8 as libc::c_int != 0 {
            arg = 'm' as i32 as libc::c_char
        } else if flag & 16 as libc::c_int != 0 {
            arg = 'l' as i32 as libc::c_char
        }
    } else if flag & 1 as libc::c_int != 0 {
        th = 1 as libc::c_int
    } else { th = flag }
    if (*THESAURUS.as_mut_ptr().offset(th as isize)).exist == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    code = get_code(cp as *mut libc::c_char, &mut arg, th);
    if !code.is_null() { return code; }
    if flag & 32 as libc::c_int != 0 { return 0 as *mut libc::c_char; }
    /* 意味素がない場合で、
       すべての文字がカタカナの場合はひらがなに変換して辞書引き */
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < strlen(cp as *const libc::c_char) {
        /* euc-jp */
        if *cp.offset(i as isize) as libc::c_int != 0xa5 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        i += 3 as libc::c_int
    }
    hira = katakana2hiragana(cp);
    code = get_code(hira as *mut libc::c_char, &mut arg, th);
    free(hira as *mut libc::c_void);
    return code;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_str_code_with_len(mut cp: *mut libc::c_char,
                                               mut len: libc::c_int,
                                               mut flag: libc::c_int)
                                               -> *mut libc::c_char
/*==================================================================*/
{
    let mut bak_char: libc::c_char = *cp.offset(len as isize);
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    *cp.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    code = get_str_code(cp as *mut libc::c_uchar, flag);
    *cp.offset(len as isize) = bak_char;
    return code;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn overflowed_function(mut str: *mut libc::c_char,
                                             mut max: libc::c_int,
                                             mut function: *mut libc::c_char)
/*==================================================================*/
{
    *str.offset((max - 1 as libc::c_int) as isize) =
        '\u{0}' as i32 as libc::c_char;
    if OptDisplay == 3 as libc::c_int {
        fprintf(stderr, b";; Too long key <%s> in %s.\n\x00" as *const u8 as *const libc::c_char, str, function);
    }
    *str.offset((max - 1 as libc::c_int) as isize) = '\n' as i32 as libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_bnst_code_all(mut ptr: *mut tools::BNST_DATA)
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        get_bnst_code(ptr, i);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn add_rep_str(mut ptr: *mut tools::MRPH_DATA,
                                     mut str_buffer: *mut libc::c_char,
                                     mut org_flag: libc::c_int,
                                     mut flag: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    let mut rep_strt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut add_len: libc::c_int = 0;
    rep_strt = get_mrph_rep(ptr);
    if !rep_strt.is_null() {
        if flag & 32 as libc::c_int != 0 {
            rep_end = strchr(rep_strt, ' ' as i32);
            if rep_end.is_null() { rep_end = strchr(rep_strt, '\"' as i32) }
        } else { rep_end = strchr(rep_strt, '/' as i32) }
    }
    if !rep_strt.is_null() && !rep_end.is_null() {
        add_len =
            rep_end.wrapping_offset_from(rep_strt) as libc::c_long as
                libc::c_int;
        if strlen(str_buffer).wrapping_add(add_len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            > 256 as libc::c_int as libc::c_ulong {
            overflowed_function(str_buffer, 256 as libc::c_int,
                                b"add_rep_str\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
            return 0 as libc::c_int;
        }
        /* org_flag == 0 のときは活用させる必要がある */
        strncat(str_buffer, rep_strt, add_len as libc::c_ulong);
    } else {
        add_len =
            strlen(if org_flag != 0 {
                (*ptr).Goi.as_mut_ptr()
            } else { (*ptr).Goi2.as_mut_ptr() }) as libc::c_int;
        if strlen(str_buffer).wrapping_add(add_len as
            libc::c_ulong).wrapping_add(2
            as
            libc::c_int
            as
            libc::c_ulong)
            > 256 as libc::c_int as libc::c_ulong {
            overflowed_function(str_buffer, 256 as libc::c_int,
                                b"add_rep_str\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
            return 0 as libc::c_int;
        }
        if org_flag != 0 {
            /* ナ形容詞の場合は語幹で検索 */
            if Language == 1 as libc::c_int &&
                strcmp(Class[(*ptr).Hinshi as
                    usize][0 as libc::c_int as usize].id as
                           *const libc::c_char,
                       b"\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\x00" as
                           *const u8 as *const libc::c_char) == 0 &&
                (strcmp(Type[(*ptr).Katuyou_Kata as usize].name as
                            *const libc::c_char,
                        b"\xe3\x83\x8a\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\x00"
                            as *const u8 as *const libc::c_char) == 0 ||
                    strcmp(Type[(*ptr).Katuyou_Kata as usize].name as
                               *const libc::c_char,
                           b"\xe3\x83\x8a\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\xe7\x89\xb9\xe6\xae\x8a\x00"
                               as *const u8 as *const libc::c_char) == 0
                    ||
                    strcmp(Type[(*ptr).Katuyou_Kata as usize].name as
                               *const libc::c_char,
                           b"\xe3\x83\x8a\xe3\x83\x8e\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\x00"
                               as *const u8 as *const libc::c_char) == 0)
            {
                add_len -= 2 as libc::c_int
            }
            strncat(str_buffer, (*ptr).Goi.as_mut_ptr(),
                    add_len as libc::c_ulong);
            /* 原形 */
        } else {
            strcat(str_buffer, (*ptr).Goi2.as_mut_ptr());
            /* 表記 */
        }
    }
    return add_len;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn bgh_code_match_for_sm(mut result_code:
                                               *mut libc::c_char,
                                               mut sm: *mut libc::c_char)
                                               -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    code = db_get(tools::sm2code_db, sm);
    if !code.is_null() {
        i = 0 as libc::c_int;
        while *code.offset(i as isize) != 0 {
            if case_match::_cf_match_element(result_code, code.offset(i as isize),
                                             0 as libc::c_int,
                                             code_depth(code.offset(i as isize),
                                                        11 as libc::c_int) +
                                                 1 as libc::c_int) != 0 {
                strcat(result_code, sm);
                free(code as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            i += 11 as libc::c_int
        }
        free(code as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn make_key_and_get_code(mut ptr: *mut tools::BNST_DATA,
                                               mut strt: libc::c_int,
                                               mut end: libc::c_int,
                                               mut str_buffer: *mut libc::c_char,
                                               mut ret_buffer: *mut libc::c_char,
                                               mut used_key: *mut libc::c_char,
                                               mut flag: libc::c_int)
{
    let mut fpp: *mut *mut tools::FEATURE =
        &mut (*(*ptr).mrph_ptr.offset(end as isize)).f;
    let mut m: dic::MRPH_DATA =
        dic::MRPH_DATA {
            type_0: 0,
            num: 0,
            parent: 0 as *mut tools::tnode_b,
            child: [0 as *mut tools::tnode_b; 32],
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
            f: 0 as *mut tools::_FEATURE,
            Num: 0,
            SM: 0 as *mut libc::c_char,
            Pos: [0; 4],
            Type: [0; 9],
        };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_key: [libc::c_char; 256] = [0; 256];
    let mut add_len: libc::c_int = 0;
    /* 末尾まで文字列を作り出し終わり、DBを引く */
    if strt > end {
        let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
        /* 「サ変+する」がなく「する」だけになるような場合はskip */
        if end > 0 as libc::c_int &&
            strcmp(str_buffer,
                   b"\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
                       *const libc::c_char) == 0 {
            return;
        }
        if *str_buffer.offset(strlen(str_buffer).wrapping_sub(1 as libc::c_int
            as
            libc::c_ulong)
            as isize) as libc::c_int == '+' as i32 {
            *str_buffer.offset(strlen(str_buffer).wrapping_sub(1 as
                libc::c_int
                as
                libc::c_ulong)
                as isize) = '\u{0}' as i32 as libc::c_char
        }
        code = get_str_code(str_buffer as *mut libc::c_uchar, flag);
        if !code.is_null() {
            /* DBをひく */
            strcat(ret_buffer, code);
            free(code as *mut libc::c_void);
            if strlen(used_key).wrapping_add(strlen(str_buffer)).wrapping_add(3 as libc::c_int as libc::c_ulong)
                > 256 as libc::c_int as libc::c_ulong {
                overflowed_function(used_key, 256 as libc::c_int,
                                    b"make_key_and_get_code\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char);
                return;
            }
            if *used_key != 0 {
                strcat(used_key,
                       b"|\x00" as *const u8 as *const libc::c_char);
            }
            strcat(used_key, str_buffer);
        }
        return;
    } else {
        /* 複合名詞の前の部分 (表記のみ, 代表表記やALTは用いていない) */
        if strt < end {
            if flag & 32 as libc::c_int != 0 {
                buf =
                    get_mrph_rep_from_f((*ptr).mrph_ptr.offset(strt as isize),
                                        0 as libc::c_int); /* 表記 */
                if buf.is_null() {
                    buf =
                        (*(*ptr).mrph_ptr.offset(strt as
                            isize)).Goi2.as_mut_ptr()
                }
            } else {
                buf =
                    (*(*ptr).mrph_ptr.offset(strt as isize)).Goi2.as_mut_ptr()
            }
            if strlen(str_buffer).wrapping_add(strlen(buf)).wrapping_add(3 as
                libc::c_int
                as
                libc::c_ulong)
                > 256 as libc::c_int as libc::c_ulong {
                overflowed_function(str_buffer, 256 as libc::c_int,
                                    b"make_key_and_get_code\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char);
                return;
            }
            strcat(str_buffer, buf);
            if flag & 32 as libc::c_int != 0 {
                strcat(str_buffer,
                       b"+\x00" as *const u8 as *const libc::c_char);
            }
            make_key_and_get_code(ptr, strt + 1 as libc::c_int, end,
                                  str_buffer, ret_buffer, used_key, flag);
            *str_buffer.offset(strlen(str_buffer).wrapping_sub(strlen(buf)) as
                isize) = '\u{0}' as i32 as libc::c_char;
            return;
        }
    }
    /* strt == end => 最後原形 */
    add_len =
        add_rep_str((*ptr).mrph_ptr.offset(end as isize), str_buffer,
                    (0 as libc::c_int == 0) as libc::c_int, flag);
    if add_len == 0 as libc::c_int {
        /* 代表表記 */
        return;
    }
    make_key_and_get_code(ptr, strt + 1 as libc::c_int, end, str_buffer,
                          ret_buffer, used_key, flag);
    strcpy(last_key.as_mut_ptr(), str_buffer);
    *str_buffer.offset(strlen(str_buffer).wrapping_sub(add_len as
        libc::c_ulong) as
        isize) = '\u{0}' as i32 as libc::c_char;
    /* ALTの代表表記 */
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
            add_len =
                add_rep_str(&mut m, str_buffer,
                            (0 as libc::c_int == 0) as libc::c_int, flag);
            if add_len == 0 as libc::c_int {
                /* 代表表記 */
                return;
            }
            if strcmp(last_key.as_mut_ptr(), str_buffer) != 0 {
                /* 異なる場合のみ調べる */
                make_key_and_get_code(ptr, strt + 1 as libc::c_int, end,
                                      str_buffer, ret_buffer, used_key, flag);
                *str_buffer.offset(strlen(str_buffer).wrapping_sub(add_len as
                    libc::c_ulong)
                    as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
        }
        fpp = &mut (**fpp).next
    }
    /* 「幸運を得る」の「幸運」を引くためには、
       「幸運/こううんa」ではなく、もとの「幸運だ/こううんだ」で引く必要がある */
    if *str_buffer.offset(0 as libc::c_int as isize) == 0 &&
        {
            buf =
                check_feature((*(*ptr).mrph_ptr.offset(end as isize)).f,
                              b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            !buf.is_null()
        } {
        /* 最後の要素 */
        strcpy(str_buffer,
               buf.offset(strlen(b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00"
                   as *const u8 as *const libc::c_char) as
                   isize));
        if strcmp(last_key.as_mut_ptr(), str_buffer) != 0 {
            /* 異なる場合のみ調べる */
            make_key_and_get_code(ptr, strt + 1 as libc::c_int, end,
                                  str_buffer, ret_buffer, used_key, flag);
            *str_buffer.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_bnst_code(mut ptr: *mut types::BNST_DATA, mut flag: libc::c_int)
/*==================================================================*/
{
    /* 文節の意味素コードを取得

       複合語の扱い
       		まず付属語を固定，自立語を減らしていく
		各形態素列に対して表記列で調べる

       分類語彙表の場合:
       「する」以外の付属語の動詞は削除する
       「結婚し始める」: 「始める」は削除し、「結婚する」で検索
       (分類語彙表ではサ変名詞は「する」付きで登録されている)
    */
    let mut strt: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    // let mut i: libc::c_int = 0;
    let mut lookup_pos: libc::c_int = 0 as libc::c_int;
    let mut str_buffer: [libc::c_char; 256] = [0; 256];
    let mut used_key: [libc::c_char; 256] = [0; 256];
    // let mut code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_code: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_num: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut exist: libc::c_int = 0;
    let mut code_unit: libc::c_int = 0;
    if flag == 1 as libc::c_int {
        result_code = (*ptr).BGH_code.as_mut_ptr();
        result_num = &mut (*ptr).BGH_num
    } else {
        result_code = (*ptr).SM_code.as_mut_ptr();
        result_num = &mut (*ptr).SM_num
    }
    exist = (*THESAURUS.as_mut_ptr().offset(flag as isize)).exist;
    code_unit = (*THESAURUS.as_mut_ptr().offset(flag as isize)).code_size;
    if exist == 0 as libc::c_int { return; }
    /* 初期化 */
    *result_code = '\u{0}' as i32 as libc::c_char;
    used_key[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    str_buffer[(256 as libc::c_int - 1 as libc::c_int) as usize] =
        '\n' as i32 as libc::c_char;
    /* result_num はinit_bnstで0に初期化されている */
    if flag == 1 as libc::c_int &&
        (*ptr).mrph_ptr.offset((*ptr).mrph_num as
            isize).offset(-(1 as libc::c_int as
            isize)) >
            (*ptr).head_ptr &&
        strcmp(Class[(*(*ptr).head_ptr.offset(1 as libc::c_int as
            isize)).Hinshi as
            usize][0 as libc::c_int as usize].id as
                   *const libc::c_char,
               b"\xe5\x8b\x95\xe8\xa9\x9e\x00" as *const u8 as
                   *const libc::c_char) == 0 &&
        strcmp((*(*ptr).head_ptr.offset(1 as libc::c_int as
            isize)).Goi.as_mut_ptr(),
               b"\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8 as
                   *const libc::c_char) == 0 {
        end =
            ((*ptr).head_ptr.wrapping_offset_from((*ptr).mrph_ptr) as
                libc::c_long + 1 as libc::c_int as libc::c_long) as
                libc::c_int
    } else {
        end =
            (*ptr).head_ptr.wrapping_offset_from((*ptr).mrph_ptr) as
                libc::c_long as libc::c_int
    }
    /* NTT: カウンタのみで引く */
    if flag == 2 as libc::c_int &&
        !check_feature((*(*ptr).head_ptr).f,
                       b"\xe3\x82\xab\xe3\x82\xa6\xe3\x83\xb3\xe3\x82\xbf\x00"
                           as *const u8 as *const libc::c_char as
                           *mut libc::c_char).is_null() {
        lookup_pos = 8 as libc::c_int;
        strt = end
    } else if (*ptr).type_0 == 2 as libc::c_int {
        strt = (*(ptr as *mut types::TAG_DATA)).settou_num
    } else { strt = 0 as libc::c_int }
    /* もっとも長いものから順に試す */
    while strt <= end {
        str_buffer[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        make_key_and_get_code(ptr, strt, end, str_buffer.as_mut_ptr(),
                              result_code, used_key.as_mut_ptr(),
                              flag | lookup_pos | OptUseRN);
        if *result_code != 0 {
            if flag == 1 as libc::c_int &&
                strstr(result_code,
                       b"sm\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                if bgh_code_match_for_sm(result_code,
                                         b"sm-sub*****\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char) != 0 {
                    assign_cfeature(&mut (*ptr).f,
                                    b"SM-\xe4\xb8\xbb\xe4\xbd\x93\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                if bgh_code_match_for_sm(result_code,
                                         b"sm-act*****\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char) != 0 {
                    assign_cfeature(&mut (*ptr).f,
                                    b"SM-\xe5\x8b\x95\xe4\xbd\x9c\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                if bgh_code_match_for_sm(result_code,
                                         b"sm-per*****\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char) != 0 {
                    assign_cfeature(&mut (*ptr).f,
                                    b"SM-\xe4\xba\xba\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                if bgh_code_match_for_sm(result_code,
                                         b"sm-loc*****\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char) != 0 {
                    assign_cfeature(&mut (*ptr).f,
                                    b"SM-\xe5\xa0\xb4\xe6\x89\x80\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
                if bgh_code_match_for_sm(result_code,
                                         b"sm-org*****\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char) != 0 {
                    assign_cfeature(&mut (*ptr).f,
                                    b"SM-\xe7\xb5\x84\xe7\xb9\x94\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 0 as libc::c_int);
                }
            }
            break;
        } else { strt += 1 }
    }
    if *result_code != 0 {
        let mut feature_buffer: [libc::c_char; 260] = [0; 260];
        *result_num =
            strlen(result_code).wrapping_div(code_unit as libc::c_ulong) as
                libc::c_int;
        if flag == 1 as libc::c_int {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"BGH:%s\x00" as *const u8 as *const libc::c_char,
                    used_key.as_mut_ptr());
        } else {
            sprintf(feature_buffer.as_mut_ptr(),
                    b"NTT:%s\x00" as *const u8 as *const libc::c_char,
                    used_key.as_mut_ptr());
        }
        assign_cfeature(&mut (*ptr).f, feature_buffer.as_mut_ptr(),
                        0 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn code_depth(mut cp: *mut libc::c_char,
                                    mut code_size: libc::c_int)
                                    -> libc::c_int
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    /* 意味素コードの深さを返す関数 (0 .. code_size-1) */
    i = 1 as libc::c_int;
    while i < code_size {
        if *cp.offset(i as isize) as libc::c_int == '*' as i32 {
            return i - 1 as libc::c_int;
        }
        i += 1
    }
    return code_size - 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn general_code_match(mut th: *mut THESAURUS_FILE,
                                            mut c1: *mut libc::c_char,
                                            mut c2: *mut libc::c_char)
                                            -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    d1 = code_depth(c1, (*th).code_size);
    d2 = code_depth(c2, (*th).code_size);
    if d1 + d2 == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    min = if d1 < d2 { d1 } else { d2 };
    if min == 0 as libc::c_int { return 0 as libc::c_int as libc::c_float; }
    l = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *(*th).format.offset(i as isize) != 0 {
        /* 指定された桁数ごとにチェック */
        if strncmp(c1.offset(l as isize), c2.offset(l as isize),
                   *(*th).format.offset(i as isize) as libc::c_ulong) != 0 {
            return 2 as libc::c_int as libc::c_float * l as libc::c_float /
                (d1 + d2) as libc::c_float;
        }
        l += *(*th).format.offset(i as isize);
        i += 1
    }
    return 2 as libc::c_int as libc::c_float * min as libc::c_float /
        (d1 + d2) as libc::c_float;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_similarity(mut exd: *mut libc::c_char,
                                         mut exp: *mut libc::c_char,
                                         mut expand: libc::c_int)
                                         -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code_size: libc::c_int = 0;
    let mut score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut tempscore: libc::c_float = 0.;
    /* 類似度計算: 意味素 - 意味素 */
    /* どちらかに用例のコードがないとき */
    if !(!exd.is_null() && !exp.is_null() && *exd as libc::c_int != 0 &&
        *exp as libc::c_int != 0) {
        return score;
    }
    if Thesaurus == -(1 as libc::c_int) {
        return score;
    } else {
        if Thesaurus == 2 as libc::c_int {
            if expand != 1 as libc::c_int { expand = 4 as libc::c_int }
        }
    }
    code_size =
        (*THESAURUS.as_mut_ptr().offset(Thesaurus as isize)).code_size;
    /* 最大マッチスコアを求める */
    j = 0 as libc::c_int;
    while *exp.offset(j as isize) != 0 {
        i = 0 as libc::c_int;
        while *exd.offset(i as isize) != 0 {
            if Thesaurus == 1 as libc::c_int {
                tempscore =
                    bgh_code_match_for_case(exp.offset(j as isize),
                                            exd.offset(i as isize)) as
                        libc::c_float
            } else if Thesaurus == 2 as libc::c_int {
                tempscore =
                    ntt_code_match(exp.offset(j as isize),
                                   exd.offset(i as isize), expand)
            } else {
                tempscore =
                    general_code_match(&mut *THESAURUS.as_mut_ptr().offset(Thesaurus
                        as
                        isize),
                                       exp.offset(j as isize),
                                       exd.offset(i as isize))
            }
            if tempscore > score { score = tempscore }
            i += code_size
        }
        j += code_size
    }
    /* スコアの幅に注意
       NTT: 0 〜 1.0
       BGH: 0 〜 7 */
    if Thesaurus == 1 as libc::c_int {
        score /= 7 as libc::c_int as libc::c_float
    }
    /* スコア: 0 〜 1.0 */
    return score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_most_similar_code(mut exd: *mut libc::c_char,
                                               mut exp: *mut libc::c_char)
                                               -> *mut libc::c_char
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code_size: libc::c_int = 0;
    let mut ret_sm_num: libc::c_int = 0 as libc::c_int;
    let mut pre_i: libc::c_int = -(1 as libc::c_int);
    let mut score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut tempscore: libc::c_float = 0.;
    let mut ret_sm: *mut libc::c_char = 0 as *mut libc::c_char;
    /* どちらかに用例のコードがないとき */
    if !(!exd.is_null() && !exp.is_null() && *exd as libc::c_int != 0 &&
        *exp as libc::c_int != 0) {
        return 0 as *mut libc::c_char;
    }
    if Thesaurus == -(1 as libc::c_int) { return 0 as *mut libc::c_char; }
    code_size =
        (*THESAURUS.as_mut_ptr().offset(Thesaurus as isize)).code_size;
    ret_sm = malloc_data((::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(strlen(exd)).wrapping_add(1 as libc::c_int as libc::c_ulong), b"get_most_similar_code\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) as *mut libc::c_char;
    *ret_sm = '\u{0}' as i32 as libc::c_char;
    /* 最大マッチスコアを求める */
    i = 0 as libc::c_int;
    while *exd.offset(i as isize) != 0 {
        j = 0 as libc::c_int;
        while *exp.offset(j as isize) != 0 {
            if Thesaurus == 1 as libc::c_int {
                tempscore =
                    bgh_code_match_for_case(exp.offset(j as isize),
                                            exd.offset(i as isize)) as
                        libc::c_float
            } else if Thesaurus == 2 as libc::c_int {
                tempscore =
                    ntt_code_match(exp.offset(j as isize),
                                   exd.offset(i as isize), 1 as libc::c_int)
            } else {
                tempscore =
                    general_code_match(&mut *THESAURUS.as_mut_ptr().offset(Thesaurus
                        as
                        isize),
                                       exp.offset(j as isize),
                                       exd.offset(i as isize))
            }
            if tempscore > score {
                score = tempscore;
                strncpy(ret_sm, exd.offset(i as isize),
                        code_size as libc::c_ulong);
                ret_sm_num = 1 as libc::c_int;
                *ret_sm.offset(code_size as isize) =
                    '\u{0}' as i32 as libc::c_char;
                pre_i = i
            } else if tempscore == score && pre_i != i {
                /* 重複を避けるため直前のiとは違うときのみ */
                strncat(ret_sm, exd.offset(i as isize),
                        code_size as libc::c_ulong);
                ret_sm_num += 1;
                pre_i = i
            }
            j += code_size
        }
        i += code_size
    }
    return ret_sm;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_word_similarity(mut exd: *mut libc::c_char,
                                              mut exp: *mut libc::c_char)
                                              -> libc::c_float
/*==================================================================*/
{
    let mut smd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut smp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score: libc::c_float = 0 as libc::c_int as libc::c_float;
    /* 類似度計算: 単語 - 単語 */
    smd = get_str_code(exd as *mut libc::c_uchar, Thesaurus);
    smp = get_str_code(exp as *mut libc::c_uchar, Thesaurus);
    if !smd.is_null() && !smp.is_null() {
        score = calc_similarity(smd, smp, 0 as libc::c_int)
    }
    if !smd.is_null() { free(smd as *mut libc::c_void); }
    if !smp.is_null() { free(smp as *mut libc::c_void); }
    return score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_sm_word_similarity(mut smd: *mut libc::c_char,
                                                 mut exp: *mut libc::c_char,
                                                 mut del: *mut libc::c_char,
                                                 mut expand: libc::c_int)
                                                 -> libc::c_float
/*==================================================================*/
{
    let mut smp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score: libc::c_float = 0 as libc::c_int as libc::c_float;
    /* 類似度計算: 意味素 - 単語 */
    smp = get_str_code(exp as *mut libc::c_uchar, Thesaurus);
    if smp.is_null() { return 0 as libc::c_int as libc::c_float; }
    if Thesaurus == 2 as libc::c_int && !del.is_null() {
        delete_specified_sm(smp, del);
    }
    if !smd.is_null() &&
        *smp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        score = calc_similarity(smd, smp, expand)
    }
    free(smp as *mut libc::c_void);
    return score;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_words_similarity(mut exd: *mut libc::c_char,
                                               mut exp:
                                               *mut *mut libc::c_char,
                                               mut num: libc::c_int,
                                               mut pos: *mut libc::c_int)
                                               -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut maxscore: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut score: libc::c_float = 0.;
    /* 類似度計算: 単語 - 単語群 */
    i = 0 as libc::c_int;
    while i < num {
        score = calc_word_similarity(exd, *exp.offset(i as isize));
        if maxscore < score {
            maxscore = score;
            *pos = i
        }
        i += 1
    }
    return maxscore;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_sm_words_similarity(mut smd: *mut libc::c_char,
                                                  mut exp:
                                                  *mut *mut libc::c_char,
                                                  mut num: libc::c_int,
                                                  mut pos: *mut libc::c_int,
                                                  mut del: *mut libc::c_char,
                                                  mut expand: libc::c_int,
                                                  mut unmatch_word:
                                                  *mut libc::c_char)
                                                  -> libc::c_float
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    let mut maxscore: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut score: libc::c_float = 0.;
    /* 類似度計算: 意味素 - 単語群 */
    i = 0 as libc::c_int;
    while i < num {
        if !(!unmatch_word.is_null() &&
            strcmp(*exp.offset(i as isize), unmatch_word) == 0) {
            score =
                calc_sm_word_similarity(smd, *exp.offset(i as isize), del,
                                        expand);
            if maxscore < score {
                maxscore = score;
                *pos = i
            }
        }
        /* マッチさせない単語 */
        i += 1
    }
    return maxscore;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_distsim_from_bnst(mut ptr1: *mut types::BNST_DATA, mut ptr2: *mut types::BNST_DATA)
                                                -> libc::c_int
/*==================================================================*/
{
    /* 分布類似度:
       返り値
       	一方でもDBにない場合 	: -1
	満点			: BGH_CODE_SIZE - 3 == 8
     */
    let mut rep1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut score: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    rep1 =
        get_bnst_head_canonical_rep(ptr1,
                                    (0 as libc::c_int == 0) as libc::c_int);
    rep2 =
        get_bnst_head_canonical_rep(ptr2,
                                    (0 as libc::c_int == 0) as libc::c_int);
    return if !rep1.is_null() && !rep2.is_null() {
        if check_feature((*ptr1).f,
                         b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                             *const libc::c_char as
                             *mut libc::c_char).is_null() &&
            check_feature((*ptr2).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            /* 体言同士 */
            if OptCaseFlag & 1048576 as libc::c_int != 0 {
                /* 代表表記をIDに変換 */
                let mut id1: *mut libc::c_char =
                    rep2id(rep1, strlen(rep1) as libc::c_int,
                           &mut *static_buffer1.as_mut_ptr().offset(0 as
                               libc::c_int
                               as
                               isize));
                let mut id2: *mut libc::c_char =
                    rep2id(rep2, strlen(rep2) as libc::c_int,
                           &mut *static_buffer2.as_mut_ptr().offset(0 as
                               libc::c_int
                               as
                               isize));
                if *id1.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    &&
                    *id2.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0 {
                    score = calc_distsim(id1, id2)
                } else { return score as libc::c_int; }
            } else { score = calc_distsim(rep1, rep2) }
            score =
                score / 1.00f64 *
                    (11 as libc::c_int - 3 as libc::c_int) as libc::c_double
        } else {
            /* 用言同士 */
            score = 0 as libc::c_int as libc::c_double
        }
        /* fprintf(stderr, ";; DistSim <%s> <%s>:%.5f\n", rep1, rep2, score); */
        if score >= (11 as libc::c_int - 3 as libc::c_int) as libc::c_double {
            11 as libc::c_int - 3 as libc::c_int
        } else { score as libc::c_int }
    } else { score as libc::c_int };
}
