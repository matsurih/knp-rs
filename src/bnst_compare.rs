#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{BNST_DATA, lib_bgh, match_matrix, tools, types};
use crate::ctools::{check_feature, Language, stderr};
use crate::lib_bgh::bgh_code_match;
use crate::lib_sm::ntt_code_match;
use crate::similarity::similarity_chinese;
use crate::thesaurus::{calc_distsim_from_bnst, general_code_match};
use crate::tools::{OptParaFix, ParaThesaurus};


#[no_mangle]
pub unsafe extern "C" fn str_part_cmp(mut c1: *mut libc::c_char, mut c2: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut pre: libc::c_int = 0;
    let mut post: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    len1 = strlen(c1) as libc::c_int;
    len2 = strlen(c2) as libc::c_int;
    len = if len1 < len2 { len1 } else { len2 };
    pre = 0 as libc::c_int;
    while len > pre && *c1.offset(pre as isize) as libc::c_int == *c2.offset(pre as isize) as libc::c_int {
        pre += 1
    }
    post = 0 as libc::c_int;
    while len > post && *c1.offset(len1 as isize).offset(-(post as isize)).offset(-(1 as libc::c_int as isize)) as libc::c_int == *c2.offset(len2 as isize).offset(-(post as isize)).offset(-(1 as libc::c_int as isize)) as libc::c_int {
        post += 1
    }
    match_0 = if pre > post { pre } else { post };
    match_0 -= match_0 % 3 as libc::c_int;
    match_0 = 2 as libc::c_int * match_0 / 3 as libc::c_int;
    return match_0;
}
#[no_mangle]
pub unsafe extern "C" fn check_fuzoku(mut ptr: *mut types::BNST_DATA, mut Hinshi: libc::c_int, mut Bunrui: libc::c_int, mut cp: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ptr.is_null() { return 0 as libc::c_int }
    i = (*ptr).mrph_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                          b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (Hinshi == 0 as libc::c_int || Hinshi == (*(*ptr).mrph_ptr.offset(i as isize)).Hinshi) && (Bunrui == 0 as libc::c_int || Bunrui == (*(*ptr).mrph_ptr.offset(i as isize)).Bunrui) && (cp.is_null() || strcmp((*(*ptr).mrph_ptr.offset(i as isize)).Goi.as_mut_ptr(), cp) == 0) {
                return 1 as libc::c_int
            }
        } else {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_fuzoku_substr(mut ptr: *mut types::BNST_DATA, mut Hinshi: libc::c_int, mut Bunrui: libc::c_int, mut cp: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ptr.is_null() { return 0 as libc::c_int }
    i = (*ptr).mrph_num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !check_feature((*(*ptr).mrph_ptr.offset(i as isize)).f,
                          b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as *const u8 as
                              *const libc::c_char as
                              *mut libc::c_char).is_null() {
            if (Hinshi == 0 as libc::c_int || Hinshi == (*(*ptr).mrph_ptr.offset(i as isize)).Hinshi) && (Bunrui == 0 as libc::c_int || Bunrui == (*(*ptr).mrph_ptr.offset(i as isize)).Bunrui) && (cp.is_null() || !strstr((*(*ptr).mrph_ptr.offset(i as isize)).Goi.as_mut_ptr(), cp).is_null()) {
                return 1 as libc::c_int
            }
        } else {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_bnst_substr(mut ptr: *mut types::BNST_DATA, mut Hinshi: libc::c_int, mut Bunrui: libc::c_int, mut cp: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ptr.is_null() { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < (*ptr).mrph_num {
        if (Hinshi == 0 as libc::c_int || Hinshi == (*(*ptr).mrph_ptr.offset(i as isize)).Hinshi) && (Bunrui == 0 as libc::c_int || Bunrui == (*(*ptr).mrph_ptr.offset(i as isize)).Bunrui) && (cp.is_null() || !strstr((*(*ptr).mrph_ptr.offset(i as isize)).Goi.as_mut_ptr(), cp).is_null()) {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jiritu_fuzoku_check(mut ptr1: *mut BNST_DATA, mut ptr2: *mut BNST_DATA, mut cp: *mut libc::c_char) -> libc::c_int {
    return if strcmp((*(*ptr1).head_ptr).Goi.as_mut_ptr(), cp) == 0 && check_fuzoku(ptr2, 0 as libc::c_int, 0 as libc::c_int, cp) != 0 || strcmp((*(*ptr2).head_ptr).Goi.as_mut_ptr(), cp) == 0 && check_fuzoku(ptr1, 0 as libc::c_int, 0 as libc::c_int, cp) != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgh_match(mut ptr1: *mut BNST_DATA, mut ptr2: *mut BNST_DATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut point: libc::c_int = 0;
    let mut max_point: libc::c_int = 0 as libc::c_int;
    if *(*ptr1).BGH_code.as_mut_ptr() == 0 ||
           *(*ptr2).BGH_code.as_mut_ptr() == 0 {
        return -(1 as libc::c_int)
    }
    i = 0 as libc::c_int;
    while (*ptr1).BGH_code[i as usize] != 0 {
        j = 0 as libc::c_int;
        while (*ptr2).BGH_code[j as usize] != 0 {
            point = bgh_code_match(
                (*ptr1).BGH_code.as_mut_ptr().offset(i as isize),
                (*ptr2).BGH_code.as_mut_ptr().offset(j as isize)
            );
            if max_point < point {
                max_point = point
            }
            j += 11 as libc::c_int
        }
        i += 11 as libc::c_int
    }
    return if (max_point - 2 as libc::c_int) < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (max_point) - 2 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn sm_match(mut ptr1: *mut BNST_DATA, mut ptr2: *mut BNST_DATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut code_size: libc::c_int = 0;
    let mut point: libc::c_float = 0.;
    let mut max_point: libc::c_float = 0 as libc::c_int as libc::c_float;
    if *(*ptr1).SM_code.as_mut_ptr() == 0 || *(*ptr2).SM_code.as_mut_ptr() == 0 {
        return -(1 as libc::c_int)
    }
    code_size = (*THESAURUS.as_mut_ptr().offset(ParaThesaurus as isize)).code_size;
    i = 0 as libc::c_int;
    while (*ptr1).SM_code[i as usize] != 0 {
        j = 0 as libc::c_int;
        while (*ptr2).SM_code[j as usize] != 0 {
            if ParaThesaurus == 2 as libc::c_int {
                point = ntt_code_match(
                    (*ptr1).SM_code.as_mut_ptr().offset(i as isize),
                    (*ptr2).SM_code.as_mut_ptr().offset(j as isize),
                    2 as libc::c_int
                )
            } else {
                point = general_code_match(
                    &mut *THESAURUS.as_mut_ptr().offset(ParaThesaurus as isize),
                    (*ptr1).SM_code.as_mut_ptr().offset(i as isize),
                    (*ptr2).SM_code.as_mut_ptr().offset(j as isize)
                )
            }
            if max_point < point {
                max_point = point
            }
            j += code_size
        }
        i += code_size
    }
    max_point = ((max_point as libc::c_double - 0.4f64) * (11 as libc::c_int - 2 as libc::c_int) as libc::c_double / (11 as libc::c_int - 4 as libc::c_int) as libc::c_double * 11 as libc::c_int as libc::c_double) as libc::c_float;
    return if max_point < 0 as libc::c_int as libc::c_float {
        0 as libc::c_int
    } else {
        max_point as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn subordinate_level_comp(mut ptr1: *mut types::BNST_DATA, mut ptr2: *mut types::BNST_DATA) -> libc::c_int {
    let mut level1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level2: *mut libc::c_char = 0 as *mut libc::c_char;
    level1 = check_feature(
        (*ptr1).f,
        b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    level2 = check_feature(
        (*ptr2).f,
        b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    return if level1.is_null() {
        (0 as libc::c_int == 0) as libc::c_int
    } else if level2.is_null() {
        0 as libc::c_int
    } else if levelcmp(
        level1.offset(strlen(b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00" as *const u8 as *const libc::c_char) as isize),
        level2.offset(strlen(b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00" as *const u8 as *const libc::c_char) as isize)
    ) <= 0 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn subordinate_level_check(mut cp: *mut libc::c_char, mut f: *mut types::FEATURE) -> libc::c_int {
    let mut level1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level2: *mut libc::c_char = 0 as *mut libc::c_char;
    level1 = cp;
    level2 = check_feature(
        f,
        b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    return if level1.is_null() {
        (0 as libc::c_int == 0) as libc::c_int
    } else if level2.is_null() {
        0 as libc::c_int
    } else if levelcmp(
        level1,
        level2.offset(strlen(b"\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00" as *const u8 as *const libc::c_char) as isize)) <= 0 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn levelcmp(mut cp1: *mut libc::c_char, mut cp2: *mut libc::c_char) -> libc::c_int {
    let mut level1: libc::c_int = 0;
    let mut level2: libc::c_int = 0;
    if strcmp(cp1, b"A-\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 1 as libc::c_int
    } else if strcmp(cp1, b"A\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 2 as libc::c_int
    } else if strcmp(cp1, b"B-\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 3 as libc::c_int
    } else if strcmp(cp1, b"B\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 4 as libc::c_int
    } else if strcmp(cp1, b"B+\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 5 as libc::c_int
    } else if strcmp(cp1, b"C\x00" as *const u8 as *const libc::c_char) == 0 {
        level1 = 6 as libc::c_int
    } else {
        fprintf(stderr, b"Invalid level (%s)\n\x00" as *const u8 as *const libc::c_char, cp1);
    }
    if strcmp(cp2, b"A-\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 1 as libc::c_int
    } else if strcmp(cp2, b"A\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 2 as libc::c_int
    } else if strcmp(cp2, b"B-\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 3 as libc::c_int
    } else if strcmp(cp2, b"B\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 4 as libc::c_int
    } else if strcmp(cp2, b"B+\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 5 as libc::c_int
    } else if strcmp(cp2, b"C\x00" as *const u8 as *const libc::c_char) == 0 {
        level2 = 6 as libc::c_int
    } else {
        fprintf(stderr, b"Invalid level (%s)\n\x00" as *const u8 as *const libc::c_char, cp2);
    }
    return level1 - level2;
}
#[no_mangle]
pub unsafe extern "C" fn calc_match(mut sp: *mut types::SENTENCE_DATA, mut pre: libc::c_int, mut pos: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut part_mt_point: libc::c_int = 0;
    let mut mt_point: libc::c_int = 0;
    let mut point: libc::c_int = 0 as libc::c_int;
    let mut flag1: libc::c_int = 0;
    let mut flag2: libc::c_int = 0;
    let mut content_word_match: libc::c_int = 0;
    let mut counter1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut counter2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str1: [libc::c_char; 4] = [0; 4];
    let mut str2: [libc::c_char; 4] = [0; 4];
    let mut str1_bk: [libc::c_char; 128] = [0; 128];
    let mut str2_bk: [libc::c_char; 128] = [0; 128];
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr1: *mut types::BNST_DATA = 0 as *mut types::BNST_DATA;
    let mut ptr2: *mut types::BNST_DATA = 0 as *mut types::BNST_DATA;
    let mut similarity: libc::c_float = 0.;
    ptr1 = &mut *(*sp).bnst_data.offset(pre as isize) as *mut types::BNST_DATA;
    ptr2 = &mut *(*sp).bnst_data.offset(pos as isize) as *mut types::BNST_DATA;
    if Language != 2 as libc::c_int {
        cp1 =
            check_feature((*ptr1).f,
                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        if !cp1.is_null() &&
               {
                   cp2 =
                       check_feature((*ptr2).f,
                                     b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char);
                   !cp2.is_null()
               } &&
               (strcmp(cp1,
                       b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                           *const u8 as *const libc::c_char) == 0 ||
                    strcmp(cp2,
                           b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                               *const u8 as *const libc::c_char) != 0 ||
                    check_feature((*ptr2).f,
                                  b"\xe5\xbd\xa2\xe5\x89\xaf\xe5\x90\x8d\xe8\xa9\x9e\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null()) ||
               !check_feature((*ptr1).f,
                              b"\xe5\x90\x8d\xe8\xa9\x9e\xe7\x9a\x84\xe5\xbd\xa2\xe5\xae\xb9\xe8\xa9\x9e\xe8\xaa\x9e\xe5\xb9\xb9\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\xbd\xa2\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
               !check_feature((*ptr1).f,
                              b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
               !check_feature((*ptr1).f,
                              b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe6\x95\xb0\xe9\x87\x8f\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
               !check_feature((*ptr1).f,
                              b"\xe4\xb8\xa6\xe3\x82\xad:\xe5\x90\x8d\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr1).f,
                                  b"\xe9\xa1\x9e\xe4\xbc\xbc\xe8\xa8\x88\xe7\xae\x97:\xe7\x9a\x84\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe9\xa1\x9e\xe4\xbc\xbc\xe8\xa8\x88\xe7\xae\x97:\xe7\x9a\x84\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
            if !check_feature((*ptr1).f,
                              b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*ptr1).f,
                                 b"\xe4\xb8\xa6\xe3\x82\xad:\xef\xbc\x9f\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                   check_feature((*ptr2).f,
                                 b"\xe7\x94\xa8\xe8\xa8\x80:\xe5\x88\xa4\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() {
                return 0 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe3\x81\x9f\xe3\x82\x81-\xe3\x81\x9b\xe3\x81\x84\x00"
                                  as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*ptr2).f,
                                 b"\xe3\x81\x9f\xe3\x82\x81-\xe3\x81\x9b\xe3\x81\x84\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() ||
                   check_feature((*ptr1).f,
                                 b"\xe3\x81\x9f\xe3\x82\x81-\xe3\x81\x9b\xe3\x81\x84\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() &&
                       !check_feature((*ptr2).f,
                                      b"\xe3\x81\x9f\xe3\x82\x81-\xe3\x81\x9b\xe3\x81\x84\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                return 0 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   check_feature((*ptr2).f,
                                 b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() ||
                   check_feature((*ptr1).f,
                                 b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char).is_null() &&
                       !check_feature((*ptr2).f,
                                      b"\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                return 0 as libc::c_int
            }
            point += 2 as libc::c_int;
            if !check_feature((*ptr1).f,
                              b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                if !check_feature((*ptr1).f,
                                  b"\xe4\xba\xba\xe5\x90\x8d\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    flag1 = 0 as libc::c_int
                } else if !check_feature((*ptr1).f,
                                         b"\xe5\x9c\xb0\xe5\x90\x8d\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                    flag1 = 1 as libc::c_int
                } else if !check_feature((*ptr1).f,
                                         b"\xe7\xb5\x84\xe7\xb9\x94\xe5\x90\x8d\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() {
                    flag1 = 2 as libc::c_int
                } else if !check_feature((*ptr1).f,
                                         b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                    flag1 = 3 as libc::c_int
                } else { flag1 = 5 as libc::c_int }
                if !check_feature((*ptr2).f,
                                  b"\xe4\xba\xba\xe5\x90\x8d\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                    flag2 = 0 as libc::c_int
                } else if !check_feature((*ptr2).f,
                                         b"\xe5\x9c\xb0\xe5\x90\x8d\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                    flag2 = 1 as libc::c_int
                } else if !check_feature((*ptr2).f,
                                         b"\xe7\xb5\x84\xe7\xb9\x94\xe5\x90\x8d\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char).is_null() {
                    flag2 = 2 as libc::c_int
                } else if !check_feature((*ptr2).f,
                                         b"\xe6\x95\xb0\xe9\x87\x8f\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char).is_null() {
                    flag2 = 3 as libc::c_int
                } else { flag2 = 5 as libc::c_int }
                if flag1 == 0 as libc::c_int && flag2 == 0 as libc::c_int {
                    point += 5 as libc::c_int;
                    content_word_match = 0 as libc::c_int
                } else if flag1 == 1 as libc::c_int &&
                              flag2 == 1 as libc::c_int {
                    point += 5 as libc::c_int;
                    content_word_match = 0 as libc::c_int
                } else if flag1 == 2 as libc::c_int &&
                              flag2 == 2 as libc::c_int {
                    point += 5 as libc::c_int;
                    content_word_match = 0 as libc::c_int
                } else if (flag1 == 0 as libc::c_int ||
                               flag1 == 1 as libc::c_int ||
                               flag1 == 2 as libc::c_int) &&
                              (flag2 == 0 as libc::c_int ||
                                   flag2 == 1 as libc::c_int ||
                                   flag2 == 2 as libc::c_int) {
                    point += 2 as libc::c_int;
                    content_word_match = 0 as libc::c_int
                } else if flag1 == 3 as libc::c_int &&
                              flag2 == 3 as libc::c_int {
                    point += 2 as libc::c_int;
                    counter1 =
                        check_feature((*ptr1).f,
                                      b"\xe3\x82\xab\xe3\x82\xa6\xe3\x83\xb3\xe3\x82\xbf\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char);
                    counter2 =
                        check_feature((*ptr2).f,
                                      b"\xe3\x82\xab\xe3\x82\xa6\xe3\x83\xb3\xe3\x82\xbf\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char);
                    if counter1.is_null() && counter2.is_null() ||
                           counter1.is_null() ||
                           !counter2.is_null() &&
                               strcmp(counter1, counter2) == 0 {
                        point += 5 as libc::c_int
                    }
                    content_word_match = 0 as libc::c_int
                } else if flag1 == 5 as libc::c_int &&
                              flag2 == 5 as libc::c_int {
                    content_word_match = 1 as libc::c_int
                } else { content_word_match = 0 as libc::c_int }
            } else { content_word_match = 1 as libc::c_int }
            if content_word_match == 1 as libc::c_int {
                if strcmp((*(*ptr1).head_ptr).Goi.as_mut_ptr(),
                          (*(*ptr2).head_ptr).Goi.as_mut_ptr()) == 0 {
                    point += 10 as libc::c_int
                } else {
                    if ParaThesaurus == -(1 as libc::c_int) {
                        mt_point = -(1 as libc::c_int)
                    } else if ParaThesaurus == 64 as libc::c_int {
                        mt_point =
                            calc_distsim_from_bnst(ptr1, ptr2) *
                                2 as libc::c_int
                    } else if ParaThesaurus == 1 as libc::c_int {
                        mt_point = bgh_match(ptr1, ptr2) * 2 as libc::c_int
                    } else {
                        mt_point = sm_match(ptr1, ptr2) * 2 as libc::c_int
                    }
                    if !check_feature((*ptr1).f,
                                      b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char).is_null() &&
                           !check_feature((*ptr2).f,
                                          b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                       {
                        if strcmp((*ptr1).Jiritu_Go.as_mut_ptr(),
                                  b"\xe3\x81\x99\xe3\x82\x8b\x00" as *const u8
                                      as *const libc::c_char) == 0 ||
                               strcmp((*ptr2).Jiritu_Go.as_mut_ptr(),
                                      b"\xe3\x81\x99\xe3\x82\x8b\x00" as
                                          *const u8 as *const libc::c_char) ==
                                   0 {
                            mt_point =
                                if mt_point < 2 as libc::c_int {
                                    mt_point
                                } else { 2 as libc::c_int }
                        }
                        if !check_feature((*ptr1).f,
                                          b"\xe6\x95\xac\xe8\xaa\x9e\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                               &&
                               check_feature((*ptr2).f,
                                             b"\xe6\x95\xac\xe8\xaa\x9e\x00"
                                                 as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char).is_null()
                           {
                            mt_point =
                                if mt_point < 2 as libc::c_int {
                                    mt_point
                                } else { 2 as libc::c_int }
                        }
                    }
                    part_mt_point = 0 as libc::c_int;
                    if mt_point < 0 as libc::c_int {
                        mt_point = 0 as libc::c_int;
                        if !check_feature((*ptr1).f,
                                          b"\xe4\xbd\x93\xe8\xa8\x80\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char).is_null()
                               &&
                               !check_feature((*ptr2).f,
                                              b"\xe4\xbd\x93\xe8\xa8\x80\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char).is_null()
                           {
                            part_mt_point =
                                str_part_cmp((*(*ptr1).head_ptr).Goi.as_mut_ptr(),
                                             (*(*ptr2).head_ptr).Goi.as_mut_ptr())
                        }
                    }
                    point +=
                        if part_mt_point + mt_point < 10 as libc::c_int {
                            (part_mt_point) + mt_point
                        } else { 10 as libc::c_int }
                }
            }
            i = (*ptr1).mrph_num - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if !(!check_feature((*(*ptr1).mrph_ptr.offset(i as isize)).f,
                                    b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char).is_null() &&
                         (*ptr1).mrph_ptr.offset(i as isize) >
                             (*ptr1).head_ptr) {
                    break ;
                }
                if !(strcmp(Class[(*(*ptr1).mrph_ptr.offset(i as
                                                                isize)).Hinshi
                                      as usize][0 as libc::c_int as usize].id
                                as *const libc::c_char,
                            b"\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00" as
                                *const u8 as *const libc::c_char) == 0) {
                    j = (*ptr2).mrph_num - 1 as libc::c_int;
                    while j >= 0 as libc::c_int {
                        if !(!check_feature((*(*ptr2).mrph_ptr.offset(j as
                                                                          isize)).f,
                                            b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char).is_null()
                                 &&
                                 (*ptr2).mrph_ptr.offset(j as isize) >
                                     (*ptr2).head_ptr) {
                            break ;
                        }
                        if !(strcmp(Class[(*(*ptr2).mrph_ptr.offset(j as
                                                                        isize)).Hinshi
                                              as
                                              usize][0 as libc::c_int as
                                                         usize].id as
                                        *const libc::c_char,
                                    b"\xe6\x8e\xa5\xe5\xb0\xbe\xe8\xbe\x9e\x00"
                                        as *const u8 as *const libc::c_char)
                                 == 0) {
                            if strcmp((*(*ptr1).mrph_ptr.offset(i as
                                                                    isize)).Goi.as_mut_ptr(),
                                      (*(*ptr2).mrph_ptr.offset(j as
                                                                    isize)).Goi.as_mut_ptr())
                                   == 0 {
                                point += 2 as libc::c_int
                            }
                        }
                        j -= 1
                    }
                }
                i -= 1
            }
            if !check_feature((*ptr1).f,
                              b"\xe3\x80\x9c\xe3\x82\x8c\xe3\x82\x8b\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe3\x80\x9c\xe3\x82\x89\xe3\x82\x8c\xe3\x82\x8b\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                   !check_feature((*ptr1).f,
                                  b"\xe3\x80\x9c\xe3\x82\x89\xe3\x82\x8c\xe3\x82\x8b\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                       !check_feature((*ptr2).f,
                                      b"\xe3\x80\x9c\xe3\x82\x8c\xe3\x82\x8b\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                point += 2 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe3\x80\x9c\xe3\x81\x9b\xe3\x82\x8b\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe3\x80\x9c\xe3\x81\x95\xe3\x81\x9b\xe3\x82\x8b\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                   !check_feature((*ptr1).f,
                                  b"\xe3\x80\x9c\xe3\x81\x95\xe3\x81\x9b\xe3\x82\x8b\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                       !check_feature((*ptr2).f,
                                      b"\xe3\x80\x9c\xe3\x81\x9b\xe3\x82\x8b\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                point += 2 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe3\x80\x9c\xe3\x81\xaa\xe3\x81\x84\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe3\x80\x9c\xe3\x81\xac\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
                   !check_feature((*ptr1).f,
                                  b"\xe3\x80\x9c\xe3\x81\xac\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() &&
                       !check_feature((*ptr2).f,
                                      b"\xe3\x80\x9c\xe3\x81\xaa\xe3\x81\x84\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                point += 2 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe3\x82\xbf\xe3\x83\xaa\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe3\x82\xbf\xe3\x83\xaa\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                point += 2 as libc::c_int
            }
            if !check_feature((*ptr1).f,
                              b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8 as
                                  *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"\xe6\x8f\x90\xe9\xa1\x8c\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char).is_null() {
                point += 3 as libc::c_int
            }
            if jiritu_fuzoku_check(ptr1, ptr2,
                                   b"\xe3\x81\x99\xe3\x82\x8b\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                point += 1 as libc::c_int
            }
            if jiritu_fuzoku_check(ptr1, ptr2,
                                   b"\xe3\x81\xa7\xe3\x81\x8d\xe3\x82\x8b\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char) != 0 ||
                   jiritu_fuzoku_check(ptr1, ptr2,
                                       b"\xe5\x87\xba\xe6\x9d\xa5\xe3\x82\x8b\x00"
                                           as *const u8 as *const libc::c_char
                                           as *mut libc::c_char) != 0 {
                point += 3 as libc::c_int
            }
        }
    } else {
        i = (*ptr1).num + 1 as libc::c_int;
        while i < (*ptr2).num {
            if !check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                              b"PU\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   strcmp((*(*(*sp).bnst_data.offset(i as
                                                         isize)).head_ptr).Goi.as_mut_ptr(),
                          b"\xe3\x80\x81\x00" as *const u8 as
                              *const libc::c_char) != 0 &&
                   strcmp((*(*(*sp).bnst_data.offset(i as
                                                         isize)).head_ptr).Goi.as_mut_ptr(),
                          b"\xef\xbc\x9b\x00" as *const u8 as
                              *const libc::c_char) != 0 {
                point = 0 as libc::c_int;
                return point
            }
            i += 1
        }
        if !check_feature((*ptr1).f,
                          b"NN\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
               !check_feature((*ptr2).f,
                              b"NN\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
               !check_feature((*ptr1).f,
                              b"NR\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"NR\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() {
            i = 3 as libc::c_int;
            while i as libc::c_ulong <=
                      (if strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr()) <
                              strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr()) {
                           strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr())
                       } else {
                           strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr())
                       }) {
                strcpy(str1.as_mut_ptr(),
                       b"   \x00" as *const u8 as *const libc::c_char);
                strcpy(str2.as_mut_ptr(),
                       b"   \x00" as *const u8 as *const libc::c_char);
                strncpy(str1.as_mut_ptr(),
                        (*(*ptr1).head_ptr).Goi.as_mut_ptr().offset(strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr()).wrapping_sub(i
                                                                                                                                  as
                                                                                                                                  libc::c_ulong)
                                                                        as
                                                                        isize),
                        3 as libc::c_int as libc::c_ulong);
                strncpy(str2.as_mut_ptr(),
                        (*(*ptr2).head_ptr).Goi.as_mut_ptr().offset(strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr()).wrapping_sub(i
                                                                                                                                  as
                                                                                                                                  libc::c_ulong)
                                                                        as
                                                                        isize),
                        3 as libc::c_int as libc::c_ulong);
                if strcmp(str1.as_mut_ptr(), str2.as_mut_ptr()) !=
                       0 as libc::c_int {
                    break ;
                }
                i += 3 as libc::c_int
            }
            if i > 3 as libc::c_int &&
                   (i as libc::c_ulong) <
                       strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr()) &&
                   (i as libc::c_ulong) <
                       strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr()) {
                point += 5 as libc::c_int
            }
        }
        if !check_feature((*ptr1).f,
                          b"CD\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char).is_null() &&
               !check_feature((*ptr2).f,
                              b"CD\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() ||
               !check_feature((*ptr1).f,
                              b"OD\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char).is_null() &&
                   !check_feature((*ptr2).f,
                                  b"OD\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() ||
               (!check_feature((*ptr1).f,
                               b"NT\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char).is_null() ||
                    !check_feature((*ptr1).f,
                                   b"NT-SHORT\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char).is_null()) &&
                   !check_feature((*ptr2).f,
                                  b"NT\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char).is_null() {
            strcpy(str1_bk.as_mut_ptr(),
                   b"\x00" as *const u8 as *const libc::c_char);
            strcpy(str2_bk.as_mut_ptr(),
                   b"\x00" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) < strlen((*(*ptr1).head_ptr).Goi.as_mut_ptr()).wrapping_sub(3 as libc::c_int as libc::c_ulong) {
                strcpy(str1.as_mut_ptr(),
                       b"   \x00" as *const u8 as *const libc::c_char);
                strncpy(str1.as_mut_ptr(),
                        (*(*ptr1).head_ptr).Goi.as_mut_ptr().offset(i as
                                                                        isize),
                        3 as libc::c_int as libc::c_ulong);
                if is_figure(str1.as_mut_ptr()) == 0 {
                    strcat(str1_bk.as_mut_ptr(), str1.as_mut_ptr());
                }
                i += 3 as libc::c_int
            }
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) < strlen((*(*ptr2).head_ptr).Goi.as_mut_ptr()).wrapping_sub(3 as libc::c_int as ibc::c_ulong) {
                strcpy(str2.as_mut_ptr(),
                       b"   \x00" as *const u8 as *const libc::c_char);
                strncpy(str2.as_mut_ptr(),
                        (*(*ptr2).head_ptr).Goi.as_mut_ptr().offset(i as
                                                                        isize),
                        3 as libc::c_int as libc::c_ulong);
                if is_figure(str2.as_mut_ptr()) == 0 {
                    strcat(str2_bk.as_mut_ptr(), str2.as_mut_ptr());
                }
                i += 3 as libc::c_int
            }
            if strcmp(str1_bk.as_mut_ptr(), str2_bk.as_mut_ptr()) == 0 {
                point += 5 as libc::c_int
            }
        }
        if strcmp((*(*ptr1).head_ptr).Goi.as_mut_ptr(),
                  (*(*ptr2).head_ptr).Goi.as_mut_ptr()) == 0 {
            point += 5 as libc::c_int
        } else {
            if check_feature((*ptr1).f,
                             b"PU\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() &&
                   check_feature((*ptr2).f,
                                 b"PU\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char).is_null() {
                similarity =
                    similarity_chinese((*(*ptr1).head_ptr).Goi.as_mut_ptr(),
                                       (*(*ptr2).head_ptr).Goi.as_mut_ptr())
            }
            point =
                (10 as libc::c_int as libc::c_float * similarity) as
                    libc::c_int
        }
        if strcmp((*(*ptr1).head_ptr).Pos.as_mut_ptr(),
                  (*(*ptr2).head_ptr).Pos.as_mut_ptr()) == 0 {
            point += 2 as libc::c_int
        }
        if point > 12 as libc::c_int { point = 12 as libc::c_int }
    }
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn calc_match_matrix(mut sp: *mut types::SENTENCE_DATA) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut calc_flag: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        calc_flag = 1 as libc::c_int;
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            if calc_flag != 0 {
                (*match_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    calc_match(sp, i, j);
                if OptParaFix == 0 as libc::c_int &&
                       check_feature((*(*sp).bnst_data.offset(i as isize)).f,
                                     b"\xe7\x94\xa8\xe8\xa8\x80\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_char).is_null() &&
                       !check_feature((*(*sp).bnst_data.offset(j as isize)).f,
                                      b"\xef\xbc\xb4\xe5\x90\x8d\xe4\xb8\xa6\xe7\xb5\x82\xe7\x82\xb9\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char).is_null() {
                    calc_flag = 0 as libc::c_int
                }
            } else {
                (*match_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                    0 as libc::c_int
            }
            j += 1
        }
        i += 1
    };
}
