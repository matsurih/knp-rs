#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]


use crate::juman::consts::OtherError;
use crate::juman::ctools::{Class, error, Form, strcmp, Type};

#[no_mangle]
pub unsafe extern "C" fn get_hinsi_id(mut x: *mut libc::c_uchar)
 -> libc::c_int {
    let mut i: libc::c_int = 0; /* yamaji */
    if x.is_null() {
        error(OtherError as libc::c_int,
              b"NULL string for hinsi.\x00" as *const u8 as
                  *const libc::c_char as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if strcmp(x as *const libc::c_char,
              b"*\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if strcmp(x as *const libc::c_char,
              b"\xe9\x80\xa3\xe8\xaa\x9e\x00" as *const u8 as
                  *const libc::c_char) == 0 as libc::c_int {
        return 127 as libc::c_int
    }
    i = 1 as libc::c_int;
    while strcmp(Class[i as usize][0 as libc::c_int as usize].id as
                     *const libc::c_char, x as *const libc::c_char) != 0 {
        i += 1;
        if Class[i as usize][0 as libc::c_int as usize].id.is_null() {
            error(OtherError as libc::c_int, x as *mut libc::c_char,
                  b" is undefined in \x00" as *const u8 as
                      *const libc::c_char,
                  b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char,
                  b".\x00" as *const u8 as *const libc::c_char,
                  -(1 as libc::c_int) as *mut libc::c_char);
        }
    }
    return i;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<bunrui>: return <int:i>
	          Class[hinsi][i].id == x
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn get_bunrui_id(mut x: *mut libc::c_uchar,
                                       mut hinsi: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if x.is_null() {
        error(OtherError as libc::c_int,
              b"NULL string for bunrui.\x00" as *const u8 as
                  *const libc::c_char as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if strcmp(x as *const libc::c_char,
              b"*\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if Class[hinsi as usize][1 as libc::c_int as usize].id.is_null() {
        error(OtherError as libc::c_int,
              Class[hinsi as usize][0 as libc::c_int as usize].id as
                  *mut libc::c_char,
              b" has no bunrui in \x00" as *const u8 as *const libc::c_char,
              b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    i = 1 as libc::c_int;
    while strcmp(Class[hinsi as usize][i as usize].id as *const libc::c_char,
                 x as *const libc::c_char) != 0 {
        i += 1;
        if Class[hinsi as usize][i as usize].id.is_null() {
            error(OtherError as libc::c_int,
                  Class[hinsi as usize][0 as libc::c_int as usize].id as
                      *mut libc::c_char,
                  b" does not have bunrui \x00" as *const u8 as
                      *const libc::c_char, x as *mut libc::c_char,
                  b" in \x00" as *const u8 as *const libc::c_char,
                  b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char,
                  b".\x00" as *const u8 as *const libc::c_char,
                  -(1 as libc::c_int) as *mut libc::c_char);
        }
    }
    return i;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<type>: return <int:i>
	        Type[i].name == x
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn get_type_id(mut x: *mut libc::c_uchar)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if x.is_null() {
        error(OtherError as libc::c_int,
              b"NULL string for type.\x00" as *const u8 as *const libc::c_char
                  as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if strcmp(x as *const libc::c_char,
              b"*\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    i = 1 as libc::c_int;
    while strcmp(Type[i as usize].name as *const libc::c_char,
                 x as *const libc::c_char) != 0 {
        i += 1;
        if Type[i as usize].name.is_null() {
            error(OtherError as libc::c_int, x as *mut libc::c_char,
                  b" is undefined in \x00" as *const u8 as
                      *const libc::c_char,
                  b"JUMAN.katuyou\x00" as *const u8 as *const libc::c_char,
                  b".\x00" as *const u8 as *const libc::c_char,
                  -(1 as libc::c_int) as *mut libc::c_char);
        }
    }
    return i;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<type>: return <int:i>
	        Form[type][i].name == x
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn get_form_id(mut x: *mut libc::c_uchar,
                                     mut type_0: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if x.is_null() {
        error(OtherError as libc::c_int,
              b"NULL string for form.\x00" as *const u8 as *const libc::c_char
                  as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if strcmp(x as *const libc::c_char,
              b"*\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if type_0 == 0 as libc::c_int {
        error(OtherError as libc::c_int,
              b"Invalid type number for \x00" as *const u8 as
                  *const libc::c_char as *mut libc::c_char,
              x as *mut libc::c_char,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    i = 1 as libc::c_int;
    while strcmp(Form[type_0 as usize][i as usize].name as
                     *const libc::c_char, x as *const libc::c_char) != 0 {
        i += 1;
        if Form[type_0 as usize][i as usize].name.is_null() {
            error(OtherError as libc::c_int,
                  Type[type_0 as usize].name as *mut libc::c_char,
                  b" does not have katuyou \x00" as *const u8 as
                      *const libc::c_char, x as *mut libc::c_char,
                  b" in \x00" as *const u8 as *const libc::c_char,
                  b"JUMAN.katuyou\x00" as *const u8 as *const libc::c_char,
                  b".\x00" as *const u8 as *const libc::c_char,
                  -(1 as libc::c_int) as *mut libc::c_char);
        }
    }
    return i;
}
