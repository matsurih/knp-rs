#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

/*
------------------------------------------------------------------------------
	LOCAL:
	definition of global variables
------------------------------------------------------------------------------
*/
use crate::juman::ctools::strchr;

static mut hankaku_table: [libc::c_uchar; 98] =
    unsafe {
        *::std::mem::transmute::<&[u8; 98],
                                 &mut [libc::c_uchar; 98]>(b"!\"#$%&()*+,-.\'/0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^abcdefghijklmnopqrstuvwxyz{|}^~_ --\x00")
    };
static mut zenkaku_table: [libc::c_uchar; 292] =
    unsafe {
        *::std::mem::transmute::<&[u8; 292],
                                 &mut [libc::c_uchar; 292]>(b"\xef\xbc\x81\xe2\x80\x9d\xef\xbc\x83\xef\xbc\x84\xef\xbc\x85\xef\xbc\x86\xef\xbc\x88\xef\xbc\x89\xef\xbc\x8a\xef\xbc\x8b\xef\xbc\x8c\xe2\x88\x92\xef\xbc\x8e\xe2\x80\x99\xef\xbc\x8f\xef\xbc\x90\xef\xbc\x91\xef\xbc\x92\xef\xbc\x93\xef\xbc\x94\xef\xbc\x95\xef\xbc\x96\xef\xbc\x97\xef\xbc\x98\xef\xbc\x99\xef\xbc\x9a\xef\xbc\x9b\xef\xbc\x9c\xef\xbc\x9d\xef\xbc\x9e\xef\xbc\x9f\xef\xbc\xa0\xef\xbc\xa1\xef\xbc\xa2\xef\xbc\xa3\xef\xbc\xa4\xef\xbc\xa5\xef\xbc\xa6\xef\xbc\xa7\xef\xbc\xa8\xef\xbc\xa9\xef\xbc\xaa\xef\xbc\xab\xef\xbc\xac\xef\xbc\xad\xef\xbc\xae\xef\xbc\xaf\xef\xbc\xb0\xef\xbc\xb1\xef\xbc\xb2\xef\xbc\xb3\xef\xbc\xb4\xef\xbc\xb5\xef\xbc\xb6\xef\xbc\xb7\xef\xbc\xb8\xef\xbc\xb9\xef\xbc\xba\xef\xbc\xbb\xef\xbf\xa5\xef\xbc\xbd\xef\xbc\xbe\xef\xbd\x81\xef\xbd\x82\xef\xbd\x83\xef\xbd\x84\xef\xbd\x85\xef\xbd\x86\xef\xbd\x87\xef\xbd\x88\xef\xbd\x89\xef\xbd\x8a\xef\xbd\x8b\xef\xbd\x8c\xef\xbd\x8d\xef\xbd\x8e\xef\xbd\x8f\xef\xbd\x90\xef\xbd\x91\xef\xbd\x92\xef\xbd\x93\xef\xbd\x94\xef\xbd\x95\xef\xbd\x96\xef\xbd\x97\xef\xbd\x98\xef\xbd\x99\xef\xbd\x9a\xef\xbd\x9b\xef\xbd\x9c\xef\xbd\x9d\xef\xbc\xbe\xef\xbf\xa3\xef\xbc\xbf\xe3\x80\x80\xe2\x80\x90\xe2\x80\x95\x00")
    };
static mut str_buffer: [libc::c_uchar; 100000] = [0; 100000];
/*
------------------------------------------------------------------------------
	FUNCTION:
	<zentohan>: convert (zenkaku)str -> (hankaku)str_out
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn zentohan(mut str: *mut libc::c_uchar)
 -> *mut libc::c_uchar {
    let mut str_out: *mut libc::c_uchar = str_buffer.as_mut_ptr();
    while *str != 0 {
        if *str as libc::c_int != 0 && 0x80 as libc::c_int != 0 &&
               (*str.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
                    0x80 as libc::c_int != 0) {
            let mut ptr: libc::c_int = 0;
            ptr = 0 as libc::c_int;
            while *zenkaku_table.as_mut_ptr().offset(ptr as isize) != 0 {
                if zenkaku_table[ptr as usize] as libc::c_int ==
                       *str as libc::c_int &&
                       zenkaku_table[(ptr + 1 as libc::c_int) as usize] as
                           libc::c_int ==
                           *str.offset(1 as libc::c_int as isize) as
                               libc::c_int {
                    let fresh0 = str_out;
                    str_out = str_out.offset(1);
                    *fresh0 =
                        hankaku_table[(ptr / 2 as libc::c_int) as usize];
                    break ;
                } else { ptr += 2 as libc::c_int }
            }
            if *zenkaku_table.as_mut_ptr().offset(ptr as isize) as libc::c_int
                   == 0 as libc::c_int {
                let fresh1 = str_out;
                str_out = str_out.offset(1);
                *fresh1 = *str;
                let fresh2 = str_out;
                str_out = str_out.offset(1);
                *fresh2 = *str.offset(1 as libc::c_int as isize)
            }
            str = str.offset(2 as libc::c_int as isize)
        } else {
            let fresh3 = str;
            str = str.offset(1);
            let fresh4 = str_out;
            str_out = str_out.offset(1);
            *fresh4 = *fresh3
        }
    }
    *str_out = 0 as libc::c_int as libc::c_uchar;
    return str_buffer.as_mut_ptr();
}
/*
------------------------------------------------------------------------------
	FUNCTION
	<hantozen>: convert (hankaku)str -> (zenkaku)str_out
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn hantozen(mut str: *mut libc::c_uchar)
 -> *mut libc::c_uchar {
    let mut str_out: *mut libc::c_uchar = str_buffer.as_mut_ptr();
    while *str != 0 {
        let mut str_tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        str_tmp =
            strchr(hankaku_table.as_mut_ptr() as *const libc::c_char,
                   *str as libc::c_int) as *mut libc::c_uchar;
        if !str_tmp.is_null() {
            let mut ptr: libc::c_int =
                str_tmp.wrapping_offset_from(hankaku_table.as_mut_ptr()) as
                    libc::c_long as libc::c_int;
            let fresh5 = str_out;
            str_out = str_out.offset(1);
            *fresh5 = zenkaku_table[(ptr << 1 as libc::c_int) as usize];
            let fresh6 = str_out;
            str_out = str_out.offset(1);
            *fresh6 =
                zenkaku_table[((ptr << 1 as libc::c_int) + 1 as libc::c_int)
                                  as usize]
        } else {
            let fresh7 = str_out;
            str_out = str_out.offset(1);
            *fresh7 = *str
        }
        str = str.offset(1)
    }
    *str_out = 0 as libc::c_int as libc::c_uchar;
    return str_buffer.as_mut_ptr();
}
