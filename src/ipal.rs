#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

use crate::ctools::{exit, stderr, stdin};
use crate::structs::IPAL_TRANS_FRAME;
use crate::tools;
use crate::types::FILE;


#[no_mangle]
pub static mut buffer: [libc::c_char; 8192000] = [0; 8192000];
#[no_mangle]
pub static mut ipal_frame: IPAL_TRANS_FRAME = IPAL_TRANS_FRAME{DATA: [0; 8192000],};
#[no_mangle]
pub static mut fp_idx: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut fp_dat: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn fprint_ipal_idx(mut fp: *mut FILE,
                                         mut entry: *mut libc::c_uchar,
                                         mut hyouki: *mut libc::c_uchar,
                                         mut pp: *mut libc::c_uchar,
                                         mut address: libc::c_ulong,
                                         mut size: libc::c_int,
                                         mut flag: libc::c_int) {
    let mut output_buf: [libc::c_uchar; 8192000] = [0; 8192000];
    let mut point: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut char_bytes: libc::c_int = 2 as libc::c_int;
    if !pp.is_null() {
        point = hyouki;
        while *point != 0 {
            /* 用例の区切り */
            if *point as libc::c_int == ' ' as i32 {
                output_buf[length as usize] = '\u{0}' as i32 as libc::c_uchar;
                if length > 0 as libc::c_int &&
                       (output_buf[0 as libc::c_int as usize] as libc::c_int
                            != '<' as i32 ||
                            output_buf[strlen(output_buf.as_mut_ptr() as
                                                  *const libc::c_char).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                           as usize] as libc::c_int ==
                                '>' as i32) {
                    /* <CT など以外 */
                    fprintf(fp,
                            b"%s-%s-%s %lu:%d\n\x00" as *const u8 as
                                *const libc::c_char, output_buf.as_mut_ptr(),
                            pp, entry, address, size);
                }
                length = 0 as libc::c_int
            } else {
                if *point as libc::c_int == ':' as i32 {
                    let fresh0 = length;
                    length = length + 1;
                    output_buf[fresh0 as usize] =
                        '\u{0}' as i32 as libc::c_uchar
                } else {
                    let fresh1 = length;
                    length = length + 1;
                    output_buf[fresh1 as usize] = *point
                }
                /* 日本語ならもう1byte進める */
                if *point as libc::c_int & 0x80 as libc::c_int != 0 {
                    char_bytes = tools::utf8_length(*point);
                    i = 1 as libc::c_int;
                    while i < char_bytes {
                        let fresh2 = length;
                        length = length + 1;
                        output_buf[fresh2 as usize] =
                            *point.offset(1 as libc::c_int as isize);
                        point = point.offset(1);
                        i += 1
                    }
                }
            }
            point = point.offset(1)
        }
        output_buf[length as usize] = '\u{0}' as i32 as libc::c_uchar;
        if length > 0 as libc::c_int &&
               (output_buf[0 as libc::c_int as usize] as libc::c_int !=
                    '<' as i32 ||
                    output_buf[strlen(output_buf.as_mut_ptr() as
                                          *const libc::c_char).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                   as usize] as libc::c_int == '>' as i32) {
            /* <CT など以外 */
            fprintf(fp,
                    b"%s-%s-%s %lu:%d\n\x00" as *const u8 as
                        *const libc::c_char, output_buf.as_mut_ptr(), pp,
                    entry, address, size);
        }
    } else {
        fprintf(fp, b"%s %lu:%d\n\x00" as *const u8 as *const libc::c_char,
                hyouki, address, size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_data(mut ipal_frame_0: *mut IPAL_TRANS_FRAME,
                                    mut point: *mut libc::c_int,
                                    mut closest: *mut libc::c_int,
                                    mut writesize: libc::c_int,
                                    mut casenum: libc::c_int,
                                    mut address: *mut libc::c_ulong,
                                    mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    fprint_ipal_idx(fp_idx,
                    (*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                                 as isize),
                    (*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                                 as isize),
                    0 as *mut libc::c_uchar, *address, writesize, flag);
    /* 「直前格要素-直前格-用言」で登録」 */
    if flag != 0 {
        /* ORの格フレームを除く */
        i = 0 as libc::c_int; /* *をけす */
        while i < casenum {
            if *closest.offset(i as isize) > 0 as libc::c_int &&
                   *(*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset(*closest.offset(i
                                                                                               as
                                                                                               isize)
                                                                               as
                                                                               isize)
                                                                 as isize) as
                       libc::c_int != '\u{0}' as i32 {
                pp =
                    strdup((*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset((i
                                                                                       *
                                                                                       3
                                                                                           as
                                                                                           libc::c_int
                                                                                       +
                                                                                       4
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      isize)
                                                                        as
                                                                        isize)
                               as *const libc::c_char);
                *pp.offset(strlen(pp) as
                               isize).offset(-(1 as libc::c_int as isize)) =
                    '\u{0}' as i32 as libc::c_char;
                fprint_ipal_idx(fp_idx,
                                (*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                                             as
                                                                             isize),
                                (*ipal_frame_0).DATA.as_mut_ptr().offset(*point.offset(*closest.offset(i
                                                                                                           as
                                                                                                           isize)
                                                                                           as
                                                                                           isize)
                                                                             as
                                                                             isize),
                                pp as *mut libc::c_uchar, *address, writesize,
                                0 as libc::c_int);
                free(pp as *mut libc::c_void);
            }
            i += 1
        }
    }
    /* データ書き出し */
    if fwrite(ipal_frame_0 as *const libc::c_void, writesize as libc::c_ulong,
              1 as libc::c_int as libc::c_ulong, fp_dat) <
           1 as libc::c_int as libc::c_ulong {
        fprintf(stderr,
                b"Error in fwrite.\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    *address = (*address).wrapping_add(writesize as libc::c_ulong);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut tag: [libc::c_char; 256] = [0; 256];
    let mut DATA: [libc::c_char; 8192000] = [0; 8192000];
    // let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut i: libc::c_int = 0;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut item: libc::c_int = 0;
    let mut casenum: libc::c_int = 0;
    let mut closest: [libc::c_int; 20] = [0; 20];
    let mut point: [libc::c_int; 64] = [0; 64];
    let mut address: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if argc < 3 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s index-filename data-filename\n\x00" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    fp_idx =
        fopen(*argv.offset(1 as libc::c_int as isize),
              b"w\x00" as *const u8 as *const libc::c_char);
    if fp_idx.is_null() {
        fprintf(stderr,
                b"Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(1 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    fp_dat =
        fopen(*argv.offset(2 as libc::c_int as isize),
              b"wb\x00" as *const u8 as *const libc::c_char);
    if fp_dat.is_null() {
        fprintf(stderr,
                b"Cannot open file (%s) !!\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(2 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    loop  {
        line += 1;
        if fgets(buffer.as_mut_ptr(), 8192000 as libc::c_int, stdin).is_null()
           {
            /* 最後のデータ */
            write_data(&mut ipal_frame, point.as_mut_ptr(),
                       closest.as_mut_ptr(), pos, casenum, &mut address,
                       flag);
            fclose(fp_idx);
            fclose(fp_dat);
            return 0 as libc::c_int
        }
        sscanf(buffer.as_mut_ptr(),
               b"%s %[^\n]\n\x00" as *const u8 as *const libc::c_char,
               tag.as_mut_ptr(), DATA.as_mut_ptr());
        if strcmp(tag.as_mut_ptr(),
                  b"ID\x00" as *const u8 as *const libc::c_char) == 0 {
            /* アドレス書き出し */
            if line != 1 as libc::c_int {
                /* 初回以外 */
                write_data(&mut ipal_frame, point.as_mut_ptr(),
                           closest.as_mut_ptr(), pos, casenum, &mut address,
                           flag);
            }
            /* 初期化 */
            pos = 0 as libc::c_int;
            item = 0 as libc::c_int;
            casenum = 0 as libc::c_int;
            memset(closest.as_mut_ptr() as *mut libc::c_void,
                   0 as libc::c_int,
                   (::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(20 as libc::c_int as
                                                        libc::c_ulong));
        } else if strncmp(tag.as_mut_ptr(),
                          b"\xe6\xa0\xbc\x00" as *const u8 as
                              *const libc::c_char,
                          strlen(b"\xe6\xa0\xbc\x00" as *const u8 as
                                     *const libc::c_char)) == 0 {
            casenum += 1;
            if casenum > 20 as libc::c_int {
                fprintf(stderr,
                        b"# of cases is more than MAX (%d).\n\x00" as
                            *const u8 as *const libc::c_char,
                        20 as libc::c_int);
                exit(1 as libc::c_int);
            }
            /* 直前格 */
            if *DATA.as_mut_ptr().offset(strlen(DATA.as_mut_ptr()) as
                                             isize).offset(-(1 as libc::c_int
                                                                 as isize)) as
                   libc::c_int == '*' as i32 {
                closest[((item - 4 as libc::c_int) / 3 as libc::c_int) as
                            usize] = item + 1 as libc::c_int
                /* この格の用例の位置 */
            }
        }
        point[item as usize] = pos;
        strcpy(&mut *ipal_frame.DATA.as_mut_ptr().offset(pos as isize) as
                   *mut libc::c_uchar as *mut libc::c_char,
               DATA.as_mut_ptr());
        if strcmp(DATA.as_mut_ptr(),
                  b"nil\x00" as *const u8 as *const libc::c_char) == 0 {
            ipal_frame.DATA[pos as usize] = '\u{0}' as i32 as libc::c_uchar;
            pos += 1 as libc::c_int
        } else {
            pos =
                (pos as
                     libc::c_ulong).wrapping_add(strlen(DATA.as_mut_ptr()).wrapping_add(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                    as libc::c_int as libc::c_int
        }
        if pos > 8192000 as libc::c_int {
            fprintf(stderr,
                    b"%d is small for IPAL record (%s).\n\x00" as *const u8 as
                        *const libc::c_char, 8192000 as libc::c_int,
                    ipal_frame.DATA.as_mut_ptr());
            exit(1 as libc::c_int);
        }
        item += 1;
        /* ORの格フレームなら読みを登録しない */
        if strncmp(tag.as_mut_ptr(),
                   b"\xe7\xb4\xa0\xe6\x80\xa7\x00" as *const u8 as
                       *const libc::c_char,
                   strlen(b"\xe7\xb4\xa0\xe6\x80\xa7\x00" as *const u8 as
                              *const libc::c_char)) == 0 {
            flag = 1 as libc::c_int;
            /* 要素をsplit */
            token =
                strtok(DATA.as_mut_ptr(),
                       b" \x00" as *const u8 as
                           *const libc::c_char); /* 読みを登録しない */
            while !token.is_null() {
                if strcmp(token,
                          b"\xe5\x92\x8c\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\x00"
                              as *const u8 as *const libc::c_char) == 0 {
                    flag = 0 as libc::c_int;
                    break ;
                } else {
                    token =
                        strtok(0 as *mut libc::c_char,
                               b" \x00" as *const u8 as *const libc::c_char)
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tolend(mut i: libc::c_int) -> libc::c_int {
    return i >> 24 as libc::c_int |
               (i >> 16 as libc::c_int & 0xff as libc::c_int) <<
                   8 as libc::c_int |
               (i >> 8 as libc::c_int & 0xff as libc::c_int) <<
                   16 as libc::c_int |
               (i & 0xff as libc::c_int) << 24 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
