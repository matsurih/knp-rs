#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{fclose, fgets, fopen, fprintf, sscanf, strcmp, strncmp};
use crate::ctools::{exit, malloc, malloc_data, stderr, strtok};
use crate::structs::{_dtcond, CDB_FILE, DT, DTRULE};
use crate::types::{DBM_FILE, DTCOND, FILE};

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
#[no_mangle]
pub static mut DTFile: [*mut libc::c_char; 44] = [0 as *const libc::c_char as *mut libc::c_char; 44];
#[no_mangle]
pub static mut DTrule: [*mut DT; 44] = [0 as *const DT as *mut DT; 44];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn trans_eq(mut eq: *mut libc::c_char) -> libc::c_int {
    if strcmp(eq, b"eq\x00" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int;
    } else if strcmp(eq, b"ne\x00" as *const u8 as *const libc::c_char) == 0 {
        return 2 as libc::c_int;
    } else if strcmp(eq, b"gt\x00" as *const u8 as *const libc::c_char) == 0 {
        return 3 as libc::c_int;
    } else if strcmp(eq, b"ge\x00" as *const u8 as *const libc::c_char) == 0 {
        return 4 as libc::c_int;
    } else if strcmp(eq, b"lt\x00" as *const u8 as *const libc::c_char) == 0 {
        return 5 as libc::c_int;
    } else if strcmp(eq, b"le\x00" as *const u8 as *const libc::c_char) == 0 {
        return 6 as libc::c_int;
    } else {
        fprintf(stderr, b";; Invalid DT equal (%s)\n\x00" as *const u8 as *const libc::c_char, eq);
        exit(1 as libc::c_int);
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_svm_str(mut buf: *mut libc::c_char,
                                      mut rule: *mut DTRULE) -> libc::c_int
/*==================================================================*/
{
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nc: *mut *mut DTCOND = 0 as *mut *mut DTCOND;
    nc = &mut (*rule).cond;
    token = strtok(buf, b" \x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        *nc =
            malloc(::std::mem::size_of::<DTCOND>() as libc::c_ulong) as
                *mut DTCOND;
        sscanf(token, b"%d:%f\x00" as *const u8 as *const libc::c_char,
               &mut (**nc).num as *mut libc::c_int,
               &mut (**nc).value as *mut libc::c_float);
        (**nc).eq = 0 as libc::c_int;
        (**nc).next = 0 as *mut _dtcond;
        token =
            strtok(0 as *mut libc::c_char,
                   b" \x00" as *const u8 as *const libc::c_char);
        nc = &mut (**nc).next
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_dt_str(mut buf: *mut libc::c_char,
                                     mut rule: *mut DTRULE) -> libc::c_int
/*==================================================================*/
{
    let mut class: [libc::c_char; 3] = [0; 3];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eq: [libc::c_char; 3] = [0; 3];
    let mut nc: *mut *mut DTCOND = 0 as *mut *mut DTCOND;
    sscanf(buf, b"%s %f %[^\n]\x00" as *const u8 as *const libc::c_char,
           class.as_mut_ptr(), &mut (*rule).cf as *mut libc::c_float, buf);
    if strncmp(class.as_mut_ptr(),
               b"OK\x00" as *const u8 as *const libc::c_char,
               2 as libc::c_int as libc::c_ulong) != 0 {
        return 1 as libc::c_int;
    } else { (*rule).class = 1 as libc::c_int }
    nc = &mut (*rule).cond;
    token = strtok(buf, b" \x00" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        *nc =
            malloc_data(::std::mem::size_of::<DTCOND>() as libc::c_ulong,
                        b"read_dt_str\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char) as *mut DTCOND;
        sscanf(token, b"%d:%[^:]:%f\x00" as *const u8 as *const libc::c_char,
               &mut (**nc).num as *mut libc::c_int, eq.as_mut_ptr(),
               &mut (**nc).value as *mut libc::c_float);
        (**nc).eq = trans_eq(eq.as_mut_ptr());
        (**nc).next = 0 as *mut _dtcond;
        token =
            strtok(0 as *mut libc::c_char,
                   b" \x00" as *const u8 as *const libc::c_char);
        nc = &mut (**nc).next
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn read_dt_file(mut dt: *mut DT,
                                      mut filename: *mut libc::c_char)
/*==================================================================*/
{
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    (*dt).ContextRuleNum = 0 as libc::c_int;
    if filename.is_null() {
        fprintf(stderr,
                b";; DTFile is not specified!!\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    } else {
        fp = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            fprintf(stderr,
                    b";; Cannot open file (%s) !!\n\x00" as *const u8 as
                        *const libc::c_char, filename);
            exit(1 as libc::c_int);
        }
    }
    while !fgets(buf.as_mut_ptr(), 5120 as libc::c_int, fp).is_null() {
        if read_dt_str(buf.as_mut_ptr(),
                       &mut *(*dt).ContextRules.as_mut_ptr().offset((*dt).ContextRuleNum
                           as
                           isize))
            == 0 as libc::c_int {
            (*dt).ContextRuleNum += 1
        }
    }
    fclose(fp);
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dt_comp_elem(mut r: libc::c_float,
                                      mut d: libc::c_float,
                                      mut eq: libc::c_int,
                                      mut flag: libc::c_int) -> libc::c_int
/*==================================================================*/
{
    /* flagがたっていれば、floatのまま評価する */
    if flag != 0 {
        if eq == 1 as libc::c_int {
            return if d == r { 1 as libc::c_int } else { 0 as libc::c_int };
        } else {
            if eq == 2 as libc::c_int {
                return if d != r {
                    1 as libc::c_int
                } else { 0 as libc::c_int };
            } else {
                if eq == 3 as libc::c_int {
                    return if d > r {
                        1 as libc::c_int
                    } else { 0 as libc::c_int };
                } else {
                    if eq == 4 as libc::c_int {
                        return if d >= r {
                            1 as libc::c_int
                        } else { 0 as libc::c_int };
                    } else {
                        if eq == 5 as libc::c_int {
                            return if d < r {
                                1 as libc::c_int
                            } else { 0 as libc::c_int };
                        } else {
                            if eq == 6 as libc::c_int {
                                return if d <= r {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int };
                            }
                        }
                    }
                }
            }
        }
    }
    if eq == 1 as libc::c_int {
        return if d as libc::c_int == r as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    } else {
        if eq == 2 as libc::c_int {
            return if d as libc::c_int != r as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        } else {
            if eq == 3 as libc::c_int {
                return if d as libc::c_int > r as libc::c_int {
                    1 as libc::c_int
                } else { 0 as libc::c_int };
            } else {
                if eq == 4 as libc::c_int {
                    return if d as libc::c_int >= r as libc::c_int {
                        1 as libc::c_int
                    } else { 0 as libc::c_int };
                } else {
                    if eq == 5 as libc::c_int {
                        return if (d as libc::c_int) < r as libc::c_int {
                            1 as libc::c_int
                        } else { 0 as libc::c_int };
                    } else {
                        if eq == 6 as libc::c_int {
                            return if d as libc::c_int <= r as libc::c_int {
                                1 as libc::c_int
                            } else { 0 as libc::c_int };
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn dt_classify(mut data: *mut libc::c_char, mut pp: libc::c_int) -> libc::c_float
/*==================================================================*/
{
    let mut dt: *mut DT = 0 as *mut DT; /* テスト用feature */
    let mut d: DTRULE = DTRULE { class: 0, cf: 0., cond: 0 as *mut DTCOND };
    let mut rc: *mut DTCOND = 0 as *mut DTCOND;
    let mut dc: *mut DTCOND = 0 as *mut DTCOND;
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    /* 0はすべての格用 */
    if !DTrule[0 as libc::c_int as usize].is_null() {
        dt = DTrule[0 as libc::c_int as usize]
    } else { dt = DTrule[pp as usize] }
    read_svm_str(data, &mut d);
    i = 0 as libc::c_int;
    while i < (*dt).ContextRuleNum {
        rc = (*dt).ContextRules[i as usize].cond;
        flag = 1 as libc::c_int;
        /* rule条件のループ */
        while !rc.is_null() {
            dc = d.cond;
            /* rule側と同じfeatureをデータから探す */
            while (*rc).num != (*dc).num {
                dc = (*dc).next;
                if dc.is_null() {
                    fprintf(stderr,
                            b";; DT rule mismatched! (%d)\n\x00" as *const u8
                                as *const libc::c_char, (*rc).num);
                    exit(1 as libc::c_int);
                }
            }
            /* feature番号が1のとき類似度なので、floatで比較 */
            if dt_comp_elem((*rc).value, (*dc).value, (*rc).eq,
                            if (*rc).num == 1 as libc::c_int {
                                1 as libc::c_int
                            } else { 0 as libc::c_int }) == 0 {
                flag = 0 as libc::c_int;
                break;
            } else { rc = (*rc).next }
        }
        /* すべての条件を満たしたとき */
        if flag != 0 {
            return (*dt).ContextRules[i as usize].cf;
            /* dc = d.cond;
	    while (dc) {
		* 類似度を返す *
		if (dc->num == 1) {
		    return dc->value;
		}
		dc = dc->next;
	    } */
        }
        i += 1
    }
    return -(1 as libc::c_int) as libc::c_float;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn init_dt()
/*==================================================================*/
{
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 44 as libc::c_int {
        if !DTFile[i as usize].is_null() {
            DTrule[i as usize] =
                malloc_data(::std::mem::size_of::<DT>() as libc::c_ulong,
                            b"init_dt\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char) as *mut DT;
            read_dt_file(DTrule[i as usize], DTFile[i as usize]);
            if i == 0 as libc::c_int { break; }
        } else { DTrule[i as usize] = 0 as *mut DT }
        i += 1
    };
}
/*====================================================================
                               END
====================================================================*/
