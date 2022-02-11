#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
ptr_wrapping_offset_from, register_tool)]

use crate::{db, structs};
use crate::case_match::cf_match_element;
use crate::configfile::check_dict_filename;
use crate::ctools::{assign_cfeature, check_feature, exit, malloc_data, Outfp, stderr, stdout};
use crate::db::{db_close, db_get, db_read_open};
use crate::lib_sm::assign_sm;
use crate::read_data::assign_general_feature;
use crate::read_rule::case2num;
use crate::structs::{CDB_FILE, MRPH_DATA};
use crate::tools::{hash, OptDisplay, OptDisplayNE, OptNEcache, OptNEcase, OptNECRF, OptNEend, OptNElearn, OptNEparent};
use crate::types::{BNST_DATA, CF_PRED_MGR, DBM_FILE, FEATURE, SENTENCE_DATA, size_t};

#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
/* SVMの結果を確率に近似するシグモイド関数の係数 */
#[no_mangle]
pub static mut ne_db: DBM_FILE = 0 as *const  CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut DBforNE: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut TagPosition: [[libc::c_char; 20]; 33] = [[0; 20]; 33];
#[no_mangle]
pub static mut Tag_name: [*mut libc::c_char; 11] =
    [b"ORGANIZATION\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"PERSON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"LOCATION\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ARTIFACT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"DATE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"TIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"MONEY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"PERCENT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"OTHER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"OPTIONAL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut Position_name: [*mut libc::c_char; 5] =
    [b"head\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"middle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"tail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"single\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut Imi_feature: [*mut libc::c_char; 5] =
    [b"\xe7\xb5\x84\xe7\xb9\x94\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe4\xb8\xbb\xe4\xbd\x93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe5\xa0\xb4\xe6\x89\x80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut Chara_name: [*mut libc::c_char; 9] =
    [b"\xe6\xbc\xa2\xe5\xad\x97\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe3\x81\x8b\xe3\x81\xaa\xe6\xbc\xa2\xe5\xad\x97\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe3\x82\xab\xe3\x82\xbf\xe3\x82\xab\xe3\x83\x8a\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe8\x8b\xb1\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe6\x95\xb0\xe5\xad\x97\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x00\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
/* 最大スコアの経路 */
#[no_mangle]
pub static mut NE_mgr: [NE_MANAGER; 200] =
    [NE_MANAGER{feature: [0; 1024],
                notHEAD: 0,
                NEresult: 0,
                prob: [0.; 33],
                max: [0.; 33],
                parent: [0; 33],}; 200];
#[no_mangle]
pub static mut ne_cache: [*mut NE_CACHE; 1024] = [0 as *const NE_CACHE as *mut NE_CACHE; 1024];
/*====================================================================
		     タグ・ポジション−コード対応
====================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_tagposition() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            strcpy(TagPosition[(i * 4 as libc::c_int + j) as usize].as_mut_ptr(), Tag_name[i as usize]);
            strcat(TagPosition[(i * 4 as libc::c_int + j) as usize].as_mut_ptr(), b":\x00" as *const u8 as *const libc::c_char);
            strcat(TagPosition[(i * 4 as libc::c_int + j) as usize].as_mut_ptr(), Position_name[j as usize]);
            j += 1
        }
        i += 1
    }
    strcpy(TagPosition[32 as libc::c_int as usize].as_mut_ptr(),
           b"OTHER:single\x00" as *const u8 as *const libc::c_char);
}
/*====================================================================
		   タグ・ポジション−コード対応関数
====================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ne_tagposition_to_code(mut cp: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !TagPosition[i as usize].as_mut_ptr().is_null() {
        if strcmp(TagPosition[i as usize].as_mut_ptr(), cp) == 0 { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ne_code_to_tagposition(mut num: libc::c_int)
 -> *mut libc::c_char {
    return TagPosition[num as usize].as_mut_ptr();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_db_for_NE() 
 /*==================================================================*/
 {
    let mut db_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    db_filename =
        check_dict_filename(DBforNE, (0 as libc::c_int == 0) as libc::c_int);
    ne_db = db_read_open(db_filename);
    if ne_db.is_null() {
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Opening %s ... failed.\n\x00" as *const u8 as
                        *const libc::c_char, db_filename);
        }
        fprintf(stderr,
                b";; Cannot open POS table for NE <%s>.\n\x00" as *const u8 as
                    *const libc::c_char, db_filename);
        exit(1 as libc::c_int);
    } else {
        if OptDisplay == 3 as libc::c_int {
            fprintf(Outfp,
                    b"Opening %s ... done.\n\x00" as *const u8 as
                        *const libc::c_char, db_filename);
        }
    }
    free(db_filename as *mut libc::c_void);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn close_db_for_NE() {
    db_close(ne_db);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_ne_cache() {
    memset(ne_cache.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut NE_CACHE>() as
                libc::c_ulong).wrapping_mul(1024 as libc::c_int as
                                                libc::c_ulong));
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn clear_ne_cache() {
    let mut i: libc::c_int = 0;
    let mut ncp: *mut NE_CACHE = 0 as *mut NE_CACHE;
    let mut next: *mut NE_CACHE = 0 as *mut NE_CACHE;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !ne_cache[i as usize].is_null() {
            ncp = ne_cache[i as usize];
            while !ncp.is_null() {
                free((*ncp).key as *mut libc::c_void);
                next = (*ncp).next;
                free(ncp as *mut libc::c_void);
                ncp = next
            }
        }
        i += 1
    }
    init_ne_cache();
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn register_ne_cache(mut key: *mut libc::c_char, mut NEresult: libc::c_int) {
    /* NEの解析結果を登録する */
    let mut i: libc::c_int = 0;
    let mut ncpp: *mut *mut NE_CACHE = 0 as *mut *mut NE_CACHE;
    ncpp = &mut *ne_cache.as_mut_ptr().offset((hash as unsafe extern "C" fn(_: *mut libc::c_uchar, _: libc::c_int) -> libc::c_int)(key as *mut libc::c_uchar, (strlen as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_ulong)(key) as libc::c_int) as isize) as *mut *mut NE_CACHE;
    while !(*ncpp).is_null() && strcmp((**ncpp).key, key) != 0 {
        ncpp = &mut (**ncpp).next
    }
    if (*ncpp).is_null() {
        *ncpp = malloc_data(
            ::std::mem::size_of::<NE_CACHE>() as libc::c_ulong,
            b"register_ne_cache\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        ) as *mut NE_CACHE;
        i = 0 as libc::c_int;
        while i < 33 as libc::c_int {
            (**ncpp).ne_result[i as usize] = 0 as libc::c_int;
            i += 1
        }
        (**ncpp).key = strdup(key);
        (**ncpp).next = 0 as *mut ne_cache
    }
    (**ncpp).ne_result[NEresult as usize] = 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_ne_cache(mut key: *mut libc::c_char, mut NEresult: libc::c_int) -> libc::c_int {
    let mut ncp: *mut NE_CACHE = 0 as *mut NE_CACHE;
    ncp = ne_cache[hash(key as *mut libc::c_uchar, strlen(key) as libc::c_int) as usize];
    while !ncp.is_null() {
        if strcmp((*ncp).key, key) == 0 {
            return (*ncp).ne_result[NEresult as usize]
        }
        ncp = (*ncp).next
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_mrph_ne(mut fp: *mut FEATURE) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cp: [libc::c_char; 32] = [0; 32];
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int - 1 as libc::c_int {
        sprintf(cp.as_mut_ptr(), b"NE:%s\x00" as *const u8 as *const libc::c_char, ne_code_to_tagposition(i));
        if !check_feature(fp, cp.as_mut_ptr()).is_null() {
            return i
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        sprintf(cp.as_mut_ptr(), b"NE:OPTIONAL:%s\x00" as *const u8 as *const libc::c_char, Position_name[i as usize]);
        if !check_feature(fp, cp.as_mut_ptr()).is_null() {
            return 33 as libc::c_int + 3 as libc::c_int + i
        }
        i += 1
    }
    return 33 as libc::c_int - 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_chara(mut mrph_data: *mut MRPH_DATA)
 -> libc::c_int 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* 記号 */
    if !(*mrph_data).Goi.as_mut_ptr().is_null() && strncmp((*mrph_data).Goi.as_mut_ptr(), b"\xe3\x83\xbb\x00" as *const u8 as *const libc::c_char, 3 as libc::c_int as libc::c_ulong) == 0 {
        return 5 as libc::c_int
    }
    i = 0 as libc::c_int;
    while strcmp(Chara_name[i as usize], b"\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\x00" as *const u8 as *const libc::c_char) != 0 {
        if !check_feature((*mrph_data).f, Chara_name[i as usize]).is_null() {
            break ;
        }
        i += 1
    }
    return i + 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_pos(mut ret: *mut libc::c_char, mut mrph_data: *mut MRPH_DATA, mut num: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0; /* 再帰的に代入するため */
    let mut j: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut pos: [libc::c_char; 128] = [0; 128];
    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    /* 品詞曖昧性のある場合 */
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int + 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 128 as libc::c_int + 1 as libc::c_int {
            if !Class[i as usize][j as usize].id.is_null() {
                sprintf(pos.as_mut_ptr(), b"\xe5\x93\x81\xe6\x9b\x96-%s\x00" as *const u8 as *const libc::c_char, Class[i as usize][j as usize].id);
                if !check_feature((*mrph_data).f, pos.as_mut_ptr()).is_null() {
                    if OptNECRF != 0 {
                        sprintf(buf.as_mut_ptr(), b"%s:%s\x00" as *const u8 as *const libc::c_char, ret, Class[i as usize][j as usize].id);
                    } else {
                        sprintf(buf.as_mut_ptr(), b"%s%d%d%d10:1 \x00" as *const u8 as *const libc::c_char, ret, i, j, num);
                    }
                    strcpy(ret, buf.as_mut_ptr());
                    flag += 1
                }
            }
            j += 1
        }
        i += 1
    }
    if flag > 1 as libc::c_int || flag != 0 && OptNECRF != 0 { return ret }
    /* 品詞曖昧性のない場合 */
    if OptNECRF != 0 {
        sprintf(ret, b":%s\x00" as *const u8 as *const libc::c_char, Class[(*mrph_data).Hinshi as usize][(*mrph_data).Bunrui as usize].id);
        return ret
    }
    if (*mrph_data).Bunrui != 0 {
        sprintf(ret, b"%d%d%d10:1 \x00" as *const u8 as *const libc::c_char, (*mrph_data).Hinshi, (*mrph_data).Bunrui, num);
    } else {
        sprintf(ret, b"%d0%d10:1 \x00" as *const u8 as *const libc::c_char, (*mrph_data).Hinshi, num);
    }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn get_cache(mut ret: *mut libc::c_char, mut key: *mut libc::c_char, mut num: libc::c_int) -> *mut libc::c_char {
    let mut NEresult: libc::c_int = 0; /* 再帰的に代入するため */
    // let mut ncp: *mut NE_CACHE = 0 as *mut NE_CACHE;
    let mut buf: [libc::c_char; 256] = [0; 256];
    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    NEresult = 0 as libc::c_int;
    while NEresult < 33 as libc::c_int - 1 as libc::c_int {
        if check_ne_cache(key, NEresult) != 0 {
            if OptNECRF != 0 {
                sprintf(buf.as_mut_ptr(), b"%s:%d\x00" as *const u8 as *const libc::c_char, ret, NEresult);
            } else {
                sprintf(buf.as_mut_ptr(), b"%s%d%d30:1 \x00" as *const u8 as *const libc::c_char, ret, NEresult + 1 as libc::c_int, num);
            }
            strcpy(ret, buf.as_mut_ptr());
        }
        NEresult += 1
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_feature(mut ret: *mut libc::c_char, mut mrph_data: *mut MRPH_DATA, mut num: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0; /* 再帰的に代入するため */
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feature_name: [*mut libc::c_char; 3] = [b"\xe4\xba\xba\xe5\x90\x8d\xe6\x9c\xab\xe5\xb0\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char, b"\xe7\xb5\x84\xe7\xb9\x94\xe5\x90\x8d\xe6\x9c\xab\xe5\xb0\xbe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char, 0 as *mut libc::c_char];
    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    /* 文節後方に人名末尾、組織名末尾という語があるか */
    j = 1 as libc::c_int;
    while !((*mrph_data.offset(j as isize)).f.is_null() ||
                !check_feature((*mrph_data.offset(j as isize)).f,
                               b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*mrph_data.offset(j as isize)).f,
                               b"\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                !check_feature((*mrph_data.offset(j as isize)).f,
                               b"\xe6\x8b\xac\xe5\xbc\xa7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null()
    ) {
        i = 0 as libc::c_int;
        while !feature_name[i as usize].is_null() {
            if !check_feature((*mrph_data.offset(j as isize)).f, feature_name[i as usize]).is_null() {
                if OptNECRF != 0 {
                    sprintf(ret, b"H:%s \x00" as *const u8 as *const libc::c_char, feature_name[i as usize]);
                } else {
                    sprintf(ret, b"%d%d40:1 \x00" as *const u8 as *const libc::c_char, i + 3 as libc::c_int, num);
                }
            }
            i += 1
        }
        j += 1
    }
    /* 人名末尾、組織名末尾であるか */
    i = 0 as libc::c_int;
    while !feature_name[i as usize].is_null() {
        if !check_feature((*mrph_data).f, feature_name[i as usize]).is_null() {
            if OptNECRF != 0 {
                sprintf(buf.as_mut_ptr(), b"S:%s \x00" as *const u8 as *const libc::c_char, feature_name[i as usize]);
            } else {
                sprintf(buf.as_mut_ptr(), b"%s%d%d40:1 \x00" as *const u8 as *const libc::c_char, ret, i + 1 as libc::c_int, num);
            }
            strcpy(ret, buf.as_mut_ptr());
        }
        i += 1
    }
    if OptNECRF == 0 { return ret }
    /* 以下はOptNECRFの場合のみ実行 */
    if *ret.offset(0 as libc::c_int as isize) == 0 {
        sprintf(ret, b"NIL \x00" as *const u8 as *const libc::c_char);
    }
    /* カテゴリの情報 */
    strcat(ret, b"CT\x00" as *const u8 as *const libc::c_char);
    cp = check_feature(
        (*mrph_data).f,
        b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if !cp.is_null() {
        strcat(ret, cp.offset(strlen(b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00" as *const u8 as *const libc::c_char) as isize));
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_parent(mut ret: *mut libc::c_char, mut mrph_data: *mut MRPH_DATA, mut num: libc::c_int) -> *mut libc::c_char {
    let mut j: libc::c_int = 0; /* 再帰的に代入するため */
    let mut c: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut pcp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ncp: *mut libc::c_char = 0 as *mut libc::c_char;
    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    if num != 2 as libc::c_int + 1 as libc::c_int { return ret }
    pcp = check_feature(
        (*mrph_data).f,
        b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !pcp.is_null() {
        if OptNECRF != 0 {
            strcpy(ret, b"CS\x00" as *const u8 as *const libc::c_char);
        }
        ccp =
            check_feature(
                (*mrph_data).f,
                b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !ccp.is_null() {
            if OptNECRF != 0 {
                sprintf(buf.as_mut_ptr(), b"%s:%s\x00" as *const u8 as *const libc::c_char, ret, ccp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize));
            } else {
                c = case2num(ccp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize)) + 3 as libc::c_int;
                if strcmp(b"\xef\xbc\xb4\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char, ccp) == 0 {
                    c = 1 as libc::c_int
                }
                sprintf(buf.as_mut_ptr(), b"%s%d60:1 \x00" as *const u8 as *const libc::c_char, ret, c);
            }
            strcpy(ret, buf.as_mut_ptr());
        }
        if OptNECRF != 0 {
            sprintf(buf.as_mut_ptr(), b"%s P:%s\x00" as *const u8 as *const libc::c_char, ret, pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize));
        } else {
            ncp =
                db_get(ne_db,
                       pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize));
            sprintf(buf.as_mut_ptr(), b"%s%s6:1 \x00" as *const u8 as *const libc::c_char, ret,
                    if !ncp.is_null() {
                        ncp as *const libc::c_char
                    } else {
                        b"\x00" as *const u8 as *const libc::c_char
                    }
            );
            free(ncp as *mut libc::c_void);
        }
        strcpy(ret, buf.as_mut_ptr());
    }
    /* 文節後方にあるか */
    if OptNECRF == 0 ||
           strstr(ret,
                  b"CS\x00" as *const u8 as *const libc::c_char).is_null() {
        j = 1 as libc::c_int;
        while !((*mrph_data.offset(j as isize)).f.is_null() ||
                    !check_feature(
                        (*mrph_data.offset(j as isize)).f,
                        b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() ||
                    !check_feature(
                        (*mrph_data.offset(j as isize)).f,
                        b"\xe6\x8b\xac\xe5\xbc\xa7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null()) {
            pcp = check_feature(
                (*mrph_data.offset(j as isize)).f,
                b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
            if !pcp.is_null() {
                if OptNECRF != 0 {
                    strcpy(ret, b"CS\x00" as *const u8 as *const libc::c_char);
                }
                ccp = check_feature(
                    (*mrph_data.offset(j as isize)).f,
                    b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                if !ccp.is_null() {
                    if OptNECRF != 0 {
                        sprintf(buf.as_mut_ptr(), b"%s:%s\x00" as *const u8 as *const libc::c_char, ret, ccp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize));
                    } else {
                        c = case2num(ccp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\x00" as *const u8 as *const libc::c_char) as isize).offset(1 as libc::c_int as isize)) + 3 as libc::c_int;
                        if strcmp(b"\xef\xbc\xb4\xe4\xbf\x82:\xe6\x9c\xaa\xe6\xa0\xbc\x00" as *const u8 as *const libc::c_char, ccp) == 0 {
                            c = 1 as libc::c_int
                        }
                        sprintf(buf.as_mut_ptr(), b"%s%d70:1 \x00" as *const u8 as *const libc::c_char, ret, c);
                    }
                    strcpy(ret, buf.as_mut_ptr());
                }
                if OptNECRF != 0 {
                    sprintf(buf.as_mut_ptr(),
                            b"%s P:%s\x00" as *const u8 as
                                *const libc::c_char, ret,
                            pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                                  as *const u8 as
                                                  *const libc::c_char) as
                                           isize).offset(1 as libc::c_int as
                                                             isize));
                } else {
                    ncp =
                        db_get(ne_db,
                               pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                                     as *const u8 as
                                                     *const libc::c_char) as
                                              isize).offset(1 as libc::c_int
                                                                as isize));
                    sprintf(buf.as_mut_ptr(),
                            b"%s%s7:1 \x00" as *const u8 as
                                *const libc::c_char, ret,
                            if !ncp.is_null() {
                                ncp as *const libc::c_char
                            } else {
                                b"\x00" as *const u8 as *const libc::c_char
                            });
                    free(ncp as *mut libc::c_void);
                }
                strcpy(ret, buf.as_mut_ptr());
                break ;
            } else { j += 1 }
        }
    }
    if OptNECRF != 0 &&
           strstr(ret, b"P\x00" as *const u8 as *const libc::c_char).is_null()
       {
        strcat(ret, b"NIL NIL\x00" as *const u8 as *const libc::c_char);
    }
    if OptNECRF != 0 &&
           !(!check_feature((*mrph_data).f,
                            b"\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char).is_null() ||
                 !check_feature((*mrph_data).f,
                                b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut libc::c_char).is_null()) &&
           check_feature((*mrph_data).f,
                         b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e\x00" as
                             *const u8 as *const libc::c_char as
                             *mut libc::c_char).is_null() {
        strcat(ret, b" NIL\x00" as *const u8 as *const libc::c_char);
    }
    pcp =
        check_feature((*mrph_data).f,
                      b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char);
    if !pcp.is_null() {
        if OptNECRF != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s H:%s\x00" as *const u8 as *const libc::c_char, ret,
                    pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char)
                                   as
                                   isize).offset(1 as libc::c_int as isize));
        } else {
            ncp =
                db_get(ne_db,
                       pcp.offset(strlen(b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                             as *const u8 as
                                             *const libc::c_char) as
                                      isize).offset(1 as libc::c_int as
                                                        isize));
            sprintf(buf.as_mut_ptr(),
                    b"%s%s9:1 \x00" as *const u8 as *const libc::c_char, ret,
                    if !ncp.is_null() {
                        ncp as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
            free(ncp as *mut libc::c_void);
        }
        strcpy(ret, buf.as_mut_ptr());
    }
    if OptNECRF != 0 &&
           (!check_feature((*mrph_data).f,
                           b"\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char).is_null() ||
                !check_feature((*mrph_data).f,
                               b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null()) {
        if strlen((*mrph_data).Goi2.as_mut_ptr()) <
               (128 as libc::c_int / 3 as libc::c_int) as libc::c_ulong {
            sprintf(buf.as_mut_ptr(),
                    b"%s S:%s\x00" as *const u8 as *const libc::c_char, ret,
                    (*mrph_data).Goi2.as_mut_ptr());
        } else {
            sprintf(buf.as_mut_ptr(),
                    b"%s S:LONG_WORD\x00" as *const u8 as *const libc::c_char,
                    ret);
        }
        strcpy(ret, buf.as_mut_ptr());
    }
    if OptNECRF != 0 {
        if !check_feature((*mrph_data).f,
                          b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00" as
                              *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
               (!check_feature((*mrph_data).f,
                               b"\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                   as *const u8 as *const libc::c_char as
                                   *mut libc::c_char).is_null() ||
                    !check_feature((*mrph_data).f,
                                   b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) {
            strcat(ret, b" SINGLE\x00" as *const u8 as *const libc::c_char);
        } else if !check_feature((*mrph_data).f,
                                 b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            strcat(ret, b" START\x00" as *const u8 as *const libc::c_char);
        } else if !check_feature((*mrph_data).f,
                                 b"\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() ||
                      !check_feature((*mrph_data).f,
                                     b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_char).is_null() {
            strcat(ret, b" END\x00" as *const u8 as *const libc::c_char);
        } else if !check_feature((*mrph_data).f,
                                 b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
            strcat(ret, b" INTER\x00" as *const u8 as *const libc::c_char);
        } else {
            strcat(ret, b" OTHER\x00" as *const u8 as *const libc::c_char);
        }
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_imi(mut ret: *mut libc::c_char,
                                 mut mrph_data: *mut MRPH_DATA,
                                 mut num: libc::c_int) -> *mut libc::c_char 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* 再帰的に代入するため */
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut cp: [libc::c_char; 128] = [0; 128];
    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    if num != 2 as libc::c_int + 1 as libc::c_int { return ret }
    /* 組織、人、主体、場所 */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        sprintf(cp.as_mut_ptr(),
                b"\xe6\x84\x8f\xe5\x91\xb3-%s\x00" as *const u8 as
                    *const libc::c_char, Imi_feature[i as usize]);
        /* 意味素性があるか */
        if !check_feature((*mrph_data).f, cp.as_mut_ptr()).is_null() {
            sprintf(buf.as_mut_ptr(),
                    b"%s%d180:1 \x00" as *const u8 as *const libc::c_char,
                    ret, i + 1 as libc::c_int);
            strcpy(ret, buf.as_mut_ptr());
        }
        /* 文節後方にあるか */
        j = 1 as libc::c_int;
        while !((*mrph_data.offset(j as isize)).f.is_null() ||
                    !check_feature((*mrph_data.offset(j as isize)).f,
                                   b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() ||
                    !check_feature((*mrph_data.offset(j as isize)).f,
                                   b"\xe6\x8b\xac\xe5\xbc\xa7\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) {
            if !check_feature((*mrph_data.offset(j as isize)).f,
                              cp.as_mut_ptr()).is_null() {
                sprintf(buf.as_mut_ptr(),
                        b"%s%d280:1 \x00" as *const u8 as *const libc::c_char,
                        ret, i + 1 as libc::c_int);
                strcpy(ret, buf.as_mut_ptr());
                break ;
            } else { j += 1 }
        }
        i += 1
    }
    /* 固有表現 */
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int - 1 as libc::c_int {
        sprintf(cp.as_mut_ptr(),
                b"\xe6\x84\x8f\xe5\x91\xb3-%s\x00" as *const u8 as
                    *const libc::c_char, Tag_name[i as usize]);
        /* 意味素性があるか */
        if !check_feature((*mrph_data).f, cp.as_mut_ptr()).is_null() {
            sprintf(buf.as_mut_ptr(),
                    b"%s%d380:1 \x00" as *const u8 as *const libc::c_char,
                    ret, i + 1 as libc::c_int);
            strcpy(ret, buf.as_mut_ptr());
        }
        /* 文節後方にあるか */
        j = 1 as libc::c_int;
        while !((*mrph_data.offset(j as isize)).f.is_null() ||
                    !check_feature((*mrph_data.offset(j as isize)).f,
                                   b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null() ||
                    !check_feature((*mrph_data.offset(j as isize)).f,
                                   b"\xe6\x8b\xac\xe5\xbc\xa7\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char).is_null()) {
            if !check_feature((*mrph_data.offset(j as isize)).f,
                              cp.as_mut_ptr()).is_null() {
                sprintf(buf.as_mut_ptr(),
                        b"%s%d480:1 \x00" as *const u8 as *const libc::c_char,
                        ret, i + 1 as libc::c_int);
                strcpy(ret, buf.as_mut_ptr());
                break ;
            } else { j += 1 }
        }
        i += 1
    }
    return ret;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn intcmp(mut a: *const libc::c_void,
                                mut b: *const libc::c_void) -> libc::c_int 
 /*==================================================================*/
 {
    return *(b as *mut libc::c_int) - *(a as *mut libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_crf_feature(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut k: libc::c_int = 0;
    let mut s: [[libc::c_char; 256]; 4] = [[0; 256]; 4];
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        get_pos(s[0 as libc::c_int as usize].as_mut_ptr(),
                (*sp).mrph_data.offset(i as isize), 0 as libc::c_int);
        get_feature(s[1 as libc::c_int as usize].as_mut_ptr(),
                    (*sp).mrph_data.offset(i as isize), 0 as libc::c_int);
        get_parent(s[2 as libc::c_int as usize].as_mut_ptr(),
                   (*sp).mrph_data.offset(i as isize),
                   2 as libc::c_int + 1 as libc::c_int);
        get_cache(s[3 as libc::c_int as usize].as_mut_ptr(),
                  (*(*sp).mrph_data.offset(i as isize)).Goi2.as_mut_ptr(),
                  0 as libc::c_int);
        /* 見出し 品詞 品詞細分類 品詞曖昧性 文字種 文字数
	   (表層格 係り先の主辞 主辞 文節内位置) キャッシュ */
	/* featureは1024字まで */
        sprintf(NE_mgr[i as usize].feature.as_mut_ptr(),
                b"%s %s %s A%s %s L:%d %s %s C%s\x00" as *const u8 as
                    *const libc::c_char,
                if strlen((*(*sp).mrph_data.offset(i as
                                                       isize)).Goi2.as_mut_ptr())
                       <
                       (128 as libc::c_int / 3 as libc::c_int) as
                           libc::c_ulong {
                    (*(*sp).mrph_data.offset(i as isize)).Goi2.as_mut_ptr() as
                        *const libc::c_char
                } else {
                    b"LONG_WORD\x00" as *const u8 as *const libc::c_char
                },
                Class[(*(*sp).mrph_data.offset(i as isize)).Hinshi as
                          usize][0 as libc::c_int as usize].id,
                Class[(*(*sp).mrph_data.offset(i as isize)).Hinshi as
                          usize][(*(*sp).mrph_data.offset(i as isize)).Bunrui
                                     as usize].id,
                s[0 as libc::c_int as usize].as_mut_ptr(),
                Chara_name[(get_chara((*sp).mrph_data.offset(i as isize)) -
                                1 as libc::c_int) as usize],
                strlen((*(*sp).mrph_data.offset(i as
                                                    isize)).Goi2.as_mut_ptr()).wrapping_div(3
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong),
                s[1 as libc::c_int as usize].as_mut_ptr(),
                s[2 as libc::c_int as usize].as_mut_ptr(),
                if OptNEcache != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    s[3 as libc::c_int as usize].as_mut_ptr() as
                        *const libc::c_char
                });
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_svm_feature(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut f: [libc::c_int; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut s: [[libc::c_char; 256]; 5] = [[0; 256]; 5];
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bnstb: [libc::c_char; 7] = [0; 7];
    let mut bnsth: [libc::c_char; 7] = [0; 7];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        /* 括弧始を除く記号は固有表現の先頭にはならない(ルール)  */
        NE_mgr[i as usize].notHEAD = 0 as libc::c_int; /* 末尾空白 */
        if get_chara((*sp).mrph_data.offset(i as isize)) == 5 as libc::c_int
               &&
               check_feature((*(*sp).mrph_data.offset(i as isize)).f,
                             b"\xe6\x8b\xac\xe5\xbc\xa7\xe5\xa7\x8b\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
            NE_mgr[i as usize].notHEAD = 1 as libc::c_int
        } /* 末尾空白 */
        j = i - 2 as libc::c_int; /* 末尾空白 */
        while j <= i + 2 as libc::c_int {
            if !(j < 0 as libc::c_int || j >= (*sp).Mrph_num) {
                k =
                    i - j + 2 as libc::c_int +
                        1 as libc::c_int; /* 末尾空白 */
                id =
                    db_get(ne_db,
                           (*(*sp).mrph_data.offset(j as
                                                        isize)).Goi2.as_mut_ptr()); /* 末尾空白 */
                get_pos(s[0 as libc::c_int as usize].as_mut_ptr(),
                        (*sp).mrph_data.offset(j as isize),
                        k); /* 末尾空白 */
                get_cache(s[1 as libc::c_int as usize].as_mut_ptr(),
                          (*(*sp).mrph_data.offset(j as
                                                       isize)).Goi2.as_mut_ptr(),
                          k); /* 末尾空白 */
                get_feature(s[2 as libc::c_int as usize].as_mut_ptr(),
                            (*sp).mrph_data.offset(j as isize), k);
                get_parent(s[3 as libc::c_int as usize].as_mut_ptr(),
                           (*sp).mrph_data.offset(j as isize), k);
                get_imi(s[4 as libc::c_int as usize].as_mut_ptr(),
                        (*sp).mrph_data.offset(j as isize), k);
                if !check_feature((*(*sp).mrph_data.offset(j as isize)).f,
                                  b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    sprintf(bnstb.as_mut_ptr(),
                            b"%d00:1 \x00" as *const u8 as
                                *const libc::c_char, k);
                } else {
                    bnstb[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                };
                if !check_feature((*(*sp).mrph_data.offset(j as isize)).f,
                                  b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    sprintf(bnsth.as_mut_ptr(),
                            b"%d90:1 \x00" as *const u8 as
                                *const libc::c_char, k);
                } else {
                    bnsth[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                };
                sprintf(buf.as_mut_ptr(),
                        b"%s%s%d:1 %s%s%s%d%d20:1 %s%s%d%d50:1 %s%s\x00" as
                            *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                        if !id.is_null() {
                            id as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, k,
                        if bnstb[0 as libc::c_int as usize] as libc::c_int !=
                               0 {
                            bnstb.as_mut_ptr() as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        if bnsth[0 as libc::c_int as usize] as libc::c_int !=
                               0 {
                            bnsth.as_mut_ptr() as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, s[0 as libc::c_int as usize].as_mut_ptr(),
                        get_chara((*sp).mrph_data.offset(j as isize)), k,
                        if OptNEcache != 0 {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            s[1 as libc::c_int as usize].as_mut_ptr() as
                                *const libc::c_char
                        },
                        if OptNEend != 0 {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            s[2 as libc::c_int as usize].as_mut_ptr() as
                                *const libc::c_char
                        },
                        strlen((*(*sp).mrph_data.offset(j as
                                                            isize)).Goi2.as_mut_ptr()).wrapping_div(3
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong),
                        k,
                        if OptNEparent != 0 {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            s[3 as libc::c_int as usize].as_mut_ptr() as
                                *const libc::c_char
                        },
                        if OptNEcase != 0 {
                            s[4 as libc::c_int as usize].as_mut_ptr() as
                                *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        });
                free(id as *mut libc::c_void);
            }
            j += 1
        }
        /* svm_lightでは素性が昇順である必要があるためソートする */
        j = 0 as libc::c_int;
        cp = buf.as_mut_ptr();
        while sscanf(cp, b"%d:1\x00" as *const u8 as *const libc::c_char,
                     &mut *f.as_mut_ptr().offset(j as isize) as
                         *mut libc::c_int) != 0 {
            cp = strstr(cp, b" \x00" as *const u8 as *const libc::c_char);
            if cp.is_null() { break ; }
            cp = cp.offset(1);
            j += 1
        }
        qsort(f.as_mut_ptr() as *mut libc::c_void, j as size_t,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
              Some(intcmp as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
        NE_mgr[i as usize].feature[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        loop  {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 > 0 as libc::c_int) { break ; }
            sprintf(tmp.as_mut_ptr(),
                    b"%d:1 \x00" as *const u8 as *const libc::c_char,
                    f[j as usize]);
            strcat(NE_mgr[i as usize].feature.as_mut_ptr(), tmp.as_mut_ptr());
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn output_feature(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        cp =
            check_feature((*(*sp).mrph_data.offset(i as isize)).f,
                          b"NE\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
        if !cp.is_null() {
            code =
                ne_tagposition_to_code(cp.offset(3 as libc::c_int as isize))
        } else { code = 33 as libc::c_int - 1 as libc::c_int }
        NE_mgr[i as usize].NEresult = code;
        if OptDisplay == 3 as libc::c_int {
            fprintf(stderr,
                    b"%d %s\t%s\n\x00" as *const u8 as *const libc::c_char,
                    code,
                    (*(*sp).mrph_data.offset(i as isize)).Goi2.as_mut_ptr(),
                    NE_mgr[i as usize].feature.as_mut_ptr());
        } else if OptNECRF != 0 {
            /* CRFの学習結果の文字としてのソート順と、数字としてのソート順を一致させるため
		   codeに100を足している */
            fprintf(stderr,
                    b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    NE_mgr[i as usize].feature.as_mut_ptr(),
                    code + 100 as libc::c_int);
        } else {
            j = 0 as libc::c_int;
            while j < 33 as libc::c_int {
                fprintf(stderr,
                        if j == code {
                            b"+1 \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"-1 \x00" as *const u8 as *const libc::c_char
                        });
                j += 1
            }
            fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    NE_mgr[i as usize].feature.as_mut_ptr());
        }
        i += 1
    }
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn apply_model(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // let mut UTF8buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if OptNECRF != 0 {
        clear_crf();
        i = 0 as libc::c_int;
        while i < (*sp).Mrph_num {
            if OptDisplayNE == 3 as libc::c_int {
                fprintf(stderr,
                        b"%d %s\t%s\n\x00" as *const u8 as
                            *const libc::c_char, i,
                        (*(*sp).mrph_data.offset(i as
                                                     isize)).Goi2.as_mut_ptr(),
                        NE_mgr[i as usize].feature.as_mut_ptr());
            }
            crf_add(NE_mgr[i as usize].feature.as_mut_ptr());
            i += 1
        }
        crf_parse();
    }
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        if OptDisplayNE == 3 as libc::c_int {
            fprintf(stderr,
                    b"%d %s\t%s\n\x00" as *const u8 as *const libc::c_char, i,
                    (*(*sp).mrph_data.offset(i as isize)).Goi2.as_mut_ptr(),
                    NE_mgr[i as usize].feature.as_mut_ptr());
        }
        j = 0 as libc::c_int;
        while j < 33 as libc::c_int {
            if NE_mgr[i as usize].notHEAD != 0 &&
                   j != 33 as libc::c_int - 1 as libc::c_int &&
                   (j % 4 as libc::c_int == 0 as libc::c_int ||
                        j % 4 as libc::c_int == 3 as libc::c_int) {
                NE_mgr[i as usize].prob[j as usize] =
                    0 as libc::c_int as libc::c_double
                /* ヒューリスティックルール */
            } else {
                if OptNECRF != 0 {
                    get_crf_prob(i, j,
                                 &mut *(*NE_mgr.as_mut_ptr().offset(i as
                                                                        isize)).prob.as_mut_ptr().offset(j
                                                                                                             as
                                                                                                             isize));
                }
                if OptDisplayNE == 3 as libc::c_int {
                    fprintf(stderr,
                            b"%2d %f\t\x00" as *const u8 as
                                *const libc::c_char, j,
                            NE_mgr[i as usize].prob[j as usize]);
                    if j % 4 as libc::c_int == 3 as libc::c_int &&
                           j != 33 as libc::c_int - 2 as libc::c_int {
                        fprintf(stderr,
                                b"\n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    if j == 33 as libc::c_int - 1 as libc::c_int {
                        fprintf(stderr,
                                b"\n\n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                }
            }
            j += 1
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn constraint(mut pre: libc::c_int,
                                    mut self_0: libc::c_int,
                                    mut last: libc::c_int) -> libc::c_int 
 /*==================================================================*/
 {
    /* 前後に来れるタグの制約に違反すれば1を返す  */
    if pre == 33 as libc::c_int - 1 as libc::c_int { pre += 3 as libc::c_int }
    if self_0 == 33 as libc::c_int - 1 as libc::c_int {
        self_0 += 3 as libc::c_int
    }
    if pre == -(1 as libc::c_int) {
        if self_0 % 4 as libc::c_int == 1 as libc::c_int ||
               self_0 % 4 as libc::c_int == 2 as libc::c_int {
            return 1 as libc::c_int
        }
        return 0 as libc::c_int
    }
    if last != 0 &&
           (self_0 % 4 as libc::c_int == 0 as libc::c_int ||
                self_0 % 4 as libc::c_int == 1 as libc::c_int) {
        return 1 as libc::c_int
    }
    if (pre % 4 as libc::c_int == 0 as libc::c_int ||
            pre % 4 as libc::c_int == 1 as libc::c_int) &&
           self_0 % 4 as libc::c_int != 1 as libc::c_int &&
           self_0 % 4 as libc::c_int != 2 as libc::c_int {
        return 1 as libc::c_int
    }
    if pre % 4 as libc::c_int != 0 as libc::c_int &&
           pre % 4 as libc::c_int != 1 as libc::c_int &&
           (self_0 % 4 as libc::c_int == 1 as libc::c_int ||
                self_0 % 4 as libc::c_int == 2 as libc::c_int) {
        return 1 as libc::c_int
    }
    if (pre % 4 as libc::c_int == 0 as libc::c_int ||
            pre % 4 as libc::c_int == 1 as libc::c_int) &&
           pre / 4 as libc::c_int != self_0 / 4 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn viterbi(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut score: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        j = 0 as libc::c_int;
        while j < 33 as libc::c_int {
            /* 文頭の場合 */
            if i == 0 as libc::c_int {
                if !(constraint(-(1 as libc::c_int), j, 0 as libc::c_int) !=
                         0) {
                    NE_mgr[i as usize].max[j as usize] =
                        NE_mgr[i as usize].prob[j as usize]; /* 文頭 */
                    NE_mgr[i as usize].parent[j as usize] =
                        -(1 as libc::c_int)
                }
            } else {
                /* 文頭、文末以外 */
                NE_mgr[i as usize].max[j as usize] =
                    0 as libc::c_int as libc::c_double;
                let mut current_block_10: u64;
                k = 0 as libc::c_int;
                while k < 33 as libc::c_int {
                    if i == (*sp).Mrph_num - 1 as libc::c_int {
                        if constraint(k, j, 1 as libc::c_int) != 0 {
                            current_block_10 = 7746791466490516765;
                        } else { current_block_10 = 4956146061682418353; }
                    } else if constraint(k, j, 0 as libc::c_int) != 0 {
                        current_block_10 = 7746791466490516765;
                    } else { current_block_10 = 4956146061682418353; }
                    match current_block_10 {
                        4956146061682418353 => {
                            score =
                                NE_mgr[(i - 1 as libc::c_int) as
                                           usize].max[k as usize] *
                                    NE_mgr[i as usize].prob[j as usize];
                            if score > NE_mgr[i as usize].max[j as usize] {
                                /* 同点の場合は無視 */
                                NE_mgr[i as usize].max[j as usize] = score;
                                NE_mgr[i as usize].parent[j as usize] = k
                            }
                        }
                        _ => { }
                    }
                    k += 1
                }
            }
            j += 1
        }
        i += 1
    }
    max = 0 as libc::c_int as libc::c_double;
    j = 0 as libc::c_int;
    while j < 33 as libc::c_int {
        if NE_mgr[((*sp).Mrph_num - 1 as libc::c_int) as
                      usize].max[j as usize] > max {
            max =
                NE_mgr[((*sp).Mrph_num - 1 as libc::c_int) as
                           usize].max[j as usize];
            NE_mgr[((*sp).Mrph_num - 1 as libc::c_int) as usize].NEresult = j
        }
        j += 1
    }
    i = (*sp).Mrph_num - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        NE_mgr[(i - 1 as libc::c_int) as usize].NEresult =
            NE_mgr[i as usize].parent[NE_mgr[i as usize].NEresult as usize];
        i -= 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_ne_feature_mrph(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut cp: [libc::c_char; 128] = [0; 128];
    /* 形態素に付与 */
    i = 0 as libc::c_int; /* OTHERの場合 */
    while i < (*sp).Mrph_num {
        if !(NE_mgr[i as usize].NEresult ==
                 33 as libc::c_int - 1 as libc::c_int) {
            sprintf(cp.as_mut_ptr(),
                    b"NE:%s\x00" as *const u8 as *const libc::c_char,
                    ne_code_to_tagposition(NE_mgr[i as usize].NEresult));
            assign_cfeature(&mut (*(*sp).mrph_data.offset(i as isize)).f,
                            cp.as_mut_ptr(), 0 as libc::c_int);
        }
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn assign_ne_feature_tag(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: [libc::c_char; 2048] = [0; 2048];
    let mut cp_nai: [libc::c_char; 2048] = [0; 2048];
    /* タグに付与 */
    j = 0 as libc::c_int;
    while j < (*sp).Tag_num {
        /* 同一タグの固有表現は一種類まで */
        i = 0 as libc::c_int;
        while i < (*(*sp).tag_data.offset(j as isize)).mrph_num {
            if !check_feature((*(*(*sp).tag_data.offset(j as
                                                            isize)).mrph_ptr.offset(i
                                                                                        as
                                                                                        isize)).f,
                              b"NE\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() {
                break ;
            }
            i += 1
        }
        /* 対象のタグに固有表現が無ければ次のタグへ */
        if !(i == (*(*sp).tag_data.offset(j as isize)).mrph_num) {
            /* ORGANIZATION、PERSONの場合は意味素として与える */
            if strcmp(Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(j as
                                                                          isize)).mrph_ptr.offset(i
                                                                                                      as
                                                                                                      isize)).f)
                                    / 4 as libc::c_int) as usize],
                      b"ORGANIZATION\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                assign_sm((*sp).tag_data.offset(j as isize) as *mut BNST_DATA,
                          b"\xe7\xb5\x84\xe7\xb9\x94\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
            } else if strcmp(Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(j
                                                                                 as
                                                                                 isize)).mrph_ptr.offset(i
                                                                                                             as
                                                                                                             isize)).f)
                                           / 4 as libc::c_int) as usize],
                             b"PERSON\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                assign_sm((*sp).tag_data.offset(j as isize) as *mut BNST_DATA,
                          b"\xe4\xba\xba\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
            }
            sprintf(cp.as_mut_ptr(),
                    b"NE:%s:\x00" as *const u8 as *const libc::c_char,
                    Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(j as
                                                                        isize)).mrph_ptr.offset(i
                                                                                                    as
                                                                                                    isize)).f)
                                  / 4 as libc::c_int) as usize]);
            loop  {
                if get_mrph_ne((*(*(*sp).tag_data.offset(j as
                                                             isize)).mrph_ptr.offset(i
                                                                                         as
                                                                                         isize)).f)
                       == 33 as libc::c_int - 1 as libc::c_int {
                    if OptNElearn != 0 {
                        fprintf(stdout,
                                b"Illegal NE ending %s \"%s %s\"!!\n\x00" as
                                    *const u8 as *const libc::c_char,
                                (*sp).KNPSID,
                                (*(*(*sp).tag_data.offset(j as
                                                              isize)).mrph_ptr.offset(i
                                                                                          as
                                                                                          isize).offset(-(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize))).Goi2.as_mut_ptr(),
                                (*(*(*sp).tag_data.offset(j as
                                                              isize)).mrph_ptr.offset(i
                                                                                          as
                                                                                          isize)).Goi2.as_mut_ptr());
                    } else {
                        fprintf(stderr,
                                b"Illegal NE ending %s \"%s %s\"!!\n\x00" as
                                    *const u8 as *const libc::c_char,
                                (*sp).KNPSID,
                                (*(*(*sp).tag_data.offset(j as
                                                              isize)).mrph_ptr.offset(i
                                                                                          as
                                                                                          isize).offset(-(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize))).Goi2.as_mut_ptr(),
                                (*(*(*sp).tag_data.offset(j as
                                                              isize)).mrph_ptr.offset(i
                                                                                          as
                                                                                          isize)).Goi2.as_mut_ptr());
                    };
                    break ;
                } else {
                    if strlen(cp.as_mut_ptr()).wrapping_add(strlen((*(*(*sp).tag_data.offset(j
                                                                                                 as
                                                                                                 isize)).mrph_ptr.offset(i
                                                                                                                             as
                                                                                                                             isize)).Goi2.as_mut_ptr()))
                           >=
                           (128 as libc::c_int * 16 as libc::c_int) as
                               libc::c_ulong {
                        fprintf(stderr,
                                b";; Too long tag data for %s... .\n\x00" as
                                    *const u8 as *const libc::c_char,
                                cp.as_mut_ptr());
                        exit(1 as libc::c_int);
                    }
                    strcat(cp.as_mut_ptr(),
                           (*(*(*sp).tag_data.offset(j as
                                                         isize)).mrph_ptr.offset(i
                                                                                     as
                                                                                     isize)).Goi2.as_mut_ptr());
                    if get_mrph_ne((*(*(*sp).tag_data.offset(j as
                                                                 isize)).mrph_ptr.offset(i
                                                                                             as
                                                                                             isize)).f)
                           % 4 as libc::c_int == 3 as libc::c_int ||
                           get_mrph_ne((*(*(*sp).tag_data.offset(j as
                                                                     isize)).mrph_ptr.offset(i
                                                                                                 as
                                                                                                 isize)).f)
                               % 4 as libc::c_int == 2 as libc::c_int {
                        assign_cfeature(&mut (*(*sp).tag_data.offset(j as
                                                                         isize)).f,
                                        cp.as_mut_ptr(), 0 as libc::c_int);
                        break ;
                    } else {
                        /* 複数のタグにまたがっている場合は次のタグに進む */
                        i += 1;
                        if i == (*(*sp).tag_data.offset(j as isize)).mrph_num
                           {
                            i = 0 as libc::c_int;
                            sprintf(cp_nai.as_mut_ptr(),
                                    b"NE\xe5\x86\x85:%s\x00" as *const u8 as
                                        *const libc::c_char,
                                    Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(j
                                                                                        as
                                                                                        isize)).mrph_ptr.offset(i
                                                                                                                    as
                                                                                                                    isize)).f)
                                                  / 4 as libc::c_int) as
                                                 usize]);
                            assign_cfeature(&mut (*(*sp).tag_data.offset(j as
                                                                             isize)).f,
                                            cp_nai.as_mut_ptr(),
                                            0 as libc::c_int);
                            j += 1
                        }
                    }
                }
            }
        }
        j += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ne_analysis(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    if OptNECRF != 0 { make_crf_feature(sp); } else { make_svm_feature(sp); }
    if OptNElearn != 0 {
        output_feature(sp);
    } else {
        /* モデルを適用 */
        apply_model(sp);
        /* 文全体で最適化 */
        viterbi(sp);
        /* 結果を付与 */
        assign_ne_feature_mrph(sp);
        /* 人名をひとつのタグにするためのルールを読む */
        assign_general_feature((*sp).mrph_data as *mut libc::c_void,
                               (*sp).Mrph_num, 6 as libc::c_int,
                               0 as libc::c_int, 0 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_ne(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        cp =
            check_feature((*(*sp).mrph_data.offset(i as isize)).f,
                          b"NE:OPTIONAL\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                code = 33 as libc::c_int - 1 as libc::c_int + j;
                j += 1
            }
        } else {
            cp =
                check_feature((*(*sp).mrph_data.offset(i as isize)).f,
                              b"NE\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
            if !cp.is_null() {
                code =
                    ne_tagposition_to_code(cp.offset(3 as libc::c_int as
                                                         isize))
            } else { code = 33 as libc::c_int - 1 as libc::c_int }
        }
        NE_mgr[i as usize].NEresult = code;
        i += 1
    }
    /* 人名をひとつのタグにするためのルールを読む */
    assign_general_feature((*sp).mrph_data as *mut libc::c_void,
                           (*sp).Mrph_num, 6 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn for_ne_analysis(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    /* 構文・格解析結果から、固有表現解析用のfeatureを付与する */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    // let mut l: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cp: [libc::c_char; 128] = [0; 128];
    let mut cpm_ptr: *mut CF_PRED_MGR = 0 as *mut CF_PRED_MGR;
    /* 主辞の情報 */
    j = 0 as libc::c_int;
    while j < (*sp).Bnst_num - 1 as libc::c_int {
        assign_cfeature(
            &mut (*(*(*sp).bnst_data.offset(j as isize)).head_ptr).f,
            b"\xef\xbc\xb4\xe6\x96\x87\xe7\xaf\x80\xe4\xb8\xbb\xe8\xbe\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int
        );
        if strlen((*(*(*sp).bnst_data.offset(j as isize)).head_ptr).Goi.as_mut_ptr()) < (128 as libc::c_int / 3 as libc::c_int) as libc::c_ulong {
            sprintf(
                cp.as_mut_ptr(),
                b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e:%s\x00" as *const u8 as *const libc::c_char,
                (*(*(*sp).bnst_data.offset(j as isize)).head_ptr).Goi.as_mut_ptr()
            );
        } else {
            sprintf(
                cp.as_mut_ptr(),
                b"\xef\xbc\xb4\xe4\xb8\xbb\xe8\xbe\x9e:LONG_WORD\x00" as *const u8 as *const libc::c_char
            );
        };
        i = 1 as libc::c_int;
        while !(*(*(*sp).bnst_data.offset(j as isize)).head_ptr.offset(-(i as isize))).f.is_null() {
            if (*(*(*sp).bnst_data.offset(j as  isize)).head_ptr.offset(-(i as isize))).f.is_null() || !check_feature((*(*(*sp).bnst_data.offset(j as isize)).head_ptr.offset(-(i as isize)).offset(1 as libc::c_int as isize)).f, b"\xe6\x96\x87\xe7\xaf\x80\xe5\xa7\x8b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() {
                break ;
            }
            assign_cfeature(
                &mut (*(*(*sp).bnst_data.offset(j as isize)).head_ptr.offset(-(i as isize))).f,
                cp.as_mut_ptr(),
                0 as libc::c_int
            );
            i += 1
        }
        j += 1
    }
    /* 親の情報 */
    if OptNEparent == 0 {
        /* 文節を前からチェック */
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num - 1 as libc::c_int {
            if strlen(
                (*(*(*sp).bnst_data.offset((*(*sp).bnst_data.offset(j as isize)).dpnd_head as isize)).head_ptr).Goi.as_mut_ptr())
                   < (128 as libc::c_int / 3 as libc::c_int) as libc::c_ulong
               {
                sprintf(cp.as_mut_ptr(),
                        b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e:%s\x00"
                            as *const u8 as *const libc::c_char,
                        (*(*(*sp).bnst_data.offset((*(*sp).bnst_data.offset(j
                                                                                as
                                                                                isize)).dpnd_head
                                                       as
                                                       isize)).head_ptr).Goi.as_mut_ptr());
            } else {
                sprintf(cp.as_mut_ptr(),
                        b"\xef\xbc\xb4\xe4\xbf\x82\xe3\x82\x8a\xe5\x85\x88\xe3\x81\xae\xe4\xb8\xbb\xe8\xbe\x9e:LONG_WORD\x00"
                            as *const u8 as *const libc::c_char);
            };
            assign_cfeature(&mut (*(*(*sp).bnst_data.offset(j as
                                                                isize)).head_ptr).f,
                            cp.as_mut_ptr(), 0 as libc::c_int);
            sprintf(cp.as_mut_ptr(),
                    b"\xef\xbc\xb4%s\x00" as *const u8 as *const libc::c_char,
                    check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                  b"\xe4\xbf\x82\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char));
            assign_cfeature(&mut (*(*(*sp).bnst_data.offset(j as
                                                                isize)).head_ptr).f,
                            cp.as_mut_ptr(), 0 as libc::c_int);
            j += 1
        }
    }
    /* 格フレームの意味情報 */
    if OptNEcase != 0 {
        /* タグを後からチェック */
        j = (*sp).Tag_num - 1 as libc::c_int;
        while j > 0 as libc::c_int {
            cpm_ptr = (*(*sp).tag_data.offset(j as isize)).cpm_ptr;
            if !cpm_ptr.is_null() {
                i = 0 as libc::c_int;
                while i < (*cpm_ptr).cf.element_num {
                    num =
                        (*cpm_ptr).cmm[0 as libc::c_int as
                                           usize].result_lists_d[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].flag[i
                                                                                     as
                                                                                     usize];
                    /* 前方に存在し、主辞の直後に助詞が来る場合のみ */
                    if !((*(*(*cpm_ptr).elem_b_ptr[i as
                                                       usize]).head_ptr.offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).Hinshi
                             != 9 as libc::c_int ||
                             (*(*cpm_ptr).elem_b_ptr[i as usize]).num > j) {
                        /* 組織、人、主体、場所 */
                        k = 0 as libc::c_int;
                        while k < 4 as libc::c_int {
                            if cf_match_element((*(*cpm_ptr).cmm[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].cf_ptr).sm[num
                                                                                           as
                                                                                           usize],
                                                Imi_feature[k as usize],
                                                (0 as libc::c_int == 0) as
                                                    libc::c_int) != 0 {
                                sprintf(cp.as_mut_ptr(),
                                        b"\xe6\x84\x8f\xe5\x91\xb3-%s\x00" as
                                            *const u8 as *const libc::c_char,
                                        Imi_feature[k as usize]);
                                assign_cfeature(&mut (*(**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(i
                                                                                                        as
                                                                                                        isize)).head_ptr).f,
                                                cp.as_mut_ptr(),
                                                0 as libc::c_int);
                            }
                            k += 1
                        }
                        /* 固有表現 */
                        k = 0 as libc::c_int;
                        while k < 9 as libc::c_int - 1 as libc::c_int {
                            if cf_match_element((*(*cpm_ptr).cmm[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].cf_ptr).sm[num
                                                                                           as
                                                                                           usize],
                                                Tag_name[k as usize],
                                                (0 as libc::c_int == 0) as
                                                    libc::c_int) != 0 {
                                sprintf(cp.as_mut_ptr(),
                                        b"\xe6\x84\x8f\xe5\x91\xb3-%s\x00" as
                                            *const u8 as *const libc::c_char,
                                        Tag_name[k as usize]);
                                assign_cfeature(&mut (*(**(*cpm_ptr).elem_b_ptr.as_mut_ptr().offset(i
                                                                                                        as
                                                                                                        isize)).head_ptr).f,
                                                cp.as_mut_ptr(),
                                                0 as libc::c_int);
                            }
                            k += 1
                        }
                    }
                    i += 1
                }
            }
            j -= 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn ne_corefer(mut sp: *mut SENTENCE_DATA,
                                    mut i: libc::c_int,
                                    mut anaphor: *mut libc::c_char,
                                    mut ne: *mut libc::c_char,
                                    mut yomi_flag: libc::c_int)
 -> libc::c_int 
 /*==================================================================*/
 {
    /* 固有表現(ORGANIZATION)と */
    /* 共参照関係にあると判断された固有表現タグの付与されていない表現に */
    /* 固有表現タグを付与する */
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ne_tag: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cp: [libc::c_char; 128] = [0; 128];
    let mut word: [libc::c_char; 128] = [0; 128];
    if strlen(anaphor) == 3 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    ne_tag = 0 as libc::c_int;
    while ne_tag < 9 as libc::c_int {
        /* どのタグであるかを"NE:"に続く4文字で判断する */
        if strncmp(ne.offset(3 as libc::c_int as isize),
                   Tag_name[ne_tag as usize],
                   4 as libc::c_int as libc::c_ulong) == 0 {
            break ;
        }
        ne_tag += 1
    }
    /* ORGANIZATION、PERSONの場合のみ */
    if strcmp(Tag_name[ne_tag as usize],
              b"ORGANIZATION\x00" as *const u8 as *const libc::c_char) != 0 &&
           (strcmp(Tag_name[ne_tag as usize],
                   b"PERSON\x00" as *const u8 as *const libc::c_char) != 0 ||
                yomi_flag == 0) {
        return 0 as libc::c_int
    } /* 接尾辞を含むものには未対応 */
    end =
        (*(*sp).tag_data.offset(i as
                                    isize)).head_ptr.wrapping_offset_from((*sp).mrph_data)
            as libc::c_long as libc::c_int;
    if !check_feature((*(*(*sp).tag_data.offset(i as isize)).head_ptr).f,
                      b"\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char).is_null()
       {
        end -= 1
    }
    start = end;
    while start >= 0 as libc::c_int {
        word[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        k = start;
        while k <= end {
            strcat(word.as_mut_ptr(),
                   (*(*sp).mrph_data.offset(k as isize)).Goi2.as_mut_ptr());
            k += 1
            /* 先行詞候補 */
        }
        if strcmp(word.as_mut_ptr(), anaphor) == 0 { break ; }
        start -= 1
    }
    if strcmp(word.as_mut_ptr(), anaphor) != 0 { return 0 as libc::c_int }
    /* 形態素に付与、NEresultに記録 */
    j = start;
    if j == end {
        sprintf(cp.as_mut_ptr(),
                b"NE:%s:single\x00" as *const u8 as *const libc::c_char,
                Tag_name[ne_tag as usize]);
        assign_cfeature(&mut (*(*sp).mrph_data.offset(j as isize)).f,
                        cp.as_mut_ptr(), 0 as libc::c_int);
        NE_mgr[j as usize].NEresult =
            ne_tag * 4 as libc::c_int + 3 as libc::c_int
        /* single */
    } else {
        j = start;
        while j <= end {
            if j == start {
                sprintf(cp.as_mut_ptr(),
                        b"NE:%s:head\x00" as *const u8 as *const libc::c_char,
                        Tag_name[ne_tag as usize]);
                assign_cfeature(&mut (*(*sp).mrph_data.offset(j as isize)).f,
                                cp.as_mut_ptr(), 0 as libc::c_int);
                NE_mgr[j as usize].NEresult = ne_tag * 4 as libc::c_int
                /* head */
            } else if j == end {
                sprintf(cp.as_mut_ptr(),
                        b"NE:%s:tail\x00" as *const u8 as *const libc::c_char,
                        Tag_name[ne_tag as usize]);
                assign_cfeature(&mut (*(*sp).mrph_data.offset(j as isize)).f,
                                cp.as_mut_ptr(), 0 as libc::c_int);
                NE_mgr[j as usize].NEresult =
                    ne_tag * 4 as libc::c_int + 2 as libc::c_int
                /* tail */
            } else {
                sprintf(cp.as_mut_ptr(),
                        b"NE:%s:middle\x00" as *const u8 as
                            *const libc::c_char, Tag_name[ne_tag as usize]);
                assign_cfeature(&mut (*(*sp).mrph_data.offset(j as isize)).f,
                                cp.as_mut_ptr(), 0 as libc::c_int);
                NE_mgr[j as usize].NEresult =
                    ne_tag * 4 as libc::c_int + 1 as libc::c_int
                /* middle */
            }
            j += 1
        }
    }
    /* ORGANIZATION、PERSONの場合は意味素として与える */
    if strcmp(Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(i as
                                                                  isize)).head_ptr).f)
                            / 4 as libc::c_int) as usize],
              b"ORGANIZATION\x00" as *const u8 as *const libc::c_char) == 0 {
        assign_sm((*sp).tag_data.offset(i as isize) as *mut BNST_DATA,
                  b"\xe7\xb5\x84\xe7\xb9\x94\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char);
    } else if strcmp(Tag_name[(get_mrph_ne((*(*(*sp).tag_data.offset(i as
                                                                         isize)).head_ptr).f)
                                   / 4 as libc::c_int) as usize],
                     b"PERSON\x00" as *const u8 as *const libc::c_char) == 0 {
        assign_sm((*sp).tag_data.offset(i as isize) as *mut BNST_DATA,
                  b"\xe4\xba\xba\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    }
    /* タグに付与 */
    sprintf(cp.as_mut_ptr(),
            b"NE:%s:%s\x00" as *const u8 as *const libc::c_char,
            Tag_name[ne_tag as usize], anaphor);
    assign_cfeature(&mut (*(*sp).tag_data.offset(i as isize)).f,
                    cp.as_mut_ptr(), 0 as libc::c_int);
    sprintf(cp.as_mut_ptr(),
            b"NE\xe5\x86\x85:%s\x00" as *const u8 as *const libc::c_char,
            Tag_name[ne_tag as usize]);
    k = 0 as libc::c_int;
    while (start as libc::c_long) <
              (*(*sp).tag_data.offset((i - k) as
                                          isize)).mrph_ptr.wrapping_offset_from((*sp).mrph_data)
                  as libc::c_long {
        k += 1;
        assign_cfeature(&mut (*(*sp).tag_data.offset((i - k) as isize)).f,
                        cp.as_mut_ptr(), 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn make_ne_cache(mut sp: *mut SENTENCE_DATA) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    // let mut ne_tag: libc::c_int = 0;
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 各形態素の情報を記憶 */
    i = 0 as libc::c_int;
    while i < (*sp).Mrph_num {
        register_ne_cache((*(*sp).mrph_data.offset(i as
                                                       isize)).Goi2.as_mut_ptr(),
                          NE_mgr[i as usize].NEresult);
        i += 1
    };
}
/*====================================================================
                               END
====================================================================*/
