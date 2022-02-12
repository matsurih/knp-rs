#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc::getcwd;

use crate::juman;
use crate::juman::consts::{_ISlower, _ISupper, AllocateError, OpenError};
use crate::juman::ctools::{__ctype_b_loc, __ctype_tolower_loc, __ctype_toupper_loc, asctime, car, cdr, error, exit, fgetc, fopen, fprintf, fputc, fputs, fseek, ftell, getenv, Jumangram_Dirname, LineNo, LineNoForError, localtime, malloc, perror, ProgName, realloc, s_feof, s_read, sprintf, stderr, strcat, strcmp, strcpy, strlen, time, tolower, toupper, ungetc};
use crate::juman::structs::{timespec, tm};
use crate::juman::types::{CELL, FILE, size_t, time_t};

#[no_mangle]
pub static mut Jumanrc_Fileptr: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut Cha_errno: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Cha_stderr: *mut FILE = 0 as *const FILE as *mut FILE;
/*
------------------------------------------------------------------------------
	FUNCTION:
	<check_filesize>: check filesize
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn check_filesize(mut fp: *mut FILE) -> libc::c_int {
    let mut size: libc::c_int = 0;
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    size = ftell(fp) as libc::c_int;
    fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    return size;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<my_fopen>: do "fopen"/<filename> and error processing
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_fopen(mut filename: *mut libc::c_char,
                                  mut mode: *mut libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(filename, mode);
    if fp.is_null() {
        error(OpenError as libc::c_int,
              b"can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, filename,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    return fp;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<pathfopen>: do "fopen"/<filename_path> ( <path> + <filename> )
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn pathfopen(mut filename: *mut libc::c_char,
                                   mut mode: *mut libc::c_char,
                                   mut path: *mut libc::c_char,
                                   mut filename_path: *mut libc::c_char)
                                   -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    strcpy(filename_path, path);
    strcat(filename_path, filename);
    fp = fopen(filename_path, mode);
    return fp;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<my_pathfopen>: do <pathfopen> and error processing
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_pathfopen(mut filename: *mut libc::c_char,
                                      mut mode: *mut libc::c_char,
                                      mut path: *mut libc::c_char,
                                      mut filename_path: *mut libc::c_char)
                                      -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = pathfopen(filename, mode, path, filename_path);
    if fp.is_null() {
        error(OpenError as libc::c_int,
              b"Can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, filename_path,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    return fp;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<my_feof>: if <fp> points to "EOF" return <TRUE> else return <FALSE>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_feof(mut fp: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = fgetc(fp);
    return if c == -(1 as libc::c_int) {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        ungetc(c, fp);
        0 as libc::c_int
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<append_postfix>: append <affix>  to <filename>
	<change_postfix>: change <affix1> of <filename> to <affix2>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn append_postfix(mut filename: *mut libc::c_char,
                                        mut affix: *mut libc::c_char) {
    if strcmp(&mut *filename.offset((strlen as
        unsafe extern "C" fn(_:
                             *const libc::c_char)
                             ->
                             libc::c_ulong)(filename).wrapping_sub((strlen
        as
        unsafe extern "C" fn(_:
                             *const libc::c_char)
                             ->
                             libc::c_ulong)(affix))
        as isize), affix) != 0 &&
        *filename.offset(strlen(filename).wrapping_sub(1 as libc::c_int as
            libc::c_ulong)
            as isize) as libc::c_int != '.' as i32 {
        strcat(filename, affix);
    };
}

#[no_mangle]
pub unsafe extern "C" fn change_postfix(mut filename: *mut libc::c_char,
                                        mut affix1: *mut libc::c_char,
                                        mut affix2: *mut libc::c_char) {
    if strcmp(&mut *filename.offset((strlen as
        unsafe extern "C" fn(_:
                             *const libc::c_char)
                             ->
                             libc::c_ulong)(filename).wrapping_sub((strlen
        as
        unsafe extern "C" fn(_:
                             *const libc::c_char)
                             ->
                             libc::c_ulong)(affix1))
        as isize), affix1) == 0 {
        *filename.offset(strlen(filename).wrapping_sub(strlen(affix1)) as
            isize) = '\u{0}' as i32 as libc::c_char
    }
    strcat(filename, affix2);
}
/*	
------------------------------------------------------------------------------
	PROCEDURE:
	<getpath>: get <cur_path> and <juman_path>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn getpath(mut cur_path: *mut libc::c_char, mut juman_path: *mut libc::c_char) {
    // let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    getcwd(cur_path, 4096 as libc::c_int as libc::size_t);
    strcpy(juman_path, Jumangram_Dirname.as_mut_ptr());
    if *cur_path.offset(strlen(cur_path).wrapping_sub(1 as libc::c_int as
        libc::c_ulong) as
        isize) as libc::c_int != '/' as i32 {
        strcat(cur_path, b"/\x00" as *const u8 as *const libc::c_char);
    }
    if *juman_path.offset(strlen(juman_path).wrapping_sub(1 as libc::c_int as
        libc::c_ulong)
        as isize) as libc::c_int != '/' as i32 {
        strcat(juman_path, b"/\x00" as *const u8 as *const libc::c_char);
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<my_alloc>: do "malloc" (library function) and error processing
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_alloc(mut n: libc::c_int) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = malloc(n as libc::c_ulong);
    if p.is_null() {
        error(AllocateError as libc::c_int,
              b"Not enough memory. Can\'t allocate.\x00" as *const u8 as
                  *const libc::c_char as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    return p;
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<my_realloc>: do "realloc" (library function) and error processing
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_realloc(mut ptr: *mut libc::c_void,
                                    mut n: libc::c_int) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = realloc(ptr, n as libc::c_ulong);
    if p.is_null() {
        error(AllocateError as libc::c_int,
              b"Not enough memory. Can\'t allocate.\x00" as *const u8 as
                  *const libc::c_char as *mut libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    return p;
}
/*
------------------------------------------------------------------------------
	PROCEDURE
	<my_exit>: print error-number on "stderr", and "exit"
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_exit(mut exit_code: libc::c_int) {
    fprintf(stderr, b"exit(%d)\n\x00" as *const u8 as *const libc::c_char,
            exit_code);
    exit(exit_code);
}


#[no_mangle]
pub unsafe extern "C" fn lower(mut c: libc::c_char) -> libc::c_char {
    return if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
        _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_int;
                    __res =
                        if __c < -(128 as libc::c_int) ||
                            __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as
                                isize)
                        }
                } else { __res = tolower(c as libc::c_int) }
            } else {
                __res =
                    *(*__ctype_tolower_loc()).offset(c as libc::c_int
                        as isize)
            }
            __res
        }) as libc::c_char
    } else { c };
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<upper>: if <char:c> is a small character, upper <c>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn upper(mut c: libc::c_char) -> libc::c_char {
    return if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
        _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_int;
                    __res =
                        if __c < -(128 as libc::c_int) ||
                            __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as
                                isize)
                        }
                } else { __res = toupper(c as libc::c_int) }
            } else {
                __res =
                    *(*__ctype_toupper_loc()).offset(c as libc::c_int
                        as isize)
            }
            __res
        }) as libc::c_char
    } else { c };
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<my_strlen>: return length of the string which is pointed to by <s>.
	             if <s> == NULL return 0
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_strlen(mut s: *mut libc::c_uchar) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    if !s.is_null() { while *s.offset(n as isize) != 0 { n += 1 } }
    return n;
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<my_strcpy>:
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_strcpy(mut s1: *mut libc::c_uchar,
                                   mut s2: *mut libc::c_uchar) {
    if s2.is_null() {
        s1 = 0 as *mut libc::c_uchar;
        return; }
    strcpy(s1 as *mut libc::c_char, s2 as *const libc::c_char);
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<my_strcmp>: 
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn my_strcmp(mut s1: *mut libc::c_uchar,
                                   mut s2: *mut libc::c_uchar)
                                   -> libc::c_int {
    if s1.is_null() && s2.is_null() { return 0 as libc::c_int; }
    if s1.is_null() { return -(1 as libc::c_int); }
    if s2.is_null() { return 1 as libc::c_int; }
    return strcmp(s1 as *const libc::c_char, s2 as *const libc::c_char);
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<compare_top_str>: if <s1> = <s2...> or <s2> = <s1...> return TRUE
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn compare_top_str(mut s1: *mut libc::c_uchar,
                                         mut s2: *mut libc::c_uchar)
                                         -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while *s1.offset(i as isize) as libc::c_int != 0 &&
        *s2.offset(i as isize) as libc::c_int != 0 {
        if *s1.offset(i as isize) as libc::c_int !=
            *s2.offset(i as isize) as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<compare_top_str1>: if <s1> = <s2...> return TRUE
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn compare_top_str1(mut s1: *mut libc::c_uchar,
                                          mut s2: *mut libc::c_uchar)
                                          -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = strlen(s1 as *const libc::c_char) as libc::c_int;
    l2 = strlen(s2 as *const libc::c_char) as libc::c_int;
    if l1 > l2 { return 0 as libc::c_int; }
    loop {
        let fresh0 = l1;
        l1 = l1 - 1;
        if !(fresh0 != 0) { break; }
        if *s1.offset(l1 as isize) as libc::c_int !=
            *s2.offset(l1 as isize) as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<compare_top_str2>: if <s1...> = <s2> return TRUE
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn compare_top_str2(mut s1: *mut libc::c_uchar,
                                          mut s2: *mut libc::c_uchar)
                                          -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = strlen(s1 as *const libc::c_char) as libc::c_int;
    l2 = strlen(s2 as *const libc::c_char) as libc::c_int;
    if l1 < l2 { return 0 as libc::c_int; }
    loop {
        let fresh1 = l2;
        l2 = l2 - 1;
        if !(fresh1 != 0) { break; }
        if *s1.offset(l2 as isize) as libc::c_int !=
            *s2.offset(l2 as isize) as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*
------------------------------------------------------------------------------
	FUNCTION:
	<compare_end_str>: if <s1> = <...s2> or <s2> = <...s1> return TRUE
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn compare_end_str(mut s1: *mut libc::c_uchar,
                                         mut s2: *mut libc::c_uchar)
                                         -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = strlen(s1 as *const libc::c_char) as libc::c_int;
    l2 = strlen(s2 as *const libc::c_char) as libc::c_int;
    return if l1 >= l2 {
        if strcmp(s1.offset(l1 as isize).offset(-(l2 as isize)) as
                      *const libc::c_char, s2 as *const libc::c_char) ==
            0 as libc::c_int {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strcmp(s2.offset(l2 as isize).offset(-(l1 as isize)) as
                         *const libc::c_char, s1 as *const libc::c_char) ==
        0 as libc::c_int {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<ls>: print <char *:p> file-information on <FILE *:fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn ls(mut fp: *mut FILE, mut p: *mut libc::c_char,
                            mut f: *mut libc::c_char) {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut stbuf: juman::structs::stat =
        juman::structs::stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
    strcpy(path.as_mut_ptr(), p);
    strcat(path.as_mut_ptr(), f);
    juman::ctools::stat(path.as_mut_ptr(), &mut stbuf);
    fprintf(fp, b"%8ld bytes: %s\n\x00" as *const u8 as *const libc::c_char,
            stbuf.st_size, path.as_mut_ptr());
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<print_current_time>: print current local-time on <FILE *:fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn print_current_time(mut fp: *mut FILE) {
    let mut t: time_t = 0;
    let mut tp: *mut tm = 0 as *mut tm;
    time(&mut t);
    tp = localtime(&mut t);
    fprintf(fp, asctime(tp));
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<print_execute_time>: print two kinds of execution time on <FILE *:fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn print_execute_time(mut fp: *mut FILE,
                                            mut dt: libc::c_int,
                                            mut dp: libc::c_float) {
    dp = (dp as libc::c_double / 1000000.0f64) as libc::c_float;
    fprintf(fp,
            b"execution time: %8.3fs\n\x00" as *const u8 as
                *const libc::c_char, dt as libc::c_float as libc::c_double);
    fprintf(fp,
            b"processor time: %8.3fs\n\x00" as *const u8 as
                *const libc::c_char, dp as libc::c_double);
}
/*
------------------------------------------------------------------------------
        PROCEDURE:
        <set_jumanrc_fileptr>: set Jumanrc_Fileptr

	WIN32 用に juman.ini を見に行くように変更
	RC_DEFAULTがない場合にexitするかどうかをflagで制御するように変更  (2002/11/08)
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn set_jumanrc_fileptr(mut option_rcfile:
                                             *mut libc::c_char,
                                             mut look_rcdefault_p:
                                             libc::c_int,
                                             mut exit_rc_notfound_p:
                                             libc::c_int) {
    /*
      rcfileをさがす順

      <makeint, makemat>
      	$HOME/.jumanrc
	→ rcfileがなくてもよい

      <juman server, standalone> 
       	-r オプション
	$HOME/.jumanrc                _WIN32 の場合は探す必要はない (Changed by Taku Kudoh)
        c:\(winnt|windows)\juman.ini  _WIN32 の場合juman.ini を探す (Changed by Taku Kudoh)
	RC_DEFAULT (Makefile)         
	→ rcfileがなければエラー

      <juman client>
       	-r オプション
	$HOME/.jumanrc
	→ rcfileがなくてもよい
    */
    let mut user_home_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    if !option_rcfile.is_null() {
        Jumanrc_Fileptr =
            fopen(option_rcfile,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if Jumanrc_Fileptr.is_null() {
            fprintf(stderr,
                    b"not found <%s>.\n\x00" as *const u8 as
                        *const libc::c_char, option_rcfile);
            exit(0 as libc::c_int);
        }
    } else {
        user_home_ptr =
            getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if user_home_ptr.is_null() {
            /* error(ConfigError, "please set <environment variable> HOME.", EOA); */
            filename[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        } else {
            sprintf(filename.as_mut_ptr(),
                    b"%s/.jumanrc\x00" as *const u8 as *const libc::c_char,
                    user_home_ptr);
        }
        if filename[0 as libc::c_int as usize] as libc::c_int ==
            '\u{0}' as i32 ||
            {
                Jumanrc_Fileptr =
                    fopen(filename.as_mut_ptr(),
                          b"r\x00" as *const u8 as *const libc::c_char);
                Jumanrc_Fileptr.is_null()
            } {
            if look_rcdefault_p != 0 {
                Jumanrc_Fileptr =
                    fopen(b"/usr/local/etc/jumanrc\x00" as *const u8 as
                              *const libc::c_char,
                          b"r\x00" as *const u8 as *const libc::c_char);
                if Jumanrc_Fileptr.is_null() {
                    if exit_rc_notfound_p != 0 {
                        fprintf(stderr,
                                b"not found <.jumanrc> and <RC_DEFAULT> file.\n\x00"
                                    as *const u8 as *const libc::c_char);
                        exit(0 as libc::c_int);
                    } else { Jumanrc_Fileptr = 0 as *mut FILE }
                }
            } else { Jumanrc_Fileptr = 0 as *mut FILE }
        }
    };
}
/*
------------------------------------------------------------------------------
        PROCEDURE:
        <set_jumangram_dirname>: read Jumanrc_File 
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn set_jumangram_dirname() {
    let mut cell1: *mut CELL = 0 as *mut CELL;
    let mut cell2: *mut CELL = 0 as *mut CELL;
    Jumangram_Dirname[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    LineNo = 0 as libc::c_int;
    while s_feof(Jumanrc_Fileptr) == 0 {
        LineNoForError = LineNo;
        cell1 = s_read(Jumanrc_Fileptr);
        if strcmp(b"\xe6\x96\x87\xe6\xb3\x95\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\x00"
                      as *const u8 as *const libc::c_char,
                  (*car(cell1)).value.atom as *const libc::c_char) == 0 {
            cell2 = car(cdr(cell1));
            if !(!cell2.is_null() &&
                {
                    cell2 = car(cdr(cell1));
                    ((*cell2).tag) == 1 as libc::c_int
                }) {
                fprintf(stderr,
                        b"error in .jumanrc\x00" as *const u8 as
                            *const libc::c_char);
                exit(0 as libc::c_int);
            } else {
                strcpy(Jumangram_Dirname.as_mut_ptr(),
                       (*cell2).value.atom as *const libc::c_char);
            }
        }
    };
    /* fclose(Jumanrc_Fileptr); */
}
/*
==============================================================================
		Oct. 1996       A.Kitauchi <akira-k@is.aist-nara.ac.jp>
==============================================================================
*/
static mut progpath: [libc::c_char; 1024] =
    unsafe {
        *::std::mem::transmute::<&[u8; 1024],
            &mut [libc::c_char; 1024]>(b"juman\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    };
static mut filepath: [libc::c_char; 1024] = [0; 1024];
/*
------------------------------------------------------------------------------
        PROCEDURE
        <cha_exit>: print error messages on stderr and exit
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn cha_exit(mut status: libc::c_int,
                                  mut format: *mut libc::c_char) {
    if Cha_errno != 0 { return; }
    if Cha_stderr != stderr {
        fputs(b"500 \x00" as *const u8 as *const libc::c_char, Cha_stderr);
    }
    if !progpath.as_mut_ptr().is_null() {
        fprintf(Cha_stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
                progpath.as_mut_ptr());
    }
    fprintf(Cha_stderr, format);
    if status >= 0 as libc::c_int {
        fputc('\n' as i32, Cha_stderr);
        if Cha_stderr == stderr { exit(status); }
        Cha_errno = 1 as libc::c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn cha_exit_file(mut status: libc::c_int,
                                       mut format: *mut libc::c_char,
                                       mut a: *mut libc::c_char,
                                       mut b: *mut libc::c_char,
                                       mut c: *mut libc::c_char,
                                       mut d: *mut libc::c_char,
                                       mut e: *mut libc::c_char,
                                       mut f: *mut libc::c_char,
                                       mut g: *mut libc::c_char,
                                       mut h: *mut libc::c_char) {
    if Cha_errno != 0 { return; }
    if Cha_stderr != stderr {
        fputs(b"500 \x00" as *const u8 as *const libc::c_char, Cha_stderr);
    }
    if !progpath.as_mut_ptr().is_null() {
        fprintf(Cha_stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
                progpath.as_mut_ptr());
    }
    if !(LineNo == 0 as libc::c_int) {
        if LineNo == LineNoForError {
            fprintf(Cha_stderr,
                    b"%s:%d: \x00" as *const u8 as *const libc::c_char,
                    filepath.as_mut_ptr(), LineNo);
        } else {
            fprintf(Cha_stderr,
                    b"%s:%d-%d: \x00" as *const u8 as *const libc::c_char,
                    filepath.as_mut_ptr(), LineNoForError, LineNo);
        }
    }
    fprintf(Cha_stderr, format, a, b, c, d, e, f, g, h);
    if status >= 0 as libc::c_int {
        fputc('\n' as i32, Cha_stderr);
        if Cha_stderr == stderr { exit(status); }
        Cha_errno = 1 as libc::c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn cha_perror(mut s: *mut libc::c_char) {
    cha_exit(-(1 as libc::c_int), b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    perror(s);
}

#[no_mangle]
pub unsafe extern "C" fn cha_exit_perror(mut s: *mut libc::c_char) {
    cha_perror(s);
    exit(1 as libc::c_int);
}
