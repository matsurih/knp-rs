#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{case_analysis, case_data, case_ipal, ctools, db, dic, lib_scase, lib_sm, structs, tools, types};

//! 用言の共起情報

#[no_mangle]
pub unsafe extern "C" fn init_event() {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*dic::DICT.as_mut_ptr().offset(13 as libc::c_int as isize)).is_null() {
        filename = case_ipal::check_dict_filename(*dic::DICT.as_mut_ptr().offset(13 as libc::c_int as isize), 0 as libc::c_int)
    } else {
        filename = case_ipal::check_dict_filename(b"event/event.db\x00" as *const u8 as *const libc::c_char as *mut libc::c_char, 0 as libc::c_int)
    }
    if tools::OptDisplay == 3 as libc::c_int {
        fprintf(ctools::Outfp, b"Opening %s ... \x00" as *const u8 as *const libc::c_char, filename);
    }
    tools::event_db = db::db_read_open(filename);
    if tools::event_db.is_null() {
        if tools::OptDisplay == 3 as libc::c_int {
            fputs(b"failed.\n\x00" as *const u8 as *const libc::c_char, ctools::Outfp);
        }
        tools::EventDicExist = 0 as libc::c_int
    } else {
        if tools::OptDisplay == 3 as libc::c_int {
            fputs(b"done.\n\x00" as *const u8 as *const libc::c_char, ctools::Outfp);
        }
        tools::EventDicExist = (0 as libc::c_int == 0) as libc::c_int
    }
    free(filename as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn close_event() {
    if tools::EventDicExist == (0 as libc::c_int == 0) as libc::c_int {
        db::db_close(tools::event_db);
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_event(mut cp: *mut libc::c_char) -> libc::c_float {
    // let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_float = 0.;
    value = db::db_get(tools::event_db, cp);
    if !value.is_null() {
        retval = atof(value) as libc::c_float;
        free(value as *mut libc::c_void);
        return retval
    }
    return 0 as libc::c_int as libc::c_float;
}

#[no_mangle]
pub unsafe extern "C" fn make_voice_str(mut ptr: *mut types::TAG_DATA) -> *mut libc::c_char {
    if (*ptr).voice & 2 as libc::c_int != 0 {
        return b":P\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    } else {
        if (*ptr).voice & 1 as libc::c_int != 0 {
            return b":C\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        } else {
            if (*ptr).voice & 4 as libc::c_int != 0 {
                return b":PC\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char
            }
        }
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn make_pred_str_with_cc(mut sp: *mut types::SENTENCE_DATA, mut ptr: *mut types::TAG_DATA, mut flag: libc::c_int) -> *mut libc::c_char

 {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut voice: [libc::c_char; 3] = [0; 3];
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut closest: libc::c_int = 0;
    cp = lib_sm::check_feature(
        (*ptr).f,
        b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if !cp.is_null() {
        vtype =
            strdup(cp.offset(strlen(b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as *const libc::c_char) as isize))
    } else {
        vtype =
            strdup(b":\xe5\x8b\x95\x00" as *const u8 as *const libc::c_char)
    }
    cp = make_voice_str(ptr);
    if !cp.is_null() {
        strcpy(voice.as_mut_ptr(), cp);
    } else {
        voice[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    /* 用言部分 */
    str = lib_scase::make_pred_string(
        ptr,
        0 as *mut structs::MRPH_DATA,
        0 as *mut libc::c_char,
        tools::OptCaseFlag & 32 as libc::c_int,
        flag,
        0 as libc::c_int
    );
    strcat(str, vtype);
    if voice[0 as libc::c_int as usize] != 0 {
        strcat(str, voice.as_mut_ptr());
    }
    free(vtype as *mut libc::c_void);
    if flag == (0 as libc::c_int == 0) as libc::c_int {
        /* 直前格要素の取得 */
        closest = case_analysis::get_closest_case_component(sp, (*ptr).cpm_ptr);
        if closest > -(1 as libc::c_int) {
            cp = case_analysis::pp_code_to_kstr((*(*ptr).cpm_ptr).cf.pp[closest as usize][0 as libc::c_int as usize]);
            ccstr = ctools::malloc_data(
                strlen(cp).wrapping_add(strlen((*(*(*(*ptr).cpm_ptr).elem_b_ptr[closest as usize]).head_ptr).Goi.as_mut_ptr())).wrapping_add(strlen(str)).wrapping_add(3 as libc::c_int as libc::c_ulong),
                b"make_pred_str_with_cc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            ) as *mut libc::c_char;
            sprintf(ccstr, b"%s-%s-%s\x00" as *const u8 as *const libc::c_char, (*(*(*(*ptr).cpm_ptr).elem_b_ptr[closest as usize]).head_ptr).Goi.as_mut_ptr(), cp, str);
            free(str as *mut libc::c_void);
            return ccstr
        }
    }
    return str;
}

#[no_mangle]
pub unsafe extern "C" fn get_event_value(mut sp1: *mut types::SENTENCE_DATA,
                                         mut p1: *mut types::TAG_DATA,
                                         mut sp2: *mut types::SENTENCE_DATA,
                                         mut p2: *mut types::TAG_DATA)
 -> libc::c_float {
    // let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_float = 0.;
    if tools::EventDicExist == (0 as libc::c_int == 0) as libc::c_int {
        str1 =
            make_pred_str_with_cc(sp1, p1,
                                  (0 as libc::c_int == 0) as libc::c_int);
        str2 =
            make_pred_str_with_cc(sp2, p2,
                                  (0 as libc::c_int == 0) as libc::c_int);
        buf =
            ctools::malloc_data(
                strlen(str1).wrapping_add(strlen(str2)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                b"get_event_value\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            ) as *mut libc::c_char;
        sprintf(buf, b"%s|%s\x00" as *const u8 as *const libc::c_char, str2, str1);
        free(str1 as *mut libc::c_void);
        free(str2 as *mut libc::c_void);
        val = get_event(buf);
        /* backoff */
        if val == 0 as libc::c_int as libc::c_float {
            str1 = make_pred_str_with_cc(sp1, p1, 0 as libc::c_int);
            str2 = make_pred_str_with_cc(sp2, p2, (0 as libc::c_int == 0) as libc::c_int);
            sprintf(buf, b"%s|%s\x00" as *const u8 as *const libc::c_char, str2, str1);
            free(str1 as *mut libc::c_void);
            free(str2 as *mut libc::c_void);
            val = get_event(buf);
            if val == 0 as libc::c_int as libc::c_float {
                str1 = make_pred_str_with_cc(sp1, p1, (0 as libc::c_int == 0) as libc::c_int);
                str2 = make_pred_str_with_cc(sp2, p2, 0 as libc::c_int);
                sprintf(buf, b"%s|%s\x00" as *const u8 as *const libc::c_char, str2, str1);
                free(str1 as *mut libc::c_void);
                free(str2 as *mut libc::c_void);
                val = get_event(buf);
                if val == 0 as libc::c_int as libc::c_float {
                    str1 = make_pred_str_with_cc(sp1, p1, 0 as libc::c_int);
                    str2 = make_pred_str_with_cc(sp2, p2, 0 as libc::c_int);
                    sprintf(buf, b"%s|%s\x00" as *const u8 as *const libc::c_char, str2, str1);
                    free(str1 as *mut libc::c_void);
                    free(str2 as *mut libc::c_void);
                    val = get_event(buf)
                }
            }
        }
        free(buf as *mut libc::c_void);
        return val
    }
    return -(1 as libc::c_int) as libc::c_float;
}

#[no_mangle]
pub unsafe extern "C" fn get_cf_event_value(mut cf1: *mut types::CASE_FRAME, mut cf2: *mut types::CASE_FRAME) -> libc::c_float {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_float = 0.;
    if tools::EventDicExist == (0 as libc::c_int == 0) as libc::c_int {
        buf =
            ctools::malloc_data(
                strlen((*cf1).cf_id.as_mut_ptr()).wrapping_add(strlen((*cf2).cf_id.as_mut_ptr())).wrapping_add(2 as libc::c_int as libc::c_ulong),
                b"get_cf_event_value\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            ) as *mut libc::c_char;
        sprintf(buf, b"%s-%s\x00" as *const u8 as *const libc::c_char, (*cf1).cf_id.as_mut_ptr(), (*cf2).cf_id.as_mut_ptr());
        val = get_event(buf);
        free(buf as *mut libc::c_void);
        return val
    }
    return -(1 as libc::c_int) as libc::c_float;
}
