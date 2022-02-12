#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::consts::OpenError;
use crate::juman::ctools::{car, cdr, error, fclose, fprintf, fputc, fputs, getpath, my_alloc, pathfopen, print_current_time, s_feof, s_read, strcmp, strcpy, strlen, strrchr};
use crate::juman::types::{CELL, FILE, FORM, TYPE};

extern "C" {
    #[no_mangle]
    static mut ProgName: *mut libc::c_char;
    #[no_mangle]
    static mut LineNo: libc::c_int;
    #[no_mangle]
    static mut LineNoForError: libc::c_int;
}


#[no_mangle]
pub static mut Type: [TYPE; 128] = [TYPE{name: 0 as *const libc::c_uchar as *mut libc::c_uchar,}; 128];
#[no_mangle]
pub static mut Form: [[FORM; 128]; 128] = [[FORM{name: 0 as *const libc::c_uchar as *mut libc::c_uchar, gobi: 0 as *const libc::c_uchar as *mut libc::c_uchar, gobi_yomi: 0 as *const libc::c_uchar as *mut libc::c_uchar,}; 128]; 128];
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<initialize_type_form>: initialize <TYPE:Type>, <FORM:Form>
------------------------------------------------------------------------------
*/
unsafe extern "C" fn initialize_type_form() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        Type[i as usize].name = 0 as *mut libc::c_void as *mut libc::c_uchar;
        j = 0 as libc::c_int;
        while j < 128 as libc::c_int {
            Form[i as usize][j as usize].name =
                0 as *mut libc::c_void as *mut libc::c_uchar;
            Form[i as usize][j as usize].gobi =
                0 as *mut libc::c_void as *mut libc::c_uchar;
            Form[i as usize][j as usize].gobi_yomi =
                0 as *mut libc::c_void as *mut libc::c_uchar;
            j += 1
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<print_type_form>: print <TYPE:Type>, <FORM:Form> on <fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn print_type_form(mut fp: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while !Type[i as usize].name.is_null() && i < 128 as libc::c_int {
        fprintf(fp, b"%s\n\x00" as *const u8 as *const libc::c_char,
                Type[i as usize].name);
        j = 1 as libc::c_int;
        while !Form[i as usize][j as usize].name.is_null() &&
                  j < 128 as libc::c_int {
            fprintf(fp,
                    b"\t%-30s %-20s\n\x00" as *const u8 as
                        *const libc::c_char,
                    Form[i as usize][j as usize].name,
                    Form[i as usize][j as usize].gobi);
            j += 1
        }
        fputc('\n' as i32, fp);
        i += 1
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<read_type_form>: read-in <TYPE:Type>, <FORM:Form> from <fp>
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn read_type_form(mut fp: *mut FILE) {
    let mut cell1: *mut CELL = 0 as *mut CELL;
    let mut cell2: *mut CELL = 0 as *mut CELL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    LineNo = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while s_feof(fp) == 0 {
        LineNoForError = LineNo;
        cell1 = s_read(fp);
        Type[i as usize].name =
            my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                          libc::c_ulong).wrapping_mul(strlen((*car(cell1)).value.atom
                                                                 as
                                                                 *const libc::c_char)).wrapping_add(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                         as libc::c_int) as *mut libc::c_uchar;
        strcpy(Type[i as usize].name as *mut libc::c_char,
               (*car(cell1)).value.atom as *const libc::c_char);
        cell1 = car(cdr(cell1));
        j = 1 as libc::c_int;
        loop  {
            cell2 = car(cell1);
            if cell2.is_null() { break ; }
            Form[i as usize][j as usize].name =
                my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                              libc::c_ulong).wrapping_mul(strlen((*car(cell2)).value.atom
                                                                     as
                                                                     *const libc::c_char)).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong)
                             as libc::c_int) as *mut libc::c_uchar;
            strcpy(Form[i as usize][j as usize].name as *mut libc::c_char,
                   (*car(cell2)).value.atom as *const libc::c_char);
            Form[i as usize][j as usize].gobi =
                my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                              libc::c_ulong).wrapping_mul(strlen((*car(cdr(cell2))).value.atom
                                                                     as
                                                                     *const libc::c_char)).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong)
                             as libc::c_int) as *mut libc::c_uchar;
            if strcmp((*car(cdr(cell2))).value.atom as *const libc::c_char,
                      b"*\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                strcpy(Form[i as usize][j as usize].gobi as *mut libc::c_char,
                       b"\x00" as *const u8 as *const libc::c_char);
            } else {
                strcpy(Form[i as usize][j as usize].gobi as *mut libc::c_char,
                       (*car(cdr(cell2))).value.atom as *const libc::c_char);
            }
            if !car(cdr(cdr(cell2))).is_null() {
                /* 語尾の表記に漢字が混ざっている場合 */
                Form[i as usize][j as usize].gobi_yomi =
                    my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                                  libc::c_ulong).wrapping_mul(strlen((*car(cdr(cdr(cell2)))).value.atom
                                                                         as
                                                                         *const libc::c_char)).wrapping_add(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong)
                                 as libc::c_int) as *mut libc::c_uchar;
                if strcmp((*car(cdr(cdr(cell2)))).value.atom as
                              *const libc::c_char,
                          b"*\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    strcpy(Form[i as usize][j as usize].gobi_yomi as
                               *mut libc::c_char,
                           b"\x00" as *const u8 as *const libc::c_char);
                } else {
                    strcpy(Form[i as usize][j as usize].gobi_yomi as
                               *mut libc::c_char,
                           (*car(cdr(cdr(cell2)))).value.atom as
                               *const libc::c_char);
                }
            } else {
                Form[i as usize][j as usize].gobi_yomi =
                    my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                                  libc::c_ulong).wrapping_mul(strlen(Form[i as
                                                                              usize][j
                                                                                         as
                                                                                         usize].gobi
                                                                         as
                                                                         *const libc::c_char)).wrapping_add(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong)
                                 as libc::c_int) as *mut libc::c_uchar;
                strcpy(Form[i as usize][j as usize].gobi_yomi as
                           *mut libc::c_char,
                       Form[i as usize][j as usize].gobi as
                           *const libc::c_char);
            }
            j += 1;
            cell1 = cdr(cell1)
        }
        i += 1
    };
}
/*
------------------------------------------------------------------------------
	PROCEDURE:
	<katuyou>: call <initialize_type_form> and <read_type_form>

	juman_pathの前にカレントディレクトリを見るように変更 (2002/11/08)
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn katuyou(mut fp_out: *mut FILE) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cur_path: [libc::c_char; 4096] = [0; 4096];
    let mut juman_path: [libc::c_char; 4096] = [0; 4096];
    let mut katuyoufile_path: [libc::c_char; 4096] = [0; 4096];
    let mut prog_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    getpath(cur_path.as_mut_ptr(), juman_path.as_mut_ptr());
    /* program basename (juman, makeint, ...) */
    if !ProgName.is_null() {
        prog_basename = strrchr(ProgName, '/' as i32);
        if !prog_basename.is_null() {
            prog_basename = prog_basename.offset(1)
        } else { prog_basename = ProgName }
    }
    loop  {
        fp =
            pathfopen(b"JUMAN.katuyou\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, katuyoufile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        fp =
            pathfopen(b"JUMAN.katuyou\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, cur_path.as_mut_ptr(),
                      katuyoufile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        if !prog_basename.is_null() &&
               strcmp(prog_basename,
                      b"juman\x00" as *const u8 as *const libc::c_char) != 0
               &&
               {
                   fp =
                       pathfopen(b"JUMAN.katuyou\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"r\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"../dic/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 katuyoufile_path.as_mut_ptr());
                   !fp.is_null()
               } {
            break ;
        }
        fp =
            pathfopen(b"JUMAN.katuyou\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, juman_path.as_mut_ptr(),
                      katuyoufile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        error(OpenError as libc::c_int,
              b"can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, katuyoufile_path.as_mut_ptr(),
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if !fp_out.is_null() {
        print_current_time(fp_out);
        fprintf(fp_out,
                b"%s parsing... \x00" as *const u8 as *const libc::c_char,
                katuyoufile_path.as_mut_ptr());
    }
    initialize_type_form();
    read_type_form(fp);
    if !fp_out.is_null() {
        fputs(b"done.\n\n\x00" as *const u8 as *const libc::c_char, fp_out);
    }
    fclose(fp);
}
