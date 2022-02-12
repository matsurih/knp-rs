#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::consts::{GramError, OpenError};
use crate::juman::ctools::{car, cdr, error, fclose, fprintf, fputc, fputs, getpath, LineNo, LineNoForError, my_alloc, my_exit, pathfopen, print_current_time, ProgName, s_feof, s_read, stderr, strcmp, strcpy, strlen, strrchr};
use crate::juman::types::{CELL, CLASS, FILE};

#[no_mangle]
pub static mut Class: [[CLASS; 129]; 129] =
    [[CLASS{id: 0 as *const libc::c_uchar as *mut libc::c_uchar,
            cost: 0,
            kt: 0,}; 129]; 129];
#[no_mangle]
pub static mut CurPath: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut JumanPath: [libc::c_char; 4096] = [0; 4096];

#[no_mangle]
pub unsafe extern "C" fn error_in_grammar(mut n: libc::c_int,
                                          mut line_no: libc::c_int) {
    match n {
        0 => {
            fprintf(stderr,
                    b"\nparse error at line %d\n\x00" as *const u8 as
                        *const libc::c_char, line_no);
            fprintf(stderr,
                    b"\ttoo many classfication.\n\x00" as *const u8 as
                        *const libc::c_char);
            my_exit(GramError as libc::c_int);
        }
        1 => {
            fprintf(stderr,
                    b"\nparse error at line %d\n\x00" as *const u8 as
                        *const libc::c_char, line_no);
            fprintf(stderr,
                    b"\ttoo many sub-classfication.\n\x00" as *const u8 as
                        *const libc::c_char);
            my_exit(GramError as libc::c_int);
        }
        _ => {
            fprintf(stderr,
                    b"\nparse error at line %d\n\x00" as *const u8 as
                        *const libc::c_char, line_no);
            my_exit(GramError as libc::c_int);
        }
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE
	<initialize_class>: initialize <Class[][]>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn initialize_class() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int + 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 128 as libc::c_int + 1 as libc::c_int {
            Class[i as usize][j as usize].id =
                0 as *mut libc::c_void as *mut libc::c_uchar;
            Class[i as usize][j as usize].kt = 0 as libc::c_int;
            Class[i as usize][j as usize].cost = 0 as libc::c_int;
            j += 1
            /* k.n */
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE
	<print_class_>: print <Class[][]> on <fp> according to format
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn print_class_(mut fp: *mut FILE,
                                      mut tab1: libc::c_int,
                                      mut tab2: libc::c_int,
                                      mut flag: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    i = 1 as libc::c_int;
    while !Class[i as usize][0 as libc::c_int as usize].id.is_null() &&
              i < 128 as libc::c_int {
        if tab1 > 0 as libc::c_int {
            n = 0 as libc::c_int;
            while n < tab1 { fputc(' ' as i32, fp); n += 1 }
        }
        fprintf(fp, b"%3d: %s\x00" as *const u8 as *const libc::c_char, i,
                Class[i as usize][0 as libc::c_int as usize].id);
        if Class[i as usize][0 as libc::c_int as usize].kt != 0 {
            fputs(flag, fp);
        }
        fputc('\n' as i32, fp);
        j = 1 as libc::c_int;
        while !Class[i as usize][j as usize].id.is_null() &&
                  j < 128 as libc::c_int {
            if tab2 > 0 as libc::c_int {
                n = 0 as libc::c_int;
                while n < tab2 { fputc(' ' as i32, fp); n += 1 }
            }
            fprintf(fp,
                    b"        %3d: %s\x00" as *const u8 as
                        *const libc::c_char, j,
                    Class[i as usize][j as usize].id);
            if Class[i as usize][j as usize].kt != 0 { fputs(flag, fp); }
            fputc('\n' as i32, fp);
            j += 1
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE
	<read_class>: read-in <Class[][]> from <fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn read_class(mut fp: *mut FILE) {
    let mut cell1: *mut CELL = 0 as *mut CELL;
    let mut cell2: *mut CELL = 0 as *mut CELL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut katuyou_flag: libc::c_int = 0 as libc::c_int;
    LineNo = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        j = 0 as libc::c_int;
        LineNoForError = LineNo;
        cell1 = s_read(fp);
        cell2 = car(cell1);
        if !cell2.is_null() {
            Class[i as usize][j as usize].id =
                my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                              libc::c_ulong).wrapping_mul(strlen((*car(cell2)).value.atom
                                                                     as
                                                                     *const libc::c_char)).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong)
                             as libc::c_int) as *mut libc::c_uchar;
            strcpy(Class[i as usize][j as usize].id as *mut libc::c_char,
                   (*car(cell2)).value.atom as *const libc::c_char);
            if !cdr(cell2).is_null() {
                katuyou_flag = 1 as libc::c_int;
                Class[i as usize][j as usize].kt =
                    (0 as libc::c_int == 0) as libc::c_int
            } else { katuyou_flag = 0 as libc::c_int }
            cell1 = car(cdr(cell1));
            j += 1
        } else { error_in_grammar(2 as libc::c_int, LineNo); }
        loop  {
            cell2 = car(cell1);
            if cell2.is_null() { break ; }
            Class[i as usize][j as usize].id =
                my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                              libc::c_ulong).wrapping_mul(strlen((*car(cell2)).value.atom
                                                                     as
                                                                     *const libc::c_char)).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong)
                             as libc::c_int) as *mut libc::c_uchar;
            strcpy(Class[i as usize][j as usize].id as *mut libc::c_char,
                   (*car(cell2)).value.atom as *const libc::c_char);
            if katuyou_flag != 0 || !cdr(cell2).is_null() {
                Class[i as usize][j as usize].kt =
                    (0 as libc::c_int == 0) as libc::c_int
            }
            cell1 = cdr(cell1);
            j += 1;
            if j >= 128 as libc::c_int {
                error_in_grammar(1 as libc::c_int, LineNo);
            }
        }
        i += 1;
        if i >= 128 as libc::c_int {
            error_in_grammar(0 as libc::c_int, LineNo);
        }
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<grammar>: initialize <Class[][]> and read-in <Class[][]>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn grammar(mut fp_out: *mut FILE) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut grammarfile_path: [libc::c_char; 4096] = [0; 4096];
    let mut prog_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    /* program basename (juman, makeint, ...) */
    if !ProgName.is_null() {
        prog_basename = strrchr(ProgName, '/' as i32);
        if !prog_basename.is_null() {
            prog_basename = prog_basename.offset(1)
        } else { prog_basename = ProgName }
    }
    getpath(CurPath.as_mut_ptr(), JumanPath.as_mut_ptr());
    loop  {
        fp =
            pathfopen(b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, grammarfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        fp =
            pathfopen(b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, CurPath.as_mut_ptr(),
                      grammarfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        if !prog_basename.is_null() &&
               strcmp(prog_basename,
                      b"juman\x00" as *const u8 as *const libc::c_char) != 0
               &&
               {
                   fp =
                       pathfopen(b"JUMAN.grammar\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"r\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"../dic/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 grammarfile_path.as_mut_ptr());
                   !fp.is_null()
               } {
            break ;
        }
        fp =
            pathfopen(b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, JumanPath.as_mut_ptr(),
                      grammarfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        error(OpenError as libc::c_int,
              b"can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char,
              b"JUMAN.grammar\x00" as *const u8 as *const libc::c_char,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if !fp_out.is_null() {
        print_current_time(fp_out);
        fprintf(fp_out,
                b"%s parsing... \x00" as *const u8 as *const libc::c_char,
                grammarfile_path.as_mut_ptr());
    }
    initialize_class();
    read_class(fp);
    if !fp_out.is_null() {
        fputs(b"done.\n\n\x00" as *const u8 as *const libc::c_char, fp_out);
    }
    fclose(fp);
}
