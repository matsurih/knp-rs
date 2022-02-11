#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, ptr_wrapping_offset_from, register_tool)]


use crate::ctools::{exit, realloc, stderr, stdin};
use crate::db::{db_close, db_put, db_write_open};
use crate::structs::CDB_FILE;
use crate::types::DBM_FILE;

#[no_mangle]
pub static mut OptEncoding: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn content_process(mut content: *mut libc::c_char,
                                         mut pre_content: *mut *mut libc::c_char,
                                         mut pre_content_size: *mut libc::c_int,
                                         mut Type: libc::c_int,
                                         mut Separator: *mut libc::c_char)
 -> libc::c_int {
    let mut content_len: libc::c_int = strlen(content) as libc::c_int;
    if Type == 2 as libc::c_int {
        if *pre_content_size == 0 as libc::c_int {
            *pre_content_size = 6553600 as libc::c_int;
            *pre_content =
                malloc(*pre_content_size as libc::c_ulong) as
                    *mut libc::c_char
        }
        while content_len >= *pre_content_size {
            *pre_content_size *= 2 as libc::c_int;
            *pre_content =
                realloc(*pre_content as *mut libc::c_void,
                        *pre_content_size as libc::c_ulong) as
                    *mut libc::c_char
        }
        strcpy(*pre_content, content);
    } else if Type == 1 as libc::c_int {
        content_len =
            (content_len as libc::c_ulong).wrapping_add(strlen(*pre_content))
                as libc::c_int as libc::c_int;
        if !Separator.is_null() {
            content_len =
                (content_len as libc::c_ulong).wrapping_add(strlen(Separator))
                    as libc::c_int as libc::c_int
        }
        while content_len >= *pre_content_size {
            *pre_content_size *= 2 as libc::c_int;
            *pre_content =
                realloc(*pre_content as *mut libc::c_void,
                        *pre_content_size as libc::c_ulong) as
                    *mut libc::c_char
        }
        if !Separator.is_null() { strcat(*pre_content, Separator); }
        strcat(*pre_content, content);
    } else if Type == 4 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < strlen(*pre_content) {
            if *content.offset(i as isize) as libc::c_int == '1' as i32 {
                *(*pre_content).offset(i as isize) =
                    '1' as i32 as libc::c_char
            }
            i += 1
        }
    } else if Type == 3 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while (i_0 as libc::c_ulong) < strlen(*pre_content) {
            if *content.offset(i_0 as isize) as libc::c_int == '0' as i32 {
                *(*pre_content).offset(i_0 as isize) =
                    '0' as i32 as libc::c_char
            }
            i_0 += 1
        }
    }
    panic!("Reached end of non-void function without returning");
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut db: DBM_FILE = 0 as *mut CDB_FILE;
    let mut Type: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut pre_content_size: libc::c_int = 0 as libc::c_int;
    let mut Separator: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 2048] = [0; 2048];
    let mut pre_key: [libc::c_char; 2048] = [0; 2048];
    let mut pre_content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: *mut libc::c_char =
        malloc(6553600 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    let mut content: *mut libc::c_char =
        malloc(6553600 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if argc == 2 as libc::c_int {
        Type = 1 as libc::c_int
    } else if argc == 3 as libc::c_int &&
                  strcmp(*argv.offset(2 as libc::c_int as isize),
                         b"-append\x00" as *const u8 as *const libc::c_char)
                      == 0 {
        Type = 1 as libc::c_int
    } else if argc == 4 as libc::c_int &&
                  strcmp(*argv.offset(2 as libc::c_int as isize),
                         b"-append\x00" as *const u8 as *const libc::c_char)
                      == 0 {
        Type = 1 as libc::c_int;
        Separator = strdup(*argv.offset(3 as libc::c_int as isize))
    } else if argc == 3 as libc::c_int &&
                  strcmp(*argv.offset(2 as libc::c_int as isize),
                         b"-and\x00" as *const u8 as *const libc::c_char) == 0
     {
        Type = 3 as libc::c_int
    } else if argc == 3 as libc::c_int &&
                  strcmp(*argv.offset(2 as libc::c_int as isize),
                         b"-or\x00" as *const u8 as *const libc::c_char) == 0
     {
        Type = 4 as libc::c_int
    } else if argc == 3 as libc::c_int &&
                  strcmp(*argv.offset(2 as libc::c_int as isize),
                         b"-z\x00" as *const u8 as *const libc::c_char) == 0 {
        Type = 5 as libc::c_int
    } else {
        fprintf(stderr,
                b"usage: %s database_name [-append string|-and|-or|-z]\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    /* データベース作成 */
    db = db_write_open(*argv.offset(1 as libc::c_int as isize));
    fprintf(stderr,
            b"Create Database <%s>.\n\x00" as *const u8 as
                *const libc::c_char, *argv.offset(1 as libc::c_int as isize));
    pre_key[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    *buffer.offset((6553600 as libc::c_int - 1 as libc::c_int) as isize) =
        '\n' as i32 as libc::c_char;
    num = 0 as libc::c_int;
    while !fgets(buffer, 6553600 as libc::c_int, stdin).is_null() {
        /* 行の長さチェック */
        if *buffer.offset((6553600 as libc::c_int - 1 as libc::c_int) as
                              isize) as libc::c_int != '\n' as i32 {
            fprintf(stderr,
                    b"Line %d is larger than %d bytes.\n\x00" as *const u8 as
                        *const libc::c_char, num, 6553600 as libc::c_int);
            free(buffer as *mut libc::c_void);
            exit(1 as libc::c_int);
        }
        /* キーの長さチェック */
        cp = strchr(buffer, ' ' as i32);
        if !cp.is_null() {
            if cp.wrapping_offset_from(buffer) as libc::c_long >=
                   2048 as libc::c_int as libc::c_long {
                fprintf(stderr,
                        b"Key is too long (in %s).\n\x00" as *const u8 as
                            *const libc::c_char, buffer);
                free(buffer as *mut libc::c_void);
                exit(1 as libc::c_int);
            }
            /* keyとcontentに分離 */
            sscanf(buffer,
                   b"%s %[^\n]\x00" as *const u8 as *const libc::c_char,
                   key.as_mut_ptr(), content);
            let fresh1 = num;
            num = num + 1;
            if fresh1 % 100000 as libc::c_int == 0 as libc::c_int {
                fputc('*' as i32, stderr);
            }
            /* 直前のkeyと同じなら連結して保存 */
            if strcmp(pre_key.as_mut_ptr(), key.as_mut_ptr()) == 0 {
                content_process(content, &mut pre_content,
                                &mut pre_content_size, Type, Separator);
            } else {
                /* 書き込み */
                if pre_key[0 as libc::c_int as usize] != 0 {
                    db_put(db, pre_key.as_mut_ptr(), pre_content, Separator,
                           Type);
                }
                strcpy(pre_key.as_mut_ptr(), key.as_mut_ptr());
                content_process(content, &mut pre_content,
                                &mut pre_content_size, 2 as libc::c_int,
                                Separator);
            }
        } else {
            /* スペースがないとき (スペースの前に\0を含む場合もひっかかる) */
            fprintf(stderr,
                    b"Line %d is strange.\n\x00" as *const u8 as
                        *const libc::c_char, num);
            let fresh0 = num;
            num = num + 1;
            if fresh0 % 100000 as libc::c_int == 0 as libc::c_int {
                fputc('*' as i32, stderr);
            }
        }
    }
    if pre_key[0 as libc::c_int as usize] != 0 {
        db_put(db, pre_key.as_mut_ptr(), pre_content, Separator, Type);
    }
    if pre_content_size > 0 as libc::c_int {
        free(pre_content as *mut libc::c_void);
    }
    if !Separator.is_null() { free(Separator as *mut libc::c_void); }
    fputc('\n' as i32, stderr);
    db_close(db);
    free(buffer as *mut libc::c_void);
    free(content as *mut libc::c_void);
    return 0 as libc::c_int;
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
