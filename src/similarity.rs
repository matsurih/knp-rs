#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//! 中国語類似度計算

use libc;

use crate::{atoi, ctools, fprintf, free, sprintf, strcmp, strlen};
use crate::case_ipal::malloc_db_buf;
use crate::configfile::open_dict;
use crate::ctools::{stderr, strtok};
use crate::db::db_get;
use crate::structs::CDB_FILE;
use crate::types::DBM_FILE;

#[no_mangle]
pub static mut hownet_def_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut hownet_tran_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut hownet_antonym_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut hownet_category_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut hownet_sem_def_db: DBM_FILE = 0 as *const CDB_FILE as *mut CDB_FILE;
#[no_mangle]
pub static mut HownetDefExist: libc::c_int = 0;
#[no_mangle]
pub static mut HownetTranExist: libc::c_int = 0;
#[no_mangle]
pub static mut HownetAntonymExist: libc::c_int = 0;
#[no_mangle]
pub static mut HownetCategoryExist: libc::c_int = 0;
#[no_mangle]
pub static mut HownetSemDefExist: libc::c_int = 0;
/* HowNet variables */
#[no_mangle]
pub static mut tran_w1: [*mut libc::c_char; 100] = [0 as *const libc::c_char as *mut libc::c_char; 100];
#[no_mangle]
pub static mut tran_w2: [*mut libc::c_char; 100] = [0 as *const libc::c_char as *mut libc::c_char; 100];
#[no_mangle]
pub static mut concept_w1: [*mut libc::c_char; 50] = [0 as *const libc::c_char as *mut libc::c_char; 50];
#[no_mangle]
pub static mut concept_w2: [*mut libc::c_char; 50] = [0 as *const libc::c_char as *mut libc::c_char; 50];
#[no_mangle]
pub static mut concept_sem_w1: [*mut libc::c_char; 50] = [0 as *const libc::c_char as *mut libc::c_char; 50];
#[no_mangle]
pub static mut concept_sem_w2: [*mut libc::c_char; 50] = [0 as *const libc::c_char as *mut libc::c_char; 50];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_hownet()
/*==================================================================*/
{
    hownet_def_db =
        open_dict(27 as libc::c_int,
                  b"ebcf/hownet_def.db\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  &mut HownetDefExist);
    hownet_tran_db =
        open_dict(28 as libc::c_int,
                  b"ebcf/hownet_tran.db\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  &mut HownetTranExist);
    hownet_antonym_db =
        open_dict(29 as libc::c_int,
                  b"ebcf/hownet_antonym.db\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  &mut HownetAntonymExist);
    hownet_category_db =
        open_dict(30 as libc::c_int,
                  b"ebcf/hownet_category.db\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  &mut HownetCategoryExist);
    hownet_sem_def_db =
        open_dict(31 as libc::c_int,
                  b"ebcf/hownet_sem_def.db\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  &mut HownetSemDefExist);
}
/* get hownet def for word */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_hownet_def(mut str: *mut libc::c_char)
                                        -> *mut libc::c_char
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if HownetDefExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
    key =
        malloc_db_buf(strlen(str).wrapping_add(1 as libc::c_int as
            libc::c_ulong) as
            libc::c_int);
    sprintf(key, b"%s\x00" as *const u8 as *const libc::c_char, str);
    return db_get(hownet_def_db, key);
}
/* get hownet translation for word */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_hownet_tran(mut str: *mut libc::c_char)
                                         -> *mut libc::c_char
/*==================================================================*/
{
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if HownetTranExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
    key =
        malloc_db_buf(strlen(str).wrapping_add(1 as libc::c_int as
            libc::c_ulong) as
            libc::c_int);
    sprintf(key, b"%s\x00" as *const u8 as *const libc::c_char, str);
    return db_get(hownet_tran_db, key);
}
/* get hownet antonym for word */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_hownet_antonym(mut str1: *mut libc::c_char,
                                            mut str2: *mut libc::c_char)
                                            -> libc::c_int
/*==================================================================*/
{
    let mut key1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if HownetAntonymExist == 0 as libc::c_int { return 0 as libc::c_int; }
    key1 =
        malloc_db_buf(strlen(str1).wrapping_add(strlen(str2)).wrapping_add(2
            as
            libc::c_int
            as
            libc::c_ulong)
            as libc::c_int);
    sprintf(key1, b"%s:%s\x00" as *const u8 as *const libc::c_char, str1,
            str2);
    value1 = db_get(hownet_antonym_db, key1);
    if !value1.is_null() {
        ret = atoi(value1);
        free(value1 as *mut libc::c_void);
    } else {
        key2 =
            malloc_db_buf(strlen(str1).wrapping_add(strlen(str2)).wrapping_add(2
                as
                libc::c_int
                as
                libc::c_ulong)
                as libc::c_int);
        sprintf(key2, b"%s:%s\x00" as *const u8 as *const libc::c_char, str2,
                str1);
        value2 = db_get(hownet_antonym_db, key2);
        if !value2.is_null() {
            ret = atoi(value2);
            free(value2 as *mut libc::c_void);
        } else { ret = 0 as libc::c_int }
    }
    return ret;
}
/* get hownet def for sememe */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_hownet_sem_def(mut key: *mut libc::c_char)
                                            -> *mut libc::c_char
/*==================================================================*/
{
    if HownetSemDefExist == 0 as libc::c_int { return 0 as *mut libc::c_char; }
    return db_get(hownet_sem_def_db, key);
}
/* get hownet category for word */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_hownet_category(mut key: *mut libc::c_char)
                                             -> libc::c_int
/*==================================================================*/
{
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if HownetCategoryExist == 0 as libc::c_int { return 0 as libc::c_int; }
    value = db_get(hownet_category_db, key);
    if !value.is_null() {
        ret = atoi(value);
        free(value as *mut libc::c_void);
    } else { ret = 0 as libc::c_int }
    return ret;
}
/* calculate word similarity */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn similarity_chinese(mut str1: *mut libc::c_char,
                                            mut str2: *mut libc::c_char)
                                            -> libc::c_float
/*==================================================================*/
{
    let mut sim: libc::c_float = 0.;
    let mut p1: libc::c_float = 0.;
    let mut p2: libc::c_float = 0.;
    let mut p3: libc::c_float = 0.;
    let mut p4: libc::c_float = 0.;
    let mut dis: libc::c_float = 0.;
    let mut alpha: libc::c_float = 0.;
    let mut nc1: libc::c_int = 0;
    let mut nc2: libc::c_int = 0;
    let mut ns: libc::c_int = 0;
    let mut nc1_sem: libc::c_int = 0;
    let mut nc2_sem: libc::c_int = 0;
    let mut ns_sem: libc::c_int = 0;
    let mut beta1: libc::c_float = 0.;
    let mut beta2: libc::c_float = 0.;
    let mut beta3: libc::c_float = 0.;
    let mut beta4: libc::c_float = 0.;
    let mut def_w1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut def_w2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut def_sem_w1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut def_sem_w2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trans_w1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trans_w2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tran_num_w1: libc::c_int = 0;
    let mut tran_num_w2: libc::c_int = 0;
    let mut concept_num_w1: libc::c_int = 0;
    let mut concept_num_w2: libc::c_int = 0;
    let mut concept_sem_num_w1: libc::c_int = 0;
    let mut concept_sem_num_w2: libc::c_int = 0;
    let mut is_include: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut is_sim: libc::c_int = 0;
    /* initialization */
    sim = 0.0f64 as libc::c_float;
    p1 = 0.0f64 as libc::c_float;
    p2 = 0.0f64 as libc::c_float;
    p3 = 0.0f64 as libc::c_float;
    p4 = 0.0f64 as libc::c_float;
    dis = 0.0f64 as libc::c_float;
    nc1 = 0 as libc::c_int;
    nc2 = 0 as libc::c_int;
    ns = 0 as libc::c_int;
    nc1_sem = 0 as libc::c_int;
    nc2_sem = 0 as libc::c_int;
    ns_sem = 0 as libc::c_int;
    diff = 0 as libc::c_int;
    is_sim = 0 as libc::c_int;
    def_w1 = 0 as *mut libc::c_char;
    def_w2 = 0 as *mut libc::c_char;
    trans_w1 = 0 as *mut libc::c_char;
    trans_w2 = 0 as *mut libc::c_char;
    def_sem_w1 = 0 as *mut libc::c_char;
    def_sem_w2 = 0 as *mut libc::c_char;
    /* set parameters */
    alpha = 1.6f64 as libc::c_float;
    beta1 = 0.1f64 as libc::c_float;
    beta2 = 0.1f64 as libc::c_float;
    beta3 = 0.7f64 as libc::c_float;
    beta4 = 0.1f64 as libc::c_float;
    /* get translation and concept definition for words */
    def_w1 = get_hownet_def(str1);
    def_w2 = get_hownet_def(str2);
    trans_w1 = get_hownet_tran(str1);
    trans_w2 = get_hownet_tran(str2);
    if def_w1.is_null() || def_w2.is_null() || trans_w1.is_null() ||
        trans_w2.is_null() {
        return 0.0f64 as libc::c_float;
        /* memory leak? */
    }
    i = 0 as libc::c_int;
    if !trans_w1.is_null() {
        tran_w1[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        tran_w1[0 as libc::c_int as usize] =
            strtok(trans_w1, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 100 as libc::c_int {
                fprintf(stderr,
                        b"Too many translations for one word\x00" as *const u8
                            as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            tran_w1[i as usize] = 0 as *mut libc::c_char;
            tran_w1[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if tran_w1[i as usize].is_null() { break; }
        }
    }
    tran_num_w1 = i;
    i = 0 as libc::c_int;
    if !trans_w2.is_null() {
        tran_w2[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        tran_w2[0 as libc::c_int as usize] =
            strtok(trans_w2, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 100 as libc::c_int {
                fprintf(stderr,
                        b"Too many translations for one word\x00" as *const u8
                            as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            tran_w2[i as usize] = 0 as *mut libc::c_char;
            tran_w2[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if tran_w2[i as usize].is_null() { break; }
        }
    }
    tran_num_w2 = i;
    i = 0 as libc::c_int;
    if !def_w1.is_null() {
        concept_w1[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        concept_w1[0 as libc::c_int as usize] =
            strtok(def_w1, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 50 as libc::c_int {
                fprintf(stderr,
                        b"Too many concept definitions for one word\x00" as
                            *const u8 as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            concept_w1[i as usize] = 0 as *mut libc::c_char;
            concept_w1[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if concept_w1[i as usize].is_null() { break; }
        }
    }
    concept_num_w1 = i;
    i = 0 as libc::c_int;
    if !def_w2.is_null() {
        concept_w2[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        concept_w2[0 as libc::c_int as usize] =
            strtok(def_w2, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 50 as libc::c_int {
                fprintf(stderr,
                        b"Too many concept definitions for one word\x00" as
                            *const u8 as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            concept_w2[i as usize] = 0 as *mut libc::c_char;
            concept_w2[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if concept_w2[i as usize].is_null() { break; }
        }
    }
    concept_num_w2 = i;
    /* get concept definition for the first sememe of two words */
    def_sem_w1 = get_hownet_sem_def(concept_w1[0 as libc::c_int as usize]);
    def_sem_w2 = get_hownet_sem_def(concept_w2[0 as libc::c_int as usize]);
    i = 0 as libc::c_int;
    if !def_sem_w1.is_null() {
        concept_sem_w1[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        concept_sem_w1[0 as libc::c_int as usize] =
            strtok(def_sem_w1, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 50 as libc::c_int {
                fprintf(stderr,
                        b"Too many concept definitions for one sememe\x00" as
                            *const u8 as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            concept_sem_w1[i as usize] = 0 as *mut libc::c_char;
            concept_sem_w1[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if concept_sem_w1[i as usize].is_null() { break; }
        }
    }
    concept_sem_num_w1 = i;
    i = 0 as libc::c_int;
    if !def_sem_w2.is_null() {
        concept_sem_w2[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        concept_sem_w2[0 as libc::c_int as usize] =
            strtok(def_sem_w2, b":\x00" as *const u8 as *const libc::c_char);
        loop {
            i += 1;
            if i == 50 as libc::c_int {
                fprintf(stderr,
                        b"Too many concept definitions for one sememe\x00" as
                            *const u8 as *const libc::c_char);
                return 0 as libc::c_int as libc::c_float;
                /* memory leak? */
            }
            concept_sem_w2[i as usize] = 0 as *mut libc::c_char;
            concept_sem_w2[i as usize] =
                strtok(0 as *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if concept_sem_w2[i as usize].is_null() { break; }
        }
    }
    concept_sem_num_w2 = i;
    /* step 1 */
    if tran_num_w1 > 0 as libc::c_int && tran_num_w2 > 0 as libc::c_int {
        is_sim = 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < tran_num_w1 {
        if is_sim == 0 { break; }
        j = 0 as libc::c_int;
        while j < tran_num_w2 {
            if !tran_w1[i as usize].is_null() &&
                !tran_w2[j as usize].is_null() &&
                strcmp(tran_w1[i as usize], tran_w2[j as usize]) == 0 {
                j += 1
            } else {
                is_sim = 0 as libc::c_int;
                break;
            }
        }
        i += 1
    }
    if is_sim != 0 {
        sim = 1.0f64 as libc::c_float;
        return sim;
        /* memory leak? */
    }
    /* step 2 */
    if concept_num_w1 > 0 as libc::c_int && concept_num_w2 > 0 as libc::c_int
    {
        is_sim = 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < concept_num_w1 {
        if is_sim == 0 { break; }
        j = 0 as libc::c_int;
        while j < concept_num_w2 {
            if !concept_w1[i as usize].is_null() &&
                !concept_w2[j as usize].is_null() &&
                strcmp(concept_w1[i as usize], concept_w2[j as usize]) == 0
            {
                j += 1
            } else {
                is_sim = 0 as libc::c_int;
                break;
            }
        }
        i += 1
    }
    if is_sim != 0 {
        sim = 0.95f64 as libc::c_float;
        return sim;
        /* memory leak? */
    }
    /* step 3 */
    if get_hownet_antonym(str1, str2) != 0 {
        sim = 1.0f64 as libc::c_float;
        return sim;
        /* memory leak? */
    }
    /* step 5 */
    /* step 5.1 */
    is_include = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <
        (if concept_num_w1 < concept_num_w2 {
            concept_num_w1
        } else { concept_num_w2 }) {
        j = 0 as libc::c_int;
        while j <
            (if concept_num_w1 < concept_num_w2 {
                concept_num_w2
            } else { concept_num_w1 }) {
            if !concept_w1[(if concept_num_w1 < concept_num_w2 {
                i
            } else { j }) as usize].is_null() &&
                !concept_w2[(if concept_num_w1 < concept_num_w2 {
                    j
                } else { i }) as usize].is_null() &&
                strcmp(concept_w1[(if concept_num_w1 < concept_num_w2 {
                    i
                } else { j }) as usize],
                       concept_w2[(if concept_num_w1 < concept_num_w2 {
                           j
                       } else { i }) as usize]) == 0 {
                is_include += 1;
                break;
            } else { j += 1 }
        }
        i += 1
    }
    if is_include ==
        (if concept_num_w1 < concept_num_w2 {
            concept_num_w1
        } else { concept_num_w2 }) {
        p1 = 1.0f64 as libc::c_float
    }
    /* step 5.2 */
    diff =
        if concept_num_w1 < concept_num_w2 {
            concept_num_w1
        } else { concept_num_w2 };
    i = 0 as libc::c_int;
    while i <
        (if concept_num_w1 < concept_num_w2 {
            concept_num_w1
        } else { concept_num_w2 }) {
        if !concept_w1[i as usize].is_null() &&
            !concept_w2[j as usize].is_null() &&
            strcmp(concept_w1[i as usize], concept_w2[i as usize]) !=
                0 as libc::c_int {
            diff = i;
            break;
        } else { i += 1 }
    }
    if diff > 0 as libc::c_int {
        dis =
            get_hownet_category(concept_w1[(diff - 1 as libc::c_int) as
                usize]) as libc::c_float
    }
    if dis > 0 as libc::c_int as libc::c_float {
        p2 =
            (1.0f64 * alpha as libc::c_double /
                (dis + alpha) as libc::c_double) as libc::c_float
    }
    /* step 5.3 */
    if is_include ==
        (if concept_num_w1 < concept_num_w2 {
            concept_num_w1
        } else { concept_num_w2 }) {
        ns = is_include
    } else {
        i = 0 as libc::c_int;
        while i < concept_num_w1 {
            j = 0 as libc::c_int;
            while j < concept_num_w2 {
                if !concept_w1[i as usize].is_null() &&
                    !concept_w2[j as usize].is_null() &&
                    strcmp(concept_w1[i as usize], concept_w2[j as usize])
                        == 0 {
                    ns += 1
                }
                j += 1
            }
            i += 1
        }
    }
    nc1 = concept_num_w1;
    nc2 = concept_num_w2;
    if nc1 != 0 as libc::c_int || nc2 != 0 as libc::c_int {
        p3 =
            (2.0f64 * ns as libc::c_double / (nc1 + nc2) as libc::c_double) as
                libc::c_float
    } else { p3 = 0.0f64 as libc::c_float }
    /* step 5.4 */
    i = 0 as libc::c_int;
    while i < concept_sem_num_w1 {
        j = 0 as libc::c_int;
        while j < concept_sem_num_w2 {
            if !concept_sem_w1[i as usize].is_null() &&
                !concept_sem_w2[j as usize].is_null() &&
                strcmp(concept_sem_w1[i as usize],
                       concept_sem_w2[j as usize]) == 0 {
                ns_sem += 1
            }
            j += 1
        }
        i += 1
    }
    nc1_sem = concept_sem_num_w1;
    nc2_sem = concept_sem_num_w2;
    if nc1_sem != 0 as libc::c_int || nc2_sem != 0 as libc::c_int {
        p4 =
            (2.0f64 * ns_sem as libc::c_double /
                (nc1_sem + nc2_sem) as libc::c_double) as libc::c_float
    } else { p4 = 0.0f64 as libc::c_float }
    /* step 5.5 */
    sim = p1 * beta1 + p2 * beta2 + p3 * beta3 + p4 * beta4;
    /* free memory */
    if !trans_w1.is_null() { free(trans_w1 as *mut libc::c_void); }
    if !trans_w2.is_null() { free(trans_w2 as *mut libc::c_void); }
    if !def_w1.is_null() { free(def_w1 as *mut libc::c_void); }
    if !def_w2.is_null() { free(def_w2 as *mut libc::c_void); }
    if !def_sem_w1.is_null() { free(def_sem_w1 as *mut libc::c_void); }
    if !def_sem_w2.is_null() { free(def_sem_w2 as *mut libc::c_void); }
    return sim;
}
